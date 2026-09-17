# Concurrent Marking

Concurrent Marking은 g1gc가 애플리케이션을 멈추지 않고 힙 내의 모든 살아있는 객체를 추적하는 과정이다.

이 과정이 성공적으로 끝나야 비로소 Mixed GC를 수행할 수 있다.

즉 Mixed GC를 위한 사전 조사 단계라고 보면 된다.

총 5단계로 나뉘며 각 단계가 STW인제 Concurrent인지 튜닝의 핵심이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FbtksbS%2FbtsJyKslIOz%2FAAAAAAAAAAAAAAAAAAAAANN-zXo236hff-P6l0Ehm2mBfxLFdZmwKTdtF7184dkL%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1767193199%26allow_ip%3D%26allow_referer%3D%26signature%3Dn%252FTAni8%252F8MfsP4QYFovBxaRk5X0%253D)

Concurrent Marking이 시작되는 trigger는 정확히 IHOP 고정임계치에 의한 경우와 g1gc가 스스로 학습하여 결정하는 adaptive 방식 두가지로 나뉜다.

`-XX:InitiatingHeapOccupancyPercent` 보통 young gc가 끝난후 이 비율을 계산하고 계산 결과가 넘으면 (디폴트 45) 다음 young gc가 시작될때 initial mark가 함께 시작된다.

jdk9부터는 Adaptive IHOP가 켜져있는데 `-XX:+G1UseAdaptiveIHOP` G1GC가 단순히 45%의 숫자에 의존하지 않고 예측하기 시작한다.

마킹 시간 측정: 과거에는 얼마나 걸렸을까

할당 시간 측정: 애플리케이션이 메모리를 얼마나 빨리 채우는걸까

위의 질문을 스스로 한 뒤에 45라면 너무 늦어 30에 시작해야겠다하고, 생각하고 반대로 여유가 있으면 45보다 더 늦게 생각도 가능하다.

이 기능이 켜져있으면 사용자가 설정한 값은 초기값으로만 사용되고 g1gc가 알아서 타이밍을 조절한다.

그 외에도 Humongous Object 할당시 old region 크기를 50%를 넘는 거대 객체가 할당될때 g1gc는 힙이 급격하게 찰것을 우려해 즉시 마킹 사이클을 체크하거나 시작할 수 있다. jdk 버전에 따라 다르긴함

Metaspace 부족시에도 트리거된다 힙 메모리가 넉넉해도 클래스 메타데이터를 저장하는 Metaspace 영역이 꽉 차서 확장이 필요하면 언로딩을 위해 Concurrent Marking을 시작할 수 있다.

혹은 수동 호출

### Initial Mark (초기 마킹)

**상태: STW (멈춤)**

gc의 시작점인 Root Set(스택 변수, 전역 변수 등)에서 직접 참조하는 객체들을 마킹한다.

보통 YoungGC가 발생할 때 꼽사리 껴서 같이 수행된다. 그래서 로그를 보면

```
GC pause (G1 Evacuation Pause) (young) (initial-mark)
```

라고 찍힌다. Young GC 때문에 어차피 멈춘 김에 old 영역의 root도 같이 찍어두는 것이다.

### Root Region Scan

**상태: Concurrent (멈추지 않음)**

Initial Mark 단계에서 마킹된 Survivor 영역의 객체들이 참조하고 있는 Old 영역의 객체들을 찾아서 마킹한다.

이 작업은 반드시 다음 Young GC가 시작되기 전에 끝나야한다 만약 이 작업중에 메모리가 꽉차서 Young GC가 터지려고 하면, 이 스캔이 끝날때까지 Young GC가 대기해야하므로 지연이 발생할 수 있다.

### Concurrent Mark (동시 마킹) - 가장 김

**상태: Concurrent (멈추지 않음)**

본격적인 탐색이다 전체 힙을 뒤지며 살아있는 객체 그래프 Reference Chain을 끝까지 추적한다.

이때 SATB(Snapshot at the Beginning) 알고리즘을 사용하고 마킹 시작 시점의 스냅샷을 기준으로 살아있는 객체를 판단한다.

애플리케이션 스레드와 동시에 돌기 때문에 cpu를 나눠서 쓴다.

만약 이 단계가 너무 느려서 메모리 차는 속도를 못따라가면 Full GC가 발생한다.

### Remark (재마킹)

**상태: STW (멈춤)**

3번 단계인 Concurrent Mark 동안 애플리케이션이 계속 돌았으니, 그 사이 객체 참조 관계가 변했을 것이다.

그 변경된 분을 최종적으로 반영하여 마킹을 확정짓는 단계다. SATB 버퍼에 기록된 변경 사항들을 처리한다.

STW가 발생하지만 멀티 스레드로 병렬 처리해 최대한 빨리 끝낸다.

### Cleanup

**상태: STW + Concurrent**

1. Accounting (STW): 각 region 별로 살아있는 객체가 얼마나 되는지 비율을 계산한다. 이걸 알아야 mixed gc때 가비지 많은 것을 골라낼수있다.
2. RSet Scrubbing (STW): Remembered Set을 갱신한다.
3. Empty Region Reclaim(Concurrent): 살아있는 객체가 하나도 없는 100% 가비지 Region은 즉시 초기화하여 Free List에 반환한다. 

Cleanup단계가 끝나면 YoungGC 부터 Mixed GC 모드로 전환되어 아까 적어둔 Candidate Set의 Old Region들을 Young Gen과 함계 청소하기 시작한다.


```log
[0.005s][info][gc] Using G1

#1
[0.167s][info][gc] GC(0) Pause Young (Normal) (G1 Evacuation Pause) 23M->3M(260M) 0.941ms
[0.308s][info][gc] GC(1) Pause Young (Normal) (G1 Evacuation Pause) 43M->4M(260M) 1.245ms

#2
[0.662s][info][gc] GC(2) Pause Young (Normal) (GCLocker Initiated GC) 152M->8M(260M) 3.542ms

#3
[0.874s][info][gc] GC(3) Pause Young (Concurrent Start) (Metadata GC Threshold) 109M->10M(260M) 3.362ms

#4
[0.874s][info][gc] GC(4) Concurrent Mark Cycle
[0.878s][info][gc] GC(4) Pause Remark 11M->11M(54M) 0.746ms
[0.878s][info][gc] GC(4) Pause Cleanup 11M->11M(54M) 0.002ms
[0.878s][info][gc] GC(4) Concurrent Mark Cycle 3.754ms

#5
[1.000s][info][gc] GC(5) Pause Young (Normal) (G1 Preventive Collection) 40M->11M(54M) 4.661ms
[1.101s][info][gc] GC(6) Pause Young (Normal) (G1 Evacuation Pause) 33M->12M(54M) 5.834ms
[1.219s][info][gc] GC(7) Pause Young (Normal) (G1 Evacuation Pause) 40M->13M(54M) 3.073ms
[1.294s][info][gc] GC(8) Pause Young (Normal) (G1 Evacuation Pause) 37M->14M(54M) 3.314ms
```

로그 분석을 좀 해보자.

GC(0) Pause Young (Normal)은 Young Gen에서 JVM이 처음 수행안 일반적인 gc다

G1 Evacuation Pause 달려있는애들은 Young Generation 과정에서 살아남은 객체를 old generation으로 이동시키는 작업이다.

이 과정에서 잠시 gc가 멈추며 evacuation 작업이 수행된다 23 -> 3 은 gc이후에 메모리 변화량이다.

GCLocker Initiated GC라는건 GCLocker는 JNI코드가 실행되는 동안 gc를 방지하고 잠금이 해제된 이후 GC를 강제로 유발할수있는기능이다.

Pause Young(Concurrent Start)는 young gc가 발생함과 동시에 old gen에서 concurrent mark cycle을 시작한다.

Metadata GC Threashold를 보니 메타데이터 영역이 gc 임계값에 도달해 gc가 발생했다 주로 클래스 메타데이터나 관련 정보가 많이 쌓일때 트리거된다.

Concurrent Mark Cycle: old gen에 대한 마크 작업을 진행하고 이는 young gc와 별개로 old generation의 객체를 마크하고 추적하는 동작이다.

Pause Remark는 Concurrent Mark 작업이 완료된 이후 Remark 단계에서 GC가 일시적으로 중단된것이고 Pause Cleanup은 청소 작업을 위해 GC 일시적 중단이다.