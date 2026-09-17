# Abstract Methods and Abstract Classes

### Abstract Methods
- An abstract method refers to a method that must be overridden in a child class to be used.
- The purpose of declaring and using abstract methods in Java is to ensure that child classes inheriting a class containing abstract methods must implement those abstract methods.
- An abstract method only has a declaration; its implementation is not written. (The unimplemented part is overridden and used by child classes.)

```java
abstract 반환타입 메소드이름();
```
A semicolon is added directly at the end of the declaration to indicate that there is no implementation.

<br>

### Abstract Classes
- In Java, a class that contains one or more abstract methods is called an abstract class.
- Such abstract classes allow defining a set of methods that exhibit `polymorphism`, a crucial characteristic in object-oriented programming.
- That is, if a method that must be used is declared as an abstract method in an abstract class, all classes inheriting this class must redefine this abstract method.

```java
abstract class ClassName{
  // ...
  abstract 반환타입 메소드이름();
}
```
Since such abstract classes contain abstract methods whose behavior is not defined, `instance creation is not possible`. An abstract class first requires creating a child class through inheritance, and only after the child class overrides all abstract methods of the abstract class can an instance of the child class be created.

> Except for the fact that an abstract class contains abstract methods, it is identical to a regular class in all other aspects.
> That is, it can also include constructors, fields, and regular methods.

```java
abstract class Animal {
  abstract void cry();
}
class Cat extends Animal{
  void cry() {
    System.out.println("야옹");
  }
}
public class PolymorphismEx02{
  // Animal a = new Animal(); 추상 클래스는 인스턴스 생성 불가능
  Cat c = new Cat();
  
  c.cry(); // 야옹
}
```

<br>

### Purpose of Abstract Methods
- To ensure that child classes inheriting a class containing abstract methods must implement those abstract methods.
- If implemented as a regular method, users might choose to implement that method or not.
