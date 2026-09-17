# Bias Locking, Revocation

`synchronized` 블럭은 내부적으로 monitor lock을 이용한다.

보통 락은 cas같은 원자 연산을 통해 획득 및 해제가 되는데, 이 cas는 cpu 명령 단위로 lock을 걸기때문에 비싸다.

대부분의 애플리케이션에서는 **항상 같은 스레드가 같은 객체를 잠근다**는 패턴이 매우 흔하다.

```java
void foo() {
    synchronized (lock) {
        // 같은 스레드에서 반복 호출
    }
}
```

그래서 매번 cas하는게 불필요하며 jvm은 이 패턴을 최적화하기 위해

bias locking 편향락을 도입했다.

락 객체를 특정 스레드에 편향 시켜 이후 동일 스레드가 락을 획득할 때 cas를 생략하는 최적화다.

객체 헤더 mark workd에 thread id를 저장해 이 객체는 해당 스레드에 편향되어있다고 나타낸다.

이 값을 통해서 동일 스렏드에 재진입시 cas없이 빠르게 락을 획득할 수 있다.

상태 전이 과정:
- biased
- lightweight lock
- heavyweight lock 순으로 전환

비활성화 하려면 `-XX:-UseBiasedLocking`를 쓸수있긴한데 그리고 jdk 15이후에는 이 옵션 사라졌다고 한다. 그냥 안쓰는걸로 왜냐구? 그건 아래에서 멀티스레드 환경과 함께 설명드리겟다

mark word는 아래처럼 생겼는데

```
[ ThreadID | Epoch | Biased bit | Lock bits | Age | HashCode ... ]
```

마크데이터는 객체의 메타데이터를 담고있는 헤더값이고 이곳에 편향할 스레드 정보를 직접 기록한다.

근데 문제는 단일 스레드에선 편한데 다중 스레드 환경에서는 오히려 오버헤드가 될수있다.

여러 스레드가 동일 객체를 번갈아 잠가 bias가 자주 깨지고 다시 재설정된다면 frequent bias recovation(회수)이 발생해 safepoint로 인한 stw가 발생한다.

gc중 클래스 epoch 갱신을 하게되면 기존 객체 bias가 무효화가 되어 stw가 또 발생할수 있으며

다중 스레드 초기화 패턴 각 스레드가 처음 접근시 bias reassign 반복해 startup latency가 발생할수도 있다.

safepoint 발생 빈도가 증가해서 stop latancy도 증가함

이런 이유로 멀티스레드 경합이 높은 서비스 환경에서는 비활성화 하는 경우가 많긴하다.

jdk 15 이상부터는 그냥 삭제되어있다. 이게 없음.

멀티스레드 서버에서는 비활성화가 걍 기본선택지니까. 8~14면 아까 위에 나온 옵션으로 비활성화 시켜두자.

### 운영 방안

safe point 분석 bias revocation이 safepoint를 유발하기 때문에 아래 옵션들로 진단이 가능하다.

```
-XX:+PrintSafepointStatistics -XX:+UnlockDiagnosticVMOptions
-XX:+PrintGCApplicationStoppedTime
```

이 중 `Time to safepoint`가 큰 경우, bias recovation이나 native 대기 스레드가 병목일 가능성이 높다.


bias 해제 정책 조정:  
jvm은 클래스 단위로 bias 정책을 관리하는데 클래스 레벨 epoch 증가시 해당 클래스 객체의 bias가 무효화되고 특정 클래스가 bias 전환을 자주 일으키면 jvm이 자동으로 bias 비활성화를 결정한다.

이 정책은 자동이므로 일반적으로 수동 조정은 어렵지만 특정 클래스가 경합을 유발하는가 jfr이나 gc log를 통해서 확인할 수 있다.

최신 JDK에서는 Lightweight Lock과 Lock Elision 기술로 대체

```
③	Lightweight Lock (경량 락)	CAS 기반	여러 스레드가 교대로 접근 (짧은 경쟁)

④	Heavyweight Lock (중량 락)	OS 수준 monitor (mutex) 사용	락 경쟁이 심하거나 오래 대기하는 경우
```