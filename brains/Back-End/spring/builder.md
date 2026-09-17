# Spring boot @Builder 에 대하여

### Builder Pattern
Builder Pattern은 객체 생성에서 주입하는 것에 대한 방식이다.  
객체를 생성할 때는 두가지 패턴이 존재하는데 생성자 패턴과 빌더 패턴이다.  
생성자 패턴은 우리가 흔하게 사용했던 `Constructor`이다.  

```java
@Getter
@Setter
public class Car{
  
  private String id;
  private String name;

  public Car(String id , String name) {
    this.id = id;
    this.name = name;
  }
}
```

```java
public class CarImpl{

  private String id = "1";
  private String name = "car";

  Car car1 = new Car(id , name);
  Car car2 = new Car(name , id);
}
```

위는 Car객체를 구현한건데 일반 생성자 패턴을 사용하면 코드에서 파라미터에 대한 정확성과 오류를 찾기 어려워진다.  
즉, 다른 사람이 코드를 볼때 어떤 파라미터가 정확하게 전달되었는지 확인하기 힘들다.  
물론 본인또한 확인하고 오류를 찾기 어려울꺼다.  
  
그렇기에 Builder를 사용한다.  
  
빌더를 사용하는 방식에는 Builder class 를 스태틱으로 가져와 사용하는 방식과 Lombok으로 편하게 사용하는 두가지 방식이 존재하는 것 같다.

```java
import lombok.Builder;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class Car {

  private String id;
  private String name;

  @Builder
  public Car(String id, String name) {
    this.id = id;
    this.name = name;
  }
}
```
이렇게 하여 간단하게 롬복을 통해 @Builder 어노테이션을 생성자에 적용해준다.
  
  
```java
public class CarImpl {
  private String id = "1";
  private String name = "car";

  Car car3 = Car.builder()
        .id(id)
        .name(name)
        .build();
}
```  

그 후 이런식으로 생성자 파라미터를 주입시켜준다.  
  
이렇게 하면 각 인자에 대한 파라미터 주입이 명확해진다.