# MySQL MyISAM, InnoDB

Before understanding the differences, both are common MySQL storage engines.

A storage engine is an internal module that implements how SQL data is actually stored, retrieved, and modified.

Even for the same `SELECT`, `INSERT`, or `UPDATE` operations, how data is written to disk, how locks are applied, and how recovery is performed are all responsibilities of the storage engine.

### MyISAM

MyISAM is an engine used since the early days of MySQL, with its core goals being **simplicity and fast reads.**

It stores data and indexes separately as `.MYD` and `.MYI` files directly on the file system.

It's closer to a high-performance file access layer than a database engine.

Thanks to this structure, implementation is simple and read performance is fast, but there are almost no internal mechanisms for data protection.

#### Lock, Concurrency Features

MyISAM does not support transactions at all. There is no `COMMIT` or `ROLLBACK`, and multiple queries cannot be grouped into a logical operation.

It only provides table-level locks; when one session starts writing, the entire table is locked.

Because of this, performance rapidly collapses in concurrent write scenarios, and data consistency becomes entirely the application's responsibility.

#### Limitations

If the server terminates abnormally, table files can be corrupted, requiring manual recovery.

Data loss can also occur during the recovery process. It does not support foreign keys, so referential integrity is not guaranteed.

Consequently, there is no reason to use it in modern services, except for legacy purposes that are close to read-only.

### InnoDB

InnoDB is an engine designed from the outset with transaction data consistency as its goal.

All data changes occur within transactions, and it directly manages memory, centered around the Buffer Pool.

Data is stored in a clustered index structure, sorted by the primary key, which provides predictable access patterns and stable performance.

#### Lock Model, MVCC

InnoDB supports ACID transactions and is designed to minimize interference between reads and writes through MVCC.

Locks are primarily row-level, and locking only occurs when contention is necessary.

Additional locks like gap locks and next-key locks are design choices to prevent phantom reads (a phenomenon where query results differ within the same transaction), prioritizing consistency.

#### Crash Recovery

InnoDB ensures crash safety using redo logs and undo logs.

Even if the server terminates abruptly, it automatically recovers to a consistent state based on the logs upon restart.

It supports foreign keys, allowing referential integrity to be enforced at the DB level, and with its concurrent write stability, it overwhelmingly outperforms MyISAM in real-world service performance. It is effectively the standard.

- MyISAM: Simple, fast reads; a legacy engine that sacrifices transactions, recovery, and concurrency.
- InnoDB: A modern OLTP standard engine responsible for transactions, consistency, and recovery.
