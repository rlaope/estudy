# What is Kotlin and Why Do We Need It?

Kotlin is a new programming language that runs on the Java platform.
Kotlin is concise, practical, and emphasizes interoperability with Java code.
Kotlin can be used in almost any place where Java is currently in use.
Kotlin's main goal is to provide a more concise, productive, and safer alternative language suitable for all uses where Java is currently employed.

## Statically Typed Language
Kotlin is also a statically typed language. Statically typed means that the type of all program components can be known at compile time, and the compiler verifies types every time an object field or method is used within the program. In dynamically typed languages, any value can be assigned to a variable regardless of its type, and verification of method or field access occurs at runtime, which can lead to shorter code and more flexible creation and use of data structures. However, errors that are not caught at compile time can occur at runtime.

On the other hand, unlike Java, Kotlin does not require programmers to explicitly specify the type of every variable. This is because the Kotlin compiler can automatically infer variable types from the context.

Advantages of Static Typing
- Performance: Method calls are faster because there's no need to determine which method to call at runtime.
- Reliability: The compiler verifies program correctness, reducing the likelihood of runtime program errors.
- Maintainability: It's easier to work with unfamiliar code because you can tell what type other objects in the code belong to.
- Tool Support: Static typing enables safer refactoring, allows tools to provide more accurate code completion, and facilitates better IDE and other support features.

## Conciseness
Kotlin implicitly provides boilerplate code that exists in Java, such as getters, setters, and logic for assigning constructor parameters to fields, so Kotlin code doesn't get cluttered by such boilerplate.

> However, it cannot adhere to the principle of avoiding setters.

## Safety
The fact that Kotlin runs on the **JVM already guarantees a significant level of safety**. Kotlin's type system tracks non-nullable values and prohibits code that uses operations that could lead to an NPE at runtime.

```kt
var s2: String? = null // null이 될 수 있음
var s2: String = "" // null이 될 수 없음
```

## Kotlin Build Process

![](https://github.com/cheese10yun/TIL/raw/master/assets/kotlin-complie-flow.png)
Code compiled by the Kotlin compiler depends on the Kotlin runtime library.
