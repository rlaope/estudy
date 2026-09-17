# Evacuation Failure

Evacuation Failure는 g1gc에서 가장 치명적인 상황중 하나로

g1은 evacuation(객체 복사) 기반으로 region을 회수하는데 이 copy 단계에서 실패하는 경우다.

발생 조건으로는 다음과 같다.

#### 1. promotion copy 공간 부족

young -> survivor/old 승격 과정 또는 mixed gc에서 old region evacuation 과정에서 복사할 대상 객체가 들어갈 free region이 부족한 상황이다.

즉 survivor이 가득 찼다거나 old region free region이 부족하다거나 reserve영역이 부족(`-XX:G1ReservePercent`로 관리하는 비상 여유 공간), Humongous 할당때문에 region이 쪼개져 여유 region이 없다거나..

g1은 evacuation을 시작하기 전에 대략적인 카피 비용과 필요한 region 수를 예측하지만, 그예측이 틀리거나 런타임 상태가 변하면 중간에 공간이 모자라서 실패함.

#### 2. RSet이 너무 커져서 copy 작업중 timeout

Evacuation은 copy + 참조 업데이트 + RSet 유지가 필요한데

RSet이 커지면 copy중에 매우 많은 pointer update가 필요해 실패할 수 있다.

#### 객체 크기 폭증 / Survivor overflow

특히 Young GC에서 노화된 객체가 많이 생기면 Survivor에 못 들어가서 old로 승격해야 하는데 공간이 모자라도 실패한다.

### Evacuation Failure가 발생한다면 hotspot은 어떻게 처리하나?

핵심: 그 region은 더 이상 evacuation 방식으로 비우지 못하고 그대로 old 형태로 잔류한다.

구체적으로는 아래 처리 과정이 발생한다.

1. fail한 region을 pinned region으로 마킹함
   1. evacuation하려했지만, 실패한 region은 더 이상 evacuate 대상이 될 수 없는 old region으로 마킹함. 이 region은 유효 객체 + 죽은 객체가 섞여있으며 compact 처리되지 않은 상태로 남음
2. gc는 해당 region을 do-not evacuate 처리
   1. 이 region은 이후 mixed gc에서 대상이 되지 않음, 즉 g1 major 특징인 region-level compaction이 이 region에서는 아예 불가능함
3. 추가적인 region 할당 및 fallback 시나리오
   1. hotspot은 즉시 다음 동작을 시도함
      1. 남은 free region을 최대한 확보
      2. 다른 region evacuate 우선순위 변경
      3. 세대 비율 조정
      4. gc cycle 즉시 중지하고 리마킹 또는 새로운 mark cycle 시작
4. evacuation failure가 여러번 누적되면 최종적으로 full gc 유도
   1. pinned region이 많아지면 다음 현상 발생
      1. mixed gc 효과 없음 -> old live ratio가 유지됨
      2. fragmentation 증가
      3. humongous region 누족
      4. free region 감소

최종적으로 g1이 더이상 Incremental 수집이 불가능하다고 판단하면 full gc fallback

### jdk 코드

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
copy_to_survivor_space → allocate 실패 → handle_evacuation_failure
→ region.set_evacuation_failed()
→ heap.evacuation_failure_occurred()
→ G1EvacuationFailure::do_evacuation_failure()
→ rebuild_region_sets()
→ 필요시 Full GC fallback
```