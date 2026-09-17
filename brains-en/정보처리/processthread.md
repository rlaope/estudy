# Process and Thread

### Program
- Dictionary definition
  - `A file that can be executed for a certain task`


### Process
- Dictionary definition
  - `A computer program continuously running on a computer`
  - An **instance of a program running** in memory (an independent entity)
  - A unit of work that is allocated system resources by the operating system
  - In other words, in a dynamic sense, it refers to an executed program.
- Examples of allocated system resources
  - CPU time
  - Address space required for operation
  - An independent memory region structured as Code, Data, Stack, Heap
- Characteristics
  - Each process is allocated its own independent memory region (structured as `Code`, `Data`, `Stack`, `Heap`).
  - By default, each process has at least one thread (the main thread).
  - Each process runs in a separate address space, and one process cannot access the variables or data structures of another process.
  - If one process needs to access resources of another process, inter-process communication (IPC) must be used.
  > e.g.) Communication methods using pipes, files, sockets, etc.


<br><br>

### Thread
- Dictionary definition
  - `A unit of multiple execution flows running within a process`
  - **A specific execution path of a process**
  - An execution unit that uses the resources allocated to a process
- Characteristics
  - Within a process, threads are each allocated their own Stack, while sharing the Code, Data, and Heap regions.
  - Threads are multiple flows of execution operating within a single process, sharing the address space and resources within that process among themselves.
  - Each thread has its own separate registers and stack, but they can read and write to the heap memory.
  - If one thread modifies a process resource, other neighboring threads can immediately see the result of that modification.

<br>

### Java Thread
- There is almost no difference from a general thread, and the `JVM` acts as the operating system.
- In Java, processes do not exist; only threads do, and a Java thread is a block of executable code scheduled by the JVM.
- In Java, thread scheduling is entirely performed by the JVM.
- The JVM also manages much information related to threads, such as:
  - How many threads exist
  - Where the memory location of the program code executed by the thread is
  - What the state of the thread is
  - What the thread's priority is.
- In other words, developers only need to write thread code to be run as a Java thread and request the JVM to start executing that thread code.

<br>

## Difference Between Multi-Process and Multi-Thread

### Multi-Process
- What is multi-processing?
  - It is the act of configuring a single application into multiple processes, with each process handling one task.
- Advantages
  - If a problem occurs in one of several child processes, the impact does not spread beyond the termination of that child process.
- Disadvantages
  - Overhead in Context Switching
    - During Context Switching, heavy operations such as cache memory initialization occur, leading to overhead and significant time consumption.
    > `Context Switching`: Refers to the process of saving the state of the currently running Task (Process, Thread) and loading and applying the state values of the next Task to be run. Specifically, it describes the operation where a running process goes into a waiting state, its current state is saved, and then the next process in line, which was waiting, starts running by restoring its previously saved state.
    - Since processes are each allocated independent memory regions, there is no shared memory between processes. Therefore, when Context Switching occurs, all data in the cache must be reset, and cache information must be reloaded.

 <br>

 ### Multi-Thread
 - What is multi-threading?
   - It is the act of configuring a single application into multiple threads, with each thread handling one task.
   - Many operating systems, such as Windows and Linux, support multi-processing but are based on multi-threading.
   - Web servers are typical multi-threaded applications.
 - Advantages
   - Reduced system resource consumption (increased resource efficiency)
     - The number of system calls to create processes and allocate resources is reduced, allowing for more efficient resource management.
   - Increased system throughput (reduced processing cost)
     - Data exchange between threads becomes simpler, and system resource consumption decreases.
     - Context Switching is faster due to the smaller workload between threads.
   - Reduced program response time due to simple communication methods
     - Threads share all memory within a process except for the Stack region, which reduces the burden of communication.
 - Disadvantages
   - Requires careful design.
   - Debugging is difficult.
   - Benefits are hard to expect in single-process systems.
   - Threads cannot be controlled by other processes (i.e., individual threads cannot be controlled from outside their process).


<br>
<br>

### Why Use Multi-Threading Instead of Multi-Processing
- What does it mean to use multi-threading instead of multi-processing?
  - Simply put, it means solving multiple tasks within a single program rather than launching multiple programs.
- Why divide tasks that can be done with multiple processes (multi-process) into multiple threads within a single process?
  - Increased resource efficiency
    - When tasks executed as multi-processes are executed as multi-threads, **system calls for creating processes and allocating resources are reduced**, allowing for more efficient resource management.
    - This is because during Context Switching between processes, not only are CPU registers replaced, but data in the cache memory between RAM and CPU is also initialized, leading to significant overhead.
    - Since threads share memory within a process, unlike independent processes, data exchange between threads becomes simpler, and system resource consumption decreases.
  - Reduced processing cost and shorter response time
    - Also, the cost of communication between threads is lower than between processes, reducing the burden of communication between tasks.
    - -> This is because threads share all memory except for the Stack region.
    - The speed of switching between threads is faster than between processes.
    - -> This is because during Context Switching, threads only handle the Stack region.
  - Important note!
    - *Synchronization issues*
    - Resource sharing between threads uses global variables, which can lead to conflicts when used concurrently.
