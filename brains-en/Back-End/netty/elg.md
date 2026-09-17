# EventLoopGroup

Before diving in, let's take a deep dive into the Bootstrap configuration and binding required to set up and run a Server Netty application.

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

Looking at the Bootstrap configuration, the first thing done is creating EventLoops as shown below.

```java
EventLoopGroup bossGroup = new NioEventLoopGroup(1);
EventLoopGroup workerGroup = new NioEventLoopGroup();
```

> Since we will be looking at it based on Nio, the code above instantiates `NioEventLoopGroup`. An `EventLoopGroup` combines multiple `EventLoop`s.

The `bossGroup` is a group of `EventLoop`s that establish new connections, and the `workerGroup` is a group responsible for reading and writing data within established connections.

### NioEventLoopGroup

First, let's look at `NioEventLoopGroup` creation.

![](https://mark-kim.blog/static/6eb1dc1e9480521981e689f223c3dd3b/cf465/eventloop_1.webp)

It has various constructors, and the properties initialized and assigned through many of them are as follows:

- nThreads: The number of threads to use, or more precisely, the number of `EventLoop`s.
  - Thread 1:1 EventLoop
  - The default value is twice the number of available cores.
- executor: `Executor` configuration to allocate threads for `EventLoop`s within the `EventLoopGroup`.
  - Acts as a thread pool (Thread Provider) that allocates threads for `EventLoop`s.
- selectorProvider: Selector Provider (Nio, epoll, etc.)
- selectorStrategyFactory: Factory that creates a selector strategy.
  - SelectStrategy: A policy that controls the loop operating to perform a select. For example, it can set policies such as delaying or skipping the select if there are events that need immediate processing.
- rejectedExecutionHandler: Handler for tasks that cannot be executed by the `Executor`.

`NioEventLoopGroup` uses the properties above as arguments during creation to instantiate `EventLoop`s.

And many of `NioEventLoop`'s constructors ultimately call `super()`, invoking the constructor of its superclass, `MultithreadEventLoopGroup`.

### MultithreadEventLoopGroup

When `NioEventLoopGroup` is instantiated, it calls the constructor of its superclass, `MultithreadEventLoopGroup`.

Looking at the code below, you can see the default number of threads mentioned earlier.

![](https://mark-kim.blog/static/b3ab725f234ee7588b8a7284e76acffa/cf465/eventloop_2.webp)

The code shows that it's set to 2 * the number of available processors, and it then calls the constructor of its superclass,

`MultithreadEventExecutorGroup`.

![](https://mark-kim.blog/static/56a479287950467952e7cc61ae61b079/cf465/eventloop_3.webp)

Looking at the `MultithreadEventExecutorGroup` constructor, if the executor is not explicitly configured, it uses `DefaultThreadFactory`. Its internal logic reveals that it creates and allocates a new thread every time.

Then, through the `newChild()` method, it creates `EventLoop` instances according to the provided number of threads and stores them in an array.

`newChild()` is an abstract method that provides implementations based on the selector type, as shown below.

![](https://mark-kim.blog/static/f1280a69407a412e6e5f268068577ec7/cf465/eventloop_4.webp)

For Nio, you should look at the `newChild()` implementation of `NioEventLoopGroup`.

![](https://mark-kim.blog/static/ae70feec0250e42862578d222cf44c9c/cf465/eventloop_5.webp)

It creates `EventLoop` instances using the executor, selectorProvider, selectorStrategyFactory, etc., passed as constructor arguments within the `EventLoopGroup`.

### NioEventLoop Creation and Initialization

The various properties received by `NioEventLoopGroup`'s constructor are ultimately for creating an `EventLoop`. And looking at `NioEventLoop`'s constructor, it is as follows.

![](https://mark-kim.blog/static/6c7696570c2dca50a485aeff78d9f764/cf465/eventloop_6.webp)

Looking at the constructor, it initializes the `EventLoop` using various properties passed during `NioEventLoopGroup` instantiation, and the selector is allocated by calling `openSelector()`.

![](https://mark-kim.blog/static/73a740394ac7b2b2ab5b66204e598e42/cf465/eventloop_7.webp)

Calling `openSelector()` returns a new selector according to the provider implementation, as shown above (Nio, epoll, etc.).

In traditional Nio, channels requiring I/O processing were directly registered with the selector, but in Netty, since the event loop has the selector, you just need to register the channel with the event loop.

### SingleThreadEventLoop

Looking at the `NioEventLoop` constructor above, it calls the constructor of its superclass, `SingleThreadEventLoop`.

![](https://mark-kim.blog/static/83e584dff0e96c8e57cb4f9a650c71a8/cf465/eventloop_8.webp)

As can be seen in the code, creation is delegated to the superclass, `SingleThreadEventExecutor`.

Within the `SingleThreadEventExecutor` constructor, a `Thread` is allocated from the `Executor` and a `TaskQueue` is instantiated, as shown below.

![](https://mark-kim.blog/static/089f08049f61741ef36b5d158f848673/cf465/eventloop_9.webp)

As can be seen above, an event loop consists of 1 thread + 1 selector + 1 task queue.

We've examined the process of creating `nioEventLoopGroup` and assigning various properties in the code below:

```java
EventLoopGroup bossGroup = new NioEventLoopGroup(1);
EventLoopGroup workerGroup = new NioEventLoopGroup();
```

The important point is that each `nioEventLoop` has its own `Selector`, thread, and `taskQueue`, and by including identical `EventLoop`s in an `EventLoopGroup`, resources are distributed.

Now, using Bootstrap, the `EventLoopGroup` above is configured by dividing roles into boss and child.

```java
EventLoopGroup bossGroup = new NioEventLoopGroup(1);
EventLoopGroup workerGroup = new NioEventLoopGroup();

ServerBootstrap b = new ServerBootstrap();
b.group(bossGroup, workerGroup)
        .channel(NioServerSocketChannel.class)
        .childHandler(...);
```

And at this point, the channel, event loops, and handlers to be used are set as `handler` and `channelHandler`, respectively.
