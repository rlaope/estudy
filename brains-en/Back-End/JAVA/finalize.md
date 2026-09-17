# What is the role of the Finalize method in Java?

### finalize()

The `finalize()` method is defined in Java's Object class and is used to write code that executes just before an object is reclaimed by garbage collection.

```java
protected void finalize() throws Throwable {
    try {
        // 객체 정리 로직
    } finally {
        super.finalize();
    }
}
```

This method is automatically called by the garbage collector, providing a last opportunity to reclaim resources used by the object.

For example, if an object uses system resources such as network connections or file handles, you can write code in the `finalize()` method to explicitly release these resources.

However, using this method in practice is not recommended. This is because the exact timing of garbage collector execution cannot be predicted, and thus the timing of `finalize()` method invocation also cannot be predicted.

This can lead to resource leaks or negatively impact performance.

Therefore, it is better to explicitly release resources at the necessary time using `try-finally` blocks or the `try-with-resources` statement instead.

> As of Java 9, the `finalize()` method has been deprecated, and alternatives using `Cleaner` and `PhantomReference` were introduced in Java 12.
