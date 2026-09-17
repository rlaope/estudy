# Why did G1GC choose SATB instead of Incremental Update?

### G1GC's core is Region live ratio -> Needs a clear snapshot

G1GC mixed GC selects regions based on the following criteria:

```
old region의 live ratio (live bytes %)
```

With Incremental Update, the object graph changes very rapidly during concurrent marking, making it difficult to calculate an accurate live ratio.

On the other hand, SATB:
- fixes the snapshot
- stably calculates the region live set at the end of marking
- makes mixed GC candidate selection more accurate
- improves pause time control performance

In other words, it aligns well with G1's core design.

### Incremental Update has very high remark phase costs

Incremental Update must process all new references that changed during marking.

This is also why CMS has a long remark phase.

G1 is a collector designed to minimize remark pauses; if the remark phase is long, pause time control becomes impossible.

SATB only needs to record changed old references, so the remark phase is short, performing only buffer flushing + minimal adjustments.

Therefore, a short remark pause is crucial, leading to the adoption of SATB.

### SATB is more natural given the characteristics of Evacuation-based GC.

Evacuation GC is a strategy that copies objects and then discards old regions.

Marking-based information is very important in this process.

With Incremental Update, reference changes continue to occur even during copying, making it difficult to maintain marking consistency.

SATB maintains a snapshot from the start of marking, making reference updates much more stable during evacuation.

### Efficient from a Write Barrier perspective

Incremental Update's write barrier performs the following tasks:
1. records new references
2. expands the graph along this new reference chain
3. requires subsequent correction to complete the reachable graph

SATB's write barrier has relatively stable costs.

```
SATB: enqueue(old_ref)
Incremental Update: enqueue(new_ref) + 추가 follow-up graph scan
```

1. Region-based live ratio calculation becomes accurate.
2. Remark pause can be subtly reduced.
3. Consistent with evacuation-based collection.
4. Philosophically aligns well with pause-time control strategy.
