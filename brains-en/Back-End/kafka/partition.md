# Kafka Partition Assignment Strategies

Kafka's rebalancing approaches are categorized into eager rebalancing and cooperative rebalancing. There are four main partition assignment strategies:

1. Range Partition Assignment Strategy - Eager Rebalancing
2. RoundRobin Partition Assignment Strategy - Eager Rebalancing
3. Sticky Partition Assignment Strategy - Eager Rebalancing
4. CooperativeSticky Partition Assignment Strategy - Cooperative Rebalancing

Let's explore each one.

### Range Partition Assignment Strategy

The Range partition assignment strategy was the default eager rebalancing partition assignment strategy prior to Kafka v2.4.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdzHrep%2FbtsDawsTnD5%2F1HIQI79qC8TQGW95raqwx0%2Fimg.png)

The Range partition assignment strategy follows this process:

1. It lists the subscribed partitions and consumers in order.
2. Then, it determines the number of partitions each consumer should receive, calculated by dividing the number of partitions in the topic by the total number of consumers in the consumer group.
3. If the number of consumers and partitions exactly match, all consumers receive an equal allocation of partitions.
4. If the number of partitions cannot be evenly divided by the number of consumers, consumers earlier in the order receive additional partitions.

**In the image above, each topic has 2 partitions and there are 3 consumers. Therefore, dividing 2 by 3 results in 2/3, meaning one consumer does not receive any partitions.**

One of the major advantages of this strategy is that data processing for a specific domain can be consistently managed by a single consumer. For example, consider a real-time log analysis system where topic A manages log data and topic B manages error information. With the Range strategy, two types of data with the same partition number are assigned to the same consumer.

This allows a single consumer to process and analyze log data and its corresponding error information together, maintaining data consistency and improving processing efficiency.

### RoundRobin Partition Assignment Strategy

It is one of the eager rebalancing partition assignment strategies, which **distributes partitions evenly among all consumers in the consumer group.**

Similar to Range, partitions and consumers are sorted in lexicographical order before assignment.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdnerLh%2FbtsC6UIhRJH%2FghmxyleQX0etDbMzEAht70%2Fimg.png)
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdnerLh%2FbtsC6UIhRJH%2FghmxyleQX0etDbMzEAht70%2Fimg.png)

Its advantage lies in effectively utilizing all available consumers and improving performance.

That is, since all consumers process tasks distributed evenly, efficient resource utilization is possible.

**However, a major drawback of this strategy is that it does not attempt to minimize partition movements when the number of consumers changes (i.e., when rebalancing occurs).** For example, if a specific consumer loses its connection, the assigned partitions must be reallocated to other consumers. Such unnecessary partition movements can affect consumer performance.

In the image above, if Consumer 2's connection is lost, you can see partitions A-1, B-0, B-1 being assigned.

As a result, a situation occurs where 3 out of 4 partitions are reassigned to other available consumers.

Consequently, unnecessary partition movements negatively impact performance because consumers require additional resources to fetch and process data from new partitions.

<br>

### Sticky Partition Assignment Strategy

It is one of the eager rebalancing assignment strategies. When rebalancing is required, this strategy **prioritizes mapping consumers' partition information from before the rebalancing operation to minimize unnecessary partition movements during the rebalancing process.**

![](https://blog.kakaocdn.net/dn/bICYBw/btsC32tyfXB/TtiaTa1YtQ2Fv7KKyJe1P0/img.png)
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcnwtxF%2FbtsDdBHtMi9%2FnbopYdBoPFvLe3saacYDIk%2Fimg.png)

This improves upon the issue of unnecessary partition movements that occurred with the RoundRobin approach.

It minimizes partition movements while maintaining as balanced an assignment as possible among available consumers.

For example, in the image above, if Consumer 2's instance goes down or fails, Consumer 1 and Consumer 3 retain their previously assigned partitions, and the partitions held by Consumer 2 are assigned to the new Consumer 3. By doing so, it minimizes unnecessary partition movements without moving everything, thereby maintaining system balance.

However, the Sticky partition assignment strategy does not always retain existing partitions and consumers. This is because the strategy's top priority is to maintain as balanced a partition assignment as possible, and its secondary goal is to preserve as much of the existing partition assignment information as possible when rebalancing occurs.

Therefore, the Sticky partition assignment strategy enables efficient rebalancing by minimizing unnecessary partition movements while maintaining balanced throughput. In environments where this strategy is applied, partition balance and the efficiency of the rebalancing process become crucial factors.

### CooperativeSticky Partition Assignment Strategy

This strategy adds a new concept, cooperative rebalancing, to the existing Sticky partition assignment strategy.

This strategy focuses on individual consumers rather than the entire consumer group, allowing for more flexible and efficient rebalancing.

Cooperative rebalancing focuses only on specific partitions that require rebalancing, while the remaining partitions are kept as they are. That is, **it only stops consumption for specific partitions that require consumer reassignment, while other remaining partitions continue to consume data.** Minimizing work interruption for individual consumers rather than the entire consumer group offers advantages in overall data processing performance.

Therefore, the CooperativeSticky partition assignment strategy allows consumers to continue consuming data from partitions that are not being reassigned, thereby minimizing the impact of rebalancing and enabling efficient rebalancing.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbkpvOy%2FbtsC6p9FRGi%2F2hBrcRlPP9lsh4HyS4MQHK%2Fimg.png)

**It is particularly useful in environments where consumer group configurations change frequently, and it quickly adapts to situations requiring the addition or removal of consumers or the reassignment of partitions, performing efficient rebalancing.**

Furthermore, it supports both `dynamic membership` and `static membership`, allowing for dynamic and flexible adjustment of consumer group configurations according to application requirements.

Through all these characteristics, the CooperativeSticky partition assignment strategy minimizes data processing delays during rebalancing and enables fast and efficient rebalancing, thereby supporting overall consumer group performance improvement and stability.
