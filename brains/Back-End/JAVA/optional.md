# Optional 알아보기

### Optional
- `Java8`부터 Optinal\<T> 클래스를 사용해 **NullPointException(이하 NPE)** 를 방지할 수 있도록 했다.
- Integer나 Double 클래스처럼 T타입의 객체를 포장해주는 래퍼클래스다.
- null이 올 수 있는 값을 감싸는 Wrapper 클래스로, 참조하더라도 NPE가 발생하지 않도록 도와준다. 즉, 예상치못한 NPE예외를 제공되는 메서드로 간단히 회피할 수 있어 복잡한 조건문 없이도 null값으로 인해 발생하는 예외를 처리할 수 있다.

```java
public final class Optional<T> {
 
  // If non-null, the value; if null, indicates no value is present
  private final T value;
   
  ...
}
```

<br>

## Optional 사용법

### Optional 객체의 생성
- of() 메서드나 ofNullable()메서드를 사용해서 객체를 생성할 수 있다.
- null발생 가능성이 있는 값이면 ofNullable() 을 통해 생성해야 NPE를 발생시키지 않는다.
- 이때 명시된 값이 null이면 비어있는 Optional 객체를 반환한다.
> 그리고 orElse 또는 orElseGet메소드를 이용해 null인 경우에도 안전하게 값을 가져올 수 있다.

```java
// Optional의 value는 값이 있을 수도 있고 null 일 수도 있다.
Optional<String> optional = Optional.ofNullable(getName());
String name = optional.orElse("anonymous"); // 값이 없다면 "anonymous" 를 리턴
```

### Optional 객체에 접근
- `get()` 메서드를 사용하면 Optional 객체에 접근이 가능하다.
- 만약 Optional 객체에 저정된 값이 null 이면 NoSuchElementException예외가 발생한다.
- 따라서 get() 메서드를 호출하기 전에 `isPresent()` 메서드를 사용해 객체에 저장된 값이 있는지를 확인 후 호출하는 것이 좋다.

```java
Optional<String> opt = Optional.ofNullable("자바 Optional 객체");

if(opt.isPresent()) {

    System.out.println(opt.get());

}
```

### orElse~~()
다음과 같은 메서드를 이용하면 null 대신에 대체할 값을 지정할 수 있다.
1. orElse() : 지정된 값이 존재하면 그 값을 반환하고, 없으면 인수로 `전달된 값`을 반환한다.
2. orElseGet() : 지정된 값이 존재하면 그 값을 반환하고, 값이 존재하지 않으면 인수로 전달된 `람다표현식의 결과 값`을 반환한다.
3. orElseThrow() : 지정된 값이 존재하면 그 값을 반환하고, 값이 존재하지 않으면 인수로 전달된 `예외를 발생`시킨다.

<br>

> **orElse , orElseGet의 차이**  
> orElse는 값이 null이든 아니든 호출. 매개변수로 값을 취한다.  
> orElseGet : 값이 null일때만 호출된다. 매개변수로 Suppiler를 취한다.

<br>

**JpaRepository에서는 findById() 메서드를 Optional을 리턴한다**