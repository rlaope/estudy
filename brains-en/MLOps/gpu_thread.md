# GPU Thread Execution Model

POSIX threads (pthreads), managed by backend engineers at the Linux OS layer, have independent stack memory in the order of megabytes, and the kernel controls context switching. In contrast, how can millions of GPU threads, created during a single GPU kernel execution, be scheduled at ultra-high speed and perform computations concurrently at the hardware level without OS kernel intervention?

The thread model of backend systems and the massively parallel thread model of GPUs share the same terminology, but their mechanisms for controlling hardware resources and executing instructions are entirely different.

- **Backend OS Threads (MIMD-based)**: Follow a Multiple Instruction, Multiple Data (MIMD) architecture. Each thread possesses its own program counter and an independent virtual memory stack area, allowing it to execute entirely different lines of code independently across CPU cores.
- **GPU Threads (SIMT-based)**: Follow a Single Instruction, Multiple Threads (SIMT) architecture. Millions of threads are created, but they operate in a grid-like fashion, **bound by the hardware's physical instruction execution scheduling unit, the Warp structure.**
- **Warp**: A Warp is the smallest fundamental hardware unit in NVIDIA GPUs that executes instructions concurrently. It always consists of a bundle of 32 threads, and these 32 threads execute the exact same source code line (instruction) with different data every clock cycle.
- **Thread Block and SM**: A logically grouped collection of threads (up to 1024) defined by the programmer. This block is entirely allocated to an SM (Streaming Multiprocessor), which is a physical cluster of computational cores inside the GPU. Threads within a block can communicate and synchronize with each other via shared memory, a high-speed, dedicated SRAM area.

<br>

## Defining Physical/Structural Bottlenecks

At the hardware thread scheduler layer, backends and GPUs each experience unique bottlenecks due to the limitations of their control mechanisms.

- **OS Context Switching Overhead (TLB/Cache Flush)**: When the number of active threads in a CPU backend exceeds the number of physical cores, the OS kernel triggers a timer interrupt to switch threads. This involves backing up register sets to the kernel stack, replacing virtual memory mapping information (Page Table), and polluting the CPU cache (L1/L2), leading to significant system latency (several microseconds).
- **Warp Divergence**: A hardware paralysis phenomenon that occurs when GPU threads encounter an `if else` conditional statement in the source code, causing the execution paths of the 32 threads to diverge. Due to the nature of the SIMT architecture, a warp cannot execute two different instructions in a single clock cycle. Therefore, while 16 threads satisfying the `if` condition are executing, the execution units of the remaining 16 threads are forcibly locked and put into a waiting state, effectively halving the hardware throughput.
- **Occupancy Degradation due to Register Pressure**: The size of the physical register file within an SM is fixed. If individual GPU threads declare too many variables, increasing the register allocation per thread, the SM cannot host as many thread blocks due to physical limitations. This leads to a shortage of waiting warps, resulting in hardware idle states where memory latency cannot be hidden.

### Architectural Innovations and Solutions

GPUs have solved the scheduling limitations of backend threads by extremely simplifying instruction control and optimizing hardware wiring.

- **Hardware Warp Scheduler Zero-Overhead Scheduling**: GPU thread scheduling does not involve OS kernel software. A dedicated hardware chip within the SM, the warp scheduler, checks a scoreboard every clock cycle, selects a warp that has finished waiting for memory I/O and is ready for execution, and immediately assigns it to a computational unit. Since the register state of all warps is constantly preserved on the physical chip, the context switching cost is completely zero.
- **Execution Masking Mechanism**: When warp divergence occurs, the hardware activates conditional mask bits, which are special register marks internally. Through a 32-bit mask map, it electrically controls which threads (1) will store computation results in the current clock and which threads (0) will ignore the computation and only receive the clock signal, thereby forcibly handling branch logic within a single instruction pipeline.

<br>

## How It Works

When a GPU kernel is called from the host CPU, we trace the low-level physical path of data and instructions moving through the hardware grid and warp pipeline.

1.  **Kernel Launch and GigaThread Scheduler Activation**: When the CPU calls `kernel<<<Grid, Black>>>()` via the CUDA driver, the GigaThread engine, the GPU's internal global hardware scheduler, receives this request.
2.  **Physical SM Distribution of Thread Blocks**: The GigaThread engine physically distributes the thread blocks requested by the kernel to available SMs across the GPU. The number of blocks that can reside in one SM is determined at runtime by calculating the register requirements per thread and the shared memory size.
3.  **Thread Warping (Warp Digestion)**: Thread blocks that have settled in an SM are physically split into warps of 32 threads each and registered in the warp scheduler's waiting slots. At this point, thread IDs 0 to 31 are mechanically assigned to Warp 0, and IDs 32 to 63 to Warp 1, and so on.
4.  **Instruction Fetch and SIMT Data Path Reach**: When the warp scheduler selects a waiting warp, the instruction code pointed to by that warp's program counter is brought into the L1 Instruction Cache. The single instruction signal, after passing through the decoder, travels along the power supply bus inside the SM and is simultaneously connected to the controllers of 32 ALUs (or Tensor Cores).
5.  **Concurrent Register File Access**: The 32 threads simultaneously access different address sections of the register file SRAM to retrieve operand data, pass it through the allocated 32 physical execution pipelines, and then write back the result to their respective hardware register spaces.

### System Low-Level Metrics and Profiling Log Analysis

To diagnose GPU hardware thread execution status and bottlenecks, we analyze the meaning of low-level metrics output by profilers and internal kernel counters.

### **NVIDIA Nsight Compute `ncu` Core Hardware Counter Metrics**

- ``sm__wraps_active.avg.pct_of_peak` (Active Warps): This is the percentage of warps physically loaded onto the SM and occupying resources. A low value indicates that the GPU is not being filled with threads due to kernel launch delays on the host CPU side or severe register scarcity.`
- ``smsp_warp_issue_stalled_barrier.per_warp` (Barrier Stall): This is the number of times the pipeline stalled, waiting for other warps to arrive, upon encountering the `__syncthreads()` synchronization instruction between threads within a thread block. It suggests an imbalanced workload distribution in the algorithm.`
- ``smsp__thread_inst_executed_per_inst_executed.ratio` (Divergence Metric): This is a key indicator for detecting warp divergence, with a theoretical maximum of 32. A value of 32 means all 32 threads are active during one instruction execution. If this metric logs a low value like 16.5, it means that nearly half of the cores are not participating in computations every clock cycle due to conditional branching, leading to wasted power.`

### CUDA Compiler `nvcc` Resource Allocation Low-Level Log Output when `--ptxas-options=-v` Option is Enabled

```
ptxas info    : Compiling entry function '_Z11train_kernelPfS_' for 'sm_80'
ptxas info    : Used 64 registers, 1024 bytes smem, 320 bytes cmem[0]
```

The compiler's analysis of the agent kernel indicates that it consistently occupies 64 registers per thread.

Since the maximum register limit per SM for the Ampere architecture sm_80 is 65,536, this kernel can only host a maximum of 65536/64 = 1024 threads, or 32 warps, on a single SM. This means the hardware is bottlenecked at 50% of its theoretical maximum of 2048 threads, a state that can be diagnosed through the logs.

### Example

For optimal warp execution models in AI frameworks or runtime environments,
it's beneficial to understand thread block dimension settings and compilation configurations. When writing custom CUDA extension cards for PyTorch or tuning Triton compiler settings, it is an absolute rule to set the number of threads per block as a multiple of the hardware warp size.

```py
# Triton compiler or custom accelerator layer configuration example
# If the internal thread block size is defined with an ambiguous value like 250, which is not a multiple of 32,
# the last warp will execute with 26 physically empty threads (Execution Masked), leading to permanent resource waste.
BLOCK_SIZE_X = 128  # 32 * 4 (exactly 4 warps configured for scheduler slots)
BLOCK_SIZE_Y = 2    # Total 256 threads configured for the block
```

- **Eliminating Conditionals via Warp-level Primitive Utilization (Software Optimization):** To avoid synchronization through `if-else` branch statements, branch divergence is circumvented at the code level by using hardware shuffle instructions (`__shfl_sync`) that allow 32 warp threads to directly exchange data among themselves within registers without shared memory.

```cuda
// Anti-pattern: Causes warp divergence due to branching
if (threadIdx.x == 0) {
    shared_mem[0] = local_val;
}
__syncthreads();

// Optimized pattern: Directly uses the hardware warp data bus to eliminate both branching and synchronization overhead
// 32 threads simultaneously copy the register value of thread 0 to their own registers (takes 1 clock cycle)
float warp_shared_val = __shfl_sync(0xFFFFFFFF, local_val, 0);
```
