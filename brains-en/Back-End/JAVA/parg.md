# Parallel GC Tuning

Parallel GC is Java's representative throughput-oriented garbage collector. This means it's optimized to increase the total time an application actually performs work, rather than reducing individual pause times.

Tuning Parallel GC should primarily be approached in a goal-oriented manner.

That is, instead of directly specifying detailed memory sizes, it is more effective to present the JVM with **maximum pause time and throughput goals**, allowing the JVM to dynamically adjust the heap size accordingly.

It can be seen as a tuning mechanism somewhat similar to G1GC's adaptive IHOP functionality.

### Goal Setting

Parallel GC has very powerful automatic adjustment capabilities, often referred to as Ergonomics.

Therefore, the process involves providing hints to the JVM according to the following priorities:

1.  max pause time: the maximum acceptable pause time due to GC
2.  throughput: the ratio of application execution time (excluding GC time) to total execution time
3.  footprint: minimizing heap memory usage

The JVM strives to satisfy goal 1, then if that's met, goal 2, and then goal 3.

### Flags

Let's look at the main flags in the order of the three priorities discussed above.

#### Setting Max Pause Time Goal

This is the soft limit that should be set first. The JVM can automatically reduce the size of the young/old generations to finish GC within this time.

-   `-XX:MaxGCPauseMillis=<N>`
-   A setting that attempts to ensure GC pauses do not exceed N milliseconds.
-   If this value is set too low, the JVM may reduce heap size to meet the target, leading to excessively frequent GCs and a sharp drop in overall throughput.

https://docs.oracle.com/javase/8/docs/technotes/guides/vm/gctuning/parallel.html

> Isn't this also a tuning point in G1GC? G1 also specifies this value to set the maximum pause time.
>
> In G1GC, it selects regions to clean and cleans as many regions as possible within the target time. If the target time is set too short, it might postpone cleaning the old generation, risking a Full GC later. Its default is 200ms.
>
> In Parallel GC, this option resizes the entire young and old generations to meet the time target. If the target time is set too short, the heap shrinks, causing GC to occur frantically often. The default value is unlimited, and this constraint is the highest priority.

To summarize the difference from G1, G1 selects the number of regions to clean within the flag's value, while Parallel GC shrinks the heap to satisfy that time. Their operating mechanisms are different. Therefore, in G1, shrinking it too much is not good because the heap becomes too small.

#### Setting Throughput Goal

This is the raison d'être of Parallel GC. It sets the ratio of time GC occupies out of the total execution time.

-   Option: `-XX:GCTimeRatio=<N>`
-   Formula: The GC time ratio is `1 / (1 + N)`.
    -   The default value is 99, meaning 1/100, or 1%, of the total time is spent on GC.
    -   If N=19 is set, it becomes 1/20 = 5%.
-   If `MaxGCPauseMillis` is satisfied but this goal is not met, the JVM attempts to increase the old generation to reduce GC frequency.

#### Setting Heap Memory Size

When the above two goals cannot be resolved, physical memory limits are specified.
-   `-Xms<size>`: Initial heap size. It's generally good to set it equal to `-Xmx` to reduce the overhead of dynamic size changes.
-   `-XmX<size>`: Maximum heap size.

### Step-by-Step Tuning Process

Tuning should not involve applying all flags at once but rather observing and adjusting.

First, a monitoring environment must be established because the current state needs to be known before tuning. Enable GC logs.

`-Xlog:gc*`(Java 9+), `-XX:+PrintGCDetails -XX:+PrintGCTimeStamps` (Java 8 and below)

**Step 2: Memory Allocation**: Set an appropriate heap size for the application to run without `OutOfMemoryError`. Typically, 3-4 times the required live data size is recommended.

**Step 3: Set Pause Time Goal (MaxGCPauseMillis)** Set this according to the application's SLA.

**Step 4: Adjust Throughput Goal**: `GCTimeRatio`. If the pause time is satisfactory but GC occurs too frequently, you should adjust the throughput goal or increase the heap size.

### Adjusting Generation Ratios

Only manually adjust Generation ratios when automatic tuning does not work as intended.

-   `-XX:NewRatio=<N>`: Ratio of Young Gen to Old Gen (default 2)
    -   When N=2, the young:old ratio is 1:2.
    -   If object creation and destruction are very frequent, increasing the young generation might be advantageous.
-   `-XX:SurvivorRatio=<N>`: Ratio of Eden space to Survivor space. If it's too small, the survivor space can overflow, causing objects to be promoted directly to the Old generation.

Parallel GC tuning is suitable when throughput is the highest priority. If low latency is critical, consider switching to G1GC or ZGC.

Heap Size Fluctuation: Parallel GC can continuously change heap size to achieve its goals. In a production environment, setting `-Xms` and `-Xmx` to the same value is good practice to increase predictability by fixing the heap size.
