# Overriding and Overloading

### Overriding
- Redefining a method of an inherited class.
- For method overriding, the return type, method name, number of parameters, and data types of parameters must all be identical.

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
- If you change the object instance creation code here to `A a = new B();`, the `add` method in class B will be called. That is,
- In inheritance, **when a superclass and a subclass have methods with the same name**, the method that is called is determined by the instance.

> The technique by which an instance's method is called is known as `virtual method`.

<br>

### Overloading
- It is the definition of methods that use the same name but differ in the number of parameters, their data types, or their return types.
- Overloading is often seen when creating constructors.

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
- All constructors have the same name, `name`, but differ in the number of parameters.
- When a user calls the `name` method, the Java compiler searches for a `name` method that has the same parameter types, the same number of parameters, and the same return type as the one called.
