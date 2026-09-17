# HTTP Keep-Alive

HTTP Keep-Alive maintains a TCP connection for a certain period once it's established, minimizing 3-way handshakes for each HTTP communication to reduce latency and allow continuous use of the connection.

Today, we'll delve deeper into various Keep-Alive scenarios and questions.

### Why does HTTP Keep-Alive disconnect before TCP Keep-Alive?

HTTP Keep-Alive actively manages connections at the application layer based on an idle-timeout.

**Servers close connections if there are no requests for a certain period to efficiently use resources.**

In contrast, **TCP Keep-Alive is a passive mechanism at the kernel level to check connection liveness**, so its default value is very long, like 2 hours.

- HTTP Keep-Alive: Resource management purpose -> short idle timeout
- TCP Keep-Alive: Connection liveness check purpose -> long idle timeout

### What problems arise if there are too many HTTP Keep-Alive connections?

For each connection, the server must maintain the following:
- file descriptor
- socket buffer
- thread or event loop context

If there's an abnormally high number of Keep-Alive connections,
- FD exhaustion
- Increased thread blocking (for blocking applications)
- Throttling due to delayed eviction
- Increased load balancer connections

can occur. Therefore, most servers limit `keepalive_timeout`, `max_keepalive_requests`, `worker_connections`, and so on.

### HTTP/1.1 Keep-Alive vs HTTP/2 Persistent Connection

HTTP/1.1 Keep-Alive simply reuses requests sequentially over a single TCP connection.

While it improves bandwidth efficiency, it suffers from the Head-of-Line Blocking problem.

HTTP/2 uses
- a single TCP connection
- multiplexing (based on Stream ID)
- header compression (HPACK)

to process multiple requests in parallel over a single TCP connection.
Therefore, even with the same Persistent Connection structure, HTTP/2 is much more efficient.

### Why do Keep-Alive connections get dropped by NAT/firewalls?

HTTP Keep-Alive connections remain in a prolonged idle state where no TCP packets flow.

NATs and firewalls typically delete idle connection table entries within 30 to 120 seconds.

Since HTTP Keep-Alive does not send packets at the TCP level, no refresh occurs in the NAT table,

which can lead to connections being dropped. To prevent this,
- TCP Keep-Alive ON
- Adjust kernel keepalive time
- Application-level ping

are necessary.

### How does a client detect if a server crashes while in a Keep-Alive state?

Detection is impossible with HTTP Keep-Alive alone.

It can only be detected at the TCP level.

That is, if the server crashes -> no TCP FIN/RST.

The client remains unaware that the socket is dead until it sends the next HTTP request, at which point it might receive
- an RST
- ETIMEDOUT
- Broken Pipe
and only then becomes aware. Therefore, for real-time services, don't rely solely on Keep-Alive; heartbeat, ping, and retry strategies are necessary.

### What criteria should be used to adjust the Timeout in HTTP Keep-Alive?

The adjustment criteria are the following three:
1. Service characteristics (request interval)
2. Server resources (FDs, number of workers)
3. Load balancer idle timeout (NLB/ALB, ELB)

Generally, it's best to set it slightly shorter than the LB Idle Timeout.

For example, if the ALB Idle Timeout is 60 seconds, the server's Keep-Alive might be set to 55 seconds.

### How is Keep-Alive related to HTTP Pipelining?

In HTTP/1.1, Keep-Alive is a prerequisite for pipelining.

However, pipelining is rarely used due to the Head-of-Line Blocking problem.

In HTTP/2, the structure itself changed to multiplexing instead of pipelining.

### In what situations would you completely disable server Keep-Alive?

It's advisable to disable it in the following situations:
- Very short requests (e.g., IoT servers)
- High FD burden due to a large number of short-lived connections
- When a tangled proxy chain makes connection reuse risky
- Environments where there's no security need to maintain connections for long periods

In practice, it's usually not disabled, but it's sometimes turned off in high-load IoT environments.

### What impact does Keep-Alive have on latency?

Keep-Alive eliminates the following:
- TCP 3-way handshake overhead
- Slow start initial phase

Especially since the handshake alone adds 1 RTT, its effect is amplified in high-latency WAN environments.

### The Difference Between HTTP Keep-Alive and Connection Pool

- HTTP Keep-Alive: Keeps the actual OS socket alive
- Connection Pool: A structure that manages sockets for reuse by the application

Both servers and clients configure connection pools. Keep-Alive is a foundational technology for these pools.
