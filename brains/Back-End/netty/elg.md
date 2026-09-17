# EventLoopGroup

들어가기 앞서 Server Netty 애플리케이션을 설정하고 실행하는데 필요한 아래 Bootstrap 설정과 바인딩에 관하여 딥다이브 해보겠다.

```java
public static void main(String[] args) throws InterruptedException {
    EventLoopGroup bossGroup = new NioEventLoopGroup(1);
    EventLoopGroup workerGroup = new NioEventLoopGroup();

    try {
        ServerBootstrap b = new ServerBootstrap();
        b.group(bossGroup, workerGroup)
                .channel(NioServerSocketChannel.class)
                .childHandler(new NettyServerInitializer());

        // 서버 시작
        log.info("start server...");
        ChannelFuture f = b.bind(8080).sync();
        f.addListener((ChannelFuture future) -> {
            if (future.isSuccess()) {
                log.info("Server bound");
            } else {
                log.error("Bind attempt failed", future.cause());
            }
        });

        f.channel().closeFuture().sync();
    } finally {
        log.info("close server...");
        workerGroup.shutdownGracefully();
        bossGroup.shutdownGracefully();
    }
}
```

Bootstrap 설정을 보면 가장 먼저 하는것이 아래와 같이 EventLoop를 생성하는 것이다

```java
EventLoopGroup bossGroup = new NioEventLoopGroup(1);
EventLoopGroup workerGroup = new NioEventLoopGroup();
```

> Nio 기준으로 살펴볼것이기 때문에 위 코드는 NioEventLoopGroup을 인스턴스화한 코드다. EventLoopGroup은 여러개의 EventLoop를 조합한다.

bossGroup은 새로운 연결을 수립하는 EventLoop들을 모아둔 그룹이고, worker group은 연결된 커넥션내 데이터를 read write하는 역할의 그룹이다.


### NioEventLoopGroup

우선 NioEventLoopGroup 생성부터 살펴보겠다.

![](https://mark-kim.blog/static/6eb1dc1e9480521981e689f223c3dd3b/cf465/eventloop_1.webp)

다양한 생성자들을 가지고있고 그리고 많은 생성자를 통해 초기화 및 할당하는 속성은 아래오 ㅏ같다.

- nThreads: 사용할 스레드의 개수고 더 자세히 말하면 EventLoop의 개수
  - Thread 1:1 EventLoop
  - 기본값은 사용가능한 커널 수의 두배
- executor: EventLoopGroup내 EventLoop들이 사용할 Thread를 할당해줄 Executor 설정
  - EventLoop가 사용할 스레드를 할당해주는 스레드풀 역할(Thread Provider)
- selectorProvider: Selector Provider(Nio, epoll 등등)
- selectorStrategyFactory: selector 전략을 생성하는 Factory
  - SelectStrategy: select를 하기위해 동작하는 루프를 제어하는 정책이다 예를 들어 즉시 처리해야할 이벤트가 있는경우 select를 지연시키거나 skip하는등의 정책 설정이 가능하다.
- rejectedExecutionHandler: Executor에 의해 실행할 수 없는 작업에대한 Handler

NioEventLoopGroup은 위 속성들을 생성할 때 인자로 받아서 EventLoop를 생성할 때 사용한다.

그리고 NioEventLoop의 많은 생성자를 결국 super()를 호출함으로써 상위 클래스인 `MultithreadEventLoopGroup` 내 생성자를 호출한다.

### MultithreadEventLoopGroup

NioEventLoopGroup를 인스턴스화하면 상위 클래스인 MultithreadEventLoopGroup의 생성자를 호출하는데,

아래 코드를 보면 위에서 언급한 default thread 개수를 볼 수 있다.

![](https://mark-kim.blog/static/b3ab725f234ee7588b8a7284e76acffa/cf465/eventloop_2.webp)

2 * 사용 가능한 프로세서수로 되어있는것을 코드를 통해 알수있으며 그대로다시 상위클래스인

MultithreadEventExecutorGroup 생성자를 호출한다.

![](https://mark-kim.blog/static/56a479287950467952e7cc61ae61b079/cf465/eventloop_3.webp)

`MultithreadEventExecutorGroup` 생성자를 보면 우선 executor 설정을 따로 하지 않으면 DefaultThreadFactory를 사용하는데 내부 로직을 보니 매번 새로운 thread를 만들어 할당해준다.

그리곤 newChild() 메서드를 통해 제공된 thread 수에 따라 eventloop 인스턴스를 생성하고 배열에 저장한다.

newChild()는 추상메서드로 아래처럼 selector 형식에 따라 구현체를 제공한다.

![](https://mark-kim.blog/static/f1280a69407a412e6e5f268068577ec7/cf465/eventloop_4.webp)

nio기준으로는 NioEventLoopGroup 구현체의 newChild()를 보면된다.

![](https://mark-kim.blog/static/ae70feec0250e42862578d222cf44c9c/cf465/eventloop_5.webp)

앞서 EventLoopGroup내 생성자 인수로 넘어온 executor, selectorProvider, selectorStrategyFactory등을 활용해 EventLoop 인스턴스를 생성한다.

### NioEventLoop 생성 및 초기화

NioEventLoopGroup의 생성자로 받은 여러 속성들은 결국 EventLoop를 생성하기 위함이다. 그리고 NioEventLoop의 생성자를 보면 아래와 같다.

![](https://mark-kim.blog/static/6c7696570c2dca50a485aeff78d9f764/cf465/eventloop_6.webp)

생성자를 보면 NioEventLoopGroup에서 인스턴스화할때넘어오는 여러 속성들을 통해 EventLoop를 초기화하고 selector는 `openSelector()`를 호출함으로써 할당한다.

![](https://mark-kim.blog/static/73a740394ac7b2b2ab5b66204e598e42/cf465/eventloop_7.webp)

openSelector()를 호출하면 위처럼 provider 구현체에 따라 새로운 selector를 반환한다 nio epoll 등등

기존 nio에선 io처리가 필요한 channel을 직접 selector에 등록했는데 네티에서는 이벤트루프가 selector를 가지고있으므로 channel을 eventloop에 등록해주면 끝이다.

### SingleThreadEventLoop

위에서 NioEventLoop 생성자를 보면 상위 클래스인 SingleThreadEventLoop의 생성자를 호출한다.

![](https://mark-kim.blog/static/83e584dff0e96c8e57cb4f9a650c71a8/cf465/eventloop_8.webp)

코드에서 볼 수 있듯이, 생성을 상위 클래스인 SingleThreadEventExecutor에 위임한다.

SingleThreadEventExecutor내 생성자에선 아래같이 Executor로부터 Thread를 할당하고 TaskQueue를 인스턴스화 한다.

![](https://mark-kim.blog/static/089f08049f61741ef36b5d158f848673/cf465/eventloop_9.webp)

위에서 알수있듯 eventloop는 1 thread + 1 selector + 1 task queue로 구성되어 있다.

아래 코드에서 nioEventLoopGroup을 생성하고 여러 속성을 할당하는 과정을 알아봤는데

```java
EventLoopGroup bossGroup = new NioEventLoopGroup(1);
EventLoopGroup workerGroup = new NioEventLoopGroup();
```

중요한 점은 nioEventLoop는 자체적인 Selector, thread 그리고 taskQueue를 가지고 있으며 동일한 EventLoop를 EventLoopGroup에 포함함으로써 리소스를 분산시킨다는것이다.

이제 위 EventLoopGroup을 Bootstrap을 사용해 boss랑 child로 역할을 분담해 설정한다.

```java
EventLoopGroup bossGroup = new NioEventLoopGroup(1);
EventLoopGroup workerGroup = new NioEventLoopGroup();

ServerBootstrap b = new ServerBootstrap();
b.group(bossGroup, workerGroup)
        .channel(NioServerSocketChannel.class)
        .childHandler(...);
```

그리고 이때 사용될 channel, event loop들 handler를 각각 handler, channelHandler로 설정한다.

