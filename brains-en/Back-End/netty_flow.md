# Netty Message Flow

Netty requests message transmission through a `Channel`.

```java
Channel channel = ...
channel.writeAndFlush(message);
```

![](https://user-images.githubusercontent.com/81006587/218373375-e1660f25-adf4-4c4a-9aa3-93cfec8eef46.png)

The `Channel` delivers messages to the `ChannelPipeline`. The `ChannelPipeline` fundamentally contains `TailContext` and `HeadContext`. (These can be considered the beginning and end of the Pipeline.)

Between the Tail and Head, user-registered **`ChannelHandlerContext`s** are connected in a chain structure, and the delivered message flows along this chain in the Outbound direction.

Each time a message passes through a handler in the `ChannelPipeline`, it checks whether the `EventExecutor` thread bound to the handler matches the thread currently requesting message transmission.

If the threads are different, the message is inserted into a Queue, and execution is returned immediately. Messages accumulated in the Queue are then processed asynchronously by the `EventExecutor`. In the first `ChannelHandlerContext` of the Pipeline, the requesting thread and the `EventExecutor` are always different, and the message is queued.

```java
abstract class AbstractChannelHandlerContext ... {
    private void write(Object msg, boolean flush, ChannelPromise promise) {
    	...
        EventExecutor executor = next.executor();
        if (executor.inEventLoop()) {
            if (flush) {
                next.invokeWriteAndFlush(m, promise);
            } else {
                next.invokeWrite(m, promise);
            }
        } else {
            final WriteTask task = WriteTask.newInstance(next, m, promise, flush);
            if (!safeExecute(executor, task, promise, m, !flush)) {
                task.cancel();
            }
        }
        ...
    }
}
```

If the user has not configured a separate `EventExecutor` (default), all handlers will share and use the `Channel`'s `EventLoop` thread. Therefore, message buffering in the Queue will not occur except at the Pipeline's Tail.

Conversely, if the user has configured an `EventExecutor` for a specific Handler, messages will be buffered in the Queue at handlers where the `Executor` differs, and then processed asynchronously by different `EventExecutor`s.

```java
abstract class AbstractChannelHandlerContext ... {
    public EventExecutor executor() {
        if (executor == null) {
            return channel().eventLoop();
        } else {
            return executor;
        }
    }
}
```

![](https://user-images.githubusercontent.com/81006587/218383017-f15474b8-ba22-4b65-9b94-39467096e6e8.png)

Messages that have passed through the Pipeline are then delivered back to the `Channel`.

Netty's `Channel` internally transmits messages over the network via an NIO channel.

```java
public class NioSocketChannel ... {
    protected void doWrite(ChannelOutboundBuffer in) throws Exception {
        SocketChannel ch = javaChannel();
        ...
        ByteBuffer buffer = nioBuffers[0];
        int attemptedBytes = buffer.remaining();
        final int localWrittenBytes = ch.write(buffer);
        if (localWrittenBytes <= 0) {
            incompleteWrite(true);
            return;
        }
        ...
    }
}
```
