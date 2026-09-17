# 코틀린의 함수형 프로그래밍

## 함수형 프로그래밍
함수형 프로그래밍이라는 용어는 **불변성**을 선호하고 순수 함수를 사용하는 경우에 **동시성**을 쉽게 구현할 수 있으며, 반복보다는 변형을 사용하고, 조건문보다는 필터를 사용하는 코딩 스타이을 지창한다.

## 알고리즘에서 fold 사용하기

fold 함수를 사용해 시퀀스나 컬렉션을 하나의 값으로 축약시킨다.
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

fold는 2개의 인자를 받는다, 첫 번째는 누적자의 초기 값이며, 두 번째는 두 개의 인자를 받아 누적자를 위해 새로운 값을 리턴하는 함수이다.

## reduce 함수를 사용해 축약하기

비어있지 않은 컬렉션의 값을 축약하고 싶지만 누적자의 초기값을 설정하고 싶지 않을 경우 reduce를 사용할 수 있다.  
reduce 함수는 fold 함수랑 거의 같은데 사용 목적도 같다. reduce 함수에는 **누적자의 초기값 인자가 없다**는 것이 fold와 가장 큰 차이점이다.

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


