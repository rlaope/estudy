# Redis Serialization Sync

Let's say we store a JSON object as a value in Redis.
For example, `userUid: 1 { "user": "khope" }`.
But what if the `user` field were to change to `username`? How could we handle this?

### Duel Mapping

Should we modify the logic in the code to accommodate both the old and new fields? This would incur no database load.

read: Read the JSON from Redis and use the `username` field if it exists. If `username` is null and only `user` exists, should we treat the `user` value as `username`?

write: Moving forward, should new data always be stored with `username` in Redis?

```java
String json = redis.get(key);
UserDto user = objectMapper.readValue(json, UserDto.class);

if (user.getUsername() == null && user.getUser() != null) {
  user.setUsername(user.getUser()); // Assign old field value to new field?
}
```

### Lua

Or, should we try an in-place update with a Lua script?

This method directly modifies data within Redis, reducing network costs associated with fetching data to the application and sending it back, and also ensures atomicity.

We could iterate through all keys in Redis using the `EVAL` command to change the key name inside the JSON... but is `EVAL` okay given Redis is single-threaded?

```lua
-- Find keys matching a specific pattern in Redis and change the 'user' field to 'username'
local keys = redis.call('KEYS', 'user_cache:*') -- SCAN is recommended in practice
for i, key in ipairs(keys) do
    local val = redis.call('GET', key)
    if val then
        local updated = string.gsub(val, '"user":', '"username":')
        redis.call('SET', key, updated, 'KEEPTTL') -- Preserve existing TTL
    end
end
```

Instead of changing all keys, it might be better to run a script multiple times to safely change one key at a time. `KEYS` is inherently risky.

```lua
-- Script to process only one specific key
local val = redis.call('GET', KEYS[1])
if val then
    local updated = string.gsub(val, '"user":', '"username":')
    redis.call('SET', KEYS[1], updated, 'KEEPTTL')
end
```

---

How about utilizing a background migration tool?

A batch script that reads Redis keys with `SCAN`, transforms the data, and then `SET`s it back... it's simple, but effective.

<br>

### Are EVAL, KEYS, SCAN okay?

Because Redis operates on a single thread, `EVAL` and `KEYS` processing too many keys at once can block Redis, causing all requests to wait until the operation completes.

Therefore, they should absolutely not be used for bulk processing in a production environment.

`SCAN` is fine. While `KEYS` fetches all keys, `SCAN` is cursor-based, fetching a limited number (default 10) and returning a cursor for the next iteration. It's non-blocking, occupying Redis for only a very short time, so it's okay.

### Safe Synchronization Method

How to change field names without burdening single-threaded Redis:

**Gradual migration from the application?**
- This involves writing a batch application that processes data incrementally from outside Redis, rather than running internal logic within Redis, for example, updating 100 items at a time. It would read the data for these keys using `MGET` (reads multiple).
- Given the requirements, field name changes might not occur very frequently, but perhaps they do? If it's an external integration system, prior notice is usually given. Should we handle it with a batch job?
- Implement throttling by adding a `thread sleep` after each cycle of 100 items is processed to reduce CPU utilization.
