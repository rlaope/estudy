# Kafka Rebalancing

Kafka consumers are responsible for processing messages from each partition of a topic.

However, if a specific consumer encounters an issue, ownership of the partitions it was processing is transferred to another consumer.

This process is called **rebalancing** and typically occurs in the following situations:

1. When a new consumer is added to the consumer group (+)
2. When an existing consumer leaves the group (-)
3. When a new partition is created in a subscribed topic
4. When the topic a consumer is subscribed to changes

The most common situation where rebalancing frequently occurs is during application deployment.

This is because when an application is deployed, the existing application shuts down, and then the new application starts, leading to the deletion of old consumers and the creation of new ones. During this process, rebalancing occurs at least twice.

Rebalancing comes with the following issues:
- **Downtime**: Messages are not processed while rebalancing is in progress. This causes application latency.
- **Message duplication/loss issues**: During the rebalancing process, it's determined from which position in a partition messages should be read. In this process, messages can be duplicated or lost.

<br>

## Partition Assignment Strategy

The Partition Assignment Strategy refers to the method by which Kafka consumers determine which partitions of a topic to consume.

This strategy defines the relationship between consumers and partitions, affecting data processing efficiency and performance.

Kafka's partition assignment is divided into two types: **eager rebalancing** and **cooperative rebalancing**.

Accordingly, there are four partition assignment strategies:

1. Range Partition Assignment Strategy
2. RoundRobin Partition Assignment Strategy
3. Sticky Partition Assignment Strategy
4. CooperativeSticky Partition Assignment Strategy

### Eager Rebalance

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbjMAGZ%2FbtsC4KsCUrd%2FO2ydf2K2gYhRWvfsuK7GMk%2Fimg.png)

The Range, RoundRobin, and Sticky assignment strategies fall under the eager rebalancing assignment strategy.

When a rebalance occurs, all consumers stop receiving data from Apache Kafka and relinquish their group's partition assignments.

During this process, a Stop-The-World (STW) event occurs where all consumers simultaneously halt their operations. This temporarily suspends data processing for the entire consumer group.

During this time, the consumer group becomes idle, neither consuming messages nor allowing offset commits. However, producers continue to write to partitions regardless of the rebalancing, leading to a rapid increase in **LAG** during the waiting period.

This can temporarily impact the data processing performance of the consumer group.

After rebalancing, consumers rejoin the group and are assigned new partitions. It's important to note that there is **no guarantee that consumers will necessarily receive the same partitions they had before.**

In other words, after rebalancing, consumers are likely to be assigned new partitions. Therefore, eager rebalancing groups should be used cautiously, as they can affect the group's stability and data processing performance.

### Cooperative Rebalance (Incremental Rebalance)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbfNaii%2FbtsC4G4UlwS%2FCgiceJq9OOhzm8rYQttcyK%2Fimg.png)

This approach corresponds to the CooperativeSticky partition assignment strategy. Introduced in `Kafka v2.4`, this is a relatively recent and advanced rebalancing method.

This method is characterized by **only a portion of partitions moving from one consumer to another.**

This allows other Kafka consumers, not directly involved in the rebalancing, to continue processing data, minimizing the impact on the overall group's performance.

It's a process of gradually finding a stable partition assignment state, involving several rebalancing iterations.

1. In the first phase, the leader informs all consumers that they will lose ownership of some partitions.
2. In the second phase, consumers relinquish ownership of these partitions.
3. In a subsequent phase, the coordinator assigns these orphaned partitions to new consumers.

This method requires several iterations until a stable partition assignment state is achieved.

However, it avoids service interruptions that occur with eager rebalancing, making it a particularly important method for large-scale consumer groups where rebalancing can take a significant amount of time.

Therefore, cooperative rebalancing offers the advantage of flexibly adjusting partition assignments while maintaining the overall group's performance.

Nevertheless, this method should also be approached cautiously, and the appropriate strategy should be chosen based on the group's specific circumstances.
