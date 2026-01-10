# TLAB(Thread-Local Allocation Buffer)

TLAB은 각 스레드가 young에서 미리 떼어 받아두는 전용 할당 구역이다.

이 구역 안에서는 객체 생성이 락 없이 pointer만 증가시키는 방식으로 매우 빠르게 된다.

eden은 원래 연속으로 쌓는 공간인데, young eden은 빈 공간에 객체를 순서대로 넣는 방식으로 할당한다.

가장 단순한 모델은 전역 포인터 하나다.

- `eden_top`: 다음 객체를 놓을 위치
- `eden_end`: eden의 끝

객체를 만들때마다 객체 크기 size를 계산하고 if (eden_top + size <= eden_end) 인지 확인후에 addr = eden_top; eden_top += size를 진행한다.

이게 **bump-pointer allocation**이다. 빠른 이유는 free list 탐색 같은게 없고 포인터만 올리기 때문

문제는 멀티 스레드에서는 eden_top 하나를 공유하고 있기 때문에 race condition이 있을수있고 매 할당마다 CAS 연산을 하거나 경쟁 심하면 재시도해야되고

같은 캐시 라인을 여러 코어가 번갈아쓰면 또 튕긴다 이를 cache line bouncing이라고 한다

**즉 락을 쓰든 cas를 쓰든 공유 포인터 업데이트 경쟁 때문에 할당이 느려진다.**

이게 락 비용의 본질이고 아마 mutex 아니여도 공유 자원 경쟁 자체가 비용일듯

### TLAB

TLAB은 해당 공유 포인터 경쟁을 없애기위해서 eden을 조각내서 나눠주는 것이다.

jvm이 eden을 스레드 단위로 작은 구간을 떼어준다.

전역은 eden이라는 큰 땅이라는 뜻이고 스레드별은 그 땅 구간마다 임대받는것. 그것이 TLAB

각 스레드는 자기 TLAB에 대해 아래 3개 포인터만 갖는다.
- `tlab_start`
- `tlab_top`
- `tlab_end`

중요한점은 이 포인터들은 **해당 스레드만 만진다.**

그러니까 tlab_top += size 같은 연산은 원자성도 필요없고 락도 필요없다 그냥 레지스터 로컬캐시에 있는 값을 올리는 수준

락 없이 포인터만 증가한다는 것의 의미는 전역 공유 변수를 매 할당마다 갱신할 필요 없고 CAS Loop나 락경합이 없고 캐시라인 bounce가 확 줄어든다.

- size 계산
- new_top = tlab_top + size
- if (new_top <= tlab_end)
  - addr = tlab_top; tlab_top = new_top
- 헤더(마크워드/클래스포인터) 세팅 후 반환

위처럼 객체 생성이 동작하게 된다. 이 루프는 다른 스레드와 경쟁하는 공유자원이 없어서 매우 빠르다.

즉 멀티스레드에서 힙 할당 포인터 경쟁을 없애기 위해서 스레드별로 별도 버퍼를 미리 지급해 거기서만 연산하도록 해놓은것이다.

<br>

### Fast Path vs Slow Path

Fast Path는 TLAB 내부 할당이고 락 없고, cas없고 매우 빠르며 대부분의 작은 객체는 여기로 들어간다.

SLow Path는 TLAB의 재할당(refill) 또는 밖의 할당 (outside TLAB)인데 다음 상황이면 느린 경로로 간다.
1. TLAB 공간이 부족: 새로 TLAB을 eden에서 다시 떼어오거나(refill)
2. TLAB에 넣기 애매한 큰 객체: TLAB 밖에 바로 할당 (전역 포인터 CAS 사용)
3. Eden 여유가 부족: 결국 young gc 트리거 압력이 올라갈 수 있음

즉, 성능 관점에서 TLAB이 잘맞는가는 할당이 fast path에 얼마나 머무는가로 결정됨

### GC와의 관계, 낭비

TLAB은 gc가 수집하는 별도 영역이 아니다.

TLAB은 eden의 일부 스레드가 전용으로 쓰는 것 뿐 young gc가 일어나면 eden 전체를 정리할때 TLAB 안에있는 객체도 일반 eden 객체처럼 처리가 되게된다.

여기서 TLAB 낭비(waste)라는 개념이 생길 수 있는데 TLAB은 스레드에게 덩어리로 준다 글런데 스레드가 gc 직전에 할당을 멈추면 TLAB 끝부분이 남는다.

결과적으로 eden안에는 자투리가 생기고 이를 TLAB waste라고 한다. JVM은 이 낭비가 과해지지 않도록 TLAB 크기를 동적으로 튜닝한다.

(할당률/스레드수/리필빈도/낭비율 등을 보며 조절)

<br>

### 큰 객체와 TLAB (Humongous와는 구분해서 이해해야함)

여기서는 크개 두 개를 구분해야하는데  TLAB 밖에 할당하는 allc과 G1의 Humongous Object에 관한 구별을 해야된다.

#### TLAB 밖에 할당 (outside TLAB)

단순히 TLAB에 넣기엔 크거나/정렬/조건상 비효율이라서 eden에 전역 할당 방식으로 들어가는 경우다.

이건 gc알고리즘(g1/parallel 등)과 무건화게 할당 경로 관점이다

#### Humongous Object

이건 g1에서 객체가 리전 크기의 일정 비율(보통 50%이상)을 넘으면 young(eden) 일반 할당이 아니라 humongouse region을 별도로 잡는 정책이다.

humongouse는 g1의 리전 정책이고 outside tlab은 할당 경로다. 큰 객체는 outside TLAB이 되는 경우가 많고 그 중 g1에서는 조건을 만족하면 humongouse로 갈수도 있다.

### GC 알고리즘 별 TLAB

#### Serial / Parallel / CMS / G1

- 공통: 애플리케이션(뮤테이터)쪽 할당 최적화로 TLAB을 사용한다.
- 차이: young gc 방식(복사/리전/병렬)이 다를뿐 **스렏드 별로 빠른 할당**이라는 TLAB의 본질은 같다.

#### 참고: gc 복사시 PLAB과 구분

gc가 살아있는 객체를 다른곳으로 복사할 때도 버퍼가 나오는데 그건 보통 **PLAB(Promotion Local Allocation Buffer)**같은 이름으로 부른다.

- **TLAB**: 애플리케이션 스레드가 새 객체를 생성할 때
- **PLAB**: GC 스레드가 evacuation/promotion 복사할때 서로 목적이 다르다.

<br>

## Debuggingg TLAB

TLAB 때문에 느린가? 에 대한것을 확인하는 방법을 몇가지 알아보자.

JFR 이벤트로 다음을 보면 바로 감이 온다.
- `ObjectAllocationInNewTLAB` (새 TLAB 받아서 할당)
- `ObjectAllocationOutsideTLAB` (TLAB 밖 할당)

OutsideTLAB가 비정상적으로 많으면 
- 객체가 큰 편이거나
- TLAB 크기/튜닝이 안맞거나
- 특정 코드 경로에서 큰 배열/버퍼를 자주 만들거나를 의심 가능

JVM Flag로 `-XX:+PrintTLAB` (TLAB 통계 출력), `-XX:+PrintGCDetails` (gc 상세 로그와 같이 봄) jdk9+ q부터는 `-X:log:gc+tlab=debug` (환경에 따라 trace debug 레벨 조정) 등으로 로그 확인이 가능한데

출력에서 refill 횟수(너무 잦으면 tlab이 작거나 할당이 많거나), waste(너무 크면 tlab이 과하게 크거나 스레드 패턴이 불리하거나), outside tlab 비율(큰 객체/특이 할당 패턴) 등을 체크해볼 수 있다.

- refill이 너무 잦으면 -> Fast Path가 자주 끊김 -> 성능 손해 가능
- waste가 너무 크면 eden이 낭비되고 -> young gc 압력이 올라갈 수 있음
- outside TLAB이 많으면 큰 객체/배열/버퍼 생성 이 많을 가능성이 있으니 체크해보자

- `-XX:+UseTLAB` 보통은 켜져있고 끄면 대개 손해긴 하다
- `-XX:TLABSize`/ `-XX:MinTLABSize`: 고정/최소 크기 관련이다.
- `-XX:TLABWasteTargetPercent`: TLAB낭비 허용률이고 너무 공격적으로 바꾸면 역효과가 있을 순 있다. cpu 너무 쓴다거나

정리해보면 

JFR Allocation Profiling으로 outside alloc 원인을 찾아볼 수 있고

큰 객체, byte buffer, array, json, string 등 생성위치를 줄이거나 재사용하도록 코드 수정을 해보고 그 다음에 TLAB 파라미터를 만지자.

대부분의 튜닝에선 본질적으로 vm의 세팅이 잘못되어 발생한 비효율도 있겠지만, 애플리케이션 코드레벨에서의 낭비가 대게 더 많이 발생하게 된다.

이 철학을 잘 가지고 성능 개선을 해보자.