# jemalloc, arena, memory fragmentation

jemalloc is a malloc developed by Jason Evans for FreeBSD.

It **minimizes memory fragmentation** and provides **concurrency in multiprocessor/multithreaded environments**.

jemalloc introduces new concepts like Arena and Thread Cache, which we will explore one by one.

### Arena

An arena is a division of memory into multiple parts. By default, 16 threads (processor x 4) select an arena in a round-robin fashion.

When a thread accesses an arena for memory allocation, it uses a lock.

In other words, it's an independent memory allocator instance designed to prevent memory allocation request conflicts, maintaining its own memory pool with an internal lock.

Newly created threads are assigned an arena based on a round-robin scheme or thread-local storage.

Since threads allocate memory only from their assigned arena, lock contention is significantly reduced during parallel processing, leading to improved throughput.

<br>

### Thread Cache

For frequent small-unit memory allocations, each thread is given a "thread cache" area, allowing direct malloc calls without referencing the Arena.

This allows operations requiring small memory allocations to use the cache within the thread, reducing arena contention and improving performance.

<br>

## Memory Fragmentation

Memory fragmentation is a problem where memory space is not used efficiently due to an uneven distribution of memory usage during allocation.

Memory fragmentation issues are categorized into two types: external fragmentation and internal fragmentation.

-   **External Fragmentation**: Occurs when allocating and deallocating memory of various sizes. The total space might be large, but it's not uniformly distributed, leading to insufficient contiguous space for allocation.
    -   For example, imagine a total memory area of 100KB, with 40KB remaining (20KB used, 20KB used). If a 30KB memory allocation is needed, it cannot be allocated because no contiguous free space of that size exists.
-   **Internal Fragmentation**: Occurs when less memory is used than the allocated memory size.
    -   For instance, if 32 bytes of space are requested but only 24 bytes are actually used, 8 bytes are wasted.

Algorithms to solve external fragmentation include Buddy Allocation, and for internal fragmentation, Slab Allocation exists.

### Buddy Allocation

This algorithm solves external fragmentation by dividing memory into halves to find the best-fit memory block.

It allocates memory in powers of 2. Programmers must determine or write code to find the upper bound for x when allocating 2^x.

For example, if a system has 2000KB of physical memory, 2^10 (1024KB) would be the largest allocatable block, making x=10 the upper bound.

Since it's impossible to allocate all physical memory in a single chunk, the remaining 976KB (2000 - 1024) must be allocated in smaller blocks.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2Fk41an%2FbtqOqUQ4cJ3%2FAAAAAAAAAAAAAAAAAAAAAOIbGQYRKegmPUo5lPlC5iuoi1rJ_0d7eUDTImYkpuHJ%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1753973999%26allow_ip%3D%26allow_referer%3D%26signature%3DmR4WU86fN%252BS5lT54n5Xx5QVeXgE%253D)

Here, memory fragments are called buddies. An advantage is that they are easy to merge with their original buddies after a task completes. However, a disadvantage is that buddies can lead to internal fragmentation, and there's significant overhead when creating them due to memory splitting.

-   When memory is allocated, it searches for an appropriate-sized memory slot (a 2^k block that is at least equal to or larger than the requested memory).
    -   If an appropriate-sized memory slot is found, it's allocated to the program.
    -   If an appropriate-sized memory slot is not found, it attempts to create one.
        -   It searches for empty memory slots, dividing them in half, larger than the requested memory size.
        -   Upon reaching the lower bound, it allocates that memory (memory of the lower bound size).
        -   It returns to the first step (to find an appropriate-sized memory).
        -   This process repeats until an appropriate memory slot is found.
-   When memory is deallocated:
    -   It deallocates the memory block.
    -   It checks surrounding blocks - have they also been deallocated?
    -   If so, it combines the two memory blocks and returns to the second step, repeating this process until all deallocated memory reaches the upper bound or until it encounters surrounding blocks that are not deallocated.

> This method of deallocating memory is quite efficient, as compaction occurs relatively quickly using the most effective compaction number, such as log2(u/l) (log2(u) - log2(l)). Typically, buddy memory allocation systems are implemented using a binary tree, which means dividing memory blocks into two states: used or unused.

![alt text](./image/jemalloc1.png)

As mentioned above, while external fragmentation can be reduced, internal fragmentation cannot be prevented because memory is divided into 2^x units.

To address this, the slab allocation algorithm is used.

<br>

### Slab Allocation

This is a memory management method designed to efficiently allocate/deallocate objects of the same size. It maintains a cache for each object type and reduces the overhead of memory fragmentation initialization.

As mentioned earlier, memory is managed in cache units based on fixed object sizes/types.

A slab is a concept that includes one or more contiguous page bundles, containing regions of the same size internally.

In other words, it was conceived from the idea of "managing objects of the same size and type by grouping them into pre-defined spaces (slabs)."

-   Since only objects of the same size are managed, there is no internal fragmentation.
-   Objects always reside in the same location, leading to a high cache hit rate.
-   Memory is pre-divided in a fixed manner, eliminating the need for dynamic partitioning.

```
[ Cache (객체 타입별)]
  ├─ [ Slab 1 (4KB, 일부 사용 중) ]
  ├─ [ Slab 2 (4KB, 모두 사용 중) ]
  └─ [ Slab 3 (4KB, 모두 비어 있음) ]
```

Huge objects (chunk-aligned) are allocated and managed in a red-black tree, allowing them to be found in O(log n) time.

jemalloc allocates small or large allocations to chunk page runs using the buddy algorithm and slab algorithm.

To improve performance, a high cache hit rate is essential, and programs designed with good locality exhibit strong performance.

These techniques can be seen as having effectively leveraged these principles to evolve into a high-performance memory allocator.

There's a rumor that applying jemalloc to Firefox resulted in approximately a 22% reduction in memory usage.

Traditional ptmalloc allocates memory via brk() and mmap(), internally using a bin chunk arena structure. However, it's fundamentally a global lock-based structure, leading to significant lock contention and memory fragmentation issues in multi-threaded environments. Therefore, if you're developing high-performance applications, consider jemalloc.

It's also said to provide more detailed information during profiling, but this seems to be a matter of personal preference.
