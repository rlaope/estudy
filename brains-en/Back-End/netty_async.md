# Netty Async Flow Control

Last time, we looked at how Netty identifies sockets and channels from a component called Selector (a KV table stored in heap memory), detects events, and processes operations. This time, we'll explore how business logic consumes those results. It's good to learn about asynchronous flow control, and today we'll cover Future and Promise.

And also Listener.

## Async Flow Control

In Netty, async flow control refers to the mechanism that manages the flow to prevent the entire system from crashing, even when an event loop handles one or many channels, in situations such as:
- A channel reads too much or writes are backed up
- A handler takes too long or is blocking
- Backpressure due to a slow peer

The core consists of three axes.

### Read Flow Control

`ChannelConfig#setAutoRead(true/false)`: If true, Netty continuously reads and raises events when the socket is ready to read. If false, the application only reads when it explicitly calls `ctx.read()` or `channel.read()`.

This allows for manual backpressure, such as signaling "don't read now, as we don't have the capacity to process it".

If the downstream (e.g., external API, DB, message queue) is slow and queues build up, switch to `autoRead=false`. Once the queue drains, set it back to true.

### Write Flow Control

If a write cannot be immediately flushed to the OS, Netty accumulates it in the OutboundBuffer. If this buffer grows too large, it can lead to out-of-memory errors or delays, so Netty provides the following:

- `Channel.isWritable()`
- `channelWritabilityChanged(...)` event
- `WriteBufferWaterMark(low/high)`: By default, if the high watermark is exceeded, the channel becomes unwritable.

In other words, if the outbound buffer becomes too full, `isWritable` turns false, and handlers receive this signal to stop or slow down additional writes, thereby controlling the flow.

In Netty, the primary signal for true backpressure is typically `isWritable`.

### Event Loop Protection

An event loop processes events for multiple channels in a round-robin fashion using a single thread.

Therefore, if a handler performs blocking I/O (e.g., external API calls, DB access, file I/O), all channels associated with that event loop will halt.

Thus, Netty's async flow control also involves the discipline of keeping the event loop short.

Offload long-running tasks to a separate `blockingPool` or `EventExecutorGroup`, and then return only the results to the event loop for subsequent processing.

### Future Netty, Concurrent Package

While Java's standard `java.util.concurrent.Future` also exists, Netty seemingly found it insufficient and created a more powerful Future.

> Isn't `java.util.concurrent.Future` also a class representing an asynchronous operation?

Future is a half-baked asynchronous concept. To be precise, it has an inherent limitation: the computation runs asynchronously, but retrieving the result must be done synchronously.

The core purpose of `java.util.concurrent.Future` is to wait for results using `get()` (blocking), and it lacks callbacks.

Netty's Future, on the other hand, centers around asynchronous callbacks via `addListener`, with `await`/`sync` being auxiliary functions that should be avoided in the EventLoop.

**Differences in cancellation/interrupt models** also exist:
- Java's Future primarily relies on interrupt-based cancellation, such as `cancel(true)`.
- Netty's Future operates in accordance with the EventLoop single-threaded model and I/O completion model. The meaning of interrupt is relatively weak, and channel closure or failure completion are more natural termination methods.

**Different control over execution context (thread)**
- Java's Future's completion location depends on the Executor, and its callback concept is weak.
- Netty, based on the EventLoop, strongly couples completion and listener execution, and this coupling is key to Netty's performance stability.

Additionally, Netty's Future is closely tied to I/O operations. It offers extended functionalities, such as naturally attaching code that continues to the pipeline (next stage) after success or failure, to listeners.

The biggest reason for its new development is the combination of the blocking nature of the existing `Future.get()`, pipeline listeners, and the way Netty operates with OS event-based completion.

<br>

## Netty Promise Future Listener

### ChannelFuture

In Netty, the result of an asynchronous operation (primarily I/O) comes as a `ChannelFuture`.

APIs like `connect`, `write`, and `close` return immediately, and their success or failure is determined when the returned `ChannelFuture` completes.

`future.isSuccess()`, `future.cause()`, `future.channel()`, `future.sync() or await()` allow for blocking waits but should be avoided.

### GenericFutureListener

This is a callback that will be invoked upon completion: `future.addListener(..)`.

When the future completes, the listener is called, typically to execute subsequent actions.

An important point is that listeners are often executed on the thread that completes the `Future`, which is usually the EventLoop.

Therefore, heavy tasks or blocking calls should not exist within the listener itself.

### Promise

A `Promise<V>` is both a Future and an object whose **completion I control**.

`promise.setSuccess(value)` / `promise.setFailure(cause)`

In other words, it's used by Netty's internal user code to represent the completion of an asynchronous operation.

Why is it needed?
- `ChannelFuture` often represents the result of Netty's I/O operations. If you want to integrate custom asynchronous tasks in a Netty-style, you need a container whose completion you can control.
- For example, when an external thread pool task finishes, it returns to the EventLoop to complete the Promise, and then proceeds with the next Netty pipeline stage.

Let's look at a code example.

Offloading requests received in `channelRead` to a separate thread pool,

completing the result with a Promise, returning to the EventLoop, and then `writeAndFlush`,

attaching a `GenericFutureListener` to a `ChannelFuture` for success/failure handling,

and also handling `isWritable()` / `channelWritabilityChanged` for write backpressure.

```java
import io.netty.channel.*;
import io.netty.util.concurrent.*;

import java.nio.charset.StandardCharsets;
import java.util.ArrayDeque;
import java.util.Queue;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public final class PromiseFutureListenerExampleHandler extends SimpleChannelInboundHandler<Object> {

    // Pool to offload blocking tasks (e.g., external API, DB)
    private final ExecutorService blockingPool = Executors.newFixedThreadPool(16);

    // Queue to temporarily store writes during backpressure (for demo)
    private final Queue<Object> pendingWrites = new ArrayDeque<>();

    // Backpressure control example: turn off reading if too many accumulate
    private static final int MAX_INFLIGHT = 100;
    private int inflight = 0;

    @Override
    public void channelActive(ChannelHandlerContext ctx) {
        // autoRead can be controlled if needed
        ctx.channel().config().setAutoRead(true);
    }

    @Override
    protected void channelRead0(ChannelHandlerContext ctx, Object msg) {
        // Inflight limit (similar to session/concurrency limits)
        if (inflight >= MAX_INFLIGHT) {
            // Stop reading if no more can be received
            ctx.channel().config().setAutoRead(false);
            // msg might require buffer/reference management (simplified to Object here)
            return;
        }

        inflight++;

        // 1) Create Promise: I will complete this task with setSuccess/setFailure.
        Promise<String> promise = new DefaultPromise<>(ctx.executor()); // ctx.executor() == EventLoop

        // 2) Action upon Promise completion (listener)
        promise.addListener((Future<String> f) -> {
            // This listener is designed to run on the EventLoop by default (DefaultPromise is ctx.executor based)
            try {
                if (f.isSuccess()) {
                    String response = f.getNow();

                    // 3) ChannelFuture: asynchronous result of write
                    ChannelFuture writeFuture;
                    if (ctx.channel().isWritable()) {
                        writeFuture = ctx.writeAndFlush(toByteBuf(ctx, response));
                    } else {
                        // If backpressure, add to queue and flush later
                        pendingWrites.add(toByteBuf(ctx, response));
                        return;
                    }

                    // 4) Attach GenericFutureListener to ChannelFuture
                    writeFuture.addListener((ChannelFuture wf) -> {
                        if (wf.isSuccess()) {
                            // Write successful
                        } else {
                            // Write failed - check cause
                            Throwable cause = wf.cause();
                            wf.channel().close();
                        }
                    });

                } else {
                    // Asynchronous task failed
                    Throwable cause = f.cause();
                    ctx.writeAndFlush(toByteBuf(ctx, "ERROR: " + cause.getMessage()))
                       .addListener(ChannelFutureListener.CLOSE);
                }
            } finally {
                inflight--;
                // Inflight reduced, resume reading
                if (!ctx.channel().config().isAutoRead() && inflight < MAX_INFLIGHT) {
                    ctx.channel().config().setAutoRead(true);
                    // After enabling autoRead, trigger read if necessary
                    ctx.read();
                }
            }
        });

        // 5) Execute actual blocking task (e.g., external API call) in a separate pool
        blockingPool.execute(() -> {
            try {
                // (Important) This is not an EventLoop thread. Never call Netty operations like ctx.write directly here.
                String result = doBlockingCall(msg);

                // 6) It's safe to pass Promise completion to be executed on the EventLoop (thread-safe/consistency)
                ctx.executor().execute(() -> promise.setSuccess(result));

            } catch (Throwable t) {
                ctx.executor().execute(() -> promise.setFailure(t));
            }
        });
    }

    @Override
    public void channelWritabilityChanged(ChannelHandlerContext ctx) {
        // When the channel becomes writable again, drain pendingWrites.
        if (ctx.channel().isWritable()) {
            while (!pendingWrites.isEmpty()) {
                Object out = pendingWrites.poll();
                ChannelFuture f = ctx.write(out);
                f.addListener((ChannelFuture wf) -> {
                    if (!wf.isSuccess()) {
                        wf.channel().close();
                    }
                });
            }
            ctx.flush();
        }
        ctx.fireChannelWritabilityChanged();
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        ctx.close();
    }

    private static Object toByteBuf(ChannelHandlerContext ctx, String s) {
        return ctx.alloc().buffer().writeBytes(s.getBytes(StandardCharsets.UTF_8));
    }

    private static String doBlockingCall(Object msg) {
        // Demo: Assume blocking operations like external API/DB calls happen here
        // Thread.sleep(50);
        return "OK";
    }
}

```
