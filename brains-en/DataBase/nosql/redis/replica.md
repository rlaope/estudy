# Redis important fact about replication

### Async Replication, WAIT

Redis asynchronously replicates master data by default to provide low latency and high performance.

Replication reliability is ensured by asynchronously checking the amount of data received from the master periodically to confirm which command streams have been processed.

Redis can request synchronous replication for specific data using `WAIT`.

<br>

### important facts about redis replication

As mentioned above, Redis uses asynchronous replication by default, and a single master can have multiple replicas.

A replica can also have other replicas, and in addition to connecting sub-replicas to the master, replication can be cascaded. (Since Redis 4.0, all sub-replicas receive the same replication stream from the master.)

Replication operates in a non-blocking manner on both the master and replicas, allowing queries to continue being processed even while the master is synchronizing or replicating to a replica.

Replication can also be used for scalability, with multiple replicas serving read-only queries or slow `ON` operations. It also offers the advantage of improving data stability and high availability (though the cost of managing replica nodes increases).

Replication can be used to reduce the write cost of persisting all data on the master node. This is achieved by configuring the master's `redis.conf` not to persist, and instead saving intermittently or using a replica with AOF (append-only file) enabled to write data to disk.

If master 'a' only replicates to a replica without persistence options, and 'a' goes down and is reloaded, master 'a' will load with an empty dataset. The nodes replicating from it will then synchronize, potentially losing the previous dataset. Even for in-memory datasets, if persistence is required, be sure to enable persistence options.

<br>

### redis replication works

Redis replication works in the following way:

1. A Redis master node has a unique Replication ID and an Increment Offset, even if no replicas are connected. This offset increases as replication occurs and changes are transmitted to replicas.
2. When a replica connects to the master, it uses the **`PSYNC` command to send the previous master's replication ID and the offset processed so far, allowing only the relevant part of the data to be replicated.**
3. If the master's buffer lacks a backlog or the replication ID refers to an invalid ID, a full synchronization occurs.
4. When a full synchronization occurs, the master starts a background saving process to create an RDB file and simultaneously buffers new write commands received from clients.
5. After the background saving is complete, the RDB file is first sent to the replica, followed by the buffered write commands.

<br>

### Allow writes only with n attached replicas

Since Redis 2.8, write queries are only performed on a Redis master if `n` replicas are connected.

However, because Redis uses asynchronous replication, there's no guarantee that a replica has actually received a specific write, which can lead to data loss.

To address this, replicas and masters exchange `Ping-Pong` messages to confirm the amount of replication stream processed by the replica.

The master then remembers the last ping time from each replica and measures the lag, allowing write operations only if `n` specified replicas have a lag of less than `m` seconds.

```
min-replicas-to-write <N>
min-replicas-max-lag <M>
```

If the conditions are not met, the master responds with an error and does not perform the write.

<br>

### How Redis Replication deals with expires on keys

Replicas do not expire keys on their own. Relying on the synchronization of master and replica clocks could lead to significant data consistency flaws.

Keys are expired using the following technique: **When a key expires or is evicted on the master, a `DEL` command is sent to all replicas.**

This approach can cause issues if a replica does not receive the `DEL` command. A logically expired key might persist. To handle this, a logical clock is used to communicate key expirations only for read operations that do not violate consistency.

<br>

### Partial sync after restarts and failovers

Since Redis 4.0, an instance promoted to master after a failover can perform partial synchronization with the replicas of the previous master. During this process, the promoted replica can provide a portion of the backlog using the previous master's replication ID and offset.

At this point, the promoted master acquires a new replication ID. The reason for a new ID is to prevent duplicate replication ID and offset pairs if the old master recovers.

If restarted after a graceful shutdown, the RDB file stores the information needed to resynchronize with the master, allowing for easy partial synchronization.

It is said that replicas restarted via AOF cannot perform partial synchronization. Therefore, before shutting down an instance, you should switch to RDB persistence, perform the restart, and then re-enable AOF.

[[Redis Replication]]
