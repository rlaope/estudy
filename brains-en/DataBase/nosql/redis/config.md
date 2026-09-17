# Efficient Redis Server Configuration

To use Redis effectively, it's good practice to modify some server configuration files and Redis's own configuration file.

Let's look at some settings that are beneficial to enable for several advantages.

### Disable THP

Linux manages memory in page units, with the default page size fixed at 4KB. As memory size increases, the size of the TLB (Translation Lookaside Buffer), which manages pages, also grows. This can lead to overhead when accessing memory, so the THP (Transparent Huge Page) feature was introduced to automatically manage larger pages.

However, for database applications like Redis, using this feature can actually degrade performance and increase latency, so it's recommended to disable it.

```sh
echo never > /sys/kernel/mm/transparent_hugepage/enabled
```

You can temporarily disable it using the command above. If you want to apply it permanently, add the following statement to Redis's `/etc/rc.local` file.

```sh
if test -f /sys/kernal/mm/transparent_hugepage/enbled; then
	echo never > /sys/kernal/mm/transparent_hugepage/enbled
fi
```

<br>

### vm.overcommit_memory = 1

When Redis saves files to disk, it uses `fork()` to create a background process, and during this, a mechanism called COW (Copy On Write) operates. In this mechanism, the parent and child processes share the same memory pages. However, whenever data is modified by Redis, **memory pages are copied, which can lead to a rapid increase in memory usage if data changes frequently.**

Therefore, situations may arise where the Redis process needs to sequentially over-allocate memory during its execution. To handle this, `vm.overcommit_memory` should be set to 1. This allows for momentary over-allocation of memory, preventing incorrect behavior and avoiding performance degradation or errors during background data saving. (It is set to 0 by default.)

Adding `vm.overcommit_memory=1` to the `/etc/sysctl.conf` file will apply this setting permanently. To apply the setting without rebooting, execute `sysctl vm.overcommit_memory=1`.

<br>

### Change somaxconn, syn_blocking settings

The `tcp-backlog` parameter in Redis's configuration file specifies the size of the TCP backlog queue that a Redis instance uses when communicating with clients.

The `tcp-backlog` value specified in `redis.conf` cannot be greater than the server's `somaxconn` (socket max connection) and `syn_blocking` values. Since the default `tcp-backlog` value is 511, the server settings should be configured to be at least greater than this value.

You can resolve this by adding statements to the `/etc/sysctl.conf` file, but to apply them without restarting, use `sysctl`.

```bash
# redis config 
tcp-backlog 1024 

# sysctl 
sysctl -w net.ipv4.tcp_max_syn_backlog=1024
sysctl -w net.core.somaxconn=1024
```

 **Why set it to a large value?**

 `syn_backlog` stores `SYN + ACK` flag packets in the backlog during a TCP half-open state, before the 3-way handshake is complete. If the client sends an `ACK` response, it's cleared; otherwise, it's kept until timeout. The Redis backlog size cannot exceed the `syn_backlog` and `somaxconn` settings.

If `syn_backlog` or `somaxconn` = 128, then 128 + 1 = 129 -> 129 * 2 = 258 -> approximate value of 258 = 256. This means Redis cannot have a backlog of more than approximately 256. Even if Redis has `tcp-backlog` 511, if `syn_backlog` and `somaxconn` = 128, it can only handle 256 connections.

> Simply put, even if Redis has a large backlog queue size to store the number of clients to be connected to Linux, Redis cannot process them efficiently if the system limits are too low, so these settings are increased.
