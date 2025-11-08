# JVM Safepoint

safepoint는 JVM 내부에서  **스레드의 실행을 일시 중단시키는 지점을 의미한다.**

GC, deoptimization, thread dump 등 jvm 내부 관리 작업을 수행하기 위해 모든 자바 스레드를 안전하게 멈춰야 할 때 사용된다.

jvm 특정 작업 gc같은거를 수행하기 위해 애플리케이션 스레드의 실행 상태를 일관된 시점으로 맞추는 기법이다.

왜냐면 스레드가 아무 위치에서 멈추게 되면, 메모리 상태가 불완전해질 수도 있으므로 jvm이 지정한 안전한 지점 즉 safepoint에서만 멈출수 있게 되는것이다.

### 동작 과정

1. jvm이 safe point를 요청한다 `Safepoint request`
2. 모든 java 스레드에게 safepoint에 진입하라 라는 신호를 보낸다.
3. 각 스레드는 jit compile된 코드 내의 safepoint polling 위치에서 신호를 감지하고 멈춘다.
4. 모든 스레드가 멈추면 jvm은 gc, class redefinition과 같은 작업을 수행한다.
5. 4의 작업이 완료되면 safepoint를 해제하고 스레드들을 재개한다.

발생시점 예시로는

Garbage Collection

Thread dump (jstack 실행 시)

Deoptimization (JIT 코드 → 인터프리터 코드로 변경)

Class redefinition (HotSwap)

Bias locking revocation

등을 들어볼 수 있을듯.

> Bias Locking Revocation(바이어스 락 해제)는 JVM의 락 최적화 기법 중 하나인 Biased Locking이 해제되는 과정


### 성능관련 이슈

safepoint는 시스템의 성능에 직접적인 영향을 줄 수 있다.

모든 스레드가 safepoint에 진입해야만 jvm이 관리 작업을 시작할 수 있기 때문에

어떤 스레드가 cpu를 오래 점유하거나 네이티브 코드에 머무르면 safepoint 진입 지연이 발생한다. (Safepoint Stall)

이 지연시간은 gc로그에 아래처럼 뜰수있음.

```sql
Safepoint "Cleanup", Time since last: 100000 ms, Time to safepoint: 1234 ms
```

여기서 Time to safepoint 값이 크다면, 특정 스레드가 safepoint polling을 오랫동안 수행하지 못하고 있다는 의미임.

jit compile된 코드는 다음과 같은 폴링 구문이 삽입되어있음.

```java
while (!SafepointSynchronize::do_call()) {
    // regular Java execution
}
```

이 poling은 함수 호출 전, 루프 진입 전 등 jvm이 안전하다고 판단하는 시점에 위치하며 네이티브 코드나 장시간 블로킹되는 i/o 호출 중에는 polling이 불가능하므로 safepoint 지연의 주요 원인이 된다.

### 진단법

- GC 로그에서 Time to safepoint 확인
- `-XX:+PrintGCApplicationStoppedTime` 옵션 활성화
- `-XX:+PrintSafepointStatistics` 로 세부 통계 확인
- jstack 혹은 jcmd VM.safepoint 사용

