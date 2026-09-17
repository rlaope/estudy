# Deadlock

When two or more transactions are running concurrently and each holds a lock that the other needs, causing them to fall into a waiting state and unable to proceed, this situation is called a **Deadlock**.
- Deadlock is a common problem in databases that support transactions
- Deadlocks in multi-threaded applications are dangerous because they can completely halt the application
- Most DBMS have deadlock detection that automatically resolves deadlocks when they occur (even if it's not an actual deadlock, it may be treated as one if the timeout setting is exceeded)
  - During this process, some of the transactions in progress may be canceled, so the application level should be configured to recover by re-executing those transactions

### Lock Types
- **shared lock**: slock, a read lock used when reading a specific row. If transaction A holds an slock, transaction B can only read but cannot write. In other words, other transactions can acquire slock but not xlock
- **exclusive lock**: xlock, a write lock used when writing to a specific row. If transaction A holds an xlock on a specific row, transaction B cannot read or write to that row. It must wait for the lock to be released by that transaction. In other words, other transactions cannot acquire slock or xlock
- **Intention Lock**: A lock placed at the table level in advance to indicate the intention to place a certain row-level lock on a specific row within the table later
  - Intention Shared Lock (IS) and Intention Exclusive Lock (IX) exist
    - When `select .. lock in share mode` is executed, IS is acquired first, then slock is acquired at the row level
    - For `select .. for update`, `insert`, `delete`, `update`, IX is acquired first, then xlock is acquired at the row level

The reason for using intention locks is to fundamentally prevent transaction B from acquiring a lock on a specific row in a table when transaction A already has a lock on that table (to avoid deadlock)
- When row-level writes are occurring, the table schema should not be changed. Since write queries have already acquired IX, they can also prevent table schema changes
- When updating, the lock is usually placed on the updated row, but if the WHERE clause doesn't properly use an index or there is no index, a table lock is acquired

### Ways to Reduce Deadlocks

- Keep transactions concise
- Configure indexes well (the fewer records scanned, the narrower the lock range)
- When using locking reads (`select for update`, `select lock in share mode`), actively use lower transaction isolation levels like Read Committed if possible (consistency may be slightly reduced)
- When modifying multiple data within a transaction, always make the lock order sequential
  - ex. When modifying tables a, b, c, if each transaction modifies in the order a, b, c, the probability is lower
  - For example, if transactions run concurrently in the application modifying different data in orders like a, c, b or c, b, a, the probability of deadlock increases
    - Using table locks as an example, if the first transaction locks a and the second locks c, when the first transaction tries to modify c, the second transaction already holds the lock. After modifying b, it tries to lock a, but the first transaction holds it, so it falls into a waiting state and deadlock occurs
- Be careful with batch `insert on duplicate key update` that includes multiple rows in one statement. A single batch statement behaves like multiple statements with transactions, so if the PKs of data included in each batch query overlap, there is a probability of deadlock
- You can completely serialize transactions
  - Create a semaphore table with only one row of data, have each transaction update the semaphore table before accessing other tables, and transactions that access the semaphore table later fall into a waiting state, guaranteeing complete sequential execution of each transaction

<br>

### Deadlock Stat

You can check if a deadlock is occurring or what locks are in place using the **SHOW ENGINE INNODB STATUS;** command.

```
------- TRX HAS BEEN WAITING 5 SEC FOR THIS LOCK TO BE GRANTED:
RECORD LOCKS space id 770 page no 3 n bits 80 index PRIMARY of table `example_db`.`test` trx id 774317 lock_mode X locks rec but not gap waiting
```

You can see a message like above indicating that there is a transaction waiting due to an exclusive lock (lock mode x) on example_db.

### Example

With batch insert on duplicate, query1: insert a at pk:1, insert b at pk:2, query2: a at pk:2, b at pk:1. Query1 acquires lock on row upsert pk=1, query2 acquires lock on pk=2. If these two queries unluckily enter different connections and behave as if bundled together, operating in the above order, a deadlock occurs.

The same applies to insert statements. When executing inserts for the same key in the order a, b, c, a, b, c start and a locks first, but the moment it rolls back, b and c compete. In the case of insert, an exclusive lock must be acquired, but when a duplicated key error occurs, it has the characteristic of first acquiring a shared lock on that index record. So b and c first acquire shared locks on the same index record and then try to acquire exclusive locks, causing a deadlock (b can't acquire xlock because of c's slock, and vice versa for c->b). If a commits here, b and c immediately get duplicated key errors, but the transactions are still open. Eventually, in this situation, when the preceding transaction rolls back, only one of the remaining transactions succeeds and the rest are forcibly rolled back due to deadlock. This case may not seem likely at first glance, but remember it.

In the case right above, insert is a very unusual situation, but you can confirm that delete works normally without being unusual.
- When competing for the same key with delete in the order a, b, c
  - a acquires xlock
  - b waits for xlock, c waits for xlock
  - When a commits, the row that is the target of the lock disappears, so both b and c end their waiting state and return affected row=0. The transaction remains open
  - When a rolls back, b acquires xlock, c continues waiting

Comparing the two situations above, in the case of insert, when a duplicated key error occurs, it attempts to acquire a shared lock, so more than one transaction simultaneously acquires a shared lock on the same row (because shared locks can be acquired simultaneously). Transaction a terminates, but when transaction b tries to acquire xlock again, it fails because of c's shared lock. Vice versa for the opposite case.

But in the case of delete, since it attempts to acquire xlock, only one transaction can acquire the lock, so the rest wait and no deadlock occurs.

### Remediation Methods

- Query improvement: prevent the above situations from occurring
- Isolation level adjustment: to make it serializable
- Adjust deadlock detection options in the storage engine
  - deadlock_search_depth_long
  - deadlock_search_depth_short
  - deadlock_timeout_long
  - deadlock_timeout_short
