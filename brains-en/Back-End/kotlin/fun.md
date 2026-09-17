# Functional Programming in Kotlin

## Functional Programming
The term functional programming favors **immutability** and makes **concurrency** easy to implement when using pure functions, uses transformations rather than iterations, and advocates a coding style that uses filters rather than conditional statements.

## Using fold in Algorithms

The fold function is used to reduce a sequence or collection to a single value.
```kt
internal class Fold {

    @Test
    internal fun name() {
        val numbers = intArrayOf(1, 2, 3, 4)
        val sum = sum(*numbers)
        println(sum) // 10
    }

    fun sum(vararg nums: Int) =
            nums.fold(0) { acc, n -> acc + n }
}
```

fold takes two arguments: the first is the initial value of the accumulator, and the second is a function that takes two arguments and returns a new value for the accumulator.

## Reducing with the reduce function

If you want to reduce the values of a non-empty collection but don't want to set an initial value for the accumulator, you can use reduce. The reduce function is almost identical to the fold function and serves the same purpose. The biggest difference from fold is that the reduce function **does not have an initial accumulator value argument**.

```kotlin
@Test
    internal fun `reduce sum`() {
        val numbers = intArrayOf(1, 2, 3, 4)
        val sum = sumReduce(*numbers)
        // acc: 1, i: 2
        //   acc: 3, i: 3
        //   acc: 6, i: 4
    }


    fun sumReduce(vararg nums: Int) =
        nums.reduce { acc, i ->
        println("acc: $acc, i: $i")
        acc + i
    }
```
