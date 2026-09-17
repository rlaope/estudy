# Low Level, JVM Async Programming

### Asynchronous (Async)

Asynchronous programming refers to a structure that allows commands to be executed without waiting for results, enabling other tasks to continue.

It is frequently used in disk and network I/O, among other areas, and allows for efficient utilization of CPU resources.

- Synchronous (sync): Executes tasks sequentially, blocking until completion.
- Asynchronous (async): Requests tasks without waiting, responds via callbacks, etc.
- Parallel: Executes multiple tasks simultaneously.
- Multithreading: Multiple execution flows within a process.

### thread, context switching

- In an OS, programs run as processes, and threads are the units of execution within them.
- Java's CompletableFuture and Kotlin's Coroutine either create threads internally or use OS threads (executors).
- **Context Switching** is a time-consuming operation at the kernel level that involves saving and restoring registers, stack, program counter, etc.
- Asynchronous programming minimizes situations where the CPU is idle by avoiding context switching as much as possible, thereby utilizing resources efficiently. (Reduces scaling costs, allows fewer threads to handle more traffic).
- However, if blocking calls exist in an async/non-blocking system, it can lead to even greater bottlenecks, so caution is advised.

Disadvantages of async include unpredictable execution due to its non-sequential nature, difficulty in debugging, and complex exception handling.

There's also a risk of coroutine/async context leakage and memory leaks. Testing, logging, and tracing are also difficult.

Therefore, there's a steep learning curve, as using asynchronous techniques without proper validation can actually decrease throughput.

### Non Blocking I/O

Blocking I/O means that other tasks cannot be executed while the called task is in progress, i.e., control has been transferred.

Non-Blocking I/O is characterized by the caller still retaining control, allowing other tasks to proceed.

- **Blocking I/O**
    - `read(fd)` call -> User thread stops until kernel receives data -> CPU waste
- **Non-Blocking I/O + Event-Driven**
    - User thread calls `read(fd)` -> Registers with the kernel to be notified when data arrives.
    - When data is ready, the kernel notifies via `epoll`, `kqueue`, `IOCP`, etc.
    - User thread proceeds with subsequent tasks using callbacks or polling.
> Java NIO, Netty, Kotlin Coroutine, etc., use this structure.

- `epoll` is an **I/O event monitoring mechanism** provided by the Linux kernel.
    - Unlike `select()` and `poll()`, it can monitor thousands of file descriptors (fd) with O(1) performance.
    - An `epoll` instance is created with `epoll_create()`, and interested sockets (fd) are registered with `epoll_ctl()`. Then, `epoll_wait()` waits until an event occurs (non-blocking).
```c
int epfd = epoll_create(0);
epoll_ctl(epfd, EPOLL_CTL_ADD, sockfd, &event);
epoll_wait(epfd, events, MAX_EVENTS, timeout);
```

Servers like Netty, Undertow, and Vert.x internally use this structure to maintain thousands of connections even with a single thread.

- `IOCP` is a fully asynchronous I/O model based on Windows, where operations like `WSARecv` and `WriteFileEx` are registered as tasks and notify via callbacks upon completion.
    - A port is created with `CreateIoCompletionPort()` and an fd is bound.
    - Registers I/O operations (ReadFileEx).
    - Upon I/O completion, the kernel notifies a Worker Thread.
> In Java on Windows, `AsynchronousSocketChannel` internally utilizes IOCP.

<br>

### Comparison of Java and Coroutine Implementations

Java (CompletableFuture) is based on ForkJoinPool (thread pool), where the execution unit is a real thread, and thread switching costs exist. The suspend operation involves thread waiting. Therefore, if a thread blocks, performance degradation occurs.

Coroutine is based on a state machine, where the execution unit is a user-mode thread, and switching costs are very low with stackless coroutines. The suspend operation also saves the function state and jumps to the next state.

Therefore, coroutines have lighter execution units, allowing for more execution units, and are also easier to read. This is because they can implement async non-blocking behavior with sequential code syntax.

However, compatibility issues may arise, and CompletableFuture can be used without coroutine dependencies, still offering powerful features for parallel processing or simple asynchronous chaining.

If Kotlin is being well-adopted, then using coroutines seems to be the right choice.
