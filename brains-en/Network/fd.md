# File Descriptor in Network(cc. KeepAlive)

It is an integer number assigned by the operating system to identify I/O resources such as open files, sockets, and pipes.

In Linux, all of the following are treated as FDs:

- Regular files
- Directories
- Sockets (TCP, UDP)
- Pipes
- epoll instances
- eventfd
- signalfd

In other words, an FD acts as a pointer for all I/O, not just simple file handling.

### How Processes Use FDs

A process maintains an FD table for all I/O targets.

For example, when a server accepts one TCP socket:

- A new entry is created in the process's FD table.
- The reference count in the kernel's global file table is incremented.
- The TCP socket is actually allocated in kernel memory.

Conclusion: One client connection = one additional FD occupied.

### FD Exhaustion

This refers to a state where new FDs can no longer be allocated, either within a process or across the entire system.

The causes are simple:

- The server maintains too many TCP connections simultaneously.
- Idle connections that are not closed accumulate.
- The FD limit is low (`ulimit -n`).
- Resource leaks (missing `fd close`).

When FDs are exhausted, errors such as the following occur:

- EMFILE (per-process FD limit exceeded)
- ENFILE (system-wide FD limit exceeded)
- The server can no longer accept sockets.
- Logging failures
- Inability to open files

Since FDs are a core infrastructure resource, if they are exhausted, the server starts behaving as if it's dead.

### FD Limit Values (Most Important Part)

In Linux, FD limits exist at three levels:

System-wide limit `proc/sys/fs/file-max`: The total number of file handles that can be open simultaneously across the entire system.

Per-user limit `nofile` within `/etc/security/limits.conf`: Here, soft/hard limits can be configured.

Per-process limit `ulimit -n`: The number of FDs a single server process can open.

Most servers have a very low default of 1024, and in production environments, this is typically raised to around 65535-200000.

### Why FDs Get Exhausted in a Keep-Alive Environment

As you might easily predict from the explanation, HTTP keep-alive is a feature that maintains TCP connections for a long time, meaning connections are not closed after request processing.

Thus, as the number of clients increases, allocated FDs remain occupied for longer, leading to an increase in FD usage. Idle connections also continue to consume FDs.

The following scenarios frequently lead to issues:
- High load leading to a long keep-alive timeout setting
- Load balancers or clients maintaining abnormal connections
- Persistent connections from crawlers, bots, or slow clients
- Servers where the FD limit has not been raised

Therefore, high-load servers must take measures to prevent FD exhaustion, such as:
- `keepalive_timeout`: Set appropriately
- `max_keepalive_requests`: Limit
- Limit connection pool size
- Connection offloading in front of a reverse proxy
- Reducing the number of H2/H3 connections itself

FDs are not merely measured by the number of open files; they actually refer to kernel resources and thus incur overhead.

For example, for each TCP socket:

```
소켓 상태 구조체
send buffer
receive buffer
TCP control block
retransmission timer
SYN queue / accept queue
```

These are necessarily maintained in memory, and as the number of FDs increases, kernel memory usage and context switching costs also increase.

```bash
# process 단위
lsof -p <pid> | wc -l
cat /proc/<pid>/limits

# 전체 시스템 단위
cat /proc/sys/fs/file-nr
cat /proc/sys/fs/file-max

# Prometheus
node_filefd_allocated
process_open_fds
```

In summary, FDs are I/O resource identifiers, and as a server maintains many connections, FDs accumulate and are prone to exhaustion in a keep-alive environment.

Therefore, FDs should be treated as one of the core resources for performance, capacity, and connection management.
