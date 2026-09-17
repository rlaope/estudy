# ByteBuf, Memory Management

Let's explore the differences between Java's `ByteBuffer` and Netty's `ByteBuf`.

Java's default `ByteBuffer` is inconvenient to use and has many performance limitations (e.g., requiring `flip()` for read/write mode switching).

Netty's `ByteBuf` improves upon this by having separate `reader index` and `writer index`.

The `reader index` tracks how much has been read, and the `writer index` tracks how much has been written, allowing for direct read/write operations without `flip()`.

### Memory Allocation (Direct Buffer vs Heap Buffer)

This can be seen as crucial for zero-copy operations and runtime (GC) performance.

A `Heap Buffer` (`Unpooled.buffer()`) resides within the JVM heap memory. Its drawback is that when a socket sends data, the OS cannot directly access data located in the JVM heap.

The heap buffer must be copied to a direct memory region accessible by the OS, which then sends it via the socket. This copying incurs wasted overhead.

However, a **Direct Buffer** can be written directly to the native memory area outside the JVM heap, allowing the OS to access its memory address directly and send it to the socket, thus eliminating the copying process.

On the other hand, memory allocation/deallocation costs are very high because OS system calls are required. This is why Netty employs pooling.

### Pooling and reference counting

Netty doesn't create expensive direct buffers every time; instead, it keeps them in a pool and lends them out. This approach can lead to memory leaks.

Since direct memory is an area not managed by the GC, developers must manually manage reference counts.

-   `refCnt`: The reference count, which starts at 1 when an object is created.
-   `retain()`: Increments the count by 1.
-   `release()`: Decrements the count by 1.
-   Deallocation: When `refCnt` becomes 0, it returns to the pool.

```java
// Your code
try {
    // ... logic ...
} finally {
    m.release(); // [Very good] Return it after use. Without this, a memory leak occurs.
}
```

In this way, you must call `release()` at the end to reclaim the direct memory space.

Forgetting this can lead to a direct memory OOM, or if managed incorrectly, attempting to reference an already released buffer might cause an `IllegalReferenceCountException` (trying to use something that has already been `release()`d and whose count is 0).

### Slice, CompositeByteBuf

Netty never copies bytes when combining or slicing data.

It merely creates a new view. For example, when you want to read only the header from a large data block, `slice` doesn't copy the data into a new array; instead, it creates a thin object that only holds pointers.

Like `ByteBuf header = msg.slice(0, 10);`, it shares memory with the original `msg`. This means the copying cost is zero (assuming 0-10 is the header).

`CompositeByteBuf` is for combining. When you need to send a header and body together, it doesn't create a new array and copy both into it; instead, it logically concatenates the two buffers.

```java
CompositeByteBuf compBuf = Unpooled.compositeBuffer();
compBuf.addComponents(true, headerBuf, bodyBuf);
// No actual memory copying occurs.
```

```java
package kr.honestfund.nice.proxy.server;

import io.netty.buffer.ByteBuf;
import io.netty.channel.ChannelHandlerContext;
import io.netty.handler.codec.ReplayingDecoder;
import lombok.extern.slf4j.Slf4j;
import java.nio.charset.Charset;
import java.util.List;

// State Definition: Is it time to read the header, or the body?
enum NiceState {
    READ_HEADER,
    READ_BODY
}

@Slf4j
public class NiceMessageDecoder extends ReplayingDecoder<NiceState> {

    private final Charset charset = Charset.forName("EUC-KR");
    private int bodyLength = 0;

    public NiceMessageDecoder() {
        super(NiceState.READ_HEADER); // Set initial state
    }

    @Override
    protected void decode(ChannelHandlerContext ctx, ByteBuf in, List<Object> out) {
        switch (state()) {
            case READ_HEADER:
                // [Key Point] No need to check in.readableBytes().
                // If ReplayingDecoder finds insufficient data, it automatically pauses here,
                // and resumes execution when more data arrives (allowing you to code as if it were blocking).
                
                // Read 10 bytes (to minimize String object creation, reading as byte[] and parsing is recommended)
                CharSequence headerStr = in.readCharSequence(10, charset);
                try {
                    // "0000000100" -> 100 (Whether the 10-byte header itself is included depends on the protocol)
                    // Based on the written code: headerStr itself is assumed to be the body length.
                    bodyLength = Integer.parseInt(headerStr.toString());
                    
                    // [State Transition] Now, let's go read the body.
                    checkpoint(NiceState.READ_BODY); 
                } catch (NumberFormatException e) {
                    log.error("Invalid Header: {}", headerStr);
                    ctx.close();
                    return;
                }
                break;

            case READ_BODY:
                // Read by body length.
                // Again, if the data is less than bodyLength, ReplayingDecoder automatically waits.
                String body = in.readCharSequence(bodyLength, charset).toString();
                
                // [Complete] Pass the entire message (Header + Body) to the subsequent handler (NiceProxyHandler).
                // If necessary, the Header can also be combined and passed.
                out.add(headerStr + body); 
                
                // [Reset] Return to the header state for the next packet.
                checkpoint(NiceState.READ_HEADER);
                break;
        }
    }
}
```
