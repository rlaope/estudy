# Object Lifecycle (Creation, Usage, Destruction)

An object's lifecycle can be broadly divided into creation, usage, and destruction phases.

### Object Creation
An object's lifecycle begins with its creation using the `new` keyword.

At this point, the constructor is called, and the necessary resources for that object are allocated.

The created object is stored in the heap memory area and can be accessed via a reference variable.

### Object Usage

Once an object is created, you can call its methods or modify its state using its fields. While an object is in use, it is not subject to garbage collection.

### Object Destruction

When an object is no longer needed, there will be no variables referencing it. At that point, it can be removed from memory by the garbage collector.

This is the point when the object's reference count becomes zero. Garbage collection is performed automatically and cannot be directly controlled by the developer.

In Java, although you can define actions to be performed before an object is reclaimed from memory via the `finalize()` method, the exact timing of this method's invocation cannot be precisely predicted, making it unsuitable for reliable resource deallocation.

> Thus, objects go through a lifecycle of creation, usage, and destruction, requiring appropriate resource management and garbage collection handling at each stage. This prevents memory leaks and resource leaks, enabling efficient system operation.
