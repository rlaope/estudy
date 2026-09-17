### MRR, BKA

When the MySQL server's optimizer formulates an execution plan, it combines statistical information and optimizer options to create the optimal plan.

**Optimizer options are broadly categorized into join-related optimizer options and optimizer switches.**

In this post, we will explore MRR and Batched Key Access, which are among the techniques the optimizer uses to optimize join performance.

<br>

### MRR(Multi Range Read)

It is also known as DS-MRR (Disk Sweep Multi Range Read).

The join method supported by the MySQL server until now involves reading one record from the driving table and then finding matching records in the driven table to perform the join.

This is called Nested Loop Join. Due to the internal structure of the MySQL server, join processing is handled by the MySQL engine, but the actual searching and reading of records is the responsibility of the storage engine.

In this scenario, if records in the driven table are searched for each record in the driving table, the storage engine, which is responsible for finding and reading records, cannot perform any optimization.

To compensate for this drawback, the MySQL server uses a **join buffer** to read records from one of the tables involved in the join, buffer those values, and then minimize the number of requests to the storage engine so it can read records in a single batch. This allows for accessing data in sorted order, thereby minimizing disk data page reads.

> Even if the data pages are already in memory, for example, in the InnoDB buffer pool, this still minimizes access to the buffer pool.

This reading method is called MRR, and the join method executed by applying this technique is called BKA (Batched Key Access) join.

BKA join optimization is disabled by default because BKA joins have disadvantages.

Depending on the query's characteristics, it can sometimes be beneficial, but other times it may require additional sorting operations, which can negatively impact performance. Therefore, it should be applied with careful consideration.
