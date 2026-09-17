# Cache System Design and Implementation

When traffic rapidly increases to tens of thousands of API requests per second, it can overload service servers and lead to stability issues.

If a server cannot handle the traffic, performance degrades, response times increase, and outages can occur.

This results in a decline in service quality, with threads backing up or cascading failures.

Initially, if we change the structure from `server -> db` to `server -> remote cache -> db` (using Redis), Redis itself might crash due to CPU overload.

Memory usage can also increase. So, what's the next step when the remote cache becomes overloaded?

### Remote Cache & Local Cache

During peak times or when a large volume of traffic causes a surge in cached data, memory can become insufficient, or the CPU might process commands slowly (Redis is single-threaded). This can cause Redis itself to crash, leading to a cache stampede where traffic floods the database.

A cache stampede can, in turn, crash the database, creating a cascading failure.

Therefore, we could try to reduce the load on Redis by caching the data Redis caches in a server's local cache.

However, this also brings up many points to consider:

-   What data should be placed in the local cache?
-   How should synchronization be handled?

And so on. If we consider the types of data to cache, they can generally be divided into two categories:

Globally used data or unique values for a single user.

Let's call them static data and user data.

#### static data

For static data, an increase in traffic will lead to many `get` operations for a single key. Let's say it's something like "product:static".

Sending many read requests to Redis will cause Redis CPU overload, leading to command backlogs and delays.

This means delays will occur everywhere this static data is accessed.

For static data, even if the same value is locally cached on all servers, it's not a unique value per user.

So, it's fine if user requests are routed in a non-sticky manner. The primary goal is to reduce the load on a specific Redis hot key through local caching.

> I'll implement the code below.

#### user data

User data is data that needs to be cached per user. As the number of users increases, memory usage will naturally rise, and many keys will be generated, such as "user:1" ... "user:99999...".

A simple approach is to store the data compressed. This would consume some CPU for decompression but save memory, and since the server decompresses, it's a trade-off.

When storing DTO-like data in Redis, consider compressing it rather than just dumping JSON. (Whether to compress or not depends on the size).

If data processing fails due to overload, provide a fallback value or open a circuit to block requests.

Since cache stampede must be avoided at all costs, it's better to fail fast at the cache level and then normalize the system.

### Implementation

To summarize:

-   **static data:** Global values like system settings, announcements, or category lists. These can primarily be stored locally with a relatively long TTL. If these values change, they can be invalidated via pub/sub, and since all servers cache the same value, efficiency is maximized.
-   **user data:** User-specific values like user profiles, points, or personal settings. These are good for an auxiliary cache to distribute Redis load, and the TTL should be very short. Pub/sub invalidation is essential upon change, and it saves Redis I/O during frequent requests.

#### Redis Pub/Sub based Invalidation Design

All servers should subscribe to the same Redis topic.

When data is modified on a specific server, an invalidation message is published.

```
{
  "cacheType": "USER_CACHE or UNIVERSAL_CACHE",
  "key": "user:123",
  "originServer": "server-a-id"
}
```

If Server A modifies user information, it updates Redis.

Server A publishes a message to the Pub/Sub Topic indicating that `user:123` has been invalidated.

Servers B, C, and D receive the message and remove the corresponding entry from their respective Caffeine (local) caches.

Subsequently, any request for that user, regardless of which server it goes to, will find the L1 cache empty, fetch the latest data from Redis, and repopulate L1.

Cache Population

```java
@Configuration
@EnableCaching
public class CacheConfig {

    @Bean
    public CacheManager cacheManager() {
        SimpleCacheManager cacheManager = new SimpleCacheManager();
        
        // 1. Common Data Cache (Long TTL)
        CaffeineCache staticCache = new CaffeineCache("staticCache",
                Caffeine.newBuilder()
                        .expireAfterWrite(1, TimeUnit.HOURS)
                        .maximumSize(500)
                        .build());

        // 2. User Data Cache (Short TTL)
        CaffeineCache userCache = new CaffeineCache("userCache",
                Caffeine.newBuilder()
                        .expireAfterWrite(1, TimeUnit.MINUTES)
                        .maximumSize(10000)
                        .build());

        cacheManager.setCaches(Arrays.asList(staticCache, userCache));
        return cacheManager;
    }
}
```

```java
@Configuration
public class RedisConfig {

    @Bean
    RedisMessageListenerContainer container(RedisConnectionFactory connectionFactory,
                                            MessageListenerAdapter listenerAdapter) {
        RedisMessageListenerContainer container = new RedisMessageListenerContainer();
        container.setConnectionFactory(connectionFactory);
        container.addMessageListener(listenerAdapter, new PatternTopic("cache-invalidation"));
        return container;
    }

    @Bean
    MessageListenerAdapter listenerAdapter(CacheMessageSubscriber subscriber) {
        return new MessageListenerAdapter(subscriber, "onMessage");
    }
}

// Message object
@Data
@AllArgsConstructor
@NoArgsConstructor
public class CacheMessage implements Serializable {
    private String cacheName;
    private String key;
}
```

Let's define the Redis pub/sub subscriber.

```java
@Component
@RequiredArgsConstructor
public class CacheMessageSubscriber {
    private final CacheManager cacheManager;

    public void onMessage(String message) {
        // In reality, JSON deserialization logic is needed
        // ObjectMapper.readValue(message, CacheMessage.class)
        CacheMessage cacheMessage = deserialize(message); 
        
        Cache cache = cacheManager.getCache(cacheMessage.getCacheName());
        if (cache != null) {
            cache.evict(cacheMessage.getKey()); // Remove local cache
            System.out.println("L1 cache invalidated: " + cacheMessage.getKey());
        }
    }
}
```

```java
@Service
@RequiredArgsConstructor
public class UserService {
    private final RedisTemplate<String, Object> redisTemplate;
    private final CacheManager cacheManager;

    @Cacheable(cacheNames = "userCache", key = "#userId")
    public UserProfile getUserProfile(String userId) {
        // Executed on L1 Miss: Logic to fetch from Redis (L2) (omitted)
        return fetchFromRedis(userId);
    }

    public void updateUserProfile(String userId, UserProfile profile) {
        // 1. Update DB/Redis
        updateInRedis(userId, profile);

        // 2. Publish Pub/Sub message (invalidate L1 on all servers)
        redisTemplate.convertAndSend("cache-invalidation", new CacheMessage("userSpecificCache", userId));
    }
}
```

Additionally, to prevent cache stampede, for user-specific data, you can use `@Cacheable` with the `sync = true` option when querying Redis. This prevents multiple threads from accessing Redis simultaneously by applying a lock.

```java
@Cacheable(cacheNames = "userSpecificCache", key = "#userId", sync = true)
```

Furthermore, a server that publishes a message will also receive its own subscription message because the publisher is also a subscriber. This could lead to an infinite loop if not handled. Eviction doesn't significantly impact performance, so you could let it happen, or you could include the server ID in the message and implement logic to skip processing if it's the same server.

This is a trade-off between network cost and consistency. In systems with tens of thousands of data updates per second, the pub/sub messages themselves could become a network burden. In such cases, it might be more advantageous to abandon pub/sub and set a very short TTL for the L1 local cache, allowing it to expire naturally.

Using Caffeine's Weak Keys, if memory pressure is severe, enabling the `weakKeys` option can make the cache more aggressively cleaned up during garbage collection. This seems to create weak references.

<br>

For now, we've covered read caching to some extent, setting different configurations for global and user-specific caches while maintaining consistency through Redis pub/sub upon changes.

So, how about writes? If there are many write operations directly to the database, that itself can cause database overload.

First, let's look at cache write strategies.

-   **Write Through:** Data is written to both the cache and the database simultaneously. The advantage is guaranteed data consistency, but the disadvantage is increased write latency. It also puts a load on the database.
-   **Write-Back (Write-Behind):** Data is written to Redis first, and then accumulated for a certain period/count before being asynchronously written to the database. This reduces database load and improves write performance, but there's a risk of data loss if Redis fails.

Considering the constraint of not writing directly to the database, the write-back strategy seems more suitable.

Assuming a non-sticky environment, if a write occurs on any server, the L1 cache of other servers must be invalidated immediately.

1.  Request: User requests a data change.
2.  L2 Write: The server saves the latest data to Redis.
3.  Local Update: Updates its own local cache.
4.  Pub/Sub Message: Propagates a message via Redis pub/sub, like "Hey everyone, this key has changed, clear it!"
5.  Write-Behind: Registers the changed key information as a pending change in a Redis list or set.
6.  DB Write: A separate batch worker periodically reads data from Redis and performs bulk updates to the database.

Assuming user data, writes are relatively more frequent than for static data, so let's consider writing to Redis first and updating the database later.

For static data, writes are very infrequent, so the impact of data loss is significant. In this case, Write-Through might be acceptable. For now, that's the approach.

```java
@Service
@RequiredArgsConstructor
public class UserCacheService {
    private final RedisTemplate<String, Object> redisTemplate;
    private final CacheManager cacheManager;

    public void updateUserData(String userId, UserProfile profile) {
        // 1. Redis (L2) Update
        String redisKey = "user:" + userId;
        redisTemplate.opsForValue().set(redisKey, profile);

        // 2. Local (L1) Update (current server)
        cacheManager.getCache("userCache").put(userId, profile);

        // 3. Publish L1 invalidation message to other servers
        redisTemplate.convertAndSend("cache-invalidation", new CacheMessage("userCache", userId));

        // 4. Record change log for Write-Behind (store in Set to remove duplicates)
        redisTemplate.opsForSet().add("pending-db-updates", userId);
    }
}
```

A scheduler periodically persists the data written to Redis into the database.

```java
@Component
@RequiredArgsConstructor
public class WriteBehindWorker {
    private final RedisTemplate<String, Object> redisTemplate;
    private final UserRepository userRepository;

    @Scheduled(fixedDelay = 5000) // Update DB every 5 seconds
    public void persistToDb() {
        // 1. Get all target keys for update
        Set<Object> pendingUserIds = redisTemplate.opsForSet().members("pending-db-updates");
        if (pendingUserIds == null || pendingUserIds.isEmpty()) return;

        // 2. Multi-Get these data from Redis (network optimization)
        List<String> keys = pendingUserIds.stream().map(id -> "user:" + id).toList();
        List<Object> profiles = redisTemplate.opsForValue().multiGet(keys);

        // 3. DB Bulk Save (using JPA saveAll, etc.)
        // userRepository.saveAll(profiles...);
        System.out.println(profiles.size() + " items saved to DB in bulk.");

        // 4. Delete processed keys
        redisTemplate.delete("pending-db-updates");
    }
}
```

Ultimately, this approach involves writing to Redis first, with a batch process then persisting it to the database. The advantage is reduced load, but the downside is that changes are not immediately reflected in the database, and data can be lost if Redis fails. This feels like adjusting C/A in the CAP theorem, where one must consider and decide whether to gain performance or strong consistency.

But let's not stop here and think about the problem further. If we write to the cache, we don't acquire a separate DB lock. How can we resolve concurrency issues here?

If "thundering herd" (따닥) or duplicate requests lead to an operation being performed twice, consistency can be broken.

One approach is to use **distributed locks**. Since multiple servers are distributed, `synchronized` is useless. With Redis distributed locks, only one write operation is allowed for a specific key (userid) at a time.

Alternatively, ensure atomic operations using a **Lua script**. Bundle the process of writing data to Redis, sending an invalidation message, and adding to the write-behind queue into a single atomic unit.

Execute the Lua script within Redis to prevent other requests from interfering.

Applying idempotency keys can also prevent this. Clients can send a `request-id` with each request, and the server can record processed IDs in Redis for a short period, ignoring identical incoming requests.

```java
@Service
@RequiredArgsConstructor
public class ConcurrentUserCacheService {
    private final RedissonClient redissonClient;
    private final RedisTemplate<String, Object> redisTemplate;
    private final CacheManager cacheManager;

    public void updateUserDataWithLock(String requestId, String userId, UserProfile profile) {
        String lockKey = "lock:user:" + userId;
        RLock lock = redissonClient.getLock(lockKey);

        try {
            // 1. Attempt to acquire lock (max 5 seconds wait, hold for 10 seconds)
            if (lock.tryLock(5, 10, TimeUnit.SECONDS)) {
                try {
                    // 2. Idempotency check (prevent duplicate requests)
                    String idempotencyKey = "req:" + requestId;
                    Boolean isFirstRequest = redisTemplate.opsForValue()
                            .setIfAbsent(idempotencyKey, "processing", Duration.ofMinutes(5));
                    
                    if (Boolean.FALSE.equals(isFirstRequest)) {
                        return; // Request already processed, exit
                    }

                    // 3. Business logic and cache update (atomic execution recommended)
                    updateCacheAndPublish(userId, profile);
                    
                } finally {
                    lock.unlock(); // Release lock
                }
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }

    private void updateCacheAndPublish(String userId, UserProfile profile) {
        String redisKey = "user:" + userId;
        
        // Using Redis pipeline or Lua script is safer.
        redisTemplate.execute(new SessionCallback<Object>() {
            @Override
            public Object execute(RedisOperations operations) {
                operations.multi(); // Start transaction
                operations.opsForValue().set(redisKey, profile);
                operations.opsForSet().add("pending-db-updates", userId);
                operations.convertAndSend("cache-invalidation", 
                    new CacheMessage("userCache", userId));
                return operations.exec();
            }
        });

        // Immediately update the current server's L1 cache as well
        cacheManager.getCache("userCache").put(userId, profile);
    }
}
```

1.  If "thundering herd" requests come in, only the server that first acquires the Redis lock performs the logic. Requests arriving later are filtered out at the idempotency key check stage after acquiring the lock.
2.  If another server queries data while a write is in progress, it might read an older version from the local cache before the writing server sends the pub/sub message. If consistency is extremely critical, consider acquiring a lock even during reads or setting a short TTL to increase the probability of hitting L2.
3.  To prevent duplicate data from entering the write-behind queue, the code above uses `opsForSet().add()`, which automatically removes duplicates. Avoid using lists.
