## Database Partitioning

I will delve deeper into database partitioning, including its purpose and pros and cons.

In the next post, I will explore partitioning types and partitioning techniques.

<br>

## What is Partitioning?

Let's start by understanding the background of partitioning.

As service scales grew, trying to accommodate very large tables with massive amounts of data within a single database inevitably led to strain, resulting in capacity limitations and performance degradation.

With the advent of extremely large tables, such as in VLDB (Very Large DBMS), numerous performance-related issues, like capacity constraints, arose. To address these problems, the technique of partitioning emerged.

### Partitioning Technique
The technique of splitting and managing a table into units called partitions is known as the partitioning technique.

It is a general term referring to the act of splitting logical data elements into multiple entities.

In other words, it means physically dividing a large table or index into smaller, more manageable units called partitions. Even with physical data division, applications accessing the database are unaware of it.

### Purpose
The purpose of using partitioning techniques is considered from the following aspects:

1. Performance
Primarily, it improves the performance of DML (Select, Update, Insert, Delete) operations.

It is efficient in high-volume database write environments.

In particular, it is efficient by reducing the access range for data that frequently undergoes Full Scans.

In OLTP systems with many INSERTs, it reduces contention by distributing INSERT operations across smaller units, i.e., partitions.

> OLTP stands for Online Transaction Processing, which refers to transaction operations occurring online, including insert, update, and delete operations.

2. Availability
Physical partitioning reduces the possibility of data loss/corruption.

Each partitioned area (per partition) can be backed up and recovered independently.

Since Disk I/O is distributed and processed on a per-partition basis, Update performance can be improved.

3. Manageability
It simplifies management by allowing large tables to be removed.

### Advantages
From a management perspective, it enables backup at the partition level and facilitates operations such as adding, deleting, and modifying data.
- The likelihood of losing all data is reduced, improving availability.
- Data can be backed up and recovered per partition.
- I/O operations are processed on a per-partition basis, improving Update performance.

From a performance perspective, DML operations are performed on a per-partition basis, which reduces the data scan area and improves performance.
- The access range for Full Scans is reduced to partitions, improving performance.
- Queries become lighter as only necessary data can be retrieved quickly.

### Disadvantages

- The cost of Join operations can increase.
    - Caution is needed because when performing operations like queries on data not designated as a partition key, joins may occur across multiple partitioned tables.
- Tables and indexes cannot be partitioned separately. -> The disadvantage is that they must always be partitioned together.
