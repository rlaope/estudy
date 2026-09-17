# Cache System Design

cc. https://github.com/rlaope/cache-labs includes practical code. This post covers conceptual organization and additional learning about accessibility.

> Content organized while designing a local cache + Redis dual-cache structure.

---

## Table of Contents

1. [What to Cache](#1-what-to-cache)
2. [How to Implement](#2-how-to-implement)
3. [How to Maintain Cache Consistency](#3-how-to-maintain-cache-consistency)
4. [Cache Design in a Distributed Environment](#4-cache-design-in-a-distributed-environment)

---

## 1. What to Cache

### 1.1 Data Suitable for Local Cache

Local cache stores data directly in the application's memory (e.g., Heap). It offers overwhelmingly fast speeds due to no network I/O, but data replication between servers is challenging.

- **Extremely frequently accessed 'Hot' data**: Configuration values or common code data that must be referenced with every request.
- **Static data with very low change frequency**: Policy settings, country codes, category lists, etc., that affect the system's business logic.
- **Data where server-to-server inconsistency is not critical**: UI layout information, etc., where a slight time lag (based on TTL) does not significantly impact user experience.
- **Data for preventing Redis Hotspots**: When requests for a specific key are too numerous, exceeding the CPU performance of a single Redis instance (Cache Stampede), a secondary local cache is used to mitigate this.

> However, Redisson documentation states that using a local cache can speed up read performance by up to 45 times. Of course, vendor documentation might be a bit exaggerated, but the impact of eliminating network I/O is palpable. There's also a saying, "A common misconception is that Redis is always the fastest caching option. In reality, in-process caching is faster than Redis," which suggests that blind faith in Redis should be avoided, as should blind faith in any technology.

### 1.2 Data Suitable for Redis Cache

Redis is a shared store in a distributed environment for multiple server instances. It is essential when data consistency must be maintained or when dealing with data too large to fit into local memory.

- **Shared State information**: Data that must remain consistent regardless of which server a user connects to, such as login sessions, user tokens, or shopping cart information.
- **Real-time counting and ranking**: Like counts or real-time popular search query rankings (utilizing Sorted Set) that require accurate aggregation in a distributed environment.
- **High-frequency query results for reducing DB load**: Query results that are expensive to retrieve but commonly used by many users, such as search results or detailed post information.
- **Distributed Locks and Rate Limiting**: Metric data used to protect resources accessed concurrently by multiple nodes or to limit API call frequency.

### 1.3 Data Not to Cache

Caching is an act of sacrificing 'consistency' for 'performance' or incurring 'additional costs'. In the following cases, the disadvantages of caching outweigh the benefits.

- **Financial/payment data where real-time consistency is critical**: Data like account balances or payment statuses, where even a 1ms inconsistency is unacceptable, should always directly reference the DB (Source of Truth).
- **One-time data with no reusability**: Data that is queried only once and never again (e.g., a one-time result for a specific search filter). This leads to low cache hit rates and wastes memory.
- **Security-sensitive personal information (unencrypted PII)**: Personal information that could be critical if exposed via memory dumps or Redis should be avoided from caching or strongly encrypted beforehand.
- **Data that changes frequently but has low read frequency**: This only creates overhead because the cache must be updated (invalidated) with every write operation, offering no read benefits.

> There's a famous saying, "There are only two hard things in Computer Science: cache invalidation and naming things." It's half-joke, half-truth; if you cache something that doesn't need caching, you'll truly experience invalidation hell. If the hit rate is low, it's probably better not to cache it at all.

### 1.4 Strategy for Handling Frequently Changing Data

When data changes frequently, the key challenges are 'when to invalidate the cache' and 'how to synchronize with the DB'.

**Write-Through / Write-Around / Write-Back:**
- **Write-Through**: Writes data to both the DB and the cache simultaneously. Offers good consistency but increases write latency.
- **Write-Back (Write-Behind)**: Writes data to the cache first, then asynchronously reflects it to the DB based on a schedule or event. Provides the best write performance but carries the risk of data loss in case of server failure.

> A CodeAhoy article summarizes it as: "Write-back gives max performance at the cost of consistency risk, while write-through gives strong consistency at cost of performance. Decide what the priority is." Ultimately, it's a trade-off, and some developers reportedly use Write-Back to Redis as a buffer during peak times. However, it's not recommended for critical data as it can lead to data loss if a failure occurs.

**Cache Invalidation vs Update:**
- When data changes, it's common to **delete (evict)** the existing cache. The update method carries a high risk of incorrect data remaining in the cache due to race conditions.

**TTL (Time To Live) Strategy:**
- For frequently changing data, use a shorter TTL, but to prevent a flood of requests to the DB when the cache expires (Cache Stampede), employ a technique of randomizing the expiration time (Jitter).

**Utilizing CDC (Change Data Capture):**
- By detecting DB transaction logs and asynchronously updating the cache via a message queue (e.g., Kafka), application logic and cache update logic are separated, enhancing scalability.

### cc

**Local Cache vs Redis Cache**
- [Redis + Local Cache: Implementation and Best Practices](https://medium.com/@max980203/redis-local-cache-implementation-and-best-practices-f63ddee2654a) - Practical Guide to Dual Cache Implementation
- [Redis Cache vs. In-Memory Cache: When to Use What](https://blog.nashtechglobal.com/redis-cache-vs-in-memory-cache-when-to-use-what/) - Criteria for Choosing Local/Distributed Cache
- [Distributed Caching - Redis.io](https://redis.io/glossary/distributed-caching/) - Official Redis Distributed Caching Documentation

**Cache Write Strategies (Write-Through / Write-Back / Write-Around)**
- [Caching Strategies and How to Choose the Right One](https://codeahoy.com/2017/08/11/caching-strategies-and-how-to-choose-the-right-one/) - Comparison and Analysis of Caching Strategies
- [Cache Invalidation Strategies - Design Gurus](https://www.designgurus.io/blog/cache-invalidation-strategies) - In-depth Guide to Cache Invalidation Strategies
- [Cache Invalidation and Methods - GeeksforGeeks](https://www.geeksforgeeks.org/system-design/cache-invalidation-and-the-methods-to-invalidate-cache/) - Cache Invalidation Methodologies

**Cache Stampede (Thundering Herd)**
- [Thundering Herd / Cache Stampede](https://distributed-computing-musings.com/2021/12/thundering-herd-cache-stampede/) - Problem Definition and Solutions
- [Cache Stampede & The Thundering Herd Problem](https://medium.com/@sonal.sadafal/cache-stampede-the-thundering-herd-problem-d31d579d93fd) - Real-world Cases and Response Patterns
- [Thundering Herd - Ehcache Documentation](https://www.ehcache.org/documentation/2.8/recipes/thunderingherd.html) - Solution using BlockingCache

---

## 2. How to Implement

### 2.1 L1 + L2 Dual Cache Structure (Multi-level Cache)

This strategy combines local cache (L1) and distributed cache (L2) to overcome the limitations of a single cache.

**Structure:**
- **L1 (Local Cache)**: Memory of each application server (e.g., Caffeine, Guava). Network cost approaches zero, providing extreme performance.
- **L2 (Remote Cache)**: A shared store like Redis. Multiple servers share data and maintain data consistency.

**Lookup Flow:**
`Application → Check L1 (terminate on hit) → Check L2 (copy to L1 and return on hit) → Check DB (copy to L1/L2 and return on hit)`

**Synchronization Issue (The Invalidation Problem):**
- The core challenge is how to invalidate the L1 data on each server when L2 data changes.
- **Solution**: A common approach is to use Redis Pub/Sub to broadcast an "L1 cache deletion" message to all servers when a specific key changes.

> Using an L1-L2 structure offers advantages in terms of availability, as L1 can sustain operations even if Redis fails. However, due to the possibility of Pub/Sub message loss, it's safer to keep the L1 TTL short.

### 2.2 Cache-Aside Pattern (Look-aside)

This is the most commonly used pattern, where the application directly manages the cache and caches data only when needed.

**Read Logic:**
1. Check if data exists in cache (Hit)
2. If not found (Miss), query from DB
3. Store the retrieved data in cache and return it

**Write Logic (Important):**
- **Update DB First, then Evict Cache**: Update the DB first, then delete the cache.
- **Why delete?** Updating the cache with a new value carries a high risk of data inconsistency between the DB and the cache if multiple requests interfere (Race Condition). Deleting, on the other hand, is safer because the next lookup will fetch the latest data from the DB.

**Characteristics:**
- Even if a cache failure occurs, the system can continue to serve requests via the DB (Graceful Degradation).
- However, initial requests might lead to concentrated DB load (Cold Start problem).

> There's a reason Cache-Aside is the most common choice. The service remains alive even if the cache goes down. Architectures where the cache stands in front of the DB, like Read-Through, mean cache failure equals service failure, which is a significant burden.

### 2.3 TTL (Time To Live) Strategy

This is a key strategy for balancing data freshness and system load.

**Fixed TTL:**
- Assigns a fixed duration, such as 1 minute or 1 hour, depending on the data's nature.

**Jitter (Randomized TTL):**
- **Problem**: If numerous cache keys expire at the same time, it can lead to a Cache Stampede, where a flood of requests hits the DB simultaneously.
- **Solution**: Add a random duration (Jitter) of about 5-10% to the configured TTL to distribute expiration times.

**Soft TTL vs Hard TTL:**
- **Hard TTL**: Data is immediately deleted upon expiration.
- **Soft TTL**: Data is proactively refreshed in the background when expiration is imminent, providing users with consistently hit data (Refresh-ahead).

> TTL configuration is a tug-of-war between "too long means stale data, too short means reduced hit rate." It's safer to start with a conservatively short TTL and gradually increase it while monitoring. Jitter is truly essential; I've seen databases crash due to simultaneous expirations in the morning after a midnight deployment when it wasn't used.

### 2.4 Cache Key Design

Cache keys serve as data identifiers and addresses for management. Clear and systematic naming conventions are essential.

**Using Hierarchical Structure:**
- Use `:` or `::` as delimiters to represent hierarchy.
- Format: `{service_name}:{domain}:{identifier}:{attribute}`
- Example: `user-service:user:1234:profile`

**Versioning:**
- If the data structure (DTO) changes, deserialization errors can occur due to old-version caches.
- Including a version in the key allows for safe deployment.
- Example: `v1:user:1234` → `v2:user:1234`

**Balance of Readability and Length:**
- Too long wastes memory, too short makes its purpose unclear.
- Use meaningful abbreviations, but design them to avoid duplication.

**Consideration of Value Size:**
- When designing keys, the size of the value to be stored under that key should also be considered.
- For Redis, excessively large values (tens of MBs) can cause performance degradation, so lists or sets should be appropriately partitioned.

> Poor key naming can cause significant trouble later. Especially without versioning, you'll face deserialization errors and rollbacks every time you change a DTO. It's good practice to start with `v1:` from the beginning. Also, when debugging with the `KEYS *` command in Redis, a well-defined hierarchical structure makes filtering easier, like `KEYS user-service:*`.

---

## 3. How to Maintain Cache Consistency

### 3.1 Cache Invalidation Strategy

This strategy dictates how to handle the cache when data changes.

**Delete vs Update:**
- **Update**: When the DB is modified, the cache value is also updated. However, if two requests occur simultaneously, there's a high risk of a Race Condition where the final values in the DB and cache differ.
- **Delete (Evict)**: When the DB is modified, the cache is completely deleted. This is much safer as the next lookup will fetch the latest data from the DB, and it's the recommended approach in practice.

**Transactional Messaging:**
- If a DB update succeeds but cache deletion fails, consistency is broken.
- To prevent this, after a successful DB transaction, an event is published to a message queue (Kafka, RabbitMQ) to ensure the cache is reliably deleted.

**Double Deletion:**
- To prevent momentary consistency errors that can occur in a distributed environment, this technique involves deleting the cache once just before a DB modification, and then again after the modification is complete, with a slight delay.

> As an example of why the Update method is risky: if request A modifies a value to 10 and request B modifies it to 20 almost simultaneously, the DB might commit in A→B order, resulting in a final value of 20, but the cache might reflect B→A, resulting in a final value of 10. The Delete method avoids this concern.

### 3.2 Write-Through vs Write-Behind (Write-Back)

This addresses how to synchronize the cache and DB when a write request comes in.

| Pattern | Operation Method | Advantages | Disadvantages |
|---|---|---|---|
| **Write-Through** | Writes data to both DB and cache simultaneously | Cache always stays up-to-date (strong consistency) | Increased write latency |
| **Write-Behind** | Writes to cache first, then asynchronously reflects to DB later | Very fast write performance (advantageous for bulk writes) | Risk of data loss in case of cache failure |

> Write-Behind is a pattern often used in game servers. User behavior logs, for instance, don't need to be written to the DB in real-time. However, for things like payments, Write-Behind should never be used, as server failure would mean losing payment data.

### 3.3 Preventing Cache Stampede

In high-traffic environments, it's crucial to prevent a Cache Stampede, where a flood of requests hits the DB simultaneously when a specific cache key expires, potentially paralyzing the server.

**Jitter (Random TTL):**
- Distribute expiration times by setting slightly different expiration times for all caches (e.g., 10 minutes + 0-30 seconds random).

**Distributed Lock (Redis Lock):**
- When a cache expires, a lock is applied to ensure that only one request can access the DB to refresh the cache.
- Other requests either wait briefly or receive the previous cached data.

**PER (Probabilistic Early Recomputation):**
- This algorithm eliminates the 'moment of expiration' itself by probabilistically refreshing the cache in advance before its expiration time.

> Once a stampede occurs, the DB can crash instantly. It's especially risky when hot keys, like popular product detail pages, expire. The Lock method is reliable but introduces waiting times, and PER is a bit complex to implement. Applying Jitter first and then monitoring is the most practical approach.

### 3.4 Synchronization Between Local Cache and Redis

When using both L1 (Local) and L2 (Redis), if data is modified on one server, the L1 cache on other servers becomes 'stale data'.

**Pub/Sub-based Invalidation:**
1. Data modification occurs on one server → DB update
2. That server publishes a "Delete Key-X" message to a specific Redis channel (e.g., `cache-invalidation-topic`)
3. All application servers subscribed to the same channel immediately delete the corresponding key from their local caches (L1) upon receiving the message

**Short L1 TTL Configuration:**
- To account for potential Pub/Sub message loss, the L1 cache's TTL is set much shorter than L2's (e.g., 10-30 seconds) to naturally complement synchronization.

**Utilizing Redis Streams:**
- Since Pub/Sub does not guarantee message delivery, if higher reliability is needed, invalidation events can be delivered via Redis Streams or Kafka.

> A critical drawback of Pub/Sub is its "fire and forget" nature. If a subscriber disconnects and then reconnects, all messages sent during that period are lost. Therefore, a short L1 TTL is essential. If perfect synchronization is required, a message broker like Kafka should be used, but this introduces a trade-off of increased architectural complexity.

---

## 4. Cache Design in a Distributed Environment

### 4.1 Limitations of Sticky Session

Sticky Session is a method of always sending a specific user's requests to the same server.

**Load Imbalance:**
- If 'heavy users' flock to a particular server, it can lead to a hotspot phenomenon where only that node becomes overloaded.

**Scalability Issue:**
- Session redistribution is difficult when scaling servers up or down.
- If one server goes down due to a failure, all session data assigned to that server is lost.

**Lack of Flexibility:**
- The load balancer must manage session state, increasing infrastructure complexity and contradicting the stateless design philosophy.

> Although Sticky Session might seem simple, it ultimately makes servers stateful, hindering scalability. In today's cloud environments, statelessness is fundamental for using autoscaling, which Sticky Session prevents.

### 4.2 Problems with Round-Robin / Random Routing

This addresses issues in a stateless routing environment where requests are delivered to different servers each time.

**Reduced Local Cache Hit Ratio:**
- Even if user A's data is stored in Server 1's local cache, if the next request goes to Server 2, a cache miss will occur.

**Data Inconsistency:**
- Even if Server 1 updates data and refreshes its local cache, Server 2's local cache might still contain old data, risking users seeing different data each time.

> Using only local cache with Round-Robin halves the caching effect. That's why an L1+L2 dual-cache structure is necessary. L2 (Redis) ensures consistency, while L1 is used solely for performance boosting.

### 4.3 Distributed Invalidation Strategy

This is a key strategy for maintaining identical cache states across all distributed servers.

**Centralized Invalidation:**
- When data changes, delete the key in a shared store like Redis (L2), and ensure each server recognizes this.

**Event-based Invalidation:**
- Detect DB changes (CDC) or publish change events at the application level to send an "expiration" signal to all nodes caching that data.

**Accepting Eventual Consistency:**
- Real-time synchronization of all nodes incurs very high performance costs.
- If permissible by business logic, design with a short TTL so that consistency is naturally achieved over time.

> Strong Consistency vs Eventual Consistency is an eternal trade-off. Real-time synchronization would require distributed locks on every request, which would cripple performance. Since most services can tolerate a few seconds of inconsistency, the combination of Eventual Consistency + short TTL is practical.

### 4.4 Cache Synchronization Using Pub/Sub

This is a practical method for ensuring consistency among L1 (local) caches using Redis's Pub/Sub feature.

**Operating Principle:**
1. Data modification occurs on one server → DB update
2. That server publishes a "Delete Key-X" message to a specific Redis channel (e.g., `cache-invalidation-topic`)
3. All application servers subscribed to the same channel immediately delete the corresponding key from their local caches (L1) upon receiving the message

**Caveats:**
- Redis Pub/Sub operates on a 'Fire and Forget' basis and does not guarantee 100% message delivery.
- To account for message loss due to network failures, it's essential to concurrently set a short local TTL.

> Implementing Pub/Sub itself isn't difficult. However, in operation, message loss scenarios inevitably occur. Messages might be lost during server restarts or network interruptions. Therefore, one shouldn't rely solely on Pub/Sub; it's safer to keep the L1 TTL short, around 10-30 seconds.

### 4.5 Consistent Hashing

This algorithm prevents massive cache misses that occur when scaling cache nodes up or down.

#### Problems with Traditional Hashing

Using the `hash(key) % N` (number of nodes) method, if even one node changes, the mapping location of almost all keys changes, leading to a cache miss storm.

```
예시: 노드 3대 → 4대로 증설 시

hash("user:1") = 10 → 10 % 3 = 1 (Node-1) → 10 % 4 = 2 (Node-2) ❌ reassigned
hash("user:2") = 15 → 15 % 3 = 0 (Node-0) → 15 % 4 = 3 (Node-3) ❌ reassigned
hash("user:3") = 12 → 12 % 3 = 0 (Node-0) → 12 % 4 = 0 (Node-0) ✅ maintained

→ Most keys move to different nodes, causing a cache miss explosion
```

#### How the Hash Ring Works

**1. Ring Configuration:**
- Imagine the output range of a hash function (e.g., 0 to 2^32-1) connected in a circle.
- 0 and 2^32-1 are connected, forming a circular structure like a clock.

**2. Node Placement:**
- Each cache node is passed through a hash function and placed at a specific position on the ring.
- Example: `hash("Node-A") = 1000`, `hash("Node-B") = 5000`, `hash("Node-C") = 9000`

**3. Key Mapping:**
- Data keys also determine their position on the ring using the same hash function.
- Data is stored on the **first node encountered clockwise** from that position.

```
Hash Ring Visualization (assuming range 0 ~ 10000):

        0/10000
           |
    9000 ──┼── 1000
   (Node-C) │  (Node-A)
           │
    7000 ──┼── 3000
           │
        5000
      (Node-B)

- key "user:1" → hash = 2000 → clockwise → Node-B (5000)
- key "user:2" → hash = 6000 → clockwise → Node-C (9000)
- key "user:3" → hash = 9500 → clockwise → Node-A (1000, wraps around 0)
```

#### Behavior on Node Addition/Deletion

**Node Addition (Node-D at 7000):**
```
Before: key(6000) → Node-C(9000)
After:  key(6000) → Node-D(7000)  ← Only this key reassigned

→ Only keys between 5000 and 7000 move to Node-D
→ Other keys are unaffected
```

**Node Deletion (Node-B removed):**
```
Before: key(2000) → Node-B(5000)
After:  key(2000) → Node-C(9000)  ← Only this key reassigned

→ Only keys between 1000 and 5000 move to Node-C
→ Other keys are unaffected
```

> In the traditional method, if one node changes, ~100% of all keys are reassigned. With Consistent Hashing, on average, only K/N (total number of keys / number of nodes) are reassigned. If 1 out of 10 nodes fails, only 10% are affected.

#### Need for Virtual Nodes

Using only physical nodes can lead to uneven distribution on the ring.

**Problem Scenario:**
```
Node-A: 1000
Node-B: 1500  ← A and B are too close
Node-C: 9000

→ Node-A is responsible only for 1000~1500 (range of 500)
→ Node-C is responsible for 1500~9000 (range of 7500)
→ Severe load imbalance
```

**Solution - Virtual Nodes:**
- Place multiple virtual nodes on the ring for each physical node.
- Example: Node-A → Node-A#1, Node-A#2, Node-A#3 (each with a different hash value)

```
3 physical nodes, 3 virtual nodes each = total 9 points

Node-A#1: 1000    Node-B#1: 2000    Node-C#1: 3000
Node-A#2: 4500    Node-B#2: 6000    Node-C#2: 7500
Node-A#3: 8000    Node-B#3: 9500    Node-C#3: 500

→ Evenly distributed across the ring, balancing the load
```

> In practice, 100-200 virtual nodes per physical node are sometimes used. While a higher number leads to more even distribution, there's a trade-off of increased memory and search costs.

#### Java Implementation Example

```java
import java.util.*;
import java.util.concurrent.ConcurrentSkipListMap;
import java.security.MessageDigest;
import java.nio.charset.StandardCharsets;

public class ConsistentHashRing<T> {

    // Thread-safe version of TreeMap, maintains sorted state
    private final ConcurrentSkipListMap<Long, T> ring = new ConcurrentSkipListMap<>();
    private final int numberOfVirtualNodes;
    private final Set<T> physicalNodes = new HashSet<>();

    public ConsistentHashRing(int numberOfVirtualNodes) {
        this.numberOfVirtualNodes = numberOfVirtualNodes;
    }

    /**
     * Add physical node - place virtual nodes on the ring
     */
    public void addNode(T node) {
        physicalNodes.add(node);
        for (int i = 0; i < numberOfVirtualNodes; i++) {
            // Virtual node names: "NodeA#0", "NodeA#1", ...
            long hash = hash(node.toString() + "#" + i);
            ring.put(hash, node);
        }
    }

    /**
     * Remove physical node - remove corresponding virtual nodes from the ring
     */
    public void removeNode(T node) {
        physicalNodes.remove(node);
        for (int i = 0; i < numberOfVirtualNodes; i++) {
            long hash = hash(node.toString() + "#" + i);
            ring.remove(hash);
        }
    }

    /**
     * Find node corresponding to key - closest node clockwise
     */
    public T getNode(String key) {
        if (ring.isEmpty()) {
            return null;
        }

        long hash = hash(key);

        // Finds the first key greater than or equal to hash (clockwise search)
        Map.Entry<Long, T> entry = ring.ceilingEntry(hash);

        // If not found, wraps around to the beginning of the ring (circular structure)
        if (entry == null) {
            entry = ring.firstEntry();
        }

        return entry.getValue();
    }

    /**
     * MD5 hash function - used for even distribution
     */
    private long hash(String key) {
        try {
            MessageDigest md = MessageDigest.getInstance("MD5");
            byte[] digest = md.digest(key.getBytes(StandardCharsets.UTF_8));
            // Convert the first 8 bytes to long
            return ((long) (digest[0] & 0xFF) << 56)
                 | ((long) (digest[1] & 0xFF) << 48)
                 | ((long) (digest[2] & 0xFF) << 40)
                 | ((long) (digest[3] & 0xFF) << 32)
                 | ((long) (digest[4] & 0xFF) << 24)
                 | ((long) (digest[5] & 0xFF) << 16)
                 | ((long) (digest[6] & 0xFF) << 8)
                 | ((long) (digest[7] & 0xFF));
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public int getRingSize() {
        return ring.size();
    }

    public Set<T> getPhysicalNodes() {
        return Collections.unmodifiableSet(physicalNodes);
    }
}
```

**Usage Example:**

```java
public class ConsistentHashExample {
    public static void main(String[] args) {
        // Create hash ring with 150 virtual nodes
        ConsistentHashRing<String> hashRing = new ConsistentHashRing<>(150);

        // Add cache server nodes
        hashRing.addNode("cache-server-1");
        hashRing.addNode("cache-server-2");
        hashRing.addNode("cache-server-3");

        // Check which node a key maps to
        String[] keys = {"user:1001", "user:1002", "product:5001", "session:abc123"};

        System.out.println("=== Initial State (3 nodes) ===");
        Map<String, String> initialMapping = new HashMap<>();
        for (String key : keys) {
            String node = hashRing.getNode(key);
            initialMapping.put(key, node);
            System.out.println(key + " → " + node);
        }

        // Add 1 server
        System.out.println("\n=== After adding cache-server-4 ===");
        hashRing.addNode("cache-server-4");

        int movedCount = 0;
        for (String key : keys) {
            String newNode = hashRing.getNode(key);
            String moved = newNode.equals(initialMapping.get(key)) ? "" : " ← MOVED";
            if (!moved.isEmpty()) movedCount++;
            System.out.println(key + " → " + newNode + moved);
        }
        System.out.println("Moved keys: " + movedCount + "/" + keys.length);

        // Remove 1 server
        System.out.println("\n=== After removing cache-server-2 ===");
        hashRing.removeNode("cache-server-2");

        for (String key : keys) {
            String node = hashRing.getNode(key);
            System.out.println(key + " → " + node);
        }
    }
}
```

**Example Output:**

```
=== Initial State (3 nodes) ===
user:1001 → cache-server-2
user:1002 → cache-server-1
product:5001 → cache-server-3
session:abc123 → cache-server-1

=== After adding cache-server-4 ===
user:1001 → cache-server-2
user:1002 → cache-server-4 ← MOVED
product:5001 → cache-server-3
session:abc123 → cache-server-1
Moved keys: 1/4

=== After removing cache-server-2 ===
user:1001 → cache-server-4
user:1002 → cache-server-4
product:5001 → cache-server-3
session:abc123 → cache-server-1
```

> The code above is a basic implementation for learning purposes. In actual production, it's common to use libraries that implement the `ketama` algorithm (e.g., SpyMemcached, Jedis's ShardedJedis) or methods like Redis Cluster's Hash Slot. Implementing it yourself involves many considerations such as hash function selection, collision handling, and thread-safety, so using a proven library is safer.
