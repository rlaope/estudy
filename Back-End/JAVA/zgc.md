# ZGC (Z Garbage Collector)

기존 gc(g1포함)은 pause time을 줄이려고 노력했지, pause time을 구조적으로 제한하지는 못했다.

특히 다음 문제가 남아 있었는데.

- heap이 커질수록 root scan / object move 비용이 증가
- compaction (객체 이동)은 본질적으로 stw에 가까움
- low latency system에서 수 ms 이상의 stw도 치명적임

zgc 목표는 명확하다. **heap 크기와 무관하게 pause time을 10ms이하로 제한한다.**

이 목표 하나 때문에 jvm memory model 자체를 바꾼것

### ZGC의 핵심 설계 철학

zgc는 기존 gc들과 다르게 가정한다.

객체의 주소는 안정적이며 gc중에는 객체 이동을 제한해야 한다. root scan은 stw로 해야 아전한다.

zgc 전제 상황
- 객체 주소는 가변적이여도 된다.
- 객체 이동은 항상 가능해야한다.
- 읽는 쪽이 비용을 지불한다 (read barrier)

위 전제때문에 zgc는 Colored Pointer 라는 방식을 사용한다.

### Colored Pointer

포인터에 메타데이터를 심는다. zgc는 객체 포인터 자체에 gc 상태 정보를 인코딩 한다.

64 bit 포인터 구조 (개념적)

```css
[ Marked | Remapped | Finalizable | Reserved | Address ]
```
- 실제 메로리 주소는 하위 비트
- 상위 비트는 gc의 상태 비트다.

이게 의미하는 바는 gc 상태를 보려면 객체를 읽기만 하면 된다라는 뜻.

기존 gc는 객체 상태를 보려면 별도의 메타데이터 구조가 필요했다 (root scan, RSet, Card Table)

그러나 ZGC는 포인터 하나만 보면 되고 별도 구조 탐색이 불필요했다. stw root scan 최소화 기능임.

### ZGC Read Barrier

ZGC는 Write Barrier보다 Read Barrier가 핵심이다.

애플리케이션 스레드가 객체를 읽는순간, jvm이 개입하는 메커니즘인데.

```
Object o = ref.field;
```

이때 내부적으로 다음이 숳행된다.

1. 포인터의 color 확인
2. 상태에 따라 mark 필요 -> mark 수행, relocation 필요? -> 새 주소 리다이렉션
3. 항상 최신 주소 반환

으로 동작한다. 읽기는 쓰기보다 빈도가 높고 비용이 커보인다.

하지만 현대 cpu 분기 예측에 매우 유리하고 대부분의 경우 no-op, stw를 없애는 대가로 충분히 감당 가능하다.

zgc선택은 애플리케이션 스레드가 조금씩 비용을 나눠내는 선택을 한 것.

> no-op: 실제로 아무 동작도 하지 않는 연산으로 조건검사만 사용하고 상태 변경이나 비용있는 작업없이 그대로 종료되는 경우를 말함.


### Relocation

ZGC는 객체를 Concurrent 하게 이동함.

- GC Thread가 객체를 새 영역으로 복사
- old -> new 매핑 정보 유지
- 포인터는 여전히 old 주소를 가질 수 있음
- read barrier가 접근시 자동으로 remap함 (g1의 쓸때 참조 변경하는 write barrier랑 다름)

객체 이동은 즉시 반영이 아니라 지연 반영이다.

이 구조로 모든 포인터를 한번에 고칠 피룡가 없으며 접근되는 객체만 점진적으로 수행하면 된다. stw에서 할일이 거의 없음

### pause time

zgc에서 pause time은 매우 짧은데 그 이유는 단순히 병렬 gc가 아니다.

stw에서 남은 작업을 알아보자 위에서 알아본 메모리 구조의 변경으로 남은 stw에서의 작업은
- Root Set Snapshot 시작/종료
- 매우 짧은 상태전환
  - Marking -> Concurrent
  - Relocation -> Concurrent
  - Remapping -> Read Barrier로 분산

그래서 heap이 10GB이든 1TB이든 Pause는 거의 동일하다.

### ZGC의 힙 구조

ZGC는 Heap을 Region이 아니라 ZPage라는 단위로 나눈다.

ZPage는 크기가 가변이며 Small / Medium / Large Object 전용 페이지로 나누어져 있다.

Compaction 대상이 명확하다.

이 구조 덕분에 Fragmentation 관리가 용이하며 Large Object 이동 비용이 최소화 된다.

### ZGC 한계와 비용

CPU 비용
- Read Barrier는 공짜가 아니다.
- Allocation Rate 높은 시스템에서는 cpu 사용량이 증가된다.

메모리 사용량
- Relocation 중에는 Old + New 객체가 공존한다
- Peak 메모리 사용량이 증가한다.

플랫폼 제약
- 64bit 필수
- pointer tagging 지원 필요
- 임베디드 / 저사양 환경 부적합

### ZGC vs G1GC

| 항목          | G1GC      | ZGC        |
| ----------- | --------- | ---------- |
| Pause 목표    | Soft Goal | Hard Goal  |
| Root Scan   | STW       | 거의 없음      |
| Object Move | STW 중심    | Concurrent |
| Barrier     | Write 중심  | Read 중심    |
| Heap 크기 영향  | 큼         | 거의 없음      |


그래서 zgc는 g1보다 더욱 low latncy system에 어울리며 cpu 매우 타이트하거나 힙이 작은 단순 서비스에서는 어울리지 않다.

