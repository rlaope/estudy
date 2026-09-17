# Failover Consistency

Let's assume a scenario where a Redis master dies, and a replica is promoted to master via failover. Due to the asynchronous replication structure, some writes are lost, leading to the system serving old values.

In this situation, we can build an environment to either reduce this problem proactively or detect and recover from it reactively. Let's explore each approach.

### 1. Not using Redis as the true source of truth

This is an easy and obvious point: for values that must not be wrong, such as money, balances, or contract statuses, it's safer to use a persistent DB like MySQL as the source of truth and use Redis for caching, session information, and other supplementary data.

In a master-down + async replication environment, writes that have already been lost cannot be technically recovered. From the new master's perspective, it doesn't even know such writes existed.

If consistency is critical, Redis should be updated only after a successful DB commit. This also implies a lower consistency for the cache store.

### 2. Methods to reduce the probability of incorrect values during writes

Redis uses `WAIT` to write in a semi-synchronous manner.

While Redis's default is asynchronous replication, clients can use the `WAIT` command to request that a write waits until N replicas have received it, for a maximum of M milliseconds.

```lua
-- 1) 값 쓰기
SET user:123:balance 10000

-- 2) 현재 마스터 기준, 복제본 1개 이상이 받아갈 때까지 최대 100ms 기다리기
WAIT 1 100
```

In practice, you can call `WAIT` after `SET`, or bundle them within a `MULTI` transaction or Lua script.

The advantage is an increased probability that values written just before the master dies will also be present on replicas, but the disadvantage is increased write latency.

If the specified number of replicas are slow or down, the write might fail or time out.

Nevertheless, it's often used selectively for keys that require safe writes, even if slightly slower, rather than completely losing data.

Redis options include the following:

```
min-replicas-to-write 1
min-replicas-max-lag 5
```

This means accepting writes only if there is at least 1 replica and the replication lag with that replica is within 5 seconds.

If these conditions are not met, the write command returns an error. In other words, it prevents critical writes from occurring when the replication status is poor.

This prevents a scenario where a master writes extensively and then dies while a slave is severely lagging, leading to that heavily lagged slave being promoted.

### Patterns for verifying if a value can be trusted at read time

If a failover has already occurred, you can verify at the application level whether the key's value is up-to-date.

For example, if Redis stores data like this:

```json
"user:123:profile" = {
  "version": 42,
  "updated_at": "2025-12-10T01:23:45Z",
  "name": "홍길동",
  "grade": "VIP"
}
```

The application assumes that the DB (source of truth) always holds a higher version.

If the version read from Redis is smaller than what's in the DB, it's stable. If it's smaller than the previous version known by the client, it indicates rollback detection.

The processing pattern in this case is:
- When in a suspicious state, re-read from the DB to ensure consistency, then
- Overwrite Redis again. (read-through + repair)

This allows for self-healing during the first few requests after a failover, by detecting 'Oh, this version has decreased.'

#### Redis is cache, DB is the real value; if in doubt, re-query DB + rebuild cache

A slightly simpler pattern is:

1. Read from Redis
2. If the value is nonsensical from a business perspective (e.g., negative balance, status reversal, etc.)
   1. Re-read from DB
   2. Overwrite Redis value

```kt
fun getUserBalance(userId: Long): Long {
    val cached = redis.get("user:$userId:balance")?.toLongOrNull()

    if (cached != null && cached >= 0) {
        // 정상적인 값으로 보이면 그냥 사용
        return cached
    }

    // 이상한 값이거나 없으면 DB를 소스로 사용
    val dbValue = userRepository.findBalance(userId)
    redis.set("user:$userId:balance", dbValue.toString())
    return dbValue
}
```

The core idea is to assume that Redis (cache) can always be wrong and to establish rules for each domain on how to detect errors and to what extent to roll back/recover.

### Designing the failover process itself conservatively

There are also things that can be done from an operational perspective.

1. When using automatic failover tools (Sentinel, Cluster)
   1. Do not set promotion conditions too aggressively (quickly),
   2. Adjust timeouts to properly distinguish between network partitions and actual failures.
2. After failover
   1. If possible, force reads for specific domains to prioritize the DB for a short period (several seconds to tens of seconds).
   2. Another strategy is to run a forced rebuild (inconsistency check batch) for critical keys. Of course, this might cause a traffic surge to the DB, so it's a trade-off.
3. If the old master comes back online
   1. It must be re-registered as a replica of the new master to prevent it from being promoted again with incorrect data.

### If an incorrect value has already been served

If an incorrect value has already been served to the client, atomic rollback is technically impossible.

It must be resolved with compensatory logic at the domain level.

For example, even if an incorrect balance was displayed, actual withdrawals should be verified against the DB and blocked if inconsistent.

If business logic was actually executed based on an incorrect state,
- Recalculate based on audit logs/event logs
- Provide customer compensation, re-sending, settlement batches, or even a re-query button on the customer UI.

Therefore, it's important to leave audit event logs throughout the entire flow, including cache, DB, and message queues,

to be able to reconstruct the scope of impact during a failure.

## Summary

Ultimately, this involves adjusting C and A in CAP theory, as both cannot be fully satisfied.

If consistency is critical, it's more important to focus on consistent behavior and data integrity, even if it takes a bit longer, rather than reducing latency with a cache.

If data is not critical and accommodating high traffic is paramount, it's better to slightly lower the consistency level and handle it with corresponding domain logic, retry logic, or manual developer intervention.

If both are needed to some extent, let's design a BASE-based system through appropriate compromise, ensuring eventual consistency, soft state, and basic availability.

In any case, it's crucial to design the system by carefully considering the trade-offs based on the requirements.
