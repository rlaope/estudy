# Kotlin Functional Interfaces and SAM Constructors

### Overview

I'm going to organize some notes on the differences between using functions as arguments and using anonymous objects in the lambda section of the "Kotlin in Action" book, as well as other seemingly important concepts.

![](image/sam_0.png)

### Lambda

Lambdas are one way to concisely express functions in functional programming.

They typically omit the process of defining and calling a function, directly expressing only the function's body.

A lambda is an anonymous function that implements a functional interface. In other words, a lambda serves as an implementation for a functional interface, and a functional interface implemented using a lambda expression can be used just like a class that implements a regular interface.

### Anonymous Objects

In Java, historically, event handling for things like clicks required creating and passing an anonymous object as an argument.

```
button.setOnClickListener(new View.OnClickListener() {
     @Override
     public void onClick(View view) {
                
     }
});
```

Kotlin allows passing **lambdas** instead of anonymous objects. (Of course, you can still pass an anonymous `object` like in Java).

```
button.setOnClickListener { view -> ... }

button.setOnClickListener(object : View.OnClickListener {
    override fun onClick(view: View) {
        ...
    }
})
```

The reason such code works is that the `OnClickListener` class has only a single abstract method. Interfaces that have only one abstract method are called **SAM (Single Abstract Method) interfaces or functional interfaces**.

### SAM Constructors

SAM constructors, as the name suggests, are used to create objects that implement SAM interfaces.

```
val listener = OnClickListener { 
    // onClick implementation
}
```

The code above is an implementation of `OnClickListener` written using a SAM constructor.

A SAM constructor is written by appending curly braces after the SAM interface's name and providing the lambda expression as an argument. When written this way, the compiler automatically creates an object that implements the functional interface and implements the lambda expression as the corresponding abstract method of that object.

SAM constructors also work for SAM interfaces with multiple arguments. However, the argument names of the SAM interface's abstract method must match the names used in the lambda expression.

### Anonymous Object Usage vs. Lambda Usage

When using anonymous classes, a new object instance is created and used every time the method is called.

In contrast, lambdas create an object only **once and then reuse it**, making them much more efficient.

### SAM Constructors in Java

SAM constructors are a Kotlin-specific feature, and the concept of SAM constructors does not exist in Java. In Java, when using lambda expressions, you either write a functional interface yourself or use a predefined functional interface provided by Java. In this case, **an anonymous class implementing the functional interface is created, and the lambda expression is passed to it**.

For example, the `Runnable` interface represents a function with no parameters and no return value, and it can be implemented with a lambda expression as follows:

```
Runnable r = () -> System.out.println("Hello, World!");
```

The lambda expression written this way is assigned to the `r` variable.

This means that when a lambda is passed as an argument in Kotlin, an anonymous object is created and passed in Java.

Wait, what? Lambdas create an object only once and reuse it, but in Java, an anonymous object is created?

To address this performance issue in Java, lambda expressions can be passed as parameters to methods that use streams or functional interfaces. In this scenario, the lambda expression performs object creation only once and can be reused, thereby mitigating performance degradation.

Furthermore, since Java 8, interfaces like `Supplier` can be used to replace object creation code with lambda expressions. This approach allows objects to be created only when needed and prevents their creation when not required.

#### Caution

> In Kotlin, `this` within a lambda expression refers not to the lambda itself, but to the object enclosing that lambda. (A lambda written at the top level refers to nothing).
>
> Conversely, a Java lambda written at the top level refers to the anonymous object that implements it.

### Conclusion

To conclude, we've explored anonymous objects, functional interfaces, and SAM constructors. I believe these are truly important concepts, and lambdas are a syntax that has revolutionized the era of development. I wish everyone a joyful programming life.
