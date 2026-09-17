# NUMA (Kernel Level)

Understanding NUMA at the kernel level means understanding how the operating system abstracts the hardware's asymmetrical memory architecture and how it allocates resources to minimize performance loss.

### Kernel's NUMA Abstraction: Node, Zone

The Linux kernel uses a hierarchical structure to manage physical memory.

- Node(`pg_data_t`): This is the top-level structure in the kernel that represents a NUMA node. Each node has its own memory management structure.
- Zone: Memory within each node is further divided into `ZONE_DMA`, `ZONE_NORMAL`, `ZONE_HIGHMEM`, and so on.
- Distance(SLIT): The kernel quantifies the distance (cost) between nodes by referring to ACPI's SLIT (System Locality Information Table). If the local access cost is 10, neighboring nodes might be 20, and more distant nodes 30, for example.

### Memory Allocation Policies

The kernel provides several policies to determine which node to allocate memory from when a process requests it.

- **Local Strategy (Default)**: The process attempts to allocate memory from the node to which the currently executing CPU belongs.
- **Interleave**: Pages are allocated alternately across multiple nodes, which is advantageous for preventing bandwidth bottlenecks on specific nodes.
- **Preferred**: A specific node is preferred, but if there's no space, memory is allocated from another node.
- **Bind**: Memory must be allocated exclusively from specific nodes; if there's no space, it results in an OOM (Out Of Memory) error.

Important Concept: First Touch Policy In Linux, physical pages are allocated not when `malloc()` is called, but when **data is first written** to that memory. At this point, the memory is placed on the node to which the CPU performing the write operation belongs.

### NUMA Scheduling and Locality

The CPU scheduler must consider memory locality when determining where to run a process.

- **Task Migration Dilemma**: If the CPU scheduler moves a process to a CPU on another node for load balancing, the memory it was previously using becomes remote memory, leading to performance degradation.
- **NUMA Balancing (Auto-NUMA)**: The kernel's background thread (knumad) periodically monitors a process's memory access patterns. If a process is using more memory from another node, it performs one of the following actions:
  - Page Migration: Memory pages are moved to the node where the process resides.
  - Task Migration: The process is moved to the node where its memory resides.

### Kernel Parameters and Performance Issues

#### **Zone Reclaim Mode**

When a specific node runs out of memory, the kernel must decide whether to use available memory from other nodes (Remote Access) or to reclaim local memory by clearing the local node's cache (Reclaim).

In database environments, incorrect settings can lead to performance fluctuations due to unnecessary cache invalidation.

#### **Transparent Huge Pages (THP)**

While using large memory pages (e.g., 2MB) increases the TLB hit rate, it has the side effect of increasing overhead during page migration between NUMA nodes.

---

There are ways to check NUMA status using kernel interfaces.

Using `numactl --hardware` allows you to check the CPU/memory distribution per node and the distance between nodes.

`numastat`: This command allows you to check statistics on whether memory allocation succeeded on each node (numa_hit) or if it was pulled from other nodes (numa_miss).

`/proc/selc/numa_maps`: This allows for detailed analysis of how much memory a specific process is using from which node.
