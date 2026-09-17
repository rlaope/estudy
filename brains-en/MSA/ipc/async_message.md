# Asynchronous Message Pattern Application Communication

Messaging is a communication method where services exchange messages asynchronously, and typically, a message broker acts as an intermediary between services.

When a client sends a message to a service to make a request, if the service instance that received the request can respond, it sends a separate message back to the client.

Because it's asynchronous communication, the client doesn't block while waiting for a response, and sends messages assuming it won't receive an immediate response.

### Message Channels
- Messages are exchanged via channels. The sender's business logic calls a sending port interface, which then delivers the message to the receiver through the message channel.
- The receiver's message handler adapter class is invoked to process the message, and this class calls a receiving port interface implemented with the consumer's business logic.

Channels consist of two types.
1. Point-to-point: Delivers a message to only one of the consumers reading the channel.
2. Publish-subscribe channel: Delivers messages to all consumers looking at the same channel (event messages).

<br>

## Messaging Interaction Styles
- Asynchronous Request/Response Style

Clients/services interact in an asynchronous request/response style, exchanging a pair of messages.

When a client sends a command message containing the task to perform and its parameters to a channel, the service processes the request and sends a message containing the result back to a point-to-point channel.

At this point, the two channels are separated into a request channel and a response channel, respectively.

The distinction between messages for each request-response can be made using a MessageId. If the server uses the same CorrelationId in the response message as the MessageId of the processed message, requests and responses can be identified.

- One-way Notification
The client sends only a one-way notification to the service, and the service does not send a separate response.

- Publish/Subscribe

The client publishes messages to a channel read by multiple consumers, and the service publishes domain events that notify of changes to domain objects.

Services subscribe to the event channels of domain objects they are interested in to process messages.

<br>

## Message Broker

A message broker is an infrastructure service that enables services to communicate with each other.

Brokerless messaging is possible without a broker, but it requires a service discovery mechanism, makes it difficult to implement mechanisms like guaranteed delivery, and both the message sender and receiver must be running, leading to lower availability. Therefore, **message broker-based messaging** is widely used.

A message broker is a point through which all messages pass, and senders and receivers do not need to know each other's network locations.

In other words, knowing only the message broker is sufficient for sending/receiving messages.

Message broker products include ActiveMQ, RabbitMQ, and Kafka. Each product has its pros and cons, so you should choose one that fits the characteristics of the application you intend to implement.

### Advantages
- Loose Coupling: Clients simply send requests by sending messages to the appropriate channel. Since clients don't need to know the service instance, a discovery mechanism to inform them of the service instance's location is also unnecessary.
- Message Buffering: A message broker buffers messages until they can be processed. With synchronous request/response protocols like HTTP, both the client and service must be running during the exchange. However, with messaging, messages simply accumulate in a queue until the consumer can process them.

### Disadvantages
- Potential Performance Bottleneck: The message broker risks becoming a performance bottleneck.
- Potential Single Point of Failure: The message broker must have high availability.
- Added Operational Complexity: A messaging system is also a system component that needs to be installed, configured, and operated.

<br>

## Receiver Contention and Message Order Preservation

Can message receivers be scaled out while maintaining message order? While deploying multiple threads and services increases application throughput, concurrent processing requires messages to be processed exactly once and in order.

Suppose there are three service instances reading the same point-to-point channel, and the sender sequentially transmits 'order created', 'order changed', and 'order cancelled' event messages. Simply put, one might think messages could be delivered concurrently to designated receivers by type, but due to network issues or various other problems, if the message processing order is disrupted, the system could malfunction.

To solve this problem, message brokers use sharded channels.

- Sharded channels consist of multiple shards, and each shard operates like a channel.
- The sender specifies a shard key in the message header, and the message broker assigns messages to shards/partitions based on their shard keys.
- The message broker groups receiver instances, treating them as a single logical receiver, assigns each shard to one receiver, and reassigns shards when a receiver starts or stops.

Each order event message has a shard key, and events for a specific order are published to the same shard. Since only one consumer instance reads messages from that shard, message processing order is guaranteed.

<br>

## Handling Duplicate Messages
> This is an important part.

While it would be ideal if a message broker delivered each message exactly once, enforcing this is difficult, and message brokers typically promise at-least-once delivery.

This isn't an issue when the system is healthy, but if the client, network, or broker itself fails, the same message might be delivered multiple times. If a client suddenly fails after processing a message and updating the DB but before acknowledging the message, the message broker will resend the unacknowledged message or send it to a client replica when the client restarts.

While it would be ideal if the message broker guaranteed the original order when resending messages, in reality, it's not easy to resend all related events for a failed event.

**There are two ways to handle duplicate messages.**

- Write Idempotent Message Handlers

A process is idempotent if calling it with the same input multiple times has no additional side effects. Under the assumption that the message broker maintains order during message retransmission, an idempotent message handler can be executed multiple times without issues.

- Method for Tracking and Filtering Duplicate Messages

This can be solved if the consumer tracks message processing status via message IDs and filters out duplicate messages. That is, the consumer can store the message IDs it consumes in a DB table, and when processing a message, record the message ID as part of the transaction that creates/modifies business entities in the DB table.

<br>

## Transactional Messaging

A service publishes messages as part of a transaction that updates the database. That is, if DB updates and message sending are not grouped into a single transaction, a problem arises because the service could crash after the DB update but before the message is sent.

In situations where distributed transactions are not provided, a mechanism is absolutely necessary for applications to reliably publish messages.

### Utilizing DB Tables as Message Queues

For RDBMS-based applications, the transactional outbox pattern can be adopted, which uses a DB table as a temporary message queue.

Create a DB table named OUTBOX in the sending service and insert messages into the OUTBOX table as part of the DB transaction that creates, modifies, or deletes business objects.

Since local transactions guarantee ACID properties, messages are reliably inserted into the OUTBOX table. The message relay mechanism, which reads the OUTBOX table and publishes messages to the message broker, can reliably ensure the situation mentioned earlier.

### Two Methods for Moving Messages from DB to Message Broker

- Event Publishing: Polling Publisher Pattern
The simplest way to publish messages inserted into an OUTBOX table in an RDBMS application is for a message relay to poll the table and query for unpublished messages.

> DB polling can be used for small-scale scenarios, but frequent polling incurs costs. Therefore, for larger scales, the DB transaction log tailing method can be adopted.

- Event Publishing: Transaction Log Tailing Pattern

A method where a message relay tails the DB transaction log.

Since committed updates in an application remain as transaction log entries in each DB, a transaction log miner can read the transaction log and publish changes one by one to the message broker. (Tracks transaction logs without queries, but requires development effort)

<br>

## Improving Availability with Asynchronous Messaging

The problem with synchronous communication is that the client must unconditionally wait until the called service responds, which can degrade application availability.

-> In an MSA, where services are composed of small units, communication costs increase. If synchronous communication is used indiscriminately, availability issues can become more severe. Therefore, synchronous interactions can be eliminated.

### Asynchronous Interaction Style
Synchronous interactions can be eliminated by using messaging-based asynchronous communication to send the final response. That is, instead of requesting and interacting with other services every time data is needed, it's a method of subscribing to events published by that service to maintain up-to-date data.

### Data Replication
Data replicas can maintain up-to-date data by subscribing to events published by the service that owns the data. That is, instead of requesting and interacting with other services every time data is needed, it's a method of subscribing to events published by that service to maintain up-to-date data.

While it can be useful in cases where business logic is written based on data from other services, creating large-scale replicas is inefficient.

### Finishing After Returning Response
To eliminate synchronous communication during request processing, requests can be handled as follows.

- Validate the request based on locally available data (i.e., validate the request based on local state through data replication).
- Update the DB by inserting messages into the OUTBOX table.
- Return a response to the client.
