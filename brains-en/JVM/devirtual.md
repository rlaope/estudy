# Devirtualization: Polymorphism Implemented by the JVM

Let's explore devirtualization, a technique that shows how JIT or AOT compilers improve performance and eliminate the overhead of vtable lookups that occur when the JVM implements polymorphism.

To clearly understand this process, we first need to examine what happens in internal memory when the Java Virtual Machine (JVM) calls a method on an object declared with an interface or superclass type.

### Physical Location of the vtable

The most widespread misconception is that every object has its own vtable, which is not true; if every object had a vtable, memory waste would be extreme.

**The vtable is located in the metaspace, a native memory area, not in the Java heap.**

At the C++ level, it exists as a variable-length array at the end of the `InstanceKlass` struct, which represents class metadata, with exactly one per class.

**The connection with objects** is made through the oop (Ordinary Object Pointer) created in the Java heap, which has an 8-byte mark word and a 4-byte or 8-byte Klass Pointer in its object header. This Klass Pointer points to its `InstanceKlass` in Metaspace, allowing access to the vtable.

#### Internal Structure and Creation Principle of vtable

A vtable is essentially a **one-dimensional array of pointers containing the execution memory addresses of methods.**

In C++ source code, it is managed as a `vtableEntry` array.

When a class is loaded, the JVM copies the vtable array according to the following strict rules:

1.  **Inheritance Copy**: A child class copies the parent class's vtable array as is.
2.  **Fixed Index Guarantee**: Methods present in the parent class always have the same index (`vtable_index`) in the child class's vtable. For example, if `Object::hashCode()` is at index 3 in the vtable, then `hashCode()` will also be at index 3 in the vtables of inheriting classes like String and Dog.
3.  **Overriding**: If a child class overrides a parent's method, the pointer at that index is **overwritten with the new method address of the child** instead of the parent's method address.
4.  **Extension**: If a child class adds an entirely new method, it is assigned a new index at the very end of the vtable array.

### `invokevirtual` Instruction Flow

When the bytecode instruction `invokevirtual` is executed, the CPU and JVM internally perform the following pointer chasing operations to resolve polymorphism.

For example, `animal.getLegCount()`: `animal` is of type Animal at compile time, but a Dog object is called at runtime.

1.  **Constant Pool Resolution**: The JIT or Interpreter already determines at compile time which index (e.g., Index 5) `Animal::getLegCount()` uses in the Animal class's vtable. This index remains unchanged for the child class Dog.
2.  **Object Access**: Moves to the heap memory address of the receiver object `animal` (oop).
3.  **Reading Klass**: Reads the Klass Pointer from the object header to obtain the `InstanceKlass` address of the Dog class in Metaspace.
4.  **vtable Access**: Adds the vtable start offset to the `InstanceKlass` memory address to move to the Dog's vtable array memory.
5.  **Index Lookup**: Reads the value stored at the 5th index (vtable[5]), which was determined at compile time. Since Dog has overridden the method, this contains the metadata (`Method*` struct) of `Dog::getLegCount()`, not Animal's.
6.  **Code Execution Jump**: Jumps the CPU's instruction pointer to the memory of the code cache where the actual machine code compiled from the `_verified_entry_point` recorded in that Method struct is located.

### Virtual Method Call and vtable Lookup Process (Before Devirtualization)

All instance methods in Java are fundamentally **virtual methods**.

This means that at compile time (`javac`), it's impossible to know exactly which code will be executed. Instead, the method to be called is determined at runtime by checking the actual type of the object allocated in memory.

This is called **Dynamic Dispatch**.

Inside the JVM (in a C++ runtime environment), a **virtual method table (vtable)** is used to handle this process.

During a method call, a complex pointer dereferencing process occurs:

1.  **Object Header Reference**: Moves to the memory address of the called object (oop) and reads the object header (mark word + klass pointer).
2.  **Klass Metadata Reference**: Follows the Klass Pointer in the object header to access the `InstanceKlass` struct, which contains the metadata for that class, located in metaspace outside the heap.
3.  **vtable Lookup**: Accesses the `vtable` array located within the `InstanceKlass`.
4.  **Method Address Acquisition**: Uses the fixed index assigned to the method to be called to read the memory address of the actual compiled machine code (or interpreter entry point) that needs to be executed.
5.  **Branch (Jump)**: Moves the CPU's instruction pointer to the acquired memory address to execute the method.

This process causes multiple memory reads with each call, reducing the efficiency of the CPU's instruction pipeline and hindering optimizations (such as inlining).

### Devirtualization Application Process

Devirtualization is an optimization technique where the compiler analyzes the code and simplifies the complex 5-step vtable lookup process described above into a **single step of direct call**.

The compiler (C2 or GraalVM with PGO) determines the following through runtime profiling data or Class Hierarchy Analysis (CHA):
- **At this call site, only instances of a specific class (e.g., Dog) are always used.**

If this fact is proven, the compiler generates machine code that directly **jumps to the method address of the Dog class (static call)** instead of generating machine code that reads the object header and searches the vtable. The vtable lookup itself is omitted.

### Java Code and Compiler Transformation Example

To aid understanding, I will explain a simple Java code example and the internal changes in compiler processing.

```java
interface Animal {
    int getLegCount();
}

class Dog implements Animal {
    @Override
    public int getLegCount() {
        return 4;
    }
}

public class Main {
    public int calculateLegs(Animal animal) {
        // Method call with interface type (virtual method call site)
        return animal.getLegCount() * 2;
    }

    public static void main(String[] args) {
        Main m = new Main();
        Animal myDog = new Dog();
        for (int i = 0; i < 10000; i++) {
            m.calculateLegs(myDog);
        }
    }
}
```

**In the initial execution state with the interpreter or C1 compiler**, `animal.getLongCount()` inside the `calculateLegs` method is a virtual method call. Each time the loop runs, the JVM goes to the actual memory pointed to by the `animal` variable, reads the object header, looks up the vtable in the Dog class's metadata, finds the address of `getLegCount`, and executes it.

**What if profiling data accumulates?** While the loop runs thousands of times, the compiler collects profiling data (`MethodData` struct). The collection results record that the `animal` object passed to the `calculateLegs` method is 100% of type `Dog`. This is called a Monomorphic Call.

**The C2 compiler applies Devirtualization (IR transformation)**. When C2 optimizes `calculateLegs`, it removes the vtable lookup logic and internally transforms the code as follows. The conceptual C2 IR transformation state is:

```java
// Conceptual code internally optimized by the compiler
public int calculateLegs(Animal animal) {
    // 1. Type check (Guard insertion)
    if (animal.getClass() != Dog.class) {
        deoptimize(); // If not a Dog, cancel C2 optimization and revert to interpreter
    }

    // 2. Devirtualization (skips vtable lookup and directly calls Dog's method)
    // return Dog::getLegCount(animal) * 2; 

    // 3. (Additional optimization) Inlining applied
    // The implementation of getLegCount(), 'return 4;', is directly inserted at the call site
    return 4 * 2; 
}
```

As a result, the heavy process of searching the vtable is omitted, and the final generated machine code transforms into a very simple and fast code that returns the constant 8 after a type check.

### vtable Limitations from the JIT Compiler's Perspective

As seen in the steps above, to process `invokevirtual`, the CPU must jump through memory at least 3-4 times.

Because pointers must be continuously followed, the probability of CPU L1/L2 cache misses increases.

The most critical point is that inlining is fundamentally blocked because the target code to be executed can only be known by searching memory.

> Inlining is only possible if the compiler can be 100% certain at compile time which code will be executed.

To eliminate such hardware-level inefficiencies, devirtualization performs an extreme optimization by omitting vtable lookups and directly inserting the code.
