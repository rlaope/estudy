# Humongous Object, Mixed GC Trigger

Let's review.

### What is a Humongous Object?

G1GC manages the heap by dividing it into regions of equal size.

An object whose size exceeds 50% of the region size is called a humongous object.

Criterion: If the region size is 8MB, all objects exceeding 4MB become humongous objects.

Unlike regular objects, they are not allocated in Eden and do not go through the copying process, because the object is too large and the copying cost is too high.

### Where are they placed?

Humongous objects are placed in a dedicated space called a humongous region.

Logically, a humongous region **belongs to the old generation.** If an object's size is larger than a single region, it occupies multiple contiguous regions.

> It appears as Old gen, but I recall it being managed as a separate area. I should check `jcmd jvm info` and look into the JDK source.

### +Humongous

Although humongous objects logically belong to the old generation, they are specially managed in a separate area called a humongous region, unlike regular objects.

Based on OpenJDK 11 to 17 code,

In `src/hotspot/share/gc/g1/heapRegion.hpp`, you can see:
- StartsHumongous (SH): The region containing the start of the object.
- ContinuesHumongous(CH): Information about contiguous regions used when an object is large and extends into subsequent regions.

In the allocation logic, if you look at `G1CollectedHeap::attempt_allocation_humongous`, you can see it takes a separate path, unlike `attempt_allocation` for regular objects.

`src/hotspot/share/gc/g1/g1CollectedHeap.cpp`

https://github.com/openjdk/jdk/blob/master/src/hotspot/share/gc/g1/g1CollectedHeap.cpp

```cpp
// g1CollectedHeap.cpp 예시 (의사 코드)
HeapWord* G1CollectedHeap::attempt_allocation_humongous(size_t word_size) {
    // 1. 몇 개의 리전이 필요한지 계산
    uint num_regions = humongous_obj_allocate_find_first(num_regions, word_size);
    // 2. 연속된 리전 확보 시도
    // 3. 해당 리전들을 Humongous 타입으로 설정
}
```

This method finds as many contiguous empty regions as needed. While regular objects can go into any empty region, humongous objects are large and require contiguous space, like train cars.

The types are also separated: G1RegionType `src/hotspot/share/gc/g1/g1HeapRegionType.hpp`

https://github.com/openjdk/jdk/blob/master/src/hotspot/share/gc/g1/g1HeapRegionType.hpp

```cpp
// 리전 타입을 정의하는 부분에서 Humongous를 별도로 구분합니다.
bool is_humongous() const { return _type >= HumongousMask; }
bool is_starts_humongous() const { return _type == StartsHumongous; }
bool is_continues_humongous() const { return _type == ContinuesHumongous; }
```

`G1CollectedHeap:eagerly_reclaim_humongous_regions` is why Humongous objects are treated specially even though they are in the old gen. Normally, the old gen is only cleaned up after a full GC or concurrent marking, but humongous objects can be collected even during a young GC if there are no references to them.

In `src/hotspot/share/gc/g1/g1CollectedHeap.cpp`, this method is called during young GC to immediately deallocate humongous objects with empty RSets. This is a strategy to quickly free up memory for large objects.

However, it seems these codes have been moved to https://github.com/openjdk/jdk/blob/master/src/hotspot/share/gc/g1/g1YoungCollector.cpp, where the following functions can be found:

```cpp
void G1CollectedHeap::eagerly_reclaim_humongous_regions() {
  assert_at_safepoint_on_vm_thread();

  // 1. Eager Reclaim 옵션이 꺼져있거나 후보가 없으면 바로 리턴
  if (!G1EagerReclaimHumongousObjects ||
      _humongous_reclaim_candidates.is_empty()) {
    return;
  }

  G1TraceEagerReclaimHumongousObjects tlog(_humongous_reclaim_candidates.length());

  // 2. 클로저(Closure)를 생성하여 힙 리전을 순회하며 조건에 맞는 객체를 찾음
  G1EagerlyReclaimHumongousObjectsClosure cl;
  heap_region_iterate(&cl);

  // 3. 통계 업데이트
  tlog.set_reclaimed(cl.reclaimed_count());
}
```

But the version currently on GitHub doesn't use the pseudo-code above; instead, it checks if the object is already dead, if the RSet is complete, and if it's not pinned, to register it as a final candidate.

```java
bool humongous_region_is_candidate(G1HeapRegion* region) const {
      assert(region->is_starts_humongous(), "Must start a humongous object");

      oop obj = cast_to_oop(region->bottom());

      // Dead objects cannot be eager reclaim candidates. Due to class
      // unloading it is unsafe to query their classes so we return early.
      if (_g1h->is_obj_dead(obj, region)) {
        return false;
      }

      // If we do not have a complete remembered set for the region, then we can
      // not be sure that we have all references to it.
      if (!region->rem_set()->is_complete()) {
        return false;
      }
      // We also cannot collect the humongous object if it is pinned.
      if (region->has_pinned_objects()) {
        return false;
      }
      // Candidate selection must satisfy the following constraints
      // while concurrent marking is in progress:
      //
      // * In order to maintain SATB invariants, an object must not be
      // reclaimed if it was allocated before the start of marking and
      // has not had its references scanned.  Such an object must have
      // its references (including type metadata) scanned to ensure no
      // live objects are missed by the marking process.  Objects
      // allocated after the start of concurrent marking don't need to
      // be scanned.
      //
      // * An object must not be reclaimed if it is on the concurrent
      // mark stack.  Objects allocated after the start of concurrent
      // marking are never pushed on the mark stack.
      //
      // Nominating only objects allocated after the start of concurrent
      // marking is sufficient to meet both constraints.  This may miss
      // some objects that satisfy the constraints, but the marking data
      // structures don't support efficiently performing the needed
      // additional tests or scrubbing of the mark stack.
      //
      // We handle humongous objects specially, because frequent allocation and
      // dropping of large binary blobs is an important use case for eager reclaim,
      // and this special handling increases needed headroom.
      // It also helps with G1 allocating humongous objects as old generation
      // objects although they might also die quite quickly.
      //
      // TypeArray objects are allowed to be reclaimed even if allocated before
      // the start of concurrent mark.  For this we rely on mark stack insertion
      // to exclude is_typeArray() objects, preventing reclaiming an object
      // that is in the mark stack.  We also rely on the metadata for
      // such objects to be built-in and so ensured to be kept live.
      //
      // Non-typeArrays that were allocated before marking are excluded from
      // eager reclaim during marking.  One issue is the problem described
      // above with scrubbing the mark stack, but there is also a problem
      // causing these humongous objects being collected incorrectly:
      //
      // E.g. if the mutator is running, we may have objects o1 and o2 in the same
      // region, where o1 has already been scanned and o2 is only reachable by
      // the candidate object h, which is humongous.
      //
      // If the mutator read the reference to o2 from h and installed it into o1,
      // no remembered set entry would be created for keeping alive o2, as o1 and
      // o2 are in the same region.  Object h might be reclaimed by the next
      // garbage collection. o1 still has the reference to o2, but since o1 had
      // already been scanned we do not detect o2 to be still live and reclaim it.
      //
      // There is another minor problem with non-typeArray regions being the source
      // of remembered set entries in other region's remembered sets.  There are
      // two cases: first, the remembered set entry is in a Free region after reclaim.
      // We handle this case by ignoring these cards during merging the remembered
      // sets.
      //
      // Second, there may be cases where eagerly reclaimed regions were already
      // reallocated.  This may cause scanning of these outdated remembered set
      // entries, containing some objects. But apart from extra work this does
      // not cause correctness issues.
      // There is no difference between scanning cards covering an effectively
      // dead humongous object vs. some other objects in reallocated regions.
      //
      // TAMSes are only reset after completing the entire mark cycle, during
      // bitmap clearing. It is worth to not wait until then, and allow reclamation
      // outside of actual (concurrent) SATB marking.
      // This also applies to the concurrent start pause - we only set
      // mark_in_progress() at the end of that GC: no mutator is running that can
      // sneakily install a new reference to the potentially reclaimed humongous
      // object.
      // During the concurrent start pause the situation described above where we
      // miss a reference can not happen. No mutator is modifying the object
      // graph to install such an overlooked reference.
      //
      // After the pause, having reclaimed h, obviously the mutator can't fetch
      // the reference from h any more.
      if (!obj->is_typeArray()) {
        // All regions that were allocated before marking have a TAMS != bottom.
        bool allocated_before_mark_start = region->bottom() != _g1h->concurrent_mark()->top_at_mark_start(region);
        bool mark_in_progress = _g1h->collector_state()->mark_in_progress();

        if (allocated_before_mark_start && mark_in_progress) {
          return false;
        }
      }
      return _g1h->is_potential_eager_reclaim_candidate(region);
    }

```

The logic for deciding whether to collect is inside a function within this closure object.

```cpp
virtual bool do_heap_region(G1HeapRegion* hr) {
      // First prepare the region for scanning
      _g1h->rem_set()->prepare_region_for_scan(hr);

      // Now check if region is a humongous candidate
      if (!hr->is_starts_humongous()) {
        _g1h->update_region_attr(hr);
        return false;
      }

      uint index = hr->hrm_index();
      if (humongous_region_is_candidate(hr)) {
        _g1h->register_humongous_candidate_region_with_region_attr(index);
        _worker_humongous_candidates++;
        // We will later handle the remembered sets of these regions.
      } else {
        _g1h->update_region_attr(hr);
      }

      // Sample card set sizes for humongous regions before GC: this makes the policy
      // to give back memory to the OS keep the most recent amount of memory for these regions.
      _humongous_card_set_stats.add(hr->rem_set()->card_set_memory_stats());

      log_debug(gc, humongous)("Humongous region %u (object size %zu @ " PTR_FORMAT ") remset %zu code roots %zu "
                               "marked %d pinned count %zu reclaim candidate %d type %s",
                               index,
                               cast_to_oop(hr->bottom())->size() * HeapWordSize,
                               p2i(hr->bottom()),
                               hr->rem_set()->occupied(),
                               hr->rem_set()->code_roots_list_length(),
                               _g1h->concurrent_mark()->mark_bitmap()->is_marked(hr->bottom()),
                               hr->pinned_count(),
                               _g1h->is_humongous_reclaim_candidate(index),
                               cast_to_oop(hr->bottom())->is_typeArray() ? "tA"
                                                                         : (cast_to_oop(hr->bottom())->is_objArray() ? "oA" : "ob")
                              );
      _worker_humongous_total++;

      return false;
    }
}
```

```java
 if (humongous_region_is_candidate(hr)) {
        _g1h->register_humongous_candidate_region_with_region_attr(index);
        _worker_humongous_candidates++;
        // We will later handle the remembered sets of these regions.
      } else {
        _g1h->update_region_attr(hr);
      }
```

In the `do_heap_region` method, the section above shows that humongous regions registered as candidates will be subject to collection at the end of young GC.

In any case, humongous objects can easily lead to memory fragmentation, and the inability to find contiguous space can cause premature full GCs.

Originally, they were only deallocated during full GC or cleanup phases, but **in recent JDK versions, they have been optimized for early deallocation even during periodic young GC phases if there are no references.** This is likely the `eagerly_reclaim_humongous_regions` part we saw earlier.

## Mixed GC Trigger

Occurs immediately after a concurrent marking cycle, initiated by IHOP, successfully completes.

However, simply exceeding IHOP does not necessarily mean that mixed GC will actively occur.

The following parameters influence the trigger and execution:

**G1HeapWastePercent (most important trigger condition)**
- After concurrent marking finishes, G1 calculates how much reclaimable space there is.
  - Condition: If the proportion of reclaimable space in the entire heap is lower than `G1HeapWastePercent`, G1 will not perform a costly mixed GC. In other words, enough garbage must accumulate for mixed GC to execute.

> Ah, I knew this, but if you can't answer, you don't know it, right?

**G1MixedGCLiveThresholdPercent**
- Condition: The proportion of live objects within a specific region must be lower than the configured value (default is 85) for that region to be included in the mixed GC target (CSet).

**G1MixedGcCountTarget**
- Mixed GC does not clean up the entire Old region at once but performs it in multiple steps.
- Based on this count target, it determines how many old regions to collect each time, and if it's deemed inefficient, the mixed GC cycle may be interrupted.

**Induced trigger: evacuation failure**
- During young GC, if there isn't enough space to move objects to the survivor or old regions, an evacuation failure occurs. This situation triggers G1 to aggressively attempt mixed GC, or in the worst case, leads to a full GC.

In summary, a Humongous object is a large object occupying more than half a region, allocated in contiguous regions of the old generation (for separate management, let's check the code).

Mixed GC is initiated by IHOP, but its actual execution is ultimately determined by `G1HeapWastePercent` to assess if there's enough worth in cleaning?

<br>
