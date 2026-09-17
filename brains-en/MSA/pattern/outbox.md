# Transactional Outbox Pattern

### Transactional Event

Services we develop typically publish messages alongside transactions that update the database.

Problems can arise if the database update operation and message transmission are not grouped into a single transaction.

This is because if these two operations (DB update, event publish) are not performed atomically within the service, issues can occur during system failures.

![](https://velog.velcdn.com/images/eastperson/post/d068766f-1f11-451a-83d8-fc4a7187d8b4/image.png)

DB 업데이트와 메시지 발행이 원자적으로 수행되지 않을 경우 발생할 수 있는 문제

**2PC(Two Phase Commit)**

Traditionally, distributed transactions were used to ensure atomicity. Distributed transactions use 2PC to ensure atomicity by allowing transaction participants to commit or roll back. However, distributed transactions can suffer a performance degradation of more than 10 times compared to a single database. Furthermore, being a synchronous IPC (Inter-Process Communication) form, it also has availability issues.

Modern architectures prioritize availability over consistency. For these reasons, distributed transaction methods are not suitable for modern applications, and contemporary message brokers do not offer such features.

We need to ensure the following two things:
1. Messages must be published before the database transaction commits. Conversely, if the transaction rolls back, messages are no longer sent.
2. The message service must send messages to the broker in the order they were sent.

<br>

### Transaction Outbox Pattern

To solve the problems that can arise regarding transaction and message publishing atomicity mentioned above, we can consider a method of storing messages in the database as part of the transaction that updates the DB.

Then, a separate process or batch program reads the stored events and sends them to the message broker. This is the **Outbox Pattern.**

The application stores message content in an outbox table within the database. Other applications or processes can read data from the outbox table and use that data to perform operations. In case of failure, it can be re-executed until completion.

Therefore, the outbox pattern can ensure that messages are successfully sent at least once.

![](https://velog.velcdn.com/images/eastperson/post/c67e25eb-6f91-467e-a31e-6b3e3108b4d8/image.png)

Outbox Pattern

![](https://velog.velcdn.com/images/eastperson/post/3ff5b14d-f0bf-4dce-bcb3-e8d8c14eddfe/image.png)

Outbox Pattern

Here, 'outbox' means a mailbox for sending. It refers to a storage location where unsent or failed-to-send messages are collected. It's a separate store for data to be sent as messages.

![](https://velog.velcdn.com/images/eastperson/post/8315f545-08a9-4aec-8739-1f95a2cf2a76/image.png)

Outbox Pattern

The components of this pattern are as follows:
- Sender - The service that sends messages
- Database - The service that stores entities and the message Outbox
- Message Outbox - A table for storing messages in the case of a relational DB, or a property of a database record in the case of NoSQL
- Message relay - A service that sends messages stored in the outbox to the message broker

In this pattern, a separate process called Message Relay is added. The outbox table acts as a temporary message queue and is grouped transactionally with entity updates. The Message Relay asynchronously reads the data stored in the outbox table, publishes the messages, and delivers them to the message broker.

There are two ways to implement the message relay for the outbox pattern: Polling publisher and Transaction log tailing.

<br>

### Polling Publisher Pattern

A simple way to publish messages inserted into an outbox table in an RDBMS application is to poll the table to query for unpublished data.

The message relay sends each queried message one by one to its respective destination channel, publishing it to the message broker. Then, it deletes the message from the outbox table.

![](https://velog.velcdn.com/images/eastperson/post/e4787feb-f33b-460e-9b76-b8a688197c44/image.png)

Polling Publisher Pattern

DB polling is an easy method to use for small-scale scenarios. However, frequent DB polling incurs costs, and the feasibility of using NoSQL DBs depends on their query capabilities.

<br>

### Transaction Log Tailing Pattern

This method involves the message relay tailing the DB transaction log (commit log).

Committed updates from the application remain as transaction log entries in each DB.

This method involves a transaction log miner reading the transaction log and publishing each change as a message to the broker.

![](https://velog.velcdn.com/images/eastperson/post/e7044be2-03cb-444b-a5b8-7870bbef4863/image.png)

Transaction Log Tailing Pattern

In the case of MySQL, the transaction log miner reads transaction log entries and converts each log entry corresponding to an inserted message into a message, publishing it to the message broker.

It can publish messages output to an RDBMS outbox table or messages added to a record in a NoSQL DB.

It can be implemented by reading changes using tools like MySQL's mysqlbinlog, PostgreSQL's WAL, or Oracle's redolog, or due to implementation complexity, specialized tools may be used. Related tools include Debezium, LinkedIn Databus, DynamoDB Streams, and Eventuate Tram.

<br>

### Outbox Pattern With Kafka Connect

Kafka-Connect runs as a separate service outside of the Kafka broker. The diagram below uses PostgreSQL and shows records being added to the outbox table when entity updates occur. Kafka-Connect **deploys a Debezium connector to capture database changes at runtime.** Debezium tracks the write-ahead log of the outbox table database and publishes topic messages defined by a custom connector.

![](https://velog.velcdn.com/images/eastperson/post/aa7d325f-7888-459d-bb0f-23e4a1b86b8c/image.png)

Outbox Pattern With Kafka Connect

This method guarantees at least once delivery. There are times when the same event is published multiple times while the connector is down and then running, so consumers must be idempotent and ensure that duplicate events are not reprocessed.

The process of detecting changed data in logs like this is called **CDC (Change Data Capture)**. Debezium's MySQL Connector reads the binlog to create change events for insert, update, and delete operations and sends them to a Kafka topic. Thus, all events performed on the DB are reliably collected, and by publishing these events, the correct order is guaranteed.
