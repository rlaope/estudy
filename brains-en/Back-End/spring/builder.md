# About Spring Boot @Builder

### Builder Pattern
The Builder Pattern is a way to inject values during object creation.
When creating objects, there are two patterns: the constructor pattern and the builder pattern.
The constructor pattern is the `Constructor` we commonly use.

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

The above implements a Car object, but using the regular constructor pattern makes it difficult to ensure parameter accuracy and find errors in the code.
In other words, it's hard for others to see which parameters were passed correctly.
Of course, it would also be difficult for you to verify and find errors.
  
That's why we use a Builder.
  
It seems there are two ways to use a builder: by using a static Builder class, or by using Lombok for convenience.

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
This way, you can easily apply the `@Builder` annotation to the constructor via Lombok.
  
  
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

After that, you inject the constructor parameters in this manner.
  
This clarifies the parameter injection for each argument.
