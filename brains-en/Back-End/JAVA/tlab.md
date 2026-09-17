# TLAB (Thread-Local Allocation Buffer)

TLAB is a dedicated allocation area that each thread pre-allocates from the young generation.

Within this area, object creation is extremely fast, achieved by simply incrementing a pointer without locks.

`eden` is originally a contiguous allocation space, and young `eden` allocates objects sequentially into empty space.

The simplest model uses a single global pointer:

- `eden_top`: position for the next object
- `eden_end`: end of `eden`

Each time an object is created, its size is calculated, and if `(eden_top + size <= eden_end)` is true, then `addr = eden_top; eden_top += size` is performed.

This is **bump-pointer allocation**. It's fast because there's no free list traversal; it just increments a pointer.

The problem is that in multi-threaded environments, `eden_top` is shared, which can lead to race conditions. This requires a CAS operation for every allocation, or retries if contention is high.

If multiple cores alternately use the same cache line, it can be invalidated again. This is called cache line bouncing.

**In other words, whether using locks or CAS, allocation slows down due to contention over shared pointer updates.**

This is the essence of lock overhead, and perhaps contention for any shared resource incurs overhead, not just mutexes.

### TLAB

TLAB fragments and distributes `eden` to eliminate this shared pointer contention.

The JVM carves out small segments of `eden` on a per-thread basis.

Globally, `eden` is like a large plot of land, and each thread leases a segment of that land. That is TLAB.

Each thread only has the following three pointers for its own TLAB:
- `tlab_start`
- `tlab_top`
- `tlab_end`

The important point is that these pointers are **only manipulated by that specific thread.**

Therefore, operations like `tlab_top += size` don't require atomicity or locks. It's simply incrementing a value in a register or local cache.

The implication of incrementing a pointer without locks is that there's no need to update a global shared variable for every allocation, no CAS loops or lock contention, and cache line bouncing is significantly reduced.

- Calculate size
- `new_top = tlab_top + size`
- `if (new_top <= tlab_end)`
  - `addr = tlab_top; tlab_top = new_top`
- Set header (mark word/class pointer) and return.

Object creation works as described above. This loop is very fast because there are no shared resources competing with other threads.

In other words, to eliminate heap allocation pointer contention in multithreaded environments, each thread is pre-allocated a separate buffer to perform operations exclusively within it.

<br>

### Fast Path vs Slow Path

The Fast Path is for allocations within the TLAB; it's lock-free, CAS-free, and very fast. Most small objects go here.

The Slow Path involves TLAB re-allocation (refill) or outside TLAB allocation, and it's taken in the following situations:
1. Insufficient TLAB space: requiring a new TLAB to be carved out from `eden` (refill).
2. Large objects that are awkward to fit into the TLAB: are allocated directly outside the TLAB (using global pointer CAS).
3. Insufficient `eden` space: which can ultimately increase pressure to trigger a young GC.

Therefore, from a performance perspective, whether TLAB is effective is determined by how much allocation stays on the fast path.

### Relationship with GC, and Waste

TLAB is not a separate area collected by the GC.

TLAB is merely a portion of `eden` used exclusively by a thread. When a young GC occurs and the entire `eden` is cleaned up, objects within the TLAB are processed just like regular `eden` objects.

Here, the concept of TLAB waste can arise. TLABs are given to threads in chunks. However, if a thread stops allocating just before a GC, the end portion of the TLAB remains unused.

As a result, fragments are left within `eden`, and this is called TLAB waste. The JVM dynamically tunes the TLAB size to prevent this waste from becoming excessive.

(adjusting based on allocation rate, number of threads, refill frequency, waste percentage, etc.)

<br>

### Large Objects and TLAB (Distinguish from Humongous)

Here, it's important to distinguish between two main concepts: allocation outside the TLAB and G1's Humongous Objects.

#### Allocation Outside TLAB (outside TLAB)

This occurs when an object is simply too large to fit into the TLAB, or when it's inefficient due to alignment or other conditions, and thus enters `eden` via global allocation.

This is an allocation path perspective, independent of GC algorithms (G1, Parallel, etc.).

#### Humongous Object

In G1, if an object exceeds a certain percentage of the region size (usually 50% or more), it's a policy to allocate it in a separate humongous region, rather than a regular young (`eden`) allocation.

Humongous is a G1 region policy, while outside TLAB is an allocation path. Large objects often become outside TLAB allocations, and among those, in G1, if they meet certain conditions, they might go to a humongous region.

### TLAB by GC Algorithm

#### Serial / Parallel / CMS / G1

- Common: TLAB is used for allocation optimization on the application (mutator) side.
- Difference: While the young GC methods (copying/regions/parallel) differ, the essence of TLAB as **fast per-thread allocation** remains the same.

#### Note: Distinguish from PLAB during GC Copying

When the GC copies live objects to another location, a buffer is also used, which is typically referred to as **PLAB (Promotion Local Allocation Buffer)**.

- **TLAB**: When application threads create new objects.
- **PLAB**: When GC threads copy objects during evacuation/promotion. Their purposes are different.

<br>

## Debugging TLAB

Is it slow because of TLAB? Let's look at a few ways to check this.

You can get a quick sense by looking at the following JFR events:
- `ObjectAllocationInNewTLAB` (allocation after receiving a new TLAB)
- `ObjectAllocationOutsideTLAB` (allocation outside TLAB)

If `ObjectAllocationOutsideTLAB` is unusually high, you can suspect:
- Objects are generally large.
- TLAB size/tuning is incorrect.
- Large arrays/buffers are frequently created in specific code paths.

You can check logs using JVM flags like `-XX:+PrintTLAB` (prints TLAB statistics), `-XX:+PrintGCDetails` (viewed with detailed GC logs). From JDK 9+, `-X:log:gc+tlab=debug` (adjust trace/debug level depending on the environment), etc.

In the output, you can check refill count (if too frequent, TLAB is too small or allocations are too many), waste (if too large, TLAB is excessively big or thread patterns are unfavorable), and the outside TLAB ratio (large objects/unusual allocation patterns).

- If refills are too frequent -> Fast Path is frequently interrupted -> potential performance loss.
- If waste is too large, `eden` is wasted -> which can increase young GC pressure.
- If `outside TLAB` allocations are numerous, there's a high chance of many large object/array/buffer creations, so check this.

- `-XX:+UseTLAB`: Usually enabled; disabling it generally leads to performance loss.
- `-XX:TLABSize`/ `-XX:MinTLABSize`: Related to fixed/minimum size.
- `-XX:TLABWasteTargetPercent`: TLAB waste allowance percentage. Changing it too aggressively can have adverse effects, such as excessive CPU usage.

To summarize,

you can find the cause of outside allocations using JFR Allocation Profiling.

Try modifying your code to reduce or reuse the creation of large objects, byte buffers, arrays, JSON, strings, etc., and *then* adjust TLAB parameters.

In most tuning scenarios, while there might be inefficiencies caused by incorrect VM settings, waste at the application code level is usually a more significant factor.

Keep this philosophy in mind when pursuing performance improvements.
