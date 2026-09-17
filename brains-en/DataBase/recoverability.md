# Recoverability

Recoverability refers to the possibility of recovery when a transaction fails.

Transactions can fail midway for various reasons, such as hardware or software issues.

Even in such cases, transactions must guarantee atomicity, so if a transaction fails, it must be possible to restore to the previous state, which is called a rollback.

In short, it means there are no anomalies during rollback, and the DBMS must ensure that the schedule is recoverable.

### irrecoverable Schedule

This refers to a schedule where recovery to the previous state is impossible even after a rollback.

![](https://velog.velcdn.com/images/j_user0719/post/d071091d-c762-46ee-aa7e-8dec75c05042/image.png)

As shown above, a dirty read means that another transaction views the work in progress even though the current transaction has not yet finished its operation.

Ultimately, a scenario where a rollback cannot be performed while holding data read via a dirty read, as described above, is called an irrecoverable schedule.

<br>

### Recoverable Schedule

A recoverable schedule means that a transaction **does not commit until all other transactions that are modifying the data it is reading have either committed or rolled back.**

If T1 reads a value modified and written by T2, then T1 processes that value only after T2 (the dependent transaction) has completed.

If data is used in the order T1 -> T2 -> T3 -> T4, then transactions prior to T4 wait until T4 commits or rolls back.

However, this structure incurs significant costs during rollback because multiple transactions are nested and stacked.

### Cascadelss Schedule

To solve the above problem, the following schedule emerged: data can only be read after the transaction that wrote it has committed or rolled back.

Ultimately, this schedule prevents reads but does not block writes.

### Strict Schedule

A strict schedule is one that blocks writes as well, in addition to what Cascadless does.

If the sequence is W1(A) -> W2(A) -> W2 Commit -> W1 rollback, it's possible that when W1 rolls back, the data reverts to its state before W1 occurred, causing the data committed by W2 to disappear. However, since the above schedule (Cascadeless) itself has no reads, it can be considered cascadeless. A strict schedule further enhances this by not only preventing reads but also blocking writes.

To summarize, a recoverable schedule allows both reads and writes, but a dependent transaction cannot commit until the transaction it depends on has either committed or rolled back.

Cascadeless prevents reading data from uncommitted transactions, and strict blocks writes as well.
