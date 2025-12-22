# Concurrent Marking

Concurrent Marking은 g1gc가 애플리케이션을 멈추지 않고 힙 내의 모든 살아있는 객체를 추적하는 과정이다.

이 과정이 성공적으로 끝나야 비로소 Mixed GC를 수행할 수 있다.

즉 Mixed GC를 위한 사전 조사 단계라고 보면 된다.

총 5단계로 나뉘며 각 단계가 STW인제 Concurrent인지 튜닝의 핵심이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FbtksbS%2FbtsJyKslIOz%2FAAAAAAAAAAAAAAAAAAAAANN-zXo236hff-P6l0Ehm2mBfxLFdZmwKTdtF7184dkL%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1767193199%26allow_ip%3D%26allow_referer%3D%26signature%3Dn%252FTAni8%252F8MfsP4QYFovBxaRk5X0%253D)

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