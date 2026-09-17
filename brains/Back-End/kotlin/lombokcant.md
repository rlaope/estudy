# Kotlin에서는 왜 lombok 라이브러리를 사용할 수 없을까?

## 개요

자바로 개발을 할때 Lombok을 사용하지 않은 사람은 거의 없을 것이다. 
코틀린으로 넘어와서는 data클래스때문에 Lombok을 사용하지 않는 줄 알았는데 그냥 아예 못쓰는거라니.. 오늘 한번 정리해보겠다.

## 자바와 코틀린을 같이 사용할 경우
코틀린 컴파일러 -> .class 생성 -> java 컴파일러 -> .class 생성 -> 어노테이션 프로세싱 -> .class 파일 생성

여기서 어노테이션 프로세싱이 일어나는 위치가 코틀린 컴파일러 이후 자바 컴파일을 할 때 일어나기 때문에 롬복을 코틀린에서 사용할 수 없습니다. 

## data class
그래도 코틀린에서는 data class라는 친구가 있기때문에 사실상 롬복을 쓸 필요는 없어보입니다.

데이터 클래스가 제공하는 함수
- equals()/hashCode() pair
- toString() of the form "User(name=John, age=42)"
- componentN() functions corresponding to the properties in their order of declaration.
- copy() function (see below).

```kotlin
data class JeongWoo(
    val name: String,
    val age: Int
)
```