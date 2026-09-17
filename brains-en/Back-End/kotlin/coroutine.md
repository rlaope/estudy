# Kotlin Coroutines and Concurrency Programming

## Coroutine
**An individual task is called a routine**, and coroutine is a compound word created to mean that multiple routines cooperate (co).

## Process and Thread
When a program executes, a process starts. A process includes all its memory, stack, open files, etc., so context switching between processes incurs significant overhead.

In contrast, threads only have their own independent stack and share the rest among themselves, so **context switching costs are low**, making them widely used in programming.

### Threads
To create a thread routine, you can inherit from the Thread class or implement the Runnable interface.

```kt
class SimpleThread: Thread(){
    override fun run(){
        println("SimpleThread Current Thread : ${Thread.currentThread}")
    }
}

class SimpleRunnable: Runnable{
    override fun run(){
        println("SimppleRunnable Current Threads : ${Thread.currentThread()}")
    }
}

fun main(){
    val thread = SimpleThread()
    thread.start()

    val runnable = SimpleRunnable()
    val thread1 = Thread(runnable)
    thread1.start()
}
```

It can be used this way, and if you use an anonymous class, it can be used as follows.

```kt
object: Thread(){
    override fun run(){
        println("Main Current Threads : ${Thread.currentThread()}")
    }.start()
}

Thread {
    println("Main2 Current Threads : ${Thread.currentThread()}")
}.start()
```

### Using a Thread Pool
You can design a system to create a certain number of threads beforehand and reuse them as needed.

**newFixedThreadPool()** creates threads equal to the number of arguments, and when a task needs to be performed, it selects a reusable thread from this pool.

```kt
val service: ExecutorService = Executors.newFixedThreadPool(8)
```

## Basic Concepts of Coroutines
Processes or threads incur significant overhead when attempting context switching to suspend their current task and execute another routine.

Coroutines can reduce this overhead by **suspending the routine without costly context switching**.

```kt
fun main(){
    GlobalScope.launch {
        delay(1000)
        println("world!")
    }

    println("hello")
    Thread.sleep(2000L)
}
```

This is a basic example of a coroutine, where the `GlobalScope.launch { ... }` part is the coroutine code.

In `main`, "hello" is printed and the program terminates after 2 seconds. The coroutine code runs in the background, separate from the main thread, and executes after 1 second.

Therefore, "hello" is printed, then "world" is printed after 1 second, and then the main thread terminates after another 1 second.

Since coroutines are used, this becomes Non-Blocking code that runs independently of the main thread.

Functions used in coroutines must be suspend functions declared with `suspend()` to utilize coroutine features.

### Using async
`async` can also launch new coroutines, but unlike `launch`, it returns a result via `Deferred<T>`.

At this time, `await()` can be used to receive the deferred result.

```kt
private fun workInParallel(){
    val one = GlobalScope.async {
        doWork1()
    }

    val two = GlobalScope.async{
        doWork2()
    }

    GlobalScope.launch {
        val combined = one.await() + "_"
    }

    suspend fun doWork1(): String{
        delay(1000L)
        return "Work1"
    }

    suspend fun doWork2(): String{
        delay(3000L)
        return "Work2"
    }
}
```

When used this way, the `combined` variable will store the combined value after both the `one` and `two` threads have completed. Therefore, it will print the `combined` variable after 3 seconds, which is when `doWork2` called by `two` completes.

### Start Point Properties
`launch` and `async` can specify the start timing using the `start` parameter.
- DEFAULT: Starts immediately.
- LAZY: Starts the coroutine lazily, initiated by `start()`, `await()`, etc.
- ATOMIC: Starts using an optimized method.
- UNDISPATCHED: Starts using a distributed processing method.

```kt
val sam = async(start = CoroutineStart.LAZY) { doWork1() }
...
sam.start() // sam.await()
```

### runBlocking
`runBlocking` launches a new coroutine and blocks the current thread until it completes.

In the examples above, `delay` or `readLine` was used in the main thread to prevent it from terminating before the coroutine finished. By using `runBlocking` in the `main` function, the main thread can be held until the coroutine completes.

```kt
fun main() = runBlocking {
    laucnh {
        delay(1000)
        println("world!")
    }

    println("hello")
    // Thread.sleep(2000L)
}
```

As such, the main thread will not terminate until "world" is printed, even without using `sleep`.
This can also be used in member methods within a class.

### join() function
To wait for a coroutine's task to complete, you can use the `join` function.

```kt
val sam = launch {
    delay(1000L)
    println("world")
}

println("hello")
sam.join()
```

Using `join` explicitly waits for the coroutine to complete. To cancel it, you can use the `cancel()` function.

### Coroutines
Coroutines always execute within a specific context.

The `Dispatcher` determines which context to execute in.

```kt
val sam = laucnh(Dispatchers.Default){
    delay(1000L)
    println("world")
}

val sam2 = launch(newSingleThreadContext("MyThread")) { }
```
It is used as a parameter for `launch` in this way, and there are 4 types of `Dispatchers` that can be configured.

- Dispatchers.Unconfined: Works on the main thread, not a recommended option.
- Dispatchers.Default: Default value for the dispatcher.
- Dispatcher.IO: A shared pool suitable for I/O-intensive operations, especially for files or socket I/O with many blocking operations.
- newSingleThreadContext: The user creates and uses a new thread pool directly. Creating new threads is costly, and they should be released or terminated when no longer needed.

## Basic Operation Control

- The `repeat()` function can be used to write repetitive code.
- The `cancel()` function can be used to terminate a function.
- The `cancelAndJoin()` function can be used to terminate a function.
- `withContext(NonCancellable) { ... }` can be used to guarantee the execution of `finally` blocks. If a `finally` block contains time-consuming operations or suspend functions, its execution might not be guaranteed. Therefore, `withContext` ensures that the `try-catch-finally` block operates within its context.
- The `withTimeout(time)` function can be used to cancel a coroutine after a certain execution time.

### Channels
Channels act as a **promised conduit for exchanging data**.

When implementing channels, the `SendChannel` and `ReceiveChannel` interfaces provide a way to transmit streams of values.

For actual transmission, **the suspend functions `send()` and `receive()` are used**.

```kt
fun main() = runBlocking {
    val channel = Channel<Int>()
    launch {
        for(x in 1..5) channel.send(x * x)
    }
    repeat(5){
        println(channel.receive())
    }
    println("Done")
}
```

Values sent in the `launch` block can be received and read. Unlike a regular queue, if there are no elements to deliver, the channel can be closed via `close`.

Also, when creating a channel, **passing an `Int` value within parentheses sets the buffer size** accordingly.

### produce
`produce` is a coroutine with a channel attached, making it easy to construct producer-side code.

Sending values to a channel makes it a producer, and consumers use the `consumeEach` function as an extension to consume stored elements instead of a `for` loop.

```kt
fun main(){
    val result = producer()
    result.consumeEach { println("$it") }
}

fun CorutineScope.producer(): RecieveChannel<Int> = produce {
    var total = 0
    for(x in 1..5){
        total += x
        send(total)
    }
}
```

`producer` produces values and returns a `ReceiveChannel`. Therefore, `result` uses the `consumeEach` extension function of `ReceiveChannel` to process each element.

### select
`select` allows you to receive result values that depend on the execution time of each channel.

```kt
val routine1 = GlobalScope.produce {
    delay(Random().nextInt(100).toLong)
    send("routine1")
}

val routine2 = GlobalScopoe.produce {
    delay(Random().nextInt(100).toLong)
    send("routine2")
}

val result = select<String> {
    routine1.onReceive { result -> result }
    routine2.onReceive { result -> result }
}

println("Result = $result")
```

Through `select` used in `result`, the `result` value will store whichever of `routine1` or `routine2` completes first.

## Synchronization Techniques

### synchronized methods and blocks
In Kotlin, to use a synchronized method, you must use the `@Synchronized` annotation.
```kt
@Synchronized fun synchronizedMethod() {
	println("sync = ${Thread.currentTread()}")
}
```

### volatile
Java's `volatile` can be used in Kotlin in the same way.

Variables are usually cached for performance, but if multiple threads read or write values, the data can become inconsistent or corrupted.

To prevent this, you can declare a variable with the `volatile` keyword **to prevent it from being cached**.

If one thread reads and writes a volatile variable, and other threads only read the volatile variable, it is guaranteed that the volatile variable was most recently written by the reading thread.

### Atomic Variables
Atomic variables refer to the incrementing, decrementing, adding, or subtracting of a specific variable being performed as a single machine instruction. Since **no one can interfere while this operation is being performed**, it ensures data integrity.

While there are no issues in sequential programs, if a large number of independent routines share a single variable, the code can be interrupted at any time.

Using atomic variables ensures integrity because the variable's operation part is compiled into a single CPU machine instruction.

```kt
var counter = AtomicInteger(0) // Initialize as an atomic variable

...

counter.incrementAndGet() // Increment value using the atomic variable's member method

...

println("Count = ${counter.get()}") // Read value

```

### Mutual Exclusion
Mutual exclusion **ensures that concurrency never occurs when code is in a critical section, and only one routine can access it.**

In Kotlin, you can create a critical section using `Mutex`'s `lock` and `unlock`.

```kt
val mutex = Mutex()
...
mutex.lock()
... // Critical section code to protect
mutex.unlock()
...
```

A critical section can be created this way. `tryLock()` can be used to check if the critical section is locked, and `hostLock()` can be used to check if it's locked by the owner.

Using the `withLock` lambda expression allows for easy use of patterns like `mutex.lock() try{...} finally{ mutex.unlock() }`.

```kt
mutex.withLock {

	... // Critical section code
    
}
```
