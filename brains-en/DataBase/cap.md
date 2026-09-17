# CAP Theorem

### What is the CAP Theorem?
The CAP theorem (or Brewer's theorem) states that a distributed **database system connected via a network can satisfy only two of the three characteristics: Consistency, Availability, and Partition Tolerance, and cannot satisfy all three**. Let's explore each component.

### Consistency
**Consistency means that when requesting any data, the response will either return the most recently changed data or return a failure**. In other words, for all reads, DB nodes must always have the same data.

If a DB maintains two instances, it means that the same value is returned whether you request it from 'a' or 'b'. This implies that if someone performs a READ before synchronization after an update to 'a' or 'b', the data is not returned immediately but processed after all nodes are synchronized.

### Availability
**Availability means that all requests receive a normal response**. In other words, **even if some cluster nodes fail, operations such as READs and WRITEs must always return successfully**.

### Partition Tolerance
**Partition tolerance means that the system must continue to operate even if communication failures occur between DB nodes**. Let's say there are instances 'a' and 'b', and a network failure occurs between them. A user queries DB 'a', and instance A operates independently, even though it doesn't know the state of B. This is called partition tolerance.

### Availability VS Partition Tolerance
The concepts of availability and partition tolerance can be a bit confusing. Personally, I've understood **availability as when a node fails and goes down**, while partition tolerance is **when communication between nodes is not functioning normally**.

From a client's perspective, there's a difference. In the former case, if there are clients 'a' and 'b', both use the same non-failing node. In the latter case, each client might be looking at a different node.

<br>

## Real-world DB Comparison

![](Image/cap.png)

This is a triangular graph showing which two characteristics a DB possesses according to the CAP theorem. Let's look at a few DB examples.

### MySQL
MySQL falls into CA in the triangular graph above. However, in practice, it can belong to either CP or CA depending on the configuration. When MySQL is used as-is, it typically falls into CA. This is because it uses a paradigm where there is a main master node and slave nodes that replicate it. In this scenario, MySQL satisfies availability and consistency.

**However, MySQL supports cluster configuration. With this setting, MySQL becomes a system that satisfies CP**. This is because if there are no cluster nodes to maintain the data, the cluster will terminate.

### DynamoDB
DynamoDB is a key/value NoSQL database supported by AWS, known to belong to AP. Dynamo hashes the key value, then mods this value to store it on the appropriate server. It also replicates data to some nodes among all nodes and performs data versioning. **DynamoDB allows for strongly consistent settings**. If this setting is enabled, DynamoDB transforms into a CP system. With this setting, the most recent data is guaranteed to be returned. Consequently, availability might be slightly reduced because it searches all nodes.
