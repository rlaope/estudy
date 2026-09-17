# Process Memory Structure

### Program Execution

Program execution has two important meanings.
1. It means that an executable file, which existed in the file system, is loaded into memory.
2. The program is allocated CPU time and is executing instructions.

When an executable file from the file system is loaded into memory, the entire executable file is not loaded. Only a portion is loaded into memory, and the rest resides in a specific area of the disk called the **swap area**.

### Process Memory Areas

A process's address space consists of Code, Data, Stack, and Heap areas.

We refer to this address space as virtual memory or logical memory.

#### Code Area

**This is the space where the code of program functions written by the user is converted into machine language instructions that the CPU can execute and stored.** It is determined at compile time and cannot be changed midway, so it is Read-Only.

#### Data Area

**This is the space for storing data used by the program, such as global variables or static variables.** Code that references global or static values is made to point to the address in the data area once compilation is complete. Since global variables can be changed, it is Read-Write.

#### Stack Area

**This is a temporary storage space for the return address and data (local variables, parameters, return values) after a called function has finished execution.**

This area is recorded upon function call and disappears once the function's execution is complete. The mechanism follows the LIFO method learned in the stack data structure.

Since the size of the stack area is determined at compile time, it cannot be allocated indefinitely. Therefore, if a recursive function is called repeatedly or a function has too many local variables that exceed memory capacity, a stack overflow occurs.

#### Heap Area

**A memory area that holds dynamic data used by the programmer as needed.**

The heap area is determined at runtime. In Java, objects are created in the heap area and cleaned up by the GC.

<br>

### Kernel Address Space

Since the operating system is also a process, the kernel also has the same address spaces: code, data, and stack areas.

The code area contains code for system calls, interrupt handling, resource management (such as CPU and memory), and code for providing convenient interfaces.

The data area stores data structures for maintaining the state of currently executing processes, CPU utilization (known as PCB - Process Control Block), and data structures for managing hardware resources such as CPU and memory.

The stack area stores the kernel stack for each process. While a process stores its return address during a function call, the kernel stores an address within the kernel. Each process is managed with its own separate stack.

#### Does the kernel not have heap memory?

The kernel is a key component of the Linux operating system and a core **interface** connecting computer hardware and processes. It communicates as effectively as possible between these two managed resources.

The kernel is not equal to the operating system; rather, it is a key component of the operating system. For example, in Linux, the kernel performs functions such as memory management, process management, acting as an interpreter between hardware and processes, or handling system calls and security. In other words, in Linux, **the kernel should be viewed not as a single process, but as an interface connecting computer hardware and processes.**

The kernel does not use heap memory. The kernel provides the necessary functions for dynamic memory allocation in the operating system, and the operating system manages memory pools using the functions provided by the kernel.
