# CPU and GPU Design Philosophy Differences: Latency Optimization (CPU) vs Throughput Optimization (GPU)

In modern enterprise backend architectures, to avoid even 0.001 seconds of response delay, CPUs boost clock speeds per core to over 5GHz and are packed with massive cache memory. In contrast, GPU architectures, designed for AI training, set clocks at a mere 1.5GHz but feature tens of thousands of cores. Why have these two architectures taken such diametrically opposed paths at the transistor level?

Computer architecture design is a product of compromise: deciding what kind of transistors, and how many, to place on a limited die (silicon chip area). CPUs and GPUs represent hardware design paradigms that stand at complete opposite ends of the spectrum.

- **Latency Optimization**: This design approach minimizes the absolute time it takes for a single task or thread to start and complete, with the core focus on maximizing the speed of serial processing.
- **Throughput Optimization**: This design approach maximizes the total amount of work that can be processed per unit of time (e.g., 1 second), even if the completion speed of a single task is somewhat slow. It's an architecture that pushes parallel processing to its extreme.
- **ALU (Arithmetic Logic Unit)**: This is a physical hardware circuit that performs actual mathematical calculations such as addition, multiplication, and bitwise operations. CPUs deploy a small number of ultra-high-performance complex ALUs, whereas GPUs deploy thousands to tens of thousands of simple-function ALUs.
- **Control Unit**: This is a hardware scheduler layer that fetches and decodes instruction codes, controls data flow, and predicts branches. CPUs allocate a significant portion of their die area to this control unit and cache memory.

<br>

## Problem Definition

When code written by a software engineer executes on hardware, CPUs and GPUs each face different types of physical limitations and bottlenecks.

- **Von Neumann Bottleneck and Memory Wall**: This is a speed difference bottleneck that occurs because ALU operation speeds have advanced to nanoseconds, while main memory DRAM data transfer speeds haven't kept pace. While the CPU reads data from memory, it takes approximately 200-300 clock cycles, during which the hardware stalls, unable to perform any operations.
- **Control Flow Branch Uncertainty Bottleneck**: Complex business logic code includes numerous conditional statements like if-else and switch-case. Until the result of a conditional statement is known, the next instruction cannot be fetched, leading to a pipeline bubble bottleneck where the hardware pipeline becomes empty, causing single-thread performance to plummet.
- **Data Dependency Bottleneck**: If the input for the next operation is the final output of a previous operation, the next operation's thread must wait until the previous operation has fully passed through the physical silicon circuit.

### Architectural Innovations and Solutions

To solve the physical bottlenecks defined above (especially the limitations of memory bandwidth and latency), the two hardware types have chosen completely different hardware structural innovations.

- **CPU Innovations (Massive Control Units and Cache Dominance)**: CPUs solve the memory wall and branch bottlenecks through prediction and hiding. Over 50% of the die area is filled with L1, L2, and L3 cache memory, ensuring data is supplied directly to the ALU from the cache line without needing to go to main memory. Additionally, they incorporate hardware-level Branch Predictors, **Speculative Execution** (which pre-calculates results before conditional statements are resolved), and **Out of Order Execution (OoO)** engines that reorder instructions at runtime, forcibly preventing single-thread stalls.
- **GPU Innovations (Latency Hiding through Massive Context Switching)**: GPUs have completely abandoned prediction. GPUs have no branch predictors, no out-of-order execution engines, and cache memory is designed to be very small, serving only to assist frequently used data transfer bandwidth rather than for data preservation. Instead, that area is entirely filled with ALUs. GPUs solve memory stalls by "swapping in" numerous other threads: if thread group 1 stalls while fetching data from memory, the hardware scheduler switches control to thread group 2 in 0 clock cycles, ensuring the ALUs are fully utilized without a single second of idleness.

<br>

## Principles

Let's trace the low-level mechanisms that occur in the CPU and GPU's internal circuits and data paths when application code is executed.

### CPU Data Path

- **Fetch/Decode**: The hardware frontend fetches instructions from the I-Cache. Instructions accumulate in the Out-of-Order (OoO) execution queue, and through register renaming, software threads' virtual registers are mapped to physical registers.
- **Execution ALU**: Instructions with resolved data dependencies are fed into the ALU from the Reservation Station, regardless of their original order. They pass through complex arithmetic units like Barrel Shifters and multi-precision floating-point units.
- **Cache Hit/Miss Path**: When reading data, the hardware prefetcher pulls data into the L2/L3 cache in advance, aiming for transfer completion at the L1 cache level, right next to the ALU bus. If a cache miss occurs, the pipeline locks and enters a waiting state.

### GPU Data Path

- **Warp Issuing:** The warp scheduler, located in the GPU's SM (Streaming Multiprocessor), scans the execution-ready queue. The minimum unit of execution in a GPU is not a single thread, but a warp, which is a group of 32 threads.
- **SIMT Execution Path:** When a single instruction is decoded, it is simultaneously delivered to the 32 ALUs within the SM. Each of the 32 ALUs retrieves independent data from its own register file and executes the same operation concurrently. This is the physical path of the SIMT (Single Instruction, Multiple Threads) architecture.
- **Zero overhead context switch**: The moment a warp encounters an HBM memory access instruction (Instruction Global Memory Load) and stalls, the warp scheduler, without the software overhead of backing up registers to memory, simply toggles a hardware switch circuit to connect the instruction pointer of the next executable warp to the ALU activation line. Because the GPU's register file is physically partitioned to be large enough to simultaneously hold tens of thousands of thread contexts, the cost of register backup/restore is zero.

<br>

## Performance Metrics and Mathematical Modeling

Let's quantify the differences between the two design philosophies using mathematical models from computer architecture theory.

**Compute Density Comparison Model**

This models the pure ratio of compute units to the total die area $A_{\text{total}}$.

$$\text{Density}_{\text{CPU}} = \frac{A_{\text{ALU}}}{A_{\text{Control}} + A_{\text{Cache}} + A_{\text{ALU}}} \approx 10\% \sim 20\%$$

$$\text{Density}_{\text{GPU}} = \frac{A_{\text{ALU\_Array}}}{A_{\text{Simple\_Control}} + A_{\text{Small\_Cache}} + A_{\text{ALU\_Array}}} \approx 70\% \sim 80\%$$

**Throughput Derivation via Little's Law**

We transform the relationship $L = TH \times W$, where L is the number of threads residing in the system, W is latency, and TH is throughput.

$$TH = \frac{L}{W}$$

- CPU: It drastically reduces the latency W of a single thread, maintaining TH through branch prediction and caching. The number of concurrent threads in the system is very small.
- GPU: Although memory latency W is long, spanning hundreds of cycles, it expands the number of concurrently active threads to tens of thousands, boosting throughput TH to tens of times that of a CPU.

<br>

## Layer and Profiling Settings

### Thread Divergence Penalty

If complex if-else branch statements exist within a kernel function written in GPU code, the SIMT structure is broken at the hardware level.

When 15 out of 32 threads take the 'if' path and 17 take the 'else' path, the GPU, lacking a branch predictor, cuts off ALU power to the 'else' path threads and makes them wait while the 'if' path executes, then sequentially executes the next part. This is called Warp divergence.

Even if GPU utilization is reported at 90%, the actual computation speed can be slower than that of a CPU.

### Interpreting Nsight System Profiling Metrics

`SM Eligible Warps per Cycle` is the average number of warps ready for execution per clock cycle. If this metric drops below 1, it means that the reserve threads available to hide memory latency have been exhausted, indicating a Memory Bound state where the GPU is idle, waiting for data.

### Task Allocation Strategy Workload Placement

- **CPU-Suitable Workloads:** Complex pointer tracking, frequent conditional branching, transaction processing, offset calculations, etc. (e.g., databases, query parsing, REST APIs)
- **GPU-Suitable Workloads**: Infinite repetition of identical operations with no data interference and complete independence (e.g., image pixels, matrix operations, deep learning weight dot product operations)
