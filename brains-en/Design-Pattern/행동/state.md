# State Pattern

A design pattern where, when an object behaves differently depending on its state, instead of directly checking the state and calling the corresponding behavior, it **objectifies the state** and delegates behavior to act differently as needed.

- Specific state of an object = Class
- Behavior based on state = Method within a class
- Encapsulate state classes with an interface
- Elevator example

![](https://velog.velcdn.com/images%2Fjinmin2216%2Fpost%2Fd253c3c3-9ed7-4830-9c5d-34f7edb9cd19%2F%E1%84%89%E1%85%A1%E1%86%BC%E1%84%90%E1%85%A2%20%E1%84%91%E1%85%A2%E1%84%90%E1%85%A5%E1%86%AB%20%E1%84%8B%E1%85%B5%E1%84%86%E1%85%B5%E1%84%8C%E1%85%B5.jpeg)
State Pattern Image

## Why Use It
Explaining the reason for use through an elevator example
  
An elevator has an 'up' state, a 'down' state, and a 'stop' state.

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

In situations like the above, as more states and behaviors (such as door open, door close) are added, more variables, methods, and conditional statements within methods are required.
  
The State Pattern helps solve such problems by objectifying states.

![](https://velog.velcdn.com/images%2Fjinmin2216%2Fpost%2Fa643a530-1dff-4663-bff2-2ba65cf5f6e2%2F%E1%84%89%E1%85%A1%E1%86%BC%E1%84%90%E1%85%A2%20%E1%84%91%E1%85%A2%E1%84%90%E1%85%A5%E1%86%AB%20%E1%84%8B%E1%85%B5%E1%84%86%E1%85%B5%E1%84%8C%E1%85%B52.png)
State Pattern Image 2

## Implementation

After defining classes for each state (up, down, stop), they are grouped by an interface (encapsulation).
  
The elevator performs changed behaviors by calling methods of the state interface.

```java
public interface ElevatorState {
    public void pushUpButton();
    public void pushDownButton();
    public void pushStopButton();
}
```

Defines methods for behaviors that change with the elevator's state.

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

Implemented so that behavior changes when a button is pressed in the corresponding state.
  
Since the elevator's state changes frequently, it was implemented as a singleton (to reduce memory waste by not creating new instances every time).

### The Elevator with the Pattern Applied

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

### Client Behavior

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
Replaces inheritance (i.e., provides flexibility for users to easily change algorithm strategies).

### State Pattern
Replaces conditional statements like if-else, switch (i.e., used when a single object needs to perform the same operation differently based on its state).
  
Also, indiscriminate use of the State Pattern can lead to the disadvantage of an increased number of classes.
