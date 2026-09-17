# Understanding Optional

### Optional
- Since Java 8, the `Optional<T>` class has been used to prevent **NullPointerException (NPE)**.
- It's a wrapper class that packages objects of type T, similar to Integer or Double classes.
- It's a Wrapper class that encapsulates values that might be null, helping to prevent NPEs even when referenced. In other words, it allows unexpected NPE exceptions to be easily avoided using provided methods, enabling null-related exceptions to be handled without complex conditional statements.

```java
public final class Optional<T> {
 
  // If non-null, the value; if null, indicates no value is present
  private final T value;
   
  ...
}
```

<br>

## Optional Usage

### Creating Optional Objects
- Objects can be created using the `of()` method or the `ofNullable()` method.
- If a value might be null, it should be created via `ofNullable()` to prevent an NPE.
- In this case, if the specified value is null, an empty Optional object is returned.
> And you can safely retrieve values even if they are null by using the `orElse` or `orElseGet` methods.

```java
// The value of Optional can be present or null.
Optional<String> optional = Optional.ofNullable(getName());
String name = optional.orElse("anonymous"); // If no value, returns "anonymous"
```

### Accessing Optional Objects
- The `get()` method allows access to the Optional object.
- If the value stored in the Optional object is null, a `NoSuchElementException` is thrown.
- Therefore, it's good practice to check if a value is present in the object using the `isPresent()` method before calling `get()`.

```java
Optional<String> opt = Optional.ofNullable("자바 Optional 객체");

if(opt.isPresent()) {

    System.out.println(opt.get());

}
```

### orElse~~()
The following methods allow you to specify an alternative value instead of null.
1. orElse(): Returns the specified value if it exists; otherwise, returns the `value passed` as an argument.
2. orElseGet(): Returns the specified value if it exists; otherwise, returns the `result of the lambda expression` passed as an argument.
3. orElseThrow(): Returns the specified value if it exists; otherwise, `throws the exception` passed as an argument.

<br>

> **Difference between orElse and orElseGet**
> orElse is called whether the value is null or not. It takes a value as a parameter.
> orElseGet: Is called only when the value is null. It takes a Supplier as a parameter.

<br>

**In JpaRepository, the `findById()` method returns an Optional.**
