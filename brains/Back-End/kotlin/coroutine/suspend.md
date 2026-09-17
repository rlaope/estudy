# suspend fun

suspend fun으로 선언되는 일시 중단 함수는 **함수 내에 일시 중단 지점을 포함할 수 있는 특별한 함수다.**

코루틴은 언제든지 일시 중단하고, 스레드를 양보할  수 있다는 것을 기억하자. 일시 중단 함수는 코루틴에서 실행되는 일시 중단 지점이 포함된 코드들을 재사용할 수 있는 코드의 집합으로 만드는 역할을 한다.

```kotlin
fun main() = runBlocking<Unit> {
    delay(100L)
    println("Hello Coroutines")
    delay(100L)
    println("Hello Coroutines")
}
```

이 코드에서 delay 함수와 println 함수가 반복된다. 따라서 이 코드는 다음과 같이 함수로 만들 수 있다.

```kotlin
fun delayAndPrintHelloCoroutines() {
    delay(100L)
    println("Hello Coroutines")
}
```

하지만, IDE에서 살펴보면 오류가 나는 것을 확인할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FPNvre%2FbtsFDDLL7Lm%2FyXnmSTWR7uHX2RRyawKAMk%2Fimg.png)

바로 일시 중단 함수인 delay는 일시 중단 함수(suspend fun)에서만 호출될 수 있다는 의미다. 왜 이런 오류가 나는 것일까? 바로 delay 함수는 코루틴이 일정 시간 동안 스레드를 양보하도록 만드는 함수여서 일시 중단 함수로 선언되었기 때문이다. 따라서 이 함수를 호출하는 곳도 일시 중단 함수여야한다.

단순히 fun에서 suspend fun으로 바꿔주면 문제는 해결된다.

```kotlin
suspend fun delayAndPrintHelloCoroutines() {
    delay(100L)
    println("Hello Coroutines")
}
```

<br>

### 일시 중단 함수는 코루틴이 아니다.

그렇다면, 다음 main 함수를 실행시키는데 시간이 얼마나 걸릴까?

```kotlin
fun main() = runBlocking<Unit> {
    delayAndPrintHelloCoroutines()
    delayAndPrintHelloCoroutines()
}

suspend fun delayAndPrintHelloCoroutines() {
    delay(100L)
    println("Hello Coroutines")
}
```

정답은 바로 200밀리초다. 일시 중단 함수가 호출된다고 해서 새로운 코루틴이 생성되지 않기 때문에 100밀리초를 대기하고 프린트를 출력하기 때문에 하나의 코루틴에서 실행되기 때문이다. 이를 확인하기 위해 다음 코드를 실행해보면 200밀리초보다 약간 더 시간이 걸리는 것을 확인할 수 있다.

```kotlin
fun main() = runBlocking<Unit> {
    val startTime = System.currentTimeMillis()
    delayAndPrintHelloCoroutines()
    delayAndPrintHelloCoroutines()
    println("${System.currentTimeMillis() - startTime}") // 211 출력
}
```

만약 일시 중단 함수가 새로운 코루틴에서 실행하게 하려면 launch, async 와 같은 코루틴 빌더로 감싸야한다.

```kotlin
fun main() = runBlocking<Unit> {
    val startTime = System.currentTimeMillis()
    val job1 = launch {
        delayAndPrintHelloCoroutines()
    }
    val job2 = launch {
        delayAndPrintHelloCoroutines()
    }
    job1.join()
    job2.join()
    println("${System.currentTimeMillis() - startTime}") // 110 출력
}
```

그러면 각 일시 중단 함수가 병렬로 실행되어 실행 시간이 110 밀리초의 시간이 걸리는 것을 확인할 수 있다.

<br>

### 일시 중단 함수 내부에서 일시 중단 함수 호출

```kotlin
fun main() = runBlocking<Unit> {
    val startTime = System.currentTimeMillis()
    delayAndPrintHelloCoroutines()
    delayAndPrintHelloCoroutines()
}
```

이 코드에서 delayAndPrintHelloCoroutines 메서드는 fun으로 선언되었다면 오류가 난다. 이유는 delay가 일시 중단 함수였기 때문이다. 여기에서 볼 수 있듯 일시 중단 함수는 일시 중단 함수에서만 호출이 가능하다. suspend fun에서 호출되는 함수는 모두 suspend fun이여야 된다는 뜻이다.


#### 코루틴에서 일시 중단 함수

**그렇다면, 가장 상위의 일시 중단 함수는 어디에서 호출될까?** 바로 코루틴이다. 일반 함수는 한 번 실행되면 끝까지 실행될 뿐 일시 중단 이라는 기능 자체가 없다. 일시 중단 기능은 코루틴에만 있는 특별한 기능이기 때문에 코루틴에서 호출되어야 한다. 위의 코드에서는 runBlocking 코루틴 내부에서 일시 중단 함수가 호출되고 있다.
