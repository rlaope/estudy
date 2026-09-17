# vDSO(virtual Dynamic Shared Object)

vDSO is a small shared library that the Linux kernel pre-maps into user space memory.

It is a high-performance system call optimization mechanism provided by the Linux kernel, and the JVM utilizes it as-is.

Its purpose is to allow system calls that can be safely processed in user space, without needing to trap into the kernel, to be handled **without kernel entry**.

In other words, syscall -> kernel -> return to user mode.

This structure reduces the overhead of relatively frequent calls (such as time-related calls) by omitting this process.

Internally, the JVM frequently uses the following calls in various places such as time lookups, thread scheduling, random number seeds, and GC timestamps.

`gettimeofday()`, `clock_gettime()`, `time()`, `clock_getres()`

Among these, calls like `clock_gettime(CLOCK_MONOTONIC)` are repeatedly invoked in almost all GC phases and JVM scheduling.

What if these functions are implemented via vDSO? The JVM processes them quickly in user mode without a kernel round trip.

### JVM vDSO

Time-based operations frequently occur within the JVM. Let's look at a few examples:

1.  GC timestamp
    1.  GC start ~ GC end time
    2.  STW
    3.  Young/Old GC event timestamp

All the above values are retrieved using internal JVM functions like `os::javaTimeNanos()`, `os::elapsedTime()`, and `os::elapsed_counter()`, which internally call `clock_gettime()`. If vDSO is enabled, it is much faster.

In addition, there are JIT Compile Scheduling, lock mutex timing, thread scheduling, and more. For VMs with very frequent time-based operations, the cumulative difference, though subtle, can be significant depending on the presence of vDSO.

On Linux, you can check if vDSO is enabled using the command `cat /proc/$(pgrep -n java)/maps | grep vdso`.

```
7ffe39bff000-7ffe39c00000 r--p 00000000 00:00 0                          [vvar]
7ffe39c00000-7ffe39c01000 r-xp 00000000 00:00 0                          [vdso]
```

If you see it, it's enabled.

To check which functions are called from vDSO, you can use `objdump -T /usr/lib64/libc.so.6 | grep vdso`.

When a context switch occurs, transitioning from user mode to kernel mode, the instruction cache and other caches of the user thread are flushed.

This is because the memory regions accessed by user-space code and those accessed by the kernel generally do not overlap.

When switching to kernel mode during a context switch, other potential caches are also invalidated by flushing the Translation Lookaside Buffer (TLB).

When the call returns, these caches must be refilled, and the effects of such transitions persist even after control returns to user space.

Therefore, it can be seen that virtual Dynamic Shared Objects (vDSO) are used to reduce these costs.

Its purpose is to accelerate system calls that do not actually require kernel privileges, by handling them within user-space memory.

Simply put, it's like saying, "Why go to the kernel level when you don't need to? Don't go, use this instead."
