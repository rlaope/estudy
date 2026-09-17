# CQRS (Command Query Responsibility Segregation)

## Limitations of a Single Model
> ORM techniques are not suitable for query functions, so state change models and query models should be separated.

When implementing query functionality, it often requires data from multiple aggregates.

However, due to the nature of query screens, faster query speeds are always better, so the implementation method needs careful consideration.

If you use a method that references aggregates using identifiers, you cannot utilize JPA's query optimization features like eager loading.

Even if you connect through direct referencing, the same association must be handled with either eager or lazy loading depending on the characteristics of the query screen.

These problems arise because a single domain model is used for both changing system state and querying it.

While ORM techniques are suitable for implementing domain state change functions, implementing functions that retrieve and display data from multiple aggregates involves many considerations, leading to complex implementations.

To reduce implementation complexity, **state change models and query models are separated**.

## CQRS
The functionalities provided by a system can largely be divided into functions that change state and functions that query state information.

From a domain model perspective, state change functions primarily modify the state of a single aggregate.

On the other hand, displaying data required for query functions often necessitates two or more aggregates.

Since the scope of these two functions does not exactly align, implementing both types of functions with a single model leads to a complex model.

The method used to resolve the complexity that arises when using a single model is `CQRS`.

CQRS, an acronym for Command Query Responsibility Segregation, is a pattern that separates models for commands (which change state) and models for queries (which provide state).

![](https://user-images.githubusercontent.com/43809168/100109843-9b06ca00-2eaf-11eb-816d-0f017a51afd7.png)

The more complex the domain, the greater the difference in the data scope handled by command functions and query functions, making CQRS suitable.

Using CQRS allows you to choose implementation technologies appropriate for each model, as follows.

![](https://user-images.githubusercontent.com/43809168/100110332-2aac7880-2eb0-11eb-8df9-2e9a77dc9cc4.png)

The command model for state changes is implemented using an object-based domain model.

The query model uses data types that contain the information required for query functions.

The command model is designed to focus on executing domain logic that changes state, while the query model is designed to focus on querying data to be displayed on the screen.

![](https://user-images.githubusercontent.com/43809168/100110318-26805b00-2eb0-11eb-98e8-fc305bbdac4a.png)

The command model and query model can also use different data stores.

The command model might use an RDBMS that supports transactions, while the query model might use a memory-based NoSQL database with good query performance.

![](https://user-images.githubusercontent.com/43809168/100110513-5fb8cb00-2eb0-11eb-865d-a976741daf68.png)

## Web and CQRS
Typical web services have more requests that query state than requests that change state.

Therefore, to improve query performance, development teams typically optimize queries and cache in-memory query data to increase response speed.

Alternatively, a separate read-only store might be used.

These various techniques ultimately produce the same effect as applying CQRS.

## CQRS Pros and Cons
An advantage is that when implementing the command model, one can focus solely on the domain itself.

Query-related logic is removed from the command model, reducing complexity and allowing focus on implementing domain logic.

Furthermore, caching techniques can be applied at the query unit level, and specialized queries can be freely used, which is advantageous for improving query performance.

Using a query store can significantly increase query throughput.

![](https://user-images.githubusercontent.com/43809168/100110983-f08fa680-2eb0-11eb-8fd1-858358dae290.png)

However, a disadvantage is that it requires more code to implement and more implementation technologies.

The decision to adopt the CQRS pattern should be made by considering its pros and cons.
