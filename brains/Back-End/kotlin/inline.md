# infix, inline function

### infix

infix 키워드는 간단하게 함수를 가독성 있게 표현할 수 있는 키워드다.

객체와 객체 사이에 사용하는 함수 표현이라고 생각하면 된다.

```kotlin
infix fun String.add(other: String): String {
	return this + other
}

fun main() {
	val price = "3000"
	println(price add "원") // 3000 언
}

```

주로 확장 함수에 적용되는 것 같고 사용법도 쉽다. 근데 별로 안쓸것같기도..


### inline

inline을 알기 전에 일급 함수에 대해서 알아보겠다. 일급 함수는 간단하게 말하면 함수를 매개변수로 하는 함수라고 생각하면 된다 (고차함수, 람다, 익명함수)

코틀린은 이러한 일급함수 때문에 함수형 프로그래밍을 지원한다고 볼 수 있다.

```kotlin
fun doSomething(body : () -> Unit) {
    body()
}

fun callFunction() {
    doSomething { println("문자열 출력!") }
}
```

이렇게 작성했을 때 자바로 변환한다면 다음과 같아진다.

```java
public void callFunction() {
    doSomething( new Function() {
        @Override
        public void invoke() {
            System.out.println("문자열 출력!");
        }
    });
}
```

이런식으로 Function Object가 생성되는 것을 볼 수 있다. 이러한 현상을 막을 수 있는 것이 inline function이다.

```kotlin
inline fun doSomething(body : () -> Unit) {
    body()
}

fun callFunction() {
    doSomething { println("문자열 출력!") }
}
```

```java
public void callFunction() {
    System.out.println("문자열 출력!");
}
```

위와같이 단순하게 inline 키워드만 붙여주면 우리가 원하는 방향으로 자바 코드로 변환이 되는 것을 확인할 수 있다.

inline을 활용한 예제를 살펴보자면 대표적으로 Scope Functions가 있다. 대표적으로 also를 활용한 예제를 보자면

```kotlin
@kotlin.internal.InlineOnly
@SinceKotlin("1.1")
public inline fun <T> T.also(block: (T) -> Unit): T {
    contract {
        callsInPlace(block, InvocationKind.EXACTLY_ONCE)
    }
    block(this)
    return this
}
```

```kotlin
fun main() {
    val prices = listOf(3000, 5000, 6000)
    prices.also {
        println("Total Price : ${it.sum()}")
    }
}

- 결과 -
Total Price : 14000
```

also 함수는 inline을 활용하는 것을 볼 수 있다.

그러나 inline을 꼭 사용해야 효율이 좋다! 라는 생각이 들 수 있지만 물론 맞지만 성능에 영향은 미미하다. 왜냐면 JVM에서 이미 inline을 해주고 있다고 한다.

언제 써야하냐면 바로 **고차 함수(High-Order Functions)** 를 작성할 때 사용하는 것이 효과적이다. (filter, map, fold ....)

고차함수를 작성하고 inline을 쓰지 않는다면 java 코드에서 익명객체를 생성하는 비용이 발생하는 것을 확인할 수 있기에 inline을 적용시키면 객체를 생성하지 않고 우리가 원하는 방향으로 변환되는 것을 확인할 수 있다.