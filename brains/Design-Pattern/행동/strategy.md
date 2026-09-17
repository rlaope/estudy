# 전략 패턴

## 전략 패턴 Strategy Pattern
객체들이 할 수 있는 행위 각각에 대해 **전략 클래스를 생성**하고, **유사한 행위들을 캡슐화** 하는 인터페이스를 정의하며, 객체의 **행위를 동적으로 바꾸고 싶은 경우 직접 행위를 수행하지 않고 전략을 바꿔주기만함으로써 행위를 유연하게 확장하는 방법**을 말합니다.  
  
간단히 말해서 객체가 할 수 있는 행위들 각각을 전략으로 만들어 놓고, 동적으로 행위의 수정이 필요한 경우 전략을 바꾸는 것 만으로 행위의 수정이 가능하도록 만든 패턴입니다.  
  
### 전략 패턴 사용 이유

예를 들어, 기차와 버스 클래스가 있고 이 두 클래스는 Movable 인터페이스를 구현했다고 가정하겠다. 그리고 그 객체를 사용하는 Client도 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F9916204B5BF8DAD105)

기차는 선로를 따라 이동하고, 버스는 도로를 따라 이동한다.  
그러다 시간이 흘러 선로를 따라 움직이는 버스가 개발되었다고 가정해보면  
버스의 move 메서드만 수정해주면 된다.  
  
그런데 이렇게 수정하면 SOLID 원칙중 OCP에 위배된다.  
OCP에 의하면 기존의 move()를 수정하지 않으면서 행위가 수정되어야하지만, 지금은 버스의 move() 메서드를 직접 수정했다.  
  
또한 지금과 같은 방식의 변경은 시스템이 확장 되었을 때 유지보수를 어렵게한다.  
예를 들어, 버스와 같이 도로를 따라 움직이는 택시, 자가용, 고속버스, 오토바이 등이 추가된다고 할 때, 모두 버스와 같이 move() 메서드를 사용한다.  
만약에 새로 개발된 선로를 따라 움직이는 버스와 같이, 선로를 따라 움직이는 택시, 자가용, 고속버스..등이 생긴다면, 택시 자가용, 고속버스의 move() 메서드를 일일이 수정해야 할 뿐더러, 같은 메서드를 여러 클래스에서 똑같이 정의하고 있으므로 메서드의 중복이 발생하고 있습니다.  
  
즉, 문제점은
- OCP 위배
- 시스템이 커져 확장이 될 경우 메서드의 중복 문제 발생

따라서 이를 해곃하조 전략 패턴을 사용하려고 합니다.

## 전략 패턴의 구현
이번에는 위와 같이 선로를 따라 이동하는 버스가 개발된 상황에서 시스템이 유연하게 변경되고 확장될 수 있도록 전략패턴을 사용해보도록 하겠습니다.

1. 전략을 생성하는 방법

현재 운송수단은 선로, 도로 이렇게 두가지가 있고 즉 움직이는 방식에 대한 Strategy 클래스를 생성합니다. (RailLoadStrategy, LoadStrategy)  
  
그리고 이 두 클래스는 move() 메서드를 구현하고, 어떤 경로로 움직이는지에 대해 구현합니다.  
  
또한 두 전략 클래스를 캡슐화하기 위해 MovableStrategy 인터페이스를 생성합니다.  
이렇게 캡슐화 하는 이유는 운송수단에 대한 전략 뿐만 아니라,  
다른 전략들이 추가적으로 확장되는 경우를 고려한 설계입니다.  

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99804D3E5BF8E1C80C)

2. 운송 수단에 대한 클래스를 정의

기차와 버스 같은 운송 수단은 move() 메서드를 통해 움직일 수 있습니다.  
그런데 이동 방식을 직접 메서드로 구현하지 않고, 어떻게 움직일 것인지에 대한 전략을 설정해,  
그 전략의 움직임 방식을 사용해 움직이도록 합니다.  
  
그래서 전략을 설정하는 메서드인 setMovableStrategy()가 존재합니다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F997EE1415BF8E4BD13)

3. 이제 Train과 Bus 객체를 사용하는 Client를 구현합니다.  

Train과 Bus객체를 생성한 후 각 운송 수단이 어떤 방식으로 움직이는지 설정하기 위해 setMovableStrategy() 메서드를 호출합니다.  
  
그리고 전략 패턴을 사용하면 프로그램 상으로 로직이 변경되었을 때, 얼마나 유연하게 수정할 수 있는지 살펴보기 위해  
선로를 따라 움직이는 버스가 개발되었다는 상황을 만들어 버스의 이동 방식 전략을 수정합니다.

```java
public class Client {
    public static void main(String args[]){
        Moving train = new Train();
        Moving bus = new Bus();

        /*
            기존의 기차와 버스의 이동 방식
            1) 기차 - 선로
            2) 버스 - 도로
         */
        train.setMovableStrategy(new RailLoadStrategy());
        bus.setMovableStrategy(new LoadStrategy());

        train.move();
        bus.move();

        /*
            선로를 따라 움직이는 버스가 개발
         */
        bus.setMovableStrategy(new RailLoadStrategy());
        bus.move();
    }
}
```