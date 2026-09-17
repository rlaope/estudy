# What is the difference between `companion object` and `object`?

In Kotlin, there is an `object` for singletons. And `companion object` is a variation of the `object` concept.

It is a singleton object that belongs to a specific class. A `companion object` is like a companion to a class. A `companion object` cannot stand alone.

Like a baby in a kangaroo's pouch. Because it is a companion to a specific class, it can access all private-level methods and properties through the outer class instance.

### Example

Here is an example of a cake shop.

Every time a new cake is baked, `cakeCount` must be incremented, and the `cakeCount` variable must be shared across all cake instances.

```kt
class Cake(var flavour: String) {

    init {
        println("Baked with Love : $flavour cake ")
        incrementCakeCount()
    }

    private fun incrementCakeCount() {
        cakeCount += 1
    }

    companion object {
        var cakeCount = 0
    }
}

fun main() {
    val cake1 = Cake("Chocolate")
    val cake2 = Cake("Vanilla")
    val cake3 = Cake("Butterscotch")

    println(Cake.cakeCount)
}
```

The idea is quite similar to Java's static inner classes. However, remember that a class is a Kotlin class that can be instantiated multiple times, while the `object` keyword is for a real object with a single instance, and all its members are instance members.
