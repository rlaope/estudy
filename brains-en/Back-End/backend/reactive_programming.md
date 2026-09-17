# Reactive Programming: What You Need to Know

## Synchronous, Asynchronous, Blocking, Non-Blocking

### Synchronous
Tasks proceed sequentially, and the next task is performed **after waiting for one task to complete.**

### Asynchronous
It **does not wait** for a task to complete and **continues to perform other tasks while one task is in progress.**

### Blocking
> The caller is interested in the callee's result.

In blocking, when a caller invokes a callee function, the caller function is blocked until the invoked function completes its operation. This means the caller cannot perform other tasks and waits.

### Non-Blocking
> The caller is not interested in the callee's result.

In non-blocking, even if the caller invokes a callee function, control is immediately returned even if the invoked function has not completed its operation.

<br>

## Essential Elements of a Reactive System

### Responsiveness
The system must respond quickly to user requests or external events. -> It must avoid infinite waiting states and minimize latency to enhance user experience.

### Resilience
The system must be able to cope with fluctuating workloads. This allows it to adjust resource usage and maintain or scale performance.

### Fault Tolerance
The system must be robust against hardware, software errors, failures, or partial failures. That is, even if a failure occurs, the system must continue to operate or recover quickly.

### Message-Driven
Reactive systems implement communication between components using a message-driven architecture. Event-driven message passing enables interaction between loosely coupled components.

### Backpressure
Reactive systems implement a backpressure mechanism to cope with workloads arising from data streams.

It prevents overflow through regulation between data producers and consumers.

<br>

## Reactive Streams

Reactive Streams is a specification and part of a library for processing asynchronous data streams.

It is a common mechanism for handling data streams asynchronously, regardless of the library framework, and provides an interface for easily using this mechanism.

**Components**
- Publisher
- Subscriber
- Subscription

### Publisher
Generates data streams and publishes data.

When data is published, it is delivered to the Subscriber.

### Subscriber
A Subscriber consumes data streams and receives data from the Publisher.

When data is received, it can be processed or other tasks can be performed.

### Subscription
Represents the connection between a Publisher and a Subscriber.

A Subscriber can request or cancel data through the Subscription.

Classes that implement the Subscription interface manage subscriptions.

Reactive Streams is designed to simplify asynchronous processing of data streams and efficiently handle large amounts of data. One of its main goals is speed control and overflow prevention through **back-pressure**. Additionally, Reactive Streams is used in various asynchronous programming environments and is supported by Java's CompletableFuture, Spring WebFlux, RxJava, Project Reactor, and more.

<br>

## Java NIO
Java NIO is a package that improves the functionality of Java IO and supports asynchronous non-blocking I/O.

The core idea is to efficiently process I/O operations using channels and buffers.

### Channel, Buffer

In Java NIO, I/O operations are performed through channels and buffers.

A Channel represents a connection through which data can be read and written, and it can be associated with various I/O sources such as files, sockets, etc.

A Buffer is a memory area that temporarily stores data and is used for I/O operations.

### Selector
A Selector monitors multiple channels and detects whether events have occurred.

It helps manage multiple channels asynchronously from a single thread.

This makes it useful for handling multiple client connections or multiple network sockets.
