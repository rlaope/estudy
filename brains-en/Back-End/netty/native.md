# Netty Native Memory Tuning, BackPressure

In Netty, native memory usually refers to the combination of the following:

- **Direct ByteBuf (off-heap)**: Memory allocated by `ByteBufAllocator` using `ByteBuffer.allocateDirect()`-like methods.
- **Netty Pooled allocator's chunk/arena structure**: A structure that pre-allocates large blocks and then divides them for use.
- Additionally, if JNI/SSL (OpenSSL), compression libraries, etc., are used, native memory is also allocated there.

For example, if a message in a message-based communication is 14KB, the direct memory usage won't be just 14KB; it can be higher due to the pooling structure and cache backlog.

### why off-heap

Socket I/O involves data moving between kernel buffers and user space. Direct memory can be advantageous in certain scenarios as it reduces intermediate copying, especially the cost of copying from heap to direct memory.

Netty primarily uses Pooled + Direct for high performance.

However, the downside is that it's an area not managed by GC. If leaks or backlogs occur, the heap might be fine, but direct memory can exhaust.

Therefore, in production, setting limits, observability, backpressure, and leak prevention are essential.

For instance, let's assume requests up to 200 RPS for a 14KB message.

The payload throughput would be very low: `14kb x 200/s = 2,800kb/s`, which is 2.8MB/s. Assuming an RTT of 50-100ms, the inflight requests would be 10 for 50ms and 20 for 100ms.

Even generously doubling for requests + responses, it's 20 x 14KB x 2 = 560KB, which is less than 1MB of pure data.

However, in production, direct memory usage often appears to be hundreds of MB. The reasons are always two-fold:

- **Pooled allocator's baseline usage (chunks/arenas/thread cache)**
  - Instead of allocating small buffers from the OS on demand, it acquires large blocks (chunks) and reuses them.
  - As the number of worker threads increases, a baseline is established due to arenas/caching.
- **Outbound backlog (write queue builds up when the peer is slow)**
  - If writes cannot fully enter the OS send buffer, they accumulate in the Netty outbound buffer.
  - This is the primary factor that drives direct memory peak usage to hundreds of MB.

So, if we propose a hypothesis and budget before measurement, and then validate with real-world scenarios:

- Normal (baseline): 150~250MB
  - Due to the pooling structure/thread cache, this much is occupied even with low traffic.
- Peak (peer delay + backlog): 300 ~ 450MB
  - If peer delays occur and flushes are pushed back, it can spike to this range.
- For the operational budget ceiling, 256MB should be sufficient by default. If there are latency fluctuations, many connections, or SSL is involved, let's allocate 512MB.

### OutOfDirectMemoryError

You might encounter situations where heap GC is fine, but direct memory continuously increases, even with constant traffic.

A common cause can be memory leaks due to not calling `release()` after `retain()` or `retainSlice()`, causing the memory region to be held indefinitely.

For instance, in `SimpleChannelInboundHandler`, not understanding `autoRelease` behavior and moving references outside,

or omitting `release()` calls in exception paths.

In staging, you can use `LeakDetector` to get accurate stack traces, define ownership rules, and enforce them in the code to prevent such issues.

> This is a debugging feature to catch leaks in Netty's `ByteBuf` (reference-counted objects).

```java
@Override
public void channelRead(ChannelHandlerContext ctx, Object msg) {
  try {
    // Processing logic
  } finally {
    ReferenceCountUtil.release(msg);
  }
}
```

For leaks, use `LeakDetector` to capture the stack and prevent recurrence with ownership rules.

### LockDetector

Also known as `ResourceLeakDetector`, this is a debugging feature to catch leaks in Netty's `ByteBuf` (reference-counted objects).

- Netty `ByteBuf`s (especially Direct) are not automatically reclaimed by GC; memory is returned only when `release()` is called to set the reference count to 0.
- If `retain()`/`retainedSlice()` is held somewhere without a corresponding `release()`, direct memory will increase.
- `LeakDetector` detects when a `ByteBuf` is not deallocated and logs the code path where the leak occurred as a stack trace.

When Netty allocates a `ByteBuf`, depending on sampling/options, it attaches a tracking object (based on weak references) to that buffer. If the buffer becomes a GC candidate but there's no record of it being properly released via `release()`, it logs messages like "LEAK: ByteBuf.release() was not called".

The log includes the stack trace showing where this buffer was retained. This means it tracks missing `refCnt` releases rather than directly reading direct memory from the OS.

#### Level

To reduce performance overhead, `LeakDetector` has levels based on how aggressively it tracks, though specifics may vary by version:

- **Simple**: Tracks only a portion through sampling. It can catch leaks, but stack information might be limited. The cost is relatively low.
- **ADVANCED**: Tracks more detailed stacks than Simple and is often used to catch leaks in staging. The cost is higher, but it's easier to find which code caused the leak.
- **PARANOID**: Tracks almost all allocations (or a very high percentage), making it the most effective. However, the cost is highest (potential performance degradation and log explosion). Typically used when leaks are hard to reproduce but must be caught.

In production, it's acceptable to disable it due to overhead, assuming you are confident in your testing.

```
-Dio.netty.leakDetection.level=advanced
# 또는 paranoid / simple / disabled

or

ResourceLeakDetector.setLevel(ResourceLeakDetector.Level.ADVANCED);
```

### Write Backlog Surge

If the peer server slows down, responses will be delayed, and direct memory usage can simultaneously spike to its peak because it's held continuously.

The channel might not be able to keep up with writes for a while (partial write).

This can happen when reads continue (even with `AUTO_READ=false`), but writes cannot be sent, causing the outbound buffer to grow due to accumulation.

The solution is to switch to an unwritable state by configuring `WriteBufferWaterMark`.

In `channelWritabilityChanged`, **reading can be paused/resumed**, and flush batches should be performed (if possible).

```java
childOption(ChannelOption.WRITE_BUFFER_WATER_MARK,
    new WriteBufferWaterMark(32 * 1024 * 1024, 64 * 1024 * 1024)); // 32MB/64MB

childOption(ChannelOption.AUTO_READ, false); // Manual read control

@Override
public void channelWritabilityChanged(ChannelHandlerContext ctx) {
  if (ctx.channel().isWritable()) {
    ctx.read(); // Resume
  } // If unwritable, maintain paused read state
  ctx.fireChannelWritabilityChanged();
}
```

In summary, peaks are mostly caused by backlogs, which can be prevented from escalating using watermarks and read control.

#### Actions that actually occur after becoming unwritable

Internally in Netty:
- If `totalPendingWriteBytes >= highWaterMark`,
- `channel.isWritable()` becomes `false`.
- A `channelWritabilityChanged()` event is fired.
- Most common cause: Socket writes are delayed, and unsent data accumulates in the `ChannelOutboundBuffer`.

The server should stop generating additional writes. Logic like `if(!ch.isWritable) { stop producing }` should be implemented. If writes continue even when unwritable, the outbound buffer will grow further, leading to direct memory exhaustion, so this must be avoided.

It's also good to block inbound traffic. For a proxy synchronous response server, if `AUTO_READ = false`, reading should remain paused when unwritable. If `AUTO_READ = true`, `setAutoRead(false)` should be called to block inbound traffic.

For requests already received, there are three policy options:
1. Load them into a bounded queue and wait (a limit is necessary).
2. Reject immediately (busy/overload).
3. Terminate the connection (fail-fast).

The key is that the unwritable period is not for continuous processing, but for stopping growth, halting inbound production, and draining accumulated items.

#### Actions when it becomes writable again

Internally in Netty:

- Outbound data is sent, and `totalPendingWriteBytes` decreases.
- `totalPendingWriteBytes <= lowWaterMark`.
- `channel.isWritable()` reverts to `true`.
- A `channelWritabilityChanged()` event is fired.

The server resumes paused reads (calling `ctx.read()` if `AUTO_READ=false`, or `setAutoRead(true)` if `AUTO_READ=true`).

Paused writes should also be resumed, and message generation/queue consumption/proxy forwarding that was halted during the unwritable state should restart.

However, when resuming, implement batch rate limiting to prevent a sudden surge.

If there's an internal queue waiting, it should be drained. If items were queued while unwritable, resume sending and drain them gradually while checking `isWritable`.

### Observability

JVM NMT (Native Memory Tracking)

Enabling NMT in production/staging allows you to see the native memory composition.

```
-XX:NativeMemoryTracking=summary
-XX:+UnlockDiagnosticVMOptions
```

```bash
jcmd <pid> VM.native_memory summary
jcmd <pid> VM.native_memory detail
```

With [Netty allocator metrics](https://github.com/micrometer-metrics/micrometer/blob/main/micrometer-core/src/main/java/io/micrometer/core/instrument/binder/netty4/NettyAllocatorMetrics.java), you can extract usage/arena status for the Pooled allocator. In production, you need to look at these values along with process RSS and NMT to truly understand the root cause.

`LeakDetector` can also be used; the principle is to use ADVANCED in development/staging (PARANOID if necessary), and usually disable it in production due to overhead, as mentioned above.

### Summary

Prevent backlog delays with `WriteBufferWaterMark` + `AUTO_READ` control,

and prevent direct memory exhaustion by setting an upper limit.

```
-XX:MaxDirectMemorySize=256m
```

If a leak is suspected, set up `LeakDetector` or observability for monitoring.
