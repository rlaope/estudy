# CoroutineContext

In the CoroutineDispatcher section, we learned about Dispatcher and CoroutineExceptionHandler.

- Dispatcher: A manager that holds the thread pool where coroutines will execute.
- CoroutineExceptionHandler: A handler for when an Exception occurs in a coroutine.

Interestingly, these two elements can be placed directly where a **CoroutineContext** is expected.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FFmduT%2FbtrcSswLPkL%2F7MPfksDRG1B2KjKnpkpqjk%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FN7ODh%2FbtrcUjGhQQ9%2F6QWBBQGLgCkYUnb6J0AK%2Fimg.png)

**This is possible because both Dispatcher and CoroutineExceptionHandler are implementations of interfaces that extend CoroutineContext.**

<br>

### CoroutineContext

**You can think of CoroutineContext as the environment in which a Coroutine executes.** The Dispatcher and CoroutineExceptionHandler mentioned above are also part of the coroutine's execution environment, and both can be included in a CoroutineContext to set up the coroutine's execution environment.

#### Combining CoroutineContexts

Let's combine a Dispatcher and a CoroutineExceptionHandler to create a single Context.

Here, we use the `operator fun plus` on CoroutineContext.

```kotlin
public interface CoroutineContext {
    public operator fun plus(context: CoroutineContext): CoroutineContext 
     ..
}
```

```kotlin
val exceptionHandler = CoroutineExceptionHandler { coroutineContext, throwable ->  }

val coroutineContext = Dispatchers.IO + exceptionHandler
```

Interpreting this code, a single CoroutineContext now includes `Dispatcher.IO` and `CoroutineExceptionHandler`, allowing the context to handle exceptions occurring on the IO Thread.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbuDmcO%2FbtrcMGQ401A%2FyFlyu5qLr331sqoyjf4tHk%2Fimg.png)

The CoroutineContext created this way can be used by placing it where a CoroutineContext is expected.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FpAjS0%2FbtrcNJzfJdG%2FWPGTQhLwFe8VYFqvJmqB5K%2Fimg.png)

<br>

#### Accessing CoroutineContext

We learned above that CoroutineContext is a collection of CoroutineContexts. Now, we will explore how to access a specific CoroutineContext from such a collection.

The image above was a simplified summary for easy understanding, but expressed in more detail, it looks like this:

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fsa3cv%2FbtrcMGctGUx%2FXoQ9RLAJFkdzevsXbtKpuK%2Fimg.png)

There are two CoroutineContexts that make up the CoroutineContext: Dispatcher and CoroutineExceptionHandler. Let's say the key for Dispatcher is "keyA" and for CoroutineExceptionHandler is "keyB". Of course, the key is not actually a string, but we express it this way for easier understanding.

In the image above, we want to retrieve the ExceptionHandler from the parent CoroutineContext.

```kotlin
fun main() {
    val exceptionHandler = CoroutineExceptionHandler { coroutineContext, throwable -> }

    val coroutineContext = Dispatchers.IO + exceptionHandler // Parent CoroutineContext = Dispatcher + ExceptionHandler

    val exceptionHandlerFromContext = coroutineContext[exceptionHandler.key] // Accessing child CoroutineContext via Key

    if (exceptionHandler === exceptionHandlerFromContext) { // Comparing for identity to check if they are the same object
        println(true)
    }
}
```

As shown above, it's possible to request a child CoroutineContext by passing a key to the CoroutineContext.

If you compare the value retrieved from the context via the key with the previously combined CoroutineContext using identity (`===`) comparison, you can see that `true` is printed.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbmgCZp%2FbtrcM1Hxi7w%2FDnYHHuIDEZiKYs71krajUK%2Fimg.png)

<br>

#### Removing CoroutineContext from CoroutineContext

If CoroutineContext can be accessed as shown above, it can naturally also be removed. The removal of a CoroutineContext is possible through the `minusKey()` method.

```kotlin
fun main() {
    val exceptionHandler = CoroutineExceptionHandler { coroutineContext, throwable -> }

    val coroutineContext = Dispatchers.IO + exceptionHandler

    val minusContext = coroutineContext.minusKey(exceptionHandler.key)
}
```

This can be illustrated as follows:

1. Request to remove CoroutineExceptionHandler (CoroutineContext)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdAftz8%2FbtrcOj1nnsd%2F07Q4eWEixskFmRFNOtAbQ1%2Fimg.png)

2. Removal complete

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FZxykz%2FbtrcMGKo46X%2F4YkqPwG5CpMF7X45NaR8y1%2Fimg.png)

3. Returns itself

Calling `minusKey` returns the removed CoroutineContext.

```kotlin
public fun minusKey(key: Key<*>): CoroutineContext
```

```kotlin
val minusContext = coroutineContext.minusKey(exceptionHandler.key)
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbUp4ka%2FbtrcOfEY90Q%2FaeUWRldIj5IBuHbv67sBsK%2Fimg.png)
