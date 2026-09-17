# JVM Memory Structure

### What is JVM?
- JVM stands for Java Virtual Machine.
- It acts as an intermediary between Java and the operating system, helping Java programs run independently of the OS.
- It automatically manages memory using a garbage collector and operates on a `stack`-based architecture, unlike other register-based hardware.

### Java Program Execution Steps
![Execution Steps](./image/jvm실행.png)

First, a Java source file is converted into bytecode by the Java compiler. Then, the `JVM` reads this bytecode and, after various complex processes, enables the program to run on any operating system.

For example, if a Java source file was created on Linux and you want to run it on Windows, you only need to install a Windows-compatible JVM. This illustrates the characteristic that JVMs are OS-dependent.

<br>

### JVM Memory Structure
- Broadly speaking, the JVM structure can be divided into four main components: Garbage Collector, Execution Engine, Class Loader, and Runtime Data Area.

![Memory Structure](./image/jvm메모리구조.png)

A Java source file becomes a class file in bytecode format through the Java compiler.
Then, as the Class Loader reads this class file, the JVM begins its execution.

#### 1. Class Loader
This module loads class files into the JVM and performs linking and placement. It dynamically loads classes at runtime.

#### 2. Execution Engine
- It reads and executes bytecode instructions placed in the Runtime Data Area within the JVM via the Class Loader.
- When the JVM first emerged, it had the disadvantage of being slow due to its interpreter-based approach, but this was compensated for by the JIT compiler method.
- JIT makes execution faster by converting bytecode into native code like assembly language, but conversion costs are incurred.
- For this reason, the JVM does not execute all code using the JIT compiler method; instead, it uses the interpreter method and switches to the JIT compiler method once certain criteria are met.

#### 3. Garbage Collector
- It searches for and removes unreferenced objects among those created in the Heap memory area.
- It is not precisely known when the GC performs its role.

#### 4. Runtime Data Area
- This is the JVM's memory area where data used when executing Java applications is loaded.
- This area can be broadly divided into Method Area, Heap Area, Stack Area, PC Register, and Native Method Stack.

![Area](./image/runtimearea.png)

1. `Method area`: This is a memory area shared by all threads. The Method Area stores bytecode for classes, interfaces, method fields, static variables, etc.
2. `Heap area`: This area is shared by all threads and is where objects and arrays created with the `new` keyword are allocated. Only classes loaded in the Method Area can be instantiated here, and the Garbage Collector checks for and removes unreferenced memory in this area.
3. `Stack area`: Each time a method is called, a stack frame (space for the method) is created. It stores values used within the method, including parameters, local variables, return values of called methods, and temporary values generated during operations. Finally, when a method finishes execution, its frame is deleted.
![Stack Area](./image/stackArea.png)

4. `PC Register`: This is created when a thread starts, with one existing for each thread. It records which part and which instruction the thread should execute, holding the address of the currently executing JVM instruction.
5. `Native Method stack`: This is the memory area for native code written in languages other than Java.

<br>

> References: [Oracle Java SE Documentation](https://docs.oracle.com/en/java/)
