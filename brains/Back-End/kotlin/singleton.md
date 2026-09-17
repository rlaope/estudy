# Kotlin에서 싱글톤 클래스를 어떻게 생성할까요?

`object` 키워드를 사용하세요 그게 전부입니다!

```kt
object ClassName {
}
```

이제 이 코드가 의미하는 바에 대한 요지를 빠르게 알려드리겠습니다.

가볍게 생각하지 마세요, 싱글톤 패턴은 사용 언어에 관계없이 매우 중요한 소프트웨어 디자인 패턴이기 때문입니다.

우슨 그 의미를 이해해야 합니다.

싱글톤 클래스는 말 그대로 Single all the time을 의미합니다. 여러번 인스턴스화되는 것으로 제한되며 싱글톤 클래스의 인스턴스가 생성되면 애플리케이션/앱 전체에서 이 클래스의 유일한 인스턴스로 영원히 유지됩니다.

자바에서는 우리는 싱글톤 클래스를 생성하기 위한 몇가지 단계를 따릅니다.

1. private constructor
2. 정적인 메서드 getInstance()
   1. if 클래스가 존재하는 경우 -> 하나 생성
   2. else -> 기존 인스턴스 반환
3. 스레드 안정성을 보장하기 위해 getInstance() 메서드 동기화

이 과정은 많은 보일러플레이트 코드를 유발합니다. Kotlin에서는 모든 단계를 단일로 좁혀 개발자의 작업을 쉽게 만듭니다.

이제 아까 위의 코드를 백스테이지에서 무슨 일이 일어나는지 이해해보려면 코드를 디 컴파일 해볼까요?

```java
public final class ClassName {
    @NotNull
    public static final ClassName INSTANCE;
    private ClassName() {

    }

    static {
        ClassName var0 = new ClassName()
        INSTANCE = var0
    }
}
```