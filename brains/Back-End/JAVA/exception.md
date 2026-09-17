# 예외 처리

프로그램을 만들다 보면 수없이 많은 오류가 발생한다. 물론 오류가 발생하는 이유로는 프로그램이 오동작을 하지 않기 하기 위한 자바의 배려이다. 하지만 때로는 이러한 오류를 무시하고 싶을 때도 있고, 오류가 날 때 그에 맞는 적절한 처리를 하고 싶을 때도 있다. 이에 자바는 `try catch` 구문을 이용해 오류를 처리한다.  
  
  <br>

### 예외는 언제 발생할까?

- 존재하지 않는 파일을 열려고 시도해 보자.
```java
BufferedReader br = new BufferedReader(new FileReader("나없는파일"));
br.readLine();
br.close();
```
- 위 코드를 실행하면 다음과 같은 오류가 발생한다.

```
Exception in thread "main" java.io.FileNotFoundException: 나없는파일 (지정된 파일을 찾을 수 없습니다)
    at java.io.FileInputStream.open(Native Method)
    at java.io.FileInputStream.<init>(Unknown Source)
    at java.io.FileInputStream.<init>(Unknown Source)
    at java.io.FileReader.<init>(Unknown Source)
    ...
```
 존재하지 않는 파일을 열려고 시도하면 `FileNotFoundException`라는 이름의 예외가 발생한다.  
   
- 이번에는 0으로 다른 숫자를 나누는 경우를 살펴보자.

```java
int c = 4 / 0;
```
위 코드가 실행되면 다음과 같은 오류가 발생한다.
```
Exception in thread "main" java.lang.ArithmeticException: / by zero
    at Test.main(Test.java:14)
```
4를 0으로 나누면 `ArithmeticException` 예외가 발생한다. 
  
마지막으로 한가지 오류만 더 들어 보자. 다음의 오류는 정말 빈번하게 일어난다.

```java
int[] a = {1, 2, 3};
System.out.println(a[3]);
```
오류의 내용은 다음과 같다.
```
Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException: 3
    at Test.main(Test.java:17)
```
a[3]은 a 배열의 4번째 값이므로 a 배열에서 구할 수 없는 값이다. 그래서  `ArrayIndexOutOfBoundsException` 오류가 발생했다.  
  
자바는 이와같은 예외가 발생하면 프로그램을 중단하고 오류메시지를 보여준다.

<br>

### 예외 처리하기
- 다음은 예외처리를 위한 try catch문의 기본 구조이다

```java
try{
  ...
}catch(예외1){
  ...
} catch(예외 2){

}
```
- try 문안의 수행할 문장들에서 예외가 발생하지 않는다면 catch문 다음의 문장들은 수행이 되지 않는다.
- try 문 안의 문장을 수행하는 도중에 예외가 발생하면 예외에 해당되는 catch문이 수행한다.
- 숫자를 0으로 나누었을 때 발생하는 예외를 처리하려면 다음과 같이 할 수 있다.

```java
int c;
try {
    c = 4 / 0;
} catch(ArithmeticException e) {
    c = -1;  // 예외가 발생하여 이 문장이 수행된다.
}
```
ArithmeticException이 발생하면 c에 -1을 대입하도록 예외를 처리한 것이다.   ArithmeticException e에서 e는 ArithmeticException 클래스의 객체, 즉 오류 객체에 해당한다.  
 이 오류 객체를 통해 해당 예외 클래스의 메서드를 호출할수 있다.
 <br>

### finally
- 프로그램 수행 도중 예외가 발생하면 프로그램이 중지되거나 예외 처리에 의해 catch 구문이 실행된다.
- 하지만 어떤 예외가 발생하더라도 반드시 실행되어야 하는 부분이 있어야 한다면 어떻게 해아할까?  
  
다음의 예제를 보도록 하자.
```JAVA
public class Sample {
    public void shouldBeRun() {
        System.out.println("ok thanks.");
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        int c;
        try {
            c = 4 / 0;
            sample.shouldBeRun();  // 이 코드는 실행되지 않는다.
        } catch (ArithmeticException e) {
            c = -1;
        }
    }
}
```
- finally 구문은 try 문장 수행 중 예외발생 여부에 상관없이 무조건 실행된다.
- 위 코드를 실행하면 `sample.shouldBeRun()` 메소드가 수행되어 "ok.thanks" 문장이 출력될 것이다.