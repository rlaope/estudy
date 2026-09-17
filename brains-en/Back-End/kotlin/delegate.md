# Kotlin Delegates

Class delegates allow replacing inheritance with composition, and property delegates allow replacing a property's getter and setter with those from another class.

## Implementing Composition with Delegates
When you want to create a class that contains an instance of another class and delegate operations to it, you create an interface containing the methods to be delegated, implement that interface in your class, and then use the `by` keyword to create the outer wrapper class.

```kotlin
interface Dialable {
    fun dial(number: String): String
}

class Phone : Dialable {
    override fun dial(number: String): String = "Dialing $number"
}

interface Snappable {
    fun takePictrue(): String
}

class Camera : Snappable {
    override fun takePictrue() = "Taking Picture"
}

class SmartPhone(
        private val phone: Dialable = Phone(),
        private val camera: Snappable = Camera()
) : Dialable by phone, Snappable by camera

```

In the constructor, `Phone` and `Camera` are instantiated, and the `by` keyword is used to delegate all public functions to the `Phone` and `Camera` instances.

```kt
class SmartPhoneTest {

    @Test
    internal fun `dialing delegates to internal phone`() {
        val smartPhone = SmartPhone()
        val dial = smartPhone.dial("111")
        println(dial) // Dialing 111
    }

    @Test
    internal fun `Taking picture delegates to internal camera`() {
        val smartPhone = SmartPhone()
        val message = smartPhone.takePictrue()
        println(message) // Taking Picture
    }
}
```
The `by` keyword allows calling the delegated functions. If delegation were not used with the `by` keyword, the `SmartPhone` implementation class, which implements `Dialable` and `Snappable`, would have to provide the detailed implementation for each superclass. This implementation is delegated via the `by` keyword.

## Using the lazy Delegate
If you want to defer the initialization of a property until it's needed, you can use the `lazy` delegate from the Kotlin standard library.

## Providing a Map as a Delegate
If you want to initialize an object by providing a map containing multiple values, you can implement the `getValue` and `setValue` functions required for a Kotlin map to act as a delegate.

```kt
class Project(val map: MutableMap<String, Any>) {
    val name: String by map
    val priority: Int by map
    val completed: Boolean by map
}

class ProjectTest {

    @Test
    internal fun `use map delegate for project`() {
        val project = Project(
                mutableMapOf(
                        "name" to "Lean Kotlin",
                        "priority" to 5,
                        "completed" to true
                )
        )

        println(project)
        // Project(map={name=Lean Kotlin, priority=5, completed=true})
    }
}
```

The `Project` constructor takes a `MutableMap` as an argument and initializes all properties of the `Project` class with values corresponding to the map's keys. This code works because `MutableMap` has extension functions `setValue` and `getValue` with the correct signatures required to act as a `ReadWriteProperty` delegate.
