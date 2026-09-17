# epoll

`epoll` is Linux's high-performance event-driven I/O monitoring system.

It is a core technology that must be understood to efficiently monitor and process non-blocking I/O in network servers or high-speed file handlers.

**Representative existing I/O event monitoring methods include the following:**
- `select()`: Passes all FDs as an array for the kernel to iterate, limited to 1024 FDs, sequential search O(n)
- `poll()`: Passed as a `struct pollfd[]` array, allows dynamic size but still sequential search O(n)
- `epoll`: Registers FDs with the kernel and only detects changes, event-driven O(1), strong for scalability

epoll enables event detection that can monitor thousands or tens of thousands of file descriptors, making it the de facto standard for high-performance network servers.

epoll creates a **separate data structure** within the kernel to store the FDs to be monitored.

Applications simply register FDs in this data structure and only receive notifications when changes occur (event-driven).

Flow
1. `epoll_create()` -> Creates an epoll instance (returns an FD) - Object creation
2. `epoll_ctl()` -> Registers file descriptors to monitor (EPOLLIN, EPOLLOUT, EPOLLET, etc.) - Register/Modify/Delete
3. `epoll_wait()` -> Blocks/returns until an event occurs - Event monitoring

**epoll's Core Modes**
- LT (Level Triggered): Default mode, detects events every time as long as the event persists.
- ET (Edge Triggered): Notifies only once when a change occurs, used in high-speed servers, requires reading the entire buffer.
> ET mode must be used with non-blocking sockets and read completely using a while loop.

### Why a while loop is needed for epoll LT vs ET

- Since ET notifies only once when a change occurs, data loss can occur if the buffer is not read until it's empty (without a while loop).

For example, if a client sends more than 8KB of data at once and the server reads 1KB at a time, data will be lost in ET mode without a while loop.

**Key Points**
- If `read()` is called only once, only part of the data arrives, and the rest is not notified. (ET mode)
- A `while(read(fd, buf, sizeof(buf)) > 0)` loop is necessary.

```c
if (event.events & EPOLLIN) {
    while ((n = read(fd, buf, sizeof(buf))) > 0) {
        // Process
    }
    if (n == 0 || (n < 0 && errno != EAGAIN)) {
        close(fd); // Terminate
    }
}
```

### epoll + nonblocking socket - O_NONBLOCK

Since epoll is asynchronous, using blocking sockets will cause `read`/`write` to block in addition to `epoll_wait()`, leading to performance degradation.
- If `socket()` -> `accept()` -> `read()` are used without `O_NONBLOCK`
    - `read()` will block in a waiting state, stopping the entire server.

### epoll Internal Structure - Linux Kernel `/proc` Tree

As mentioned, epoll manages FDs using a separate data structure within the kernel. Let's understand the concept of how this data structure operates.

Let's check for epoll's presence in the `/proc` tree and examine the source code level flow.

Let's check the internal state of an epoll FD via `/proc/PID/fdinfo/N`.

```bash
ls -l /proc/$(pgrep your_server_bin)/fd
cat /proc/$(pgrep your_server_bin)/fdinfo/N
```

epoll is implemented within the kernel with the following structure:

```c
struct eventpoll {
	spinlock_t lock; 
	struct rb_root rbr; // Red-black tree for FD management
	struct list_head rdlist; // Ready list for occurred events
	wait_queue_head_t wq; // Queue used for waiting in epoll_wait
	struct file *file;  // The epoll FD itself
}
```
- The FD for event monitoring is created with `epoll_create()`.
- It is registered in the tree with `epoll_ctl()`.
- When an event occurs, it is added to `rdlist`.
- When `epoll_wait()` is called, it is retrieved from `rdlist`.
    - Internally, `epoll_wait` calls `do_epoll_wait()` -> `ep_poll()` in sequence.

### `/proc`

Linux typically stores information about each process's open file descriptors in the `/proc/PID/` path.

Since epoll is also an FD, it appears in this structure.

```bash
ps aux | grep your_epoll_server
```

```bash
ls -l /proc/<PID>/fd
```

```bash
# result
3 -> socket:[12345]
4 -> anon_inode:[eventpoll]
```
> `anon_inode:[eventpoll]` indicates that it is an epoll FD, an anonymous inode within the kernel.

If you want to see the detailed information of that epoll FD, you can type `cat /proc/<PID>/fdinfo/4`.
```makefile
pos:     0
flags:   020000002
mnt_id:  12
tfd:     5 events: 1 data: 5
tfd:     6 events: 1 data: 6
```

- tfd: targetFD (monitored target)
- events: monitored event (EPOLLIN=1)
- data: user-defined data or FD

You can check how registered monitored targets are stored within epoll.

With the command `cat /proc/sys/fs/epoll/max_user_watches`, you can check the following:
- The maximum number of watch items a user can register with epoll.
- The default value is 1,000,000, and if this is exceeded, `epoll_ctl` returns `ENOSPC`.
