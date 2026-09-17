# Making Coroutine Jobs Lazy

When using the coroutine builder `launch`, you can observe that a Job is created.

**A Job is an asynchronous operation that does not return a result and will execute to completion unless an exception occurs.**

This article will delve deeper into controlling when and how a Job's asynchronous operations are executed.

### Job Creation

**When using the coroutine builder `launch` method without any specific options, the created asynchronous Job starts executing immediately after creation.**

```kotlin
fun main() = runBlocking<Unit> {
	val job = launch {
		println(1)
	}
}
// 1
```

If you create a Job as shown in the code above, it starts executing immediately upon creation. Creating Jobs in this manner can reduce the flexibility of coroutine execution, as they must be created and run precisely where needed.

**To address this, there's an option to create a Job and then have the coroutine dispatched to a thread for execution only when needed.**

<br>

### Executing Jobs Lazily

To prevent a Job from executing immediately after creation, you must pass the `CoroutineStart.LAZY` argument to the `launch` method when creating the Job, as shown below.

```kotlin
fun main() = runBlocking<Unit> {
  val job = launch(start = CoroutineStart.LAZY) {
    println(1)
  }
}
// Nothing is printed.
```

When a Job is created as above, it does not execute but enters a waiting state (created state). We refer to this as lazy execution of the Job.

Now, let's explore how to execute these lazily created Jobs.

<br>

#### start() or join()

A lazily created Job can be executed via `start()`.

Calling `start()` or `join()` immediately executes the created coroutine.

```kotlin
fun main() = runBlocking<Unit> {
  val job = launch(start = CoroutineStart.LAZY) {
    println("가나다")
  }

  job.start() // or job.join()
}
// 가나다
```
