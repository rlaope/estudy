# Redis Replication

### Replication?

It's the process of copying data from one Redis instance to another in near real-time. (Redis Cluster)

Thus, even if the first Redis node providing service goes down, the second Redis node, having received the data, can continue to provide service. (failover)

In Redis, the first node is called the master node, and subsequent nodes are called replica nodes.

Without replication, if a Redis instance goes down due to human error or a software issue, and if AOF was enabled and a large amount of data had accumulated, the instance might take several minutes to start, leading to downtime.

If the cause of the downtime is a **hardware issue?** Restarting the service could take a significant amount of time, and data might not be recoverable.

Master and replica should be placed on physically separate machines.

Key features of Redis Replication are as follows:
- **Redis performs asynchronous replication.**
- Redis can have multiple replicas.
- A replica server can also create another replica server. (e.g., replica1 -> replica2)
- If a replica server starts when the master has a large amount of data, a massive amount of master data is sent to the replica server. Even during this process, the master does not stop and continues to process requests normally.
    - This is because the task of sending data for replication (generating an RDB file) is handled by a child process.
- Having replica servers handle read requests is also a good method for load balancing. Commands like `sort`, in particular, are recommended to be executed on replica servers.
- Performing AOF writes and RDB file generation on replica servers is also a good way to reduce load on the master. However, if such a configuration is set and the master is configured for automatic startup, data loss may occur.

<br>

### Replication Methods

Let's explore the data synchronization and replication methods for Redis master and replica one by one.

#### Full Synchronization

> Redis 2.8.18 and later versions provide the ability to replicate without creating RDB files on disk.

**Replication Process**
1. The master starts a child process to save data to an RDB file in the background.
2. While data is being saved, new commands received by the master are processed and stored in a replication buffer.
3. Once the RDB file save is complete, the master sends the file to the replica server.
4. The replica server receives the file, saves it to disk, and loads it into memory.
5. The master sends the commands stored in the replication buffer to the replica server.

- If the master goes down, the replica server sends a connect request to the master once every second.

- When the master comes back online, it syncs with the replica server according to the replication process.

- Even if there are multiple replica servers, only one RDB file is generated.

#### Partial Resynchronization

> Partial synchronization has been available since Redis 2.8.

Master and replica each have their server's `run id` and `replication offset`.

If the network connection between the master and replica server is lost, the master stores the data to be sent to the replica server in a backlog-buffer. When reconnected, if the **backlog-buffer** has not overflowed, it compares the `run id` and `offset` and synchronizes from that point onwards. This is called partial synchronization.

The size of the backlog-buffer is configured with the `repl-backlog-size` parameter.

If the network disconnection time is prolonged and the master's backlog-buffer overflows, a full synchronization is performed upon reconnection.

Full synchronization is also performed if either the master or the replica server restarts.

#### Master: Diskless Synchronization

> Redis version 2.8.18 and later provides diskless synchronization.

This feature can be used if Redis is used as a cache and the disk performance of the machine where the master is installed is poor.
- Diskless operation applies only to the master.
- The replica server saves the received data to an RDB file.
- This method involves the master's child process directly writing RDB data to the replica server via a socket.

It is configured via the `redis.conf` parameter. `repl-diskless-sync no or yes, default no`

The default is set to `no`, and it must be set to `yes` for diskless synchronization.

When requests come from multiple replica servers, data is primarily sent to the first replica server's socket, and once completed, the next replication is processed.

There is also an option to wait for requests to process several replica servers at once. The `repl-diskless-sync-delay 5` option can be adjusted: after the first request arrives, it waits for 5 seconds for requests from other replica servers, and if requests arrive, they are processed together. This means if synchronization requests arrive from 3 replica servers within 5 seconds, they can be performed in parallel. If immediate processing is desired, it can be set to 0.

<br>

#### Replica: Diskless Synchronization (repl-diskless-load)

> This feature is available from Redis 6.0.

This is diskless synchronization on the replica server, meaning RDB files are not created on the replica server.

There are three options for `redis-diskless-load` in `redis.conf`: `disabled/on-empty-db/swapdb, default disabled`.
- disabled: Diskless is not used (disk is used).
- on-empty-db: Applies if the replica server has no data; if data exists, an RDB file is created before replication.
- swapdb: Operates diskless regardless of whether data exists on the replica server. In this case, existing data is preserved in memory (RAM) as a contingency. If replication succeeds, the data preserved in RAM is deleted. If replication fails, it recovers with the data preserved in RAM. This mode requires sufficient memory, as it needs memory for both existing data and new data.

> Additionally, there is a way to use replica servers as read-only. Since Redis 2.6, you can set the `replica-read-only` option (default `no`) to `yes` in `redis.conf` to use it as read-only. Even if data is entered into the replica server, it will disappear when resynced with the master.
