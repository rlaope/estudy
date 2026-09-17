# Creating Pair Instances with mapOf() and to

Kotlin provides several top-level functions for map creation, such as `mapOf`, which creates a map from a list of `Pair` instances. The signature of the `mapOf` function in the Kotlin standard library is as follows:

`fun <K, V> mapOf(vararg pairs: Pair<K, V>: Map<K, V>)`

`Pair` is a data class that has two elements named `first` and `second`. The signature of the `Pair` class is as follows:

`data class Pair<out A, out B>: Seriallizable`
The `first` and `second` properties of the `Pair` class correspond to the generic values A and B. While you can create a `Pair` class using a constructor that takes two arguments, it is more common to use the `to` function.

```kotlin
@Test
internal fun `create map using to function`() {
    val mapOf = mapOf("a" to 1, "b" to 2, "c" to 2)

    then(mapOf).anySatisfy { key, value ->
        then(key).isIn("a", "b", "c")
        then(value).isIn(1, 2)
    }
}
```
