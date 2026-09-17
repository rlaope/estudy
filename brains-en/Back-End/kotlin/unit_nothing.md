# Differences between Unit and Nothing in Kotlin

## Unit

**Equivalent to Java's void.**

It is said that a function implicitly returns Unit if it doesn't return anything useful or has no value to return.

And such functions can perform operations with side effects. They might log/print something or perform manipulations without a return value.

```kt
fun printHelloUnit(name: String?): Unit {
    if (name != null)
        println("Hello $name")
    else
        println("Hi there!")
}

//The Unit return type declaration is also optional. The above code is equivalent to:

fun printHello(name: String?) {
    if (name != null)
        println("Hello $name")
    else
        println("Hi there!")
}
```

In the example above, if you explicitly mention `Unit` as the return type, **the compiler suggests removing the explicit mention because the `Unit` type is redundant.**

## Nothing

Literally, it means nothing, **there is no return to life, and the game ends there.**

That is, the function will not return from here; instead, an exception will be thrown or it will enter an infinite loop.

Furthermore, any code written after calling a function with a `Nothing` return type will be marked as unreachable by the compiler.

### Conclusion

- `Unit` says, 'I will return, but there's nothing of value for you to care about.'
- `Nothing` is like saying, 'I will never return.'

Thus, it helps to refer to the functionalities more clearly and individually.

```kt
class NothingClass {

    fun returnName(isSuccess: Boolean): String? {
        return if (isSuccess) {
            println("Sara")
            "Sara"
        } else null
    }

    fun reportError(): Nothing {
        println("no name found")
        throw RuntimeException()
        // var i = 1 // unreachable code
    }
// here if you don't specify Nothing explicitly, it shows compile time error
fun iWillAlwaysThrowException() : Nothing =  throwException("Unnecessary Exception")
}

fun main() {
val nothingClass = NothingClass()
val name: String = nothingClass.returnName(true) ?: nothingClass.reportError() // Compiles and the return type is String or Nothing 
val noName: String = nothingClass.returnName(false) ?: nothingClass.reportException()
}
```
