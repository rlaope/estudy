# Envoy Proxy Internal Architecture

Istio merely acts as the brain for policy enforcement; the actual role of controlling data flow, splitting packets, and delivering them falls to Envoy Proxy.

90% of service mesh optimization and troubleshooting depends on understanding Envoy's internals.

### Threading Model

Why is Envoy overwhelmingly fast? Envoy uses a single-process, multi-threaded model.

Each thread has an independent event loop.

- Main Thread: Responsible for management tasks such as server startup and shutdown, xDS API updates, and signal handling.
- Worker Threads: Process actual requests. Each thread operates in a non-blocking manner, and a single connection is pinned to a specific worker thread for its lifetime, minimizing context switching overhead.

Nginx has a multi-process structure, making memory management complex. Node.js uses a single event loop, requiring clustering for multi-core utilization.

However, Envoy launches as many workers as there are cores within a single process, achieving both memory efficiency and performance simultaneously.

### L3/L7/L7 Fitler Chain

All data entering Envoy passes through a pipeline called a fitler chain.

Thanks to this structure, features can be extended modularly. Let's explore.

1. Network Filter (L3/L4): Handles data at the IP and TCP levels. (e.g., TLS inspector, RBAC filter)
2. HTTP Connection Manager (HCM): A core filter that interprets L4 data as L7.
3. HTTP Filter (L7): Performs tasks such as HTTP header modification, routing, gRPC transformation, and compression. (ex.
router filter)

The **order** of the filter chain is crucial. Consider that an authentication filter must execute before a routing filter to block unauthorized access.

### Four Core Components: LRCE

These are the four core objects you should look for first when analyzing Envoy configurations.

- **Listener**: A gateway that waits for requests on a specific IP port (like a phone number for receiving calls).
- **Route**: Determines which cluster to send a request to based on its host and path.
- **Cluster**: A logical group of endpoints that perform the same function.
- **Endpoint**: The actual destination `pod ip:port` where the request will be delivered.

### Tracing Envoy Request Rejections

In case of a 503 communication failure via Envoy, you can quickly identify the culprit by checking the access logs. Let's look at the response flags recorded in the logs.

- UH (No Healthy Upstream): When there are no healthy endpoints within the cluster (e.g., pod is dead or health check failed).
- NR (No Route Configured): When no routing rule matching the requested URL is found.
- UF (Upstream Connection Failure): When the TCP connection itself fails from the target service (e.g., network issue).
- UO (Upstream Overflow): When the circuit breaker activates and blocks the request.
