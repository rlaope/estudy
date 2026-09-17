# JVM Warm UP 적용하기

1. **JVM은 자주 실행되는 코드를 컴파일하고 캐시한다.** 
2. **클래스는 필요할 때 Lazy Loading으로 메모리에 적재된다.**

이 두가지가 JVM Warm UP의 핵심 아이디어다.

우리는 운동을 하기 직전에 최대한의 능률을 끌어올리기 위해 체조나 가벼운 달리기를 하는 등의 워밍업을 하고 JVM도 마찬가지로 최대의 성능을 끌어올리기 위해 워밍업이 필요하다. 이를 `JVM Warm UP`이라고 한다.

자바 애플리케이션의 배포 직후 레이턴시의 원인은 클래스가 메모리에 적재되지 않음과 코드가 최적화된 기계어로 컴파일되지 않았기 때문이라는 두 가지 원인을 알 수 있다.

해결방법은 간단하다. 클래스를 메모리에 미리 적재하고 코드를 최적화하여 컴파일하면 된다. 우리가 직접? 아니다 **애플리케이션이 기동되는 시점에, 자주 호출될 것으로 예상되는 지점의 코드를 충분히 많이 실행시키면 된다.** 마치 기계를 예열해두는 것 처럼

스프링에서 예시를 보겠다 아래 코드처럼 `ApplicationRunner`를 이용해 스프링 애플리케이션을 기동할 때 특정 코드를 실행할 수 있도록 작성할 수 있다. 아래 코드는 자주 사용될 거으로 예상되는 `findDetailCategoryById` 메서드를 호출함으로써 웜업하는 예시다. **(클래스로더 웜업)**

```java
@Component
public class WarmupRunner implements ApplicationRunner {

    private final CategoryController categoryController;

    public WarmupRunner(final CategoryController categoryController) {
        this.categoryController = categoryController;
    }

    @Override
    public void run(final ApplicationArguments args) throws Exception {
        try {
            categoryController.findDetailCategoryById(1L);
        } catch (Exception e) {
            // do nothing
        }
    }
}
```

```bash
xx... INFO 17860 --- [nio-8080-exec-2] c.a.d.c.presentation.CategoryController: before 소요시간 25ms
```

```bash
xx... INFO 17860 --- [nio-8080-exec-2] c.a.d.c.presentation.CategoryController: after 소요시간 2ms
```

before after를 비교해보면 차이가 약 12배로 극명하다. 

JIT Compiler Warm up도 이어서 해보겠다 하는 방법은 가능하다 핫스팟으로 지정될 메서드들을 사전에 동일하게 반복 실행해서 JIT Compiler가 최적화하기를 유도한다.  **C1 컴파일러의 기본 Threshold는 1,500회이고 C2 컴파일러의 기본 Threshold는 10,000회**이다. 이를 참고해 적절한 횟수로 웜업하면 된다.

. 각 Threshold를 만족할만큼 웜업하기에 시간이 오래걸릴수도 있으므로, 웜업과 기동시간의 트레이드 오프를 잘 생각하여 웜업을 진행해주면 된다.

 