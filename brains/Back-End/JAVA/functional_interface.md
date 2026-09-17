# Java 8 Functional Interface


1개의 추상 메서드를 갖는 인터페이스를 말한다.

Java 8 부터 인터페이스는 기본 구현체를 포함한 디폴트 메서드를 포함할 수 있다.

여러 개의 디폴트 메서드가 있더라도 **추상 메서드가 오직 하나**라면 함수형 인터페이스다.

자바의 람다 표현식은 함수형 인터페이스로만 사용이 가능하다.

## Functional Interface

위에서 말했듯 추상 메서드가 오직 하나인 인터페이스가 함수형 인터페이스다.

추상 메서드가 하나라는 것은 default method, static method 는 여러 개가 존재해도 상관이 없다는 뜻이다.

그리고 `@FunctinalInterface` 어노테이션을 사용하는데, 이 어노테이션은 해당 인터페이스가 함수형 인터페이스 조건에 맞는지 검사해준다.

이 어노테이션이 없어도 함수형 인터페이스로 동작하고 사용하는데에는 문제가 없지만, 인터페이스 검증과 유지보수를 위해 붙여주는 것이 좋다.

<br>

### Create Functional Interface

```java
@FunctionalInterface
interface CustomInterface<T> {

    T myCall();

    default void printDefault() {
        System.out.println("default");
    }

    static void printStatic() {
        System.out.println("static");
    }
}
```

위 인터페이스는 함수형 인터페이스다.

default method, static method를 넣어도 문제가 없다.

만약 함수형 인터페이스 형식에 맞지 않는다면 @FunctionalInterface가 에러를 띄워준다.

`Multiple non-overriding abstract methods found in interface com.practice.notepad.CustomFunctionalInterface`

### 실제 사용

```java
CustomInterface<String> customInterface = () -> "custom";

// abstract method
String s = customInterface.myCall();
System.out.println(s);

// default method
customInterface.printDefault();

// static method
CustomFunctionalInterface.printStatic();
```

함수형 인터페이스라서 람다식으로 표현할 수 있다.

String 타입으로 래핑했기 때문에 myCall()메서드는 String 타입을 리턴한다.

default, static method들도 사용할 수 있다.

**result**
```
custom
default
static
```