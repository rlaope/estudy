# Kernel Space vs User Space

### Kernel Space

Kernel Space is a privileged area where the operating system kernel runs.

It operates at CPU privilege level Ring 0 (the highest privilege).

The kernel primarily performs the following roles:

- Process management (scheduling, context switch)
- Memory management (page table, virtual memory, NUMA, TLB)
- File system management (VFS -> ext4/xfs ...)
- Network stack processing (TCP/IP)
- Device driver operation
- Security policy handling (LSM, seccomp, etc.)

In other words, **hardware access and system resource management** are all tasks performed by the kernel, and user processes cannot access them directly.

Kernel Space Code Example
- The Linux Kernel itself
- Drivers (NIC Driver, block device drivers)
- System call handlers
- TCP/IP stack (including netfilter, conntrack)

### User Space

User Space is a non-privileged area where general applications run, with privilege level Ring 3.

- Web servers (Nginx, Spring Boot, NodeJS)
- CLIs (ls, grep, bash)
- Docker daemon
- JVM, Python Interpreter etc..

User Space cannot directly access CPU, memory, disk, or network hardware; all hardware requests must go through Kernel Space.

### Why the Distinction?

If a user process corrupts kernel memory through an incorrect pointer access, the entire system can crash.

Therefore, the kernel isolates user space to protect itself, i.e., for system stability.

Also, since malicious code can be mixed into the user area, the kernel must isolate it in Ring 3.

Preventing direct manipulation of hardware prevents privilege escalation, which is for security.

Furthermore, since the kernel must perform high-performance I/O processing (NIC, block I/O, TCP stack), it optimizes interrupt handling, DMA control, memory mapping, etc., in a dedicated area.

### System Call

User Space -> Use kernel functionality -> Return to User Space

At this point, a system call is necessarily triggered.

For example, the `write(fd, buf, size)` call involves:

1. `write` call in user space
2. System call number is placed in a register, and entry into the kernel occurs.
3. CPU switches from Ring 3 to Ring 0 (privileged mode).
4. The kernel requests I/O from the device driver.
5. After the operation is complete, return to user space (Ring 0 -> Ring 3).

The important point is that this process incurs context switch + privilege transition overhead.

Therefore, in high-performance servers, it's crucial to reduce the following:
1. Reduce the number of syscalls
2. Use `epoll`
3. `sendfile` (zero-copy)
4. Use `io_uring`

These methods are linked to server performance optimization.

```
┌───────────────────────────────┐
│           User Space          │
│  App, JVM, Python, Nginx      │
│  ────────────────┐            │
└──────────────────┼────────────┘
                   ▼  System Call
┌──────────────────┴────────────┐
│          Kernel Space          │
│ Scheduler, Memory Manager      │
│ TCP/IP stack, VFS, Drivers     │
└────────────────────────────────┘
                   ▼
             Hardware (CPU, NIC, Disk)
```

### Clarifying a few terms mentioned above

**Interrupt Handling** refers to hardware sending a signal to the CPU that something needs to be processed now.

e.g., a NIC receiving a packet and generating an interrupt that a packet has arrived, a disk reading and generating an interrupt that data is ready, a keyboard input generating an interrupt that a key has been pressed.

Without interrupts, performance would degrade because the system would have to continuously poll to check if data has arrived.

**DMA (Direct Memory Access)** control refers to

DMA is a technology that allows devices to read and write data directly to memory without involving the CPU.

We can take the process of a NIC receiving packets as an example.
1. The NIC copies packets to system memory using DMA.
2. The NIC sends an interrupt to the CPU.
3. The CPU processes by reading only the copied memory address.

In the days before DMA, copying occurred from hardware -> CPU -> memory, leading to very high CPU load.

With DMA, operations occur from hardware -> memory, freeing the CPU from copying tasks and becoming key to implementing high-speed I/O.

**Memory mapping** is when the kernel directly connects a specific memory region (such as a file or device memory) to a process's virtual address space.

If a file is memory-mapped with `mmap()`, reading from memory directly becomes reading file data without calling `read()`.

TCP sockets also manage kernel buffers based on `mmap`.

It's important because it's a structure that reads memory directly without `read`/`write`, improving performance, and it's a zero-copy technology, which also boosts performance.

Of course, process memory is not shared with other memory by default, but in situations where sharing is needed, it's easier to think of sharing as cheaper than copying. It's good because functions like `open`, `read`, `write`, `lseek` can be reduced to a single `mmap`.

### Reducing the number of syscalls

Every time user space enters kernel space, a system call overhead occurs with each `read`, `write`, `recv`, `send` call.

Switching from Ring 3 to Ring 0, context switching, kernel validation, and kernel buffer operations make it expensive.

Therefore, reducing these is essential for better performance.

Let's look at using `epoll`. Socket I/O basically works as follows:

```scss
read()  
Blocks if no data  
Another read()
Another read()
```

This makes it impossible to monitor thousands or tens of thousands of sockets.

However, `epoll` only notifies when an event occurs on a socket.
- New packet arrival
- Connection disconnected
- Writable

In other words, notify only when needed -> drastic reduction in syscall count.

That's why high-performance servers like Netty are `epoll`-based.

Next, let's look at `sendfile` (zero-copy).

Generally, the file -> network transfer process is as follows:

1. File -> kernel buffer
2. Kernel buffer -> user space buffer
3. User space buffer -> kernel network buffer
4. Transmitted via network card

This means 3 copies occur. But with `sendfile`? It allows direct transfer from file -> kernel -> NIC.

In other words, it's zero-copy without the user space copying step, reducing CPU usage.

Nginx, Kafka, Envoy, etc., all use `sendfile`.

Finally, let's explore `io_uring` (Linux's latest I/O API).

Historically, I/O required syscalls, using the `epoll` + `read`/`write` approach.

But `io_uring` enables the following:
1. Direct I/O requests to the kernel queue from user space
2. Polling for results from the completion ring
3. Executing tens or hundreds of I/O operations without syscalls
4. The `read` and `write` calls themselves disappear

In other words, it's a structure that removes almost all bottlenecks of traditional Linux I/O, making it the fastest Linux I/O method.

Redis, PostgreSQL, Nginx, etc., have also started supporting it.
