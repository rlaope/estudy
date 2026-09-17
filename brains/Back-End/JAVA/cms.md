# CMS GC (튜닝 전략 parallel, cms)

Concurrent Mark Seep GC는 이름에서 알 수 있뜻

애플리케이션의 지연시간을 최소화하기 위해 애플리케이션과 동시에 실행되는 gc다.

자바 9부터는 deprectaed 되었고 자바 14에서 완전히 제거되었지만 g1 gc나 zgc같은 현대적 컬렉터의 기반이되는 중요한 개념을 담고있다.

### STW를 줄여라

cms gc의 기본 철학인데 기존 serial, parallel gc는 실행되는 동안 모든 애플리케이션 스레드를 멈춘다.

하지만 cms는 전체 과정을 쪼개어 일부 단계만 stw를 발생시키고 나머지 대부분은 애플리케이션과 함께 실행해 응답성을 높인다.

#### young gc와 old gc의 조합

cms gc는 주로 old generation을 관리하며, young generation은 보통 ParNew GC라고 불리는 멀티스레드 기반의 copying gc를 함께 사용한다.

- **ParNew**: minor gc 발생시 살아남은 객체를 survivor 영역으로 옮기며 stw가 발생한다.
- **Old GC CMS**: old 영역의 객체를 정리할 때 4단계 과정을 거친다.

### CMS GC 동작 4단계

1. **Initial Mark (STW 있음)**
   1. gc root에 직접 참조되는 객체들만 찾는 특징이 있다. 아주 가까운 객체만 찾기 때문에 stw시간이 짧다
2. **Concurrent Marking (STW 없음)**
   1. 목적은 initial mark에서 생존한 객체를 따라가며 모든 참조 관계를 추적한다.
   2. 애플리케이션이 돌아가는 와중에 별도 gc 스레드가 작업을 수행한다. 프로세서 자원을 소모하지만 앱은 멈추지 않는다.
3. **Remark (STW 있음)**
   1. Concurrent Mark 동안 애플리케이션이 객체 참조를 변경했을 수 있다. 이 변경사항을 다시 확인한다. 
   2. 두 번째 stw가 발생해 이전 단계 오차를 잡기위해 필수적이다.
4. **Concurrent Sweep (STW 없음)**
   1. 도달 불가능한 unreachable 객체들을 메모리에서 제거한다
   2. 애플리케이션이 실행되는 동안 청소를 진행한다.

### CMS GC의 한계와 Full GC

cms는 응답성을 위해 큰 비용을 지불한다.

**메모리 파편화** 문제가 잇는데 기본적으로 cms는 compaction 단계를 하지 않는다. 

즉 빈공간을 중간중간 메꾸지않고 그냥 나둬서 시간이 지나면 메모리 총량은 충분해도 큰 객체가 들어갈 공간이 없는 조각난 상태가 된다.

**Concurrent Mode Failure(Full GC 원인)**도 존재하는데 old 영역 공간이 부족해지기전에 gc가 끝나야 하는데, 객체 할당속도가 gc 속도보다 빠르면 concurrent mode failure가 발생한다.

**Fallback serail old gc**: 위와 같은 파편화로인한 공간부족, 할당속도 초과등이 발생하면 cms는 포기하고 full gc를 수행한다.

이때 full gc는 싱글 스레드로 동작하는 serial old gc로 동작하여 매운 긴 stw가 발생하고 이는 시스템 전체의 성능 저하로 이어진다.

### CMS = Parallel ?

둘을 비교해보면 둘다 멀티스레드를 사용하게 되는데 gc에서 parallel과 concurrent는 엄연히 다른 의미로 쓰인다.

Parallel GC는 병렬 gc를 수행할때 여러 개의 스레드를 동원한다. 하지만 gc가 도는 동안 애플리케이션은 완전히 멈춘다. 빨리 처리하고 다시 일하는게 목표

CMS GC는 청소할때 여러 스레드를 쓰는건 같지만 애플리케이션과 gc 스레드가 동시에 concurrent 돌아간ㄷ. 멈추지 말고 조금씩 계속 치우자가 목표다

ParNew GC: CMS GC는 보통 young 영역을 청소할때 par new라는 컬렉터를 사용하는데 ParNew가 Parallel Young Generation Collectior의 약자다. cms는 young 영역의 청소용으로 parallel 기술을 빌려 쓰는 구조라서 헷갈릴 수 있다.

CMS는 full gc가 serial로 돌기때문에 full gc는 피해야할 대상이고 parallel은 무조건 발생하는 루틴같은 존재다.

그래서 cms를 쓸때는 fragmentation을 피해야하고. Concurrent Mode Failure, Promotion Failure를 잘 검출해내야한다. young영역에 넘어오는 객체를 old에 넣을 공간이 없다거나 그런형상을 피해햐아함.


<br>

## Parallel, CMS 튜닝 전략

### 힙 크기의 고정과 최적화

가장 기본이면서 중요한 단계인데, jvm이 힙 크기를 동적으로 조절할때발생하는 오버헤드를 줄여야한다.

`-Xms`, `-Xmx`를 동일하게 설정해 힙 크기 조절시 발생하는 stw와 부하를 방지하기위해 최소/최대 크기를 같게 맞춰야한다. `-Xms4g` `-Xmx4g`

`-XX:MetaspaceSize`, `-XX:MaxMetaspaceSize`: java8부터는 permGen대신 metaspace를 사용한다. 이 영역이 꽉 차서 발생하는 full gc를 방지하기 위해 적절한 초기값을 할당해야한다. (예: -XX:MetaspaceSize=256m -XX:MaxMetaspaceSize=256m)

### young old 비율 조정

parallel gc에서 가장 빈번하게 발생하는 것은 minor gc고 대부분 객체는 금방 죽는다는 세대 가설에 따라 young 영역을 효율적으로 써야한다.

- **NewRatio 조절**: 전체 힙중 young 영역의 비율을 정해야한다.
  - `-XX:NewRatio=2` 깁녹밧은 2:1 비율로 old young이 가져간다
  - 만약 단기 생존 객체가 매우 많은 api 서버라면 1로 낮춰서 young 영역을 늘리는것이 유리할 수 있다.
- **SurvivorRatio 조절**: eden 영역과 survivor 영역의 비율이다.
  - `-XX:SurvivorRatio=8` (Eden 8 s0: 1 s1: 1)
  - 객체가 survivor 영역에서 충분히 머물다 죽게하여 old 영역으로 넘어가는 양을 줄여야한다.
  
### Parallel GC 전용 튜닝 파라미터

parallel gc는 사용자가 목표로 설정하면 jvm이 알아서 최적화해주는 Ergonomics 자율 최적화 기능이 강력하다

`-XX:MaxGCPauseMillis`로 가장 중요한 목표 설정값 gc pause time을 Nms이하로 해줘 설정이 가능하다 이 값이 너무 작으면 힙 크기가 줄고 gc가 자주 발생하므로 주의해야한다 보통 200~500ms로 시작

`-XX:GCTimeRatio=N`: 전체 시간중 gc에 사용하는 시간의 비율을 설정한다 (1 / (N + 1))기본값은 99(1%)이고 만약 19로 설정하면 5%까지 gc에 쓰는걸 허용한다는 의미이다.

`-XX:ParallelGCThreads=<N>`: GC를 수행할 스레드의 개수고 보통 cpu코어수와 동일하게 설정하지만 한 서버에 여러 jvm을 띄운다면 코어수보다 적게 설정하자.

### Promotion 튜닝

young gc에서 살아남은 객체가 old로 넘어가는 기준을 조정한다.
- -XX:MaxTenuringThreshold=<N>
  - survivor 영역에서 몇 번을 살아남아야 old로 보낼지 결정한다. 기본값은 15
  - old 영역의 full gc를 줄이려면 이값을 높게 유지하는것이 좋으나 너무 높으면 survivor 영역이 터질 수 있다.

모니터링 로그 설정도 해주는게 좋다. 튜닝은 감이 아닌 데이터로 해야하니까

jdk 8기준에선 다음과같이 옵션들을 줄 수 있다/
```bash
-XX:+PrintGCDetails 
-XX:+PrintGCDateStamps 
-Xloggc:/logs/gc.log 
-XX:+UseGCLogFileRotation 
-XX:NumberOfGCLogFiles=10 
-XX:GCLogFileSize=100M
```

### CMS 튜닝

cms gc의 튜닝 목표는 어떻게든 full gc가 발생시키지 않는거(g1이랑 비슷함)다. cms는 파편화에 취약해서 old영역이 가득차기전에

미리미리 청소를 시작하고 객체가 old 영역으로 넘어오는 양을 조절하는 것이 핵심이다.

open jdk환경에서 cms gc를 튜닝할때 반드시 살펴봐야할 핵심 파라미터와 전략을 정리한다.

### GC 시작 시점 (가장 중요)

cms는 애플리케이션과 동시에 돌아가므로 old 영역이 꽉찬뒤에 시작하면 늦는다 어느정도 차면 청소를 시작할지를 정하는것이 튜닝의 80%이다.

`-XX:CMSInitiationgOccupancyFraction=<N>` 기본값은 68~92%고 old 영역의 힙 사용량이 n에 도달하면 cms gc를 시작한다.

너무 높으면 Concurrent Mode Failure 발생 위험이 커지고 너무 낮으면 gc가 너무 자주돌아 cpu를 낭비한다. 보통 70 75정도로 낮춰서 여유 공간을 확보해두는것이 좋다.

`-XX+UseCMSInitiatingOccupancyOnly`: jvm이 상황에 따라 gc 시점을 계산하지 않고 위에서 설정한 fraction 값에 도달햇을때만 gc를 수행하도록 강제한다. 예측 가능성을 위해 보통 함께 사용된다.


### remark stw 줄이기, 파편화 full gc 대비

cms는 기본적으로 compaction을 안하지만 full gc가 발생햇을때만이라도 압축을 어떻게 할지 정할 수 있다.

`-XX:+UseCMSCompactionAtFullCollection`: full gc가 발생했을 때 메모리 압축을 수행할지 여부이다. 기본값은 true

`-XX:CMSFullGCsBeforeCompaction=N`: cms gc를 몇 번 수행한후에 full gc시 압축을할지 결정한다. 예를들어 0으로 설정하면 full gc가 발생할때마다 매번 압축을 하는데 cms의 고질적인 파편화 문제를 해결하기 위해 필수적이다.

> +로 아까 parallel 에서 다뤘던것처럼 promotion 관리도 해줄수있는데 young을 확대하거나 금방 죽는 객체들이 더 많이 eden에서 죽도록.. 생존횟수를 높이고 해도 괜찮다.

```bash
# 1. CMS 사용 선언 및 Young GC용 ParNew 지정
-XX:+UseConcMarkSweepGC 
-XX:+UseParNewGC 

# 2. GC 시작 타이밍 최적화 (70% 차면 시작)
-XX:CMSInitiatingOccupancyFraction=70
-XX:+UseCMSInitiatingOccupancyOnly

# 3. Remark 단계 단축
-XX:+CMSScavengeBeforeRemark

# 4. Full GC 발생 시 매번 압축 수행
-XX:+UseCMSCompactAtFullCollection
-XX:CMSFullGCsBeforeCompaction=0

# 5. 병렬 스레드 설정 (CPU 코어 수에 따라 조절)
-XX:ParallelGCThreads=8
-XX:ConcGCThreads=4**
```

요약하면 old 영역이 70 찼을때 미리 시작하고 remark 전에 young 영역을 청소해 멈춤을 최소화하는 그런 전략