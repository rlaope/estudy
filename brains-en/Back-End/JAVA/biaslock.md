# Bias Locking, Revocation

`synchronized` blocks internally use monitor locks.

Typically, locks are acquired and released through atomic operations like CAS, but CAS is expensive because it locks at the CPU instruction level.

In most applications, the pattern where **the same thread always locks the same object** is very common.

```java
void foo() {
    synchronized (lock) {
        // Repeated calls from the same thread
    }
}
```

Therefore, performing CAS every time is unnecessary, and to optimize this pattern, the JVM introduced bias locking.

This optimization biases a lock object to a specific thread, allowing subsequent lock acquisitions by the same thread to skip CAS.

It stores the thread ID in the object header's mark word, indicating that the object is biased towards that thread.

Using this value, re-entry by the same thread can quickly acquire the lock without CAS.

State transition process:
- biased
- lightweight lock
- heavyweight lock (transitions in this order)

To disable it, you can use `-XX:-UseBiasedLocking`, but this option is said to have disappeared after JDK 15. Just don't use it. Why? I'll explain that below, along with the multi-threaded environment.

The mark word looks like this:

```
[ ThreadID | Epoch | Biased bit | Lock bits | Age | HashCode ... ]
```

Mark data is a header value containing object metadata, and thread information for biasing is directly recorded here.

The problem is that while it's convenient in a single-threaded environment, it can become an overhead in a multi-threaded environment.

If multiple threads alternately lock the same object, causing the bias to frequently break and reset, frequent bias revocation occurs, leading to STW (Stop-The-World) due to safepoints.

If class epochs are updated during GC, existing object biases become invalidated, which can also cause STW.

In multi-threaded initialization patterns, repeated bias reassignments when each thread first accesses an object can lead to startup latency.

An increased frequency of safepoints also increases stop latency.

For these reasons, it is often disabled in service environments with high multi-threaded contention.

It has simply been removed since JDK 15 and later. It doesn't exist.

In multi-threaded servers, disabling it is just the default choice. If you're on JDK 8-14, disable it using the option mentioned above.

### Operational Strategy

Safepoint analysis: Since bias revocation can trigger safepoints, it can be diagnosed with the following options.

```
-XX:+PrintSafepointStatistics -XX:+UnlockDiagnosticVMOptions
-XX:+PrintGCApplicationStoppedTime
```

Among these, if `Time to safepoint` is large, bias revocation or native waiting threads are likely bottlenecks.

Adjusting bias revocation policy:
The JVM manages bias policy at the class level. When a class-level epoch increases, the bias of objects of that class is invalidated. If a specific class frequently causes bias transitions, the JVM automatically decides to disable bias for it.

This policy is automatic, so manual adjustment is generally difficult, but you can check if a specific class is causing contention through JFR or GC logs.

Replaced by Lightweight Lock and Lock Elision in recent JDKs.

```
③	Lightweight Lock	CAS-based	Multiple threads access alternately (short contention)

④	Heavyweight Lock	Uses OS-level monitor (mutex)	For high lock contention or long waits
```
