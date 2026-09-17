# Singleton Pattern

### What is the Singleton Pattern?
- It's a design pattern where, when an application starts, a class `allocates memory only once` and creates an instance in that memory for use.

For example, if multiple objects are created for a configuration file like a registry, there's a risk that configuration values might change.
By using the Singleton pattern, which ensures only one instance is created, you can register a single instance in memory and allow multiple threads to share and use that instance concurrently. This can increase efficiency in high-request environments.

> It's important to design singletons with concurrency issues in mind.

<br>

### Advantages of the Singleton Pattern
- It prevents memory waste by acquiring a fixed memory area and using an instance with a single `new` operation.
- Since an instance of a singleton class is global, it's easy for instances of other classes to share data.
- It's used when you want to guarantee that absolutely only one instance exists.
- From the second use onwards, object loading time is reduced, which improves performance.

The Singleton pattern, with these strengths, is often used in situations where common objects, like in a DBCP (DataBaseConnection Pool), need to be created and used multiple times.

### Disadvantages of the Singleton Pattern
- If a singleton instance does too much or shares too much data, it increases coupling between instances of other classes, violating the `Open/Closed Principle`.
- This goes against object-oriented design principles, making modifications difficult and increasing maintenance costs. Furthermore, without proper synchronization in a multi-threaded environment, there's a possibility of two instances being created.

> For these reasons, the Singleton pattern should be avoided unless absolutely necessary.

<br>

## Singleton Pattern Example

```java
public class CarClass {

  private static CarClass car = new CarClass();
  private CarClass(){}

  public static CarClass getInstance() {
    return car;
  }

  // 차 사용 시작
  public static void drive() {
    isUse = true;
    System.out.println("start driving");
  }

  // 차 사용 종료
  public static void parking(){
    isUse = false;
    System.out.println("parking");
  }

  public static boolean isEnableUseCar() {
    return !isUse;
  }
}
```

There's a CarClass object. How can we ensure that only one instance of the CarClass object exists? (If the constructor is private, no one can create an object.)
In that case, declare itself as a member and load it into memory.
**Create a method that allows external access to the `car` declared as a member.**
Doing so prevents the creation and use of CarClass objects outside of the `getInstance` method.

<br>

By using `private static CarClass = new CarClass();`, an object is created only once initially. Subsequently, this object is returned via the `getInstance` method for use, and the implementation ensures that if someone is already using the car object, others cannot use it.
This approach is the `Singleton pattern`.
