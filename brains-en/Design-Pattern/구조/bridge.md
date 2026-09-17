# Bridge Pattern

The **Bridge Pattern** is a structural design pattern that divides a large class or a set of closely related classes into two separate hierarchies (abstraction and implementation), allowing them to be developed independently.

## Structure

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F999CAD3359C4C45D24)
Bridge Pattern Structure

### Abstraction
The top-level class in the functional hierarchy. It holds an instance of the implementation class and calls methods of the implementation part through that instance.

### RefinedAbstraction
A class that extends new parts in the functional hierarchy.

### Implementor
Defines the interface for implementing the functionality of the Abstraction.

### ConcreteImplementor
Implements the actual functionality.

<br>

## Example

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcjZp8c%2Fbtru3M6JjMY%2FMjcbIhdezizqNPt7N001Zk%2Fimg.png)
Bridge Pattern Example

- Color (Implementor)
- Blue, Red (ConcreteImplementor)
- Brush (Abstract)
- MonoLine, HBPencil (RefinedAbstract)

There are Red and Blue classes that implement the Color functionality, and there is an abstract Brush class that handles the actual implementation using this functionality, along with Monoline and HBPencil classes that implement the `draw` method of that abstract class.

### Color

```java
public interface Color {
    String fill();
}
```
```java
public Class Red implements Color {
    @Override
    public String fill() {
        return "Red";
    }
}

public Class Blue implements Color {
    @Override
    public String fill() {
        return "Blue";
    }
}
```


### Brush

```java
public abstract class Brush {
    protected Color color;
 
    protected Brush(Color color) {
        this.color = color;
    }
 
    public abstract String draw();
 
}
```

HBPencil and MonoLine classes that inherit from the abstract Brush class and implement the `draw` method.

```java
public class HBPencil extends Brush {
    public static final String type = "[HB 연필]";
 
    public HBPencil(Color color) {
        super(color);
    }
 
    @Override
    public String draw() {
        return type + " " + color.fill();
    }
}

public class MonoLine extends Brush {
    public static final String type = "[모노라인]";
 
    public MonoLine(Color color) {
        super(color);
    }
 
    @Override
    public String draw() {
        return type + " " + color.fill();
    }
}
```


### Test

```java
import org.assertj.core.api.Assertions;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
 
class BrushTest {
 
    @Test
    @DisplayName("브리지 패턴 테스트")
    void brushColorTest() {
        Brush redBrush = new HBPencil(new Red());
        Assertions.assertThat("[HB 연필] 빨간색".equals(redBrush.draw()));
 
        Brush blueBrush = new MonoLine(new Blue());
        Assertions.assertThat("[모노라인] 파란색".equals(blueBrush.draw()));
    }
}
```

<br>

## Characteristics
The Bridge **Pattern is a structure that redefines composite objects into an abstract hierarchy**. It changes the connection points of constituent classes to abstract classes. This allows each hierarchy to be extended and modified independently.

The Bridge Pattern **distinguishes between classes that handle functionality and abstract classes responsible for implementation**. It is used when independent changes are needed for both implementation and abstraction.

The Bridge **connects separated objects through composition instead of inheritance**. Connecting objects through composition can eliminate dependencies between them.

In the Bridge Pattern, separating functionality and implementation makes extension easier. **Separated hierarchies can be extended independently**.
