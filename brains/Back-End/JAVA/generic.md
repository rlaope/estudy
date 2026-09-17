# Java Generic

제네릭은 컴파일 타임에서 타입을 체크함으로써 코드의 안전성을 높여주는 기능이다.

```java
List<T> // 타입 매개변수
List<String> stringList = new ArrayList<>(); // 매개변수화 된 타입
```

`List<String>`으로 선언한 리스트에는 오직 string 타입의 데이터만 들어갈 수 있으며 제네릭을 사용함으로써 컴파일타임에서 리스트에 들어갈 데이터의 타입을 체크하여 안전성을 높일 수 있다.

그리고 제네릭을 사용하게 된다면 타입 캐스팅도 제공이 된다. 그렇기에 `List<String>` 의 data를 get해오게 된다면 타입을 직접 변환하지 않아도 string형의 데이터가 get되는 것이다.

### 변성

`List<Object> objectList = new ArrayList<Integer>();` 이 구문은 가능할까?

정답을 말하자면, 불가능하다. 배열같은 경우에는 `Object[] objArray = new Integer[1];`과 같이 가능한데 이는 제네릭 타입과 그냥 타입과의 차이가 있다. 이를 변성이라고 하는데, 배열을 공변이라고 하고 제네릭타입 같은 경우는 무공변이다.

**변성은 서로 다른 타입간에 어떠한 관계가 있는지 나타내는 것이다.**

- **무공변(\<T>)** 은 타입 B가 타입 A의 하위타입이 아닌 경우 Category\<B>가 Category\<A>와 아무런 관계가 없는 것이다
- **공변(\<? extends T>)** 은 타입 B가 타입 A의 하위타입일 때, Category\<B>가 Category\<A>의 하위 타입인 것이다.
- **반공변\<? super T>** 타입B가 타입A의 하위타입일때, Category\<B>가 Category\<A>의 상위 타입인 것이다.

### 제네릭 메서드

```java
public <T> void printClassName(T t) {
  System.out.println(this.t.getClass().getName());
}
```

이런식으로 함수 인자에 전달될 타입을 정의하고 그에 맞게 유연하게 여러 타입의 클래스가 들어올 수 있도록 한다.

```java
class Category<T extends Noodle> {
  private T t;
}
```

위와 같은 클래스에서 제한된 제네릭 타입을 설정할 수 있으며 위와 같이 설정되면 Category를 생성할 때는 Noodle이나 Noodle의 하위타입만 들어올 수 있게 된다.

- `<?>`: 모든 타입이 들어올 수 있다
- `<? extends Noodle>`: Noodle과 하위타입이 들어올 수 있다.
-  `<? super Noodle`: Noodle과 상위타입이 들어올 수 있다.

Noodle과 하위타입이 들어올 수 있는 성질을 뛸 때(공변)는 꺼내는 것은 가능하지만 저장하는 것은 가능하지 않다.

Noodle과 상위타입이 들어올 수 있을 때는(반공변) 저장하는 것은 가능하지만 꺼내는것은 가능하지 않다.

그렇게에 effective java에서는 producer(생성자) 입장에서는 `<? extends Noodle>`를 사용하고 consumer 소비자 입장에서는  `<? super Noodle`  사용한다.

### 제네릭 타입 소거

타입 매개변수의 경계가 없는 경우에는 Object로, 경계가 있는 경우에는 경계타입으로 파라미터를 변경한다.

`class Category`라면 Object가 경계타입이지만 `class Category<T extends Noodle>` 라면 Noodle이 경계타입이 되는 것이고 타입 안정성을 유지하기 위해 필요한 경우 타입 변환을 추가한다.

제네릭 타입을 상속받은 클래스의 다형성을 유지하기 위해 메서드의 매개변수나 리턴타입을 소거하기 위해서 만들어지는 브리지 메서드를 생성한다.
