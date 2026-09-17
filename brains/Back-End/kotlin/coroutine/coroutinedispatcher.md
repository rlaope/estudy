# CoroutineDispatcher

CoroutineDispatcher에서 Coroutine은 코루틴이며, Dipatch는 한국어로 '보내다'라는 뜻이다. 즉, CoroutineDispatcher은 코루틴을 보내는 객체를 뜻한다. 그러면, 어디로 코루틴을 보낼까? 바로 스레드로 보낸다.

모든 작업은 스레드 위에서 실행되어야하고, 코루틴 또한 작업이므로 스레드 위에서만 실행될 수 있다. 따라서 만들어진 코루틴을 스레드로 보내는 역할을 하는 객체가 필요한데 이 역할을 바로 CoroutineDispatcher가 한다.

**우리가 코루틴을 만들어서 CoroutineDispatcher로 코루틴의 실행을 요청하면 CoroutineDispatcher는 자신이 사용할 수 있는 스레드풀의 스레드중 하나에 코루틴을 보낸다.**

이 때 CoroutineDispatcher는 스레드 풀 내의 스레드 부하 상황에 맞춰 코루틴을 분배한다. 이를 시각적으로 표현하면 다음과 같다.

1. 유저가 코루틴을 생성한 후 CoroutineDispatcher 전송
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc7YLRL%2FbtrcxH9VZ9d%2FZYwUOjv3Oa2lvWcqtEETJK%2Fimg.png)

2. CoroutineDispatcher은 자신이 잡고 있는 스레드풀에서 사용할 수 있는 스레드가 어떤 스레드인지 확인한 후, 해당 스레드에 코루틴을 보낸다.
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FJLlcs%2Fbtrcs72rcMO%2FRL7ETLPINBfE5E59Fj5lv1%2Fimg.png)

3. 분배 받은 스레드는 해당 코루틴을 수행한다.
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FIUBcH%2FbtrcxKeyme5%2FCjji0o4L37PlqL4AgCMDU0%2Fimg.png)

이는 Executor 프레임워크에서 ExecutorService가 하는 역할과 매우 유사하다.

<br>

### CoroutineDispatcher 만들기

CoroutineDispatcher 중에서 사용할 수 있는 스레드가 제한되지 않은 CoroutineDispatcher도 있지만, 대부분의 CorotuineDispatcher는 사용할 수 있는 스레드가 제한되어있다. 여기서 사용할 수 있는 스레드가 제한된 CorotuineDispatcher 객체를 만들어보자.

#### 멀티스레드 사용할 수 있는 CoroutineDispatcher

코루틴에서 스레드풀을 만들기는 쉽다. 다음과 같이 단순한 코드로 스레드가 3개인 Dispatcher를 생성할 수 있다.

```kotlin
val dispatcher = newFixedThreadPoolContext(3, "ThreadPool")
```

<br>

#### 단일 스레드를 사용할 수 있는 CoroutineDispatcher

```kotlin
val dispatcher = newSingleThreadContext("SingleThread")
```


Dispatcher는 코루틴을 스레드에 보내는 역할을 한다. 디스패처의 작업 대기열에 코루틴이 실행 요청되면 해당 코루틴 디스패처에서 사용할 수 있는 스레드가 있을 때, 코루틴은 해당 스레도로 보내 실행시킨다.

**즉, 우리가 CoroutineDispatcher에 코루틴을 보내기만하면, CorotuineDispatcher는 자신이 사용할 수 있는 스레드가 있을 때 코루틴을 스레드로 보내 실행시킨다.**

<br>

### CoroutineDispatcher Basic

`coroutine-core` 라이브러리나 `coroutine-android` 라이브러리를 설정하면 미리 설정된 디스패처를 사용할 수 있어 newFixedThreadPoolContext, newSingleThreadContext를 사용해 디스패처로 별도로 생성하거나 정의할 필요가 없다. 기본으로 생성되어있는 디스패처는 다음과 같다.

- **Dispatchers.Main**: UI와 상호작용하는 작업을 실행하기 위해 사용되는 디스패처다. `coroutine-android` 라이브러리에 대한 의존성이 있어야한다.
- **Dispatcher.IO**: **Disk or Network I/O 작업을 최적화되어 있는 디스패처**, `coroutine-core` 라이브러리에 대한 의존성으로 사용할 수 있다.
- **Dispatcher.Default**: **CPU를 많이 사용하는 작업(CPU Bound)을 실행하기 위한 디스패처**, 정렬 작업이나 JSON Parsing 작업등을 위해 사용된다.

미리 정의된 디스패처는 다음과같이 launch와 함께 사용할 수 있다.

```kotlin
launch(Dispatchers.Main) {
	updateButton() // 필요한 Job 수행
}
```

