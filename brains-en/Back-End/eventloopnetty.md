# Netty Thread Model, EventLoop

Netty's core philosophy is that a single thread handles everything.

The traditional thread-per-request model created one thread per request.

However, Netty's EventLoop, a single thread, manages hundreds or thousands of channels (connections) by alternating between them.

This is considered an implementation of the **Reactor Pattern**.

### Hierarchy

- `EventLoopGroup`: Similar to a thread pool. NioEventLoopGroup is an example, containing multiple EventLoops.
- `EventLoop`: This is the single thread that actually performs work.
- `Channel(Socket)`: This is the connection with the client.
  - Once a channel is created, it is bound to a specific EventLoop for its entire lifetime.
  - All events occurring on this channel (data reception, exception occurrence, etc.) must be handled exclusively by that single thread.

### Internal Working Principle

The EventLoop thread continuously performs the following three tasks in an infinite loop until it terminates.

1.  **Select**: It checks the OS Selector (Multiplexer) to see if anyone has sent data.
2.  **Process I/O**: If there's a channel with incoming data, it reads or writes data. (Netty internal processing)
3.  **Run Tasks**: It processes general tasks accumulated in the queue.

Code submitted via `ctx.executor().execute(() -> ...)` is thrown into the queue that `Run Tasks` executes.

If a call accumulated in this internal queue is a synchronous call that blocks the thread, the loop stops, causing other connections to become unresponsive.

### Task Queue and Thread Confinement

Let's delve a bit deeper into why heavy tasks like synchronous calls should not be performed within a Netty thread.

#### MPSC Queue(Multi-Producer Single-Consumer)

When an external thread (e.g., from a blocking pool) or another EventLoop calls `ctx.write()`, it's not sent immediately.

Instead, a runnable task object is added to the task queue of the EventLoop responsible for that channel.

Let's look at the EventLoop's pseudocode.

```java
// NioEventLoop.run() 의사 코드 (핵심 로직)
protected void run() {
    for (;;) {
        // 1. Selector를 통해 I/O 이벤트 감지 (Blocking 가능)
        strategy = select(curDeadlineNanos);

        // 2. ioRatio(기본 50)에 따른 시간 분배
        final int ioRatio = this.ioRatio;
        
        if (ioRatio == 100) {
            // I/O 100% 모드: I/O 다 처리하고, Task도 다 처리함 (위험)
            processSelectedKeys();
            runAllTasks(); 
        } else {
            // 비율 모드 (기본): I/O 처리 시간을 측정해서 그만큼만 Task도 처리
            final long ioStartTime = System.nanoTime();
            
            processSelectedKeys(); // [I/O 처리] (패킷 읽기/쓰기)

            final long ioTime = System.nanoTime() - ioStartTime;
            
            // [Task 처리] I/O에 쓴 시간 * 비율만큼만 실행 (Time Slice)
            runAllTasks(ioTime * (100 - ioRatio) / ioRatio); 
        }
    }
```

As shown above, this queue is only checked when the `runAllTasks` stage is reached. If a preceding task consumes all of the `ioRatio` (which we'll discuss below) or blocks, the `write` operation in the queue will wait indefinitely.

### Advantages of Netty Thread Model

Let's explore the advantages of this thread model.

-   Minimized Context Switching: The thread doesn't need to repeatedly acquire and release the CPU. It continuously processes tasks.
-   Lock-Free (No Synchronization Needed): Since only one thread ever traverses a channel's pipeline, there's no need to apply `synchronized` to variables.

### Tuning

It's good to understand `ioRatio`. The EventLoop allocates time between I/O processing and general task processing.

The default value is 50, distributing 50% to I/O and 50% to tasks.

For servers with very high I/O, you can optimize throughput by adjusting this ratio (e.g., I/O 70).

```java
// NiceProxyServer.java 내 수정 예시

public void run() throws Exception {
    // ... 그룹 생성 ...
    workerGroup = new NioEventLoopGroup(workerEventLoopThreads);
    
    // [튜닝] 모든 Worker EventLoop의 ioRatio를 70으로 변경
    // (I/O 작업 시간의 약 43% 정도를 Task 처리에 더 씀)
    ((NioEventLoopGroup) workerGroup).setIoRatio(70);

    // ... 부트스트랩 설정 ...
}
```

### OS-level Optimization: JNI-based Transport (epoll)

To achieve high performance in a Linux environment, you should directly use the Native Transport `epoll` instead of `NioEventLoop`.

JDK NIO uses a level-triggered mechanism, which has some inefficiencies.

In Linux production environments, `EpollServerSocketChannel` is used.

-   Edge-triggered: Events are only triggered when a state change occurs, reducing `select` overhead.
-   Reduced GC: Being native C code, it also reduces Java heap memory usage.

```java
// 의존성: netty-transport-native-epoll 추가 필요

EventLoopGroup workerGroup;
Class<? extends ServerSocketChannel> channelClass;

if (Epoll.isAvailable()) {
    // 리눅스 프로덕션 환경
    workerGroup = new EpollEventLoopGroup(threads);
    channelClass = EpollServerSocketChannel.class;
    ((EpollEventLoopGroup) workerGroup).setIoRatio(70); // Epoll도 ratio 설정 가능
} else {
    // 로컬 맥/윈도우 개발 환경
    workerGroup = new NioEventLoopGroup(threads);
    channelClass = NioServerSocketChannel.class;
}

bootstrap.group(...)
         .channel(channelClass) // 동적으로 클래스 할당
         // ...
```

> Level-triggered mode continuously generates events as long as a specific signal (level) is reached.
>
> This term is used in digital logic circuits and OS interrupt handling. It's like a sensor set to trigger an alarm if water rises above 50%, reacting as long as that state is maintained.
>
> The epoll model uses an edge-triggered approach, which means it only triggers an alarm at the moment the water level *crosses* 50%, and then doesn't react afterward.
>
> It differs based on whether the state is maintained or not. That's why epoll offers higher performance, as it doesn't trigger interrupts even if the buffer isn't emptied again.
