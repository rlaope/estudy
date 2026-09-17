# infix, inline function

### infix

The `infix` keyword is a simple keyword that allows functions to be expressed in a readable way.

You can think of it as a function expression used between objects.

```kotlin
infix fun String.add(other: String): String {
	return this + other
}

fun main() {
	val price = "3000"
	println(price add "원") // 3000 언
}

```

It seems to be mainly applied to extension functions and is easy to use. But I don't think I'll use it much.

### inline

Before understanding `inline`, let's look into first-class functions. Simply put, a first-class function is a function that takes another function as a parameter (higher-order functions, lambdas, anonymous functions).

Kotlin can be seen as supporting functional programming due to these first-class functions.

```kotlin
fun doSomething(body : () -> Unit) {
    body()
}

fun callFunction() {
    doSomething { println("문자열 출력!") }
}
```

When written like this, if converted to Java, it becomes as follows.

```java
public void callFunction() {
    doSomething( new Function() {
        @Override
        public void invoke() {
            System.out.println("문자열 출력!");
        }
    });
}
```

You can see that a Function Object is created in this way. An `inline` function can prevent this phenomenon.

```kotlin
inline fun doSomething(body : () -> Unit) {
    body()
}

fun callFunction() {
    doSomething { println("문자열 출력!") }
}
```

```java
public void callFunction() {
    System.out.println("문자열 출력!");
}
```

As shown above, simply adding the `inline` keyword allows the Java code to be converted in the way we want.

Looking at examples that utilize `inline`, Scope Functions are a prime example. Let's look at an example using `also`.

```kotlin
@kotlin.internal.InlineOnly
@SinceKotlin("1.1")
public inline fun <T> T.also(block: (T) -> Unit): T {
    contract {
        callsInPlace(block, InvocationKind.EXACTLY_ONCE)
    }
    block(this)
    return this
}
```

```kotlin
fun main() {
    val prices = listOf(3000, 5000, 6000)
    prices.also {
        println("Total Price : ${it.sum()}")
    }
}

- 결과 -
Total Price : 14000
```

You can see that the `also` function utilizes `inline`.

However, one might think that using `inline` is always more efficient! While that's true, the performance impact is minimal. This is because the JVM already performs inlining.

When should you use it? It's effective when writing **Higher-Order Functions** (filter, map, fold ....).

If you write higher-order functions without using `inline`, you'll see that there's a cost associated with creating anonymous objects in the Java code. By applying `inline`, you can confirm that objects are not created, and the code is transformed in the way we desire.
