# Why can't the Lombok library be used in Kotlin?

## Overview

When developing with Java, there's hardly anyone who hasn't used Lombok. When I switched to Kotlin, I thought Lombok wasn't used because of data classes, but it turns out it can't be used at all... I'll summarize it today.

## When using Java and Kotlin together

Kotlin compiler -> .class generation -> Java compiler -> .class generation -> Annotation processing -> .class file generation

The reason Lombok cannot be used in Kotlin is that annotation processing occurs during Java compilation, after Kotlin compilation.

## data class

Still, in Kotlin, because there's `data class`, there seems to be no real need to use Lombok.

Functions provided by data classes
- `equals()`/`hashCode()` pair
- `toString()` of the form "User(name=John, age=42)"
- `componentN()` functions corresponding to the properties in their order of declaration.
- `copy()` function (see below).

```kotlin
data class JeongWoo(
    val name: String,
    val age: Int
)
```
