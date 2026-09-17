# Coroutine Job을 Lazy하게 만들기

코루틴 빌더인 launch를 사용하면 Job이 생성되는 것을 볼 수 있다.

**Job은 결과가 없는 비동기 작업으로 예외가 발생하지 않는 이상 끝까지 수행된다.**

이번 글에서는 Job의 비동기 작업을 실행하는 시점과, 실행 방법을 조절하는 것에 대해 더욱 깊숙히 다루어볼 것이다.

### Job의 생성

**코루틴 빌더인 launch 메서드를 별도의 옵션 없이 사용하면 생성된 비동기 작업 Job은 생성 후에 바로 실행된다.**

```kotlin
fun main() = runBlocking<Unit> {
	val job = launch {
		println(1)
	}
}
// 1
```

위의 코드처럼 Job을 생성할 경우 이 Job은 생성과 동시에 실행된다. 이러한 방식으로 Job을 생성하면 필요한 위치에 바로 생성해서 실행시켜야하기 때문에 코루틴 실행의 유연성이 떨어질 수 있다.

**이를 해결하기 위해 Job을 생성한 후 필요할 때 코루틴이 스레드에 보내져 실행되도록 만드는 옵션이 있다.**

<br>

### Job을 Lazy하게 실행하기

Job을 생성한 후 바로 실행되는 것을 막기 위해서는 아래와 같이 Job을 생성하는 launch 메서드에 `CoroutineStart.LAZY` 인자를 넘겨야한다.

```kotlin
fun main() = runBlocking<Unit> {
  val job = launch(start = CoroutineStart.LAZY) {
    println(1)
  }
}
// 아무것도 출력되지 않는다.
```

위와 같이 Job을 생성하면 해당 Job은 실행되지 않고 대기 상태(생성 상태)가 된다. 우리는 이것을 Job이 지연 실행된다고 한다.

이제 이렇게 생성한 Job들을 실행하는 방법에 대해서 알아보도록 하자

<br>

#### start() or join()

Lazy하게 생성된 Job은 start()를 통해 실행이 가능하다. 

start() or join() 를 호출하면 생성된 코루틴을 즉시 실행시킨다.

```kotlin
fun main() = runBlocking<Unit> {
  val job = launch(start = CoroutineStart.LAZY) {
    println("가나다")
  }

  job.start() // or job.join()
}
// 가나다
```

