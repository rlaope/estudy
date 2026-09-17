# Evacuation Failure

Evacuation Failure is one of the most critical situations in G1GC. It occurs when G1, which reclaims regions based on evacuation (object copying), fails during this copy phase.

The conditions under which it occurs are as follows:

#### 1. Insufficient promotion copy space

This is a situation where there are not enough free regions for the objects to be copied during the young -> survivor/old promotion process or during old region evacuation in mixed GC.
In other words, the survivor space is full, or there are insufficient free old regions, or the reserve area is lacking (emergency buffer managed by `-XX:G1ReservePercent`), or regions are fragmented due to Humongous allocations, leaving no free regions.
G1 estimates the approximate copy cost and the number of regions required before starting evacuation, but if this estimation is incorrect or the runtime state changes, it can fail due to insufficient space midway.

#### 2. RSet becomes too large, causing a timeout during the copy operation

Evacuation requires copying, reference updates, and RSet maintenance.
If the RSet becomes large, a significant number of pointer updates may be required during copying, leading to failure.

#### Object size explosion / Survivor overflow

Especially in Young GC, if many aged objects are created, they cannot fit into the Survivor space and must be promoted to the old generation, but this can also fail if there is insufficient space.

### How does HotSpot handle Evacuation Failure?

Key point: The region can no longer be emptied via evacuation and remains in its old form.

Specifically, the following processing steps occur:

1.  The failed region is marked as a pinned region.
    1.  The region that failed to evacuate is marked as an old region that can no longer be evacuated. This region contains a mix of live and dead objects and remains uncompacted.
2.  The GC processes the region as "do-not evacuate".
    1.  This region will not be targeted in subsequent mixed GCs, meaning region-level compaction, a key feature of G1 major GC, is entirely impossible for this region.
3.  Additional region allocation and fallback scenarios
    1.  HotSpot immediately attempts the following actions:
        1.  Maximize the acquisition of remaining free regions.
        2.  Change evacuation priorities for other regions.
        3.  Adjust generation ratios.
        4.  Immediately stop the GC cycle and restart remarking or a new mark cycle.
4.  If evacuation failures accumulate multiple times, a Full GC is eventually triggered.
    1.  If the number of pinned regions increases, the following phenomena occur:
        1.  Mixed GC becomes ineffective -> old live ratio is maintained.
        2.  Increased fragmentation.
        3.  Accumulation of humongous regions.
        4.  Decreased free regions.

Ultimately, if G1 determines that incremental collection is no longer possible, it falls back to Full GC.

### JDK Code

https://github.com/openjdk/jdk

```cpp
// 실패 감지 지점
// src/hotspot/share/gc/g1/g1ParScanThreadState.cpp

oop G1ParScanThreadState::copy_to_survivor_space(oop old) {
  size_t word_sz = old->size();
  HeapWord* obj_ptr = _g1h->allocate_in_next_plab(young_index, word_sz);

  if (obj_ptr == NULL) {
    // FAILED ALLOCATION → EVACUATION FAILURE
    return handle_evacuation_failure(old);
  }

  oop new_obj = cast_to_oop(obj_ptr);
  Copy::aligned_disjoint_words(cast_from_oop<HeapWord*>(old), obj_ptr, word_sz);

  return new_obj;
}

// evacuation 처리 핵심
// src/hotspot/share/gc/g1/g1ParScanThreadState.cpp pinned region 처리함
oop G1ParScanThreadState::handle_evacuation_failure(volatile oop old) {
  // Mark the region as failed
  _g1h->evacuation_failure_occurred();

  HeapRegion* from_region = _g1h->heap_region_containing(old);

  from_region->set_evacuation_failed();

  // The object stays in place (NOT copied)
  return old;
}

// heap level처리도 함
// src/hotspot/share/gc/g1/g1CollectedHeap.cpp
void G1CollectedHeap::evacuation_failure_occurred() {
  _evacuation_failed = true;
}

// pinned 표시 2222
// src/hotspot/share/gc/g1/heapRegion.hpp
void set_evacuation_failed() {
  _evacuation_failed = true;
}

// 실패 후 최종처리
// src/hotspot/share/gc/g1/g1EvacFailure.cpp
void G1EvacuationFailure::register_failure(HeapRegion* hr) {
  hr->set_evacuation_failed();
}

void G1EvacuationFailure::do_evacuation_failure() {
  // Called after GC work to process failed regions
  for (HeapRegion* hr : _failed_regions) {
    hr->set_evacuation_failed();
  }
}

// 실패 이후 클린업
// src/hotspot/share/gc/g1/g1CollectedHeap.cpp
void G1CollectedHeap::handle_evacuation_failure() {
  if (_evacuation_failed) {
    // Reset RSet refinement
    clear_rsets();
    // Recompute region liveness
    rebuild_region_sets();
  }
}

// 누적시 full gc fallback
if (_evacuation_failed) {
  // Fall back to Full GC in severe cases
  do_full_collection(false);
}
```

```
copy_to_survivor_space → allocation failure → handle_evacuation_failure
→ region.set_evacuation_failed()
→ heap.evacuation_failure_occurred()
→ G1EvacuationFailure::do_evacuation_failure()
→ rebuild_region_sets()
→ Full GC fallback if necessary
```
