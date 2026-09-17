# Kafka Partition

Kafka partitions play a crucial role in efficiently processing data.

Each partition forms an independent data stream, allowing messages to be processed in parallel.

This parallel processing is a key factor in maximizing Kafka's performance.

Partitions are the fundamental units that divide queues to enable parallel processing, and each topic is divided into one or more partitions.

This allows messages to be processed in parallel.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcBlHFJ%2FbtsCKry4TVy%2FWogosuf42VtkwqxBKvtV31%2Fimg.webp)

Kafka Partition Structure

Partitions have a log structure that guarantees order. Thanks to this structure, each message **has a unique offset within the partition, which allows for precise identification of the log's position.**

<br>

### Log Structure Guaranteeing Order

Kafka partitions fundamentally operate like a **log file**. In this log structure, each message is stored sequentially, and each message is assigned a **unique offset** value. This offset indicates the message's position and is used to precisely identify messages within the Kafka system.

#### Understanding a Partition's Log Structure
- **Starts from offset 0**: In Kafka, each message in a partition is assigned a sequential offset number starting from 0.
- **Offset increases with message addition**: Whenever a new message is added to a partition, it is assigned an offset value that is 1 greater than the offset of the last message. For example, if message B is stored after message A, message B will be assigned offset 1.
- **Order Guarantee**: Thanks to this log structure, messages within a partition are always stored sequentially, and the order of each message is guaranteed by its offset value.

```
| Offset | Message |
|--------|-------------|
| 0 | Message A |
| 1 | Message B | 
| 2 | Message C | 
| 3 | Message D | 
| 4 | Message E |
```

**Why is order guarantee important?**

Since the order of messages is guaranteed, consumer data can be processed in the correct sequence. This is **crucial for processing time-series data, transaction records, and other data where order is important.** Furthermore, if an issue arises with a specific message, its offset allows for precise identification of its location and problem analysis.

Advantages of partitions include **parallel processing**, which significantly enhances speed by allowing various consumers to read data from different partitions simultaneously; **order guarantee**, which is useful when sequential data processing is required as message order is preserved within a single partition; and **scalability**, as partitions can be distributed and stored across multiple nodes.

<br>

### Kafka High Availability Mechanism Through Partition Replication

**Kafka Partition Replication Concept and ISR**
- A Kafka cluster consists of multiple brokers. Within this cluster, each partition is replicated to one or more brokers.
- For example, if Partition 1 for a topic is primarily stored on Broker 1, replicas of this partition can also exist on Broker 2 and 3. This means that partition data is distributed and stored across multiple brokers within the cluster.

**What is the difference between partition replication and intra-broker replication?**

Kafka partition replication refers to replication that occurs between different brokers within a cluster. Each broker communicates with others over the network, replicating and synchronizing partition data. In contrast, replication occurring within a single broker is not Kafka's standard mode of operation. In Kafka, replicas of each partition must be distributed and stored across different brokers.

In conclusion, Kafka's high availability is achieved by replicating partition data across multiple brokers within the cluster, which is a different concept from replication occurring within a single broker.

**Leader and Follower**
- Leader: Each partition has a single leader, and all read and write operations occur through this leader.
- Follower: The remaining replicas, excluding the leader, are followers. They replicate the leader's data to increase system availability. If the leader goes down, a follower can be promoted to a new leader.

**Data Loss Prevention and Service Continuity**
- If one broker fails, another broker uses its replica of that partition to prevent data loss and maintain service continuity.

Kafka ISR (In Sync Replica) refers to the set of followers that are synchronized with the leader. This can be seen as a group of followers that can be promoted to a new leader.

<br>

### Kafka Partition Distribution Structure

In Kafka, a single topic is divided into multiple partitions for storage. These partitions can be distributed and stored across different brokers within the cluster. For example, if there is a topic named 'topic 1', Partition 1 of this topic might be stored on Broker 1, and Partition 2 on Broker 2.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fd9MXZq%2FbtsCzTRqUIE%2FaSkMSv3aZf02VqA9kqWzcK%2Fimg.webp)

Kafka Partition Distribution Structure

**Enhanced Parallel Processing**
- By having each broker manage different partitions, multiple consumers can simultaneously read from or write to partitions on different brokers. This improves the overall system's read/write throughput.
- For example, consumer A can read data from Partition 1 on Broker 1, while consumer B simultaneously reads data from Partition 2 on Broker 2.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNwiep%2FbtsCGaxPWB4%2FsX4haMv03LEnzvkqiyrqE0%2Fimg.webp)

Kafka Partition Parallel Processing

This distributed processing method enables the Kafka cluster to process more data quickly.

Since each broker performs operations independently on its own partitions, the overall system's load is balanced, and processing capacity increases.

<br>

### Order Guarantee Mechanism within Partitions

In Kafka, each partition maintains an independent message stream. Within this stream, messages are ordered according to the sequence in which they are added.

For example, Partition 1 receives messages 1 to 6 as shown below, and Partition 2 also independently receives messages 1 to 6.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdaWrzi%2FbtsCF87PbqE%2FSiO2oWZcHppetEKrPFl3IK%2Fimg.webp)

Kafka Partition Order Guarantee

- Partition1: msg1 2 3 4 5 6
- Partiton2: msg1 2 3 4 5 6

As shown above, messages are stored and processed with guaranteed order.

In this example, each partition stores messages in the order they arrive and processes them in that same order. This means Kafka is suitable for processing time-series data or log data where order is critical. For instance, if msg1 represents the start of a transaction and msg6 represents its end, Kafka can process and store these events in the exact correct sequence.

What if data arrives out of order?

For example, let's assume messages arrive as 2 5 1 4 5 6. In this case, 2 would be processed first, followed by 5, then 1, and so on.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbymnuD%2FbtsCJkNIawR%2FGLiCGUDIRIe3ob911L8Xc0%2Fimg.webp)

Kafka Partition Out-of-Order Message Processing

The meaning of **order guarantee** is that messages are processed in the order they are stored in the partition. This implies that the order in which the producer sends messages is important; if they are sent out of order, Kafka will process them in that received order.

Therefore, **it is crucial for the producer to manage the message sending order. Especially for time-series data or when transaction order is critical, messages must be sent in the correct sequence.**

In conclusion, message order guarantee in Kafka partitions means that messages are processed in the order they are stored. The order in which the producer sends messages determines the processing order within the partition. Therefore, if messages are sent out of order, Kafka stores and processes them in that received order.

<br>

### Kafka Partition Considerations

**Importance of Key Selection**
- The choice of key used by the producer when assigning data to partitions is crucial. An incorrect key selection can lead to imbalanced data distribution and concentrate load on specific brokers.
- A solution to this is to clearly define a strategy to ensure a uniform hash distribution of keys.

**Balanced Distribution**
- In a Kafka cluster, each broker manages one or more partitions. At this point, the distribution of keys assigned to each partition is important. When partitions managed by each broker are evenly allocated, it means that the hash values of the keys are uniformly distributed among the partitions.

The example image is as follows. You can see that it is allocated in a balanced manner.
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FvKYNy%2FbtsCEI2FkUX%2FzPFL08KafSZocZztNGMFyK%2Fimg.webp)

Kafka Partition Balanced Distribution

This ensures that data for each partition is evenly distributed, balancing the load across the entire cluster.

In conclusion, all brokers in the cluster are utilized efficiently, improving overall system performance and stability.

**Imbalanced Distribution**
- If keys are concentrated in specific Kafka partitions, an imbalanced distribution occurs. An excessive concentration of keys in certain partitions means other partitions are not fully utilized, leading to performance issues.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbfiL2z%2FbtsCIEk68WN%2FLK3KCuEKkZkXhsXEpSB3DK%2Fimg.webp)

Kafka Partition Imbalanced Distribution

Thus, key selection in Kafka significantly impacts overall performance and stability. A balanced key distribution ensures efficient utilization of each partition and broker, while an imbalanced distribution can lead to cluster performance degradation and stability issues, making careful consideration of key distribution strategy essential.

**Consumer Group Management**
- Within a single consumer group, the number of consumers cannot exceed the number of partitions. If it does, some consumers may not process data, leading to inefficient resource utilization. To prevent this, it is crucial to balance the number of consumers in a group with the number of partitions.

**Complexity of Partition Reassignment**
- While increasing the number of partitions is easy, reducing it can be complex and risky. Therefore, it is important to carefully decide the number of partitions during the initial design phase.
- A good approach is to conservatively set the number of partitions initially and adjust it as the system grows.
