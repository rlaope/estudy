# NioEventLoop vs EpollEventLoop

Netty supports a class called EpollEventLoop.

NioEventLoop and EpollEventLoop are separated. Is it because Nio doesn't operate on an event basis?

Let's explore the differences between the two.

### Epoll

epoll is a Linux kernel feature designed to efficiently monitor and handle I/O events for multiple file descriptors (such as connected sockets) in a Linux environment.

It was created to overcome the limitations of select and poll, boasting high performance.

NioEventLoop and EpollEventLoop primarily differ in their **event models (ET vs LT)**.

It's often cited that Netty's epoll transport utilizes ET (edge-trigger), while Java NIO is closer to traditional LT (Level-Trigger).

ET is designed to wake up primarily at the moment of state change, thereby reducing unnecessary wakeups.
- **Condition**: Events are triggered only when there's a state change (edge).
  - For example, when data that wasn't there arrives, or a change from 0 to 1.
- In other words, it only wakes up at the moment of change.
- **Result**: If you don't read as much as possible until EAGAIN when an event arrives, data might remain, but since there's no further change, the next event might not come.
- **Advantage**: Reduced unnecessary wakeups can be beneficial for high performance in some cases.
- **Disadvantage**: If there's an implementation error (not reading until EAGAIN), a bug might occur where the application appears to be stuck because no more read events arrive, even though data remains.

> EAGAIN: An error code returned by system calls in Unix-like systems, meaning **try again later**. It typically indicates "data is not ready or resources are insufficient to complete the operation immediately" during asynchronous non-blocking socket communication.

LT is designed to periodically wake up and check if the condition is met.
- **Condition**: Ready events continue to be triggered as long as there is data remaining in the buffer to be read.
- **Meaning**: It continuously wakes up as long as the state level is satisfied.
- **Result**: Even if only a small amount of data is read at once, if data remains, an event will be triggered again in the next loop.
- **Advantage**: Implementation is simple; it's safe because even if not all data is read, it will be woken up again later.
- **Disadvantage**: If the same data remains, it can be woken up repeatedly, potentially increasing wakeup/dispatch overhead.

```
# LT 흐름도
[epoll_wait/selector]  (readable 이벤트)
          |
          v
[onReadable()]
          |
          v
read() 1번(또는 조금만)
          |
          v
데이터 남아있음?
   | yes                    | no
   v                        v
(다음 loop에서도)          종료
readable 이벤트 또 옴

# ET 흐름도
[epoll_wait]  (readable 이벤트: edge 발생)
          |
          v
[onReadable()]
          |
          v
while (true):
    n = read()
    if n > 0: 계속 누적 처리
    if n == 0: (상대 close) 종료 처리
    if n < 0 && errno == EAGAIN: break  <-- 여기까지 꼭 와야 함
          |
          v
종료(EAGAIN 도달)

```

The point is that since an event might come only once, the kernel receive buffer must be completely drained at that time.

If it's not drained, even if data remains, a new edge won't be generated, and subsequent events might not arrive.

For example, let's assume 64KB arrived in the recv buffer and a readable event was triggered once by ET.

If the application reads only 8KB and returns, 56KB of data still remains, and the state is still readable.

However, ET only notifies at the moment the state changes to readable. Since it was already in a readable state, no additional edge will be generated, and no further events might arrive.

Consequently, the application appears to be stuck because the read callback isn't invoked again, even though there's more data to read.

In LT, even if this mistake is made, if data remains, another event will arrive, eventually leading to it being read again. But in ET, it's critical.

```
LT: “If data remains, it notifies again.”

ET: “It only notifies at the moment a new event occurs → so you must drain everything at that time (until EAGAIN).”
```

### NioEventLoop

NioEventLoop is based on the JDK `Selector`, meaning it receives ready `SelectionKey`s via `Selector.select(...)` and processes I/O events like read/write for each channel.

- Determines whether to block with `select` now or run immediately using `selectStrategy`.
- Performs `select()`.
- Processes selected keys.
- Handles user tasks/offloading results from the `taskQueue`.

It operates with the flow described above, and this structure, especially when there are tasks, includes a mechanism to re-check so that `select` doesn't block for too long.

**It mixes I/O waiting and task execution with logic like `wakeup`, `selectNowSupplier`, and `hasToken()`.**

```java
// Netty NioEventLoop.run() 흐름을 축약한 의사코드
protected void run() {
  for (;;) {
    int strategy = selectStrategy.calculateStrategy(selectNowSupplier, hasTasks());

    switch (strategy) {
      case CONTINUE:
        continue;

      case BUSY_WAIT:
        // NIO에서는 busy-wait 미지원 -> SELECT로 fall-through
      case SELECT:
        long deadline = nextScheduledTaskDeadlineNanos();
        nextWakeupNanos.set(deadline == -1 ? NONE : deadline);

        try {
          if (!hasTasks()) {
            strategy = select(deadline);   // 내부적으로 Selector.select(...) 계열 호출
          }
        } finally {
          nextWakeupNanos.lazySet(AWAKE);
        }
        // fall-through
      default:
        // strategy 값(ready key 수 등)을 바탕으로 selectedKeys 처리 + task 실행을 균형 있게 수행
        // ioRatio(기본 50)로 I/O 처리 시간 vs 일반 task 시간 배분
        if (ioRatio == 100) {
          processSelectedKeys();     // SelectionKey 기반 I/O dispatch
          runAllTasks();             // taskQueue 비우기
        } else {
          long ioStart = now();
          processSelectedKeys();
          long ioTime = now() - ioStart;
          runAllTasks( calcTaskBudget(ioTime, ioRatio) );
        }
    }
    if (isShuttingDown()) { closeAll(); break; }
  }
}
```

The `selectStrategy.calculateStrategy()` determines whether to call `select()`.

There's actually a comment stating that busy-wait is not supported in NIO.

NioEventLoop has a structure that divides I/O vs. task execution time using `ioRatio`.

```java
// Netty NioEventLoop.wakeup(...) 실제 동작을 요약한 의사코드
protected void wakeup(boolean inEventLoop) {
  if (!inEventLoop && nextWakeupNanos.getAndSet(AWAKE) != AWAKE) {
    selector.wakeup();  // Selector.select(...)에서 빠져나오게 함
  }
}
```

And this is the core difference: the Nio side directly uses the JDK Selector's `wakeup`.

### EpollEventLoop

EpollEventLoop calls Linux's epoll via JNI. It receives events using `epoll_wait`.

For wakeup, it **writes to `eventfd` instead of `Selector.wakeup()` to wake up `epoll_wait`**.
- Waits for events using `Native.epollWait(epollFd, events, timeout)`.
- Iterates through the returned event array to process ready events for each channel.
- Processes the `taskQueue`.

You can verify the `wakeUp`, `ioRatio`, and epoll wait-based structure in the EpollEventLoop source/reference, including xrefs.

That is, the core waiting call is `Native.epollWait()` rather than `Selector.select()`.

It directly handles native event arrays instead of Java-level `SelectionKey` sets.

```java
// Netty EpollEventLoop.run() 흐름을 축약한 의사코드
protected void run() {
  for (;;) {
    int strategy = selectStrategy.calculateStrategy(selectNowSupplier, hasTasks());

    switch (strategy) {
      case CONTINUE:
        continue;

      case BUSY_WAIT:
        strategy = epollBusyWait();      // Native.epollBusyWait(...) 사용
        break;

      case SELECT:
        if (pendingWakeup) {
          // wakeup이 곧 올 상황이면 timeboxed로 안전장치 (1초) 걸고 기다림
          strategy = epollWaitTimeboxed();
          if (strategy != 0) break;
          logWarnMissedEventfdWrite();
        }
        // deadline 기반으로 timerfd + epoll_wait 조합 호출
        strategy = epollWait(deadlineNanos);
        break;

      default:
        // ready 이벤트들을 돌면서 채널별 epollInReady/epollOutReady 등 dispatch
        // 이후 taskQueue 실행을 ioRatio 기준으로 배분
        processReadyEvents();
        runTasksWithIoRatio();
    }

    if (isShuttingDown()) { closeAll(); break; }
  }
}
```

(epoll_wait + busy-wait possible)

The epoll side actually handles the `BUSY_WAIT` case, and deadline-based `epollWait` delegates to `Native.epollWait`.

```java
// Netty EpollEventLoop.wakeup(...) 동작 요약 의사코드
protected void wakeup(boolean inEventLoop) {
  if (!inEventLoop && nextWakeupNanos.getAndSet(AWAKE) != AWAKE) {
    Native.eventFdWrite(eventFd.intValue(), 1L); // epoll_wait(...)를 깨움
  }
}
```

Epoll uses the method of writing to `eventfd` to wake up `epoll_wait`, rather than `selector.wakeup()`.

The actual syscall points for Epoll are in JNI, such as `Native.epollWait0`, `epollCtlAdd`, etc.

```java
// Netty io.netty.channel.epoll.Native 핵심 JNI 지점 (의미만 보존한 축약)
static long epollWait(FileDescriptor epollFd, EpollEventArray events, FileDescriptor timerFd,
                      int timeoutSec, int timeoutNs, long millisThreshold) throws IOException {
  long result = epollWait0(efd, eventsAddr, eventsLen, timerFdInt, timeoutSec, timeoutNs, millisThreshold);
  int ready = epollReady(result);
  if (ready < 0) throw newIOException("epoll_wait", ready);
  return result;
}

// JNI entrypoint
private static native long epollWait0(int efd, long address, int len, int timerFd,
                                      int timeoutSec, int timeoutNs, long millisThreshold);

// fd를 epoll interest set에 등록/수정/삭제
public static void epollCtlAdd(int efd, int fd, int flags) throws IOException { ... }
public static void epollCtlMod(int efd, int fd, int flags) throws IOException { ... }
public static void epollCtlDel(int efd, int fd) throws IOException { ... }

```

### Oh? So NioEventLoop uses Selector.select, meaning it doesn't use epoll?

No, NioEventLoop also ultimately uses epoll, but that epoll is not directly called by Netty.

It's the JDK selector implementation that invokes it internally.

The point where NioEventLoop calls epoll is:

```java
int ready = selector.select(timeout)
```

This `Selector.select()` is implemented as `EPollSelectorImpl` in Linux and internally uses epoll.

EpollSelectorImpl creates a pipe (or socket pair) for wakeup, registers it with epoll, and then receives events via `epoll_wait` in `doSelect()`.

In other words, the epoll call occurs internally within the JDK.

```java
Netty NioEventLoop
  -> Selector.select()
     -> sun.nio.ch.EPollSelectorImpl.doSelect(...)
        -> EPoll.wait(...) / epoll_wait JNI
```

LT and ET are originally determined by epoll flags (EPOLLET), but Java NIO Selector provides an abstract model called `selectedKeys` (a set of selected keys).

Even if its internal implementation uses epoll in Linux, it's generally not exposed as a programming model that assumes edge-triggering.

That is, the application framework Netty NIO is implemented to provide a user experience closer to level-triggering, where it reads a reasonable amount when an event arrives, and if data remains, it gets caught again in the next `select`.

It does not assume that all data must be drained until EAGAIN for the next event to arrive, as in ET.

Rather than `Selector.select()` directly implementing LT, the readiness model exposed by `Selector` does not mandate the ET (drain until EAGAIN is required) premise, and consequently provides a usage model that behaves like LT.
