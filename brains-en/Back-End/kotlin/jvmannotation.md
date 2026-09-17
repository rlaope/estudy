# @JvmField && @JvmStatic Annotations

## @JvmField

`@JvmField` means "do not generate getters and setters."

In the following Kotlin code, the property `var count` generates a getter/setter.
```kt
class A{
    var count = 0
}
```

If you attach the `@JvmField` annotation, getters and setters will not be generated when converted to Java.
```kt
class A {
   @JvmField
   var count = 0
}
```

## @JvmStatic

It generates additional getter and setter methods to allow static access to functions and properties.

In the following class A, we created a global variable by declaring a variable named `count` within a `companion object`.
```kt
class A {
    companion object {
        var count : Int = 0
    }
}
```

```java
public final class A {
   private static int count;
   public static final class Companion {
      public final int getCount() {
         return A.count;
      }
      public final void setCount(int var1) {
         A.count = var1;
      }
   }
}
```

When converted to Java, you can see that `count` is declared in class A, but the Getter and Setter are registered in the `A.Companion` class.

To access the Getter and Setter functions from Java, you must use `Companion` as follows.
```kotlin
A.Companion.getCount();
A.Companion.setCount(100);
```
This is precisely why `static` and `companion object` are said to be different. While there's no difference when using only Kotlin, a distinction arises when Java and Kotlin are used together.

To use a `companion object` like a `static` member, you must use `@JvmStatic`.

```kt
class A {
    companion object {
        @JvmStatic var count : Int = 0
    }
}
```

If you convert it to Java, you can see that `count` is declared in class A, and both getter and setter functions are generated in both class A and `A.Companion` class.

```java
public final class A {
   private static int count;
   public static final int getCount() {
      return count;
   }

   public static final void setCount(int var0) {
      count = var0;
   }

   public static final class Companion {
      public final int getCount() {
         return A.count;
      }
      public final void setcount(int var1) {
         A.count = var1;
      }
   }
}
```

In Java, you can access the above code via `A.Companion`, but direct access like `A.getCount` is also possible.
```java
A.getCount();
A.setCount(10);
A.Companion.getCount();
A.Companion.setCount(10);
```

Similarly, when using `@JvmStatic`, classes can also be accessed directly without the `Companion` keyword.

In summary, `@JvmStatic` is an annotation used to declare variables registered in a `Companion` object as if they were Java's `static` members.
