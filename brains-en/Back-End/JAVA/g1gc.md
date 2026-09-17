# G1GC

Let's delve into G1 GC in detail.

G1GC is a server-style garbage collector designed for multiprocessor systems with large amounts of memory.

G1GC is an improved GC designed to handle the increasing size of process memory on servers.

While traditional GCs were implemented to avoid extreme Stop-the-World pauses, G1GC achieves real-time performance by minimizing Stop-the-World (PauseTime) to avoid FullGC as much as possible.

- Aims for high throughput and low Stop-the-World pauses.
- Planned as an improvement over CMS.
- Focuses collection on heap regions with a high proportion of garbage.
- It has been the default since Java 9 and can be manually activated using the `-XX:+UseG1GC` option.

G1 primarily focuses on collecting and compacting reclaimable areas, i.e., areas expected to contain a lot of garbage. This is why it's named Garbage First, or G1.

| Term | Description |
|---|---|
|Evacuation|Refers to the copying and moving of objects that occurs in G1GC.|
|Region|A fixed-size division of the heap memory area managed by G1GC.|
|Humongous Region|If a newly allocated instance exceeds half the memory of a single region, it is called a Humongous Region and becomes an area separately managed by G1GC.|
|Available/Unused Region|An area where nothing is allocated, becoming a target for evacuation during the Evacuating phase.|
|CollectionSet (CSet)|The set of Regions (targets) where GC will be performed. All data within the CSet is emptied during GC (copied or moved). The set of Regions can consist of Eden, Survivor, and Old Generation. The CSet occupies less than 1% of the JVM.|
|Remembered Set (Rset)|A data structure used to remember which Region an object with a reference is in. There is one RSet per Region, enabling parallel and independent collection of Regions. It tracks the location of cross-region references to avoid a single Old generation Region = one Region referencing another. The RSet occupies less than 5% of the total memory.|
|MixedCollection|When GC occurs in both Young and Old areas, it is called Mixed GC or MixedCollection.|

### G1 Heap Allocation

![](https://wwz-frontend-asset.s3.ap-northeast-2.amazonaws.com/techblog/sandbox/aW1hZ2U%3D%287%29.png)

Unlike previous GCs, it does not divide each Generation with a fixed memory size in the Heap area; instead, it manages the heap by dividing it into Regions of uniform size.

`-XX:G1HeapRegionSize`: The JVM heap can be divided into 2048 regions, and this option allows specifying a size between 1MB and 32MB.

Newly defined Humongous and Available/Unused areas exist.

G1 copies objects from one or more Regions within the heap to a single Region, compacting/deallocating memory in the process.

Through parallel operation on multiprocessors, it reduces STW time and increases throughput.

### Operation Method

The object collection operation method of G1GC is as follows.

![](https://wwz-frontend-asset.s3.ap-northeast-2.amazonaws.com/techblog/sandbox/aW1hZ2U%3D%288%29.png)

When FullGC is performed, it goes through the steps in the order of Initial Mark - Root Region Scan - Concurrent Mark - Remark - Clean up - Copy. To reduce STW time, it performs parallel GC where each thread takes its own region and works on it.

1. Initial Mark: Finds Survivor Regions referenced by objects existing in Old Regions. STW occurs during this process.
2. Root Region Scan: Performs a scan for GC target objects in the Survivor Regions found during Initial Mark.
3. Concurrent Mark: Scans Regions across the entire heap, excluding Regions where no GC target objects are found from subsequent processing.
4. Remark: Triggers STW and finally identifies objects to be excluded from GC (i.e., live objects).
5. Clean up: Triggers STW and performs removal of unused objects in Regions with the fewest live objects. After STW ends, Regions completely emptied during the previous GC process are added to the Freelist for reuse.
6. Copy: Copies live objects from Regions that were GC targets but not completely emptied during the Cleanup phase to new (Available/Unused) Regions, performing compaction.

> G1GC Marking
> It performs marking using the SATB (Snapshot-At-The-Beginning) algorithm. It marks only live objects (a snapshot) immediately after the pause, not during it, so objects that die during marking are still considered live. The response time of the Remark phase tends to be faster compared to other GCs.

### GC Cycle

![](https://wwz-frontend-asset.s3.ap-northeast-2.amazonaws.com/techblog/sandbox/aW1hZ2U%3D%289%29.png)

Depending on the perspective, it manages memory in the young/old areas as a GC composed of 2 or 3 phases.

The Young-only Phase is where the Young Generation occurs in typical GC (including promotion to Old).

Young-only Phase with Initial Mark: A phase that performs a Concurrent Marking Cycle simultaneously with YoungGC. Specific conditions must be met to enter this phase.

MixedCollection is performed until specific conditions are met.

- G1HeapWastePercent
    - `The allowed unreclaimed space in the collection set candidates as a percentage. G1 stops the space-reclamation phase if the free space in the collection set candidates is lower than that.`
- G1MixedGCLiveThresholdPercent
    - `Old generations with a live object occupancy higher than this value are not collected during the space-reclamation phase.`
- G1MixedGCCountTarget
    - `The expected length of the space-reclamation phase in a number of collections.`
