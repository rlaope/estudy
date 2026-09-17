# 스테이트 패턴

객체가 상태에 따라 행위를 다르게 할 때, 직접 상태를 체크하여 상태에 따른 행위를 호출하는 것이 아니라 **상태를 객체화**하여 필요에 따라 다르게 행동하도록 위임하는 디자인 패턴

- 객체의 특정 상태 = 클래스
- 상태에 따른 행위 = 클래스 내 메서드
- 상태 클래스를 인터페이스로 캡슐화
- 엘리베이터 예

![](https://velog.velcdn.com/images%2Fjinmin2216%2Fpost%2Fd253c3c3-9ed7-4830-9c5d-34f7edb9cd19%2F%E1%84%89%E1%85%A1%E1%86%BC%E1%84%90%E1%85%A2%20%E1%84%91%E1%85%A2%E1%84%90%E1%85%A5%E1%86%AB%20%E1%84%8B%E1%85%B5%E1%84%86%E1%85%B5%E1%84%8C%E1%85%B5.jpeg)

## 사용 이유
엘리베이터 예제를 통해 사용 이유에 대해 설명
  
엘리베이터는 올라가는 상태 up, 내려가는 상태 down, 정지상태 stop이 존재

```java
public class NonElevator {
    public static final String up = "UP";
    public static final String down = "DOWN";
    public static final String stop = "STOP";
    private String curState = "";

    public NonElevator() {
        this.curState = stop;
    }

    public void setState(String state) {
        this.curState = state;
    }

    public void pushUpButton() {
        if (curState.equals(up)) {
            System.out.println("동작 없음");
        } else {
            System.out.println("올라감");
            curState = up;
        }
    }

    public void pushDownButton() {
        if (curState.equals(down)) {
            System.out.println("동작 없음");
        } else {
            System.out.println("내려감");
            curState = down;
        }
    }

    public void pushStopButton() {
        if (curState.equals(stop)) {
            System.out.println("동작 없음");
        } else {
            System.out.println("멈춤");
            curState = stop;
        }
    }
}
```

위와 같은 상황에서 문이 열림, 문이 닫힘과 같이 상태와 상태에 따른 행위들이 추가될수록 필요한 변수와 메서드 및 메서드 내부 조건문들이 추가된다.
  
스테이트 패턴은 상태를 객체화하여 위와 같은 문제를 해결하도록 돕는다.

![](https://velog.velcdn.com/images%2Fjinmin2216%2Fpost%2Fa643a530-1dff-4663-bff2-2ba65cf5f6e2%2F%E1%84%89%E1%85%A1%E1%86%BC%E1%84%90%E1%85%A2%20%E1%84%91%E1%85%A2%E1%84%90%E1%85%A5%E1%86%AB%20%E1%84%8B%E1%85%B5%E1%84%86%E1%85%B5%E1%84%8C%E1%85%B52.png)

## 구현

각 상태 up, down, stop을 클래스를 정의한 후, 인터페이스를 묶는다(캡슐화)
  
엘리베이터는 상태 인터페이스의 메서드를 호출하여 변경된 행위를 수행한다.

```java
public interface ElevatorState {
    public void pushUpButton();
    public void pushDownButton();
    public void pushStopButton();
}
```

엘리베이터의 상태에서 변화하는 행위들에 대한 메서드를 정의

```java
public class UpState implements ElevatorState {
    private static UpState upState;

    private UpState() {}

    public static UpState getInstance() {
        if (upState == null) {
            upState = new UpState();
        }
        return upState;
    }

    @Override
    public void pushUpButton() {
        System.out.println("동작 없음");
    }

    @Override
    public void pushDownButton() {
        System.out.println("내려감");
    }

    @Override
    public void pushStopButton() {
        System.out.println("멈춤");
    }
}

```

```java
public class DownState implements ElevatorState {
    private static DownState downState;

    private DownState() {}

    public static DownState getInstance() {
        if (downState == null) {
            downState = new DownState();
        }
        return downState;
    }

    @Override
    public void pushUpButton() {
        System.out.println("올라감");
    }

    @Override
    public void pushDownButton() {
        System.out.println("동작 없음");
    }

    @Override
    public void pushStopButton() {
        System.out.println("멈춤");
    }
}
```

```java
public class StopState implements ElevatorState {

    private static StopState stopState;

    private StopState() {}

    public static StopState getInstance() {
        if (stopState == null) {
            stopState = new StopState();
        }
        return stopState;
    }

    @Override
    public void pushUpButton() {
        System.out.println("올라감");
    }

    @Override
    public void pushDownButton() {
        System.out.println("내려감");
    }

    @Override
    public void pushStopButton() {
        System.out.println("동작 없음");
    }
}
```

해당하는 상태에서 버튼을 눌렀을 때, 행동이 변화하도록 구현
  
엘리베이터는 수시로 상태가 변화하기 때문에 싱글톤으로 구현했다.(매번 인스턴스를 생성하지 않아 메모리 낭비를 줄일 수 있도록 구현)

### 적용된 엘리베이터의 모습

```java
public class AdaptElevator {
  private ElevatorState elevatorState;

  public AdaptElevator() {
      this.elevatorState = StopState.getInstance();
  }

  public void setElevatorState(ElevatorState state) {
      this.elevatorState = state;
  }

  public void pushUpButton() {
      elevatorState.pushUpButton();
      this.setElevatorState(UpState.getInstance());
  }

  public void pushDownButton() {
      elevatorState.pushDownButton();
      this.setElevatorState(DownState.getInstance());
  }

  public void pushStopButton() {
      elevatorState.pushStopButton();
      this.setElevatorState(StopState.getInstance());
  }
}
```

### 클라이언트의 행동

```java
public class Client {
    public static void main(String[] args) {
        AdaptElevator elevator = new AdaptElevator();

        elevator.pushStopButton();
        elevator.pushDownButton();
        elevator.pushStopButton();
        elevator.pushUpButton();
        elevator.pushStopButton();
        elevator.pushUpButton();
        elevator.pushUpButton();
    }
}

print >>>
동작 없음
내려감
멈춤
올라감
멈춤
올라감
동작 없음
```

## Strategy vs State

### Strategy
상속 대체(즉, 사용자가 쉽게 알고리즘 전략을 바꿀 수 있도록 유연성을 제공)

### State Pattern
조건문 if else switch 등 대체 (즉, 한 객체가 동일한 동작을 상태에 따라 다르게 수행해야 할 경우 사용)
  
또한 무분별한 스테이트 패턴은 클래스의 수의 증가로 인한 단점이 종료