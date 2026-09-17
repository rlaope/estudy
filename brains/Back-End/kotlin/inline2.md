# inline fun 함수를 사용해 고차 함수 최적화

### 함수의 매개 변수로 람다식을 받을 경우의 문제

일반적으로 함수를 호출하면 해당 함수가 서브루틴으로써 실행된다.

반면 inline fun으로 선언된 함수를 호출하면 함수 호출을 실행하는 것이 아니라, 해당 함수가 호출된 위치에 **함수 내부의 코드가 삽입되어 실행된다.**

```kotlin
fun main(args: Array<String>) {
  printWorldAfterFunction {
    println("Hello")
  }
}

fun printWorldAfterFunction(function: () -> Unit) {
  function()
  println("World")
}
```

위의 코드에서 printlnWorldAfterFunction 함수를 `() -> Unit` 타입의 람다식과 함게 실행하면 () -> Unit 타입의 람다식은 익명 클래스로 만들어져 인스턴스화 된다. 이 코드를 바이트코드로 변환 후 자바 코드로 디컴파일 하면 다음과 같은 코드로 나온다.

```java
public final class InlineFunKt {
   public static final void main(@NotNull String[] args) {
      Intrinsics.checkNotNullParameter(args, "args");
      printWorldAfterFunction((Function0)null.INSTANCE);
   }

   public static final void printWorldAfterFunction(@NotNull Function0 function) {
      Intrinsics.checkNotNullParameter(function, "function");
      function.invoke();
      String var1 = "World";
      System.out.println(var1);
   }
}
```

우리가 입력한 function 파라미터는 Function0 를 구현하는 익명 클래스로 만들어지며, 이 클래스의 인스턴스가 `printWorldAfterFunction`에 전달되는 것을 확인할 수 있다.

> 여기서 null.INSTANCE로 표기되지만 실제로는 Function0는 Invoke 호출 시 println("Hello")를 실행하는 객체다.

**즉, 함수의 인자로 람다를 사용하면 새로운 클래스의 인스턴스가 만들어진다.**

매번 함수가 호출될 때마다 익명 클래스의 새로운 인스턴스가 메모리 상에 올라가는 것은 성능에 좋지 않다.

<br>

### inline fun을 사용한 성능 최적화

위와 같은 문제를 해결하기 위해 inline fun이 등장하고 `printWorldAfterFunction`을 inline fun으로 선언해보자.

```kotlin
fun main(args: Array<String>) {
  printWorldAfterFunction {
    println("Hello")
  }
}

inline fun printWorldAfterFunction(function: () -> Unit) {
  function()
  println("World")
}
```

다시 바이트코드로 변환하고 자바로 디컴파일 하면 다음과 같은 결과가 나온다.

```java
public final class InlineFunKt {
   public static final void main(@NotNull String[] args) {
      Intrinsics.checkNotNullParameter(args, "args");
      int $i$f$printWorldAfterFunction = false;
      int var2 = false;
      String var3 = "Hello"; 
      System.out.println(var3); // 그대로 코드로 대입
      String var4 = "World";
      System.out.println(var4); // 그대로 코드로 대입
   }

   public static final void printWorldAfterFunction(@NotNull Function0 function) {
      int $i$f$printWorldAfterFunction = 0;
      Intrinsics.checkNotNullParameter(function, "function");
      function.invoke();
      String var2 = "World";
      System.out.println(var2);
   }
}
```

이 코드에서 main함수는 더이상 printWorldAfterFunction 함수를 호출하지 않는 것을 볼 수 있다. 또한 더이상 Function0을 구현하는 익명 클래스와 그 인스턴스가 만들어지지 않는다. 대신에 printWolrdAfterFunction의 코드와 우리가 function이라는 파라미터로 넘겨준 람다식이 그대로 코드로 입력된 것을 볼 수 있다.

**이렇게하면, 코드는 약간 길어지지만 익명 객체의 인스턴스를 새로 생성하지 않아 성능 최적화가 가능해진다.**

참고로, 람다식을 인자로 받지 않는 함수를 inline fun으로 선언하게 되어도 성능에 별다른 차이가 없다.

```kotlin
inline fun printString(string: String) {
	println(string)
}
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbcbkuR%2FbtsGa0lJ2hv%2Fc1GVYcw4Ksd8XEtePRZz0k%2Fimg.png)

그 이유는.. 이 글을 잘 읽었다면 이해할거라고 믿는다.