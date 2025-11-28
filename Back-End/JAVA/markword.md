# Object Header - Mark Word

JVM 객체는 힙에 생성이 되고 힙에 올라간 각 객체의 헤더 안에 마크 워드가 포함된다.

핫스팟 jvm 기준으로 객체 헤더는 다음과 같이 구성되어 있다.

1. mark world
2. klass pointer (class metadata pointer)
3. array length

즉, 마크워드는 객체 인스턴스 내부의 헤더 공간에 위치하며, 힙 메모리 안에 포함된다.

mark word는 HotSpot JVM의 핵심 구조로, 다음 정보를 저장한다.

- lock state: biaed lock, thin lock, fat lock
- Identity HashCode
- GC 관련 비트
- Age(객체 생존 나이: young survivor old로 이동할때 증가)


#### EX.

| 비트   | 의미                                             |
| ---- | ---------------------------------------------- |
| 25비트 | hashCode                                       |
| 4비트  | age                                            |
| 1비트  | biased lock flag                               |
| 2비트  | lock 상태 (unlocked, thin, fat, marked for GC 등) |

### gc에서 마크워드가 사용되는 방식

GC Marking 단계에서 마크 워드는 다음 두 가지 방식으로 활용된다.

Stop-the-World로 마킹 (Serial, Parallel GC)
객체가 live로 표시될 때 Mark Word 내부의 특정 비트를 사용하거나, Mark Bitmap or Mark Stack에서 별도로 추적.

G1GC의 SATB + Card Table
- G1GC에서는 Mark Word가 SATB(Snapshot At The beginning) 방식에 관여한다.

SATB Write Barrier에서는 다음과 같은 일이 일어난다.

1. 어떤 객체 A의 필드가 객체 B로 바뀔 때
2. 변경전 old 참조를 SATB 버퍼에 기록함
3. 내부적으로 reference를 스냅샷으로 유지하기 위해 writer barrier를 함께 실행함.
4. 이때 마크워드의 SATB 로그를 읽고 해당 객체가 live인지 확인함
5. GC 스레드가 SATB 로그를 읽고 해당 객체가 live인지 재확인

즉, Mark Word의 marking bit는 G1의 마킹 스테이지와 연결되어 객체 생존 여부, 추적 대상 여부를 기록하는데 사용됨.

### 마크 워드, 스레드 락

마크워드는 락 상태에 따라 완전히 다른 구조로 해석ㄷ됨

unlock: hashCode, age, GC bit 저장

Baised Lock: Mark Word는 특정 스레드의 id를 가리키는 형태로 바뀜

Thin: Mark Work가 스레드 스택의 Lock Record Pointer로 변함.

Fat Lock: Mark Word는 모니터 객체 포인터로 바뀜.

즉, 마크워드는 gc 정보부터 lock 메커니즘까지 모두 한 공간에 담는 가변구조 header가 되는것.

| 구성 요소                | 위치                          |
| -------------------- | --------------------------- |
| 객체 인스턴스              | Heap                        |
| 객체 헤더(Object Header) | Heap 내부의 각 객체 시작 부분         |
| Mark Word            | **객체 헤더의 첫 8바이트(64bit) 위치** |
