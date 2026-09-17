# Optimize Higher-Order Functions Using `inline fun`

### Problems When Receiving Lambda Expressions as Function Parameters

Typically, when a function is called, it executes as a subroutine.

On the other hand, when a function declared with `inline fun` is called, instead of executing a function call, **the code inside that function is inserted and executed** at the call site.

```kotlin
fun main(args: Array<String>) {
  printWorldAfterFunction {
    println("Hello")
  }
}

fun printWorldAfterFunction(function: () -> Unit) {
  function()
  println("World")
}
```

In the code above, when the `printlnWorldAfterFunction` function is executed with a `() -> Unit` type lambda expression, the `() -> Unit` type lambda expression is created as an anonymous class and instantiated. If this code is converted to bytecode and then decompiled into Java code, it results in the following:

```java
public final class InlineFunKt {
   public static final void main(@NotNull String[] args) {
      Intrinsics.checkNotNullParameter(args, "args");
      printWorldAfterFunction((Function0)null.INSTANCE);
   }

   public static final void printWorldAfterFunction(@NotNull Function0 function) {
      Intrinsics.checkNotNullParameter(function, "function");
      function.invoke();
      String var1 = "World";
      System.out.println(var1);
   }
}
```

We can see that the `function` parameter we provided is created as an anonymous class implementing `Function0`, and an instance of this class is passed to `printWorldAfterFunction`.

> Although it's shown as `null.INSTANCE` here, `Function0` is actually an object that executes `println("Hello")` when `invoke` is called.

**In other words, using a lambda as a function argument creates a new class instance.**

Creating a new instance of an anonymous class in memory every time the function is called is detrimental to performance.

<br>

### Performance Optimization Using `inline fun`

To solve the problem mentioned above, `inline fun` was introduced. Let's declare `printWorldAfterFunction` as an `inline fun`.

```kotlin
fun main(args: Array<String>) {
  printWorldAfterFunction {
    println("Hello")
  }
}

inline fun printWorldAfterFunction(function: () -> Unit) {
  function()
  println("World")
}
```

If we convert it to bytecode again and decompile it into Java, we get the following result:

```java
public final class InlineFunKt {
   public static final void main(@NotNull String[] args) {
      Intrinsics.checkNotNullParameter(args, "args");
      int $i$f$printWorldAfterFunction = false;
      int var2 = false;
      String var3 = "Hello"; 
      System.out.println(var3); // 그대로 코드로 대입
      String var4 = "World";
      System.out.println(var4); // 그대로 코드로 대입
   }

   public static final void printWorldAfterFunction(@NotNull Function0 function) {
      int $i$f$printWorldAfterFunction = 0;
      Intrinsics.checkNotNullParameter(function, "function");
      function.invoke();
      String var2 = "World";
      System.out.println(var2);
   }
}
```

In this code, we can see that the `main` function no longer calls `printWorldAfterFunction`. Furthermore, an anonymous class implementing `Function0` and its instance are no longer created. Instead, the code of `printWorldAfterFunction` and the lambda expression we passed as the `function` parameter are directly inserted into the code.

**This way, although the code becomes slightly longer, it enables performance optimization by avoiding the creation of new anonymous object instances.**

For reference, declaring a function that does not receive a lambda expression as an argument with `inline fun` does not yield any significant performance difference.

```kotlin
inline fun printString(string: String) {
	println(string)
}
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbcbkuR%2FbtsGa0lJ2hv%2Fc1GVYcw4Ksd8XEtePRZz0k%2Fimg.png)

The reason is... I believe you'll understand if you've read this article carefully.
