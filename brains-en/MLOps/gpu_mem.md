# GPU Internal Memory Hierarchy

When a CPU software engineer declares a variable, they don't care whether that data physically goes into the L1 cache or the L2 cache.

This is because the hardware automatically optimizes everything. However, why do engineers writing GPU kernels have to explicitly code which memory region (Register, Shared Memory, Global Memory) to allocate data to?

Because CPUs and GPUs have different purposes, the way SRAM is placed and used on the silicon die is fundamentally different.

### CPU Memory Hierarchy (Transparent Cache)

L1, L2, and L3 caches are not exposed as physical address spaces to the programmer.

Hardware prefetchers and cache coherence protocols analyze these memory access patterns in real-time and automatically copy frequently used data from DRAM.

The sole purpose is to reduce latency.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FJyySF%2FbtsDIzjsJ1T%2FAAAAAAAAAAAAAAAAAAAAANe7cfFyCqtq0Uptqcxr5WUJ23sgPcP8NauuH0cEtpMx%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1780239599%26allow_ip%3D%26allow_referer%3D%26signature%3DKMI5iRFftKi8vmWiTktA0Elw50c%253D)

CPU Memory Hierarchy

### GPU Memory Hierarchy (Explicit Memory)

GPUs process hundreds of thousands of threads concurrently, so there isn't enough silicon area or power to include automated cache management logic. Instead, they expose high-speed SRAM regions that programmers can directly control.

- **VRAM (Global Memory)**: This is the GPU's main memory, implemented with HBM or GDDR technology. It offers high bandwidth but latencies can reach hundreds of cycles.
- **Shared Memory**: Located within the SM, this is an ultra-fast L1-level SRAM that programmers can directly allocate and control.
- **Register File**: Unlike CPUs, which have only dozens of registers per core, a GPU's SM contains tens of thousands of 32-bit registers in the form of a massive physical array.

<br>

## Problem Definition

When a GPU executes a large number of parallel threads, adopting the traditional CPU cache architecture as-is would lead to a physical collapse of the system.

- **Bandwidth Exhaustion due to Cache Thrashing**: If a GPU were equipped with an automatic L1 cache like a CPU, and tens of thousands of threads each demanded different data, eviction would constantly occur, pushing out existing data and loading new data into limited cache lines. This would lead to a hit rate approaching zero, consuming only VRAM bandwidth.
- **Lack of Physical Storage for Context Switching**: When a CPU switches threads, it backs up register values to main memory. A GPU needs to switch Warps (32 threads) every clock cycle. If register values were backed up to VRAM and restored for each switch, the memory bus would become paralyzed.
- **VRAM Round-trip Latency due to Data Dependencies**: In matrix multiplication, 256 threads within a single thread block must repeatedly read the same data tile. If this data were read from VRAM every time, it would accumulate enormous power consumption and latency.

### Solution

To overcome these physical limitations, GPU architecture shifted cache control from hardware to software (developers).

- **Static Allocation of Massive Register Files**: GPUs place massive register files, spanning hundreds of KB, within the SM. When a thread block is assigned to an SM, the compiler statically partitions and allocates the calculated number of registers to each thread. (Static Partitioning) Even if a thread stalls, its register values are not evicted to memory but remain on the physical chipset, enabling zero-clock context switching.
- **Software-Managed L1 Cache (Shared Memory)**: Instead of hardware guessing and moving data to L1, developers use the `__shared__` keyword to explicitly force a specific block of VRAM to be copied into the SM's internal SRAM. All threads within the block can then reuse this shared memory data dozens of times without VRAM access.

<br>

## Hardware Data Path and Memory Hierarchy Operation Principles

Tracing the physical data path from matrix data stored in VRAM (HBM) to the thread's arithmetic logic unit (ALU):

1.  **Movement from VRAM to L2 Cache**: When a GPU kernel issues a `Global Memory` load instruction, the memory controller reads data from HBM and brings it to the L2 cache located at the center of the GPU chip. The L2 cache is the only automatic cache area shared by all SMs.
2.  **Loading from L2 Cache to SM Internal SRAM (Shared Memory)**: Threads within a thread block cooperate (Coalesced Memory Access) to read data from the L2 cache and copy it into their SM's `Shared Memory` region. At this point, the data becomes local, not leaving the SM.
3.  **Synchronization Barrier**: To prevent operations from getting tangled until all data is loaded into shared memory, the `__syncthreads()` hardware barrier instruction aligns the timing of 256 threads.
4.  **Movement from Shared Memory to Register File**: Each thread loads the specific operand it will compute in the current clock cycle from Shared Memory into its individual Register File.
5.  **Computation, VRAM Write-back**: Data from the registers is sent to the ALU or Tensor Core for computation. The computed result is then either passed back through registers -> shared memory or directly through the L2 cache to be permanently written to VRAM.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FTJ04m%2FbtsDHq1A6YG%2FAAAAAAAAAAAAAAAAAAAAACdjLLwh46x7kkLodJYEgvTPZw0wovKvMoMKJZ2bSEZS%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1780239599%26allow_ip%3D%26allow_referer%3D%26signature%3Dxq6HW0XVN7F0zvH4iIoWT%252F6lvSU%253D)

GPU Memory Hierarchy and Data Flow

<br>

## Metrics and Profiling Log Analysis

To diagnose memory hierarchy design or failure bottlenecks, low-level logs and metrics output by the NVIDIA toolchain are analyzed.

#### `ptxas` Compiler Warning (Register Spilling Log)

```
ptxas info : Used 255 registers, 8 bytes smem, 16 bytes cmem[0]
ptxas warning : Spills 32 bytes to local memory
```

Interpreting the log above, too many local variables were declared within the kernel function, reaching the hardware's maximum register limit allocatable per thread (255 in this example).

The overflowing 32 bytes of data, though named Local Memory, are actually spilled to the slowest VRAM in terms of physical location. This is a spill.

If this warning appears, the performance of that kernel can drop by tens of times.

#### Nsight Compute `ncu` Shared Memory Bank Conflict Metric

-   `l1tex__data_bank_conflicts.sum`: Shared memory is divided into 32 independent bank structures to maximize bandwidth. If 32 threads within a warp happen to request different addresses in the same bank simultaneously, a hardware bottleneck occurs, serializing access. If this counter value is high, padding should be added to the memory address allocation array to physically reorder the banks and avoid conflicts.
-   `sm__occupancy.avg.pct_of_peak`: If a thread block occupies too much of the register file or shared memory, there won't be enough remaining SRAM space to load the next block onto the SM. If this metric is low, the register SRAM usage needs to be reduced.

### Profiling/Configuration

To control the physical limitations of the memory hierarchy in backend and AI infrastructure:

#### Forcing Register Limits via Launch Bounds `__lauch_bounds__`

To prevent Occupancy (the ratio of active threads within an SM) from dropping due to excessive register usage by a kernel, a forced command is given to the compiler: this kernel will use 256 threads per block, and at least 4 blocks must be maintained per SM, so suppress the maximum register usage accordingly.

```
// Forces 256 threads per block and maintains at least 4 blocks per SM to ensure Occupancy
__global__ void __launch_bounds__(256, 4) my_matmul_kernel(float* A, float* B, float* C) {
    // The compiler minimizes local variables within this block to prevent register overflow,
    // and optimizes through instruction reordering if necessary.
}
```

In a PyTorch environment, when writing GPU kernels with the Triton compiler, SRAM size can also be explicitly adjusted.

```python
@triton.jit
def matmul_kernel(a_ptr, b_ptr, c_ptr, ...):
    # 1. Load data from HBM (VRAM) using pointer arithmetic
    # During this process, data equal to BLOCK_SIZE is automatically allocated to SRAM (Shared Memory)
    a_block = tl.load(a_ptr + offsets, mask=mask)
    
    # 2. Perform cumulative operations on register-level local variables
    accumulator = tl.zeros((BLOCK_SIZE_M, BLOCK_SIZE_N), dtype=tl.float32)
    accumulator += tl.dot(a_block, b_block)
    
    # 3. Transfer the final result back to VRAM
    tl.store(c_ptr + offsets, accumulator, mask=mask)
```
