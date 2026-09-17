# 코틀린 객체 지향 프로그래밍
객체 초기화, 사용자 정의 getter, setter, late init, lazy init, 싱글톤 객체 생성, Nothing 클래스 등을 살펴본다.  
  
## const vs val
**컴파일 타임 상수에 const 변경자를 사용**한다. val 키워드는 변수에 한 번 할당되면 변경이 불가능함을 나타내지만, 이러한 할당은 실행 시간에 일어난다. (런타임)  
  
코틀린 val은 값이 변경 불가능한 변수임을 나타낸다. 자바에서는 final 키워드가 같은 목적으로 사용된다. 그렇다면 코틀린에서 const 변경자도 지원하는 이유는? 컴파일 타임 상수는 반드시 객체나, companion object와 같은 선언의 최상위 속성 또는 멤버여야한다. 컴파일 타임 상수는 문자열 또는 기본 타입의 래퍼 클래스이며, getter를 가질 수 없다. 컴파일 타임 상수는 컴파일 시점에 값을 사용할 수 있도록 main 함수를 포함한 모든 함수 바깥쪽에 할당돼야 한다.

```kt
class Task(val name: String, _priority: Int = DEFAULT_PRIORITY) {

    companion object {
        const val MIN_PRIORITY = 1 // (1)
        const val MAX_PRIORITY = 5 // (1)
        const val DEFAULT_PRIORITY = 3 // (1)
    }

    var priority = validPriority(_priority) // (2)
        set(value) {
            field = validPriority(value)
        }

    private fun validPriority(p: Int) = p.coerceIn(MIN_PRIORITY, MAX_PRIORITY) // (3)
}
```

1. 컴파일 타임 상수
2. 사용자 정의 setter를 사용하는 속성
3. private 검증 함수

<br>

## 사용자 정의 getter, setter 생성하기
다른 객체지향 언어처럼 코틀린 클래스는 데이터와 보통 캡슐화로 알려진 데이터를 조작하는 함수로 이뤄진다.  
코틀린은 특이하게 모든 것이 기본적으로 public 이다. 따라서 정보와 연관된 데이터 구조의 세부 구현이 필요하다고 추정되며 이는 데이터 은닉 철학을 침해하는 것처럼 보인다. 코틀린은 이러한 딜레마를 특이한 방법으로 해결한다. 코틀린 클래스에서는 필드는 직접 선언할 수 없다.

```kt
class Task(val name: String) {
    var priority = 3
}
```
Task 클래스는 name, priority라는 두 가지 속성을 정의한다. 속성 하나는 주 생성자 안에 선언된 반면 다른 속성은 클래스의 최상위 멤버로 선언되었다. 이 방식으로 priority 값을 할당할 수 있지만 클래스를 인스턴스화 할 때 priority에 값을 할당할 수 없다는 것이다.

```kotlin
var priority = 3
    set(value) {
        field = value
    }

val isLowPriority
    get() = priority < 3

```
위 처럼 파생 속성을 위한 getter, setter 메서드를 정의할 수 있다.

<br>

## Lazy 기법
```kt
class Customer(val name: String) {
//    val message: List<String> = loadMessage()
    val message: List<String> by lazy { loadMessage() }
    private fun loadMessage(): List<String> {
        return listOf("1", "2", "3")
    }
}

class CustomerTest {

    @Test
    internal fun `none lazy, 객체 생성 시점에 loadMessage를 호출한다`() {
        // val message: List<String> = loadMessage()
        val customer = Customer("yun")
        customer.message
        println(customer)
    }
    
    @Test
    internal fun `lazy, 객체 생성 시점에 loadMessage를 호출하지 않고, 조회 시점까지 lazy하게 간다`() {
        // val message: List<String> = loadMessage()
        val customer = Customer("yun")
        customer.message
        println(customer)
    }
}
```
- `lazy` 키워드를 사용해 원하는 시점에 데이터를 조회할 수 있다.

<br>

## lateinit
생성자에 속성 초기화를 위한 정보가 충분하지 않으면 해당 속성으로 만들고 싶을 경우 속성에 `lateinit` 키워드를 사용할 수 있다.
```kt
@EntityListeners(value = [AuditingEntityListener::class])
@MappedSuperclass
abstract class AuditingEntity : AuditingEntityId() {

    @CreationTimestamp
    @Column(name = "created_at", nullable = false, updatable = false)
    lateinit var createdAt: LocalDateTime
        protected set

    @UpdateTimestamp
    @Column(name = "updated_at", nullable = false)
    lateinit var updatedAt: LocalDateTime
        protected set
}
```
프레임워크를 사용하다 보면 인스턴스가 이미 생성된 후에 결정되는 값들이 있다 이런 경우 `lateinit`을 사용하면 된다. 만약 초기화가 이루어지지 않았다면 Not Null 항목이기 때문에 해당 항목을 조회할 때 예외가 발생한다.

## lateinit vs lazy
lateinit 변경자에는 var 속성 적용에 사용된다. lazy 대리자는 속성에 처음 접근할 때 평가되는 람다를 받는다.

<br>

## 싱글톤 생성하기
싱글톤 디자인 패턴은 특정 클래스의 인스턴스를 오직 하나만 존재하도록 메커니즘을 정의하는 것이다.
1. 클래스의 모든 생성자를 private를 정의
2. 필요하다면 해당 클래스를 인스턴스화 시키고 그 인스턴스 레퍼런스를 리턴하는 정적 팩토리 메서드까지 제공한다.

```kt
object Singleton {
    val myPriority = 3

    fun function() = "hello"
}
```
클래스 하나당 인스턴스를 딱 하나만 존재하게 만들고 싶은 경우 class 대신 object 키워드를 사용한다.

```java
public final class Singleton {
   private static final int myPriority = 3;
   public static final Singleton INSTANCE; //(1)

   public final int getMyPriority() {
      return myPriority;
   }

   @NotNull
   public final String function() {
      return "hello";
   }

   private Singleton() { // //(2)
   }

   static {
      Singleton var0 = new Singleton(); //(3)
      INSTANCE = var0;
      myPriority = 3;
   }
}
```

생성된 바이트코드를 디컴파일하면 다음과 같은 결과가 나온다.
1. INSTANCE 속성 생성
2. private 생성자
3. 싱글톤의 열성적인 인스턴스화

## Nothing
절대 리턴하지 않는 함수에 Nothing을 사용한다.
```kotlin
package kotlin

public class Nothing prifvate constructor()
```

private 생성자는 클래스 밖에서 인스턴스화할 수 없다는 것을 의미하고, 클래스 안쪽에서도 인스턴스화하지 않는다. 따라서 Nothing의 인스턴스는 존재하지 않는다. 코틀린 공식문서에서는 `결코 존재할 수 없는 값을 나타내기 위해 Nothing을 사용할 수 있다`고 명시되어 있다.

```kotlin
fun doNothing(): Nothing = throw Exceotion("Nothing at all")
```

리턴 타입을 반드시 구체적으로 명시해야 하는데 해당 메서드는 결코 리턴하지 않으므로 리턴타입은 Nothing이다.

```kotlin
val x = null
```

구체적인 타입 없이 변수에 널을 할당하는 경우 컴파일러는 x에 대한 다른 정보가 없기 때문에 추론된 x의 타입은 Nothing? 이다. 더 중요한 사실은 코틀린에서 Nothing 클래스는 실제로 다른 모든 타입의 하위 타입이라는 것.

```kotlin
val x = if (Random.nextBoolean()) "true" else throw Exception("nope")
```
x의 추론타입은 Random.nextBoolean() 함수가 생성하는 불리언이 참인 경우 문자열에 따라 달라진다. 또는 심지어 Any 일수도 있다. 이 코드는 else 절은 Nothing과 할당되는 문자열에 따른 타입에 수행하고 최종 리턴타입은 Nothing이 아닌 다른 타입이 된다.