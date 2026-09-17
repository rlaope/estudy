# Java Cleaner, PhantomReference class

In the past, Java allowed writing code that would execute just before an object was reclaimed by garbage collection, using the `finalize()` method. However, `finalize()` was not recommended for use because its execution timing could not be precisely predicted, leading to its deprecation starting from Java 9.

As an alternative, `Cleaner` and `PhantomReference` were introduced in Java 12.

### Cleaner

The `Cleaner` class is used to register cleanup tasks that need to be performed when an object becomes unreachable and just before it is garbage collected. It provides a way to perform resource cleanup or other operations that must run before the garbage collector reclaims an object. The primary purpose of `Cleaner` is to replace the use of the `finalize()` method, which is not recommended due to its unpredictability and potential performance issues.

```java
import java.lang.ref.Cleaner;

public class MyClass implements AutoCloseable {
    private static final Cleaner cleaner = Cleaner.create();

    private final SomeResource resource;

    public MyClass() {
        resource = new SomeResource();
        cleaner.register(this, new MyCleanerAction(resource));
    }

    @Override
    public void close() {
        // Perform any additional cleanup actions before the object is garbage collected
        // This method is called when the object is explicitly closed by the developer
    }

    private static class MyCleanerAction implements Runnable {
        private final SomeResource resource;

        public MyCleanerAction(SomeResource resource) {
            this.resource = resource;
        }

        @Override
        public void run() {
            // Perform cleanup actions when the object is garbage collected
            resource.cleanup();
        }
    }
}
```

In the example above, when an instance of `MyClass` becomes unreachable, the associated `MyCleanerAction` will execute, performing cleanup operations on the `SomeResource` object.

### PhantomReference

The `PhantomReference` class is one of the four reference classes available in Java, along with `SoftReference`, `WeakReference`, and `FinalReference`. `PhantomReference` is useful for tracking objects that have been enqueued for finalization but have not yet been reclaimed by the garbage collector.

Unlike other reference types, `PhantomReference` does not prevent the referenced object from being garbage collected when it becomes unreachable. Instead, it allows you to be notified when an object is enqueued for finalization and is about to be reclaimed.

```java
import java.lang.ref.PhantomReference;
import java.lang.ref.Reference;
import java.lang.ref.ReferenceQueue;

public class PhantomReferenceExample {
    public static void main(String[] args) {
        Object referent = new Object();
        ReferenceQueue<Object> referenceQueue = new ReferenceQueue<>();
        PhantomReference<Object> phantomReference = new PhantomReference<>(referent, referenceQueue);

        // At this point, referent is still reachable from the main method

        // Explicitly remove the strong reference to the object
        referent = null;

        // Force garbage collection (for demonstration purposes)
        System.gc();

        // The phantomReference is now enqueued in the referenceQueue
        Reference<?> polledReference = referenceQueue.poll();
        if (polledReference == phantomReference) {
            System.out.println("PhantomReference enqueued for finalization");
        }
    }
}
```

In the example above, the `ReferenceQueue` is optional and can be used to receive notifications when an object is enqueued for finalization. If this notification is not needed, a `PhantomReference` can be created without providing a `ReferenceQueue`.

It's worth noting that `PhantomReference` is typically used in advanced scenarios, such as creating custom cleanup mechanisms or implementing object resurrection. For most common use cases, `SoftReference`, `WeakReference`, or `Cleaner` are sufficient.
