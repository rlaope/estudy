# Coroutine Job State Management

### Job States

A Job has a total of 5 states: New, Active, Completed, Cancelling, and Cancelled.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlDPEY%2FbtrcOkr0Ne2%2FphWZLmmhFhkehvIzIEb8G%2Fimg.png)

- `New`: The Job is created.
- `Active`: The Job is running.
- `Completed`: The Job has completed execution.
- `Cancelling`: The Job is in the process of being cancelled. This "Cancelling" state exists because when a Job is cancelled, it needs to perform tasks such as releasing resources.
- `Cancelled`: The Job's cancellation is complete.

Typically, a Job enters the "Completed" state and then terminates once its work is done.

**What if it doesn't complete?**

A Job doesn't always succeed in its execution. It can sometimes be cancelled midway due to various factors. For example, when we make a request for user information over a network, we wait for the request's result. As shown in the image below, if the server sends a message indicating the request was successful or rejected, the Job will succeed and terminate.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fv4fNA%2FbtrcLBB0H2G%2F10LNc5co46tiTLwI3XUNC1%2Fimg.png)

However, if the server doesn't provide a result, the client will continue to wait for a response indefinitely.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fdk2bEE%2FbtrcNJyWaPq%2FxysFUCL5DfftkZ6NG3kSt0%2Fimg.png)

**In such situations, it's necessary to cancel the Job after a certain period. Additionally, handling exceptions that arise when a Job is cancelled is also required.**

Now, let's explore how to cancel a Job and how to handle exceptions.

#### cancel()

You can simply call cancel() to cancel a job.

```kotlin
fun main() = runBlocking<Unit> {
    val job = launch(Dispatchers.IO) {
        delay(1000)
    }

    job.cancel()
}
```

However, if you just cancel it, it's hard to identify the cause, so let's pass the reason for cancellation as an argument.

You can indicate the reason for cancellation by passing two arguments to cancel(): `message: String` and `cause: Throwable`.

Additionally, you can determine the cause of cancellation by using `getCancellationException()` on the Job.

```kotlin
fun main() = runBlocking<Unit> {
    val job = launch(Dispatchers.IO) {
        delay(1000)
    }

    job.cancel("Job Cancelled by User", InterruptedException("Cancelled Forcibly"))
    
    println(job.getCancellationException()) // cancel 원인 출력
}
```

The output of the above code is as follows. The type of Exception passed during cancellation is fixed as CancellationException. Although InterruptedException is passed above, the output is CancellationException, and the Throwable passed during cancellation is not reflected.

```
java.util.concurrent.CancellationException: Job Cancelled by User

Process finished with exit code 0
```

<br>

#### Printing the Cancellation Cause

Printing the cause is simple. When a Job's cancellation is complete, the method within `invokeOnCompletion` is called, and you can use this to print the cancellation cause.

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

In the code above, you can receive a throwable from `job.invokeOnCompletion()` and print that throwable. Even if an InterruptedException is passed, the throwable will be caught as a CancellationException.

However, the issue is that `invokeOnCompletion` is executed not only when the Job is cancelled but also when it completes execution.

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

When execution completes without cancellation, the throwable becomes null. Therefore, this part needs to be handled as follows.

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

Then you can see the following result.

```
Completed with no error

Process finished with exit code 0
```

The reason why the method within `invokeOnCompletion` is called both when the Job completes execution and when it completes cancellation is related to the Job's state variables.
