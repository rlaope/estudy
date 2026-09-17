# Kotlin에서 Unit과 Nothing의 차이점

## Unit

**Java의 void와 동일하다.**

함수가 유용한 것을 반환하지 않거나 리턴해줄 값이 아무것도 없으면 암시적으로 Unit을 반환한다고 말합니다.

그리고 이러한 기능은 부작용이 있는 작업을 수행할 수 있습니다. 무언가 기록/인쇄하거나 반환 값 없이 조작을 수행합니다.

```kt
fun printHelloUnit(name: String?): Unit {
    if (name != null)
        println("Hello $name")
    else
        println("Hi there!")
}

//The Unit return type declaration is also optional. The above code is equivalent to:

fun printHello(name: String?) {
    if (name != null)
        println("Hello $name")
    else
        println("Hi there!")
}
```

위의 예에서 반환 유형을 Unit으로 명시적으로 언급하면 **컴파일러에서는 Unit 유형이 중복되므로 명시적 언급을 제거하도록 제안합니다.**

## Nothing

문자 그대로 아무 것도 **생명으로의 복귀가 없으며 게임은 거기서 끝납니다.**를 의미합니다.

즉, 함수가 여기서에서 반환되지 않고 예외가 발생하거나 무한 루프에 들어갈 것입니다.

또한 반환 유형이 Nothing인 함수를 호출한 후에 작성하는 모든 코드는 컴파일러에서 연결할 수 없는 것으로 표시됩니다.

### 결론

- `Unit`은 내가 돌아올 것이지만 당신이 관심을 가질만한 가치는 없습니다. 라고 말합니다.
- `Nothing`은 나는 결코 돌아오지 않을 것입니다. 라고 말하는 것과 같습니다.

따라서 기능을 보다 명확하게 개별적으로 언급하는 데 도움이 됩니다.

```kt
class NothingClass {

    fun returnName(isSuccess: Boolean): String? {
        return if (isSuccess) {
            println("Sara")
            "Sara"
        } else null
    }

    fun reportError(): Nothing {
        println("no name found")
        throw RuntimeException()
        // var i = 1 // unreachable code
    }
// here if you don't specify Nothing explicitly, it shows compile time error
fun iWillAlwaysThrowException() : Nothing =  throwException("Unnecessary Exception")
}

fun main() {
val nothingClass = NothingClass()
val name: String = nothingClass.returnName(true) ?: nothingClass.reportError() // Compiles and the return type is String or Nothing 
val noName: String = nothingClass.returnName(false) ?: nothingClass.reportException()
}
```