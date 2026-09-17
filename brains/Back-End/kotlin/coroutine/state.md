# Coroutine Job 상태 관리

### Job의 상태

Job의 상태는 **생성, 실행중, 실행 완료, 취소 중, 취소 완료로 총 5가지다.**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlDPEY%2FbtrcOkr0Ne2%2FphWZLmmhFhkehvIzIEb8G1%2Fimg.png)

- `New`: Job이 생성된다.
- `Active`: Job이 실행 중이다.
- `Completed`: Job이 실행이 완료되었다.
- `Cancelling`: Job이 취소되는 중이다. Job이 취소되면 리소스 반환 등의 작업을 해야하기 때문에 취소 중 상태가 있다.
- `Cancelled`: Job의 취소가 완료되었다.

Job은 일반적으로 작업이 끝나면 실행 완료 상태가 된 다음 종료된다.


**실행 완료가 되지 않는다면?**

Job은 항상 실행에 성공하지는 않는다. 다양한 변수들로 인해 중간에 취소될 때가 있다. 예를 들어서, 네트워크를 통해 유저 정보를 달라는 요청을 했을 때, 우리는 요청 결과를 기다린다. 아래 그림처럼 서버에서 요청이 성공되거나 거부되었다는 메시지를 보내주면 Job은 성공하여 종료될 것이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fv4fNA%2FbtrcLBB0H2G%2F10LNc5co46tiTLwI3XUNC1%2Fimg.png)

하지만 만약 서버에서 결과를 주지 않늗나면 계속해서 클라이언트는 응답을 기다리고 있게 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fdk2bEE%2FbtrcNJyWaPq%2FxysFUCL5DfftkZ6NG3kSt0%2Fimg.png)

**이러한 상황에서는 일정 시간 이후 Job을 취소하는 작업이 필요하다. 또한 Job을 취소했을 때 생기는 Exception에 대한 핸들링도 필요하다.**

이제 Job을 취소하는 방법과 Exception Handling 방법에 대해서 알아보자

#### cancel()

간단하게 cancel()을 호출하여 job을 취소할 수 있다.

```kotlin
fun main() = runBlocking<Unit> {
    val job = launch(Dispatchers.IO) {
        delay(1000)
    }

    job.cancel()
}
```

하지만 그냥 취소를 하면 원인을 파악하기 힘드니 취소가 된 원인을 인자로 넣어보자/

**cancel()에는 두가지 인자 message: String, cause: Throwable**을 넘기는 것으로 취소의 원인을 알릴 수 있다.

또한 Job에 getCancellationException()을 이용함으로써 취소의 원인도 파악이 가능하다.

```kotlin
fun main() = runBlocking<Unit> {
    val job = launch(Dispatchers.IO) {
        delay(1000)
    }

    job.cancel("Job Cancelled by User", InterruptedException("Cancelled Forcibly"))
    
    println(job.getCancellationException()) // cancel 원인 출력
}
```

위의 코드 출력은 아래와 같다. cancel시 넘겨지는 Exception의 종류는 CancellationException으로 고정된다. 위에서는 InterruptedException을 넘기는데 출력되는 것은 CancellationException이며, cancel시 넘긴 Throwable은 반영되지 않는다.

```
java.util.concurrent.CancellationException: Job Cancelled by User

Process finished with exit code 0
```

<br>

#### cancel된 원인 출력하기

원인을 출력하는 방법은 간단하다. Job이 취소가 완료 될 때 invokeOnCompletion 내의 메서드가 호출이 되는데 이를 이용해 취소된 원인을 출력할 수 있다.

```kotlin
fun main() = runBlocking<Unit> {
  val job = launch(Dispatchers.IO) {
    delay(1000)
  }

  //취소된 원인 출력
  job.invokeOnCompletion { throwable ->
    println(throwable)
  }

  job.cancel("Job Cancelled by User", InterruptedException("Cancelled Forcibly"))
}
```

```
java.util.concurrent.CancellationException: Job Cancelled by User

Process finished with exit code 0
```

위 코드에서 `job.invokeOnCompletion()` 에서 throwable을 받고 해당 throwable을 출력할 수 있다. throwable은 InterruptedException을 넘기더라도 CancellationException으로 잡힌다.

**그런데 문제는 invokeOnCompletion은 Job이 취소완료 되었을 때 뿐만 아니라, 실행 완료 되었을때도 실행된다.**

```kotlin
fun main() = runBlocking<Unit> {
  val job = launch(Dispatchers.IO) {
    delay(1000)
  }

  //취소된 원인 출력
  job.invokeOnCompletion { throwable ->
    println(throwable)
  }
}
```

```
null

Process finished with exit code 0
```

취소 없이 실행이 완료되자 throwable은 null이 된다. 따라서 이 부분은 다음과 같이 handling 해줘야한다.

```kotlin
fun main() = runBlocking<Unit> {
  val job = launch(Dispatchers.IO) {
    delay(1000)
  }
  
  job.invokeOnCompletion { throwable ->
    when(throwable){
      is CancellationException -> println("Cancelled")
      null -> println("Completed with no error")
    }
  }
}
```

그럼 다음과 같은 결과를 확인할 수 있다.

```
Completed with no error

Process finished with exit code 0
```

Job이 실행이 완료되었을 때랑 취소 완료되었을 때 모두 `invokeOnCompletion`내의 메서드가 호출되는 이유는 Job의 상태 변수와 관계가 있다.