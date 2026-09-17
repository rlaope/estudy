# Why Zookeeper Adopted Paxos

[This article](https://github.com/rlaope/estudy/blob/master/brains/Back-End/paxra.md) explores the clear facts about why distributed coordinator systems like Zookeeper adopt this operational approach and their internal implementations, despite Paxos having critical drawbacks such as mathematical limitations, fatal issues like livelock, system downtime, and availability sacrifices.

From the CAP (Consistency, Availability, Partition Tolerance) perspective of distributed systems, Zookeeper is thoroughly designed as a **CP (Consistency & Partition Tolerance) system**.

In other words, when network failures occur, stopping the system (sacrificing availability) rather than serving incorrect data aligns better with the purpose of distributed coordination.

**Preventing catastrophic split-brain**: Zookeeper is primarily used for distributed locks and dynamic configuration management. If a node group that has lost quorum majority continues to allow write requests after network partition, a disaster occurs where two different servers acquire the same lock, resulting in complete data integrity collapse in payment systems or inventory management.

**Stability first**: As seen in the FLP impossibility theorem, systems must compromise between safety and liveness. Zookeeper chooses to guarantee 100% data order and consistency (safety) even if infinite waiting (availability degradation) occurs, by enforcing majority node confirmation in all consensus processes.

In other words, Zookeeper chose C over A in the CAP tradeoff.

> FLP impossibility is the concept that in asynchronous network environments, even if just one node goes down, there exists no deterministic consensus algorithm that can simultaneously satisfy 100% of both Safety and Liveness.

<br>

## Zookeeper

Zookeeper is a powerful synchronization and state management service for distributed applications.

Summarizing the core concepts and actual operational commands:

### Core Data Model: ZNode

Zookeeper stores data in a hierarchical tree structure called ZNode, similar to a Linux filesystem, maintained in memory to provide very fast read speeds.

- **Persistent Node**: The default node type that persists until explicitly deleted.
- **Ephemeral Node**: Automatically deleted when the client session that created the node is disconnected (network disconnection). Critically used for **service discovery** and node health checks in distributed environments.
- **Sequential Node**: Assigned sequentially increasing numbers when created, used to create queues when implementing distributed locks.

### Watcher Mechanism

When a client sets a Watcher on a specific ZNode, Zookeeper pushes events to the client when that node's data changes or child nodes are created/deleted. This eliminates the polling overhead of clients periodically checking for changes.

### Zookeeper CLI Command Examples

To aid technical understanding, here are commonly used shell commands:

```bash
# 1. Create persistent node (for storing environment variables)
[zk: localhost:2181] create /app_config "db_port=3306"

# 2. Create ephemeral node (register currently alive web server #1)
# With -e option, /live_servers/web_01 node is automatically deleted when client session ends
[zk: localhost:2181] create -e /live_servers/web_01 "192.168.1.10"

# 3. Query data and set Watcher
# With -w option, receive notification when /app_config data changes
[zk: localhost:2181] get -w /app_config
```

### Actual Implementation of Paxos and Detailed Usage in Zookeeper

Since theoretical Basic Paxos single-value consensus cannot handle persistent data, the industry actually implements and uses **Multi-Paxos** or its variant algorithms.

Representative Paxos implementations in the industry include Google Chubby and Spanner:

- **Google Chubby**: Google's distributed lock service that successfully implemented the Paxos algorithm in production. It's the direct inspiration for Zookeeper. Chubby consists of cells with 5 replicated servers, using Multi-Paxos to elect leaders and replicate data.
- **Google Spanner**: Google's globally distributed database that uses Paxos consensus groups per shard to synchronize transactions across data centers thousands of kilometers apart.

### Zookeeper's Implementation: ZAB (Zookeeper Atomic Broadcast)

Zookeeper doesn't use pure Paxos as-is, but independently implements a protocol called ZAB, specialized for database transaction processing that addresses logical limitations.

The reasons are as follows:

- **State Machine Replication order guarantee**: Multi-Paxos has a structural blind spot where when multiple proposals are agreed upon simultaneously, the order can be reversed as long as consensus is ultimately reached.
- **ZAB's solution (FIFO guarantee)**: Zookeeper must guarantee that previously processed transactions are replicated before subsequent transactions. ZAB ensures that only the elected leader issues transaction IDs (zxid) sequentially, and strictly enforces FIFO message order in the broadcast phase using TCP characteristics.

Let's explore more about how the livelock problem was addressed. The reason livelock occurs in basic Paxos is **because anyone can be a proposer simultaneously**. The core issue is the Dueling Proposers phenomenon where two proposers send prepare requests with higher numbers, interfering with each other's progress.

- **ZAB and Raft** established the rule that only one proposer exists at a time.
- While a leader is elected, other nodes don't make proposals but follow the leader's commands as followers. With only one proposer, there's no mutual interference, allowing the system to complete consensus without interruption and secure liveness.
- This implementation is achieved through cross-validation that accepts only proposals exceeding majority votes through voting, rather than leaving it to chance (proposal timing). Performance decreases accordingly, but accuracy increases.

### ZAB Implementation Process

1. **Discovery**: Leader election process where multiple servers elect the server with the highest `zxid` (most recent data) as leader through voting. (This process is similar to Paxos phase 1 prepare)
2. **Synchronization**: The newly elected leader synchronizes data with followers. Once synchronization completes, the leader regime is established.
3. **Broadcast (consensus)**: Similar to Paxos phase 2 accept. When a client write request arrives, the leader creates a transaction and sends it to followers. When a majority of followers respond that they've received it, the leader issues a commit command to finalize data storage.
