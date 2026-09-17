# How to Create a Singleton Class in Kotlin?

Use the `object` keyword. That's all there is to it!

```kt
object ClassName {
}
```

Now, let me quickly explain the gist of what this code means.

Don't take it lightly, as the Singleton pattern is a very important software design pattern, regardless of the language you use.

First, you need to understand its meaning.

A singleton class literally means "Single all the time." It's restricted from being instantiated multiple times, and once an instance of a singleton class is created, it remains the sole instance of that class throughout the entire application/app.

In Java, we follow several steps to create a singleton class:

1. private constructor
2. static method getInstance()
   1. if the class exists -> create one
   2. else -> return existing instance
3. synchronize the getInstance() method to ensure thread safety

This process leads to a lot of boilerplate code. Kotlin narrows down all these steps into a single one, making the developer's job easier.

Now, to understand what happens backstage with the code above, shall we decompile it?

```java
public final class ClassName {
    @NotNull
    public static final ClassName INSTANCE;
    private ClassName() {

    }

    static {
        ClassName var0 = new ClassName()
        INSTANCE = var0
    }
}
```
