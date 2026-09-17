# SQS

## What is AWS SQS
- SQS stands for Simple Queue Service.
- It can be thought of as a very simple Queue for delivering messages between applications.
- It offers excellent durability and provides secure, hosted queues, supporting dead-letter queues, standard queues, and FIFO queues.

## Differences between SQS and MQ
- SQS is, as its name suggests, a **'simple' queue service**. It does not support message routing, fan-out, or distribution lists found in other MQs. **Its sole purpose is to allow message consumers to retrieve messages produced by message producers.**
- Amazon MQ is a fully managed service that fully supports various standardized broadcast protocols like AMQP and MQTT. It is useful for implementing complex requirements and for migrating message brokers outside of AWS to AWS.

## Differences from SNS

### SNS Simple Notification Service
1. A managed service where a publisher sends messages to subscribers.
2. A publisher publishes messages to a Topic. A Topic can deliver messages to numerous Subscribers (fan out). Delivery methods include Lambda, SQS, Email, and others.
3. Do other systems care about the event? : When a Topic wants to publish a message and notify people that it has been published.

### SQS Simple Queue Service
1. A secure, managed message queuing service that helps you easily decouple and scale microservices, distributed systems, and serverless applications.
2. Systems can initiate new events from the Queue. Messages in the Queue are processed by a single consumer or a single service.
3. Does this system care about the event? : When I am the event receiver.
