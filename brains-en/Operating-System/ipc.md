# IPC(Inter Process Comunication)

Processes are independent. This means that a process is not affected by other processes.

So, what can be done when such independent processes need to communicate with other processes?

IPC makes this possible.

Processes can communicate with each other through IPC facilities provided by the kernel.

![](https://camo.githubusercontent.com/0831672a89dcda8f8d351b42614af3991fd701381fdd94b32950c6bded355eb2/68747470733a2f2f74312e6461756d63646e2e6e65742f6366696c652f746973746f72792f393944423843343935433443353730343137)

There are several types of IPC. Let's explore them.

### Anonymous PIPE

Pipes connect two processes, with a structure where only one process can write and the other can only read.

**It is half-duplex communication, communicating in only one direction.** Therefore, if you want to send/receive in both directions, you need to create two pipes.

It has the advantage of being very simple to use, and using pipes is efficient for simple data flows. The disadvantage is that implementation becomes complex when two pipes are needed for full-duplex communication.

<br>

### Named PIPE(FIFO)

Anonymous pipes are used when the communicating processes are known (e.g., parent-child processes).

Named pipes are used for communication between unrelated processes.

However, since these pipes are also half-duplex, two pipes must be set up for bidirectional communication.

<br>

### Message Queue

The I/O method is similar to named pipes.

The difference, however, is that it's a memory space, not a data stream like pipes.

By assigning numbers to the data to be used, multiple processes can handle data concurrently.

<br>

### Shared Memory

If pipes are communication mechanisms, **shared memory is a facility that supports sharing the data itself.**

Each process has its own independent memory region, which must be protected from access by other processes.

However, there are times when other processes need to use that data.

It can be transmitted via pipes, but it can also be used as a way to share memory, similar to threads.

**Shared memory allows processes to share memory regions.**

When a process requests shared memory allocation from the kernel, the kernel allocates it in the process's memory space and allows access to that memory region.

- It operates fastest among IPC mechanisms because memory can be accessed without an intermediary.

<br>

### Memory Map

It shares memory, similar to shared memory.

Memory mapping is a method of sharing by mapping an open file into memory. (i.e., the shared medium is file + memory)

It is primarily used for sharing large files.

<br>

### Sockets

Data is shared via network communication through network sockets.

It's a structure where clients and servers communicate via sockets, used for sharing data between processes remotely.

Server (bind, listen, accept) Client (connect)

In such IPC communication, semaphores and mutexes are used to synchronize and protect data between processes. (When only one process should access a shared resource at a time)
