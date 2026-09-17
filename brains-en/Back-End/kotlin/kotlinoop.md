# Kotlin Object-Oriented Programming
This document explores object initialization, custom getters and setters, `lateinit`, `lazy init`, singleton object creation, the `Nothing` class, and more.
  
## const vs val
**Use the `const` modifier for compile-time constants**. The `val` keyword indicates that a variable cannot be reassigned once assigned, but this assignment happens at runtime.
  
Kotlin's `val` indicates an immutable variable. In Java, the `final` keyword serves the same purpose. So, why does Kotlin also support the `const` modifier? Compile-time constants must be top-level properties or members of a declaration, such as an object or a companion object. Compile-time constants are strings or wrapper classes of primitive types and cannot have getters. Compile-time constants must be assigned outside of all functions, including the `main` function, so that their values are available at compile time.

```kt
class Task(val name: String, _priority: Int = DEFAULT_PRIORITY) {

    companion object {
        const val MIN_PRIORITY = 1 // (1)
        const val MAX_PRIORITY = 5 // (1)
        const val DEFAULT_PRIORITY = 3 // (1)
    }

    var priority = validPriority(_priority) // (2)
        set(value) {
            field = validPriority(value)
        }

    private fun validPriority(p: Int) = p.coerceIn(MIN_PRIORITY, MAX_PRIORITY) // (3)
}
```

1. Compile-time constant
2. Property using a custom setter
3. Private validation function

<br>

## Creating Custom Getters and Setters
Like other object-oriented languages, Kotlin classes consist of data and functions that manipulate that data, commonly known as encapsulation.
Kotlin is unique in that everything is public by default. Therefore, it is assumed that detailed implementations of data structures related to information are needed, which seems to violate the philosophy of data hiding. Kotlin resolves this dilemma in a peculiar way. In Kotlin classes, fields cannot be declared directly.

```kt
class Task(val name: String) {
    var priority = 3
}
```
The `Task` class defines two properties: `name` and `priority`. One property is declared within the primary constructor, while the other is declared as a top-level member of the class. While `priority` can be assigned a value this way, it cannot be assigned a value when the class is instantiated.

```kotlin
var priority = 3
    set(value) {
        field = value
    }

val isLowPriority
    get() = priority < 3

```
As shown above, you can define getter and setter methods for derived properties.

<br>

## Lazy Initialization
```kt
class Customer(val name: String) {
//    val message: List<String> = loadMessage()
    val message: List<String> by lazy { loadMessage() }
    private fun loadMessage(): List<String> {
        return listOf("1", "2", "3")
    }
}

class CustomerTest {

    @Test
    internal fun `none lazy, 객체 생성 시점에 loadMessage를 호출한다`() {
        // val message: List<String> = loadMessage()
        val customer = Customer("yun")
        customer.message
        println(customer)
    }
    
    @Test
    internal fun `lazy, 객체 생성 시점에 loadMessage를 호출하지 않고, 조회 시점까지 lazy하게 간다`() {
        // val message: List<String> = loadMessage()
        val customer = Customer("yun")
        customer.message
        println(customer)
    }
}
```
- You can retrieve data at the desired time using the `lazy` keyword.

<br>

## lateinit
If there isn't enough information in the constructor to initialize a property, and you want to make it that property, you can use the `lateinit` keyword on it.
```kt
@EntityListeners(value = [AuditingEntityListener::class])
@MappedSuperclass
abstract class AuditingEntity : AuditingEntityId() {

    @CreationTimestamp
    @Column(name = "created_at", nullable = false, updatable = false)
    lateinit var createdAt: LocalDateTime
        protected set

    @UpdateTimestamp
    @Column(name = "updated_at", nullable = false)
    lateinit var updatedAt: LocalDateTime
        protected set
}
```
When using frameworks, there are values that are determined after an instance has already been created. In such cases, `lateinit` can be used. If initialization has not occurred, an exception will be thrown when accessing that item because it is a Not Null item.

## lateinit vs lazy
The `lateinit` modifier is used for `var` properties. The `lazy` delegate receives a lambda that is evaluated upon the first access to the property.

<br>

## Creating Singletons
The Singleton design pattern defines a mechanism to ensure that only one instance of a particular class exists.
1. Define all constructors of the class as private.
2. If necessary, provide a static factory method that instantiates the class and returns a reference to that instance.

```kt
object Singleton {
    val myPriority = 3

    fun function() = "hello"
}
```
If you want to ensure only one instance exists per class, use the `object` keyword instead of `class`.

```java
public final class Singleton {
   private static final int myPriority = 3;
   public static final Singleton INSTANCE; //(1)

   public final int getMyPriority() {
      return myPriority;
   }

   @NotNull
   public final String function() {
      return "hello";
   }

   private Singleton() { // //(2)
   }

   static {
      Singleton var0 = new Singleton(); //(3)
      INSTANCE = var0;
      myPriority = 3;
   }
}
```

Decompiling the generated bytecode yields the following results:
1. `INSTANCE` property creation
2. Private constructor
3. Eager instantiation of the singleton

## Nothing
`Nothing` is used for functions that never return.
```kotlin
package kotlin

public class Nothing prifvate constructor()
```

A private constructor means it cannot be instantiated outside the class, nor is it instantiated inside the class. Therefore, no instance of `Nothing` exists. The official Kotlin documentation states that `Nothing can be used to represent a value that never exists`.

```kotlin
fun doNothing(): Nothing = throw Exceotion("Nothing at all")
```

The return type must be explicitly specified, and since the method never returns, its return type is `Nothing`.

```kotlin
val x = null
```

When assigning `null` to a variable without a concrete type, the compiler has no other information about `x`, so the inferred type of `x` is `Nothing?`. More importantly, in Kotlin, the `Nothing` class is actually a subtype of all other types.

```kotlin
val x = if (Random.nextBoolean()) "true" else throw Exception("nope")
```
The inferred type of `x` depends on the string if the boolean generated by the `Random.nextBoolean()` function is true. Or it could even be `Any`. This code executes the `else` clause with `Nothing` and the type according to the assigned string, and the final return type becomes something other than `Nothing`.
