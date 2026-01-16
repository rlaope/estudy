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

```cpp
bool G1EagerlyReclaimHumongousObjectsClosure::do_heap_region(HeapRegion* r) {
  G1CollectedHeap* g1h = G1CollectedHeap::heap();

  // 1. StartsHumongous 리전인지 확인 (객체의 시작점)
  if (!r->is_starts_humongous()) {
    return false;
  }

  uint region_idx = r->hrm_index();

  // 2. 이 리전이 수거 후보군에 등록되어 있는지 확인
  if (!g1h->is_humongous_reclaim_candidate(region_idx)) {
    return false;
  }

  // 3. 핵심 체크: Remembered Set(다른 영역에서 이 객체를 참조하는 정보)이 비어있는지 확인
  // 만약 다른 곳에서 참조하고 있다면 수거하면 안 됨
  G1RememberedSet* rs = r->rem_set();
  if (!rs->is_empty()) {
    // 참조가 남아있으므로 후보에서 제외
    g1h->set_humongous_is_not_reclaim_candidate(region_idx);
    return false;
  }

  // 4. 수거 확정: free_humongous_region을 호출하여 메모리 해제
  // 이 단계에서 Old Gen 영역이 즉시 비워짐
  _reclaimed_count++;
  g1h->free_humongous_region(r, &_free_region_list);

  return false;
}`
```


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

