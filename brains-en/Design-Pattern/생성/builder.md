# Builder Pattern

## Why Use the Builder Pattern
Let's understand why the Builder pattern should be used over constructors or setters, based on the following `User` class implemented with a constructor and setters.

```java
@NoArgsConstructor
@AllArgsConstructor
public class User {

    private String name;
    private int age;
    private int height;
    private int iq;

}
```

### Advantages of the Builder Pattern
1. Can set only necessary data
2. Ensures flexibility
3. Improves readability
4. Minimizes mutability

## 1. Can Set Only Necessary Data

For example, let's assume we need to create a `User` object, but the `age` parameter is not available.
  
If we were using constructors or static methods, we would either have to insert a dummy value for `age` or create a new constructor that doesn't include `age`.

```java
// 1. Method of inserting a dummy value
User user = new User("esperer", 0, 180, 150)

// 2. Method of adding a constructor or static method
@NoArgsConstructor 
@AllArgsConstructor 
public class User { 
    private String name;
    private int age;
    private int height;
    private int iq;

    public User (String name, int height, int iq) {
        this.name = name;
        this.height = height;
        this.iq = iq;
    }
    
    public static User of(String name, int height, int iq) {
        return new User(name, 0, 180, 150);
    }
    
}
```

If this task occurs only once or twice, we might grudgingly do it.
  
However, requirements inevitably change, necessitating repetitive modifications, which quickly leads to wasted time.
  
But with a builder, we can handle this dynamically.

```java
User user = User.builder()
             .name("esperer")
             .height(180)
             .iq(150).build();
```

The advantage of a builder, which allows setting only the necessary data, makes it convenient for creating test objects compared to constructors or static methods, and offers benefits such as reducing unnecessary code.

## Ensures Flexibility
For example, let's say we need to add a new variable `weight` to the `User` class to represent body weight.
  
However, let's assume there's already code that creates objects using a constructor, like this:

```java
// ASIS
User user = new User("esperer", 28, 180, 150)

// TOBE
User user = new User("esperer", 28, 180, 150, 75)
```

Then, we would face a situation where we have to modify existing code due to the newly added variable.
  
If the amount of existing code is extensive, it might be difficult to manage.
  
However, by using the Builder pattern, even if situations like adding new variables arise, it won't affect the existing code.

```java
@Test
public void 1번테스트() {
 
    // 수정 필요함 (ASIS)
    User user = new User("esperer", 28, 180, 150);
    
    // 수정 필요함 (TOBE)
    User user = new User("esperer", 28, 180, 150, 75);

    ...

}

... 

@Test
public void 100번테스트() {

    // 수정 필요함 (ASIS)
    User user = new User("esperer", 28, 180, 150);
    
    // 수정 필요함 (TOBE)
    User user = new User("esperer", 28, 180, 150, 75);

    ...

}
```

If there were 100 instances of code creating `User` objects like the example above, we would have to modify all the logic or take unnecessary steps like adding separate constructors.
  
However, if the code is written based on the Builder pattern, there's no need to modify the existing code.
  
This because the Builder pattern helps to set object values flexibly.

## Improves Readability
Using the Builder pattern can improve readability even with many parameters.
  
When creating objects with constructors, code readability rapidly decreases as the number of parameters increases.

```java
User user = new User("esperer", 28, 180, 150)
```

Looking at code like the above, it's hard to immediately grasp what 28, 180, or 150 mean, and reading the code becomes difficult even with just four or more class variables.
  
However, by applying the Builder pattern as shown below, you can intuitively understand which data is being set to which value, thereby improving readability.

```java
User user = User.builder()
             .name("esperer")
             .age(28)
             .height(180)
             .iq(150).build();
```

## Minimizes Mutability

Many developers commonly use the setter pattern.
  
However, implementing setters unnecessarily opens up the possibility of modification.
  
This makes it difficult to find where values are assigned during maintenance and leads to unnecessary code reading.
  
If values are assigned only at object creation, it's easier to locate the point where an incorrect value was introduced, significantly improving maintainability.
  
Therefore, it's best to minimize the mutability of class variables.
  
The best way to minimize mutability is to ensure immutability by declaring variables as `final`.
  
Declare the `User` class as follows:

```java
@Builder
@RequiredArgsConstructor
public class User {

    private final String name;
    private final int age;
    private final int height;
    private final int iq;

}
```

However, in some cases, variables might not be declared as immutable.
  
In such cases, the same effect can be achieved by not implementing setters, even without `final`.
  
The important thing is not to allow mutability; while enforcing it with `final` is ideal, if `final` cannot be used, simply don't include setters.
```java
@Builder
@AllArgsConstructor
public class User {

    private String name;
    private int age;
    private int height;
    private int iq;

}
```

In most cases of object creation, it's good to apply the Builder pattern.
  
Of course, there are exceptional cases.
1. When delegating object creation to a library
2. When the number of variables is two or less, and there is no possibility of change

For example, if you're creating DTOs from entity or domain objects, manually creating builders can be cumbersome, so you can delegate creation through libraries like MapStruct or Model Mapper.
  
Additionally, if there's little chance of variables increasing and the number of variables is two or less, using a static factory method might be better.
  
Overusing builders can make the code bloated, so **focus on the number of variables and the possibility of change when deciding whether to apply the Builder pattern.**
