# CUDA

### Compute Unified Device Architecture

CUDA is a parallel computing platform and programming model designed by NVIDIA.

In the past, GPUs were used only for graphics rendering, but CUDA is an interface that extends languages like C and C++ to allow direct hardware control, enabling their use for general-purpose computing (GPGPU - General Purpose computing on Graphics Processing Units).

- Allows programmers to handle complex GPU hardware instructions using familiar C++ syntax.
- Efficiently distributes thousands of threads to hardware execution units.
- Controls data transfer between CPU and GPU, and the GPU's internal memory hierarchy.

### CUDA Software Stack Structure

CUDA is not just a language; it consists of a multi-layered structure connecting hardware and software.

1.  **CUDA Libraries:** A collection of optimized functions (cuBLAS: linear algebra, cuDNN: deep learning, etc.).
2.  **CUDA Runtime API:** Provides high-level management functions such as memory allocation and kernel execution, primarily used by programmers.
3.  **CUDA Driver API:** Provides lower-level control closer to the hardware; the runtime API internally calls the driver API.
4.  **GPU Driver:** Manages communication between the operating system and GPU hardware.

### CUDA Compilation Process

CUDA code mixes host code, which runs on the CPU, and device code, which runs on the GPU.

To process this, a dedicated compiler called nvcc is used.

1.  Separation: nvcc opens and reads .cu files to separate host and device code.
2.  Host compilation: Host code is typically compiled by C++ compilers (gcc, msvc).
3.  Device compilation: Device code is converted into an intermediate assembly language called ptx (parallel thread execution).
    1.  Finally, it is converted into SASS (Source Absolute Set of Standard), a binary optimized for specific GPU architectures (Ampere, Hopper, etc.), and then executed.

### Kernel Execution Model

A kernel is the smallest unit of a function that runs in parallel on the GPU.

The CPU calls it, and thousands or more threads execute the same code on different datasets.

The GPU has a hierarchical structure to efficiently manage threads.

1.  **thread**: The smallest unit of computation, each having its own register and PC.
2.  **block**: A thread group hierarchy, comprising shared memory and a set of threads, allocated to the same SM (streaming multiprocessor).
3.  **grid**: A grid NDRange hierarchy, which is the collection of all blocks/thread groups executed by a single kernel call.

### Host-Device Communication

The CPU and GPU have physically separate memory spaces. (Based on Discrete GPU).

Data flow between them is a major cause of performance bottlenecks.

-   Memory Copy Latency (PCIe Overhead): In external GPU systems, data transfer uses the PCIe bus. Since the transfer bandwidth is significantly lower than VRAM bandwidth, frequent Memcpy operations can negate computational gains.
-   Unified Memory (Apple Silicon): Mac's M-series chips share the same physical RAM between the CPU and GPU.
    -   Zero Copy: Data can be shared without moving it, by simply passing pointers, resulting in almost no copy latency.
    -   Visibility: However, cache flush control is still necessary to ensure data visibility.

### Synchronization and Memory Visibility

GPUs operate asynchronously.

Immediately after the CPU issues a command to execute a kernel, it proceeds with the next command without waiting for the kernel to complete.

Therefore, synchronization is essential for accurate results.

**Thread-level Synchronization (Barrier)**
-   CUDA: `__syncthreads()`
-   Metal: `threadgroup_barrier(mem_flags::mem_threadgroup)`
-   The purpose is to prevent RAW (Read After Write) hazards that can occur when writing to and reading from shared memory.

Threads within the same block (thread group) wait until they all reach a specific point.

**Kernel-level Synchronization**
-   When the result of Kernel A is the input for Kernel B, it must be ensured that Kernel A has fully completed.
-   Command Queue: Commands are sequentially inserted into a queue to ensure execution order.
-   Fence/Events: Control execution dependencies between different queues or streams by exchanging hardware signals.
