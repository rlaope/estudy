# 5 JVM Method Invocation Instructions

### vtable Internal Structure

Before diving in, let's first understand vtable.

`invokevirtual` is an internal JVM data structure used to handle polymorphism.

When a class is loaded into the JVM memory area (Metaspace), vtable is used to map the actual memory addresses of all virtual methods that the class can execute in an array format.

If a subclass overrides a superclass's method, the value at the index pointing to that method in the subclass's vtable array is overwritten from the superclass's method address to the new method address redefined by the subclass.

When a method is invoked, the JVM identifies the actual class through the object's header and accesses that class's vtable. It then reads the specified index (offset) and immediately jumps to the memory address of the method to be executed, running the code.

### 5 Method Invocation Instructions

#### invokevirtual (Virtual Method Invocation)

Used to invoke a class's ordinary instance method.

This is the most frequent invocation method in Java applications. It checks the type of the actual object created in the runtime heap, not at compile time.

It then searches the object's vtable to find and execute the memory address of the overridden actual method, thereby implementing object-oriented polymorphism.

#### invokespecial (Special Method Invocation)

Used to invoke an object's constructor `<init>`, private methods, or superclass methods using `super`.

This instruction does not apply polymorphism, and the target method to be invoked is fixed precisely at compile time. Since the process of searching the vtable at runtime is omitted, the method is executed quickly and directly without the overhead of `invokevirtual`.

#### invokestatic (Static Method Invocation)

Used to invoke `static` methods declared in a class.

It does not depend on the instance state of a specific object. Similar to `invokespecial`, the target method to be invoked is determined at compile time. Since instance creation or runtime type checking is not required, no search cost is incurred, resulting in very fast execution.

#### invokeinterface (Interface Method Invocation)

Used to invoke a method when the reference type of a variable is declared as an interface, not a class.

Like `invokevirtual`, polymorphism is applied to check the type of the actual implementing object at runtime. However, in Java, classes can only have single inheritance, while interfaces can have multiple implementations.

Due to this, the rules for method placement in memory are not consistent, so `itable` is used instead of `vtable`, making the search structure slightly more complex than `invokevirtual`.

#### invokedynamic

Used to support lambda processing introduced in Java 8 and dynamically typed languages running on the JVM.

It does not determine the call target at compile time. Instead, at runtime when the program executes,

it invokes a bootstrap method like `LambdaMetadataFactory` to dynamically determine the target. Then, it links the call site through method handles. When using lambda expressions, this prevents the creation of meaningless anonymous class files, reducing memory waste and providing execution flexibility.

<br>

### itable

An internal JVM data structure used by instructions to find the memory address of the method in the actual implementation to be executed.

The difference from **vtable** is that since Java classes only allow single inheritance, `vtable` assigns fixed indices (offsets) to allow immediate access to methods like an array.

On the other hand, because interfaces allow **multiple implementations**, it's impossible to know which combination of interfaces each class will implement. Therefore, a fixed common index cannot be assigned across the entire memory, leading to the use of a more flexible `itable` structure.

### Internal Structure and Search Principle

`itable` consists of two parts: an offset table containing offset information for the interfaces implemented by the class, and the actual memory addresses of the methods belonging to each interface.

1.  **Interface Search:** When a method is invoked, the class's `itable` is accessed via the object header. First, it searches sequentially or binary to check if the interface to be invoked exists within the table. This search process makes it slightly slower than vtable's simple array access.
2.  **Method Address Confirmation**: Once a matching interface block is found, the position (offset) of the specific method to be invoked within that block is identified to determine its actual memory address.
3.  **Jump and Execute**: Finally, it jumps to the acquired address and executes the implemented code.

To overcome this search overhead of `itable`, JIT performs inline caching to optimize repeated calls at runtime.
