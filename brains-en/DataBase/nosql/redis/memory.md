# Redis Memory Management, Optimization Strategies

Redis is used for DB backup purposes with AOF/RDB methods for persistent data storage, and since it's primarily an in-memory cache, it's often used for caching.

Redis is an excellent solution that plays a significant role in improving system performance. However, *if Redis is not properly managed from a memory perspective, it can quickly lead to failures.*

<br>

### Swap

Redis stores data in memory. Therefore, if it uses more data than the physical memory (RAM) capacity, it can lead to memory exhaustion, causing swap to occur and degrading Redis's performance.

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_2.jpg)

As shown in the figure above, if more data is used than the physical memory capacity, swapping occurs. The primary function of swap space in an operating system is to act as a backing store (disk) for virtual memory when physical memory is full and more memory is needed.

This can prevent system crashes or process hangs even when actual memory is insufficient, making it more stable. However, **it uses disk space as if it were memory, which drastically slows down processing speed and leads to performance degradation.**

### Redis OOM

If the Redis server uses more data than the `maxmemory` limit without swap, it can trigger a `Redis: OOM (Out of memory) command not allowed when used memory than "maxmemory"` error.

Developers, cloud engineers, and administrators often encounter unexpected memory-related issues when adopting Redis open source for caching to improve performance. These issues can be resolved by applying an appropriate architecture tailored to the hardware specifications and business processes.

Let's explore some ways to address the problems mentioned above.

<br>

## Redis Memory Management

For memory management, Redis allows you to restrict the amount of memory used by setting the `maxmemory` option, preventing it from exceeding a certain limit.

Redis is used as an in-memory DB and stores a smaller amount of data compared to typical disk solutions. When it stores data up to its maximum capacity, new data enters Redis's memory, and existing data needs to be removed through eviction. Eviction policies can be configured via Redis's `maxmemory-policy` option.

### Maxmemory configuration

`Maxmemory configuration` involves configuring Redis to use up to a set amount of memory. This can be set in the `redis.conf` file, and the `maxmemory` amount can be specified at runtime using the `CONFIG SET` command.

**redis.conf**
```
maxmemory 100mb
```

**redis-cli**
```bash
config set maxmemory 107374182
```

> Setting `maxmemory` to 0 means there is no memory limit. This is the default behavior for 64-bit systems, while 32-bit systems use a 3GB memory limit. In other words, **64-bit systems have no memory limit**. Therefore, `maxmemory` must be carefully configured to prevent the use of swap space.

### maxmemory-policy configuration

When the `maxmemory` limit is reached, the exact eviction behavior that Redis follows depends on the `maxmemory-policy`. Among the methods Redis uses for eviction, the most commonly used is the LRU (Least Recently Used) algorithm (evicting the value that has not been used for the longest time). This is also widely used as a page replacement algorithm in operating systems.

The page replacement algorithms in operating systems are as follows:

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_4.jpg)

The page replacement algorithms applied in Redis are as follows:

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_5.jpg)

Redis only provides LRU, LFU, and RANDOM among the OS page replacement algorithms.

<br>

### Redis key eviction process

The Redis eviction process proceeds as follows:
1. A client executes a new command, adding more data.
2. Redis checks memory usage, and if it exceeds the configured `maxmemory` value, it evicts keys according to the policy.
3. The new command is executed.

**This can cause write amplification issues while Redis reclaims memory.**

In Redis's eviction mechanism, if a `maxmemory` limit is set and the `maxmemory-policy` is `noeviction`, the Redis memory reclamation process is triggered every time a client generates a new command.

Example: If the Redis server always operates in an overflow state (used_memory > maxmemory), this mechanism is triggered frequently. This affects server performance, and if replicas are connected, key removal operations are synced with replica nodes, which can lead to write amplification issues.

Therefore, it is best to configure the Redis server to always run in a state where used_memory < maxmemory.

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_6.jpg)

### Approximated LRU Algorithm

Note that Redis's LRU and LFU eviction policies are approximations.

The LRU algorithm in Redis is not implemented precisely, meaning Redis cannot always select the absolute least recently used top candidate for eviction.

**Instead, it samples a small number of keys and executes the LRU Approximated algorithm, which evicts the best key (the one with the oldest access time) among the sampled keys.**

In Redis, the LRU algorithm's approximation can be calculated by sampling a number of keys specified in the `maxmemory-samples` parameter setting. A larger sample size uses more memory but improves precision.

Parameters like `maxmemory-samples 5` can be adjusted in `redis.conf`.

The reason Redis does not use a true LRU is that it would require more memory and increase CPU usage.

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_8.jpg)

The graph above distinguishes between Redis's true LRU and approximated LRU. In Redis 3.0, the algorithm for selecting the best candidates for eviction was improved, making it more similar to a true LRU algorithm. **In Redis 3.0, setting the sample size to 10 can make it more closely resemble the functionality of a true LRU.**

However, be careful when increasing the number of samples, as it makes it similar to a true LRU algorithm. Setting the sample size beyond an appropriate level carries the risk of increased Redis CPU usage and slower response times.

<br>

In conclusion, to effectively set eviction policies, Redis can be periodically monitored for memory with `noeviction` to increase it. If data only needs to be retained for a certain period, `volatile-` policies can be considered; otherwise, `allkeys-` policies. Therefore, the system must carefully review and decide how to manage memory optimally according to business logic.
