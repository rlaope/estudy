# Four Types of Garbage Collectors

In a garbage collector, "garbage" refers to objects that are no longer in use and are only occupying memory, which need to be cleaned up.

Developers do not need to, and should not, implement this object cleanup logic directly.

GC is a concept provided in various environments, and developers can configure it directly for tuning purposes.

A garbage collector, which performs GC operations, has the following roles:

1. Memory allocation
2. Identifying memory in use
3. Identifying unused memory

> I will not cover the young, old, and perm generations of GC, the process of object allocation and deallocation, or mark and sweep here. Please refer to other articles.

Two types of GC occur: Minor GC in the young generation and Major GC in the old generation. The way these two interact leads to differences in GC approaches.

When GC occurs or objects move from one generation to another, it can cause application bottlenecks and impact performance.

**Hotspot VM** uses something called TLABs (Thread-Local Allocation Buffers).

By using a memory buffer for each thread, memory allocation operations can be performed without affecting other threads.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F224A224358FF17593F)

### Four GC Approaches

JDK 5.0 and later include the following GC approaches:

These options can be applied and selected when running WAS or Java applications.

- Serial Collector
- Parallel Collector
- Parallel Compacting Collector
- Concurrent Mark-Sweep(CMS) Collector

### Serial Collector

The Young and Old generations are processed serially (sequentially), using a single CPU.

The period during which this processing occurs is called "Stop the World." (All operations being performed by the application are halted and paused.)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F2703E43958FF0CD22F)

Initially, live objects reside in the Eden space. When the Eden space becomes full, live objects are moved to the To Survivor space (the empty space). Objects that are too large to fit into the Survivor space go directly to the Old generation. Then, objects remaining in the From Survivor space move to the Old generation. This movement to the Old generation is called Promotion.

Subsequently, objects in the Old or Perm generations follow the Mark-sweep-compact collection algorithm.

Simply put, it's an algorithm that marks unused objects, cleans them up, and compacts them.

Serial collectors are generally used on client-type devices, meaning systems where long waiting times are not a significant issue.

Explicit Specification Method

```
-XX:+UseSerialGC
```

### Parallel Collector

This approach is also known as a throughput collector.

The goal of this approach is to minimize other CPUs remaining in a waiting state.

Unlike the serial collector, it processes the collection of the young generation in parallel.

By utilizing multiple CPUs, it reduces the GC overhead and improves application throughput.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F266ED54458FF183C06)

GC in the old generation uses the Mark-sweep-compact algorithm, similar to the serial collector.

Explicit Specification Method
```
-XX:+UseParallelGC
```

### Parallel Compacting Collector

The difference from the parallel collector is that it uses a new algorithm for old generation GC.

That is, GC in the young generation is the same as with the parallel collector.

GC in the old generation goes through the following three stages:

1. Mark: Marks live objects.
2. Sweep: Examines the locations of live objects in the previously compacted area after performing GC.
3. Compact: Performs compaction. After compaction, the area is divided into compacted and empty regions.

Similar to the parallel collector, this approach is also suitable for servers using multiple CPUs.

The number of threads used for GC can be adjusted with the -XX:ParallelGCThreads=n option.

Explicit Specification Method
```
-XX:+UseParallelOldGC
```

### CMS Collector

This approach is also known as a low-latency collector.

It is suitable when the heap memory area is large.

GC for the Young generation is the same as with the parallel collector.

GC in the Old generation goes through the following stages:

1. Mark: Finds live objects with very short pause times.
2. Sweep: Marks live objects concurrently with server execution.
3. Remark: This stage re-marks objects that changed during the concurrent marking phase.
4. ConcurrentSweep: This stage cleans up the marked garbage.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F24208F4958FF1D9E2B)

CMS does not perform a compaction phase, so it does not consolidate memory to the left.

Therefore, as shown in the figure, empty spaces occur after GC, so the percentage of the Old generation is specified by the n value in the `-XX:CMSInitiatingOccupancyFraction=n` option. The default value is 68.

The CMS collector approach is suitable for servers using two or more processors. Web servers are the most appropriate target.

Explicit Specification Method

```
-XX:+UseConcMarkSweepGC
```

The CMS collector supports an incremental approach as an additional option.

This means that GC in the Young generation can be further broken down to reduce server pause times.

It is good to use when there are not many CPUs and the system requires short pause times.

To perform incremental GC, you can specify the `-XX:+CMSIncrementalMode` option.

Depending on the JVM, specifying the `-Xingc` option can have the same meaning.

However, specifying this option may lead to unexpected performance degradation. Ensure thorough testing before applying it to production servers.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F2129254758FF215238)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F2511F44C58FF216E0F)

To summarize the above, GC is performed when the allocated memory size of each generation exceeds its limit and is not an area for developers to control.

As a developer, there's no need to memorize Java's GC approaches while developing or setting up servers; understanding them is sufficient. However, it is said that when needed, the appropriate GC approach should be applied to the developed system.
