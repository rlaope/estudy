# Functions Outside a Class (A package-level/ Top-level function)

Unlike Java, Kotlin allows you to write functions outside of a class.

If the function's logic is unrelated to the class's properties or purpose, it should be placed outside the class.

This is possible because Kotlin is not exclusively an object-oriented language and can also be used functionally.

```kt
package com.example

fun hello(){
    println("hello")
}
```

When the above code is converted to Java, it takes the following form.

```java
package com.example;

public final class ExampleKt {
    public static void sayHello() {
        System.out.println("hello");
    }
}
```

In the code above, the ExampleKt class is automatically generated based on the Kotlin file's name, and package-level functions are created as static methods within that class.

### How to Specify the Class Name When Converting to Java

```kt
@file:JvmName("MyFunctions")

package com.example

fun hello(){
    println("hello")
}
```

In the Kotlin code above, the @file:JvmName("MyFunctions") annotation means that the name of the class generated from this file will be changed to MyFunctions.

```java
package com.example;

public final class MyFunctions {
    public static void sayHello() {
        System.out.println("hello");
    }
}
```

In other words, you can specify it using @file:JvmName("$ClassName").
