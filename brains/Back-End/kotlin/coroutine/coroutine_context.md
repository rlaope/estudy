# CoroutineContext

CoroutineDispatcher 편에서 우리는 Dispatcher, CoroutineExceptionHandler에 대해서 배웠다.

- Dispatcher: 코루틴이 실행될 스레드풀을 잡고 있는 관리자
- CoroutineExceptionHandler: 코루틴에서 Exception이 발생했을 때 처리기

이 두가지 요소는 신기하게도 **CoroutineContext**가 들어갈 자리에 그대로 들어갈 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FFmduT%2FbtrcSswLPkL%2F7MPfksDRG1B2KjKnpkpqjk%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FN7ODh%2FbtrcUjGhQQ9%2F6QZWBnBQGLgCkYUnb6J0AK%2Fimg.png)

**이것이 가능한 이유는 Dipatcher, CoroutineExceptionHandler가 모두 CoroutineContext를 확장하는 인터페이스의 구현체이기 때문이다.**

<br>

### CoroutineContext

**CoroutineContext는 Coroutine이 실행되는 환경이라고 생각하면 된다.** 위에 나온 Dispatcher, CoroutineExceptionHandler 또한 Coroutine이 실행되는 환경의 일부이며, 이 둘 모두는 CoroutineContext에 포함되어 Coroutine이 실행되는 환경으로 설정될 수 있다.

#### CoroutineContext와 합치기

Dispatcher, CoroutineExceptionHandler을 결합해 하나의 Context로 만들어보자.

여기서 우리는 CoroutineContext 상의 operator fun plus를 사용한다.

```kotlin
public interface CoroutineContext {
    public operator fun plus(context: CoroutineContext): CoroutineContext 
     ..
}
```

```kotlin
val exceptionHandler = CoroutineExceptionHandler { coroutineContext, throwable ->  }

val coroutineContext = Dispatchers.IO + exceptionHandler
```

이 코드를 해석하면 하나의 CoroutineContext에서 `Dispatcher.IO`, `CoroutineExceptionHandler`가 들어가 Context는 IO Thread에서 실행되는 Exception을 핸들링할 수 있게 되었다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbuDmcO%2FbtrcMGQ401A%2FyFlyu5qLr331sqoyjf4tHk%2Fimg.png)

이렇게 만들어진 CoroutineContext는 CoroutineContext가 들어가야할 자리에 넣음으로써 사용할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FpAjS0%2FbtrcNJzfJdG%2FWPGTQhLwFe8VYFqvJmqB5K%2Fimg.png)

<br>

#### CoroutineContext에 접근하기

CoroutineContext는 CoroutineContext의 집합이라는 것을 위에서 알아보았다. 이번에는 이러한 집합에서 특정한 CoroutineContext에 접근하는 방식을 알아볼 것이다.

위에 그림에서는 간단하게 표현하기 위한 요약 그림이였지만, 좀 더 상세히 표현하면 아래와 같다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fsa3cv%2FbtrcMGctGUx%2FXoQ9RLAJFkdzevsXbtKpuK%2Fimg.png)

CoroutineContext를 구성하는 두 개의 CoroutineContext인 Dispatcher와 CoroutineExceptionHandler가 존재하고, Dispatcher의 key값은 "keyA" CoroutineExceptionHandler는 "keyB"라고 하자, 물론 키값이 문자열은 아니지만 이해하기 편하기 위해 이렇게 표현했다.

위에 그림에서 ExceptionHandler를 부모 CoroutineContext로부터 가져오고 싶다.

```kotlin
fun main() {
    val exceptionHandler = CoroutineExceptionHandler { coroutineContext, throwable -> }

    val coroutineContext = Dispatchers.IO + exceptionHandler // 부모 CoroutineContext = Dispatcher + ExceptionHandler

    val exceptionHandlerFromContext = coroutineContext[exceptionHandler.key] // Key를 통한 자식 CoroutineContext 접근

    if (exceptionHandler === exceptionHandlerFromContext) { // 같은 객체인지 확인하기 위해 동일성 비교
        println(true)
    }
}
```

위에 처럼 CoroutineContext에 key를 보내서 자식 CorutineContext의 요청이 가능하다.

key를 통해 context로부터 가져온 값을 이전에 결합한 CoroutineContext와 동일성 === 비교를 통해 비교하면 true가 출력되는 것을 볼 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbmgCZp%2FbtrcM1Hxi7w%2FDnYHHuIDEZiKYs71krajUK%2Fimg.png)

<br>

#### CoroutineContext에서 CoroutineContext 제거하기

위에서와 같이 CoroutineContext에 접근이 가능하면, 당연하게 제거도 가능하다. CoroutineContext의 제거는 `minusKey()` 메서드를 통해 가능하다.

```kotlin
fun main() {
    val exceptionHandler = CoroutineExceptionHandler { coroutineContext, throwable -> }

    val coroutineContext = Dispatchers.IO + exceptionHandler

    val minusContext = coroutineContext.minusKey(exceptionHandler.key)
}
```

이를 그림으로 표현하면 다음과 같다.

1. CoroutineExceptionHandler(CoroutineContext) 제거 요청

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdAftz8%2FbtrcOj1nnsd%2F07Q4eWEixskFmRFNOtAbQ1%2Fimg.png)

2. 제거 완료

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FZxykz%2FbtrcMGKo46X%2F4YkqPwG5CpMF7X45NaR8y1%2Fimg.png)

3. 자기 자신 반환

minusKey를 호출하면 제거된 CoroutineContext가 반환된다.

```kotlin
public fun minusKey(key: Key<*>): CoroutineContext
```

```kotlin
val minusContext = coroutineContext.minusKey(exceptionHandler.key)
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbUp4ka%2FbtrcOfEY90Q%2FaeUWRldIj5IBuHbv67sBsK%2Fimg.png)

