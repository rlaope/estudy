# Synchronous Event Processing Issues, Asynchronous Event Processing

## Synchronous Event Processing Issues
> Processing events synchronously leads to performance degradation and transaction scope issues.

### Performance Degradation
- While events resolve tight coupling issues, the problem of being affected by external services remains.
- Performance degradation of an external service directly leads to performance degradation of my system.

### Transactions
- If an external service execution fails, should the transaction necessarily be rolled back?
- The way to resolve performance and transaction scope issues that arise when integrating with external services synchronously is to process events asynchronously or to link events with transactions.

## Asynchronous Event Processing

> Requirements that can be rephrased as 'When A happens, do B by at most [time]' can be implemented by processing events asynchronously.

Among the things that need to be implemented, requirements stating 'When A happens, then do B' often actually mean 'When A happens, do B by at most [time]'.

In other words, there are many cases where follow-up actions only need to be processed within a certain timeframe.

Furthermore, in requirements stating 'When A happens, then do B', if B fails, it's often acceptable to retry at regular intervals or handle it manually.

In the requirement 'When A happens, do B within a certain time', 'When A happens' can be seen as an event.

Therefore, the 'do B' functionality can be sent from the handler that processes event A.

Typically, asynchronous event processing can be implemented in the following four ways:

- Executing local handlers asynchronously
- Using a message queue
- Using an event store and event forwarder
- Using an event store and an event provision API

## Asynchronous Execution of Local Handlers
The way to execute event handlers asynchronously is to run them via a separate thread.
You can easily execute event handlers asynchronously using the @Async annotation.
To do this, enable asynchronous functionality with the @EnableAsync annotation and attach the @Async annotation to the event handler method.

## Asynchronous Implementation Using a Messaging System
Events can be processed asynchronously using messaging systems like Kafka or RabbitMQ.
A message queue processes events through the following flow:

![](https://user-images.githubusercontent.com/42582516/160272279-15436155-3bb9-42ee-bbf4-29a3c70fb478.png)

Asynchronous event processing using a message queue

If necessary, the domain functionality that generates events and the process of storing events in a message queue can be grouped into a single transaction using a global transaction.
While using a global transaction can safely deliver events to a message queue, overall performance may degrade, and some messaging systems do not support it.
When using a message queue, the entity generating the event and the event handler typically operate in separate processes.
This means that the JVM where the event originates and the JVM that processes the event are different.

## Asynchronous Processing Using an Event Store
Events can first be stored in a DB and then delivered to event handlers using a separate program.

![](https://user-images.githubusercontent.com/42582516/160272405-559286da-7d45-4439-8d49-e121456b016f.png)

Asynchronous event processing using an event store and forwarder

When an event occurs, the handler stores the event in the storage.
The forwarder periodically retrieves events from the event store and executes event handlers.
This approach uses the same DB for both the domain's state and the event store, so changes in the domain's state and event storage are handled as local transactions.
Since events are stored in physical storage, if event processing fails, the forwarder can re-read the event from the event store and execute the handler.
Alternatively, an external API can be used as follows:

![](https://user-images.githubusercontent.com/42582516/160272535-2acf53cc-2be2-42ac-a612-202d8f187b66.png)

Asynchronous event processing using an event store and API

The difference between the API approach and the forwarder approach lies in how events are delivered.
While the forwarder approach uses a forwarder to deliver events externally, the API approach has external handlers retrieve event lists via an API server.
In the forwarder approach, the forwarder tracks how far events have been processed, whereas in the API approach, the external handler itself must remember.

### Event Store Implementation
The code structure for implementing an event store is as follows:

- EventEntry - Data to be stored in the event store.
- EventStore - Provides an interface for storing and querying events.
- JdbcEventStore - An EventStore implementation class using JDBC.
- EventApi - A controller that provides a list of events using a REST API.

DDL for the table to store EventEntry.

```sql
create table evententry (
	id int not null AUTO_INCREMENT PRIMARY KEY,
	`type` varchar(255),
	`content_type` varchar(255),
	payload MEDIUMTEXT,
	`timestamp` datetime
) character set utf8mb4;
```

### Implementing an Event Handler for Event Storage
The handle() method of EventStorehandler uses the eventStore.save() method to specify the event object.

### REST API Implementation
You can execute the EventStore's get() method using the web request parameters offset and limit, and return the result as JSON.
By connecting to the URL handled by EventApi, you can obtain a list of EventEntry in JSON format.
Clients using the API perform the following steps at regular intervals:

1. Get the lastOffset, which is the offset of the last processed data. If no lastOffset is stored, use 0.
2. Execute the API using the last processed lastOffset as the offset.
3. Process the data received from the API.
4. Store offset + number of data items as lastOffset.

If an event fails using the client API, it can be reprocessed by reading from the failed event again.
Furthermore, even if the API server experiences a failure, events can be processed by retrying periodically once the server recovers.

### Forwarder Implementation
Similar to an API-based client, a forwarder can periodically read events from the EventStore and deliver them to event handlers.
It should remember the offset of the last delivered event and retrieve events starting from that last processed offset at the next query.
