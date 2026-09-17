# [Domain-Driven-Design] Bounded-Context ✍️

### Overview

Feeling the importance of DDD (Domain-Driven Design), I started studying its concepts and have posted about what I've learned regarding DDD. This time, I will study the term **Bounded-Context**.

![](image/bounded_context_0.png)

### Bounded-Context

Bounded-Context is a concept first introduced in Domain-Driven Design.

A Bounded-Context means dividing a large system into several smaller **contexts**, where specific business rules and data models are applied within each context. Each context is designed and implemented independently, and different contexts interact through interfaces.

A Bounded-Context represents a specific business problem domain within a larger domain, and within that area, **terms, concepts, and rules** are applied consistently.

Bounded-Contexts play an important role in domain modeling. When performing domain modeling, you **separate the entire domain into smaller units, identify each Bounded-Context, and design each context to operate independently**. This reduces the overall system complexity and increases flexibility.

> What is a Context?
> It means that when understanding and interpreting something, its situation, background, and other related matters must be considered together.
>
> In software development, a context represents a **business problem domain** as part of a software system. For example, in a banking system, functions such as loan applications, account openings, and transfers each have different contexts.
>
> To explain the difference between a domain and a context, a domain refers to a **specific business field**. (e.g., banking, aviation, hospitality) And a context refers to a **specific business problem domain within a larger domain**. (e.g., loan application, account opening within the banking domain)

### Benefits of Bounded-Context

1. It makes it easier to understand the components of a complex system.
2. Each context operates independently, allowing developers to work on a context-by-context basis.
3. As the system grows, separating each context can increase scalability.
4. Bounded-Contexts can simplify communication between teams.

### Is a Bounded-Context a Service in Spring Boot?

Not exactly. While Bounded-Context is a concept used in Spring Boot, it is not a service in itself.

Spring Boot provides several ways to implement Bounded-Contexts.

For example, they can be implemented as independent microservices or by dividing them into multiple modules within an application.

A Service is a component used to implement business logic. It can perform the role of providing necessary functionality within a Bounded-Context and can be part of a Bounded-Context, but it is not necessarily so. For instance, a service that provides common functionality used across multiple Bounded-Contexts might be created and utilized.

Therefore, a Bounded-Context is a broader concept than a service, and it is **used to design the overall architecture of an application.**
