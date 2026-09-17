# Coroutine

코틀린에서 Non Blocking 코드를 작성하기 위하여 코루틴이라는 것을 사용하기도 한다. 오늘 이에 대해서 알아볼건데 먼저 스레드 구조와 다중 스레드 작업의 필요성에 대해서 알아보자.

### Thread 구조와 다중 Thread 작업의 필요성

하나의 process는 여러 thread를 가지고 있고, 각 스레드는 독립적으로 작업을 수행한다.

예를 들어서 jvm의 프로세스 상에서는 다음과 같이 구성되어있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FGxqBD%2FbtrctuPsMxG%2Fta9scRvF6HCYDnQHqA2MWK%2Fimg.png)

위의 그림에서 메인 스레드를 보자. **JVM 프로세스는 메인 스레드가 메인 함수를 실행하며 시작되며, 만약 프로스세 내에 사용자 스레드가 메인 스레드 밖에 없는 상황이라면 메인 스레드가 종료되면 프로세스 또한 강제종료 된다.**

이때 메인 스레드는 한 번에 하나의 작업을 실행할 수 있다. 그런데 그림에서는 메인 스레드 말고 마찬가지로 2개의 스레드가 보인다. 이 스레드들은 사용자에 의해 생성되는 스레드로 메인 스레드와 마찬가지로 작업을 수행할 수 있다.

> 일반적으로 데몬 스레드로 생성되지만, 종종 메인 스레드와 같이 사용자 스레드로 되기도 한다. JVM 프로세스는 사용자 스레드가 모두 종료될 때 종료된다.

메인 스레드에서 많은 부하를 받는 작업은 당연히도 지양해야하며, 다른 스레드를 생성해 해당 스레드에 높은 부하를 주는 작업을 수행하도록 해야한다.

#### 한계

그러나 jvm 언어로 작성된 애플리케이션에서는 메인 스레드가 블로킹 되는 문제를 해결하기 위해서 많은 시도되었다. 가장 대표적인 방법으로는 다음 방식들이 있다.

#### Thread 클래스 상속

Thread 클래스를 상속하는 새로운 클래스를 만들고 run 메서드를 오버라이드해, 새로운 스레드에서 실행될 작업을 정의할 수 있다. 새로운 클래스의 인스턴스를 만들어 start 함수를 호출하면, 새로운 스레드에서 작업이 실행된다.

```kotlin
fun main() {
    val exampleThread = ExampleThread()

    exampleThread.start()
}

class ExampleThread : Thread() {
  override fun run() {
    println("[${Thread.currentThread().name}] New Thread Running")
  }
}

/*
출력
[Thread-0] New Thread Running
*/
```

**하지만 이 방법으로 생성한 스레드 인스턴스는 많은 메모리를 차지하면서, 재사용성이 어렵다는 단점이 있다. 또한 스레드를 개발자가 직접 생성하고 관리해야하기 때문에 메모리 누수의 가능성이 올라간다.**

이런 문제를 해결하기 위해서 한 번 생성한 스레드의 재사용성이 용이해야하며 생성된 스레드의 관리자가 개발자가 아닌 미리 구축한 시스템이 할 수 있어야한다. 이러한 역할을 하기 위해 Executor 프레임워크가 등장한다.

```kotlin
fun main() {
  // ExecutorService 생성
  val executorService: ExecutorService = Executors.newFixedThreadPool(4)

  // 작업 제출
  executorService.submit {
    println("[${Thread.currentThread().name}] 새로운 작업1 시작")
  }

  // 작업 제출
  executorService.submit {
    println("[${Thread.currentThread().name}] 새로운 작업2 시작")
  }

  // ExecutorService 종료
  executorService.shutdown()
}

/*
출력
[pool-1-thread-1] 새로운 작업1 시작
[pool-1-thread-2] 새로운 작업2 시작
*/

```

Executor 프레임워크는 개발자의 스레드 관리에 대한 책임을 낮추고 생성된 스레드 인스턴스 재사용성을 높혔다.

사용자의 요청에 따라 스레드 집합인 '스레드 풀'을 생성하고, 사용자가 작업을 제출하면 이 스레드풀의 스레드 중 하나에 작업을 할당한다.


#### Rx Library

Rx 라이브러리는 엄밀히 리액티브 프로그래밍을 돕기 위한 라이브러리인데, 데이터 스트림을 정의하고 데이터 스트림을 구독해 처리할 수 있게 해주는 라이브러리다.

라이브러리 내부에는 subscribeOn, obeserveOn 메서드 등을 통해서 데이터를 발행하는 스레드를 데이터를 구독하는 스레드를 분리할 수 있었지만, 간단한 작업들도 모두 데이터 스트림으로 만들어야하는 불편함이 존재했다.

```kotlin
publisher.subscribeOn(Schedulers.io())
         .observeOn(AndroidSchedulers.mainThread())​

```

이러한 접근 방식들에는 한계가 존재한다. 바로 **작업의 단위가 스레드**라는 점이다. 메인 스레드의 블로킹을 방지하기 위해 다른 스레드로 작업을 넘기면 된다고 했는데, 작업의 단위가 스레드라는 말이 이해가 안갈 수 있다. 더 알아보자.

스레드는 생성 비용이 비싸고 작업을 전환하는 스위칭 비용이 비싸다. 또한 한 스레드가 다른 스레드로부터의 작업을 기다려야하면, 기본적으로 다른 스레드의 작업을 기다리는 스레드는 다른 작업이 스레드를 사용할 수 없도록 **블로킹**된다. 이렇게 되면 해당 스레드는 하는 작업 없이 다른 작업이 끝마칠때까지 기다려야하기 때문에 자원은 낭비된다. 이것이 위의 작업 단위가 스레드일 경우 생기는 고질적인 문제점이다.

```kotlin
fun main() {
  // ExecutorService 생성
  val executorService: ExecutorService = Executors.newFixedThreadPool(4)

  // 작업2 제출
  val future : Future<String> = executorService.submit<String> {
    println("작업2 시작")
    Thread.sleep(2000L) // 작업 시간 2초
    println("작업2 완료")
    "작업2 결과"
  }

  // 작업1 제출
  executorService.submit {
    println("작업1 실행")
    val result = future.get() // 작업1을 중지하고 작업2가 완료되는 것을 기다림 스레드 블로킹
    println("${result}를 가지고 나머지 작업")
  }

  // ExecutorService 종료
  executorService.shutdown()
}

```

위의 코드는 다음과 같이 동작한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcX1U1Z%2FbtsFg2SuZB7%2FjUAkOwcxRThy0bPuBAZFt0%2Fimg.png)

위의 그림3을 보면 스레드 1에서 작업1 수행 도중 스레드 2의 작업 2의 결과물이 작업 1을 수행하는데 필요해졌다. 그때 작업 1을 실행하던 스레드1은 아무것도 하는일이 없어 블로킹되며 스레드2로부터 결과를 전달받아 재개하기 전까지는 많은 시간이 소요된다.

이렇게 짧은 시간동안만 블로킹되면 다행이지만, 실제 상황에서는 스레드의 성능을 반도 발휘하지 못하게 만드는 블로킹이 반복될 수 있다.

<br>

### 코루틴이 한계점을 극복하는 방법

코루틴은 작업 단위로, 스레드를 사용해 코루틴을 실행할 수 있다. 하지만 스레드 상에서 동작하는 코루틴은 언제든지 일시 중단이 가능하며, 이는 마치 스레드에 코루틴을 붙였다 땠다 할 수 있는 것과 같다. 이때문에 코루틴은 `경량 스레드`라고 불린다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdoTuZ1%2FbtsFh5H1maq%2Fmb5qvJL8nVHWcjAdvXEb8k%2Fimg.png)

경량 스레드에 대해서 더 알아보자. 아래 그림에서 코루틴을 이용해보자. 작업1, 2는 각각 코루틴1, 2로 바꾸며 코루틴 3이 추가로 실행 요청되는 상황을 가정한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fxz6lD%2FbtsFm7xyLGE%2FAJEcFdgfVFwFZlhuNqCKx0%2Fimg.png)

1. 코루틴1이 생성되어 스레드 1에 요청하고 코루틴2가 생성되어 스레드 2에 요청된다. 스레드1에서 코루틴1 실행 도중 나머지 연산에 코루틴2로부터 결과가 필요해진다. 하지만, 코루틴2의 작업이 끝나지 않아 코루틴1의 작업을 마저할 수 없다. 이때 코루틴1은 스레드1을 블로킹하는 대신 사용권한을 양보하고 다른 코루틴이 스레드 위에서 실행될 수 있게 한다.
2. 코루틴 3이 추가로 요청되면 코루틴3은 자유로워진 스레드1에서 실행된다.
3. 코루틴3의 실행을 마치면 스레드1에 사용권한을 반납한다.
4. 이후 스레드2에서 실행되면 코루틴2의 작업이 종료되고 결과를 반환한다. 그러면 코루틴1이 할당받은 작업이 없는 스레드1혹은 2를 사용해 실행된다.

**즉 정리하면, 코루틴은 스레드가 필요없을 때 사용권한을 양보한다.** 이를 통해 스레드를 블로킹하는 상황이 줄어 각 스레드를 최대한으로 활용할 수 있다. 스레드는 비용이 매우 큰 객체다. 코루틴은 스레드가 필요 없어지면, 스레드를 양보하는 방식으로 사용을 최적화한다.

정리하면 코루틴은 스레드안에서 실행되는 **일시 중단 가능한 작업 단위**다.

하나의 스레드에서 여러 코루틴이 서로 스레드를 양보해가며 실행될 수 있다.