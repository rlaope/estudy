# Native Memroy

JVM's native memory refers to the memory used by the JVM outside the heap, meaning memory directly allocated by the operating system, not the heap that stores objects.

The Java heap is managed by the JVM's GC, and its size is usually specified with -Xmx. However, native memory is memory directly requested from the operating system by the JVM's internal structures or libraries.

Therefore, the total memory usage of the JVM would be approximately Java heap + native memory + code cache.

Native memory is typically used by metaspace, thread stacks, direct byte buffers, JNI allocated areas, and so on.

```java
ByteBuffer buffer = ByteBuffer.allocateDirect(1024 * 1024 * 100); // 100MB
```

Memory allocated as shown above does not appear in the Java heap and is not subject to GC, so it must be explicitly deallocated.

<br>

### jcmd

You can find Java processes with `ps aux` and then use `jcmd` to understand the JVM's state.

jcmd is a tool for JVM monitoring that allows you to identify Java processes and obtain analytical information such as taking heap dumps or checking thread states.

If problems occur, such as a running Tomcat server suddenly running out of memory or experiencing CPU spikes, you can use jcmd to identify which process is causing the issue and immediately dump heap or thread-related information to determine the cause.

```bash
jcmd <pid> VM.native_memory summary
```

```yaml
Native Memory Tracking:
Total: reserved=256MB, committed=192MB
- Java Heap: 128MB
- Class: 10MB
- Code: 20MB
- GC: 15MB
- Thread: 8MB
...
```

By checking as above, you can identify JVM issues, from identification to memory leaks, deadlocks, or GC inefficiencies.

```bash
# thread stack trace
jcmd <pid> Thread.print

# thread heap dump
jcmd <pid> GC.heap_dump /path/to/dump.hprof
```

### NMT

The JVM can track native memory usage through its Native Memory Tracking (NMT) feature.

```bash
java -XX:NativeMemoryTracking=summary -XX:+UnlockDiagnosticVMOptions -XX:+PrintNMTStatistics ...
```

You can activate it by running the JAR with the options above. Once activated, you can track native memory using jcmd.

```bash
# 특정 ps
jcmd <PID> VM.native_memory summary

# 전체 ps
jcmd | grep java
```

```yaml
Native Memory Tracking:

Total: reserved=242MB, committed=178MB
- Java Heap (reserved=128MB, committed=128MB)
- Class (reserved=20MB, committed=15MB)
- Thread (reserved=15MB, committed=15MB)
- Code (reserved=25MB, committed=20MB)
- GC (reserved=30MB, committed=25MB)
- Compiler (reserved=8MB, committed=7MB)
- Internal (reserved=5MB, committed=4MB)
```

> Note that NMT is off by default, and enabling it incurs an additional overhead of about 1-10% for tracking. In performance-sensitive production environments, it's advisable to set it to `summary`.

It's somewhat contradictory, as this information would be even more crucial in performance-sensitive production environments. It's also recommended to use external allocators or profilers, such as checking with `pmap`, reporting memory usage with `ps top smem`, or allocating memory with `jemalloc` and viewing it with `jemalloc_stats_print`.

https://www.facebook.com/notes/facebook-engineering/scalable-memory-allocation-using-jemalloc/480222803919

It is a memory allocator developed for use in FreeBSD, providing efficient memory allocation and management in multi-threaded environments, specifically optimizing performance for large-scale multi-threaded applications.

We will explore jemalloc further in the next post.
