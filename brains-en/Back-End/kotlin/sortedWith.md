# sortedWith, Comparator Sorting Lambda Expression

Let's execute the lambda expression code for the sortedWith function of a Kotlin List<T> variable.

```kt
fun main() {
    var list = listOf(2,9,6,1,7,4,3)
    list = list.sortedWith(Comparator<Int> {a, b -> 
        when {
            a > b -> 1
            a < b -> -1
            else -> 0
        }
    })
    println(list) // 1,2,3,4,6,7,9
}
```

The core of using Comparator.
  
Depending on the comparison of the two variables passed to the function, it returns 1, -1, or 0, and the sortedWith function sorts the values in the List accordingly.
  
Therefore, executing the above code on a List<Int> variable consisting of [2,9,6,1,7,4,3] will result in an ascendingly sorted output.

```kt
fun main() {
    var list = listOf(2,9,6,1,7,4,3)
    list = list.sortedWith(Comparator<Int> {a, b -> 
        when {
            a > b -> -1
            a < b -> 1
            else -> 0
        }
    })
    println(list) // 9,7,6,4,3,2,1
}
```

If you reverse the inequality signs, the sorting direction will also be reversed.
  
And strings can also be sorted alphabetically using sortedWith.

```kt
fun main() {
    var list = listOf("나", "다", "가", "라")

    list = list.sortedWith(Comparator<String> { a, b -> 
        when {
            a > b -> 1
            a < b -> -1
            else -> 0
        }
    })

    println(list) // 가 나 다 라
}
```

Example of sorting by length first, then alphabetically.

```kt
fun main() {
    var list = listOf("나가라", "다나카", "가", "라라")

    list = list.sortedWith(Comparator<String> { a, b -> 
        when {
            a.length < b.length -> -1
            a.length == b.length -> when {
                a < b -> -1
                else -> 1
            }
            else -> 1
        }
    })

    println(list) // 가 라라 나가라 다나카
}
```
