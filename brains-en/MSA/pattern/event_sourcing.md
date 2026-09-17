# Event Sourcing

It is a concept that processes all state changes of an application by storing them sequentially as events.

By processing all states as a stream of events, application development can be simplified, and it can appropriately respond to distributed environments.

All events are recorded in chronological order, and each service only needs to observe the events it requires to maintain its latest state.

That is, a user management service can maintain the latest user information (snapshot) by referring only to user creation, deletion, or modification events, and a product-related service refers to (listens to) only events related to product orders, deliveries, etc., and maintains its latest state.

For Event Sourcing to work perfectly, all events must be recorded, and retransmission must be possible so that no service fails to receive an event.

Furthermore, events should not only be receivable within their own service or WAS but must also be broadcastable to function perfectly in a distributed environment.

Due to these characteristics of Event Sourcing, when applying an MSA system, implementing it based on events makes it easier to implement and allows for loose coupling between services.

![](https://mblogthumb-phinf.pstatic.net/MjAxOTAzMTVfMTkw/MDAxNTUyNjEyOTE4NzI2.V9ICsBgEppRERJBNeQx_BJsX4_H2cOZxqxzI3MsQPAkg.Etp3aIy-zoip4VI4UO-AN3D08KN4cPsdeLJQTSWBHEkg.PNG.rogman0/03.png?type=w800)

Event Sourcing operates similarly when there is a data modification task, by requesting the service that holds the data.

However, one difference is that **the service holding the data does not directly manipulate it.**

Looking at the diagram above, there are several elements.
- Event Producer
- Event Consumer
- Event (Command)
- Event Store (Queue)
- Event Snapshot (red circle)

Instead, it generates events like those above and stores them in the Event Store.

Message brokers like Kafka can serve as the Event Store. Then, event consumers load these events into a database or HDFS in the order they arrive.

And then, they modify or reflect the necessary data.
