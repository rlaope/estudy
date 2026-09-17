# Controlling Cache Stampede (Distributed Lock, PER)

Cache stampede refers to a phenomenon where, in situations of high traffic, when the TTL (Time To Live) of a specific data's cache expires, numerous concurrent requests experience a cache miss and simultaneously query the original database for data.

A **Distributed Lock** is a synchronization mechanism that allows only a single process to access a shared resource (in this case, DB query and cache update permissions) when multiple servers or processes try to access it, thereby preventing concurrency issues. It is typically implemented using Redis's single-threaded nature and atomic operations.

The reason for mentioning distributed locks here is that they act as a shield to prevent the database from crashing due to cache stampede. Let's compare the moment a cache expires when tens of thousands of requests pour in.

1.  Without a lock
    1.  Assume 10,000 requests arrive at the cache expiration time.
    2.  All 10,000 threads confirm that the cache is empty.
    3.  All 10,000 threads simultaneously send the same data query to the DB. The DB cannot withstand the concurrent connections and CPU load and crashes.
2.  With a distributed lock
    1.  Similarly, 10,000 requests arrive at the expiration time.
    2.  Seeing the cache is empty, they try to go to the DB, but only 1 thread acquires access.
    3.  Only the 1 thread that acquired the lock goes to the DB, while the other 9,999 wait briefly. This is not a heavy DB lock; rather, it's a decision to sacrifice server resources by holding connections on the server itself for longer than on the DB.

### Specific Situations Leading to DB Failure

Let's assume a large-scale event page opens at 12:00 PM.

1.  **Traffic Concentration and Cache Expiration**: Immediately after opening, 50,000 requests per second come in, and the TTL of the cache providing main page data expires at 12:05:00 PM.
2.  **Concurrent Cache Misses**: At 12:05:00 PM, hundreds to thousands of incoming request threads simultaneously check the cache store and find no data.
3.  **DB Connection Surge**: All threads experiencing cache misses simultaneously send heavy queries to the RDB.
4.  **Cascading Failure**: The RDB fails to process the sudden thousands of concurrent queries, CPU usage spikes to 100%, and the DB connection pool is exhausted. Consequently, the database becomes unresponsive, and application server threads waiting for it also enter a waiting state, paralyzing the entire service.

Typically, Redisson or Lettuce are often used in Kotlin environments for implementation. These are Redis client libraries, and Redisson is often preferred because it uses a pub/sub mechanism instead of spinlocks, resulting in less load, and it provides well-designed interfaces.

```kt
import org.redisson.api.RedissonClient
import java.util.concurrent.TimeUnit

class EventService(
    private val redissonClient: RedissonClient,
    private val eventRepository: EventRepository,
    private val cacheManager: CacheManager
) {
    fun getEventData(eventId: String): EventData? {
        val cacheKey = "event:$eventId"
        
        // 1. 캐시 조회 (Cache Hit)
        val cachedData = cacheManager.get(cacheKey)
        if (cachedData != null) return cachedData

        val lockKey = "lock:event:$eventId"
        val lock = redissonClient.getLock(lockKey)

        try {
            // 2. 락 획득 시도 (대기 시간 3초, 락 점유 시간 2초)
            val isLocked = lock.tryLock(3, 2, TimeUnit.SECONDS)
            if (isLocked) {
                // 3. Double Check: 락을 대기하던 스레드가 락을 획득한 경우, 
                // 앞선 스레드가 이미 캐시를 갱신했을 수 있으므로 다시 한번 확인합니다.
                val recheckedData = cacheManager.get(cacheKey)
                if (recheckedData != null) return recheckedData

                // 4. DB 조회 및 캐시 갱신 (단 하나의 스레드만 실행됨)
                val dbData = eventRepository.findById(eventId)
                if (dbData != null) {
                    cacheManager.set(cacheKey, dbData, 60, TimeUnit.SECONDS)
                }
                return dbData
            } else {
                // 락 획득 실패 시: 너무 많은 대기자가 있거나 락 타임아웃 발생
                // Fallback 전략 실행 (예: 예전 캐시 데이터 반환 또는 사용자에게 잠시 후 시도 안내)
                throw RuntimeException("현재 접속자가 많아 처리가 지연되고 있습니다.")
            }
        } finally {
            // 5. 락 해제 (자신이 획득한 락인지 확인 후 해제)
            if (lock.isLocked && lock.isHeldByCurrentThread) {
                lock.unlock()
            }
        }
    }
}
```

If you implement it with the flow above and monitor with `redis-cli`:

-   `INFO memory`: Check Redis memory usage, fragmentation ratio, and the number of `evicted_keys` due to insufficient cache space to assess infrastructure capacity.
-   `SLOWLOG get 10`: Query for queries whose execution time exceeds the configured threshold. Since Redis is single-threaded, one slow query can affect the entire system.
-   `MONITOR`: Streams all commands processed by the server in real-time. **Caution: Running this in a production environment can degrade performance by up to 50%. Use it only for extremely limited situations like debugging failures, or in development/staging environments for short periods.**

### Trade-offs and Fallback Strategies

Introducing distributed locks can prevent DB downtime due to cache stampede, but it also introduces new considerations.

-   **Latency Trade-off**: Additional network communication costs with Redis are incurred to acquire the lock. Furthermore, threads that fail to acquire the lock and wait will experience response delays for that duration, which can lead to the exhaustion of worker threads in Tomcat, etc. An appropriate `waitTime` is essential.
-   **Fallback on Redis Failure**: You also need to consider what happens if Redis itself, which manages distributed locks, fails. Strategies include reducing Redis dependency by placing a local cache with a short TTL in the server's Caffeine cache, or using a circuit breaker & stale data approach where, if Redis doesn't respond, the circuit opens, and instead of a DB query, previously expired data is returned to prioritize system survival. Alternatively, to avoid the latency of distributed locks, **PER** (Probabilistic Early Recomputation) can be considered, a technique where a specific request, chosen with a random probability, proactively updates the cache in the background before the TTL completely expires.

### PER

Distributed locks have limitations in preventing stampede (increased latency for waiting threads).

As an alternative to complement this, there is PER. PER stands for Probabilistic Early Recomputation, a lock-free algorithm based on the idea of **assigning the task of proactively updating the cache to one random thread among those accessing it just before the cache expires, instead of using a lock to make other threads wait.**

The core formula of the XFetch algorithm, theoretically implemented mainly in Varnish and Redis environments, is as follows:

$$t \ge t_{exp} - \Delta \cdot \beta \cdot \log(rand())$$

-   $t$ is the current time
-   $t_{exp}$ is the actual expiration time of the cache
-   $\Delta$ is the time it takes to recompute the cache by querying data from the DB
-   $\beta$ is a weighting factor to adjust the probability
-   $rand()$ is a random number between 0 and 1

As the current time `t` approaches the expiration time and as the query time `delta` increases, the probability of the formula being true increases exponentially. This means that one thread quietly fetches data from the DB in the background and extends the TTL before the cache fully expires, thereby fundamentally preventing large-scale cache misses and stampedes.

Here's a small question that might arise: doesn't the data disappear after the TTL? A cache miss can be interpreted as either data not existing or TTL having expired.

However, the conclusion is that PER does not recover data after TTL expires but proactively updates the cache just before expiration. If data were deleted the moment TTL passes, relying solely on basic Redis functionality, this algorithm would not work.

Therefore, it is common to use an architecture that separates TTL into logical TTL and physical TTL.

#### Physical Expiration and Logical Expiration

When implementing PER, instead of simply putting values into the cache, the data itself, including metadata, is cached.

-   **Logical TTL**: This is the time considered by the application layer as the point until which the data is fresh. It is stored as a value within the cache payload.
-   **Physical TTL**: This is the time when the Redis system actually reclaims memory and deletes the data. It is set much longer than the logical TTL to provide a buffer.

```json
{
  "data": { "eventName": "초특가 할인", "items": [...] },
  "logical_expire_at": 1711680000,  // 논리적 만료 시점 (예: 5분 뒤)
  "delta": 2.5 // DB 조회에 걸리는 예상 시간 (초)
}
```

So, let's look at a real scenario.

If the logical expiration time is 12:05:00 PM and physical expiration is 12:10 PM:

-   **12:04:58 PM, 2 seconds before logical expiration**
    -   A request comes in, and the application retrieves the data and checks `logical_expire_at`.
    -   At this point, the PER probability formula activates. Since it's close to the expiration time, the thread handling this request has about a 10% chance of being "selected."
    -   The selected thread immediately returns the existing cached data to the user, eliminating delay.
    -   Simultaneously, it **spins up a background asynchronous thread to query the latest data from the DB**, overwriting the Redis value and extending the logical expiration time.
-   **12:05:01 PM, 1 second after logical expiration, if not updated**
    -   If traffic is low or by bad luck no one was selected by the PER formula in the previous step.
    -   Even if a new request comes in at this point, a cache miss does not occur. This is because the physical expiration at 12:10 PM is still in the future.
    -   The application reads the cache and recognizes that the logical time has passed. At this point, a 100% probability update trigger is activated, and one thread goes to query the DB.
    -   **Important**: While the DB is being queried, all other incoming request threads do not bounce due to an empty cache or wait for a lock. Instead, they **receive the existing cached data, which is expired but physically still present**, and render the screen normally. This is a choice to reduce customer experience impact and increase system survival rate.

### Data Patterns and Architectural Decision Criteria

Cache control strategies are determined by data generation costs and consistency requirements.

#### Complex Operation Data High Recomputation Cost, Read-Heavy

-   Data like statistical rankings or main screen recommended product lists, which involve joining numerous tables or aggregations in the DB, taking 1-3 seconds or more to execute.
-   **Decision:** The appropriate decision is PER or asynchronous update.
-   **Reason:** If a distributed lock is applied to such data, threads for tens of thousands of requests that fail to acquire the lock would have to wait 1-3 seconds for the lock to be released, leading to immediate WAS pool exhaustion. Using PER allows quickly returning older data to the user within 1ms before the cache expires and leisurely executing heavy queries in the background to replace the cache.

#### Strictly Consistent Data Strict Consistency, Transactional

-   Remaining quantity of first-come, first-served coupons, payment/point balances, limited edition product stock.
-   **Decision:** Only distributed lock (e.g., Redlock).
-   **Reason:** In this pattern, showing users data that is 0.1 seconds old would lead to critical business failures like over-issuance or over-payment. Proactive updates or returning older data with PER are absolutely not allowed. It is standard practice to maintain a single source of truth (SSOT) through a lock and quickly fail requests that cannot wait, displaying a message that there are too many requests currently.

#### Predictable Traffic Spikes Predictable Spike

-   Time-limited special events opening at midnight every day or daily charts updated at a fixed time.
-   **Decision:** Pre-warming (pre-loading).
-   **Reason:** If it's 100% expected that traffic will surge simultaneously with expiration, rather than relying on PER or locks, a batch or worker process should query the DB at 11:50 PM, 10 minutes before the event opens, to pre-fill the cache without a TTL. This is a warming method where data is pushed in advance.

Thus, each type of data has different consistency requirements or traffic handling characteristics. Whether it's unique per user with many keys and frequent changes, or global data with infrequent changes, or global with many changes, these factors must be considered to make appropriate architectural decisions.

### Example

This is an object that stores the logical expiration time and the `delta` for the update operation along with the data body.

```kt
data class PerCachePayload<T>(
    val data: T,
    val logicalExpireAt: Long, // 논리적 만료 시간 (Epoch Milliseconds)
    val deltaMs: Long          // 데이터 연산(DB 조회 등)에 소요된 시간 (Milliseconds)
)
```

Let's develop a PER cache manager that queries the cache and triggers asynchronous updates based on probability.

```kt
import org.springframework.data.redis.core.RedisTemplate
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
import kotlin.math.ln
import kotlin.random.Random

class PerCacheManager(
    private val redisTemplate: RedisTemplate<String, Any>,
    // 백그라운드 비동기 갱신을 위한 전용 스레드 풀
    private val asyncExecutor: ExecutorService = Executors.newFixedThreadPool(10) 
) {
    // beta: 확률 가중치 (보통 1.0 사용, 높을수록 갱신이 더 일찍 발생)
    private val beta: Double = 1.0 

    fun <T> getWithPer(
        cacheKey: String,
        logicalTtlMs: Long,
        physicalTtlMs: Long,
        dbQueryBlock: () -> T // 캐시 미스 또는 갱신 시 실행할 DB 조회 로직
    ): T {
        val cachedValue = redisTemplate.opsForValue().get(cacheKey) as? PerCachePayload<T>

        val currentTime = System.currentTimeMillis()

        // 1. 캐시가 아예 없는 경우 (최초 접근 또는 물리적 만료)
        if (cachedValue == null) {
            return recomputeAndCache(cacheKey, logicalTtlMs, physicalTtlMs, dbQueryBlock).data
        }

        // 2. PER 공식 계산
        // currentTime >= logicalExpireAt - (deltaMs * beta * log(random))
        val randomValue = Random.nextDouble() // 0.0 ~ 1.0
        val earlyRecomputeThreshold = 
            cachedValue.logicalExpireAt - (cachedValue.deltaMs * beta * -ln(randomValue))

        if (currentTime >= earlyRecomputeThreshold) {
            // 3. 확률에 당첨되거나 이미 논리적 만료가 지난 경우: 비동기 갱신 트리거
            asyncExecutor.submit {
                try {
                    recomputeAndCache(cacheKey, logicalTtlMs, physicalTtlMs, dbQueryBlock)
                } catch (e: Exception) {
                    // 비동기 갱신 실패 로깅 (사용자 응답에는 영향을 주지 않음)
                }
            }
        }

        // 4. 기존 데이터(Stale Data 포함) 즉시 반환 -> 사용자 대기 없음(Zero Latency)
        return cachedValue.data
    }

    private fun <T> recomputeAndCache(
        cacheKey: String, 
        logicalTtlMs: Long, 
        physicalTtlMs: Long, 
        dbQueryBlock: () -> T
    ): PerCachePayload<T> {
        val startTime = System.currentTimeMillis()
        
        // 무거운 DB 쿼리 실행
        val freshData = dbQueryBlock() 
        val deltaMs = System.currentTimeMillis() - startTime

        val payload = PerCachePayload(
            data = freshData,
            logicalExpireAt = System.currentTimeMillis() + logicalTtlMs,
            deltaMs = deltaMs
        )

        // 물리적 TTL을 적용하여 Redis에 저장 (단위: 밀리초)
        redisTemplate.opsForValue().set(
            cacheKey, 
            payload, 
            java.time.Duration.ofMillis(physicalTtlMs)
        )

        return payload
    }
}
```

```kt
@Service
class EventService(
    private val perCacheManager: PerCacheManager,
    private val eventRepository: EventRepository
) {
    fun getEventData(eventId: String): EventData {
        return perCacheManager.getWithPer(
            cacheKey = "event:detail:$eventId",
            logicalTtlMs = 5 * 60 * 1000,  // 논리적 TTL: 5분
            physicalTtlMs = 15 * 60 * 1000, // 물리적 TTL: 15분 (논리적 만료 후 10분간 버퍼)
            dbQueryBlock = { 
                eventRepository.findComplexEventData(eventId) // 무거운 쿼리
            }
        )
    }
}
```

There's also a point to consider above: if multiple traffic streams come in, multiple threads might be "selected" for PER simultaneously in the asynchronous update part. To prevent this, duplicate updates where several background threads query the DB concurrently can occur.

To prevent this, a short-lived distributed lock can be applied within or before the asynchronous update block, optimizing it so that even background update tasks are performed by only one thread.

```kt
import org.springframework.data.redis.core.RedisTemplate
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
import java.time.Duration
import kotlin.math.ln
import kotlin.random.Random

class PerCacheManager(
    private val redisTemplate: RedisTemplate<String, Any>,
    private val asyncExecutor: ExecutorService = Executors.newFixedThreadPool(10) 
) {
    private val beta: Double = 1.0 

    fun <T> getWithPer(
        cacheKey: String,
        logicalTtlMs: Long,
        physicalTtlMs: Long,
        dbQueryBlock: () -> T
    ): T {
        val cachedValue = redisTemplate.opsForValue().get(cacheKey) as? PerCachePayload<T>
        val currentTime = System.currentTimeMillis()

        // 1. 캐시가 아예 없는 경우 (최초 접근) - 동기 방식으로 1명이 DB 조회
        if (cachedValue == null) {
            return recomputeAndCache(cacheKey, logicalTtlMs, physicalTtlMs, dbQueryBlock).data
        }

        // 2. PER 공식 계산
        val randomValue = Random.nextDouble()
        val earlyRecomputeThreshold = 
            cachedValue.logicalExpireAt - (cachedValue.deltaMs * beta * -ln(randomValue))

        // 3. 갱신 시점이 도래한 경우
        if (currentTime >= earlyRecomputeThreshold) {
            val lockKey = "lock:recompute:$cacheKey"
            
            // 핵심: SETNX를 활용한 비동기 갱신 락 획득 시도 (TTL 5초)
            // 이미 누군가 갱신 중이라면 false를 반환하여 중복 실행을 막습니다.
            val isLocked = redisTemplate.opsForValue()
                .setIfAbsent(lockKey, "LOCKED", Duration.ofSeconds(5)) ?: false

            if (isLocked) {
                asyncExecutor.submit {
                    try {
                        recomputeAndCache(cacheKey, logicalTtlMs, physicalTtlMs, dbQueryBlock)
                    } catch (e: Exception) {
                        // 에러 로깅
                    } finally {
                        // 갱신 완료 후 락 해제 (선택 사항)
                        // 삭제하지 않고 5초 TTL 만료를 기다리게 두면, 
                        // 갱신 직후 발생할 수 있는 찰나의 중복 트리거를 더 안전하게 막을 수 있습니다.
                        redisTemplate.delete(lockKey)
                    }
                }
            }
        }

        // 4. 기존 데이터 즉시 반환 (대기열 없음)
        return cachedValue.data
    }

    // recomputeAndCache 메서드는 이전과 동일합니다.
    // ...
}
```
