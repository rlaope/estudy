# 브리지 패턴

**브리지 패턴**은 큰 클래스 또는 밀접하게 관련된 클래스들의 집합을 두 개의 개별 계층구조(추상화 및 구현)로 나눈 후 각각 독립적으로 개발할 수 있도록 하는 구조 디자인 패턴이다.

## 구조

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F999CAD3359C4C45D24)

### Abstraction
기능 계층의 최상위 클래스로 구현 부분에 해당하는 클래스를 인스턴스를 가지고 해당 인스턴스를 통해 구현부분의 메서드를 호출한다.

### RefindAbstraction
기능 계층에서 새로운 부분을 확장한 클래스다.

### Implementor 
Abstraction의 기능을 구현하기 위한 인터페이스 정의다.

### ConcreteImplementor
실제 기능을 구현한다.

<br>

## Example

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcjZp8c%2Fbtru3M6JjMY%2FMjcbIhdezizqNPt7N001Zk%2Fimg.png)

- Color (Implementor)
- Blue, Red (ConcreateImplementor)
- Brush (Abstract)
- MonoLine, HBPencil (RefinedAbstract)

Color라는 기능을 구현하고 있는 Red, Blue가 있고 해당 기능을 사용하여 실제 구현을 담당하는 Brush 추상 클래스와 해당 추상 클래스의 draw 메서드의 구현을 담당하고 있는 Monoline, HBPencil 클래스가 있다.

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


### Blush

```java
public abstract class Brush {
    protected Color color;
 
    protected Brush(Color color) {
        this.color = color;
    }
 
    public abstract String draw();
 
}
```

Brush 추상 클래스를 상속받아 draw 메서드를 구현하고 있는 HBPencil 클래스와 MonoLine 클래스

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

## 특징
브리지 **패턴은 복합 객체를 다시 재정의하여 추상 계층화**된 구조이다. 구성 클래스의 연결 부분을 추상 클래스로 변경한다. 이를 통해 각각의 계층이 독립적으로 확장 및 변경이 가능하다.

브리지 패턴은 **기능을 처리하는 클래스와 구현을 담당하는 추상 클래스로 구별**한다. 구현 뿐 아니라 추상화도 독립적 변경이 필요할 때 브리지 패턴을 사용한다.

브리지는 **상속 대신 구현을 통해 분리된 객체를 연결**한다. 구현을 통해 객체를 연결하면 객체 간 종속 관계를 제거할 수 있다.

브리지 패턴에서 기능과 구현을 분리하여 확장을 보다 쉽게 할 수 있다. **분리된 계층은 독립적으로 확장 가능**하다.