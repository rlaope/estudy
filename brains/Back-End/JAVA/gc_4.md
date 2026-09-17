# Garbage Collector 4가지 종류

가비지 컬렉터에서 가비지 즉 쓰레기는 객체를 의미하며 더 이상 사용되지 않고 메모리만 점유하고 있는 객체는 정리를 해야한다.

이러한 객체 정리 로직은 개발자가 직접 구현할 필요가 없으며 하면 안된다.

GC는 여러 환경에서 제공되어지는 개념이며, 튜닝을 위해서 개발자가 직접 설정을 할 수 있다.

GC 작업을 하는 가비지 콜렉터는 다음과 같은 역할을 한다.

1. 메모리 할당
2. 사용하고 있는 메모리 인식
3. 사용되지 않는 메모리 인식

> GC의 young, old, perm 영역이나 객체가 할당되고 지워지는 과정, mark and sweep등은 여기서 알아보지 않겠다. 다른 글을 참고하기 바란다.

GC에서는 두가지 GC가 일어나는데 young 영역에서 일어나는 마이너 GC와 old에서 일어나는 메이저 gc가 있다. 이 둘은 어떻게 상호작용하느냐에 따라서 GC 방식에 차이가 난다.

GC가 발생하거나 객체가 각 영역에서 다른 영역으로 이동할 때 애플리케이션의 병목이 발생하면서 성능에 영향을 주게 된다.

**Hotspot VM**에서는 TLABs(Thread-Local Allocation Buffers) 스레드 로컬 할당 버퍼라는 것을 사용한다.

이를 통해서 각 스레드별 메모리 버퍼를 사용하면 다른 스레드에 영향을 주지 않는 메모리 할당 작업이 가능하게 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F224A224358FF17593F)


### 4가지 GC 방식

JDK 5.0 이상에는 다음과 같은 GC 방식들이 있다.

WAS나 자바 애플리케이션 수행시 이 옵션들을 적용해서 선택이 가능하다.

- Serial Collector
- Parallel Collector
- Parallel Compacting Collector
- Concurrent Mark-Sweep(CMS) Collector


### Serial Collector

Young, Old 영역이 시리얼하게(연속적으로) 처리되며 하나의 CPU를 사용합니다.

이 처리를 수행할 때를 Stop the world라고 합니다. (애플리케이션이 수행하고 있는 모든 작업을 중지하고 멈추게됩니다.)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F2703E43958FF0CD22F)

일단 살아있는 객체들은 eden 영역에 있습니다. eden 영역이 꽉 차게 되면 To Survivor 영역(비어있는 영역)으로 살아있는 객체가 이동됩니다. 이 때 Survivor 영역에 들어가기에 너무 큰 객체는 바로 Old로 갑니다. 그리고 From Survivor 영역에 남아있는 객체들은 Old 영역으로 이동합니다. Old 영역으로 이동하는 것을 Promotion이라고합니다.

이후 Old 영역이나 Perm 영역에 있는 객체들을 Mark-sweep-compact 콜렉션 알고리즘을 따릅니다.

간단히 안쓰는 것을 표시하고 정리하고 모으는 알고리즘입니다.

일반적으로 시리얼 콜렉터는 클라이언트 종류의 장비에 많이 사용됩니다, 즉 대기 시간이 많아도 크게 문제되지 않는 시스템에서 사용되는 의미입니다.

명시적 지정 방법

```
-XX:+UseSerialGC
```


### Parallel Collector

이 방식은 throughput collector로도 알려진 방식이다.

이 방식의 목표는 다른 cpu가 대기 상태로 남아 있는 것을 최소화하는 것이다.

시리얼 콜렉터와 다르게 young 영역의 콜렉션을 병렬로 처리한다.

많은 cpu를 사용하기 때문에 gc의 부하를 줄이고 애플리케이션의 처리량을 향상시킨다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F266ED54458FF183C06)

old 영역의 gc는 시리얼과 마찬가지로 Mark-sweep-compact 알고리즘을 사용한다

명시적 지정 방법
```
-XX:+UseParallelGC
```

### Parallel Compacting Collector

병렬 콜렉터와 다른 점은 old의 gc에서 새로운 알고리즘을 사용한다.

즉, young 영역에서의 gc는 병렬 콜렉터와 동일하다.

old 영역의 gc는 다음과 같은 3단계를 거친다.

1. Mark: 살아 있는 객체를 표시해놓는다.
2. Sweep: 이전에 GC를 수행하여 컴팩션된 영역에 살아 있는 객체의 위치를 조사한다.
3. Compact: 컴팩션을 수행한다. 수행 이후에는 컴팩션된 영역과 비어있는 영역으로 나뉜다.

병렬 콜렉터와 동일하게 이 방식도 여러 cpu를 사용하는 서버에 적합하다.

gc를 사용하는 스레드의 개수는 - XX:ParallelGCThreads=n 옵션으로 조정할 수 있다.

명시적 지정 방법
```
-XX:+UseParallelOldGC
```

### CMS 콜렉터

이 방식은 low-latancy collector로도 알려져 있습니다.

힙 메모리 영역의 크기가 클 때 적합합니다. 

Young 영역에 대한 GC는 병렬 콜렉터와 동일합니다.

Old 영역에 GC는 다음과 같은 단계를 거칩니다.

1. Mark: 매우 짧은 대기 시간으로 살아 있는 객체를 찾는다.
2. Sweep: 서버 수행과 동시에 살아 있는 객체를 표시해 놓는다.
3. Remark: Concurrent 표시 단계에서 표시하는 동안 변경된 객체에 대해서 다시 표시하는 단계다.
4. ConcurrentSweep: 표시되어 있는 쓰레기를 정리하는 단계다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F24208F4958FF1D9E2B)

CMS는 컴팩션 단계를 거치지 않기 때문에 왼쪽으로 메모리를 몰아 놓는 작업을 수행하지 않는다.

그래서 GC 이후에 그림과 같이 빈 공간이 발생하므로 ` -XX:CMSInitiatingOccupancyFraction=n`  옵션으로 Old 영역의 %를 n값에 지정한다. 기본값은 68이다.

CMS 콜렉터 방식은 2개 이상의 프로세서를 사용하는 서버에 적합하다. 가장 적당한 대상으로는 웹서버가 있다.

명시적 지정 방법

```
-XX:+UseConcMarkSweepGC
```

CMS 콜렉터는 추가적인 옵션으로 점진적 방식을 지원한다.

이게 뭐냐면 Young영역에서 GC를 더 잘 쪼개어 서버의 대기시간을 줄일 수 있는 것이다.

CPU가 많지 않고 시스템에 대기 시간이 짧아야할 때 사용하면 좋다.

점진적 GC를 수행하려면 `-XX:+CMSIncrementalMode` 옵션을 지정하면 된다.

JVM에 따라서는 `-Xingc`라는 옵션을 지정해도 같은 의미가 된다.

하지만 이 옵션을 지정할 경우 예기치 못한 성능 저하가 발생할 수도 있다. 충분한 테스트를 거치고 운영서버에 적용하도록 하자.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F2129254758FF215238)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F2511F44C58FF216E0F)


위의 내용을 종합하면, GC는 각 영역의 할당된 크기의 메모리가 허용치를 넘을 때 수행하며 개발자가 컨트롤할 영역이 아니다.

개발자라면, 자바의 GC 방식을 외우면서 개발하거나 서버를 세팅할 필요는 없고 이해만 하면 되지만 단, 필요할때 알맞는 GC 방식을 개발한 시스템에 적용하면 된다고 한다.