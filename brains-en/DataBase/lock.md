# Pessimistic Lock, Optimistic Lock

### Pessimistic Lock

It provides isolation levels such as Repeatable Read and Serializable.

It starts by acquiring a Shared Lock or Exclusive Lock when a transaction begins.

![](https://velog.velgcdn.com/images/junsu1222/post/df009d23-1292-4ae8-a678-d12037062cf5/image.png)

In JPA, pessimistic locks can be used through PESSIMISTIC_READ (Read Lock), PESSIMISTIC_WRITE (Write Lock), and PESSIMISTIC_FORCE_INCREMENT (forced version increment).

**Cases where deadlocks occur with pessimistic locks**
1. t1: Acquires a lock on row 1 of table x.
2. t2: Acquires a lock on row 1 of table y.
3. t1: Tries to access row 1 of table y -> waits because t2 has already acquired a lock.
4. t2: Tries to access row 1 of table x -> waits because t1 has already acquired a lock.

<br>

### Optimistic Lock

Optimistic locking uses a distinguishing column like `version` and is a lock managed at the application level, rather than using features provided by the DB.

![](https://velog.velcdn.com/images/junsu1222/post/595af105-1cc1-4591-bff1-da37989d802e/image.png)

When an optimistic lock conflict occurs, an `ObjectOptimisticLockFailureException` is thrown, and the application needs to handle this exception.

In JPA, optimistic locks can be used via the `@Version` annotation. This means optimistic locking does not acquire a lock on the resource; instead, it handles concurrency issues only when they occur.

<br>

### Comparison

Let's compare optimistic and pessimistic locks.

Optimistic locking is performance-wise better than pessimistic locking because it manages locks at the application level during data retrieval before updates. It also does not require transactions. However, if a conflict occurs, the developer must manually handle the rollback.

Pessimistic locking prevents conflicts proactively and maintains data consistency. It can lead to degraded concurrent processing performance, making it less performant than optimistic locking, and it has the potential for deadlocks.

**When pessimistic locks are beneficial**
- Data integrity is critical (e.g., money, transfers)
- When many data conflicts are expected

**When optimistic locks are beneficial**
- When data conflicts are not expected to occur frequently
- When concurrent access performance is important due due to many read operations

To show an example of applying a pessimistic lock, let's say there's a group that members belong to. I'll write the `groupRepository.getTotalGroupMemberCount(groupId)` function in the business logic that calculates the number of members. A pessimistic lock can be applied to this query. Since reads from other transactions must be blocked, `PESSIMISTIC_WRITE` should be used.

```java
public Optional<Long> getTotalGroupMemberCount(final Long groupId) {
        return Optional.ofNullable(jpaQueryFactory
            .select(memberGroup.count())
            .from(memberGroup)
            .where(memberGroup.group.id.eq(groupId))
            .setLockMode(LockModeType.PESSIMISTIC_WRITE)
            .fetchOne()
        );
    }
```

However, because rows matching the `where` clause are locked, other transactions cannot perform read or write operations on those rows until the lock is released. Assuming a group has a maximum of 30 members, 30 rows would be locked. If logic involving read and write operations on the `memberGroup` table is frequently used in other features, this could lead to a degradation in concurrent processing performance.

Now, to solve these problems, let's explore [[Distributed Lock]] in addition to optimistic locks.
