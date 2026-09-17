# How to Prevent Data Loss During Redis Failover

Let's assume a distributed lock mechanism is applied to a single piece of data using Redis distributed locks.

While a lock is held, the Redis master node goes down, and a replica node fails over.

What would happen if the failovered replica node doesn't have the lock information, causing it to be lost?

Naturally, other nodes would not be aware of the lock information, leading to concurrency issues. (Loss of lock state)

Therefore, it is crucial to resolve this problem.

### RedLock

RedLock works by acquiring locks on multiple Redis nodes, and if a majority of nodes acquire the lock, then the lock is considered obtained.

Clients **consider a lock successful only if a majority of Redis nodes acquire it**. Even if one node fails, if a majority of nodes maintain the lock state, the lock remains valid. This means availability is further enhanced.

<br>

### Synchronous Replication

The main difference between synchronous and asynchronous replication lies in how data is written to replicas.

Most synchronous replication writes data to both the primary storage and the replica simultaneously. Therefore, the primary copy and the replica always remain synchronized.

In contrast, asynchronous replication writes data to the primary storage first, then copies it to the replica. While it can be implemented to reflect changes almost in real-time, it's more common for it to occur in a scheduled manner.

Asynchronous replication tends to be much less expensive than synchronous replication because the replication process doesn't need to happen in real-time.

While synchronous replication is more costly, the data across replicated nodes remains consistent. Therefore, if maintaining data replication consistency is crucial, **you could configure replicas using synchronous replication.**

<br>

### Client-Side Recovery

This approach involves building a mechanism on the client side to detect the possibility of lock loss during failover.

The client detects failover through heartbeats or timeouts. Upon detection, it can manually release the lock or wait for the lock's expiration before retrying.

By using these methods, we can address the problem of data loss during failover not only in Redis clusters but also in other cluster environments.
