# Netty Proxy

In a system that integrates with an external organization, the external organization only supported a maximum of 80 sessions.

Therefore, a proxy server had to be built to manage traffic with a single responsibility, including service traffic control, session control, circuit breakers, and retries.

Among the options, Netty was chosen.

Of course, simple session limits, rate limits, and backpressure can also be implemented with other proxy products like Nginx, HAProxy, and Envoy.

However, the problem starts with the external system only supporting 40-80 sessions, which means more scalability than a simple connection limit is needed in this situation.

The fewer external sessions there are, the more requests will be in a waiting state.

If the external system only has 40-80 sessions, then in a scenario with an internal 2000 RPS and an external processing capability of 50 requests per second, the remaining 1950 requests must be queued at the proxy.

A problem arises at this point: if all 1950 waiting requests are handled using a blocking I/O approach, then

- The number of threads skyrockets
- CPU context switches explode
- Memory increases
- The proxy itself fails due to thread exhaustion

Simply put, if there are few external sessions, there will be many waiting requests, and the key is how to efficiently maintain those waiting requests.

And in this regard, Netty has an overwhelming advantage.

### It can handle tens of thousands of waiting requests with a single-thread Event Loop.

This is due to Netty's inherent strength, which is based on asynchronous non-blocking I/O:
- The number of worker threads is very small
- All requests are processed in the event loop based on their state.
- Requests are maintained in a lightweight state until an event arrives, as external sessions can be allocated.
- Even with 100,000 requests, there's no thread explosion.

On the other hand, with a proxy/blocking structure that doesn't support NIO:
- Threads or coroutines are needed for each request
- The server can crash just from thread reservation
- Memory overflow or context switching explodes.

It also reliably supports connection limits + asynchronous queuing + scheduling.

While other proxies easily provide connection limits,

- Complex queue scheduling
- Priority queue
- Deadline queue
- Request dispatch based on external availability events

Advanced I/O controls like these are limited.

Netty can do all of the following:
- 40 outbound session calls
- Dispatch only when an `available channel` event occurs among these sessions
- The rest are held by the event loop as lightweight objects
- Backpressure on queue overflow
- Custom control possible with user-defined policies

This level of custom I/O control is virtually impossible with Nginx, HAProxy, or Envoy.

### Netty is stable when external integration responses are slow.

For example, if an external system has a response time of 5-10 seconds, a proxy using a blocking thread model is highly likely to experience thread exhaustion as requests are held for a long time.

However, Netty:
- Processes all progress as an async state machine
- Simply remains in a `future` state if it takes a long time
- Threads can process other requests
- No thread exhaustion

Therefore, it is good for domains where external system responses are slow.
