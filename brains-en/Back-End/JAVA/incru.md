# Incremental Update

Incremental Update is an algorithm with a philosophy opposite to SATB.

Understanding the difference between these two is the watershed that distinguishes Modern GC (G1, Z) from Legacy GC (CMS).

If G1GC uses SATB (Snapshot At the Beginning, collecting based on references at the GC trigger point),

CMS uses the Incremental Update method.

SATB is past-oriented; it creates a snapshot at the start of marking and doesn't care if things change in between.

Conversely, Incremental Update (CMS) is future-oriented; if something changes during marking, it notes, "Oh? You changed. I'll re-examine you later."

### Post-Write Barrier

While SATB uses a Pre-Write Barrier (recording the value before it changes), Incremental Update uses a Post-Write Barrier (recording the state after it changes).

#### Tri-color Marking

First, you need to understand the three states of an object.
1.  **White**: Not yet scanned (garbage candidate)
2.  **Grey**: Self scanned, but references not yet scanned (in progress)
3.  **Black**: Self scanned, and references also scanned

Here's a problem: let's say the GC has completely scanned Object A and marked it Black, then moved on.

But what if the application suddenly makes Object A refer to Object C (white)? (`A.child = C`)

Then the GC won't look at A again because it's already black. As a result, C remains white despite being referenced by someone, putting it at risk of deletion.

### Incremental Update Solution: Demote to Grey.

To prevent the above problem, a write-barrier operates.

If a new reference is connected to an already scanned black object, that object is changed back to Grey to be scanned again later.

```java
// Incremental Update Logic (Pseudo code)
void write_barrier(Object src, Object new_ref) {
    // src (already scanned object) now points to new_ref (new object)
    if (is_black(src)) {
        // "Hold on, you were Black, but you got a new friend? Get re-scanned!"
        mark_grey(src); 
        // Or record it in a Dirty Card (to be re-examined during the Remark phase)
    }
    src.field = new_ref; // Actual assignment
}
```

SATB, as Snapshot At The Beginning, keeps an object alive until the end if it's alive at the start, and uses a Pre-Write Barrier to record the old version's address before the scan ends, preventing its collection. It's conservative and keeps even truly dead objects alive. However, the Remark cost is very short because it only needs to mark the old addresses accumulated in the buffer. This leads to heap memory waste but results in lower CPU usage.

Incremental Update's philosophy is to re-trace and accurately keep objects alive if their references change, using a Post-Write Barrier to record newly connected reference relationships. It is more precise and better at identifying truly live objects. However, its Remark cost is higher. The Remark time is longer because it needs to re-scan the children of Dirty objects that have been demoted to Grey.

<br>

### Why G1GC Chose SATB

The problem with Incremental Update is that during the Remark phase, STW operates by re-scanning object references in the changed state.

However, if the application changes a huge number of object references during marking, the amount of Dirty Cards that need to be re-examined increases.

Ultimately, the length of STW during the Remark phase becomes **unpredictable.** (This is one reason why CMS failed.)

In the Remark phase, SATB's task is simply to quickly mark the broken references recorded in the SATB buffer and finish.

There's no need to perform another depth-first search.

Although it creates Floating Garbage and wastes some memory, the STW time is guaranteed to be short.
