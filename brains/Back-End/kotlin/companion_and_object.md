# companion object와 object의 차이점은 무엇일까요?

Kotlin에선 싱글톤에 대한 `object`가 있습니다. 그리고 companion object는 object의 개념의 변형입니다.

특정 클래스에 속하는 싱글톤 객체입니다. companion object는 클래스의 동반자 같은 것 입니다. companion object는 독립적으로 설 수 없습니다.

캥거루 주머니 속의 아기처럼. 특정 클래스의 동반자이기 때문에 외부 클래스 인스턴스를 통해 모든 개인 수준 메서드 및 속성에 액세스할 수 있습니다.

### 예시

여기 케이크 가게의 예시가 있습니다.

새 케이크가 구워질 때 마다 cakeCount를 증가시켜야하고 변수 cakeCount는 모든 케이크 인스턴스에 공유되어야합니다.

```kt
class Cake(var flavour: String) {

    init {
        println("Baked with Love : $flavour cake ")
        incrementCakeCount()
    }

    private fun incrementCakeCount() {
        cakeCount += 1
    }

    companion object {
        var cakeCount = 0
    }
}

fun main() {
    val cake1 = Cake("Chocolate")
    val cake2 = Cake("Vanilla")
    val cake3 = Cake("Butterscotch")

    println(Cake.cakeCount)
}
```

아이디어는 Java의 정적 내부 클래스와 거의 비슷합니다. 그러나 클래스는 여러 번 인스턴스화할 수있는 코틀린 클래스이며 object 키워드는 단일 인스턴스가 있는 실제 객체에 대한 것이고 모든 멤버는 인스턴스 멤버라는 것을 기억하세요