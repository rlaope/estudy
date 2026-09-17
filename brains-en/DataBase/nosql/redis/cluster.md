# Redis Cluster

Redis is fundamentally an in-memory data store and is used in many places.

Since Redis is used even in mission-critical applications, it is important to operate it with a focus on availability.

### High Availability

For some systems, high availability is crucial.

If a critical component like Redis maintains 99% availability, system failures can occur during that 1% of the time. There can be systems that cannot tolerate even that 1% of downtime.

Many systems want to remain as stable as possible, but failures can occur due to problems we cannot anticipate. To operate without major issues even if Redis goes down, we can use Redis cluster for high availability.

<br>

### Replication

Fundamentally, the way to ensure high availability is to deploy systems redundantly, and this redundant replication introduces a new problem of synchronizing multiple replicas.

Looking at how Redis performs replication when configuring replicas: when there is data in the Primary Redis and a new replica is added, the Primary Redis is forked and all data is transferred to the Secondary Redis.

> During this process, memory usage can spike due to forking, and since data transfer methods other than forking exist, there are trade-offs.

Redis succeeds in changing values even if not all replicas are synchronized. This is a decision made for Redis performance, and replication guarantees eventual consistency.

It uses statement-based replication, and some non-deterministic functions can have different values depending on primary or replica. This is different from MySQL's binlog-based approach.

Each of these statements is assigned an ID, and if the primary and replica IDs match, the data is considered fully synchronized.

<br>

### Failover

Since we performed replication for high availability, the system operates without problems even if Redis dies.

> Actually, replication is not solely for high availability. For reads, replicas also have data, so they can be handled by distributed Redis nodes. If the primary can handle writes sufficiently and eventual consistency is acceptable, it's a good choice.

If a replica goes down, it's unfortunate if it was created for load distribution, but we still have the primary alive, so work can be processed on the primary or another replica.

What if the Primary dies? One of the replicas is promoted to primary, and the promotion criteria are determined through a consensus algorithm. There's no guarantee that the replica has the same data as the primary, but it operates. If there was only one, it wouldn't operate at all.

#### Split Brain

Network partitions can occur in distributed systems. The primary may still be alive, or a replica may think it is the primary. In such situations, the Redis cluster may not work well. (Cases where a healthy node is incorrectly judged as dead due to network partition)

This problem is called the Split Brain problem, and it is specified in the [redis docs](https://redis.io/docs/reference/cluster-spec/#failure-detection) that promotion requires agreement from all nodes through a distributed consensus algorithm.

A replica may temporarily recognize itself as primary, but if it hasn't received agreement from all nodes, the promotion process is canceled.

<br>

### Partitioning

Redis is sometimes used for partitioning purposes.

We cannot store all data in Redis memory. This is even more true for large systems... As a solution to this, besides not using Redis, we can choose to **divide and store data**. We call such methods partitioning, sharding, etc.

To process data in a divided manner, we need to know which data is located on which node. We mainly use consistent hashing. Simply put, it's a method of creating a ring and specifying ranges to determine data location.

**Redis Cluster does not use consistent hashing by default**, but uses a similar concept called **Hash Slot**. Redis Cluster hashes the content inside brackets of a key (`KEY{HASH_TARGET}KEY`) using an algorithm called CRC16. The result produces a value from 0 to 16383.

> If a request fails, it returns a MOVED result along with the location of another node where the data should be stored, and the Redis Client must perform the write operation again on the other node. If this is not supported by the client library, you must do it yourself.

#### Re Partitioning

In the case of partitioning, there are cases where new nodes are added due to insufficient nodes, in which case the developer must manually register them in the Redis cluster and specify the range of hash values that the new node should handle.

In this case, hash range adjustment occurs with other nodes and replication of some data proceeds.

<br>

### Gossip Protocol & BroadCast

As mentioned above, if data outside its hash range comes into a Redis cluster node, it returns a MOVED command along with a new destination. Also, when repartitioning, the range of hash keys changes. The question here is how can it know the configuration of other nodes?

Redis uses the `Gossip Protocol` to communicate. While exchanging gossip, they send each other's heartbeats to check for failures and also convey whether shared configurations have been updated.

There's more content if you dig deeper, but here it's good to know it as a method for nodes to exchange data.

Redis Cluster uses a monotonically increasing value called **configEpoch** for each such configuration to determine the latest state, and if the configEpoch value is smaller, that configuration is considered changed and the new value is applied.

**The configEpoch value increases during failover and also when developers manually reshard.** Ideally, configEpoch propagation would be fast enough, but in reality, it's not. In distributed systems, incorrect situations can occur for any reason, and the same configEpoch can appear and collide. This problem must be solved. Because if two different configurations exist, it may not work correctly...

When failover occurs through a distributed consensus algorithm, since all nodes must agree on the final state, configEpoch is guaranteed to be a unique value. However, changes triggered directly by developers can cause conflicts. The `CLUSTER FAILOVER [TAKEOVER]` command forcibly changes the epoch, causing conflicts.

If such a collision occurs, one version wins through the [configEpoch conflict resolution algorithm](https://redis.io/docs/reference/cluster-spec/#configepoch-conflicts-resolution-algorithm)

1. A master node discovers another master node with the same configEpoch.
2. Compare the IDs of nodes with the same configEpoch and elect the lexicographically smaller node.
3. Increase the currentEpoch of the selected node by 1 to issue a new configEpoch.

#### currentEpoch

A concept called currentEpoch, not configEpoch, has appeared. currentEpoch plays an important role in Redis cluster's broadcast algorithm.

There are various distributed consensus algorithms. Many algorithms such as Paxos, Raft, and ZAP are used. Paxos is used in Cassandra, Raft in etcd, and ZAP is used as Zookeeper Atomic Broadcast. These distributed consensus algorithms are used in the leader election process and also when determining the order of requests. (Total Order Broadcast)

**In the case of Redis cluster, it adopts a full mesh structure (a structure where nodes are connected to all other nodes).** Between these connected nodes, the gossip protocol is used to check heartbeats and update configurations. In other words, configuration changes are broadcast via the gossip protocol.

Each node opens a port and exchanges data between nodes through this port. In this process, a command called `CLUSTER MEET` is used.

Redis calls this method of communication between nodes the [Cluster Bus](https://redis.io/docs/reference/cluster-spec/#the-cluster-bus), and the way they are connected is called Cluster Topology. In this case, the Topology becomes FullMesh.

When exchanging data, configurations are also updated together. **Although you can tell if such configuration has changed with configEpoch, currentEpoch is basically used.** currentEpoch is a 64-bit unsigned int value and represents the monotonically increasing version of each event.

In a situation where gossip is exchanged, if one's own currentEpoch is smaller, the larger currentEpoch is selected. By exchanging gossip this way, all nodes agree to have the largest currentEpoch value.

#### Election, Promotion

Let's look at how to promote a replica to primary in a failover situation.

The election process proceeds from the replica. While exchanging gossip, it was confirmed that some replica failed to receive a heartbeat sent from the primary. When we know the primary is down, we know that failover should proceed, and failover must be performed very quickly and accurately to avoid system problems.

```
1. The replica's primary is in a fail state.
2. The primary uses a non-zero number of slots.
3. The link between primary and replica for replication is disconnected longer than the time given by the primary.
```

Even if a problem is detected, the replica does not start the election immediately but waits for the time defined below before starting the election. In the definition, REPLICA_RANK represents how many primary requests the replica has processed. In other words, it waits longer by the difference between the primary's offset and the replica's offset. Therefore, the more similar the replica and primary data are, the sooner it executes.

```c
DELAY = 500 milliseconds + random delay between 0 and 500 milliseconds + REPLICA_RANK * 1000 milliseconds.
```

The replica finally starts the election. It increments currentEpoch by one and requests votes from nodes connected to the Primary, and this request proceeds while exchanging gossip. It sends `FAILOVER_AUTH_REQUEST` to all primary nodes. Then it waits for twice the `NODE_TIMEOUT` (minimum 2 seconds or more).

The primary responds with approval to the replica through `FAILOVER_AUTH_ACK`, and once approval is given, it cannot vote for another replica for twice the `NODE_TIMEOUT`. This restriction doesn't completely prevent split brain, but it helps solve the problem.

When a replica receives `FAILOVER_AUTH_ACK`, it compares currentEpoch and ignores all `FAILOVER_AUTH_ACK` that are smaller. Therefore, only the last delivered vote result is applied. If the replica receives approval from the majority of primaries, it wins the election. If it doesn't receive a majority choice within a certain time, the election is suspended and a new election starts again.

From the primary's perspective, it doesn't calculate which replica is most appropriate. Replicas closer to the primary will start elections faster and are more likely to win the election. If the primary doesn't select the best replica, it doesn't necessarily approve all elections. The primary approves an election only when the following conditions are met:

1. The primary votes only once per epoch, and when approving, it guarantees this by storing a field called lastVoteEpoch on disk. The primary does not vote if the vote request is smaller than lastVoteEpoch.
2. A primary with voting rights votes only if the primary of the replica to be promoted is in a fail state.
3. The primary ignores vote requests smaller than currentEpoch. It also returns the same currentEpoch as the request. Since replicas increase currentEpoch when sending new vote requests, it is guaranteed that approval requests from old elections are smaller than the currentEpoch of new elections. If this weren't the case, delayed vote results could be misunderstood as valid old votes.
