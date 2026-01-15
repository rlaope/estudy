# Netty Async Flow Control

네티가 소켓과 채널을 Selector라는 컴포넌트로부터(힙메모리에 저장된 kv 테이블) 식별하고 이벤트를 감지해

작업을 처리하는 동작 과정을 알아보았다 저번 시간에 이번에는 그 결과값을 어떻게 비즈니스 로직이 받아먹느냐

비동기 흐름제어를 익히면 좋은데 오늘 알아볼건 future, promise이다.

그리고 listener 까지도.

## Async Flow Control

Netty에서 async flow control은 이벤트루프 1개 혹은 엄청많은 채널을 처리하면서도 
- 한 채널이 너무 많이 읽거나 write가 밀리거나
- 핸들러가 오래 걸리거나 blokcing
- 상대가 느려서 backpressure

전체 시스템이 망가지지 않게 흐름을 제어하는 메커니즘을 말한다.

핵심은 3가지 축이다.

### 읽기 흐름 제어

`ChannelConfig#setAutoRead(true/false)` 여기서 true면 소켓이 읽을 준비가 되면 netty가 계속 읽어 이벤트를 올리고 false면 애플리케이션이 직접 ctx.read() or channel.read()를 호출할때만 읽는다.

이걸로 지금 처리할 여력은 없으니 읽지 말자와 같은 수동 backpressure를 만들 수 있다.

다운스트림(예: 외부 api, db, message queue)이 느려서 큐가 쌓이면 `autoRead=false`로 전환해 큐가 내려가면 다시 true로 둔다.

### 쓰기 흐름 제어

netty는 write가 즉시 os로 flush되지 못하면 OutboundBuffer에 쌓인다. 이게 커지면 메모리가 터지거나 지연이 발생하니까 netty는 다음을 제공한다.

- `Channel.isWritable()`
- `channelWritabilityChanged(...)` 이벤트
- `WriteBufferWaterMark(low/high)`: 기본적으로 high watermark를 넘으면 unwritable로 바뀜

즉, outbound buffer가 너무 차면 isWritable=false로 바뀌고 핸들러가 그 신호를 받아 추가 write를 멈추거나 속도를 늦추는 방식으로 흐름제어를 한다.

네티에서 진짜 벡프레셔의 1차 신호는 보통 `isWritable` 이다.

### 이벤트 루프 보호

eventloop는 한 스레드가 여러 채널의 이벤트를 돌려 처리한다.

따라서 핸들러의 blocking io(외부 api 호출, db, file io)를 하면 그 event loop에 걸린 모든 채널이 멈추게 되는데

그래서 네티 async flow control은 eventloop를 짧게 유지하는 규율이기도 한다.

오래걸리는 작업은 별도의 blockingPool, EventExecutorGroup 등으로 오프로드 시키고 결과만 다시 이벤트루프로 돌려줘 이어서 처리하게 하자.

### Future Netty, Concurrent Package

자바 표준 `java.util.concurrent.Future`에도 Future가 있지만 Netty는 이걸로 부족했는지 더 강력한 Future를 만들었다.

>  `java.util.concurrent.Future` Future도 비동기 연산 나타내는 클래스 아닌가요?

Future는 반쪽짜리 비동기다 정확히 말하면 작업(calculation)은 비동기로 돌아가는데 결과 조회는 동기로 해야하는 태생적인 한계가 존재한다.

`java.util.concurrent.Future` `get()`으로 결과를 기다리는 (블로킹)용도가 핵심이고 콜백이 없다.

Netty의 Future는 addListener로 비동기 콜백이 핵심이고 await/sync는 보조 기능이고 EventLoop에서는 지양한다.

**cancellation/interrupt 모델 차이**도 존재하는데
- java Future는 cancel(true) 같은 interrupt 기반이 중심이다.
- Netty Future는 EventLoop 단일 스레드 모델과 io 완료 모델에 맞게 동작한다 interrupt 의미가 상대적으로 약하고 채널 close / 실패 완료가 더 자연스러운 종료 방식이다.

**실행 컨텍스트(스레드) 통제가 다름**
- java Future는 어디서 완료될지가 Executor에 좌우되고 콜백 개념이 약하다. 
- Netty는 EventLoop 기반으로 완료/리스너 실행이 강하게 결합되고 이 결합이 netty의 성능 안정성을 만드는 핵심

그 외에도 io작업과 밀접한 Netty Future. 성공 실패이후 pipeline(다음 단계)로 이어지는 코드를 리스너에 자연스럽게 붙일수도 있음 이런 확장기능들이 있다.

가장큰건 기존 Future.get()의 블로킹과 pipeline 리스너, os 이벤트 기반으로 완료되는 네티 동작과정과의 결합등때문에 새로 개발된것

<br>

## Netty Promise Future Listener

### ChannelFuture

Netty에서 비동기 작업(주로 io)의 결과는 `ChannelFuture`로 온다.

connect, write, close 같은 api가 즉시 반환하는데 그 반환값이 ChannelFuture로 완료되면 성공/실패가 결정된다.

`future.isSucceess()` `future.cause()`, `future.channel()` `future.sync() or await()` 블로킹 대기가 가능하지만 지양

### GenericFutureListener

완료 시점에 호출될 콜백이다 `future.addListener(..)`

완료되면 리스너가 호출되고 보통 다음 동작을 이어서 실행할 때 쓴다.

중요한점은 리스너는 해당 Future를 완료시키는 스레드 대부분의 이벤트루프 에서 실행되는 경우가 많다.

따라서 리스너 안에서도 무거운 작업/블로킹콜이 존재해서는 안된다.

### Promise

`Promise<V>`는 Future이면서 동시에 **완료를 내가 시키는** 개겣이다.

`promise.setSuccess(value)` / `promise.setFailure(cause)` 

즉, Netty 내부 사용자 코드가 어떤 비동기 작업의 완료를 표현하기 위해서 사용한다.

왜 필요할까?
- ChannelFuture는 네티의 io 작업 결과를 나타내는 경우가 많고 사용자 정의 비동기 작업을 네티 스타일로 엮는다면 결과를 내가 완료시키는 컨테이너가 필요하다.
- 예시로 외부 스레드풀 작업이 끝나면 이벤트루프로 돌아와서 프로미스를 완료후에 그 다음 네티 파이프라인을 진행하는 것.


코드 예시를 좀 볼건데

channelRead에 들어온 요청을 별도 스레드풀로 오프로드 하는것과

결과를 promise로 완료, 돌아와 이벤트 루프에서 writeAndFlush

ChannelFuture에 GenericFutureListener를 붙여 성공/실패 처리

write 백프레셔를 위한 isWritable() / channelWritabilityChanged도 같이 처리

```java
import io.netty.channel.*;
import io.netty.util.concurrent.*;

import java.nio.charset.StandardCharsets;
import java.util.ArrayDeque;
import java.util.Queue;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public final class PromiseFutureListenerExampleHandler extends SimpleChannelInboundHandler<Object> {

    // 블로킹 작업(외부 API, DB 등)을 오프로드할 풀
    private final ExecutorService blockingPool = Executors.newFixedThreadPool(16);

    // write 백프레셔 시 잠깐 쌓아두는 큐(데모용)
    private final Queue<Object> pendingWrites = new ArrayDeque<>();

    // backpressure 제어 예시: 너무 많이 쌓이면 읽기 끄기
    private static final int MAX_INFLIGHT = 100;
    private int inflight = 0;

    @Override
    public void channelActive(ChannelHandlerContext ctx) {
        // 필요 시 autoRead 제어 가능
        ctx.channel().config().setAutoRead(true);
    }

    @Override
    protected void channelRead0(ChannelHandlerContext ctx, Object msg) {
        // inflight 제한(세션/동시성 제한 같은 개념)
        if (inflight >= MAX_INFLIGHT) {
            // 더 못 받겠으면 읽기 중단
            ctx.channel().config().setAutoRead(false);
            // msg는 버퍼/레퍼런스 관리 필요할 수 있음(여기서는 Object로 단순화)
            return;
        }

        inflight++;

        // 1) Promise 생성: 이 작업의 완료를 내가 setSuccess/setFailure로 완료시킨다.
        Promise<String> promise = new DefaultPromise<>(ctx.executor()); // ctx.executor() == EventLoop

        // 2) Promise 완료 시 동작(리스너)
        promise.addListener((Future<String> f) -> {
            // 이 리스너는 기본적으로 EventLoop에서 실행되도록 설계됨(DefaultPromise가 ctx.executor 기반)
            try {
                if (f.isSuccess()) {
                    String response = f.getNow();

                    // 3) ChannelFuture: write의 비동기 결과
                    ChannelFuture writeFuture;
                    if (ctx.channel().isWritable()) {
                        writeFuture = ctx.writeAndFlush(toByteBuf(ctx, response));
                    } else {
                        // 백프레셔면 큐에 넣고 나중에 flush
                        pendingWrites.add(toByteBuf(ctx, response));
                        return;
                    }

                    // 4) ChannelFuture에 GenericFutureListener 붙이기
                    writeFuture.addListener((ChannelFuture wf) -> {
                        if (wf.isSuccess()) {
                            // write 성공
                        } else {
                            // write 실패 - 원인 확인
                            Throwable cause = wf.cause();
                            wf.channel().close();
                        }
                    });

                } else {
                    // 비동기 작업 실패
                    Throwable cause = f.cause();
                    ctx.writeAndFlush(toByteBuf(ctx, "ERROR: " + cause.getMessage()))
                       .addListener(ChannelFutureListener.CLOSE);
                }
            } finally {
                inflight--;
                // inflight 줄었으니 읽기 재개
                if (!ctx.channel().config().isAutoRead() && inflight < MAX_INFLIGHT) {
                    ctx.channel().config().setAutoRead(true);
                    // autoRead를 켠 뒤 필요하면 read 트리거
                    ctx.read();
                }
            }
        });

        // 5) 실제 블로킹 작업(예: 외부 API 호출)을 별도 풀에서 실행
        blockingPool.execute(() -> {
            try {
                // (중요) 여기선 EventLoop 스레드가 아님. 절대 ctx.write 같은 Netty 작업 직접 호출하지 말기.
                String result = doBlockingCall(msg);

                // 6) Promise 완료는 EventLoop에서 수행되도록 넘기는 게 안전(스레드 안전/일관성)
                ctx.executor().execute(() -> promise.setSuccess(result));

            } catch (Throwable t) {
                ctx.executor().execute(() -> promise.setFailure(t));
            }
        });
    }

    @Override
    public void channelWritabilityChanged(ChannelHandlerContext ctx) {
        // 채널이 다시 writable이 되면 pendingWrites를 비운다.
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
        // 데모: 여기서 외부 API/DB 호출 등 블로킹을 한다고 가정
        // Thread.sleep(50);
        return "OK";
    }
}

```
