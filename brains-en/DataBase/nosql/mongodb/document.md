# Document Database

Document Databases offer the following advantages:

### High Performance

MongoDB provides high-performance data persistence.

- The embedded data model reduces I/O activity in the database system.
- Fast queries through indexes, and keys can be added to embedded Documents and Arrays.

### Rich Query Language

MongoDB offers a rich query expression language.

### High Availability

MongoDB's replication, known as a replica set, provides the following features:

- automatic failover (if a specific node fails, another node takes over the task)
- data redundancy (a replica set is a group of MongoDB servers that provide the same data set, ensuring data reliability and high data availability.)

### Horizontal Scalability

MongoDB provides horizontal scaling and offers the following key features:

- Sharding: data distribution among machines
- MongoDB creates shard key-based zones starting from version 3.4. In a balanced cluster, MongoDB directs read and write operations to access the corresponding shard's zone directly.

### Support for Multiple Storage Engines

MongoDB offers various storage engines.

- WiredTiger Storage Engine
- MMAPv1 Storage Engine

In addition, various external engines can be added.
