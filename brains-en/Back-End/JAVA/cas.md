# CAS (Compare And Swap)

CAS is a **hardware-level atomic operation** provided by the CPU.

This operation allows shared data to be safely modified concurrently without acquiring a lock.

It operates as follows, taking three arguments:

```
CAS(memoryAddress, expectedValue, newValue)
```

And it works like this:

1. Compares the current value at the memory address with the expected value.
2. If they are the same -> Successfully replaces with the new value.
3. If they are different -> Fails and does nothing.

This entire process is executed atomically as a single CPU instruction, meaning that even if multiple threads execute concurrently, no interleaving can occur.

```java
int expected = 10;
int newValue = 11;

if (memory == expected) {
    memory = newValue;
    return true;
} else {
    return fase;
}
```

In Java, this operation is provided by `Unsafe.compareAndSwapXXX()`, `AtomicInteger.compareAndSet()`, or `VarHandle`.

- Advantages: Guarantees atomicity without locks, low thread contention, and fast.
- Disadvantages: Requires retries upon failure, leading to spin loops, and can waste CPU cycles if contention is high.
- Feature: It is the fundamental building block for implementing lock-free algorithms.

In other words, CAS is a technique for resolving contention without locking, and

most Lightweight Locks and the Atomic package are CAS-based.

- After biased lock revocation, the mark word is exchanged using CAS during the lightweight lock phase.
- `AtomicInteger.incrementAndGet()` internally uses `compareAndSet()`.

It guarantees state exchange without locks, providing the foundation for quickly switching JVM lock ownership or state, which is why it was adopted.

### Background of ConcurrentHashMap Performance Improvement

The problem with the existing `Hashtable` / `Collections.synchronizedMap` was that

lock management was simplistic.

```java
synchronized(
    // put/get all locked
)
```

That is, because the entire map was protected by a single lock, multiple threads accessing it concurrently always had to wait.
-> If there were 100 threads, 99 would be waiting.

The key improvement points of ConcurrentHashMap are:
- Up to JDK 7, it used a segment structure:
  - The map was divided into multiple segments, and individual locks were applied.
  - Multiple segments could be accessed concurrently, improving parallelism.
  - During `put` and `get` operations, keys without collisions could be processed concurrently.
- From JDK 8 onwards, a hybrid of CAS + `synchronized`:
  - The segment structure was removed, and finer-grained control at the node level was introduced.
  - It uses CAS + `synchronized` blocks + `volatile` together.
  - Most `put`, `get`, and `update` operations are handled based on CAS.

Key performance improvement principles (JDK 8 standard):

CAS - If a bucket is empty when inserting a new key-value pair, it's inserted directly with CAS (no lock needed).
`synchronized` - Only the chain of the same bucket where a collision occurred is locked.
`volatile` variables - Ensure visibility of array references and changes to node values.
`TreeBin` - If a chain becomes too long, it's converted to a red-black tree, improving search speed to O(log n).

The default value is probably 8; after 8, data in that bucket is managed by a red-black tree, otherwise it's put into a list.

In other words, ConcurrentHashMap's performance improved by applying locks only where necessary, and replacing locks with CAS when not needed (and also by using trees).
