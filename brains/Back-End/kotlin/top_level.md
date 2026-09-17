# 클래스 외부의 함수(A package-level/ Top-level 함수)

자바와 달리 코틀린에서는 클래스 외부에 함수를 작성할 수 있다.

함수 논리가 클래스의 속성이나 목적과 무관한 경우 사라져야한다.

이것은 코틀린이 독점적으로 객체 지향 언어가 아니며 기능적으로도 사용될 수 있기 때문에 가능한 일이다.

```kt
package com.example

fun hello(){
    println("hello")
}
```

위의 코드를 자바로 변환하면 다음과 같은 형태가 된다.

```java
package com.example;

public final class ExampleKt {
    public static void sayHello() {
        System.out.println("hello");
    }
}
```

위의 코드에서 ExampleKt 클래스는 코틀린 파일의 이름에 따라 자동으로 생성되며, 패키지 레벨 함수는 해당 클래스 내에 정적 메서드로 생성된다.

### 자바로 변환할 때 클래스 이름 지정해주는 방법

```kt
@file:JvmName("MyFunctions")

package com.example

fun hello(){
    println("hello")
}
```


위의 코틀린 코드에서 @file:JvmName("MyFunctions") 어노테이션은 해당 파일에서 생성되는 클래스 이름 MyFunctions로 변경한다는 의미이다.

```java
package com.example;

public final class MyFunctions {
    public static void sayHello() {
        System.out.println("hello");
    }
}
```

즉 `@file:JvmName("$ClassName")`을 사용해 지정해 줄 수 있다.