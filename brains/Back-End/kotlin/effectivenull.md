# lateinit과 Delegates.notNull<>()로 적절한 null 처리하기

Kotlin에서 nullablility는 최대한 피하는 것이 좋다.

다음과 같이 nullable 변수를 처리해 null이 아닐때 처리할 수 있는 방법이 있다.

```kt
printer?.print()
if(printer != null) printer.print() // smart cast
printer!!.print()
```

1번과 2번은 null이 아닐때만 동작하도록 설정되어 있으니 3번 케이스(not null assertions)는 NPE가 발생할 수 있는 위험이 있다.

nullable 변수를 적절하게 처리할 수 있지만, 의미없는 nullability 가능성 자체를 없애는 것이 가장 이상적이라고 볼 수 있다.

다음과 같은 방법으로 의미없는 nullability를 방지할 수 있다. (물론 nullable이어야 하는 케이스랑 아닌 케이스를 잘 구별해야한다.)

<br>

## lateinit

lateinit을 사용해서 초기화를 늦추는 것이다.

```kt
private var printer: Printer? = null

private lateinit var printer
```

예를들면 Mock 테스트에서 @BeforeEach와 같은 코드에서 동작을 할 때 null로 초기화 하는 것 보단 곧 초기화 될 것이니 지연하겠다. 라고 명시해주는 것만으로도 크나큰 차이점, 안정성을 얻을 수 있다.

일단 nullable 타입으로 선언한다면 BeforeEach가 동작하고나서도(printer는 null이 아님) !!를 통해서 타입을 캐스팅 해줘야한다. 불편하다. 그렇지만 lateinit을 사용하면 그런 귀찮은 작업들을 안해줘도 된다.

물론 lateinit을 사용하면 추가 비용이 발생할 수 있다. 그렇기에 바로 초기화 이후 사용되도록, 사용하기 이전에 반드시 초기화 되어야하는 상황에서만 사용하도록 하자.

lateinit을 사용하면 다음과 같은 이점이 있다.

- 반복되는 !! 언팩(unpack)을 하지 않아도 된다.
- 어떤 의미로 null로 나타내고 싶을때는 nullable 타입으로 만들수도 있습니다.
- 초기화 되기 이전에 상태로 돌아가지 않습니다.


<br>

## Delegates.notNull<>()

lateinit으로 초기화가 안되는 경우가 있는데 바로 Int, Double, Boolean과 같이 JVM 기본 타입과 연결된 타입의 프로퍼티를 초기화 하는 경우입니다.

이러할때는 Delegates.notNull<>()과 같이 Delegate로 위임해줄 수 있습니다.

```kt
private var isOk: Boolean by Delegates.notNull()
```

물론 lateinit 보다는 약간 느리긴합니다.



이와같이 프로퍼티 위임을 사용하면 nullability로 발생하는 다양한 문제들을 처리할 수 있습니다.

