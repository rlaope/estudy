# Mechanism by which Netty identifies completed tasks

First, the most important concept to grasp is the relationship between connections and threads, i.e., the relationship between connections and execution units.

The traditional Blocking (Tomcat) approach establishes a 1:1 relationship with a thread per request, where a connection is assigned to a single thread and remains bound to it until the connection is terminated.

Therefore, there's no real need for a method to identify that specific request, as the thread is solely responsible for that one connection.

However, in a Non-Blocking (Netty) system, N connections are assigned to a single thread, but a connection is merely an object (a chunk of memory).

The thread acts as a manager for these objects, looping without being tied to any specific connection.

Since a thread handles multiple connections, a lookup mechanism, a Selector, is needed to identify which connection an event belongs to when it occurs.

### I/O Multiplexing (Selector Pattern)

Netty's process for identifying tasks follows a three-stage structure: subscription - event - dispatch.

#### Subscription

When a client connects or sends a request, Netty asks the OS kernel to notify it when data arrives on a socket (e.g., fd 101). The owner of this socket is the `Channel_A` object. This is how it works:

A management table called Selector stores mappings in the form of Key: `socket_no`, Value: `Channel Object`.

#### Detection (Select)

Netty's event loop thread typically calls the `select()` method and waits.

The kernel filters out only the sockets where data has actually arrived (completed) from among the thousands of registered sockets.

Even if 1000 connections exist, if data has only arrived on 1, the thread will only wake up for that single one.

#### Identification and Execution (Dispatch)

When the kernel returns that there is data to read on socket 101, the thread queries the Selector table for key 101.

It obtains the associated `Channel_A` object and executes the business logic by calling `Channel_A.pipeline().fireChannelRead()`.

<br>

So, for example, if there's a response delay from an external integration system, the internal state would be as follows:

In the Blocking model, the thread enters a wait/sleep state, occupying stack space as the thread itself holds connection information and is paused. It becomes unusable, effectively 'deleted,' until a response arrives, at which point it wakes up.

In the Non-Blocking model, the thread is freed and continues in a runnable state. Connection information exists only on the Heap, registered in the Selector table. The thread immediately moves on to process other requests, and when the kernel provides a notification after a response arrives, the thread returns to handle it.

In summary, when the operating system points out **the socket number where a change occurred**, Netty immediately retrieves the corresponding `Channel` object from the pre-configured mapping table, the Selector.
