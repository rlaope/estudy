# Coroutine

In Kotlin, coroutines are sometimes used to write non-blocking code. Today, we'll explore this, starting with an understanding of thread structure and the necessity of multi-threaded operations.

### Thread Structure and the Necessity of Multi-Threaded Operations

A single process can have multiple threads, and each thread performs tasks independently.

For example, a JVM process is structured as follows:

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FGxqBD%2FbtrctuPsMxG%2Fta9scRvF6HCYDnQHqA2MWK%2Fimg.png)

JVM Process Structure

Let's look at the main thread in the diagram above. **A JVM process starts with the main thread executing the main function. If there are no user threads other than the main thread within the process, the process will be forcibly terminated when the main thread ends.**

At this point, the main thread can execute one task at a time. However, the diagram also shows two other threads besides the main thread. These threads are user-created threads that can perform tasks just like the main thread.

> They are typically created as daemon threads, but sometimes they can be user threads like the main thread. A JVM process terminates when all user threads have finished.

Naturally, tasks that place a heavy load on the main thread should be avoided. Instead, other threads should be created to handle these high-load tasks.

#### Limitations

However, many attempts have been made to solve the problem of main thread blocking in applications written in JVM languages. The most representative methods include the following:

#### Thread Class Inheritance

By creating a new class that inherits from the `Thread` class and overriding its `run` method, you can define the task to be executed in a new thread. When you create an instance of this new class and call its `start` function, the task will be executed in a new thread.

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
Output
[Thread-0] New Thread Running
*/
```

**However, thread instances created using this method consume a lot of memory and have the disadvantage of being difficult to reuse. Furthermore, because developers must directly create and manage threads, the possibility of memory leaks increases.**

To solve these problems, it's necessary for threads, once created, to be easily reusable, and for their management to be handled by a pre-built system rather than the developer. To fulfill this role, the Executor framework emerged.

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
Output
[pool-1-thread-1] 새로운 작업1 시작
[pool-1-thread-2] 새로운 작업2 시작
*/
```

The Executor framework reduces the developer's responsibility for thread management and increases the reusability of created thread instances.

It creates a 'thread pool,' a collection of threads, based on user requests, and when a user submits a task, it assigns that task to one of the threads in the pool.

#### Rx Library

The Rx Library is strictly a library designed to aid reactive programming, allowing you to define data streams and process them by subscribing to them.

Within the library, methods like `subscribeOn` and `observeOn` allowed for separating the thread that publishes data from the thread that subscribes to it. However, it had the inconvenience of requiring even simple tasks to be converted into data streams.

```kotlin
publisher.subscribeOn(Schedulers.io())
         .observeOn(AndroidSchedulers.mainThread())​

```

These approaches have limitations. Specifically, the **unit of work is a thread**. We said that to prevent main thread blocking, tasks could be offloaded to other threads, but the statement 'the unit of work is a thread' might be confusing. Let's explore further.

Threads are expensive to create, and switching between tasks incurs high overhead. Furthermore, if one thread has to wait for a task from another thread, the waiting thread is essentially **blocked**, preventing other tasks from using it. In such a scenario, the thread waits idly until the other task completes, leading to wasted resources. This is the chronic problem that arises when the unit of work is a thread.

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

The code above operates as follows:

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcX1U1Z%2FbtsFg2SuZB7%2FjUAkOwcxRThy0bPuBAZFt0%2Fimg.png)

Thread Blocking

Looking at Figure 3 above, during the execution of Task 1 on Thread 1, the result of Task 2 on Thread 2 becomes necessary for Task 1 to proceed. At that point, Thread 1, which was executing Task 1, becomes blocked and idle, and a significant amount of time passes until it receives the result from Thread 2 and can resume.

While it would be fortunate if blocking only occurred for a short period, in real-world scenarios, blocking can repeatedly occur, preventing threads from utilizing even half of their potential performance.

<br>

### How Coroutines Overcome Limitations

Coroutines are units of work that can be executed using threads. However, a coroutine running on a thread can be suspended at any time, which is akin to being able to attach and detach a coroutine from a thread. For this reason, coroutines are called `lightweight threads`.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdoTuZ1%2FbtsFh5H1maq%2Fmb5qvJL8nVHWcjAdvXEb8k%2Fimg.png)

How Coroutines Work

Let's learn more about lightweight threads. Let's use coroutines in the diagram below. Assume that Task 1 and Task 2 are replaced by Coroutine 1 and Coroutine 2, respectively, and Coroutine 3 is additionally requested for execution.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fxz6lD%2FbtsFm7xyLGE%2FAJEcFdgfVFwFZlhuNqCKx0%2Fimg.png)

How Coroutines Work 2

1. Coroutine 1 is created and requested on Thread 1, and Coroutine 2 is created and requested on Thread 2. During the execution of Coroutine 1 on Thread 1, the remaining operations require a result from Coroutine 2. However, since Coroutine 2's task is not finished, Coroutine 1 cannot complete its work. At this point, instead of blocking Thread 1, Coroutine 1 yields its execution rights, allowing another coroutine to run on the thread.
2. When Coroutine 3 is additionally requested, Coroutine 3 runs on the now-free Thread 1.
3. After Coroutine 3 finishes execution, it returns its execution rights to Thread 1.
4. Subsequently, when executed on Thread 2, Coroutine 2's task completes and returns a result. Then, Coroutine 1 executes using Thread 1 or 2, whichever is not currently assigned a task.

**In summary, coroutines yield their execution rights when the thread is not needed.** This reduces situations where threads are blocked, allowing each thread to be utilized to its fullest potential. Threads are very expensive objects. Coroutines optimize thread usage by yielding the thread when it's no longer needed.

In summary, a coroutine is a **suspendable unit of work** that runs within a thread.

Multiple coroutines can run on a single thread, yielding the thread to each other.
