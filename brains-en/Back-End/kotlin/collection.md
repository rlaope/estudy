# Kotlin Collections

## Creating a Map from a Collection
When you have a list of keys and want to create a map by associating each key with a generated value, you can use the `associateWith` function by providing a lambda that executes for each key.
```kotlin
@Test
@Test
internal fun associateWith() {
    val keys = 'a'..'f'
    val associate = keys.associate {
        it to it.toString().repeat(5).capitalize()
    }
    println(associate)
}
```

## Returning a Default Value if the Collection is Empty
When processing a collection, if all elements are excluded from selection but you want to return a default response, you can use the `ifEmpty` or `ifBlank` functions to return a default value.
```kotlin
@Test
internal fun ifEmpty() {
    val products = listOf(Product("goods", 1000.0, false))
    val joinToString = products.filter { it.onSale }
            .map { it.name }
            .ifEmpty { listOf("none") }
            .joinToString(separator = ", ")

    println(joinToString) // none

}
```
