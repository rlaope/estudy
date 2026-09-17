# RabbitMQ

An open-source message broker that delivers messages between servers.

When A wants to send messages to B, or A wants to send messages to B, C, D, E, F, etc., you can understand it as RabbitMQ receiving and delivering these messages.

RabbitMQ is a message broker that implements the AMQP protocol, where AMQP stands for Advanced Message Queuing Protocol, a protocol for exchanging messages between client applications and middleware brokers.

Configuration when sending and receiving messages in RabbitMQ

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fm90Ws%2Fbtrq0uBxpL5%2FP7DmDoDWFoLiAPtDy5juck%2Fimg.jpg)

When using servers in an MSA architecture, there are times when servers need to exchange messages with each other, and this is when RabbitMQ is used.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FmOUWA%2Fbtrq1mJC7Tf%2Fm0kzF3WVoA9AuKPfYHBwU0%2Fimg.png)

When Server 1 sends a message to Server 2, it transmits the message through RabbitMQ.

In this case, Server 1 becomes the producer and Server 2 becomes the consumer.

Conversely, when Server 2 sends a message to Server 1, Server 2 becomes the producer and Server 1 becomes the consumer.

### Exchange

Delivers messages received from the producer to queues.  
There are 4 types when delivering to queues:

1. Fanout
2. Direct
3. Topic
4. Headers

These 4 types determine the routing of which queue to send the message received from the producer to.

### Binding
Represents the relationship between exchange and queue.

The exchange can deliver messages to the queue only when they are bound.

Once the exchange rules determine how routing will occur, binding determines the rules for routing which queue to deliver the determined message to.

In other words, exchange determines routing and binding specifies the rules that enable routing. This allows messages that meet specific conditions to be sent to specific queues.

### Queue
The exchange delivers the same message received from the producer to all bound queues.

At this time, the queue delivers the received messages to consumers.

It uses a round-robin schedule to deliver messages fairly.

The queue stores messages in memory or disk before they are delivered to consumers.

<br>

## 4 Types of Exchange

### Fanout

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FuEZzE%2FbtrqXwUebZI%2F3Lu7DhORrXb2y6lWMi3KxK%2Fimg.png)

Sends the same message to all queues bound to the exchange. It's like a broadcast message.

### Direct

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fcz1GiD%2FbtrqY9RYcWf%2Fqk5isXnzc96DUenU5xCdAK%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FFgf4U%2Fbtrq1gpf7lW%2FraHbV09woJxhPLH8OkHdOk%2Fimg.png)

Routes using routing keys. This means messages sent from the exchange can be directly bound to queues through routing keys.

The default exchange used in RabbitMQ is Direct. Queues created in RabbitMQ are automatically bound, and in this case, each queue's name is designated as the routing key.

Multiple routing keys can be specified for one queue (e.g., in direct example_2, routing key is error), and the same routing key can be specified for multiple queues (e.g., in direct example_2, the second queue has a total of 3 routing keys specified).

In other words, you can have a 1:N relationship between queues based on the message's routing key.

### Topic

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcnhJU6%2Fbtrq0t3JfxE%2FjpokZEjjDL8Opwk9jbFH6K%2Fimg.png)

Delivers messages to queues where the routing key pattern matches.

While direct requires an exact routing key match to deliver messages, topic defines binding rules by setting patterns, allowing messages to be sent more flexibly than the direct method.

The rules are:  
*: substitutes 1 word  
#: substitutes 0 or more words (means none or one or more words)

When 2 routing keys are specified for a queue,  
suppose the routing key matches all topic patterns.  
Then the message is delivered only once to the queue, not twice.  
In other words, even if one queue has N routing keys and all patterns match, the message is delivered only once.

**Example when producer sends a message with a specified routing key**

1. When sent with routing key quick.orange.rabbit: Both Q1 and Q2 receive the message.
2. When sent with routing key quick.orange.fox: Only Q1 receives the message.
3. When sent with routing key lazy.brown.fox: Only Q2 receives the message.
4. When sent with routing key quick.orange.male.rabbit: No queue receives the message. -> Because it's \*.orange.\*, only 1 word can be substituted, so Q1 cannot receive it.
5. When sent with routing key lazy.orange.male.rabbit: Only Q2 receives the message. -> Because it's lazy.#, # can substitute 0 or more words, making it possible.
6. When sent with routing key orange: No queue receives the message.

### headers

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbPPFsY%2Fbtrq22YoG1a%2FDvm8p5aAkBYo2Ek5cJucKK%2Fimg.png)

A method of delivering messages to queues based on headers defined as key-value pairs.

Binding occurs when the key-value of the header defined on the producer side sending the message matches the key-value of the argument defined on the consumer side receiving the message.

The header defined by the producer is sent along with the message, and on the consumer side, arguments are defined when the exchange and queue are bound.

The header has a key called x-match. The option values are any and all.


1. x-match: all
Binding occurs only when the key-value of the header and the key-value of the argument match exactly.
2. x-match: any
Binding occurs if at least one of the header key-value pairs sent by the producer matches the argument key-value pairs.
