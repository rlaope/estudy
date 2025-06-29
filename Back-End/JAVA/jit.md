# JIT Compiler

JVM에서 JIT Compiler는 런타임에서 바이트코드를 기계어로 변환하는 컴파일러다.

JVM은 자바 코드를 .class로 컴파일해 바이트코드(중간 코드)로 실행한다. 이로인해 어떤 플랫폼에서든 종속되지 않고 write once run everywhere을 달성할 수 있는 특징이 있다.

물론 바이트코드를 사용함으로써 빌드시간이 다른 네이티브 lang에 비해서 오래걸린다는 단점이 존재하기도 한다. 

JVM에서 바이트코드는 원래 인터프리터식으로 한줄씩 읽으며 실행되는데 이는 실행시간이나 코드 최적화 측면에서도 굉장히 느리다.

이를 해결하기위해 JIT Compiler는 자주 실행되는 코드들을 감지해 기계어로 변환시켜 캐싱해 실행시킨다. 

### 동작 구조

클래스 로딩 후 초기에 jvm이 바이트코드를 인터프리팅하고 반복적으로 호출되는 메서드를 hotspot 탐지하여 호출 횟수를 카운팅한다. (InvocationCounter)

특정 임계치에 도달하게 된다면 JIT이 해당 메서드를 네이티브 머신 코드로 컴파일 해놓고 컴파일된 네이티브 코드를 대체 실행해둔다. 이후부턴 해당 메서드는 인터프리팅이 되지 않는다.

JIT의 성능은 워낙 탁월하고 밑에서도 언급하겠지만 c2 컴파일러가 돌면서 최적화가 완벽하게 되었다면 고성능 서버 애플리케이션에서 아주 뛰어난 성능을 보여주게 된다.

JIT은 아래와 같은 대표적인 최적화 기법들을 사용한다.

- Inlining: 메서드 호출 코드를 실제 구현으로 바꿔서 호출해 비용 제거
- Loop Unrolling: 루프 반복 횟수를 줄이기 위해 코드를 복제함
- Escape Analysis: 객체가 메서드 안에 존재할 경우 힙이 아닌 스택에 할당함
- Dead Code Elimination: 실행되지 않는 코드 제거
- Constant Folding: 컴파일 타임에 가능한 계산은 미리 처리

```java
public class Hello {
    public static void main(String[] args) {
        for (int i = 0; i < 10_000; i++) {
            sayHi(); // 반복 호출되므로 HotSpot 됨 → JIT 컴파일됨
        }
    }

    public static void sayHi() {
        System.out.println("Hi");
    }
}
```

`-XX:+PrintCompilation` 옵션으로 어떤 메서드가 JIT 되었는지 확인이 가능하다.

<br>

## c1, c2 compiler

c1은 빠른 컴파일 (짧은 지연)을 목표로 실행되는 애플리케이션에 최적화 되어있다. 최적화 수준은 낮지만 컴파일 수준은 빠르다. 실행 초기 성능이 좋은 특징을 가지고 있다.

c2는 늦은 컴파일 시간을 갖고잇지만 고성능 최적화(긴 시간의 투자)라는 특징을 가지고 있다. 서버 앱, 장시간 실행서비스에서 안성 맞춤이며 초기엔 느리지만 점점 빨라지는 특징을 가지고 있다

### Tiered Comilation 구조 (계층적 컴파일)

jvm은 성능과 컴파일 비용 모두를 고려해 4~5단계 정도의 티어로 구분해서 어떤 컴파일러를 사용할지 결정한다.

- tier 0 Interpreter(C1): 단순 바이트 코드 해석
- tier 1 without profiling(C1): 빠르게 컴파일 (간단 최적화)
- tier 2 with profiling(C1): 빠르게 컴파일 + 실행 정보 수집
- tier 3 with full profiling(C1): 더 많은 프로파일링 정보 수집
- tier 4 with profiling(C2): C2가 전체 최적화를 적용한 고성능 네이티브 코드 생성

여기서 c1 -> c2로 올라가는 등업 정보는 invocation counter 값과 back edge count와 같은 즉 루프 실행 횟수를 기록해 판단한다.

```
- 기본 임계치 (threshold):
  - 호출 횟수: 10,000회 (client), 10,000~15,000회 (server)
  - 루프 횟수: 15,000회 이상

- JVM 옵션으로 변경 가능:
  - -XX:CompileThreshold=10000
```

즉 같은 메서드를 수천 수만번 호출하게 되면 jvm이 이거 중요하네 라고 판단해 등업을 시키는데, 이를 컴파일때 미리 호출을 여러번시켜 jit에게 인지시키는 jvm warm up이라는 기법도 있으니 찾아보면 좋을 것 같다.

```bash
java -XX:+UnlockDiagnosticVMOptions -XX:+PrintCompilation -XX:+TieredStopAtLevel=4 YourApp

---
  101   1       YourClass::yourMethod (5 bytes)
  230   3%  4   YourClass::yourMethod (5 bytes)
```

위와 같이 동작을 확인해볼 수 있다. 첫번째 숫자는 메서드id이며 그다음엔 티어레벨 (0~4) 그리고 퍼센티지기호는 프로파일링 기반 컴파일 여부를 나타낸다.

여기서 궁금증으로 프로파일링이 뭐지 싶을 수수 있는데 프로파일링 수집 여부는 최적화 수준의 핵심 차이점이다.

프로파일을 수집하게 되면 아래와 같은 지표를 수집하게 되는 것이다.

- hot method: 얼마나 자주 호출되는가
- hot loop: 얼마나 루프가 반복되는가
- branch prediction: 조건 분기의 확률들(실제 if문마다 동작하는 비율)
- method inlining 후보: 자주 호출되는 짧은 메서드인지 여부
- type profiling: 어떤 타입이 실제로 넘어오고 있는가 object는 항상 string인지? (jvm은 type erasure 방식으로 컴파일되기 때문에)

이런 지표들을 통해 jvm이 실행되지 않는 분기라고 판단되면 코드를 지워버리거나 타입을 미리 바꾼다거나 인라이닝 한다거나 죽은 코드를 지우거나 등의 고도화된 최적화 작업들을 진행한다. 그렇기 때문에 프로파일링 정보를 수집함에 따라 최적화되는 퀄리티가 달라진다는 것이다. (물론 컴파일 시간은 는다. 최적화 작업이 더 많아지기 때문에)

JIT은 최적화가 굉장히 잘되어있어 웬만한 네이티브 애플리케이션보다 뛰어난 성능을 낸다고 한다는 카더라들도 있다고 한다; 직접 재보진 않았다.

그러나 hotspotvm에 코드 유지보수성이 어려워 only java로 구현하는 graal이 나온다고들 한다 native image기반에 aot컴파일을 지원한다고 하는데, 나는 아직 사용해본적이 없으니 알아보면 좋을듯 싶다.