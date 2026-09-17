# Concurrent Marking

Concurrent Marking is the process by which G1GC tracks all live objects in the heap without stopping the application.

Only after this process successfully completes can Mixed GC be performed.

In other words, it can be seen as a preliminary investigation stage for Mixed GC.

It is divided into 5 stages, and whether each stage is STW or Concurrent is key to tuning.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FbtksbS%2FbtsJyKslIOz%2FAAAAAAAAAAAAAAAAAAAAANN-zXo236hff-P6l0Ehm2mBfxLFdZmwKTdtF7184dkL%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1767193199%26allow_ip%3D%26allow_referer%3D%26signature%3Dn%252FTAni8%252F8MfsP4QYFovBxaRk5X0%253D)

The trigger for Concurrent Marking is precisely divided into two types: one based on a fixed IHOP threshold and an adaptive method where G1GC learns and decides on its own.

`-XX:InitiatingHeapOccupancyPercent` (default 45) is usually calculated after a Young GC. If the calculated ratio exceeds this value, the initial mark begins concurrently with the next Young GC.

Since JDK 9, Adaptive IHOP is enabled (`-XX:+G1UseAdaptiveIHOP`), and G1GC starts predicting instead of simply relying on the 45% number.

Measuring marking time: How long did it take in the past?

Measuring allocation time: How quickly does the application fill up memory?

After asking itself these questions, if 45 is too late, it might decide to start at 30. Conversely, if there's plenty of room, it might consider starting later than 45.

If this feature is enabled, the user-configured value is only used as an initial value, and G1GC automatically adjusts the timing.

Additionally, when a Humongous Object is allocated (an object larger than 50% of an old region), G1GC may immediately check or start a marking cycle, fearing a rapid increase in heap usage. This behavior can vary by JDK version.

It can also be triggered by Metaspace exhaustion. Even if heap memory is abundant, if the Metaspace area, which stores class metadata, becomes full and requires expansion, Concurrent Marking may start to facilitate unloading.

Or manual invocation.

### Initial Mark (초기 마킹)

**State: STW (Stop-The-World)**

This marks objects directly referenced by the Root Set (stack variables, global variables, etc.), which is the starting point of GC.

It usually piggybacks on a Young GC. So, the log shows:

```
GC pause (G1 Evacuation Pause) (young) (initial-mark)
```

Since the GC is paused anyway for Young GC, it takes the opportunity to mark roots in the old generation as well.

### Root Region Scan

**State: Concurrent (Not stopped)**

This step finds and marks objects in the Old generation that are referenced by objects in the Survivor space marked during the Initial Mark phase.

This operation must complete before the next Young GC begins. If memory fills up during this operation and a Young GC is about to occur, the Young GC might have to wait for this scan to finish, potentially causing a delay.

### Concurrent Mark (동시 마킹) - Longest Phase

**State: Concurrent (Not stopped)**

This is the main traversal. It scans the entire heap, tracing the Reference Chain of live objects to its end.

It uses the SATB (Snapshot at the Beginning) algorithm and determines live objects based on a snapshot taken at the start of marking.

Since it runs concurrently with application threads, it shares CPU resources.

If this phase is too slow and cannot keep up with the memory allocation rate, a Full GC will occur.

### Remark (재마킹)

**State: STW (Stop-The-World)**

Since the application continued to run during the Concurrent Mark phase (step 3), object reference relationships might have changed.

This phase is the final step to incorporate those changes and finalize the marking. It processes the changes recorded in the SATB buffer.

Although STW occurs, it uses multi-threaded parallel processing to complete as quickly as possible.

### Cleanup

**State: STW + Concurrent**

1.  Accounting (STW): Calculates the proportion of live objects in each region. This is necessary to identify regions with a lot of garbage for Mixed GC.
2.  RSet Scrubbing (STW): Updates the Remembered Set.
3.  Empty Region Reclaim (Concurrent): Regions that contain no live objects (100% garbage) are immediately initialized and returned to the Free List.

After the Cleanup phase, G1GC transitions from YoungGC to Mixed GC mode and begins cleaning up the Old Regions in the Candidate Set along with the Young Generation.

```log
[0.005s][info][gc] Using G1

#1
[0.167s][info][gc] GC(0) Pause Young (Normal) (G1 Evacuation Pause) 23M->3M(260M) 0.941ms
[0.308s][info][gc] GC(1) Pause Young (Normal) (G1 Evacuation Pause) 43M->4M(260M) 1.245ms

#2
[0.662s][info][gc] GC(2) Pause Young (Normal) (GCLocker Initiated GC) 152M->8M(260M) 3.542ms

#3
[0.874s][info][gc] GC(3) Pause Young (Concurrent Start) (Metadata GC Threshold) 109M->10M(260M) 3.362ms

#4
[0.874s][info][gc] GC(4) Concurrent Mark Cycle
[0.878s][info][gc] GC(4) Pause Remark 11M->11M(54M) 0.746ms
[0.878s][info][gc] GC(4) Pause Cleanup 11M->11M(54M) 0.002ms
[0.878s][info][gc] GC(4) Concurrent Mark Cycle 3.754ms

#5
[1.000s][info][gc] GC(5) Pause Young (Normal) (G1 Preventive Collection) 40M->11M(54M) 4.661ms
[1.101s][info][gc] GC(6) Pause Young (Normal) (G1 Evacuation Pause) 33M->12M(54M) 5.834ms
[1.219s][info][gc] GC(7) Pause Young (Normal) (G1 Evacuation Pause) 40M->13M(54M) 3.073ms
[1.294s][info][gc] GC(8) Pause Young (Normal) (G1 Evacuation Pause) 37M->14M(54M) 3.314ms
```

Let's analyze the logs.

`GC(0) Pause Young (Normal)` is a typical GC performed by the JVM in the Young Generation.

`G1 Evacuation Pause` indicates the process of moving live objects from the Young Generation to the Old Generation.

During this process, the GC pauses temporarily, and the evacuation operation is performed. `23 -> 3` represents the memory change after GC.

`GCLocker Initiated GC` means that GCLocker is a feature that prevents GC while JNI code is executing and can force a GC after the lock is released.

`Pause Young (Concurrent Start)` means that a Young GC occurred, and simultaneously, a concurrent mark cycle started in the old generation.

`Metadata GC Threshold` indicates that GC occurred because the metadata area reached its GC threshold. This is primarily triggered when a lot of class metadata or related information accumulates.

`Concurrent Mark Cycle`: This performs marking operations for the old generation, which is separate from Young GC and involves marking and tracking objects in the old generation.

`Pause Remark` indicates that GC was temporarily suspended during the Remark phase after the Concurrent Mark operation completed, and `Pause Cleanup` indicates a temporary GC suspension for cleanup operations.
