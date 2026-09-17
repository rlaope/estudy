# Computing System

## Parallel System

It is a multi-processor system.

It's easy to understand as a concept where two or more CPUs perform their respective roles.

Tightly coupled system - Processors share and use memory and clocks.

There are two structures.

### Symmetric multiprocessing, SMP

Each processor executes with an identical copy of the operating system. Also, many processors can execute simultaneously without performance degradation. Most operating systems support this.

### Asymmetric multiprocessing, AMP

Each processor exists in its own role and consists of a master processor and slave processors.

It's a structure where the master processor (CPU) assigns tasks to the slave processors. It is used in large systems.

### Advantages

1. Economical.
2. High throughput.
3. graceful degradation, fail-soft system: The failure rate decreases over time.
4. High reliability: Even if one CPU fails, other CPUs can continue to perform tasks.

![Alt text](./image/image.png)

Easy understanding: If you think of the process where multiple people in a factory produce their own items, that's a parallel system.

<br>

## Distributed System

The emergence of distributed systems came about as personal computers became widespread, the internet became active, and the concept of distributed systems emerged.

Instead of using a large mainframe computer, a distributed system is created by combining several personal computers to form a system comparable to a large computer.

It requires network infrastructure like LAN. The structure is client-server.

Loosely coupled system - Each processor has its own local memory.

This is a difference from parallel systems (parallel systems share memory). Also, communication with other processors occurs via high-speed buses and networks.

![Alt text](./image/image.1.png)

### Advantages

1. Resource sharing
2. Fast speed
3. High reliability
4. Communication.

Easy understanding: Multiple people working together to create a single piece of work.

<br>

## Clustered System

Connecting multiple computers on a network in parallel to form one large system.

Whether it's a distributed system or a parallel system, the goal is a single system architecture. It allows shared storage for multiple systems.

- Asynchronous clustering: A structure where one server runs an application while other servers wait.
- Synchronous clustering: A structure where all N host servers run an application.

### Advantages
Offers high reliability.

<br>

## Real-Time Systems

It is a system specialized for specific fields, typically used in scientific experiments, medical systems, industrial control systems, virtual reality, and fighter jets.

It's a structure processed within a limited, fixed time. It is divided into Hard-real time and Soft-real time.

Accuracy is also key in Real-Time systems.

### Hard real-time

General operating systems have difficulty supporting it. Since processing occurs in milliseconds, failure to meet timing constraints can lead to serious consequences.

Furthermore, it must always maintain synchronization with all system environments. Auxiliary storage may be limited or absent, and data is stored in short-term memory or ROM.

### Soft real-time
Compared to Hard real-time, the strictness of time adherence is slightly lower, but exceeding the time limit can still cause inconvenience or errors.

It has the advantage that response time can increase if the system becomes overloaded.

If an error occurs in Soft real-time, recovery is performed by reverting to a previous checkpoint.

<br>

## Handheld Systems
Systems previously used in mobile phones. Characteristics include limited memory capacity, slow processor performance, and small screen size.
