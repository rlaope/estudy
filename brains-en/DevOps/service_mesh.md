# Service Mesh

A service mesh is a way to control how different parts of an application share data with each other.

Unlike other systems where each **service (each part of an application)** must manage communication between services, a service mesh is built as a dedicated infrastructure layer within the application. This allows it to record how seamlessly different applications interact, making it easier to enable communication between applications, alleviate the complexity of communication as applications scale, and prevent downtime.

## When to Adopt a Service Mesh?

Modern applications involve services interacting with other services and databases to exchange data.

These interactions can be categorized as a network of services performing specific business functions, where data requests are exchanged among various services within this network.

In such cases, to prepare for scenarios where a specific service is overwhelmed with data requests, a service mesh is introduced to **route requests from one service to the next so that all operations can be optimized**.

## Differences from Microservices

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbl3eMb%2FbtrgDdXFvyb%2FNXPLOy3xtKqOm6IU4KIeP1%2Fimg.png)

### Microservices
Microservices are built by small teams, allowing for the flexibility to choose their own tools and coding languages for each microservice.

That is, each microservice is built independently, communicates with others, and has the characteristic that the entire service does not go down even if a specific service fails.

### Limitations of Inter-Service Communication in Microservices

While these microservices can communicate by embedding code within each microservice without a service mesh layer, limitations arise when communication becomes complex.

In actual microservice architectures, the service mesh for cloud-native applications has adopted an approach where numerous individual services are composed into a functional application.

## How Does a Service Mesh Work?

### Differences from Traditional Data Communication Methods

Traditionally, applications in all architectures always required rules specifying how to move from A to B for a particular request.

However, a service mesh differs in that it abstracts communication control from individual service-level logic (controlling service-to-service communication) into an infrastructure layer.

### How a Service Mesh Operates

A service mesh is built into an application as an array of network proxies.

That is, in a service mesh, requests are routed between microservices via proxies in its own infrastructure layer.

In other words, the individual proxies that make up a service mesh are not executed within the microservice itself, but rather run alongside each microservice, which is why they are also called `sidecar`s.

As shown in the figure below, these sidecars are positioned alongside microservices, requesting routing to other proxies, and these sidecars collectively form a mesh network.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcTJy7K%2FbtrgHFlSDi6%2FDtvVGwE3Q0nWNJiEuXVq1K%2Fimg.png)

### Microservices Without a Service Mesh

In microservices without a service mesh, developers must also implement the logic to control communication between services within each microservice, which prevents them from focusing on their primary business objectives.

Furthermore, since the logic controlling communication between services resides within the services themselves, it becomes difficult to identify which service communication is experiencing issues.

Moreover, if a new service is added to each application, it further complicates communication between services, and complex microservices make it difficult to locate points of failure within the application.

## Benefits of a Service Mesh
1. It captures all aspects of communication between services as performance metrics.
2. As data accumulates over time, these performance metrics can be applied to communication rules, enabling services to request services efficiently and reliably.
3. Developers can focus more on business objectives without worrying about communication between services.
4. It forms a visible infrastructure layer alongside services through software like Jaeger (a distributed tracing system), making it easier to identify and diagnose problems.
5. A Service Mesh can re-route requests away from a failed service, thereby improving application recovery capabilities during downtime.

> Jaeger?
> It is open-source software that traces transactions between distributed services, used for monitoring and troubleshooting in complex microservice environments. Distributed tracing, in this context, is a method of understanding the entire sequence of events in complex interactions between microservices.
