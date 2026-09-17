# JVM Memory Model

The JVM memory model defines how memory is allocated and managed during the execution of a Java program.

### Method Area

This area is created when the JVM starts and stores the runtime constant pool, field and method data, and constructor code for each class and interface.

### Heap Area
All object instances and arrays are allocated in this area. This area is shared by multiple JVM threads, and it is where the garbage collector operates.

### Stack Area
For each thread, the JVM creates a separate runtime stack in the Stack Area, which stores information about method calls and local variables. A stack frame is created for each method call, and within that frame, local variables, operand stacks, and information about method calls and returns are stored.

### PC Register
The PC Register stores the address of the currently executing JVM instruction. This register is created separately for each thread.

### Native Method Stacks
Native Method Stacks are used by the JVM to handle native methods.

> Native Method: A feature that calls code written in native languages such as C, C++.

These areas interact with each other during the execution of a Java program, and the JVM's role is to manage the execution of Java programs through this memory model. This memory model enables Java's characteristics such as platform independence, memory management, and garbage collection.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F992AAE475B319A3120)
