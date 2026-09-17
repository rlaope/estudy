# suspend fun

A suspend function, declared with `suspend fun`, is a **special function that can contain suspension points within its body.**

Recall that coroutines can suspend at any time and yield the thread. Suspend functions serve to create reusable blocks of code that include suspension points, which are executed within a coroutine.

```kotlin
fun main() = runBlocking<Unit> {
    delay(100L)
    println("Hello Coroutines")
    delay(100L)
    println("Hello Coroutines")
}
```

In this code, the `delay` function and `println` function are repeated. Therefore, this code can be refactored into a function as follows:

```kotlin
fun delayAndPrintHelloCoroutines() {
    delay(100L)
    println("Hello Coroutines")
}
```

However, if you check in the IDE, you'll see an error.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FPNvre%2FbtsFDDLL7Lm%2FyXnmSTWR7uHX2RRyawKAMk%2Fimg.png)

This means that `delay`, which is a suspend function, can only be called from another suspend function (`suspend fun`). Why does this error occur? It's because the `delay` function is declared as a suspend function since it makes the coroutine yield the thread for a certain period. Therefore, the caller of this function must also be a suspend function.

Simply changing `fun` to `suspend fun` resolves the issue.

```kotlin
suspend fun delayAndPrintHelloCoroutines() {
    delay(100L)
    println("Hello Coroutines")
}
```

<br>

### Suspend functions are not coroutines.

So, how long would it take to execute the following `main` function?

```kotlin
fun main() = runBlocking<Unit> {
    delayAndPrintHelloCoroutines()
    delayAndPrintHelloCoroutines()
}

suspend fun delayAndPrintHelloCoroutines() {
    delay(100L)
    println("Hello Coroutines")
}
```

The answer is 200 milliseconds. This is because calling a suspend function does not create a new coroutine; it executes within the same coroutine, waiting 100 milliseconds and then printing. To confirm this, if you run the following code, you'll see it takes slightly more than 200 milliseconds.

```kotlin
fun main() = runBlocking<Unit> {
    val startTime = System.currentTimeMillis()
    delayAndPrintHelloCoroutines()
    delayAndPrintHelloCoroutines()
    println("${System.currentTimeMillis() - startTime}") // Prints 211
}
```

If you want a suspend function to run in a new coroutine, you must wrap it with a coroutine builder like `launch` or `async`.

```kotlin
fun main() = runBlocking<Unit> {
    val startTime = System.currentTimeMillis()
    val job1 = launch {
        delayAndPrintHelloCoroutines()
    }
    val job2 = launch {
        delayAndPrintHelloCoroutines()
    }
    job1.join()
    job2.join()
    println("${System.currentTimeMillis() - startTime}") // Prints 110
}
```

Then, each suspend function will execute in parallel, and you can see that the execution time is 110 milliseconds.

<br>

### Calling suspend functions from within suspend functions

```kotlin
fun main() = runBlocking<Unit> {
    val startTime = System.currentTimeMillis()
    delayAndPrintHelloCoroutines()
    delayAndPrintHelloCoroutines()
}
```

In this code, if the `delayAndPrintHelloCoroutines` method were declared with `fun`, it would result in an error. The reason is that `delay` is a suspend function. As seen here, a suspend function can only be called from another suspend function. This means that any function called from within a `suspend fun` must also be a `suspend fun`.

#### Suspend functions in coroutines

**So, where is the topmost suspend function called from?** It's from a coroutine. A regular function, once executed, runs to completion and does not have the concept of suspension. Since suspension is a special feature exclusive to coroutines, suspend functions must be called from within a coroutine. In the code above, the suspend function is called from within the `runBlocking` coroutine.
