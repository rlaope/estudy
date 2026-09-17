# sortedWith, Comparator 정렬 람다식

코틀린 List<T> 변수의 sortedWith 함수의 람다식 코드를 실행해보자.

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

Comparator 사용에 대한 핵심.
  
함수에 전달되는 두 변수의 비교에 따라 1, -1, 0을 반환하게 되고 sortedWith 함수에는 그에 맞춰서 List에 들어있는 값들을 정렬시켜준다.
  
따라서 [2,9,6,1,7,4,3]로 이루어진 List<Int> 변수에 위 코드를 수행하면 오름차순으로 정렬된 결과가 나오게 된다.

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

부등호를 반대로 하면 정렬 또한 반대 방향이다.
  
그리고 문자열(String) 또한 sortedWith을 사용하여 사전 순으로 정렬이 가능하다.

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

길이순으로 먼저 정렬하고 그 후 사전순으로 정렬하는 예제

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