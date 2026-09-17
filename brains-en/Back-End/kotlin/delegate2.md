# Reducing Coupling with Kotlin Delegation

## Delegation
Delegation is a pattern that allows **delegating the implementation** of an interface or a Property's accessor to another object.

Responsibilities or processing are passed on in the form of Delegator -> Delegate.

It can also be called Composition + Forwarding.

### Composition
Composition means **using an object as an internal private variable instead of inheritance, making it act as a component**.

### Forwarding
It means **passing on parent methods**.

<br>

Similar to inheritance, some functionality of one object is passed to another object to perform it instead.

By utilizing delegation, coupling can be reduced.

In Kotlin, classes are `final` by default, so `open` must be added to allow inheritance.

However, opening up extension with `open` is dangerous. This is because an abnormal phenomenon might occur where a `sum` method suddenly becomes multiplication instead of addition.

Conversely, doing Delegation does not affect the parent class.

Furthermore, when there is a defect in the superclass's API, Delegation can hide it. While it's possible to change behavior through `override`, access modifiers cannot be made more private.

Kotlin promotes Delegation over inheritance.

Delegation can be done with the `by` keyword, and used as follows.

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
    KotlinDelegation(a).printTest()			// asdf
    KotlinDelegation(a).printMessage()		// 10

    a.printTest()						  // 10
    a.printMessage()					  // 10 즉 오버라이딩이 부모 클래스에게 영향 X
}
```

Here, `by` will store `BaseInterface` as a private object inside `KotlinDelegation` and generate all methods that forward to `b`.

### Advantages
- There is no increasing cost depending on object size.
- Defines an interface.
- Can delegate multiple interfaces.

### Disadvantages
- Cannot be used for `protected` methods or properties.
- Becomes difficult to understand without related knowledge.
