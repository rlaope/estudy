# Humongous Object, Mixed GC Trigger

복기좀 해보자.

### Humongous Object란

g1gc는 heap을 동일한 크기의 region으로 나누어 관리한다.

이때 region 크기의 50%를 초과하는 크기를 가진 객체를 humongous object라고 부른다.

판단 기준: 만약 region의 크기가 8mb라면 4mb를 초과하는 객체 모두 humongous object가 된다.

일반적인 객체처럼 에덴에 할당되어 복사되는 과정을 거치지 않는다, 객체가 너무 커서 복사 비용이 너무 높기 때문

### 배치되는 공간은?

humongous object는 humongouse region이라는 전용 공간에 배치된다.

논리적 위치로는 humongous region은 논리적으로 **old gen에 속한다.** 객체 크기가 region 하나보다 크기면 여러개의 연속된 region을 점유힌다.

> 근데 Old gen으로 뜨긴하는데 별도의 영역으로 관리되었던거같다. jcmd jvm info이걸로 보고, jdk좀 뜯어봐야할듯 

### +Humongous 

humongos는 논리적으로 old gen에 속하지만 일반 객체들과 달리 humongous region이라는 별도 영역에서 특별 관리된다.

openjkd 11 ~ 17 코드 기준으로

`src/hotspot/share/gc/g1/heapRegion.hpp` 여기 보면 
- StartsHumongous (SH): 객체의 시작부분이 들어있는 리전
- ContinuesHumongous(CH): 객체가 커서 다음 리전까지 이어질때 사용되는 연속 리전들 정보가 있다.

할당 로직에서도 `G1CollectedHeap::attempt_allocation_humongous` 를 보면 attempt_allocation이라는 일반 객체와 다르게 분리된 경로를 타는걸 볼수있는데

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

이 메서드에서는 필요한만큼 연속된 빈 리전을 찾고 객체는 빈 리전 아무데나 들어가면 되지만 humongous는 덩어리가 커서 기차 칸처럼 붙어있는 공간을 찾아야됨

타입도 분리되어있음 G1RegionType `src/hotspot/share/gc/g1/g1RegionType.hpp`

https://github.com/openjdk/jdk/blob/master/src/hotspot/share/gc/g1/g1HeapRegionType.hpp

```cpp
// 리전 타입을 정의하는 부분에서 Humongous를 별도로 구분합니다.
bool is_humongous() const { return _type >= HumongousMask; }
bool is_starts_humongous() const { return _type == StartsHumongous; }
bool is_continues_humongous() const { return _type == ContinuesHumongous; }
```

`G1CollectedHeap:eagerly_reclaim_humongous_regions`  Humongous가 old gen이면서도 특별하게 취급되는 이유. 보통은 old gen은 full gc나 concurrent marking이 끝나야 정리되는데 humongous 객체는 참조가 없다면 youngc때도 수거될수있음

`src/hotspot/share/gc/g1/g1CollectedHeap.cpp` 여기보면 young gc 시점에 이 메서드 호출해서 RSet이 비어있는 humongous 객체를 즉시 메모리에서 해체하기도 함. 대용량 객체가 메모리를 빠르게 확보하기 위한 전략

근데 https://github.com/openjdk/jdk/blob/master/src/hotspot/share/gc/g1/g1YoungCollector.cpp 여기로 옮겨진듯 해당 코드들이 여기서 확인가능함 아래 함수들은

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

근데 지금 github에 올라와있는버전은 위 의사코드가 아니라 객체가 이미 죽었는지 RSet이 완전한지 pinned상태가 아닌지 확인해 최종 후보로 등록하도록 동작하넹

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

실제로 수거할지 말지 결정하는 로직은 이 클로저 객체 내부의 함수에 들어있음

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

do heap region에 위 부근 보면 후보로 등록된 humongous 리전들이 나중에 young gc 끝날때 수거 대상이 된다.


어쨋든 humongous 때문에 메모리 단편화가 발생하기 쉽고, 연속된 공간을 찾지 못해 조기에 full gc가 발생할 수 있다.

원래는 full gc나 clean up 단계에서만 해제되었으나, **최신 jdk 버전에서는 주기적인 young gc 단계에서도 참조가 없다면 조기에 해제되도록 최적화되었다.** 아마 이게 아까 위에서 본 부분일듯 eagerly_reclaim_humongous_regions ㅇㅇ





## Mixed GC Trigger

IHOP에 의해 시작된 concurrent marking cycle이 성공적으로 완료된 직후에 발생한다.

하지만 단순히 ihop를 넘었다고해서 무조건 mixed gc가 활발히 일어나는것은 아니다.

다음의 파라미터들이 트리거와 실행 여부에 관여한다.

**G1HeapWastePercent(가장 중요한 트리거 조건)**
- concurrent marking이 끝나고 g1은 회수가능한 공간이 얼마나 되는지 계산한다.
  - 조건: 힙 전체에서 회수 가능한 공간의 비율이 `G1HeapWastePercent`보다 낮다면, g1은 굳은 비용이 큰 mixed gc를 수행하지 않는다. 즉 쓰레기가 충분히 쌓여야 mixed gc 실행
  
> 아 이거 알았는데 하; 답못하면 모른거지 그래 ㅇ

**G1MixedGCLiveThresholdPercent**
- 조건: 특정 region 내의 살아있는 객체 비율이 설정값보다 (기본은 85) 낮아야 해당 region이 mixed gc 대상 (CSet)에 포함된다.

**G1MixedGcCountTarget**
- MixedGC는 한번에 모든 Old 영역을 치우지 않고 여러번 나누어 수행한다.
- 이 횟수 목표에 따라 매번 얼마만큼의 old region을 수집할지 결정되며, 작업 효율이 안나온다고 판단되면 중간에 mixed gc 사이클이 중단될수도 있다.

**유도된 트리거 evacuation failure**
- young gc도중 survivor 영역이나 old영역으로 객체를 이동시킬 공간이 부족하면 evacuation failure가 발생한다. 이 상황은 g1이 공격적으로 mixed gc를 시도하게 하거나 최악의 경우 full gc로 이어지게 만드는 트리거가 된다.

요약하면 Humongous object는 region 절반 이상을 차지하는 거대 객체로 old 영역의 연속된 region(별도관리는 코드파보자) 할당된다.

mixed gc는 IHOP으로 시작되지만 실제 실행 여부는 G1HeapWastePercent를 통해 최소 청소할 가치가 있는가를  최종 결정? 

<br>

