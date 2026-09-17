# CoroutineDispatcher

In CoroutineDispatcher, 'Coroutine' refers to a coroutine, and 'Dispatch' means 'to send' in Korean. Therefore, CoroutineDispatcher refers to an object that sends coroutines. So, where does it send coroutines? It sends them to threads.

All tasks must run on a thread, and since coroutines are also tasks, they can only run on a thread. Therefore, an object is needed to send the created coroutines to threads, and CoroutineDispatcher fulfills this role.

**When we create a coroutine and request its execution from a CoroutineDispatcher, the CoroutineDispatcher sends the coroutine to one of the threads in its available thread pool.**

At this time, the CoroutineDispatcher distributes coroutines according to the load situation of the threads within the thread pool. This can be visually represented as follows.

1. User creates a coroutine and sends it to CoroutineDispatcher
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc7YLRL%2FbtrcxH9VZ9d%2FZYwUOjv3Oa2lvWcqtEETJK%2Fimg.png)

2. CoroutineDispatcher checks which threads are available in its thread pool, then sends the coroutine to that thread.
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FJLlcs%2Fbtrcs72rcMO%2FRL7ETLPINBfE5E59Fj5lv1%2Fimg.png)

3. The distributed thread executes the coroutine.
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FIUBcH%2FbtrcxKeyme5%2FCjji0o4L37PlqL4AgCMDU0%2Fimg.png)

This is very similar to the role of ExecutorService in the Executor framework.

<br>

### Creating a CoroutineDispatcher

While there are CoroutineDispatchers with an unlimited number of available threads, most CoroutineDispatchers have a limited number of available threads. Let's create a CoroutineDispatcher object with a limited number of available threads.

#### CoroutineDispatcher that can use multiple threads

Creating a thread pool in coroutines is easy. You can create a Dispatcher with 3 threads using simple code like this:

```kotlin
val dispatcher = newFixedThreadPoolContext(3, "ThreadPool")
```

<br>

#### CoroutineDispatcher that can use a single thread

```kotlin
val dispatcher = newSingleThreadContext("SingleThread")
```

A Dispatcher is responsible for sending coroutines to threads. When a coroutine is requested for execution in the dispatcher's work queue, if there is an available thread in that CoroutineDispatcher, the coroutine is sent to that thread for execution.

**In other words, if we just send a coroutine to a CoroutineDispatcher, the CoroutineDispatcher will send the coroutine to a thread for execution when it has an available thread.**

<br>

### CoroutineDispatcher Basic

If you configure the `coroutine-core` or `coroutine-android` libraries, you can use pre-configured dispatchers, eliminating the need to separately create or define dispatchers using `newFixedThreadPoolContext` or `newSingleThreadContext`. The default dispatchers are as follows:

- **Dispatchers.Main**: This dispatcher is used to execute tasks that interact with the UI. It requires a dependency on the `coroutine-android` library.
- **Dispatcher.IO**: **This dispatcher is optimized for Disk or Network I/O operations** and can be used with a dependency on the `coroutine-core` library.
- **Dispatcher.Default**: **This dispatcher is for executing CPU-intensive tasks (CPU Bound)**, such as sorting or JSON parsing.

Pre-defined dispatchers can be used with `launch` as follows:

```kotlin
launch(Dispatchers.Main) {
	updateButton() // 필요한 Job 수행
}
```
