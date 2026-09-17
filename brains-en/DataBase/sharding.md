# Database Sharding (Distributed Processing)

### What is Database Sharding?
As data in a DB grows, capacity issues arise, performance degrades, and the likelihood of problems across the entire DB system increases. To prevent this, various DB distributed processing techniques exist, and among them, we will explore Sharding.

Sharding is a Horizontal Partitioning method that distributes and stores data with the same table schema across multiple databases, reducing the index size of the table and increasing operational concurrency.

However, applying DB Sharding increases programming and operational complexity, so it is generally recommended to consider other distributed processing methods first and only use Sharding for large-scale big data management.

- Physical Scaling: Physically scale DB servers and storage.
- Apply Cache & DB Replication: For read-heavy systems, apply caching and DB replication methods.
- (DB Replication: Divides the DBMS into a Master/Slave structure, where the Master performs Insert, Update, and Delete operations, and the Slave DB copies the actual data to perform Select operations.)
- Vertical Partitioning: When only a few columns of a table are frequently used.
- Hot & Cold Data Separation: Cold data, such as photos, videos, and emails, which have long retention periods but are not frequently accessed, are separated into a dedicated DB.

<br>

### Constraints
Due to the structural characteristics of horizontal partitioning, DB Sharding methods have the following constraints, and designs must account for them.

- Cross-joins between shards are not possible, so denormalization must be accepted.
- A single transaction cannot access more than one shard.
- Generating unique Shard Keys for all shards is crucial. Auto-incrementing keys provided by the DBMS are not valid, and a separate application is needed outside the DB to manage Shard Keys (using a router).
- For safety, each shard requires a Replication Set configuration.
- The design must allow for scaling shards without service interruption.

<br>

### Types of Sharding Strategies

#### Hash Sharding
The DB is selected based on the result of hashing the Shard Key. Hashing methods include modular arithmetic, and it's necessary to design the hash function well according to the data type. This is mainly applied when the data volume is expected to remain at a certain level.

- Advantages
  - Data is uniformly distributed across each shard.
- Disadvantages
  - When adding or expanding DBs, the hash function must change, requiring reordering of already loaded data.

#### Dynamic (Range Based) Sharding
The Shard Key is partitioned based on a specific range using a Local Service. The criteria can be dynamically changed according to data traffic.

- Advantages
  - When expanding, only the Shard Key needs to be added, eliminating reordering costs.
- Disadvantages
  - Data may be concentrated in some DBs.
  - Data partitioning range criteria must be clearly defined.
  - Increased dependency on the Locator can affect the entire shard if the Locator fails.
  - When data is reallocated, the Locator's Shard Key Table must also be synchronized.
  - When running Cache and Replication for performance, incorrect routing by the Locator can lead to errors where data cannot be found.

#### Entity Group
When tables consist of various objects rather than a Key-Value relationship, entities with certain relationships are grouped within the same shard.

- Advantages
  - Efficient when queries are processed within the same physical shard.
  - Good scalability with increasing users.
  - Possesses strong cohesion within a single shard.
- Disadvantages
  - May be inefficient when queries are required across specific partitions.
