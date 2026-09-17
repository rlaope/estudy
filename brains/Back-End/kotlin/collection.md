# 코틀린 컬렉션즈

## 컬렉션에서 맵 만들기
키 리스트가 있을 때 각각의 키와 생성한 값을 연관시켜 맵을 만들고 싶을 경우 associateWith 함수에 각 키에 대해 실행되는 람다를 제공해 사용할 수 있다.
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

## 컬렉션이 빈 경우 기본값 리턴하기
컬렉션을 처리할 때 컬렉션의 모든 요소가 선택에서 제외되지만 기본 응답을 리턴하고 싶은 경우 ifEmpty, ifBlank 함수를 사용해 기본 값을 리턴할 수 있다.
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
