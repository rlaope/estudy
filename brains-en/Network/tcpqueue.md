# TCP Syn, Accept Queue

In the TCP 3-way handshake flow, the kernel explicitly separates and manages two queues.

```
SYN  →  SYN Queue  →  SYN-ACK
ACK  →  Accept Queue  →  accept()
```

This separation is a structure designed for security + performance + resource protection.

### SYN Queue (Half-open connection queue)

1.  State: SYN_RECEIVED
2.  The client sent a SYN, and the server sent a SYN-ACK, but the final ACK has not yet been received.

In other words, it's a queue that holds half-open connections where the connection is not yet fully established.

Since TCP is a stateful protocol, it uses this queue to remember that a SYN has been received.

It can also be used for retransmission handling, as a SYN-ACK retransmission is necessary if an ACK is not received.

It can serve as a defense point against SYN flood attacks (a method of overloading a server by indiscriminately sending SYN requests). Malicious traffic can be blocked at this stage.

The SYN queue is the boundary between the network-level firewall and the TCP state machine.

#### What the kernel does from its perspective

-   Generates ISN (Initial Sequence Number)
-   Stores options such as MSS, Window Scale, SACK Permitted
-   Registers a timer (for retransmission)
-   If the queue is full, SYN drop or SYN Cookie

### Accept Queue (Fully-established connection queue)

-   State: ESTABLISHED
-   3-way handshake completed state
-   Waits until the application calls `accept()`

It's necessary because applications cannot immediately accept connections.
-   Insufficient worker threads
-   Reasons such as GC, STW, CPU starvation

It exists to buffer the speed difference between the kernel and user space, and also provides back-pressure.

A back-pressure role, such as limiting new connections if `accept` cannot be called.

You can think of the Accept Queue as a buffer between the kernel and the application.

### backlog

```
listen(fd, backlog)
```

Here, `backlog` is often misunderstood, but it's the upper limit for the Accept Queue size; the SYN Queue is managed by a separate parameter.

In Linux, the actual value is determined as follows:

```
min(backlog, net.core.somaxconn)
```

### Reasons for separating SYN and Accept Queues

| Reason | Description |
| --- | --------------------------------- |
| Security | Blocks SYN Flood from reaching the Accept Queue |
| Performance | Manages connections lightly before handshake completion |
| Stability | Kernel buffers even if the app fails |
| Fairness | Separates network processing from application processing |

<br>

### Performance Tuning Cases

#### SYN Bottleneck

Let's assume an environment with a mix of SYN flood and legitimate traffic, where external traffic surges, the SYN queue becomes full, and legitimate clients also experience connection failures.

Observable metrics here include a significant increase in `SYN_RECV`, and increases in `ListenDrops` and `ListenOverflows`.

```
net.ipv4.tcp_max_syn_backlog = 8192
net.ipv4.tcp_syncookies = 1
```

Increasing the `backlog` value allows more SYNs to be handled, and attack traffic is absorbed by cookies. Only legitimate ACKs are allowed to enter the accept queue.

Here, you can think of `syncookies` as an option to only accept encrypted SYN-ACKs between the server and client.

#### Accept Bottleneck

Let's say `accept` calls are delayed due to GC or insufficient worker threads. Connection timeouts increase on the client side.

Here, check for an increase in `ESTABLISHED` connections and Accept Queue overflow, and verify CPU availability.

```
net.core.somaxconn = 4096
listen(backlog = 4096)
```

Since the bottleneck is primarily the application, let's just increase the queue size to hold more waiting connections. Of course, application tuning will still be necessary.

Separating a dedicated `accept` thread or GC tuning to reduce STW leads to solutions such as the kernel sufficiently buffering connections, enabling burst processing after app recovery, and reducing connection resets.

In other words, it's convenient to think of queues not as solving bottlenecks, but as preventing the propagation of failures.

| Misconception | Reality |
| ---------------------- | ---------------- |
| `backlog` = SYN Queue size | No |
| Increasing queue size is always good | No (increases memory/attack surface) |
| Slow `accept` means network issue | Mostly an app issue |
| SYN Flood is an L7 issue | No, it's an L4 issue |

-   The SYN Queue is a "filter that screens traffic attempting to connect."
-   The Accept Queue is a "buffer zone between the kernel and the application."
