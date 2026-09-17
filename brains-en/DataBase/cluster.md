# Cluster and Conditions in Databases

Typically, when building a database, it's common to set up and use a single database on a single server.

However, if a single server uses a single database, the service will go down if that server fails.

Alternatively, if a huge number of users flood in and a single server tries to handle it, the server will be unable to cope and crash.

For these various reasons, **a single database is built across multiple servers**.

This configuration, where a **single database is built across multiple servers, is called a cluster**.

## Conditions for a Cluster
1. High Availability
2. Parallel Processing
3. Performance Improvement

### High Availability
In databases, availability refers to the ratio of the time a database is operational to the time it is down.

In other words, availability is **expressed as the ratio of how many minutes it was down over a year**.

Databases also run on computers. Computers can stop for various reasons.

When such an event occurs, if there's only one server, a major outage will occur where the service remains down until the database is recovered.

When such an event occurs, if there's only one server, a major outage will occur where the service remains down until the database is recovered.

**When a problem in one part causes the entire system to halt, it's called SPOF (Single Point Of Failure)**. To resolve this, a high-availability cluster must be used.

> What is High Availability?
> : It means configuring database equipment (such as servers or storage) with at least two units each, so that if one fails, service can be quickly resumed using a replicated database (redundant database) that holds the same data.

A database cluster system must be able to satisfy this high availability.

### Parallel Processing
When a table has a huge amount of data and many columns, and SQL statements are processed complexly, executing these SQL statements can take a long time.

In such a situation, if the database is processed in parallel across multiple units and the results are integrated and returned, results can be obtained much faster.

Since replicated databases always hold the same data as the original database, distributing them while considering DML operations can improve overall performance.

### Performance Improvement
If the number of users accessing the database increases significantly, creating replicas of the database and using these replicated databases for read operations can help cope with a large number of users.

A database cluster must be able to satisfy such performance improvements.

## Conclusion
- A database cluster refers to **a configuration where multiple servers share and process a single database.**
- A database cluster is a system configuration that satisfies these three conditions: high availability, parallel processing, and performance improvement.
