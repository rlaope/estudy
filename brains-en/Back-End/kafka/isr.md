# Kafka ISR(In Sync Replica)

### Kafka Replication Factor

This is a configuration value that tells brokers to replicate partitions within a topic by the number of replication factors. Because replicas are created, the concepts of leader and follower (master, slave) exist, and most importantly, all read and write operations occur through the leader.

#### Leader, Follower

![](https://user-images.githubusercontent.com/45676906/211140249-e31556b5-ebf6-443b-a455-e3d08ad82940.png)

The leader handles all data read and write operations, and followers periodically monitor the leader to sync data. This method maintains replication.

### ISR

When a producer publishes a message to the leader, followers replicate that message. If a follower has an issue and fails to replicate properly, and then the leader fails and a failover occurs, a **data consistency issue** arises.

To prevent this, Kafka introduced the concept of `ISR (In Sync Replica)`.
You can think of ISR as the current replication group that is being replicated.

**Let's learn about the ISR rules.**
1. Only members belonging to the ISR can qualify as a leader. Managed as a Replica group, if a member within the ISR group is unable to perform its role as leader (e.g., unexpectedly crashes), a **follower is elected as the leader** and takes over that role.
2. The leader of the ISR group checks if followers are periodically verifying data for a **certain period (replica.lag.time.max.ms)**. If no requests are received, the leader detects an anomaly in that follower, determines that the follower can no longer act as a leader, and expels that follower from the ISR group. A follower expelled from the ISR simultaneously loses its leader qualification.
3. To perform these operations, the **data synchronization process** between the ISR leader and followers is handled with great importance. By managing partitions through ISR, the reliability of replication is enhanced.

![](https://user-images.githubusercontent.com/45676906/211140369-36f84961-662a-4c93-b636-80bd4109db57.png)

When a Producer sends a message to the leader, followers replicate data from the leader. In this example, let's assume Broker2 successfully replicated, but Broker3 failed to replicate.

First, the leader broker sends an ack to the producer.

![](https://user-images.githubusercontent.com/45676906/211140935-8450b400-bdd9-402c-a119-7b7f2a8d587c.png)

If no verification request is received within `replica.lag.time.max.ms`, the problematic follower is removed from the ISR group.

![](https://user-images.githubusercontent.com/45676906/211140553-6bf78afc-a3c3-42f2-a258-87cd281a7800.png)

Below is the ISR structure for `logtopic003` with `partition3` and `replica2`.

```bash
bin/kafka-topics.sh --describe --bootstrap-server localhost:9092 --topic logtopic003 
Topic: logtopic003 PartitionCount: 3 ReplicationFactor: 2 
Configs: 
	Topic: logtopic003 Partition: 0 Leader: 2 Replicas: 2,1 Isr: 2,1 
	Topic: logtopic003 Partition: 1 Leader: 3 Replicas: 3,2 Isr: 3,2 
	Topic: logtopic003 Partition: 2 Leader: 1 Replicas: 1,3 Isr: 3,1
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fcdk7z6%2FbtqEamKhmSA%2FbjbSXdRf5xaknJOTbMV9Wk%2Fimg.png)
