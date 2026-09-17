# 빌더 패턴

## 빌더 패턴 사용 이유
생성자와 수정자로 구현된 다음과 같은 User 클래스를 바탕으로 왜 생성자나 수정자보다 빌더를 써야하는지 이해해보자

```java
@NoArgsConstructor
@AllArgsConstructor
public class User {

    private String name;
    private int age;
    private int height;
    private int iq;

}
```

### 빌더 패턴의 장점
1. 필요한 데이터만 설정 가능
2. 유연성 확보 가능
3. 가독성 향상
4. 변경 가능성 최소화

## 1. 필요한 데이터만 설정 가능

예를 들어 User 객체를 생성해야 하는데 age라는 파라미터가 없다고 가정 한다.
  
생성자나 정적 메서드를 이용하는 경우라면 우리는 age에 더미 값을 넣어주거나 age가 없는 생성자를 새로 만들어주어야 한다.

```java
// 1. 더미값을 넣어주는 방법
User user = new User("esperer", 0, 180, 150)

// 2. 생성자 또는 정적 메소드를 추가하는 방법
@NoArgsConstructor 
@AllArgsConstructor 
public class User { 
    private String name;
    private int age;
    private int height;
    private int iq;

    public User (String name, int height, int iq) {
        this.name = name;
        this.height = height;
        this.iq = iq;
    }
    
    public static User of(String name, int height, int iq) {
        return new User(name, 0, 180, 150);
    }
    
}
```

이러한 작업이 한 두번이면 번거러워도 작업을 해줄 수 있다.
  
하지만 결국 요구사항은 계속 변하게 되어있고, 반복적인 변경을 필요로 하며 곧 시간 낭비로 이어지게 된다.
  
하지만 빌더를 이용하면 동적으로 이를 처리할 수 있다.

```java
User user = User.builder()
             .name("esperer")
             .height(180)
             .iq(150).build();
```

그리고 이렇게 필요한 데이터만 설정할 수 있는 빌더의 장점은 생성자 또는 정적 메서드와 비교하여 테스트용 객체를 생성할 때 용이하며, 불필요한 코드의 양을 줄이는 등의 이점을 안겨준다.

## 유연성 확보 가능
예를 들어 User 클래스에 몸무게를 나타내는 새로운 변수 weight를 추가해야 한다고 하자.
  
하지만 이미 다음과 같이 생성자로 객체를 만든 코드가 있다고 하자.

```java
// ASIS
User user = new User("esperer", 28, 180, 150)

// TOBE
User user = new User("esperer", 28, 180, 150, 75)
```

그러면 우리는 새롭게 추가되는 변수 때문에 기존 코드를 수정해야 하는 상황에 직면하게 된다.
  
기존에 작성된 코드의 양이 방대하다면 감당하기 어려울 수 있다.
  
하지만 빌더 패턴을 이용하면 새로운 변수가 추가되는 등의 상황이 생겨도 기존 코드에 영향을 주지 않을 수 있다.

```java
@Test
public void 1번테스트() {
 
    // 수정 필요함 (ASIS)
    User user = new User("esperer", 28, 180, 150);
    
    // 수정 필요함 (TOBE)
    User user = new User("esperer", 28, 180, 150, 75);

    ...

}

... 

@Test
public void 100번테스트() {

    // 수정 필요함 (ASIS)
    User user = new User("esperer", 28, 180, 150);
    
    // 수정 필요함 (TOBE)
    User user = new User("esperer", 28, 180, 150, 75);

    ...

}
```

만약 위와 같이 User 객체를 생성하는 코드가 100개 있다면 모든 로직을 수정해주거나 생성자를 따로 추가하는 등의 불필요한 조치를 해주어야 할 것이다.
  
하지만 빌더 패턴를 기반으로 코드가 작성되어 있다면 기존의 코드는 수정할 필요가 없다. 
  
왜냐하면 빌더 패턴은 유연하게 객체의 값을 설정할 수 있도록 도와주기 때문이다.

## 가독성 향상
빌더 패턴을 사용하면 매개변수가 많아져도 가독성을 높일 수 있다.
  
생성자로 객체를 생성하는 경우에는 매개변수가 많아질수록 코드의 가독성이 급격하게 떨어진다.

```java
User user = new User("esperer", 28, 180, 150)
```

위와 같은 코드르 보면 28과 180 또는 150이 무엇을 의미하는지 바로 파악이 힘들고, 클래스 변수가 4개 이상만 되어도 코드를 읽기가 힘들어진다.
  
하지만 다음과 같이 빌더 패턴을 적용하면 직관적으로 어떤 데이터에 어떤 값이 설정되는지 쉽게 파악하여 가독성을 높일 수 있다.

```java
User user = User.builder()
             .name("esperer")
             .age(28)
             .height(180)
             .iq(150).build();
```

## 변경 가능성 최소화

많은 개발자들이 수정자 패턴 Setter를 흔히 사용한다.
  
하지만 수정자를 구현한다는 것은 불필요하게 변경 가능성을 열어두는 것.
  
유지보수 시에 값이 할당된 지점을 찾기 힘들게 만들며 불필요한 코드 리딩 등을 유발한다.
  
만약 값을 할당하는 시점이 객체의 생성뿐이라면 객체에 잘못된 값이 들어왔을 때 그 지점을 찾기 쉬우므로 유지보수성이 훨씬 높아질 것이다.
  
그렇기 때문에 클래스 변수는 변경 가능성을 최소화하는 것이 좋다.
  
변경 가능성이 최소화하는 가장 좋은 방법은 변수를 final로 선언함으로써 불변성을 확보하는 것이다. 
  
위의 User 클래스를 다음과 같이 선언한다.

```java
@Builder
@RequiredArgsConstructor
public class User {

    private final String name;
    private final int age;
    private final int height;
    private final int iq;

}
```

하지만 경우에 따라서 불변성으로 변수를 선언하지 않을 수 있다.
  
이러한 경우라면 final 없어도 Setter를 구현하지 않음으로도 동일한 효과를 얻을 수 있다.
  
중요한 것은 변경 가능성을 열어두지 않는 것인데, final로 강제할 수 있다면 가장 바람직하지만 final을 붙일 수 없는 경우라면 Setter를 넣어주지 않으면 된다.
```java
@Builder
@AllArgsConstructor
public class User {

    private String name;
    private int age;
    private int height;
    private int iq;

}
```

객체를 생성하는 대부분의 경우에는 빌더 패턴을 적용하는 것이 좋다. 
  
물론 예외적인 케이스가 있긴 하다.
1. 객체의 생성을 라이브러리로 위임하는 경우
2. 변수의 개수가 2개 이하이며, 변경 가능성이 없는 경우

예를 들어 엔티티 객체나 도메인 객체로부터 dto를 생성하는 경우라면 직접 빌더를 만들고 하는 작업이 번거러우므로 MapStruct나 Model Mapper와 같은 라이브러리를 통해 생성을 위임할 수 있다.
  
또한, 변수가 늘어날 가능성이 거의 없으며, 변수의 개수가 2개 이하인 경우에는 정적 팩토리 메서드를 사용하는 것이 더 좋을 수도 있다.
  
빌더의 남용은 오히려 코드를 비대하게 만들 수 있으므로 **변수의 개수와 변경 가능성 등을 중점적으로 보고 빌더 패턴을 적용할지 판단한다.**