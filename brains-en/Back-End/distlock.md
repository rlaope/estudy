# When a master node goes down after acquiring a distributed lock

Let's assume there's a system operating with a Redis-based distributed lock.

Redis master-slave replication inherently operates asynchronously, which can lead to the following consistency issues:

1.  Client 1 connects to the master node and acquires a lock for a resource.
2.  The master node responds with success to the client, but goes down just before replicating the lock information to the slave nodes.
3.  Redis Sentinel or the cluster detects this and performs a failover.
4.  The new master doesn't receive the lock information from the previous master, thus it has no lock.
5.  At this point, client 2 requests and is granted a lock for the same resource on the new master, resulting in a double lock.

### 1. Redlock

This problem arises because there's only one master. To prevent a single master failure from leading to a complete system outage,

Redlock uses a setup with five or more independent masters.

A client requests a lock from all master nodes. The lock is considered successfully acquired only if it's granted by a majority (n / 2 + 1) of the nodes and the time elapsed is less than the lock's validity period.

Even if a specific node goes down, the remaining majority of nodes retain the lock information, making it very safe against data loss.

However, the downside is that it requires sending network requests to multiple nodes, which is costly, introduces latency, and can complicate infrastructure configuration.

```java
// 독립적인 3개의 마스터 노드 설정 (권장 5개)
RLock lock1 = redissonClient1.getLock("resource_lock");
RLock lock2 = redissonClient2.getLock("resource_lock");
RLock lock3 = redissonClient3.getLock("resource_lock");

// Redlock 인터페이스를 통해 여러 락을 하나로 관리
RedissonMultiLock lock = new RedissonMultiLock(lock1, lock2, lock3);

try {
    // 10초 동안 대기하며 60초 동안 락을 점유 (과반수 성공 확인)
    boolean res = lock.tryLock(10, 60, TimeUnit.SECONDS);
    if (res) {
        // 비즈니스 로직 수행
    }
} catch (InterruptedException e) {
    Thread.currentThread().interrupt();
} finally {
    // 모든 마스터 노드에 해제 요청
    lock.unlock();
}
```

<br>

### WAIT

Redis can resolve master-slave inconsistency issues using WAIT. Simply put, it makes replication behave like a synchronous process.

If a lock is acquired with SET, the master waits via WAIT until the data is actually replicated to the specified number of slave nodes.

This significantly reduces the possibility of data loss without complex algorithms, but the master must wait for the slaves to respond, meaning some of Redis's high-performance advantages must be sacrificed.

If replication is delayed due to network failures or other issues, response times will decrease.

```java
RedisCommands<String, String> commands = connection.sync();

// 1. NX(존재하지 않을 때만), PX(밀리초 단위 만료시간) 옵션으로 락 취득
SetArgs setArgs = SetArgs.Builder.nx().px(30000);
String result = commands.set("lock:resource", "client_id_1", setArgs);

if ("OK".equals(result)) {
    // 2. WAIT 1 1000: 최소 1개의 슬레이브에 복제될 때까지 최대 1000ms 동안 대기
    // dispatch를 통해 직접 Redis 커맨드 실행 가능
    Long syncedSlaves = commands.dispatch(CommandType.WAIT, new IntegerOutput(codec), 
                                          new CommandArgs<>(codec).add(1).add(1000));
    
    if (syncedSlaves != null && syncedSlaves >= 1) {
        // 복제가 확인되었으므로 안전하게 로직 수행
        doBusinessLogic();
    } else {
        // 복제 확인 실패 시 락을 즉시 해제하고 실패 처리
        commands.del("lock:resource");
    }
}
```

<br>

### Fencing Token

There can be a verification method within the application, which is the approach proposed by distributed systems expert Martin Kleppmann.

It's a robust method that acknowledges the inherent imperfections of locks themselves and performs final validation in the ultimate storage layer.

1.  When a client acquires a lock, it receives a monotonically increasing token (e.g., 1, 2, 3, 4).
2.  When the client writes data to the shared resource DB, it sends the token along.
3.  The storage rejects requests with a token number lower than the currently recorded token.
4.  This implies that the master has been replaced, and the previous client's lock is no longer valid.

The advantage is that it can prevent duplicate lock acquisitions even if the master is replaced, and it maintains final data consistency.

However, since it involves writing at the DB level, performance might be somewhat reduced.

```java
// 1. Redis에서 락과 함께 토큰(예: 34)을 얻었다고 가정
val currentToken: Long = 34

// 2. DB 업데이트 시 본인의 토큰이 최신인지 확인하며 실행
@Repository
interface ResourceRepository : JpaRepository<ResourceEntity, Long> {
    @Modifying
    @Query("""
        UPDATE ResourceEntity r 
        SET r.data = :newData, r.lastToken = :token 
        WHERE r.id = :id AND r.lastToken < :token
    """)
    fun updateWithFencing(id: Long, newData: String, token: Long): Int
}

// 서비스 레이어
val updatedRows = resourceRepository.updateWithFencing(resourceId, "value", currentToken)
if (updatedRows == 0) {
    // 본인보다 높은 토큰(예: 35)이 이미 처리됨 -> 무효한 작업으로 간주
    throw StaleLockException("이미 만료된 락 토큰입니다.")
}
```

<br>

### Consistency-First Systems

Ultimately, if the reliability or absolute strategy of distributed locks is crucial, all three methods above sacrifice performance to achieve consistency.

This is a way to adjust C/A in CAP theory. By considering ZooKeeper and etcd, which guarantee CP (Consistency Partition Tolerance, consistency-first), we can pay more attention to C. These systems:

Use quorum write (consensus algorithm-based writing), where a write request to the leader node only responds with success if the data is written to a majority of nodes. Even if the leader goes down, one of the remaining nodes will always have the latest lock information.

Employ robust leader election mechanisms like Raft and ZAB to elect the node with the most up-to-date data, preventing a scenario where a node with lost lock information becomes the leader.

They are designed so that if the TCP session between the client and server is disconnected (via sessions and ephemeral nodes), the lock is immediately released. This process is also handled consistently across all nodes. Of course, their performance is significantly lower than Redis.

There's no single right answer. If your business logic ensures idempotency, Redlock can be used. If data consistency is extremely critical, the Fencing Token approach is viable, and WAIT is also an option. In any case, these are likely better than `db lock for update`.
