# RuntimeException 과 Exception
예외를 직접 만들어 보고 어떻게 활용할 수 있는지 알아보자.  
  
다음 예제를 보자.
```java
public class Sample {
    public void sayNick(String nick) {
        if("fool".equals(nick)) {
            return;
        }
        System.out.println("당신의 별명은 "+nick+" 입니다.");
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        sample.sayNick("fool");
        sample.sayNick("genious");
    }
```
sayNick 메소드는 fool이라는 문자열이 입력되면 return 으로 메소드를 종료해 별명이 출력되지 못하도록 하고 있다.  

<br>

### RuntimeException
이제 "fool" 이라는 문자열이 입력되면 단순히 return 으로 종료하지 말고 적극적으로 예외를 발생시켜보자  
  
다음과 같은 FoolException 클래스를 Sample.java 파일에 작성하자.
```java
class FoolException extends RuntimeException{

}
```
그리고 다음과 같이 예제를 변경 해 보자.
```java
class FoolException extends RuntimeException {
}

public class Sample {
    public void sayNick(String nick) {
        if("fool".equals(nick)) {
            throw new FoolException();
        }
        System.out.println("당신의 별명은 "+nick+" 입니다.");
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        sample.sayNick("fool");
        sample.sayNick("genious");
    }
}
```
단순히 return 했던 부분을 `throw new FoolException()`이라는 문장으로 변경하였다.  
  
이제 위 프로그램을 실행하면 "fool" 이라는 입력값으로 sayNick 메소드 실행시 다음과 같은 예외가 발생한다.
```
Exception in thread "main" FoolException
    at Sample.sayNick(Sample.java:7)
    at Sample.main(Sample.java:14)
```
FoolException이 상속받은 클래스는 RuntimeException이다. Exception은 크게 두가지로 구분된다.  
  
1. RuntimeException
2. Exception
   
RuntimeException은 **실행시 발생하는 예외**이고 Exception은 **컴파일시 발생하는 예외**이다. 즉, Exception은 프로그램 작성시 이미 예측가능한 예외를 작성할 때 사용하고 RuntimeException은 발생 할수도 발생 안 할수도 있는 경우에 작성한다.  
  
그래서 Exception 을 Check Exception , RuntimeException을 Unchecked Exception 이라고도 한다.

<br>

### Exception
그렇다면 이번에는 FoolException을 다음과 같이 변경해 보자.
```java
class FoolException extends Exception {

}
```
- RuntimeException을 상속하던 것을 Exception을 상속하도록 변경했다. 이렇게 하면 Sample 클래스에서 컴파일 오류가 발생할 것이다.
- 예측 가능한 Checked Exception 이기 때문에 예외처리를 컴파일러가 **강제** 하기 때문이다.  

  
다음과 같이 변경해여 정상적으로 컴파일이 될 것이다.
```java
class FoolException extends Exception {
}

public class Sample {
    public void sayNick(String nick) {
        try {
            if("fool".equals(nick)) {
                throw new FoolException();
            }
            System.out.println("당신의 별명은 "+nick+" 입니다.");
        }catch(FoolException e) {
            System.err.println("FoolException이 발생했습니다.");
        }
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        sample.sayNick("fool");
        sample.sayNick("genious");
    }
}
```
sayNick 메소드에서 try... catch 문으로 FoolException을 처리했다.  

<br>

### 예외 던지기
- 위 예제를 보면 sayNick 메서드에서 FoolException을 발생시키고 예외처리도 sayNick 메서드에서 했다.
- 이렇게 하지않고 sayNick 을 호출한 곳에서 FoolException을 처리하도록 예외를 위로 던질 수 있는 방법이 있다.

다음 예제를 보자
```java
public class Sample {
    public void sayNick(String nick) throws FoolException {
        try {
            if("fool".equals(nick)) {
                throw new FoolException();
            }
            System.out.println("당신의 별명은 "+nick+" 입니다.");
        }catch(FoolException e) {
            System.err.println("FoolException이 발생했습니다.");
        }
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        sample.sayNick("fool");
        sample.sayNick("genious");
    }
}
```
sayNick 메서드 뒷부분에 throws 라는 구문을 이용하여 FoolException을 위로 보낼 수가 있다. ("예외를 뒤로 미루기" 라고도 한다.)  
  
위와 같이 sayNick 메서드를 변경하면 main 메서드에서 컴파일 에러가 발생할 것이다. throws 구문 때문에 FoolException의 예외를 처리해야 하는 대상이 sayNick 메서드에서 main 메서드로 변경되었기 때문이다.  
  
따라서 컴파일 오류를 해결하려면 다음과 같이 main 메서드를 변경해야한다.
```java
class FoolException extends Exception {
}

public class Sample {
    public void sayNick(String nick) throws FoolException {
        if("fool".equals(nick)) {
            throw new FoolException();
        }
        System.out.println("당신의 별명은 "+nick+" 입니다.");
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        try {
            sample.sayNick("fool");
            sample.sayNick("genious");
        } catch (FoolException e) {
            System.err.println("FoolException이 발생했습니다.");
        }
    }
}
```
- main 메서드에서 sayNick 메서드에 대한 예외처리를 하였다.
- main 과 sayNick 메서드중 어디에서 예외를 처리하는게 더 좋을까? main 과 sayNick 에서 처리하는 것은 둘이 매우 큰 차이를 가진다.  

  
sayNick 메서드에서 예외를 처리하는 경우에는 다음의 두 문장이 모두 수행이된다.  
```java
sample.sayNick("fool");
sample.sayNick("genius");
```
물론 sample.sayNick("fool"); 문장 수행 시에는 FoolException이 발생하겠지만 그 다음 문장인 sample.sayNick("genious"); 역시 수행이 된다.  
  
하지만 main 메소드에서 예외 처리를 한 경우에는 두번 째 문장인 sample.sayNick("genious");가 수행되지 않는다. 왜냐하면 이미 첫번 째 문장에서 예외가 발생하여 catch 문으로 빠져버리기 때문이다.
```java
try {
    sample.sayNick("fool");
    sample.sayNick("genious");  // 이 문장은 수행되지 않는다.
}catch(FoolException e) {
    System.err.println("FoolException이 발생했습니다.");
}
```
이러한 이유로 프로그래밍시 Exception을 처리하는 위치는 대단히 중요하다. 프로그램의 수행여부를 결정하기도 하고 트랜잭션 처리와도 밀접한 관계가 있기 때문이다. 