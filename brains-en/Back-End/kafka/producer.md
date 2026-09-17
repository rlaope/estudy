# Producer Key Options

### bootstrap.servers

This represents a list of host and port information for initial connection to the Kafka cluster.

### acks

This is the **number of acknowledgments (acks) the producer waits for before completing a request after sending a message to the leader** of a Kafka topic.

- **ack=0**: The producer does not wait for any ack from any server. Since it doesn't wait for an ack request, messages can be sent very quickly, but there is a high possibility of message loss.
- **ack=1**: The leader records the data. However, not all followers are confirmed, so message loss may still occur.
- **ack=all**: The leader waits for an ack for the data from the followers in the ISR, so as long as there are followers, strong guarantees against data loss can be provided. For perfect usage, broker settings must also be adjusted.

The default value is ack=all.

### Relationship between ack=all and broker min.insync.replicas options

The `min.insync.replicas` option **specifies the minimum number of replicas that must acknowledge a write for a message to be considered successful.** In other words, it can also be described as the minimum number of replicas that must be maintained.

#### ack=all, min.insync.replicas=1

![](https://user-images.githubusercontent.com/45676906/211141530-4f964427-8d8d-4760-92af-73af12bddd36.png)

Since the min.insync.replicas option is 1, when the producer sends a message to the leader, the leader only needs to confirm that at least one broker has successfully received the message. That is, if it has received it successfully itself, it immediately sends an ack response.

#### ack=all, min.insync.replicas=2

![](https://user-images.githubusercontent.com/45676906/211141989-3d3b4e54-491d-4259-a167-67fab09ab38e.png)

The producer sends a message to the leader. The leader receives and stores the message, and the followers fetch and store that message. The leader confirms with the followers that the message has been successfully replicated. Since the **min.insync.replica** option is 2, the leader confirms with itself and one follower, then sends an acks response to the producer.

#### ack=all, min.insync.replica=3

![](https://user-images.githubusercontent.com/45676906/211142238-ef4c58a3-d488-41f5-9ebb-4663d9642feb.png)

The leader receives and stores the message from the producer, and the followers fetch and store that message. The leader confirms with the followers that the message has been successfully replicated. Since the min.insync.replica option is 3, it confirms with itself and two followers, then sends acks.

The recommended options here are `ack=all` and `min.insync.replicas=2`.

The reason is that if the min.insync.replicas option is 2, a problem with one broker does not lead to a cluster-wide failure. (When one replica has a problem, it is excluded from the ISR group.) However, when using 3, a problem with one broker can lead to a cluster-wide failure. Therefore, it is recommended to apply min.insync.replicas option 2 with ack=all.
