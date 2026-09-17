# ZGC (Z Garbage Collector)

Existing GCs (including G1) tried to reduce pause times, but they couldn't structurally limit them.

Specifically, the following problems remained:

- As the heap grows, root scan / object move costs increase.
- Compaction (object movement) is inherently close to STW.
- Even a few milliseconds of STW can be critical in low-latency systems.

ZGC's goal is clear: **limit pause times to under 10ms, regardless of heap size.**

This goal alone led to a change in the JVM memory model itself.

### ZGC's Core Design Philosophy

Unlike existing GCs, ZGC makes different assumptions.

Object addresses are stable, and object movement should be restricted during GC. Root scans should be done with STW for safety.

ZGC's premises:
- Object addresses can be variable.
- Object movement should always be possible.
- The reader pays the cost (read barrier).

Due to these premises, ZGC uses a method called Colored Pointer.

### Colored Pointer

Metadata is embedded in the pointer. ZGC encodes GC state information directly into the object pointer itself.

Conceptual 64-bit pointer structure:

```css
[ Marked | Remapped | Finalizable | Reserved | Address ]
```
- The actual memory address is in the lower bits.
- The upper bits are GC state bits.

This means that to see the GC state, you only need to read the object.

Traditional GCs required separate metadata structures to check object states (root scan, RSet, Card Table).

However, ZGC only needs to look at a single pointer, eliminating the need to search separate structures. This minimizes STW root scans.

### ZGC Read Barrier

The Read Barrier, rather than the Write Barrier, is central to ZGC.

It's a mechanism where the JVM intervenes the moment an application thread reads an object.

```
Object o = ref.field;
```

Internally, the following actions are performed:

1. Check the pointer's color.
2. If marking is needed based on the state -> perform mark. If relocation is needed? -> redirect to the new address.
3. Always return the latest address.

This operation seems to imply that reads, which are more frequent than writes, would incur a higher cost.

However, it is highly advantageous for modern CPU branch prediction and is a no-op in most cases, a cost well worth paying to eliminate STW.

ZGC's choice is to have application threads pay a small, distributed cost.

> no-op: An operation that performs no actual work, where it only checks conditions and then terminates without changing state or incurring significant cost.

### Relocation

ZGC moves objects concurrently.

- The GC Thread copies objects to a new region.
- Mapping information from old to new is maintained.
- Pointers may still point to the old address.
- The read barrier automatically remaps when accessed (different from G1's write barrier which changes references during writes).

Object movement is not immediately reflected but is delayed.

This structure eliminates the need to fix all pointers at once; only accessed objects are progressively updated. There's almost nothing to do during STW.

### Pause Time

ZGC's pause times are very short, and this is not simply due to parallel GC.

Let's look at the remaining tasks during STW, given the memory structure changes discussed above:
- Root Set Snapshot start/end
- Very short state transitions
  - Marking -> Concurrent
  - Relocation -> Concurrent
  - Remapping -> Distributed by Read Barrier

Therefore, whether the heap is 10GB or 1TB, the pause time is almost the same.

### ZGC's Heap Structure

ZGC divides the Heap not into Regions, but into units called ZPages.

ZPages are variable in size and are divided into dedicated pages for Small / Medium / Large Objects.

The compaction target is clear.

This structure facilitates easy fragmentation management and minimizes the cost of moving Large Objects.

### ZGC Limitations and Costs

CPU Cost
- The Read Barrier is not free.
- CPU usage increases in systems with high Allocation Rates.

Memory Usage
- Old + New objects coexist during Relocation.
- Peak memory usage increases.

Platform Constraints
- 64-bit is mandatory.
- Requires pointer tagging support.
- Unsuitable for embedded / low-spec environments.

### ZGC vs G1GC

| Item          | G1GC      | ZGC        |
| ----------- | --------- | ---------- |
| Pause Goal    | Soft Goal | Hard Goal  |
| Root Scan   | STW       | Almost None      |
| Object Move | STW-centric    | Concurrent |
| Barrier     | Write-centric  | Read-centric    |
| Heap Size Impact  | Significant         | Almost None      |

Therefore, ZGC is better suited for low-latency systems than G1, and less suitable for simple services with very tight CPU constraints or small heaps.
