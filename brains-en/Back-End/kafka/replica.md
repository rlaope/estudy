# How Kafka Replication Works

### Kafka Replication Operation

If a Kafka cluster, which acts as the main hub, fails to operate normally, very serious problems can arise. Therefore, Kafka employs a mechanism called **replication** from the initial design phase to ensure stable service operation for high availability.

Kafka prevents data loss and provides flexibility by continuously offering stable service despite broker failures. When creating a topic, you must set the `replication factor` option as shown below.

After creation, execute the describe command for details.

```bash
> --create --topic peter-test01 --partitions 1 --replication-factor 3

[출력]
Create topic peter-test01
> --bootstrap-server peter-kafka01.foo.bar:9092 --topic peter--test01 --describe

[출력]
Topic: peter-test01 PartitionCount: 1 ReplicationFactor: 3 Configs : segment.bytes=1073741824
Topic: peter-test01 Partition : 0 Leader 1 Replicas :1,2,3 Isr : 1,2,3
```

- PartitionCount: Number of partitions in the topic
- ReplicationFactor: Number of replication factors
- Partition : 0 Leader : 1 Replicas : 1, 2, 3, lsr: 1,2,3: The leader refers to broker 1. The replicas are on brokers 1, 2, 3, and the currently synchronized replicas are brokers 1, 2, 3. ISR (In Sync Replica)

If you send a message named 'test message1' to the peter-test01 topic and check the contents of the segment file, you can confirm that all three brokers have the same message.

The `replication factor` option allows administrators to specify the number of replicas. With 'n' replicas, messages can be reliably sent and received without loss, even if up to 'n-1' brokers fail.

Generally, 3 replicas are recommended.

<br>

### Leader, Follower

One of the replicas is elected as the partition leader, and all read and write operations can only be processed through that leader. Producers send messages only to the leader, not to the replicas, and consumers fetch messages from the leader. When a producer sends a message to the peter-test01 topic, only the partition leader can read and write, so messages are sent to the leader, and consumers also receive messages only from the leader of partition 0.

At this time, followers do not simply wait; they prepare to become the new leader at any moment if the current leader encounters problems or issues. Followers check if the partition leader has received new messages, and if so, they replicate those messages from the leader.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FmKT6s%2FbtrAp41OpkG%2Fe9xs5DjUMKP78Jk6Ihajjk%2Fimg.png)

<br>

### Replication Maintenance and Commit

**How does replication work between leaders and followers?**
Leaders and followers are grouped into a logical group called ISR (In Sync Replica). **Only followers belonging to this group are eligible to become a new leader.** Followers within the ISR continuously follow the leader's data to maintain data consistency with the leader, and the leader waits until all followers in the ISR have received the message. However, errors can occur where followers fail to receive data from the leader. If the leadership is handed over to these followers at that point, message loss can occur.

**Besides reading and writing, the leader also checks if followers are performing replication correctly.**
The leader determines if a follower is making replication requests within a specific time period; if not, it assumes the follower has a problem and evicts it from the ISR group.

You can check the status by verifying the ISR list using the `describe` command mentioned earlier.

Once all followers within the ISR have completed replication, the leader marks it as committed (all replicas have stored the message). The position of the last committed offset is called the high water mark. **For message consistency, only committed messages can be read by consumers.**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FwG0gJ%2FbtrAp6k4y5P%2Ff6cfsCffw67BAXhFL68o0k%2Fimg.png)

**What if consumers could read uncommitted messages?**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FwBPZV%2FbtrAoEIQjkL%2FsgaBB349gA3uF0oXekkJGk%2Fimg.png)

**What if a partition leader election occurs while different consumers are consuming messages?**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbf40Y8%2FbtrAoDwoWMI%2F6zfG1oWl7vJFmMm7U7d500%2Fimg.png)

-> If consumers were allowed to read uncommitted messages, inconsistencies could arise even when consuming from the same topic partition. Therefore, **it is implemented so that consumers can only read committed messages.**

**The committed position is important.**

How can we know the committed position? When all brokers restart, the last committed offset position is stored in a file named `replication-offset-checkpoint` on the local disk to preserve committed messages. By checking the contents of this file and comparing it with other replicating brokers, it's possible to identify which broker, topic, or partition has issues.

**We can understand the recovery process through the leader epoch process.**

<br>

### Step-by-Step Replication Operation of Leader and Follower

As seen earlier, the leader is very busy reading and writing numerous messages and monitoring the replication activities of followers. If the leader engages in extensive communication with followers for replication or is heavily involved in replication operations, its performance will degrade, making it difficult to achieve Kafka's advantage of **fast performance.**

-> When handling replication operations between leaders and followers, the design should minimize communication between them to reduce the leader's load.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb5iSfF%2FbtrAtK8L05l%2FB3TldzSdxJ7miGGPQIN2Ck%2Fimg.png)

In the current state, the leader knows that all followers have sent requests to replicate the message at offset 0. However, the leader cannot know whether the followers' replication operation for offset 0 was successful.

**So, how does the leader confirm the followers' replication status?**

While RabbitMQ uses ACKs to confirm message receipt, Kafka eliminated such ACKs to enhance replication performance.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fd0dKy4%2FbtrAs8IOZbc%2Fxrk5VrftPU4ZDkGguj8ONk%2Fimg.png)

The leader receives a new message, message2, from the producer at offset 1 and stores it. Followers who have completed the replication operation for offset 0 request replication for offset 1 from the leader. Upon receiving replication requests for offset 1 from followers, the leader recognizes that the followers have successfully replicated offset 0, marks offset 0 as **committed, and increments the high water mark.**

If a follower fails to replicate offset 0, it will send a replication request for offset 0, not offset 1. Thus, the leader can **check the replication request offsets sent by followers and understand up to which offset followers have successfully replicated.**

Upon receiving replication requests for message at offset 1 from followers, the leader also includes in its response that message1 at offset 0 has been committed.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbe2J0n%2FbtrArnTTDHV%2FEP8wk0p5ILfNFaIdWOwk2k%2Fimg.png)

All followers who receive the leader's response recognize that the offset 0 page has been committed and mark it as committed, just like the leader. Then, they replicate message2, which is at offset 1.

In this way, the leader and followers repeat a series of processes to maintain the latest message state between them within the same partition.

Since Kafka is an application designed to process a large volume of messages, communicating via ACKs, like messaging systems such as RabbitMQ, can lead to performance issues. Therefore, Kafka has the advantage of excluding such communication methods and **focusing solely on the functionality of sending and receiving messages.**

**Furthermore, there is the advantage that replication between leaders and followers is very fast and reliable.**

This is because Kafka operates with followers pulling data, rather than the leader pushing it, which reduces the leader's load.

<br>

### Leader Epoch and Recovery

**Leader epoch** is used to maintain message consistency when Kafka partitions perform recovery operations. (Think of it like Redis's configEpoch, currentEpoch) It is represented as a 32-bit number managed by the controller. Leader epoch information is propagated by the **replication protocol**, and after a new leader is changed, information about the changed leader is delivered to followers. **Leader epoch is utilized as a means to replace the high water mark during recovery operations.**

**Without Leader Epoch**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb7qhBN%2FbtrAtLNpUXZ%2FSFnniNCXZMNYe7K4IrXXXK%2Fimg.png)

Without a leader epoch, the disaster recovery process is as follows.
1. The leader receives message1, stores it at offset 0; followers request to fetch offset 0.
2. Through the fetch request, followers replicate message1 from the leader.
3. The leader increments the high water mark to 1.
4. The leader receives the next message, message2, from the producer and stores it at offset 1.
5. Followers detect the leader's high water mark change in the fetch response for the next message and increment their own high water mark to 1.
6. Followers replicate message2 at offset 1.
7. Followers send a request for offset 2 to the leader, and the leader, upon receiving the request, increments the high water mark to 2.
8. Although the follower replicated message2 at offset 2, it did not receive information to increment the high water mark to 2.
9. Ultimately, the follower goes down ㅠ

When a follower recovers from a failure, the Kafka process initiates internal message recovery operations.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbgky6w%2FbtrAtbFo9vg%2FxVdyTq8Y8k2UgKIMKyZoOk%2Fimg.png)

The above shows the state of a follower recovered from a failure.

1. The follower deems messages higher than its own water mark as unreliable and deletes them. Thus, in the example above, message2 is deleted.
2. The follower sends a fetch request to the leader for new messages at offset 1.
3. At this moment, the broker that was the leader goes down due to a failure, and the sole follower is promoted to leader.

As a result, message2 was lost. (Although this is not a frequent case... still)

**With Leader Epoch**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcaHaOa%2FbtrAuMFjVKQ%2F0fi6fk7SYXru3cyeDwZSn0%2Fimg.png)

In cases without a leader epoch, the Kafka process would immediately delete messages higher than its water mark during recovery. However, with a leader epoch, **instead of unconditionally deleting messages ahead of the high water mark, it sends a leader epoch request to the leader.**

1. During recovery, the follower sends a leader epoch request to the leader.
2. The leader, upon receiving the request, sends message2 up to offset 1 to the follower as part of the leader epoch response.
3. The follower does not delete message2 at offset 1, which is higher than its high water mark. Instead, after confirming the leader's response, it **adjusts its high water mark upwards** to include message2.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbq41kx%2FbtrArdjQqfu%2FqZgUcOgSgmh3XBgPIb6iFK%2Fimg.png)

Through the leader epoch request and response process, the follower's high water mark can be advanced, and no message loss occurred!
