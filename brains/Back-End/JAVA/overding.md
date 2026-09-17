# 오버라이딩과 오버로딩 (Overriding, Overloading)

### 오버라이딩
- 상속 받은 클래스의 메서드를 재정의 하는 것
- 메서드 오버라이딩을 하려면 반환형, 메서드 이름, 매개변수 개수, 매개변수 자료형이 반드시 같아야한다

```java
public class OverridingEx{
static int sum = 0;

class A {
  public void add(){
    sum = sum + 1;
  }
}
class B extends A {
  // Overriding
  public void add(){
    sum = sum + 15;
  }
}

public static void main(String[] args){
    A a = new A();
    B b = new B();

    a.add();
    System.out.println(sum); // 1
    b.add();
   System.out.println(sum); // 11
  }
}
```
- 여기서 객체 인스턴스를 생성하는 코드를 `A a = new B();` 로 바꾸면 B 클래스에 있는 add가 호출된다 즉.
- 상속에서 **상위 클래스와 하위 클래스에 같은 이름의 메서드가 존재할 때** 호출되는 메서드는 인스턴스에 따라 결정된다.

> 인스턴스의 메서드가 호출되는 기술을 `가상 메서드` 라고 한다.

<br>

### 오버로딩
- 같은 메서드의 이름을 사용하지만 매개변수의 개수나 자료형, 반환형을 다르게 정의하는 것이다.
- 오버로딩은 주로 생성자를 만들 때 볼 수 있다.

```java
public cass name{
  String firstName;
  String middleName;
  String lastName;

  public name (String fn){
    this.firstName = fn;
  }

  public name (String fn, STring ln){
    this.firstName = fn;
    this.lastName = ln;
  }
  public name (String fn, String mn, String ln){
    this.firstName = fn;
    this.middleName = mn;
    this.lastName = ln;
  }
}
```
- 생성자의 이름은 name으로 모두 동일하지만 매개변수의 개수가 다르다.
- 자바 컴파일러는 사용자가 name 메서드를 호출하면 호출된 name과 같은 매개변수 타입, 같은 매개변수 개수, 같은 반환형을 가지는 name을 찾아 호출한다.