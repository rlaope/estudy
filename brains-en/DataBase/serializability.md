# Serializability

When transactions are executed concurrently, various anomalies can occur.

Serializability and Recoverability are important properties that prevent these anomalies from happening.

## Serializability

It is essential for a DBMS to execute multiple user requests simultaneously.

However, when executed concurrently, it must eliminate interference between transactions and consistently ensure data integrity.

This is expressed as ensuring Isolation, or in other words, ensuring Serializability.

Serializability means serialization, which refers to sequentially serializing transaction requests to eliminate transaction interference and ensure data consistency between data.

So, how can we apply this Serializability?

<br>

### Schedule

A schedule is the execution order of operations belonging to multiple transactions when they are executed concurrently.

When multiple transactions are performed, they are divided into `serial-schedule` and `non-serial-schedule` depending on whether the operations of each transaction are executed sequentially or not.

1/ **A non-serial-schedule means that internal operations within transactions are executed in parallel through an interleaved execution technique.**

> Interleaving is a memory technique that increases availability by alternating between different memory banks. Let's learn more about it later...

This method has the advantage of increased concurrency because it processes tasks in parallel.

It can perform other tasks while another transaction's I/O operation is in progress.

However, it leads to issues where data consistency cannot be guaranteed.

2. **A serial-schedule is a technique that separates transactions to prevent overlap, executing one transaction completely before starting another.**

The advantage of this technique is that it can perfectly guarantee order, but it suffers from lower performance. This is because other transactions wait until the I/O operation is finished.

<br>

### Serializability Schedule

The two methods above have very critical trade-offs, and to resolve this, to improve performance, we use non-serial schedules while ensuring the same results as serial schedules.

This is also called `conflict serializable` and is considered `conflict equivalent` to a `serial schedule`.

A conflict means that two operations clash. Here, a conflict satisfies the following three conditions:
1. The operations must belong to two different transactions.
2. The operations must work on the same data.
3. At least one of the operations must be a write operation.

Conflict Equivalence satisfies the following two conditions:
1. Two schedules composed of the same transaction operations.
2. The execution order of conflicting operations within both transactions is identical.

If a non-serial schedule is conflict equivalent to a serial schedule, applying the non-serial schedule can ensure serializability while also improving performance.
