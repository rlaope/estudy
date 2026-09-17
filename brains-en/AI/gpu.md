# GPU Architecture

### SIMT, Warp

The computational structure of a GPU is one where a single instruction processes multiple data elements simultaneously.

SIMT stands for Single Instruction, Multiple Threads. When an instruction dispatcher in the computational control unit issues a single instruction, multiple CUDA cores (ALUs) perform the same instruction based on their respective allocated data.

A Warp (Subgroup) is the smallest unit for managing hardware threads. According to NVIDIA standards, 32 threads are grouped into one warp, and they share the same program counter, executing completely simultaneously.

Branch Divergence occurs when threads within a warp take different execution paths due to `if-else` statements or similar constructs. In such cases, the hardware executes each path sequentially. Threads in the non-executing paths are deactivated (masked out), leading to reduced computational resource efficiency.

### Memory Hierarchy

The core of GPU performance optimization is to reduce data access latency by utilizing memory that is physically close to the computational units.

-   **Registers:** Located inside the Streaming Multiprocessor (SM), their scope of access is per thread. They have almost no latency, and since there's a fixed allocation per thread, exceeding it causes data to spill to off-chip memory.
-   **Shared Memory:** Located on-chip within the SM, its scope of access is per Block. It is an SRAM space controlled by the programmer through code and has the same high-speed bandwidth as the L1 cache.
-   **L1/L2 Cache:** Located inside the chip, their scope of access is SM / Device. They are automatically managed by hardware to improve data reusability.
-   **Global Memory:** Located in Device VRAM, its scope of access is per Grid. It is HBM/GDDR-based off-chip memory. While its capacity is large, its latency can be hundreds of cycles (400-800 cycles).

### Optimization

From a hardware perspective, let's explore GPU optimization and its core principles.

#### Memory Coalescing

When fetching data from global memory (VRAM), if 32 threads within a warp access contiguous and aligned addresses,

this is processed as a single transaction.

If addresses are fragmented, multiple memory requests occur, leading to significant bandwidth waste.

#### Bank Conflicts

Shared memory is typically divided into 32 independent banks for parallel access.

If different threads within a warp attempt to simultaneously access different addresses within the same bank, access becomes serialized, degrading performance.

#### Why Understanding Hardware is Important

In modern GPU computations, most operations exhibit memory-bound characteristics, where bottlenecks occur in memory bandwidth rather than computational performance itself.

Therefore, to achieve the theoretical maximum performance of a GPU, data layouts must be designed to align with hardware transaction units and cache lines.
