# Database Load Balancing (Partitioning, Sharding, Replication)

## Partitioning

The act of dividing a table based on specific criteria.

Types include vertical partitioning, which divides by column, and
horizontal partitioning, which divides by row.

Within horizontal partitioning methods, there are types such as hash partitioning and range partitioning.

When performing partitioning, a partitioning key is designated based on specific data. Therefore, the partitioning key should be determined based on the data that is most frequently queried.

<br>
## Sharding

Simply put, sharding is storing tables separated by horizontal partitioning on different DB servers.

Here, the partitioning key is called a shard key.

<br>

## Replication

It's a method of replicating a DB and storing it on multiple DB servers.

It is usually designed with a Master-Slave architecture, where the Master handles Read/Write operations and the Slave handles Read operations.

The Slave's version must be at least higher than the Master's version.

We've briefly looked at these, and we plan to delve deeper into each method.
