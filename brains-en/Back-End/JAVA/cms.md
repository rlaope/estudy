# CMS GC (Tuning Strategy: Parallel, CMS)

As its name suggests, Concurrent Mark Sweep GC

is a garbage collector that runs concurrently with the application to minimize application latency.

It was deprecated starting from Java 9 and completely removed in Java 14, but it contains important concepts that form the basis of modern collectors like G1 GC and ZGC.

### Reduce STW

This is the basic philosophy of CMS GC. Traditional Serial and Parallel GCs stop all application threads while they are running.

However, CMS breaks down the entire process, causing STW only in some phases, and runs most of the remaining phases concurrently with the application to improve responsiveness.

#### Combination of Young GC and Old GC

CMS GC primarily manages the old generation, and the young generation typically uses a multi-threaded copying GC called ParNew GC.

- **ParNew**: When a minor GC occurs, surviving objects are moved to the survivor space, causing STW.
- **Old GC CMS**: When cleaning up objects in the old space, it goes through a four-phase process.

### CMS GC Operation: 4 Phases

1.  **Initial Mark (with STW)**
    1.  It has the characteristic of finding only objects directly referenced by GC roots. Since it only finds very close objects, the STW time is short.
2.  **Concurrent Marking (no STW)**
    1.  The purpose is to trace all reference relationships by following the objects that survived the initial mark.
    2.  A separate GC thread performs this task while the application is running. It consumes processor resources, but the application does not stop.
3.  **Remark (with STW)**
    1.  The application may have changed object references during Concurrent Mark. This phase re-checks these changes.
    2.  A second STW occurs, which is essential to correct errors from the previous phase.
4.  **Concurrent Sweep (no STW)**
    1.  It removes unreachable objects from memory.
    2.  Cleaning proceeds while the application is running.

### Limitations of CMS GC and Full GC

CMS pays a high price for responsiveness.

There is a **memory fragmentation** problem. By default, CMS does not perform a compaction phase.

This means it doesn't fill in empty spaces in between, leaving them as they are. Over time, even if the total memory is sufficient, it becomes fragmented, lacking space for large objects.

**Concurrent Mode Failure (cause of Full GC)** also exists. GC should finish before the old space runs out. If the object allocation rate is faster than the GC rate, a concurrent mode failure occurs.

**Fallback serial old gc**: If space shortage due to fragmentation or exceeding allocation speed occurs as described above, CMS gives up and performs a full GC.

At this time, the full GC operates as a single-threaded serial old GC, causing a very long STW, which leads to a decrease in overall system performance.

### CMS = Parallel?

Comparing the two, both use multiple threads, but 'parallel' and 'concurrent' have distinctly different meanings in GC.

Parallel GC uses multiple threads to perform parallel GC. However, the application completely stops while GC is running. The goal is to process quickly and resume work.

CMS GC also uses multiple threads for cleaning, but the application and GC threads run concurrently. The goal is to not stop and continuously clean up little by little.

ParNew GC: CMS GC typically uses a collector called ParNew to clean the young generation. ParNew is short for Parallel Young Generation Collector. CMS borrows parallel technology for young generation cleaning, which can be confusing.

Since CMS's full GC runs serially, full GC is something to be avoided, whereas parallel GC is a routine occurrence.

Therefore, when using CMS, fragmentation must be avoided. Concurrent Mode Failure and Promotion Failure must be well detected. You must avoid situations where there is no space in the old generation for objects promoted from the young generation.

<br>

## Parallel, CMS Tuning Strategies

### Fixing and Optimizing Heap Size

This is the most basic and important step: reducing the overhead that occurs when the JVM dynamically adjusts heap size.

Set `-Xms` and `-Xmx` to the same value to prevent STW and load caused by heap size adjustments. For example, `-Xms4g -Xmx4g`.

`-XX:MetaspaceSize`, `-XX:MaxMetaspaceSize`: Since Java 8, Metaspace is used instead of PermGen. To prevent full GC caused by this area filling up, an appropriate initial value should be allocated (e.g., `-XX:MetaspaceSize=256m -XX:MaxMetaspaceSize=256m`).

### Adjusting Young-Old Ratio

In Parallel GC, minor GC is the most frequent occurrence, and according to the generational hypothesis that most objects die quickly, the young generation must be used efficiently.

-   **Adjusting NewRatio**: Determines the ratio of the young generation to the total heap.
    -   `-XX:NewRatio=2` means the old and young generations take a 2:1 ratio by default.
    -   If it's an API server with many short-lived objects, it might be advantageous to lower it to 1 to increase the young generation size.
-   **Adjusting SurvivorRatio**: The ratio of the Eden space to the survivor spaces.
    -   `-XX:SurvivorRatio=8` (Eden 8, S0: 1, S1: 1)
    -   Objects should stay in the survivor space long enough to die there, reducing the amount promoted to the old generation.

### Parallel GC Specific Tuning Parameters

Parallel GC has powerful Ergonomics, an auto-tuning feature where the JVM optimizes itself based on user-defined goals.

`-XX:MaxGCPauseMillis`: Sets the most important target value, GC pause time, to be less than Nms. If this value is too small, the heap size will shrink, and GC will occur frequently, so caution is needed. Typically, start with 200~500ms.

`-XX:GCTimeRatio=N`: Sets the ratio of time spent on GC to the total time (1 / (N + 1)). The default is 99 (1%), and if set to 19, it means up to 5% of time is allowed for GC.

`-XX:ParallelGCThreads=<N>`: The number of threads to perform GC. Usually set to the number of CPU cores, but if multiple JVMs are running on one server, set it to less than the number of cores.

### Promotion Tuning

Adjusts the criteria for objects surviving young GC to be promoted to the old generation.
-   `-XX:MaxTenuringThreshold=<N>`
    -   Determines how many times an object must survive in the survivor space before being moved to the old generation. The default is 15.
    -   To reduce full GCs in the old generation, it's good to keep this value high, but if it's too high, the survivor space might overflow.

It's also good to set up monitoring logs. Tuning should be based on data, not intuition.

For JDK 8, you can use the following options:
```bash
-XX:+PrintGCDetails 
-XX:+PrintGCDateStamps 
-Xloggc:/logs/gc.log 
-XX:+UseGCLogFileRotation 
-XX:NumberOfGCLogFiles=10 
-XX:GCLogFileSize=100M
```

### CMS Tuning

The goal of CMS GC tuning is to prevent full GC at all costs (similar to G1). CMS is vulnerable to fragmentation, so the key is to start cleaning early before the old generation fills up and to control the amount of objects promoted to the old generation.

Here are the core parameters and strategies that must be considered when tuning CMS GC in an OpenJDK environment.

### GC Start Point (Most Important)

Since CMS runs concurrently with the application, starting after the old generation is full is too late. Determining when to start cleaning based on how full it is, accounts for 80% of tuning.

`-XX:CMSInitiatingOccupancyFraction=<N>`: The default value is 68~92%. CMS GC starts when the old generation heap usage reaches N.

If it's too high, the risk of Concurrent Mode Failure increases. If it's too low, GC runs too frequently, wasting CPU. It's usually better to lower it to around 70-75% to secure free space.

`-XX+UseCMSInitiatingOccupancyOnly`: Forces the JVM to perform GC only when the `fraction` value set above is reached, without calculating the GC timing based on the situation. This is usually used together for predictability.

### Reducing Remark STW, Preparing for Fragmentation Full GC

CMS generally doesn't perform compaction, but you can decide how to compress memory only when a full GC occurs.

`-XX:+UseCMSCompactionAtFullCollection`: Determines whether to perform memory compaction when a full GC occurs. The default is true.

`-XX:CMSFullGCsBeforeCompaction=N`: Determines after how many CMS GCs compaction should occur during a full GC. For example, setting it to 0 means compaction occurs every time a full GC happens. This is essential to solve CMS's chronic fragmentation problem.

> Additionally, as discussed with Parallel GC, promotion can also be managed. It's okay to expand the young generation or increase the survival count so that more short-lived objects die in Eden.

```bash
# 1. Declare CMS usage and specify ParNew for Young GC
-XX:+UseConcMarkSweepGC 
-XX:+UseParNewGC 

# 2. Optimize GC start timing (start when 70% full)
-XX:CMSInitiatingOccupancyFraction=70
-XX:+UseCMSInitiatingOccupancyOnly

# 3. Shorten Remark phase
-XX:+CMSScavengeBeforeRemark

# 4. Perform compaction every time a Full GC occurs
-XX:+UseCMSCompactAtFullCollection
-XX:CMSFullGCsBeforeCompaction=0

# 5. Set parallel threads (adjust according to CPU core count)
-XX:ParallelGCThreads=8
-XX:ConcGCThreads=4**
```

In summary, the strategy is to start early when the old generation is 70% full and clean the young generation before the remark phase to minimize pauses.
