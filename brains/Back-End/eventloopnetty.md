# Netty Thread Model, EventLoop

Netty의 핵심 철학은 single thread가 모든것을 처리한다 이다.

기존의 전통적인 thread per request은 요청 하나당 스레드 하나를 만들었다.

하지만 netty는 이벤트루프라는 스레드 하나가 수백, 수천개의 채널(connection)을 혼자서 번갈아가며 관리한다.

이를 **Reactor Pattern**의 구현체라고 한다.

### Hierarchy

- `EventLoopGroup`: 스레드풀과 비슷하다 NioEventLoopGroup이 그 예시인데 그 안에 여러개에 EventLoop가 있는거임
- `EventLoop`: 실제 일을 하는 single thread 이다.
- `Channel(Socket)`: 클라이언트와의 연결이다.
  - 채널이 생성되면, 특정 event loop 하나에 평생 binding 된다.
  - 이 채널에서 발생하는 모든 이벤트(데이터수신, 예외발생 등)는 반드시 그 스레드 혼자서만 처리한다.

### 내부 동작 원리

EventLoop 스레드는 죽을때까지 아래 3가지일을 빙글빙글 무한루프하며 한다.

1. **Select**: 누구 데이터 보낸 사람이 있는가? 하고 os Selector(Multiplexer)를 쳐다본다.
2. **Process I/O**: 데이터가 온 채널이 있으면 데이터를 읽거나 쓴다. (Netty 내부 처리)
3. **Run Tasks**: 큐에 쌓인 일반 작업들을 처리한다.

`ctx.executor().execute(() -> ...)` 이렇게 던진 코드가 Run Tasks에서 실행시키는 그 큐에 던지는거다.

그리고 이 내부 큐에 쌓인 콜이 동기콜로 해당 thread를 block하면 루프가 멈추기 때문에 다른 연결들이 먹통이 되는것이다.

### Task Queue와 Thread Confinement

동기 콜같은 무거운 작업을 netty 스레드 안에서 하면 안되는 이유를 조금 구체적으로 더 파보겠다.

#### MPSC Queue(Multi-Producer Single-Consumer)

외부 스레드 (blocking pool)이나 다른 event loop가 `ctx.write()`를 호출하면, 바로 전송되는 것이 아니다.

해당 channel을 담당하는 eventLoop의 task queue에 작업 객체 runnable로 들어간다.

EventLoop의 의사코드를 살펴보겠다.

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

위처럼 runAllTasks 단계에 도달해야 이 큐를 확인하고,= 만약 앞선 작업이 ioRatio(아래에서 알아봄)을 다 써버렸거나 블로킹되면 큐에 들어간 write는 영원히 대기한다.


### Netty Thread Model 장점

해당 스레드 모델의 장점을 알아보면

- Context Switching 최소화: 스레드가 cpu를 잡았나 놨다 할 필요가 없다. 계속 돌면서 일을 처리하니까.
- Lock Free(동기화 불필요): 한 채널의 파이프라인은 무조건 한 스레드만 지나가니까 변수에 synchronized를 걸 필요가 없다.

### Tuning

`ioRatio`를 알면 좋은데. event loop는 io처리와 일반 task 처리 사이에서 시간 배분을 한다.

기본값은 50이고 io 50 task 50으로 분배한다.

io가 엄청 많은 서버라면 이 비율을 조정해서(io 70 등) 처리량 최적화도 가능하다.

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

### OS 레벨 기반 최적화 JNI 기반 Transport(epoll)

리눅스 환경에서 고성능 내려면 `NioEventLoop`대신 Native Transport `epoll`을 직접 써야한다.

jdk nio는 레벨 트리거 방식이라 비효율이 존재한다.

리눅스 production에서는 `EpollServerSocketChannel`을 사용한다.

- Edge-triggered: 상태 변화가 있을때만 이벤트를 발생시켜 select 오버헤드 감소
- gc 감소: native c코드라서 자바 힙메모리 사용량 감소까지 됨

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

> 레벨 트리거 방식은 특정 신호(level) 기준으로 도달해있는동안 계속해서 이벤트를 발생시키는 방식
>
> 디지털 논리회로와 os 인터럽트 처리에 쓰이는 용어다. 센서 설정 물 50퍼이상 차올랐으면 경보같은 state가 유지되면 반응하는것.
>
> epoll model을 쓰는게 이제 edge trigger 방식인건데, 물이 50퍼센트를 넘는 그 순간 변화에만 경보를 울려라. 그 뒤로는 반응 ㄴ
>
> 유지하냐 마냐에 따라 달라짐. 그래서 epoll이 더 고성능인거. 버퍼를 다시 비우지 않아도 인터럽트를 안걸어서

