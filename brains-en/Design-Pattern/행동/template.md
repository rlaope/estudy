# Template Method Pattern

![Template Method Pattern](./image/templete.png)

- When writing code, you often encounter repetitive code for things like logging, exception handling, etc.
- In such cases, the Template Method pattern is one of the patterns used to eliminate `code duplication`.
- The AbstractClass provides a template, and **subclasses inheriting it implement the concrete logic**.
- The abstract class defines the overall skeleton, and some logic is implemented in the inheriting subclasses.
- Duplicate logic is defined in the abstract class, and only the varying business logic is overridden in the inheriting classes.
- Here, duplicate logic can generally be considered the `unchanging part`, and business logic the `changing part`.

<br>

### Example
Let's write a simple example code.
1. Business logic 1 exists
2. Business logic 2 exists
3. Common logic exists to measure the execution time of each business logic

### Before

```java
public class BeforeTemplateMethodApp {

    public static void main(String[] args) {
        logic1();
        logic2();
    }

    private static void logic1() {
        StopWatch stopWatch = new StopWatch();
        stopWatch.start();

        // 비즈니스 로직 시작
        System.out.println("비즈니스 로직 1 실행");
        // 비즈니스 로직 종료

        stopWatch.stop();
        System.out.println("실행 시간 = " + stopWatch.getTotalTimeMillis());
    }

    private static void logic2() {
        StopWatch stopWatch = new StopWatch();
        stopWatch.start();

        // 비즈니스 로직 시작
        System.out.println("비즈니스 로직 2 실행");
        // 비즈니스 로직 종료

        stopWatch.stop();
        System.out.println("실행 시간 = " + stopWatch.getTotalTimeMillis());
    }
}
```

If we implement the requirements simply, common logic exists like this.
In the code above, the only differing part is `business logic execution`, and the rest is all duplicated code.
Let's refactor by applying the Template Method pattern.

### Abstract Class

```java
public abstract class AbstractTemplate {

    public void execute() {
        StopWatch stopWatch = new StopWatch();
        stopWatch.start();

        // 비즈니스 로직 시작
        call();
        // 비즈니스 로직 종료

        stopWatch.stop();
        System.out.println("실행 시간 = " + stopWatch.getTotalTimeMillis());
    }

    protected abstract void call();
}
```
- This is an abstract class that holds common logic.
- Inside the `execute()` method, the business logic part is replaced by a call to the `call()` method.
- The `call()` method is overridden in child classes that inherit this abstract class.

### Sub Class
```java
public class SubClassLogic1 extends AbstractTemplate {

    @Override
    protected void call() {
        System.out.println("비즈니스 로직 1 실행");
    }
}

public class SubClassLogic2 extends AbstractTemplate {

    @Override
    protected void call() {
        System.out.println("비즈니스 로직 2 실행");
    }
}
```
- These are SubClasses that inherit the abstract class.
- These are simple methods that only define business logic.
- If another business logic 3 is needed later, you can define and use SubClassLogic3.

### Application (Client)
```java
public class AfterTemplateMethodApp {

    public static void main(String[] args) {
        AbstractTemplate template1 = new SubClassLogic1();
        template1.execute();

        AbstractTemplate template2 = new SubClassLogic2();
        template2.execute();
    }
}
```
This is the part where it's actually used.
Calling the `execute()` method is the same, but the logic changes depending on which object is created.

<br>

### Pros and Cons
- **Pros**
  - Eliminates duplicate code and allows SubClasses to focus solely on business logic (SRP)
  - If new business logic is added later, existing code does not need to be modified (OCP)
- **Cons**
  - Requires continuous creation of class files
  - Child classes have a dependency relationship due to inheritance solely for pattern implementation, even though they don't actually use the parent class

The `Strategy pattern` is a design pattern that performs a similar role to the Template Method pattern while eliminating the disadvantages of inheritance.
