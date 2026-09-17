# Singleton Pattern

Refers to a pattern where only one instance of an object is created.

```java
ublic class Singleton {

    private static Singleton instance = new Singleton();
    
    private Singleton() {
        // 생성자는 외부에서 호출못하게 private 으로 지정해야 한다.
    }

    public static Singleton getInstance() {
        return instance;
    }

    public void say() {
        System.out.println("hi, there");
    }
}
```

## Reasons for Using the Singleton Pattern

### Advantages in terms of memory
By using a fixed memory area through a single `new` operation initially, it prevents memory waste when accessing that object instance later.

### Data Sharing
Another reason is the ease of data sharing among class data.
  
Since a singleton instance is used globally, instances of other classes can access and use it.
  
However, if instances of multiple classes access data in the singleton instance concurrently, concurrency issues can arise, so this point must be considered during design.
  
Additionally, if you want to ensure that only one instance exists from a domain perspective, you can use the Singleton pattern.

## Issues with the Singleton Pattern

### Amount of Code for Implementation
Implementing the Singleton pattern itself requires a significant amount of code. Besides the implementation method introduced earlier, when checking object creation in a static factory method and calling the constructor, the `synchronized` keyword must be used to resolve concurrency issues that can occur in a multi-threading environment.
  
Secondly, testing is difficult.
Since singleton instances share resources, their state must be reset every time for tests to be performed in a truly isolated environment.
  
Otherwise, tests cannot be performed properly because the state is shared globally across the application.
  
Thirdly, in terms of dependencies, the client becomes dependent on the concrete class. Since objects are created directly within the class using the `new` keyword, this violates DIP among SOLID principles, and also has a high probability of violating the OCP principle.
