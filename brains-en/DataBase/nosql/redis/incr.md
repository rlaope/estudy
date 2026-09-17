# Redis INCR

The Redis INCR command is an atomic operation.

Therefore, even when multiple sessions perform update operations on the same data,

each command is processed sequentially, ensuring accurate updates.

For example, if two sessions try to increment a piece of data, and both read the value as 1 simultaneously,

both session 1 and session 2 would update 1 -> 2, failing to achieve the expected value of 3.

That's why we can use INCR to handle such increment/decrement operations atomically.

Since the INCR command is an atomic operation, even if multiple requests come in, it will increment the value by one for each request. This can resolve race conditions.

### Points to Note

For example, let's assume a self-study application service stores and manages self-study application information using Redis `GETSET`, and increments the data by 1 each time a self-study application is made.

It would usually proceed in this manner:

1.  if(check max self-study capacity && check user's self-study availability status)
2.  If validation is complete, switch user to self-study status, increment current self-study applicant count +1

However, what if step 1 is not handled atomically? If multiple sessions are open and `GET SET` operations come in simultaneously, a problem will arise where the maximum capacity is exceeded. A race condition occurs.

Redis is fundamentally single-threaded, but it receives input concurrently through I/O multiplexing. Therefore, while operations within each session are handled atomically, atomicity is not guaranteed when multiple sessions are open.

This is also mentioned in the official Redis documentation.

Therefore, to resolve this, operations like `GET` and `INCR` can be grouped and executed atomically by applying a transaction and a Lock mechanism, or by using Lua Script with the `EVAL` command to ensure atomicity at the Redis level.

```lua
EVAL "
	local i = redis.call('get', 'i');
	i = i + 1;
	redis.call('set', 'i', i);
"
```

```
MULTI
... commands to combine
EXEC
```

```plaintext
❗️ Distributed locks should only be used when absolutely necessary.

Recently, many job seekers' portfolios include distributed locks.
It's good to have used it for learning purposes, but from a problem-solving perspective,
it will be difficult to advocate for distributed locks.

Applying a lock creates a bottleneck.

While it can easily solve concurrency issues,
if there's a way to avoid using locks, it's better not to.

Personally, I see it as a trade-off between difficulty and performance.

Source - F-lab Ensuring atomic Redis operations for large-scale processing
```

[This article](https://hyperconnect.github.io/2019/11/15/redis-distributed-lock-1.html) provides an example using RedissonLock. It states that combining multiple commands atomically can prevent bugs and improve performance.

<br>

### INCR Code Analysis

```c
void incrDecrCommand(client *c, long long incr) {
    long long value, oldvalue;
    robj *o, *new;
    dictEntry *de;
    o = lookupKeyWriteWithDictEntry(c->db,c->argv[1],&de);
    if (checkType(c,o,OBJ_STRING)) return;
    if (getLongLongFromObjectOrReply(c,o,&value,NULL) != C_OK) return;

    oldvalue = value;
    if ((incr < 0 && oldvalue < 0 && incr < (LLONG_MIN-oldvalue)) ||
        (incr > 0 && oldvalue > 0 && incr > (LLONG_MAX-oldvalue))) {
        addReplyError(c,"increment or decrement would overflow");
        return;
    }
    value += incr;

    if (o && o->refcount == 1 && o->encoding == OBJ_ENCODING_INT &&
        (value < 0 || value >= OBJ_SHARED_INTEGERS) &&
        value >= LONG_MIN && value <= LONG_MAX)
    {
        new = o;
        o->ptr = (void*)((long)value);
    } else {
        new = createStringObjectFromLongLongForValue(value);
        if (o) {
            dbReplaceValueWithDictEntry(c->db,c->argv[1],new,de);
        } else {
            dbAdd(c->db,c->argv[1],new);
        }
    }
    signalModifiedKey(c,c->db,c->argv[1]);
    notifyKeyspaceEvent(NOTIFY_STRING,"incrby",c->argv[1],c->db->id);
    server.dirty++;
    addReplyLongLongFromStr(c,new);
}

void incrCommand(client *c) {
    incrDecrCommand(c,1);
}

void decrCommand(client *c) {
    incrDecrCommand(c,-1);
}

```

Looking at the [Redis GitHub INCR](https://github.com/redis/redis/blob/unstable/src/t_string.c#L610) code, it's structured as follows. Let's analyze it:
1.  It checks if the input key exists. If not, it creates a new key and sets its value to 0.
2.  It converts the value of the input key to an integer. If conversion fails, it returns an error. If it's an integer, it performs the increment or decrement operation.
3.  It checks whether the converted and incremented value has overflowed.
4.  Once the operation is complete, it saves the data, returns the value to the client, and finishes.

<br>

### INCR in a Cluster Environment

Redis is often operated in a cluster environment for various reasons such as **availability, scalability, and high-volume processing**.

If Redis is operated as a cluster with two or more masters, how would concurrency issues for commands like `INCR` be handled?

First, Redis distributes keys across multiple master nodes based on hash slots. A cluster has a total of 16384 hash slots, and each master node is responsible for a portion of these slots.

So, each node typically handles about 5000 slots.

When any key is stored in the Redis cluster, it is mapped to a specific hash slot. Using a hash function, it is then routed only to the master node responsible for that slot. This means a specific key is always stored on and processed by the same master node, and other master nodes do not access that data.

In Redis, if a client connects to a slot that does not contain the data it's looking for, it is redirected via `MOVE` to the correct node. (The reason nodes can confirm whether they hold the correct hash slots in a cluster is due to the Raft distributed consensus algorithm, which allows nodes to share and recognize configuration information in real-time.)

Therefore, as they are not looking at the same data structure, `INCR` operates atomically.

**Of course, in an active-active mirroring environment, the situation could be different. Carefully mirroring data would be a crucial point.**
