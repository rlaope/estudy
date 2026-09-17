# Microservice Client Resilience Patterns

All systems, especially those in distributed environments, are prone to failure.

And dealing with and preventing such failures is one of the biggest challenges for developers.

However, such failure detection usually assumes infrastructure system issues or a complete failure of a single service.

While this assumption only helps resolve minor service failures, building a resilient system requires detecting when a service crashes and being able to bypass it.

However, detecting and bypassing performance degradation is difficult for the following reasons:

1. Service degradation occurs intermittently and propagates.
2. Remote service calls are usually synchronous and do not interrupt long-running calls.
3. Applications are typically designed to handle complete failures of remote resources, not partial degradation.

The problem caused by remote services is that if one service goes down, other services can go down with it.
Therefore, without safeguards, a single misbehaving service can bring down multiple applications in a short period.

<br>

## Client Resilience Patterns

Software patterns for client resilience focus on **preventing client crashes when remote service calls fail** due to errors or improper operation, making remote resource access unsuccessful.

The goal of these patterns is to prevent 'upstream propagation' to valuable client consumers like DB connections and thread pools.

There are four client resilience patterns, as follows:
1. Client-side Load Balancing
2. Circuit Breaker
3. Fallback
4. Bulkhead

![](https://velog.velcdn.com/images/zenon8485/post/c87ef5a4-0c85-467d-8614-3e48a4706045/image.png)

### Client-side Load Balancing

Client-side load balancing means caching the locations of individual services through a service discovery agent like Netflix **Eureka. Then, when a client request comes in, it delivers service locations one by one from the pool of service locations it manages.**

A client-side load balancer sits between the consumer and the client service to determine the service's health. If the client load balancer can determine the health of a misbehaving service, it removes that service from the pool of available service locations.

### Circuit Breaker

A circuit breaker monitors requests, and if a call takes too long, the circuit breaker interrupts that service call.

The circuit breaker continuously monitors remote resources, and if a specified number of failures accumulate, it **considers the service faulty, fails fast, and prevents requests from being sent to the faulty resource.**

### Fallback

The fallback pattern performs an action that **handles a remote service call failure.**

It is typically used to return a response by executing an alternative code path (e.g., a stub), retrieve data from a data source, or queue a request for future processing.

It does not raise an exception due to a user call issue but indicates that the request can be performed later.

### Bulkhead

Applying the bulkhead pattern **isolates calls to remote resources into resource-specific thread pools**, thereby reducing the risk that a slow call to a particular remote resource could bring down the entire application.

<br>

### Key Features Provided by Remote Call Circuit Breaker Patterns
1. **Fail Fast**: If a remote service causes degradation, failing fast prevents issues (resource exhaustion) that could bring down the entire application.
2. **Graceful Degradation**: It fails gracefully, allowing alternative mechanisms to be found using circuit breaker patterns that employ timeouts or fail-fast methods.
3. **Seamless Recovery**: The circuit breaker monitors requested resources and can re-allow resource access without human intervention.
