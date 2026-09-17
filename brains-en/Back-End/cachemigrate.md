# Cache Zero-Downtime Migration (Lazy Migration)

Let's consider the following premise as an example.

Traffic averages tens of millions of requests per day, with peak times expected to increase by more than 10 times. The read-to-write ratio is 9:1, and it's an environment with multiple service instances in a microservice architecture.

Redis memory usage is around 80%, and the cache hit rate is maintained at about 70%. Let's assume a DB failure occurs if it drops below 50%.

Redis CPU usage has about 20% headroom.

Now, let's assume a backward-incompatible change occurs, such as a JSON data field modification.

```
{ "name": "khope" } -> { "username": "khope" }
```

Let's say a change like the above occurs. We will consider zero-downtime deployment for the cache system in this scenario.

If we immediately evict the cache, traffic will flood the DB due to a cache stampede, leading to a full outage. Therefore, we will not consider this approach.

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Server A  │     │   Server B  │     │   Server C  │
│ ┌─────────┐ │     │ ┌─────────┐ │     │ ┌─────────┐ │
│ │L1 Cache │ │     │ │L1 Cache │ │     │ │L1 Cache │ │
│ │(Caffeine)│ │     │ │(Caffeine)│ │     │ │(Caffeine)│ │
│ └────┬────┘ │     │ └────┬────┘ │     │ └────┬────┘ │
└──────┼──────┘     └──────┼──────┘     └──────┼──────┘
       │                   │                   │
       └───────────────────┼───────────────────┘
                           │
                    ┌──────▼──────┐
                    │  L2 Cache   │
                    │   (Redis)   │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │  Database   │
                    └─────────────┘
```

Currently, it's a dual-layer cache. We've considered factors like increased cumulative RTT latency, potential CPU bottlenecks on Redis nodes, and service impact during Redis failures.

By placing L1 in front, hot data can be responded to with zero network cost, Redis load can be distributed, and the system can endure even during a brief Redis outage using L1 data.

Of course, L1 cache inconsistency also needs consideration. This was covered in a previous post, "Cache System Design Concepts/Implementation," so let's just note that pub/sub-based cache invalidation was chosen.

+Additionally, cache data type-specific management points, write-behind, distributed locks, etc., were also mentioned there, so we'll skip them here.

### Zero-Downtime Schema Migration

How can we perform a zero-downtime migration when the JSON structure changes, as seen in the previous requirement?

```
// Old version (V1)
{"id": 1, "name": "홍길동", "email": "hong@test.com"}

// New version (V2)
{"id": 1, "username": "홍길동", "email": "hong@test.com"}
```

We could consider simply flushing the entire cache (which we won't do), or waiting for TTL expiration, but this migration period might be too long.

We could also change the version value of the key, but this would require more memory (at least double the existing value, or less but still more), which is difficult given the current premise of 80% memory usage.

So, how can we quickly and accurately migrate existing values while maintaining the hit rate?

We can apply two methods:

- **Lazy Migration**: A method that transforms data upon reading. (hit rate)
- **Background Migration**: Since Redis CPU has headroom, background conversion can run concurrently to increase conversion speed.

```java
public <T> T getWithMigration(String key, Class<T> targetType) {
    String json = redisTemplate.opsForValue().get(key);
    if (json == null) return null;

    // Check if it's V2 format
    if (json.contains("\"username\"")) {
        return objectMapper.readValue(json, targetType);
    }

    // Convert V1 → V2
    JsonNode node = objectMapper.readTree(json);
    if (node.has("name")) {
        ((ObjectNode) node).set("username", node.get("name"));
        ((ObjectNode) node).remove("name");

        // Save the converted data again (maintain TTL)
        Long ttl = redisTemplate.getExpire(key, TimeUnit.SECONDS);
        String newJson = objectMapper.writeValueAsString(node);
        redisTemplate.opsForValue().set(key, newJson, ttl, TimeUnit.SECONDS);
    }

    return objectMapper.readValue(node.toString(), targetType);
}
```

This is pseudocode. We could define a policy for determining whether V1 or V2 data has arrived, such as using exception catching or parsing validation, but

exceptions also incur cost, and we could use aliases for field names to make both compatible (though this only applies to existing field changes; new additions or deletions would require a different implementation, but anyway).

Ultimately, if it's V1 data, we use it as V2 and then re-set that cache. This way, we can solve the problem without reducing the hit rate.

+Since Redis CPU has headroom, we can proactively perform migration in the background to shorten the duration. For example, using a Lua script:

```lua
-- migrate_user.lua (Run on Redis server)
local key = KEYS[1]
local data = redis.call('GET', key)
if not data then return -1 end

local obj = cjson.decode(data)
if obj['username'] ~= nil then return 0 end  -- Already V2

if obj['name'] ~= nil then
    obj['username'] = obj['name']
    obj['name'] = nil

    local ttl = redis.call('TTL', key)
    local newData = cjson.encode(obj)
    if ttl > 0 then
        redis.call('SETEX', key, ttl, newData)
    else
        redis.call('SET', key, newData)
    end
    return 1  -- Migration successful
end
return 0
```

We could do it this way, or create a Lua script that only performs `GET` and `SET` operations, and handle the JSON parsing logic in a Java task server.

Since Redis is single-threaded and uses an event loop, putting parsing tasks there might be a bit burdensome.

When triggering such background tasks, let's periodically check CPU usage to avoid burdening the Redis CPU.

```java
@Scheduled(fixedDelay = 10000)
public void migrateInBackground() {
    double cpuUsage = getRedisCpuUsage();

    // Dynamically adjust batch size based on CPU usage
    int batchSize;
    if (cpuUsage > 70) batchSize = 0;       // Stop
    else if (cpuUsage > 60) batchSize = 50;  // Minimum
    else if (cpuUsage > 40) batchSize = 100; // Normal
    else if (cpuUsage > 20) batchSize = 300; // Ample
    else batchSize = 500;                    // Very ample

    if (batchSize > 0) {
        scanAndMigrate(batchSize);
    }
}
```

The reason for writing it in Lua is that it allows reading, transforming, and writing in a single network round trip, is processed on the Redis server, and is more efficient than `MULTI`/`EXEC`. (Here, "efficient" is understood as "simpler".)

Okay, we now have some idea of the implementation methods, and we also need to consider deployment. Clearly, multiple services will read and use these cache values.

Since there are N servers, let's assume that these cache-related logic changes are applied to a common library and then deployed.

First, before any DB schema changes, the logic compatible with both V1 and V2 must be deployed.

```
Phase 1: Deploy backward-compatible code (all servers)
         ↓
         State where both V1 and V2 can be read
         ↓
Phase 2: DB schema change
         ↓
Phase 3: New data is saved as V2
         ↓
Phase 4: Confirm background migration completion
         ↓
Phase 5: Remove V1 compatible code (technical debt cleanup)
```

I tend to think that such cache changes can be rolled out quickly, even with a canary deployment.

An additional concern is the coexistence of V1 and V2. If a new field is added instead of an existing one being changed, and a non-nullable V2 field is identified as null in V1, issues might arise.

For this, it's acceptable to prioritize data consistency over synchronization time until cache migration is complete, by performing a slow canary deployment.

Additionally, preventing V1 from overwriting V2 is necessary. For example, if an `age` key was added to the JSON example earlier:

All V1 servers must be modified to retain unknown fields and re-save them. For example, in Jackson, using `@JsonIgnoreProperties(ignoreUnknown=false)` can be combined with `JsonAnySetter` and `JsonAnyGetter`.

```java
// Structure to be reflected in V1 code in advance
public class UserDto {
    private String name;
    
    // A container to hold unknown fields (e.g., new fields in V2)
    private Map<String, Object> unknownFields = new HashMap<>();

    @JsonAnySetter
    public void setUnknownField(String key, Object value) {
        unknownFields.put(key, value);
    }

    @JsonAnyGetter
    public Map<String, Object> getUnknownFields() {
        return unknownFields;
    }
}
```

This way, when a V1 server reads and writes data, it can be developed to pass through newly added fields like `age` without modifying them.

Additionally, we could implement logic to check if a value is null at the time of writing, and handle it as a failure or a cache miss, perhaps by looking at a global cache instead of a local one.

To summarize:

1. Deploy the unknown field preservation logic, default value handling logic, coexistence logic, and lazy migration changes to all V1 servers.
2. Update the DB schema. While fields can be added as nullable and then made not-null flexibly, this might not be an option if the policy requires strict enforcement.
3. Deploy V2 servers as a canary. Now, even if V2 writes new fields, V1 servers will not delete them.
4. Once all data is migrated, remove the optional or coexistence logic from the DB application.

In summary, before deploying V2, a field preservation deployment must precede it to ensure V1 servers do not corrupt new data.

The code can be found here: https://github.com/rlaope/cache-labs
