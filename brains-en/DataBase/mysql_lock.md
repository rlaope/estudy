# MySQL 8.0 InnoDB Locking

### Lock Types

- Shared and Exclusive Locks
- Intention Locks
- Record Locks
- Gap Locks
- Next-Key Locks
- Insert Intention Locks
- Predicate Locks for Spatial Indexes

### Shared and Exclusive Locks

The two lock types, Shared Lock and Exclusive Lock, implement standard **row-level locking**.

A Shared Lock allows the transaction holding the lock to read the row.

An Exclusive Lock allows the transaction holding the lock to update or delete the row.

As an example of a shared lock, if transaction t1 holds a shared lock on row r, and some individual transaction t2 requests a lock on row r, t2's shared lock is immediately granted. As a result, both t1 and t2 hold shared locks on r. However, t2's exclusive lock cannot be granted immediately.

As an example of an exclusive lock, if transaction t1 holds an exclusive lock on row r, and another transaction t2 requests either type of lock (shared or exclusive) on row r, it cannot be granted immediately. Instead, transaction t2 must wait until transaction t1 releases its lock on row r.

<br>

### Intention Locks

InnoDB supports **multiple granularity locking**, allowing row-level and table-level locks to coexist.

What this means is, for example, a `LOCK TABLES ... WRITE` statement uses an exclusive lock on the specified table. To make locking practical at multiple granularity levels, InnoDB supports Intention Locks.

An Intention Lock is a table-level lock that **indicates the type of lock (shared or exclusive) a transaction intends to acquire later on individual rows of the table. There are two types of Intention Locks:**

1.  Intention Shared (IS) Lock: Indicates that a transaction **intends to set a shared lock** on individual rows of the table.
2.  Intention Exclusive (IX) Lock: Indicates that a transaction **intends to set an exclusive lock** on individual rows of the table.

Table-level lock type compatibility can be checked through the following table.

![](https://velog.velcdn.com/images/junsu1222/post/6dee8089-1851-4ed3-92ec-f969197c3e99/image.png)

If the requesting transaction is compatible with an existing lock, the lock is granted. However, if it conflicts with an existing lock, the lock is not granted. The transaction waits until the conflicting existing lock is released. If a lock request conflicts with an existing lock and causes a deadlock, preventing the lock from being granted, an error occurs.

Intention Locks do not block anything except full table requests (e.g., `LOCK TABLES ... WRITE`). In other words, **the main purpose of an Intention Lock is to indicate that someone is locking rows or intends to lock rows in the table.**

For example, in the table, since IX and IX are Compatible, if transaction t1 holds an IX on a table, transaction T2 cannot hold an IX. However, if transaction t1 holds an X on a row, T2 cannot hold an X.

<br>

### Record Locks

A Record Lock is a lock on an index record. Simply put, it's a lock on a real record.

For example, the query `SELECT c1 FROM t WHERE c1 = 10 FOR UPDATE;` prevents other transactions from inserting, updating, or deleting rows where t.c1 has a value of 10.

Record locks always lock index records, even if the table is defined without an index. In such cases, InnoDB creates a hidden clustered index and uses it for record locking.

### Gap Locks

A Gap Lock is a lock on the **gap between index records, or on the gap before the first or after the last index record.**

For example, `SELECT c1 FROM t WHERE BETWEEN 10 AND 20 FOR UPDATE;` locks the gaps between all existing values in the range, preventing other transactions from inserting the value 15 into the t.c1 column, regardless of whether that value already exists in the column.

Gap locks may or may not span a single index value or multiple index values. I believe gap locks are part of a trade-off between performance and concurrency.

Different transactions can hold conflicting locks on a gap. For example, transaction t1 can hold a shared gap lock on a gap, and transaction t2 can hold an exclusive gap lock on the same gap. Conflicting gap locks are allowed because if a record is removed from an index, other transactions holding gap locks on that record must merge their locks.

In other words, InnoDB's gap locking feature is **purely suppressive**, meaning its sole purpose is to prevent other transactions from inserting into the gap.

### Next Key Locks

A Next-Key Lock is a **combination of a Record Lock on an index record and a Gap Lock on the gap before the index record.**

InnoDB performs row-level locking by setting shared or exclusive locks on the index records it encounters when searching or scanning a table index. Therefore, **row-level locks are actually index record locks.**

A Next-Key Lock on an index record also affects the gap immediately preceding that index record. If one session sets a Share or Exclusive Lock on index record R, another session cannot insert a new index record into the gap immediately preceding R in index order.

For example, if an index contains the values 10, 11, 13, 29, the possible next-key locks for this index include the following intervals, where round brackets exclude the interval endpoints and square brackets include the endpoints:

```java
(negative infinity, 10]
(10, 11]
(11, 13]
(13, 20]
(20, positive infinity)
```

In the last interval, the Next-Key Lock locks the gap between the largest value in the index and a supremum pseudo-record with a value higher than any actual index value. Since the supremum record is not an actual index record, this Next-Key Lock effectively **locks only the gap after the largest value in the index.**

<br>

### Insert Intention Locks

An Insert Intention Lock is a type of gap lock set by an INSERT operation before inserting a row.

It signals the intention to insert, so that multiple transactions inserting into the same index gap do not need to wait for each other if they are not inserting into the exact same position within the gap.

For example, suppose there are index records with values 4 and 7. Separate transactions intending to insert values 5 and 6, respectively, would each lock the gap between 4 and 7 with their respective insert intention locks before acquiring exclusive locks on their inserted rows. However, since the rows do not conflict, **they do not block each other.**

The following example further illustrates a transaction taking an Insert Intention Lock before acquiring an exclusive lock on the inserted record. Client 1 creates a table containing two instance records (90, 102) and then starts a transaction that sets an exclusive lock on index records with IDs greater than 100. This exclusive lock includes a gap lock before record 102. Client 2 starts a transaction that inserts a record into the gap, and this transaction takes an Insert Intention Lock while waiting to acquire the exclusive lock.

```java
// A
mysql> CREATE TABLE child (id int(11) NOT NULL, PRIMARY KEY(id)) ENGINE=InnoDB;
mysql> INSERT INTO child (id) values (90),(102);

mysql> START TRANSACTION;
mysql> SELECT * FROM child WHERE id > 100 FOR UPDATE;
+-----+
| id  |
+-----+
| 102 |
+-----+

// B
mysql> START TRANSACTION;
mysql> INSERT INTO child (id) VALUES (101);
```

<br>

### AUTO-INC Locks

An AUTO-INC Lock is a special table-level lock taken by transactions inserting into a table with an AUTO_INCREMENT column.

For example, if one transaction inserts values into a table, other transactions wait to perform insertions into that table. The rows inserted by the first transaction receive consecutive primary key values.

The `innodb_autoinc_lock_mode` system variable controls the algorithm used for auto-increment locking. This variable allows you to choose a trade-off between predictable sequences of auto-increment values and maximum concurrency for operations.

<br>

### Predicate Locks for Spatial Indexes

**InnoDB supports spatial indexing for columns containing spatial data.**

Next-key locks are not suitable for handling locks on operations related to spatial indexes at `REPEATABLE READ` or `SERIALIZABLE` isolation levels. This is because multi-dimensional data lacks a clear concept of absolute order, making it unclear what "next" means.

To support isolation levels for tables with SPATIAL indexes, InnoDB uses Predicate Locks. Since SPATIAL indexes contain Minimum Bounding Rectangle (MBR) values, InnoDB sets predicate locks on the MBR values used in queries to enforce consistent reads in the index. Other transactions cannot insert or modify rows that match the query conditions.
