# Kotlin Delegation으로 결합도 낮추기

## Delegation
Delegation은 interface의 구현이나 Property의 Accessor의 **구현을 다른 객체에 위임**하도록 해주는 패턴이다.

Delegator -> Delegate 형태로 책임이나 처리를 넘긴다.

Composition + Forwarding 이라고도 할 수 있다.

### Composition
Composition은 **상속 대신 객체를 내부 private 변수로 두어 구성요소로써 동작하게 하는 것**을 의미한다.

### Forwarding
**부모 메서드의 전달**을 의미한다.

<br>

상속과 비슷하게 한 객체의 기능 일부를 다른 객체에 넘겨서 대신 수행하도록 한다.

위임을 활용하면 결합도를 낮출 수 있다.

코틀린에서는 기본적으로 클래스가 final을 붙이기 때문에, 상속을 허용하려면 open을 붙여주어야한다.

하지만 open으로 확장을 열어놓는 것은 위험하다. 갑자기 sum 메서드가 덧셈이 아닌 곱셈이 되어버리는 이상 현상이 발생할 수도 있기 때문이다.

하지만 반대로 Delegation을 한다고 해서 부모 클래스에 영향을 주진 않는다.

또한 상위 클래스의 API에 결함이 있을 때 Delegation은 이를 숨길 수 있다. override를 통해 동작을 변경하는건 가능하지만, 접근 제한자를 더 private하게 바꿀 수 없다.

코틀린은 상속보다 Delegation을 추진한다.

`by` 키워드로 위임시킬 수 있고, 다음과 같이 사용할 수 있다.

```kt
interface BaseInterface {
    fun printMessage()
    fun printTest()
}

class BaseInterfaceImpl(private val x : Int) : BaseInterface {
    override fun printMessage() { print(x) }
    override fun printTest() { println(x) }
}

class KotlinDelegation(b : BaseInterface) : BaseInterface by b {
    override fun printMessage() { print("asdf") }
}

fun main() {
    val a = BaseInterfaceImpl(10)
    KotlinDeligation(a).printTest()			// asdf
    KotlinDeligation(a).printMessage()		// 10

    a.printTest()						  // 10
    a.printMessage()					  // 10 즉 오버라이딩이 부모 클래스에게 영향 X
}
```

여기서 by는 BaseInterface를 KotlinDelegation 안에 private 객체로 저장될 것이고, b로 보내는 모든 메서드를 생성할 것

### 장점
- 객체 크기에 따라 증가시키는 비용이 없다.
- interface를 정의한다.
- 여러 interface를 delegate 할 수 있다.

### 단점
- protected인 메서드나 properties에는 사용할 수 없다.
- 관련 지식 없이는 이해가 어려워진다.

