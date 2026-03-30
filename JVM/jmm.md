# JMM과 하드웨어 교차점

Java Memory Model과 실제 하드웨어 아키텍처 x86, ARM등 간의 매핑은 현대 고성능 멀티스레드 애플리케이션의 핵심이다.

### JMM과 하드웨어의 추상화 계층

JMM은 개발자에게 Happens-Before라는 논리적 보증을 제공하지만 하드웨어는 성능 극대화를 위해 비순차적 실행 out of order execution과 캐시 계층 구조를 사용한다.

JVM의 역할은 이 논리적 명령을 각 cpu의 메모리 일관성 모델에 맞는 어셈블리 명령어로 치환하는 것이다. 

- x86(Intel/AMD)는 TSO(Total Store Ordering) 하드웨어가 로드/스토어 순서를 상당히 엄격하게 유지하고 재정렬 범위는 Store-Load 재정렬만 발생한다 Store Buffer 때문 그래서 Java Barrier 삽입 비용이 상대적으로 낮다
- ARM(AArch64, Apple Silicon)은 Weakly Ordered고 매우 유연하며 명시적 배리어 없이는 순서 보장이 거의 없다. Load-Load, Load-Store, Store-Store등 대부분 발생 가능하며 베리어 삽입시 성능 차이가 지점에  따라 극명하다.


### 키워드별 하드웨어 매핑

#### Volaitle 변수 (Load-Acquire / Store-Release)

Java 21의 Hotspot JVM은 `volatile` 변수에 대해서 단순히 메모리에 읽는 것 이상의 처리를 한다

- **x86 아키텍처**
  - **Read**: 일반 mov 명령어를 사용하고 TSO 덕분에 추가 배리어가 필요없다
  - **Write**: mov 이후에 `lock addl $0, 0(%rsp)`와 같은 빈 lock 연산을 수행하거나 mfence를 사용한다. 이는 store-load 재정렬을 막아 가시성을 보장한다
- **ARM 아키텍처 LSE 확장 포함**
  - **Read**: `ldor`(Load-Acquire) 명령어를 사용하여, 이 읽기 이후의 연산이 이전으로 올라가지 못하게 막는다.
  - **Write**: stlr 명령어를 사용해 이 쓰기 이전의 연산이 이후로 내려가지 못하게 막는다. 과거의 dmb 배리어보다 훨신 효율적이다.

#### CAS (Compare-And-Swap) 및 Atomic 연산

`VarHandle` 이나 `Atomic` 클래스의 핵심인 CAS는 하드웨어의 원자적 명령어로 직접 매핑된다.

- **x86**: `lock cmpxchg` 명령어를 사용한다. lock 프리픽스가 붙으면 전체 캐시 라인에 대한 독점권을 확보하고 메모리 베리어 효과를 동시에 가진다.
- **ARM**: 과거에는 ldxr/stxr Load-Link / Store-Conditional 루프를 사용했으나 java 21이 구동되는 최신 환경 ARMv8.1+ 환경에서는 cas 명령어를 직접 사용해 하드웨어 수준에 원자성을 확보한다.


### OpenJDK 내부의 메모리 베리어 구현

JVM 내부 소스코드 `src/hotspot/os_cpu` 같은곳을 보면 OrderAccess 클래스가 각 아키텍처에 맞게 정의되어 있다.

- LoadLoad (No-op): `dmb ishld` or `ldar` 활용
- StoreStore (No-op): `dmb ishst` or `stlr` 활용
- LoadStore (No-op): `dmb ish`
- StoreLoad (lock addl or mfence): dmb ish (가장 무거운 베리어)

x86 에서는 Store Load를 제외한 나머지 3가지 배리어가 하드웨어 수준에서 이미 보장되므로 No-op(아무 연산 안함) 처리되어 성능상 이점이 크다 하지만 ARM은 이를 명시적으로 처리해야한다.

### 최적화 전략

#### Flase Sharing 거짓 공유 방지

java21에서도 여전히 중요한 이슈인데 cpu 캐시라인 주로 64바이트에 서로 다른 스레드가 사용하는 변수가 묶여있으면 한 스레드의 volatie 쓰기가 다른 스레드의 캐시를 무효화 한다.

`@jdk.internal.vm.annotation.Contended`를 사용하여 필드 간의 간격을 강제로 벌린다 (JVM 옵션 `-XX:-RestrictContented`) 필요

#### VarHandle의 활용 (Opaque vs Plain)

모든 공유 변수를 volatile로 선언할 필요는 없다. Java 9부터 도입되어 21에서 완성도가 높아진 `VarHandle`을 사용하면 더 세밀한 제어가 가능하다.

- **Opaque**: 순서는 보장하지 않지만 값의 가시성만 보장하고 싶을 때 사용한다. ARM에서 `ldar`/`stlr` 대신 일반 `mov`와 유사한 비용으로 성능을 높일 수 있따.
- **Release/Acquire**: 전체 volatile 보다 가벼운 베리어를 형성하여 특정 방향의 재정렬을 방지한다.

#### 가상스레드와의 상호작용

java21은 가상 스레드 수천개가 생성될 수 있다 이때 synchronized 블록 내에서 io를 실행하면 carrier thread가 pinning 되는 현상이 발생할 수 있다.

메모리 모델 관점에서는 ReentrantLock을 사용하는 것이 가상 스레드 스케줄링과 하드웨어 자원 활용 측면에서 더욱 유리하다.