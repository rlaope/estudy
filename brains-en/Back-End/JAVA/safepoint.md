# JVM Safepoint

A safepoint refers to a point within the JVM where thread execution is temporarily suspended.

It is used when all Java threads must be safely stopped to perform internal JVM management tasks such as GC, deoptimization, or thread dumps.

It's a technique to bring the execution state of application threads to a consistent point to perform specific JVM tasks like garbage collection.

This is because if a thread stops at an arbitrary location, the memory state might become inconsistent. Therefore, threads can only stop at a safepoint, which is a safe point designated by the JVM.

### Operation Process

1.  The JVM requests a safepoint (`Safepoint request`).
2.  It signals all Java threads to enter the safepoint.
3.  Each thread detects the signal at a safepoint polling location within its JIT-compiled code and stops.
4.  Once all threads have stopped, the JVM performs tasks such as GC or class redefinition.
5.  After task 4 is complete, the safepoint is released, and threads resume.

Examples of when it occurs include:

Garbage Collection

Thread dump (when `jstack` is executed)

Deoptimization (JIT code → interpreter code conversion)

Class redefinition (HotSwap)

Bias locking revocation

> Bias Locking Revocation is the process by which Biased Locking, one of the JVM's lock optimization techniques, is revoked.

### Performance-related Issues

Safepoints can directly impact system performance.

Because the JVM can only begin its management tasks once all threads have entered a safepoint,

if a thread occupies the CPU for a long time or remains in native code, a delay in entering the safepoint occurs (Safepoint Stall).

This delay can appear in GC logs as follows:

```sql
Safepoint "Cleanup", Time since last: 100000 ms, Time to safepoint: 1234 ms
```

If the `Time to safepoint` value here is large, it means a specific thread has been unable to perform safepoint polling for a long time.

JIT-compiled code has polling statements inserted like the following:

```java
while (!SafepointSynchronize::do_call()) {
    // regular Java execution
}
```

This polling is located at points the JVM deems safe, such as before function calls or before entering loops. Since polling is not possible during native code execution or long-blocking I/O calls, these become major causes of safepoint delays.

### Diagnosis Methods

-   Check `Time to safepoint` in GC logs
-   Enable the `-XX:+PrintGCApplicationStoppedTime` option
-   Check detailed statistics with `-XX:+PrintSafepointStatistics`
-   Use `jstack` or `jcmd VM.safepoint`
