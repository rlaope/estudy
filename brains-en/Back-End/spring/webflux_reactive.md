# 📗 Mastering Spring WebFlux, Reactive Stream, R2DBC, Mono, Flux

![](image/webflux_reactive_0.jpg)

### Overview

Non-blocking application APIs are becoming almost essential for performance improvement in large-scale software programs. Therefore, today I'd like to summarize WebFlux, Reactive Streams, and non-blocking concepts.

### Non-Blocking

Non-blocking is a programming style that allows other tasks to be performed before returning the result of processing a single task at a time.

Non-Blocking Characteristics

*   The CPU can allocate other tasks while performing I/O operations or long-running tasks, maximizing resource utilization.
*   Unlike multi-threading, context switching between threads does not occur, which optimizes CPU load and memory usage.

### Spring WebFlux

Spring WebFlux is a web framework in the Reactive-Stack added in Spring 5, supporting **Non-Blocking Reactive-Stream** for reactive application development between clients and servers.

The reasons for WebFlux's emergence include:

1.  To handle concurrency with a small number of threads and minimal hardware resources.
2.  Functional programming.

#### Spirng WebFlux Background

Spring WebFlux is one of the Spring framework modules that applies Reactive Programming to overcome the limitations of traditional Spring MVC.

Traditional Spring MVC operates with a **Thread-Per-Request model** supported by the Servlet specification. In this model, one thread is allocated per request, and if blocking occurs during the processing of a request, the thread assigned to that request remains in a blocking state. Blocking I/O is also one of the causes of wasted CPU resources. Furthermore, processing a large number of requests requires a large number of threads, necessitating a fixed number of threads on the server, which can lead to increased overhead due to **thread context switching**.

In contrast, the Reactive Programming Model operates based on event-driven Non-Blocking I/O, processing requests asynchronously without blocking the threads responsible for handling them. This approach allows other requests to be processed even during blocking I/O wait times, and more requests can be handled with limited resources.

In other words, Spring WebFlux supports this Reactive Programming Model and provides high performance and scalability.

Netty was emerging as a server for asynchronous non-blocking environments, and Spring needed new APIs for integration with Netty.

Spring WebFlux is recommended for use in the following scenarios:

*   When used for asynchronous, non-blocking reactive development.
*   Used for developing high-performance web applications that operate efficiently.
*   Suitable for microservice architectures with frequent inter-service calls.

**Spring WebFlux :**

-> **many request : 1 thread, async + Non-Blocking**

**Spring MVC : 1 request :**

-> **1 thread, sync + Blocking**

### Reactive Stream

Reactive Stream is a **standard specification for asynchronous, Non-Blocking, backpressure-based stream processing**. Using it eliminates waiting times for other tasks to complete during data processing, and once a task is processed, the next task can be handled immediately, allowing for more efficient resource utilization.

Reactive Stream uses the Publisher-Subscriber pattern, where the Publisher generates data and the Subscriber processes the generated data. If the volume and speed of data generated in this process exceed the Subscriber's processing capacity, backpressure allows the Publisher to limit data production, enabling the Subscriber to process data at a rate it can handle.

### R2DBC

R2DBC (Reactive Relational Deatabase Connectivity) is a Non-Blocking application stack that can handle concurrency with a **small number of threads** and scale with **fewer hardware resources**.

Pairing:

*   spirng mvc : jpa
*   spring webflux: r2dbc

This is how they can be viewed.

Because it's a Reactive Model, operations like `findAll()` return `Flux<T>` instead of `List<T>`, and `Mono<T>` is returned instead of `<T>`.

### Mono & Flux

The reactive library used in Spring WebFlux is Reactor, and Reactor is an implementation of Reactive Streams. That's why Reactive Streams is mentioned in the WebFlux documentation, and Reactor appears alongside it.

Mono and Flux, as key objects, are essential to understand the operational structure of WebFlux.

*   Mono: Delivers 0 to 1 data item.
*   Flux: Delivers 0 to N data items.

Mono and Flux are implementations that implement the Publisher interface of Reactive Streams.

#### Mono

![](image/webflux_reactive_1.jpg)

#### Flux

![](image/webflux_reactive_2.jpg)

### Wrapping Up

Thus, I've studied WebFlux and its related terminology. My journey to master asynchronous processing continues endlessly..!
