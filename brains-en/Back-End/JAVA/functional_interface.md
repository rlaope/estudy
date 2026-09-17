# Java 8 Functional Interface

Refers to an interface that has exactly one abstract method.

Starting from Java 8, interfaces can include default methods, which come with a basic implementation.

Even if there are multiple default methods, an interface is a functional interface if it has **only one abstract method**.

Java's lambda expressions can only be used with functional interfaces.

## Functional Interface

As mentioned above, an interface with only one abstract method is a functional interface.

Having one abstract method means that there can be multiple default methods and static methods without issue.

The `@FunctionalInterface` annotation is used to check if the interface meets the conditions for being a functional interface.

Even without this annotation, it will still function and be used as a functional interface, but it's good practice to include it for interface validation and maintainability.

<br>

### Create Functional Interface

```java
@FunctionalInterface
interface CustomInterface<T> {

    T myCall();

    default void printDefault() {
        System.out.println("default");
    }

    static void printStatic() {
        System.out.println("static");
    }
}
```

The interface above is a functional interface.

Adding default methods and static methods is not an issue.

If it doesn't conform to the functional interface format, `@FunctionalInterface` will raise an error.

`Multiple non-overriding abstract methods found in interface com.practice.notepad.CustomFunctionalInterface`

### Actual Usage

```java
CustomInterface<String> customInterface = () -> "custom";

// abstract method
String s = customInterface.myCall();
System.out.println(s);

// default method
customInterface.printDefault();

// static method
CustomFunctionalInterface.printStatic();
```

Since it's a functional interface, it can be expressed with a lambda expression.

Because it's wrapped with a String type, the `myCall()` method returns a String type.

Default and static methods can also be used.

**result**
```
custom
default
static
```
