# G1GC OpenJDK 8 and 11

OpenJDK 8에서도 G1GC를 사용할 수 있으나 나는 사용하지 않았다.

일단 나는 현재 parallel gc를 버림으로써 얻고싶은 가치는 Long Tail 현상을 방지하기 위해서

1퍼센트의 콜이 무거운 gc stw로 인해서, 응답을 늦게 뱉는 현상을 버리고 싶었기 때문인데,

g1gc는 목표 pause time을 지정하여 조절할 수 있는 장점이 있지만, g1gc open jdk8에서는

오히려 fullGC가 발생하게 되면 serial gc처럼 그냥 영역 전체를 스캔해 진행하게 된다.

### OpenJDK 8 

**Serail Full GC** 가장 치명적인 문제로 Concurrent하게 작동하지만, 메모리 회수 속도가 할당 속도를 따라가지 못하면 Evacuation Failure가 발생하고 FullGC로 전환된다.

현상은 jdk8의 g1 full gc는 Single Thread 단위로 동작하는 것이고.

예를들어 힙이 32기가고 cpu코어가 64개라도 단 1개의 코어만 사용해 메모리를 정리하며, 이로인해 수십초 심지어 분단위 stw가 발생할수도있다. 물론 힙이 이렇게 큰걸 지금 쓰고잇진않아서 길면 수초단위겠지만 그것도 길다.

두번째로 **RSet의 과도한 메모리 점유**문제도 존재한다.

JDK8에서는 RSet의 크기를 효율적으로 관리하지 못했다. g1gc는 region 단위로 나누고, region간의 참조를 추적하기 위해서 RSet이라는 별도 자료구조를 사용하는데.

힙 크기의 15~20%에 달하는 추가적인 native memory를 소비하는 경우가 빈번했다.

**자료구조적 원인**: 누가 나를 참조하는가 points into를 추적하는데 참조가 많은 인기객체 hot object가 있는 region의 RSet이 폭발적으로 켜지는 현상이 발생했다.

**튜닝의 난해함, Latency 불안정**도 있었는데 목표 응답 시간 (-XX:MaxGCPauseMillis)을 설정해도, Mixed GC(Old영역 수집) 단계에서 수집할 region의 개수 CSet을 잘못 계산하여 목표 시간을 초과하는 경우가 잦았다. 한번 수집을 시작하면 멈출수가 없었기 때문이다.

<br>

### 개선된 버전 JDK 10 ~ 17

JDK10(JEP307), JDK12(JEP344) 에서 일어났다.

#### Parallel Full GC(JDK 10, JEP307)

Full GC 발생시, 사용 가능한 모든 cpu 코어를 사용하여 병렬로 Mark-Sweep-Compact를 수행하도록 변경되었다.

알고리즘 변화도 있었는데, 기존 Serial 알고리즘에서 힙을 파티셔닝하여 여러 스레드가 동시에 Live 객체를 마킹하고 압축하는 병렬 알고리즘으로 대체되었다.

정량적 개선도 있는데
- 이론적으로 사용 코어배만큼 빨라진다
- 실제 벤치마크(SPECjbb)기준 16코어 환경에서 FullGC 시간이 18초 -> 1.5초로 약 12배 단축되었다는 사례도 있다.

#### Abortable Mixed Collections (JDK12 JEP 344)

g1이 Collectin Set(수집 대상 region 목록)을 구성할 때, 필수 영역과 선택 영역(Optional)으로 나눈다.

GC 수행 중 목표 시간 (`MaxGCPauseMillis`)를 초과할 것 같으면 선택 영역의 수집을 중단하고 즉시 리턴한다.

사라진 현상: 예측 실패로 인한 Latency Spike 튀는 현상이 획기적으로 줄었다.

정량적 수치: P99(상위 1% 느린 요청) Latency가 JDK8  대비 JDK 11/17에서 약 40~60% 감소했다.

#### Promptly Return Unsused Committed Memory JDK 12

JDK8 G1은 한 번 os로부터 힙 메모리를 가져오면(committed) 힙 사용량이 줄어들어도 os에게 잘 돌려주지 않았다.

jdk12부터는 유휴 상태일 때 적극적으로 메모리를 os에서 반환한다.

컨테이너 호나경에서 리소스 효율성이 크게 증가한 변경사항이다.

<br>

### 자료구조 및 알고리즘

jdk8, 최신 jdk g1gc 성능 차리르 만드는 핵심 자료구조의 변화에 대해서 알아보겠다.

#### RSet(Remebered Set) 최적화

RSet은 Region의 객체를 참조하고 있는 다른 region의 주소들을 저장한다.

- JDK 8의 문제 (Fine-grained Table)
  - 참조가 추가될 때마다 Hash Table에 엔트리를 추가했다.
  - Coarsening(단위 뭉치기) 로직이 단순하며, 특정 region에 참조가 몰리면 메모리 오버헤드가 급증했다.
- 개선 (Sparse -> Fine -> Coarse 레벨 최적화)
  - 최신 G1은 RSet을 3단계로 관리한다.
  - Sparse PRT(Per Region Table): 적은 수의 참조는 해시 테이블 대신 작은 배열로 관리한다.
  - Bitmaps: 참조가 많아지면 비트맵 방식으로 전환하여 메모리 사용량을 고정시킨다.
  - 결과로는 동일한 워크로드에서 RSet이 차지하는 Native Memory의 양이 jdk8대비 50%이상 감소했다.

깨알 HashTable vs HashMap을 알아보자면

물론 gc에서 쓰는 HashTable은 C++레벨의 코드지만

일단 HashTable은 java에서는 Synchronized에 모든 메서드에 락이 걸려있어 멀티 스레드 환경에서 안전하지만 매우 느리다. 병목 발생하고 key value에 널값도 못넣고 chaining이ㅏ라고 해시 충돌시 리스트로만 연결해 데이터가 많아지면 검색속도가 저하된다. 구식 Enumration 인터페이스를 쓰기도하고

HashMap은 Unsynchronized로 락이 없어서 매우 빠르고 멀티 스레드 환경에서는 ConcurrentHashMap을 써야하긴한다. key1개 value n개 null 허용한다. 더 유연한 데이터 처리가 가능하고 Chaining으로 LinkedList에 담아두다가 버킷에 일정한 수가 넘어가면 Red-Black Tree로 변환해 검색속도를 O(log n)으로 개선되었다. Iterator 인터페이스를 사용한다.

근데 gc에서 나온 hashtable은 c++개념이라 좀 다르긴하다 성능저하 이런게 있진않음

#### IHOP(Initiating Heap Occupancy Percent)의 적응형 변화

JDK8: `-XX:InitiatingHeapOccupancyPercent` (기본 45퍼센트) 고정값에 의존했다. 45% 찰때까지 기다리다가, 갑자기 할당이 몰리면 대응이 늦어 FullGC가 터졌었다.

JDK9+에서는 Adaptive IHOP로 g1이 애플리케이션 할당속도와 마킹에 걸리는 시간을 학습한다.

이 속도라면 45%가 아니라 30%에서 미리 마킹을 시작해야겠네...라고 스스로 판단한다. 이로인해 To-Space Exhaused 오류가 현저히 줄어들었다.

### 총 정리

1. **FullGC 처리 방식:** Serial -> Parallel로 처리속도 10~30배 향상
2. **Max Latency(Pause):** 예측 실패시 수 초 단위 스파이크 -> 설정값 200ms를 매우 잘 준수함.
3. **Throughtput 처리:** Baseline -> Baseline + 10~15% 향상 Latency 중심이지만 cpu 개선 처리량도 동반 상승함
4. **Native Memory OverHead:** Heap의 15~25%추가 사용 -> Heap의 5~10% 수준으로 메모리 효율 2배 증가, RSet최적화 덕분
5. **String Deduplication:** 끄거나 켜도 오버헤드가 큼 -> 기본적으로 더 효율적인 해싱 사용으로 String 중복 제거 성능 향상

<br>

여기서 의문이 생길 수 있는데, 어쨋든 G1GC도 FullGC로 돌때 Parallel로 돌면 p99박살나는건 같지않아 싶을수있다.

P99는 가장 최악의 순간만을 의미하는 것이 아니라 느린 응답이 얼마나 자주 섞여있는가로 통계적으로 나타낸다는 것을 알아야한다.;

ParallelGC는 대청소 방식이고 MajorGC가 Old영역을 꽉 찰때까지 기다렸다가 stw를 한번에 멈추고 다 비운다. 주기적으로 반드시 실행하고 거대한 스파이크가 발생하는 것이다.

반면에 G1GC는 Mixed GC로 old영역이 꽉차기 전에 Concurrent하게 조금씩 청소해놓는다. FullGC의 의미는 Failure를 의미하며 튜닝 실패나 리소스 부족시 발생하는 비상상황인 것이다.

즉 ParallelGC에서 FullGC는 무조건 루틴처럼 발생하는 현상이지만 G1GC에서는 그렇지 않고 비상상황이라고 생각해도 되는것이다.

결론적으로는 g1gc에서도 fullgc가 터지면 parallel gc 만큼 느린게 맞지만. g1gc는 그 상황을 안만들게하기위해 설계된 알고리즘이므로 거의 발생 안한다고 보면된다.

애초에 그 최악의 상황까지 갔다는거 자체는 또 다른 문제가 있을거기 때문에, 그럴대는 InitiatingHeapOccupancyPercent를 낮춰서 빈도를 올리거나 Reseved 영역을 추가해놓고 fullgc를 좀 방지하거나 evacuation failure를 방지하기 위해서 튜닝을 다시해야한다 힙사이즈를 늘리거나.