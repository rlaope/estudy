# mapOf(), to로 Pair 인스턴스 생성하기

코틀린은 Pair 인스턴스의 리스트로부터 맵을 생성하는 mapOf와 같은 맵 생성을 위한 최상위 함수를 몇가지 제공한다. 코틀린 표준 라이브러리에 있는 mapOf 함수의 시그니처는 다음과 같다.  
  
`fun <K, V> mapOf(vararg pairs: Pair<K, V>: Map<K, V>)`  
  
Pair는 first, second라는 이름의 두 개의 원소를 갖는 데이터 클래스다. Pair 클래스의 시그니처는 다음과 같다.  
  
`data class Pair<out A, out B>: Seriallizable`  
Pair 클래스의 first, second 속성은 A, B의 제네릭 값에 해당한다. 2개의 인자를 받는 생성자를 사용해서 Pair 클래스를 생성할 수 있지만 to 함수를 사용하는 것이 더 일반적이다.

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