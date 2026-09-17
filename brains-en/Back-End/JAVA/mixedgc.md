# Mixed GC

Mixed GC performs Old Region cleanup tasks on top of Young GC, which might seem similar to Old GC but is different.

It's a phase that simultaneously collects the entire Young Generation area (Eden + Survivor) plus some old regions with a lot of garbage.

Mixed GC does not occur suddenly; it can only run after a Concurrent Marking Cycle has preceded it.

1. IHOP (Initiating Heap Occupancy Percent) Reached (Trigger): When heap usage exceeds a certain threshold, typically 45%, Concurrent Marking begins.
2. Concurrent Marking: During application execution, a background thread **identifies which old regions have a lot of garbage**. At this time, regions with a low proportion of live objects are identified.
3. Remark & Cleanup: Marking is finalized, and empty regions are immediately reclaimed.
4. Mixed GC Start: Now, whenever G1GC performs a YoungGC, it incrementally includes the previously identified garbage-heavy Old Regions (Candidate Set) for cleanup.

Through this process, memory in the old generation can be reclaimed without performing a Full GC (a long STW operation).

### How it Works

The appealing aspect of Mixed GC is that it doesn't clean everything up at once.

With **Incremental Collection**, G1GC does not process all candidate old regions (Candidate Set) to be reclaimed in a single GC cycle to adhere to the target pause time `MaxGCPauseMillis`.
- If there are 100 old regions to reclaim, they are processed across 8 (default) mixed GC cycles.
- 1st MixedGC: Young Gen + 12 Old Regions
- 2nd MixedGC: Young Gen + 13 Old Regions
- ... Repeat

By processing in this divided manner, the goal is to keep Stop-the-World times short.

### Key Tuning Points for Mixed GC

These are key parameters for improving Mixed GC efficiency and preventing Full GC.

#### When to Start (Trigger)

`-XX:InitiatingHeapOccupancyPercent` (default 45): Concurrent Marking begins when 45% of the total heap memory is occupied.

For applications where memory fills up quickly: The value should be reduced, perhaps to around 40, to start marking earlier. This allows Mixed GC to clear the heap before a Full GC occurs.

If set too low, unnecessary marking operations and Mixed GCs will occur too frequently, wasting CPU resources.

#### How Efficiently to Select Regions (Efficiency)

`-XX:G1MixedGCLiveThresholdPercent` (default 85)
- If the percentage of live objects within an Old Region is above this threshold, that region is excluded from Mixed GC.
- This is because if there are too many live objects, the cost of evacuating them to another location is high, and the amount of memory to be reclaimed is small.
- Lowering this value means only highly efficient regions with a lot of garbage are selected for cleanup, which reduces GC time but might slow down the old generation reclamation rate.

#### How Many Times to Divide the Work? (Latency)

`-XX:G1MixedGCCountTarget` (default 8)
- After Concurrent Marking, this sets the target for how many Mixed GC cycles the identified garbage regions will be processed across.
- Increasing this value reduces the processing load of a single Mixed GC, shortening the Pause Time, but it takes longer for the Old Gen cleanup to complete.

#### When to Stop? (Termination)

`-XX:G1HeapWastePercent` (default 5)
- This means that about 5% of the total heap can be wasted as garbage.
- During repeated Mixed GCs, if the reclaimable garbage amount falls below this value, Mixed GC stops and reverts to Young mode.
- Increasing this value allows Mixed GC to terminate early, reducing unnecessary operations. This trades a slight memory waste for performance gains.

### Tuning Recommendations

If Full GC occasionally occurs, lower `InitiatingHeapOccupancyPercent` to start marking earlier.

If Mixed GC time is too long, increase `G1MixedGCCountTarget` to break down the work into smaller pieces, or lower `G1MixedGCLiveThresholdPercent` to clean only cost-effective regions.

If CPU usage is too high, check if Mixed GC is occurring too frequently, and consider increasing `InitiatingHeapOccupancyPercent`.

### vs. YoungGC

YoungGC deals with newly born objects, so it doesn't know which ones will live long; it attempts to copy all of them. If an evacuation failure occurs, it leaves them in place to be handled by Mixed GC and promotes them to the old generation.

At the time of Mixed GC, it tries to process these again. In the area where an evacuation failure occurred, a significant portion of the objects that were left in place will have died and become garbage after some time. (For example, if 100 objects couldn't be copied in an old region promoted due to evacuation failure, and 90 of them have since died.) G1 can now process this with much less cost, as it only needs to move the few remaining live objects. It's like Young GC failing but passing it on to Mixed GC with the thought that they'll eventually die.

YoungGC operates like "move everything in Eden!", whereas during the MixedGC Concurrent Marking process, it grades regions. For example, Region A has a 90% survival rate and 10% garbage, while Region B has a 10% survival rate and 90% garbage. It can then decide, "Okay, I should collect Region B first, as I can reclaim 90%."

In other words, Mixed GC is more efficient because it prioritizes cleaning regions with a lot of garbage (garbage first).

The areas that were left in place earlier are likely to have a high garbage ratio because they haven't been cleaned, making them a prime target for Mixed GC and thus very efficiently reclaimed.

> Note: While Mixed GC is triggered based on IHOP, Young GC is simply triggered when Eden is full and allocation pressure occurs. Since Concurrent Marking's cleanup phase determines the start of Mixed GC, if Eden is full, Mixed GC is initiated to clean Eden + a few high-scoring old regions together.
> It's obvious that Young GC is faster because it only cleans Eden, making its scope narrower and thus faster than Mixed GC, which also cleans old regions.

$$Time(Mixed) = Time(Young) + Time(Old\_Evacuation)$$
