# Message Queue (MQ) Concepts

## Message Queue

A Message Queue (MQ) is a communication method used when processes or program instances exchange data with each other.

In a broader sense, it refers to a system that implements Message Oriented Middleware (MOM).

Here, MOM refers to the sending and receiving of data between applications using asynchronous messages.

MQ can aid in the development of Service-Oriented Architecture (SOA) by providing the flexibility to defer separate processing tasks.

When exchanging messages between different processes or programs, AMQP (Advanced Message Queuing Protocol) is used.

AMQP is the MOM standard for ISO application layer protocols.

AMQP is often compared with JMS (Java Message Service), which is a standard API that supports MOM in Java.

JMS allows communication between different Java Applications but cannot communicate with other MOMs (like AMQP, SMTP, etc.).

### Advantages

- Asynchronous: Because messages are placed in a queue, they can be processed later.
- Decoupled: Can be separated from the application.
- Elasticity: If some parts fail, the entire system is not affected.
- Redundancy: Tasks can be retried in case of failure.
- Assurance: You can confirm that a task has been processed.
- Scalability: Multiple processes can send messages to the queue.

Message queuing is used for batch jobs that process large volumes of data, chat services, or when handling asynchronous data.

When creating and using web requests or general programs processed on a per-process basis, if the number of users or data increases, the number of requests waiting for a response grows. This eventually leads to delayed waiting times, causing the service to fail. Therefore, the purpose is to centralize previously distributed data processing and employ a message broker to distribute necessary program tasks.

### Use Cases for Message Queues

- Enables sending and receiving data from APIs in other locations.
- Allows asynchronous communication across various applications.
- Facilitates email sending and document uploads.
- Can handle a large volume of processes.

### JMS vs AMQP

AMQP is the MOM standard for ISO application layer protocols.

JMS is a standard API that supports MOM in Java. (JMS and AMQP are different concepts.)

JMS allows communication between different Java applications but cannot communicate with other MOMs.

Java applications using ActiveMQ's JMS library can communicate with each other. However, they cannot communicate with JMS implementations of other Java applications (those not using ActiveMQ).

AMQP allows communication between applications using different AMQP implementations, as long as the protocol matches.

It can also communicate with SMTP.

The JMS library does not support AMQP.

## Types of Open Source Message Queues

I'll organize these one by one later, but for now, there are typically four main message queue systems:

1. RabbitMQ
2. ActiveMQ
3. ZeroMQ
4. Kafka

All four of these systems commonly provide asynchronous communication and decouple senders from receivers.

However, they have different purposes depending on the task.
