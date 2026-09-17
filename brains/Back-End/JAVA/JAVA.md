# Java

## Java 란?
자바는 1995년 썬 마이크로시스템즈에서 발표한 `객체 지향 프로그래밍 언어`이다.  
자바는 가능한 적은 종속성을 갖도록 설계되었으며 `Programmers write once, run any where(WORA)` 와 같이 작성한 코드를 모든 플랫폼에서 작동 시킬 수 잇는 범용적인 언어다.  
  
  전 세계의 많은 Back-End 개발자가 선택하는 언어이며 전 세계적으로 보고된 개발자는 9백만명이다. 또한 Android 앱 개발을 위한 요일한 공식 언어다.  
    
자바는 Amazon , Twitter, Netflix 등 많은 서비스에서 사용하고 있으며 겡미 콘솔, 슈퍼컴퓨터 등 많은 곳에서 실행이 가능하다.
  
대한민국 전자정부표준 프레임워크는 Java 프레임 웤읜 Spring 이다.

<br>

## 코드 예제
```java
class HelloJava{
  public static void main(String[] args){
    Sysem.out.println("hello java");
  }
}
```
- 클래스 이름 : 클래스명 이면 저장 파일 이름과 같다.
- 메인 메소드 : 자바 애플리케이션이 실행되면 가장먼저 실행되는 부분이다.
- 본문 : 자바 문법에 맞게 코드를 작성한다.

<br>

## 자바 동작 원리

위 코드처럼 hello java 를 작성후 저장을 해보겠습니다.  
HelloJava.java 라는 위 파일이 만들어집니다.  
```
javac HelloJava.java
```
컴파일러를 통해 컴파일 합니다.

```java
> java HelloJava
hello java
```
java HelloJava 명령어를 이용해 실행을 한 결과 잘 실행되는 것을 확인하실 수 있습니다.  
  
- 자바는 코드를 저장하면 `OOO.java` 라는 파일이 생기고 그 파일을 컴파일 하면 `javac(컴파일러)`를 통해 `OOO.class` 라는 파일을 만들고 컴파일러가 바이트코드로 바꿔줍니다.
- 하지만 바이트코드는 아직 컴퓨터가 바로 해석을 할 수 없습니다. `JVM(자바 가상 머신)`이 여기서 내부적인 처리를 통해서 컴퓨터가 해석할 수 있는 바이너리 코드로 바꿔줍니다.

<br>

## 자바의 원칙
1. 단순하고 객체 지향 적이고 친숙해야 합니다. It must be simple, object-oriented, and familiar
2. 견고하고 안전해야 합니다. It must be robust and secure
3. 아키텍쳐 중립적이고 이식이 가능해야합니다. It must be architecture-neutral and portable.
4. 고성능으로 실행해야 합니다. It must execute with high performance.
5. 인터프리터해야하고, 스레드와, 동적이여야합니다. It must be interpreted, threaded, and dynamic