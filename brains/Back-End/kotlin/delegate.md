# 코틀린 대리자 delegate

클래스 대리자를 통해 상속을 합성으로 대체할 수 있고, 속성 대리자를 통해 어떤 속성의 getter, setter를 다른 클래스에 있는 속성의 getter, setter를대체할 수 있다.

## 대리자를 사용한 합성 구현
다른 클래스의 인스턴스가 포함된 클래스를 만들고, 그 클래스에 연산을 위임하고 싶은 경우 연산을 위임할 메서드가 포함된 인터페이스를 만들고, 클래스에서 해당 인터페이스를 구현한 다음 by 키워드를 사용해 바깥쪽 래퍼 클래스를 만든다.

```kotlin
interface Dialable {
    fun dial(number: String): String
}

class Phone : Dialable {
    override fun dial(number: String): String = "Dialing $number"
}

interface Snappable {
    fun takePictrue(): String
}

class Camera : Snappable {
    override fun takePictrue() = "Taking Picture"
}

class SmartPhone(
        private val phone: Dialable = Phone(),
        private val camera: Snappable = Camera()
) : Dialable by phone, Snappable by camera

```

생성자에서 Phone, Camera를 인스턴스화 하고 모든 public 함수를 Phone, Camera 인스턴스를 위임하도록 `Dialable by phone, Snappeable by camera by` 키워드를 사용했다

```kt
class SmartPhoneTest {

    @Test
    internal fun `dialing delegates to internal phone`() {
        val smartPhone = SmartPhone()
        val dial = smartPhone.dial("111")
        println(dial) // Dialing 111
    }

    @Test
    internal fun `Taking picture delegates to internal camera`() {
        val smartPhone = SmartPhone()
        val message = smartPhone.takePictrue()
        println(message) // Taking Picture
    }
}
```
by 키워드를 통해 위임 받은 함수를 호출할 수 있게 된다. 만약 by 키워드를 사용해 위임하지 않았다면 Dialable, Snappable를 구현하고 있는 구현 클래스 SmartPhone에서 각 상위 클래스의 세부 구현을 진행해야 한다. 이 구현을 by 키워드를 통해 위임했다.

## lazy 대리자 사용하기
어떤 속성이 필요할 때까지 해당 속성의 초기화를 지연시키고 싶은 경우 코틀린 표준 라이브러리의 lazy 대리자를 사용 가능하다.

## 대리자로서 Map 제공하기
여러 값이 들어 있는 맵을 제공해 객체를 초기화 하곳 싶은 경우 코틀린 맵에서 대리자가 되는 데 필요한 getValue, setValue 함수를 구현할 수 있다.

```kt
class Project(val map: MutableMap<String, Any>) {
    val name: String by map
    val priority: Int by map
    val completed: Boolean by map
}

class ProjectTest {

    @Test
    internal fun `use map delegate for project`() {
        val project = Project(
                mutableMapOf(
                        "name" to "Lean Kotlin",
                        "priority" to 5,
                        "completed" to true
                )
        )

        println(project)
        // Project(map={name=Lean Kotlin, priority=5, completed=true})
    }
}
```

Project 생성자는 MutableMap을 인자로 받아서 해당 맵의 키의 해당하는 값으로 Project 클래스의 모든 속성을 초기화한다. 이 코드가 동작하는 이유는 MutableMap에 ReadWriteProperty 대리자가 되는 데 필요한 올바른 시그니처의 setValue, getValue 확장 함수가 있기 때문이다.