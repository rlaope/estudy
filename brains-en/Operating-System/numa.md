# NUMA

NUMA (Non-Uniform Memory Access) is a memory architecture in multiprocessor systems where **the time it takes to access memory varies depending on the physical location of the processor and the memory.**

In the past, all processors used the UMA (Uniform Memory Access) method, where they accessed memory via a single shared bus. However, NUMA emerged to address the performance bottlenecks that arose as the number of processors increased.

### NUMA

In a NUMA architecture, the system is divided into multiple NUMA Nodes. Each node includes one or more CPUs and local memory.

-   **Local Memory**: This is when a CPU accesses memory within the node it belongs to. Since the distance is short and a dedicated path is used, the speed is very fast.
-   **Remote Memory**: This is when a CPU accesses memory located in another node. It must pass through an interconnect connecting the nodes, which introduces latency and results in relatively slower speeds.

In UMA, all CPUs connect to shared memory via a common bus, and access times are uniform for all memory regions. Consequently, as the number of CPUs increases, bus bottlenecks occur. It is used in personal PCs or small-scale servers.

In NUMA, CPUs and memory are grouped into nodes. Access times vary depending on location (local is fast, remote is slow), and scalability is achieved by adding nodes. It offers high scalability and is used for large-scale workload servers, HPC, and cloud environments.

![](https://netmarble.engineering/wp-content/uploads/2022/03/image6-1.png)

### NUMA Advantages and Disadvantages

Advantages include **high scalability**, allowing the construction of large systems with hundreds of CPUs, and **increased bandwidth**. Each node has its own memory bandwidth, which improves the overall data processing capability of the system.

Disadvantages include **performance imbalance**. If a program frequently references remote memory, performance can degrade sharply. **Optimization is essential**, requiring NUMA-aware optimization at the operating system or application level to place data and processes on the same node.

Most modern server-grade CPUs, such as Intel Xeon and AMD EPYC, internally use a NUMA architecture.

-   **Virtualization (VMWare, KVM)**: Optimizes performance by configuring virtual machines not to allocate resources across multiple NUMA nodes (this is also NUMA-aware).
-   **Databases**: SQL Server, Oracle, and others have built-in features to optimize memory allocation by recognizing the NUMA architecture.
