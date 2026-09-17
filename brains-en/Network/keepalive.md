# HTTP Keep-Alive, TCP Keep-Alive

### HTTP Keep-Alive

This is a feature that allows a single TCP connection to be reused for multiple HTTP requests at the HTTP layer.

In other words, it's a performance optimization that prevents repeating the TCP 3-way handshake for every request.

### How it Works

- The client includes `Connection: keep-alive` in the HTTP request header.
- The server maintains the connection for a certain period.
- Multiple HTTP requests/responses are exchanged over the same TCP connection.
  The server closes the connection after an Idle Timeout (e.g., 60s).

Its purpose is to reduce RTT, improve latency, and enhance performance through connection reuse between the server and client.

### Characteristics

- Application layer (HTTP) feature
- How long the connection remains depends on the server's configuration.
- It does not have a feature to detect if the connection has been broken.
- If there are no requests, it remains idle.

<br>

### TCP Keep-Alive

This is a network-level feature to check if a TCP socket is dead (i.e., if the peer has disappeared).

In other words, its purpose is for the system to check if that connection is still alive.

### How it Works

- The kernel sends a keepalive probe packet after a certain period.
- If there's no response from the peer, it retransmits.
- If it fails a specified number of times, the kernel forcibly closes the socket (ETIMEDOUT).

### Purpose
- Removes old dead connections.
- Maintains connections to prevent NAT/firewalls from dropping idle connections.
- Quickly detects network disconnections.

### Characteristics
- Transport TCP feature
- Very long default period
  - `tcp_keepalive_time` (default 7200 seconds, 2 hours)
  - `tcp_keepalive_intvl`
  - `tcp_keepalive_probes`

| Item                        | HTTP Keep-Alive          | TCP Keep-Alive       |
| ------------------------- | ------------------------ | -------------------- |
| Layer                       | Application(HTTP)        | Transport(TCP)       |
| Purpose                     | Connection reuse (performance improvement) | Check connection liveness |
| Timing                      | Application-configured Idle Timeout | OS kernel configuration-based (minutes to hours) |
| Packets Used                | HTTP requests/responses  | TCP keepalive probe  |
| When Disconnected           | Server attempts Idle Timeout | Kernel closes socket on probe failure |
| Prevents NAT/Firewall idle drop | Impossible               | Possible             |


- HTTP Keep-Alive = Reuse
  - Maintains the connection for continuous use.
  - A feature for performance.
  - No role in detecting if the connection is dead.
- TCP Keep-Alive = Liveness Check
  - Checks if the peer is alive.
  - Detects network disconnections and server crashes.
  - Managed by the kernel.
