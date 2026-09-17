# launch, async

CoroutineDispatcher에 코루틴을 실행 요청하는 작업은 대표적으로 launch, async 이렇게 두 가지 메서드를 통해서 가능하다. 결과 반환이 없는 단순 작업에는 launch, 결과 반환이 필요한 작업에는 async를 사용한다.

<br>

#### 결과 반환을 하지 않는 launch

launch는 결과를 반환하지 않고 launch를 수행시 Job이 반환된다.

```kotlin
val job: Job = launch { println(1) }
```

#### 결과를 반환하는 async

async는 결과를 반환해 결과값은 Deferred로 감싸서 반환된다. Deferred는 미래에 올 수 있는 값을 담아놓는 객체다.

아래 예제에서는 async 블록의 마지막 줄에 있는 1이 반환되어야 하므로 Deferred\<Int> 값이 반환된다.

```kotlin
fun main() = runBlocking<Unit> {
    val deferredInt: Deferred<Int> = async {
        1 // 마지막 줄 반환
    }
    val value = deferredInt.await()
    println(value) // 1 출력
}
```

Deferred의 await() 메서드가 수행되면 await를 호출한 코루틴(위의 코드에서는 runBlocking 코루틴)은 결과가 반환되기까지 스레드를 양보하고 대기한다.

우리는 이를 코루틴이 일시 중단되었다고 한다. 이러한 특성으로 인해 await()은 일시 중단이 가능한 코루틴 내부에서만 사용이 가능하다. 만약 일반 함수에서 await을 사요하면 일시 중단 함수는 suspend fun에서만 호출될 수 있다는 오류가 나온다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbZx7qa%2FbtrczIO1qQO%2FGFJKxG5osH7PKPRO1Only0%2Fimg.png)

이후 결과가 반환되었을 때 코루틴은 다시 재개되고 코드가 실행된다.

<br>

### 서로 다른 디스패처에서 Coroutine 실행 요청

데이터베이스로부터 Array\<int>를 받아 정렬후 텍스트 뷰에 출력한다는 과정을 생각해보자. 이 과정에서 **파일 입출력 Dispatchers.IO, Array 정렬 Dispatchers.Default, 텍스트 뷰 출력 메인 스레드 디스패처**와 같이 여러 디스패처에 맞는 작업이 들어가는데 코루틴은 다양한 적입이 서로 다른 디스패처에 요청하기 위해 간편한 방법을 제공한다.

```kotlin
suspend fun updateUI() = coroutineScope { 
    // 1. 데이터베이스 입출력 작업을 해야 하므로 IO Dispatcher을 사용해 새로운 코루틴 실행
    val deferredInt: Deferred<Array<Int>> = async(Dispatchers.IO) { 
        delay(1000L) // 데이터베이스로부터 데이터 가져오는 시간
        arrayOf(3, 1, 2, 4, 5) // 마지막 줄 반환
    }
    
    val value = deferredInt.await()
    
    // 2. Sort해야 하므로 CPU작업을 많이 해야하는 Default Dispatcher을 사용해 새로운 코루틴 실행
    val sortedDeferred = async(Dispatchers.Default) { 
        value.sortedBy { it }
    }

    val sortedArray = sortedDeferred.await() 

    // 3. UI를 업데이트 하는 코루틴은 Main Dispatcher에 실행 요청한다.
    val updateUIJob = launch(Dispatchers.Main) {  
        setTextView(sortedArray)
    }
}
```

이렇게 서로다른 작업들을 디스패처 설정을 통해 서로 다른 디스패처에 코루틴을 실행을 요청할 수 있다.

[[CoroutineDispatcher]]