# G1GC OpenJDK 8 and 11

Although G1GC can be used in OpenJDK 8, I did not use it.

The value I currently want to gain by abandoning parallel GC is to prevent the Long Tail phenomenon.

Specifically, I wanted to eliminate the situation where 1% of calls return responses late due to heavy GC STW.

While G1GC has the advantage of allowing users to specify and control the target pause time, in OpenJDK 8's G1GC,

if a Full GC occurs, it scans the entire heap region, much like a serial GC.

### OpenJDK 8

**Serial Full GC** This is the most critical issue. Although it operates concurrently, if the memory reclamation rate cannot keep up with the allocation rate, an Evacuation Failure occurs, leading to a Full GC.

The phenomenon is that G1 Full GC in JDK 8 operates in a single-threaded manner.

For example, even if the heap is 32GB and there are 64 CPU cores, only one core is used to clean up memory. This can lead to STW events lasting tens of seconds, or even minutes. Of course, I'm not currently using a heap this large, so it might only be a few seconds at most, but even that is too long.

Secondly, there is also the problem of **excessive memory consumption by RSet**.

In JDK 8, RSet sizes were not managed efficiently. G1GC divides the heap into regions and uses a separate data structure called RSet to track references between regions.

It was common for it to consume an additional 15-20% of the heap size in native memory.

**Data Structure Cause**: When tracking "points into" (who references me), RSets in regions containing popular "hot objects" with many references would grow explosively.

There was also **difficulty in tuning and unstable latency**. Even when a target response time (-XX:MaxGCPauseMillis) was set, it was common for the number of regions to collect (CSet) during the Mixed GC (Old generation collection) phase to be miscalculated, exceeding the target time. This was because once collection started, it couldn't be stopped.

<br>

### Improved Versions JDK 10 ~ 17

Improvements occurred in JDK 10 (JEP307) and JDK 12 (JEP344).

#### Parallel Full GC (JDK 10, JEP307)

When a Full GC occurs, it was changed to perform Mark-Sweep-Compact in parallel using all available CPU cores.

There was also an algorithmic change: the existing Serial algorithm was replaced with a parallel algorithm that partitions the heap, allowing multiple threads to simultaneously mark and compact live objects.

There are also quantitative improvements:
- Theoretically, it becomes faster by a factor of the number of cores used.
- There are cases where, based on actual benchmarks (SPECjbb), Full GC time in a 16-core environment was reduced from 18 seconds to 1.5 seconds, an approximately 12-fold reduction.

#### Abortable Mixed Collections (JDK12 JEP 344)

When G1 constructs the Collection Set (list of regions to collect), it divides them into mandatory and optional regions.

If it appears that the target time (`MaxGCPauseMillis`) will be exceeded during GC execution, it stops collecting optional regions and returns immediately.

Eliminated phenomenon: Latency spikes due to prediction failures have been dramatically reduced.

Quantitative figures: P99 (top 1% slowest requests) latency decreased by approximately 40-60% in JDK 11/17 compared to JDK 8.

#### Promptly Return Unsused Committed Memory JDK 12

JDK 8 G1, once it acquired heap memory from the OS (committed), did not readily return it to the OS even if heap usage decreased.

Starting from JDK 12, memory is actively returned to the OS when idle.

This change significantly increased resource efficiency in container environments.

<br>

### Data Structures and Algorithms

Let's explore the changes in core data structures that create the performance difference between JDK 8 and recent JDK G1GC.

#### RSet (Remembered Set) Optimization

RSet stores the addresses of other regions that reference objects within a given region.

- JDK 8 Issues (Fine-grained Table)
  - An entry was added to the Hash Table every time a reference was added.
  - The coarsening logic (grouping units) was simple, and memory overhead surged if references concentrated in a specific region.
- Improvements (Sparse -> Fine -> Coarse Level Optimization)
  - Modern G1 manages RSets in 3 stages.
  - Sparse PRT (Per Region Table): A small number of references are managed with a small array instead of a hash table.
  - Bitmaps: If references become numerous, it switches to a bitmap approach to fix memory usage.
  - As a result, the amount of native memory consumed by RSet for the same workload decreased by over 50% compared to JDK 8.

Let's briefly look at HashTable vs HashMap.

Of course, the HashTable used in GC is C++ level code, but

Firstly, in Java, HashTable is synchronized with locks on all its methods, making it thread-safe but very slow in multi-threaded environments. It causes bottlenecks, cannot store null values for keys or values, and uses chaining (linking with lists) for hash collisions, which degrades search performance as data increases. It also uses the older Enumeration interface.

HashMap is unsynchronized, so it's very fast and requires ConcurrentHashMap in multi-threaded environments. It allows one key to have multiple values and permits nulls. It enables more flexible data handling. For chaining, it initially stores elements in a LinkedList, but if a bucket exceeds a certain number, it converts to a Red-Black Tree, improving search performance to O(log n). It uses the Iterator interface.

However, the HashTable in GC is a C++ concept, so it's a bit different; it doesn't have these performance degradation issues.

#### Adaptive Changes in IHOP (Initiating Heap Occupancy Percent)

JDK 8: Relied on a fixed value for `-XX:InitiatingHeapOccupancyPercent` (default 45%). It would wait until 45% was full, and if allocations suddenly surged, the response would be slow, leading to a Full GC.

In JDK 9+, with Adaptive IHOP, G1 learns the application's allocation rate and the time it takes for marking.

It autonomously decides, "At this rate, I should start marking at 30% instead of 45%." This significantly reduced "To-Space Exhausted" errors.

### Summary

1.  **Full GC Processing Method:** Serial -> Parallel, improving processing speed by 10-30 times.
2.  **Max Latency (Pause):** Spikes of several seconds on prediction failure -> Very good adherence to the configured 200ms.
3.  **Throughput Processing:** Baseline -> Baseline + 10-15% improvement. While latency-focused, CPU improvement also leads to increased throughput.
4.  **Native Memory Overhead:** 15-25% additional heap usage -> 5-10% of heap, doubling memory efficiency, thanks to RSet optimization.
5.  **String Deduplication:** Significant overhead whether off or on -> Improved String deduplication performance through more efficient hashing by default.

<br>

A question might arise here: "Doesn't G1GC also suffer P99 degradation when Full GC runs in parallel?"

It's important to understand that P99 doesn't just mean the absolute worst moment, but statistically represents how often slow responses are interspersed.

ParallelGC is a "big cleanup" approach where Major GC waits until the Old generation is full, then stops the world (STW) entirely to clear everything. It runs periodically and inevitably causes huge spikes.

In contrast, G1GC performs Mixed GC, concurrently cleaning up small portions of the Old generation before it becomes full. A Full GC in G1GC signifies a failure, an emergency situation caused by tuning errors or resource shortages.

In other words, while Full GC is a routine occurrence in ParallelGC, it's not so in G1GC; it can be considered an emergency.

In conclusion, it's true that if a Full GC occurs in G1GC, it will be as slow as in Parallel GC. However, G1GC is an algorithm designed to prevent such situations, so it rarely happens.

If it reaches that worst-case scenario, it inherently indicates another underlying problem. In such cases, you would need to retune by lowering `InitiatingHeapOccupancyPercent` to increase frequency, adding reserved regions to prevent Full GC, or preventing evacuation failures by increasing heap size.
