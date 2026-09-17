# Redis Distributed Lock, RedLock


### Using Redis Distributed Locks

https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb8XI8O%2FbtszM1YHVMa%2FMbKUTWapMhO8K7YkAg8cmK%2Fimg.png

A distributed lock is **a technique that ensures atomicity to prevent data corruption when multiple processes access a shared resource during a race condition.**

A typical storage solution to use is Redis, a single-threaded NoSQL database.

Multiple servers look at a common Redis instance to check if they can access a critical section, thereby ensuring atomicity.

> Since Redis is also single-threaded, it can be a single point of failure. Therefore, additional slave resources for failover should be set up.
>

## Trade-off

We will implement a lock using a Redis Client. Redisson and Lettuce are two Redis Clients that can be considered.

Both commonly attempt to acquire a lock using spinlocks, and to minimize the excessive load, which is a disadvantage of spinlocks, they send a moderate number of requests at appropriate intervals.

This is a technique where the server notifies subscribed clients that they can use the lock, eliminating the need for clients to repeatedly request and check for lock acquisition.

> Spinlock? A method that continuously attempts to acquire a lock in an infinite loop.
>

So, which feature should we use, Lettuce or Redisson?

### Lettuce

Lettuce is a Netty-based Redis client that boasts high performance by processing requests non-blockingly.

If `spring-data-redis` is added, a Redis client is provided by default, and this is Lettuce-based.

- Advantage: It's provided by default when adding the Redis dependency, allowing for simple implementation without separate configuration.
- Disadvantage: Its implementation uses spinlocks, which can put a load on Redis.

### Redisson

Redisson provides pub/sub functionality.

Using this, distributed locks can be implemented without using spinlocks.

While it can be implemented directly, Redisson already provides an implementation.

```groovy
implementation 'org.redisson:redisson-spring-boot-starter:3.24.3'

```

It can be used by adding the following dependency.

Therefore, I will use Redisson, which can reduce the load on Redis.

> Something to consider later -> It might be good to create a library that applies locks using an annotation-based approach via AOP, even though client classes are used within the service.
>

### RedLock

Unlike Redisson and Lettuce's locking mechanisms, there is also the RedLock approach.

RedLock assumes that if there are N Redis servers, **a lock is considered acquired if a majority of nodes (more than N/2) have acquired the lock.**

RedLock was modeled with the following three properties:

1. Mutual exclusion: Only one worker can acquire the lock at any given time.
2. Deadlock-free: Even if a lock cannot be released due to some issue after acquisition and the process terminates, another worker should be able to acquire the lock.
3. Fault tolerance: As long as a Redis node is operational, all workers should be able to acquire and release the lock.

[distributed-locks](https://redis.io/docs/manual/patterns/distributed-locks/)

The RedLock algorithm proceeds as follows:

1. Get the current time in milliseconds.
2. Send lock acquisition requests sequentially to N servers. At this time, the timeout should be set much smaller than the lock's validity period. For example, if the lock's validity period is 10 seconds, the timeout would be 5-50ms. The reason for using such a short timeout is to prevent excessive communication time with a failed Redis node.
3. If there are 7 Redis servers, **the lock is considered acquired if the time taken for a majority of Redis instances (4 or more) to acquire the lock is less than the validity period.** For example, if the validity period is 10 seconds and it takes 7 seconds to acquire the lock, it's a success; if it takes 11 seconds, it's a failure.
4. The validity period after acquiring the lock is `initial validity period - time taken to acquire the lock`. If the validity period is 10 seconds and it took 3 seconds to acquire the lock, the lock will expire after 7 seconds.
5. If the lock is not acquired (i.e., a majority did not acquire it), a request is sent to all Redis servers to release the lock.

### Limitations of RedLock?

```java
// THIS CODE IS BROKEN
public void writeData(String filename, String data) {
    Lock lock = lockService.acquireLock(filename);
    if (lock == null) {
        throw new RuntimeException("Failed to acquire lock");
    }

    try {
        File file = storage.readFile(filename);
        String updated = updateContents(file, data);
        storage.writeFile(filename, updated);
    } finally {
        lock.release();
    }
}

```

The code above contains logic for writing a file using the RedLock method.

There's an issue here, and the diagram visualizing it is shown below.

!https://miro.medium.com/v2/resize:fit:1400/format:webp/1*wwwIk_UkqKwRJ5ED8Wag4w.png

Client 1 acquired the lock, but then a GC event caused a Stop-the-World (STW) pause, during which the application was suspended and the lock expired.

After that, Client 2 acquired the distributed lock and wrote data to the file.

When Client 1 finishes its STW pause and attempts to write data, a concurrency issue arises.

[Protecting a resource with a lock](https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html)

> While GC typically performs very quickly, Stop-the-World GC can, in rare cases, last long enough for a lock to expire. Martin Kleppmann's document shows that RedLock can break not only due to GC but also due to network delays or timing issues.
>

A solution to the above problem is briefly mentioned in the "Making the lock safe with fencing" section of the link above. It introduces the concept of a fencing token, a simple number that increments each time a lock is acquired. The storage server checks the value of this token and determines whether to allow writing. However, even this method does not guarantee that a consistent fencing token will be generated every time a lock is acquired.

Furthermore, there are additional points to discuss regarding these methods, and since the content is extensive, I will briefly summarize them:

1. The most practical system model for algorithms like RedLock is an [asynchronous model with unreliable failure detectors](https://courses.csail.mit.edu/6.852/08/papers/CT96-JACM.pdf). Simply put, this means processes can halt or packets can be delayed.
2. There is only one case where a time-related system can be used in the RedLock algorithm: to prevent continuously waiting for nodes that do not respond. However, setting a TTL and not receiving a response within that TTL should not automatically be interpreted as the node being down.

Such asynchronous algorithm systems generally maintain safety properties without considering timing.

Even if the system's timing is in place (process pauses, network delays), algorithm performance might be terrible, but it means the algorithm will never produce incorrect results.

However, because the RedLock algorithm can break with bad timing and cannot perfectly solve concurrency issues, I don't think it's a very good choice.

For a detailed explanation of related issues, I referred to the [Martin Kleppmann - How to do distributed locking](https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html) document, which was linked above.
