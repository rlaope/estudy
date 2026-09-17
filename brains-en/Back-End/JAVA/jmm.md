# JMM (Java Memory Model)

## JMM
The Java Memory Model (JMM) is a core concept for understanding the behavior of Java programs and addressing synchronization issues in a multi-threaded environment.

This model defines how fields, methods, objects, and other elements are represented in memory, and how they are accessed by threads.

The JMM primarily manages two memory areas: `Heap` and `Stack`.

### Heap
The heap is a memory area shared by all threads. The actual data of objects and their instance variables are allocated here. Garbage collection performs the task of removing unnecessary objects from this area.

### Stack

The stack is a memory area that each thread possesses individually. Information about method calls and local variables is stored here. Each time a method is called, a stack frame for that method call is created, and when the method finishes, that stack frame is removed.

> Additionally, the JMM provides concepts such as volatility, synchronization, and atomicity to help handle concurrency issues in multi-threaded environments. Volatility ensures that variable values are shared among threads, synchronization controls concurrent access to critical sections, and atomicity allows single operations to be performed without concurrency problems.

Through these concepts, the JMM ensures the stability and performance of Java programs.
