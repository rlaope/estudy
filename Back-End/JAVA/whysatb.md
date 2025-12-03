# G1GC는 왜 Incremental Update가 아니라 SATB를 선택했는가?


### G1GC는 Region live ratio가 핵심 -> 명확한 snapshot 필요

g1gc mixed gc는 다음 기준으로 region을 선택함

```
old region의 live ratio (live bytes %)
```

Incremental Update는 concurrent mark가 진행되는 동안 객체 그래프 변화가 매우 빠르게 변해 정확한 live ratio를 계산하기가 어렵다.

반면 SATB는
- snapshot 고정
- region live set이 marking 끝에 안정적으로 계산
- mixed gc candidate 선택이 정확해짐
- pause time 컨트롤 성능이 올라감

즉, g1 core design고 ㅏ잘 맞는다.

### Incremental Update는 remark 단계 비용이 매우 큼

Incremental Update는 marking 도중 변경된 new reference를 모두 처리해야한다.

cms는 remark 단계가 길어지는 이유도 이거다.

g1은 remark pause를 최소화 하는 collector로 remark가 길면 pause time control이 불가능하다.

satb는 변경된 old reference만 기록하면 되니까 remark 단계에는 버퍼 플러시 + 최소 보정만 수행하고 짧다.

즉 remark pause가 짧은게 중요해 satb 채택

### Evacuation 기반 GC 특성상 SATB가 더 자연스럽다.

Evacuation GC는 복사 후 old region 파기 전략이다.

이 과정에서 marking 기반 정보가 매우 중요하다.

Incremental Update에서는 복사 중에도 참조 변경이 계속 발생해 marking consistency를 유지하기가 어렵다.

SATB는 marking 시작 시점 snapshot을 유지하므로 Evacuation 수행시 참조 업데이트가 훨신 안정적이다.

### Write Barrier 측면에서 효율적

Incremental Update의 write barrier는 다음 작업을 수행한다.
1. new reference 기록
2. 이 new reference chain 따라 그래프 확장
3. 이후 reachable graph 완성 보정 필요

SATB의 write barrier는 상대적으로 비용이 안정적이다

```
SATB: enqueue(old_ref)
Incremental Update: enqueue(new_ref) + 추가 follow-up graph scan
```

1. region 기반의 live ratio 계산이 정확해짐
2. remark pause를 미세하게 줄일 수 있음
3. evacuation 기반 수집과 consistency 맞음
4. pause-time control 전략과 철학적으로 잘 맞음