# NioEventLoop vs EpollEventLoop

Netty에서는 EpollEventLoop라는 클래스를 지원한다.

NioEventLoop, EpollEventLoop가 나누어져있다. Nio는 이벤트 기반으로 돌지 않아서일까?

한번 둘의 차이를 알아보겠다.

### Epoll

epoll은 리눅스 환경에서 다수의 파일 디스크립터(연결된 소켓등)의 io이벤트를 효율적으로 감시하고 처리하기 위한 리눅스 커널 기능이다.

select나 poll의 한계를 극복하기 위해 만들어졌고 고성능을 자랑한다.

NioEventLoop, EpollEventLoop는 대표적으로 **이벤트 모델의 차이(ET vs LT)**를 가지고있다.

Netty epoll transport는 ET(edger trigger)를 활용하고 java nio는 전통적인 LT(Level-Trigger)에 가깝다는 설명이 자주 인용된다;.

ET는 상태 변화 순간 위주로 깨워서 불필요한 wakeup을 줄이는 방향으로 설계되어있다.
- **조건**: 상태 변화(edge)가 있을때만 이벤트가 발생한다
  - 예시로 없던 데이터가 돌아온다거나 0 -> 1 같은 변화 순간
- 즉 변화 순간에만 깨운다
- **결과**: 이벤트가 왔을때 가능한 많이 read until EAGAIN 읽지 않으면, 데이터는 남아있는데 변화가 없어서 다음에 이벤트가 안올수있긴하다.
- **장점**: 불필요한 wakeup이 줄어들어서 고성능에 유리한 경우가 있다.
- **단점**: 구현 실수 (EAGAIN까지 안읽음)하면 데이터가 남는데도 더 이상 read가 안와 멈춘것처럼 보이는 버그가 있을수있다.



> EAGAIN: unix 계열 시스템에서 시스템 호출이 **나중에 다시 시도하라**라는 의미로 반환하는 오류코드, 주로 비동기 논블로킹 소켓 통신시 "데이터가 준비되지 않았거나 자원이부족해 당장은 작업을 완료할 수 없다" 를 나타낸다.

LT는 상태에 만족한다면 주기적으로 깨워 체크한다는 방향으로 설계되어있다.
- **조건**: 버퍼에 읽을 데이터가 남아있는동안 계속 ready 이벤트가 발생한다.
- **의미**: 상태 level이 만족하는 동안 계속 wakeup한다
- **결과**: 한번에 조금만 읽어도 데이터가 남아있으면 다음 루프에서도 다시 이벤트가 온다
- **장점**: 구현이 단순하다 다 못읽어도 다음에 다시 깨워주니 안전하다
- **단점**: 같은 데이터가 남아있으면 반복해서 깨울 수 있어 wakeup/dispatch 오버헤드가 증가할 수 있다.


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

포인트는 이벤트가 한 번만 올수있으니 그때 커널 수신 버퍼를 다 비워야한다.

안비우면 남아있어도 엣지가 다시 안생겨서 이벤트 다음께 안올 수 있다ㅏ.

예를들어 recv buffer에 64kb가 들어왔고 ET로 readable 이벤트가 1번 왔다고 가정했을때

애플리케이션이 8kb만 읽고 리턴하면 아직 56kb의 데이터가 남아있다 상태는 여전히 readable

그런데 et는 readable 상태로 바뀐 순간만 알려주고 readable 상태였으니 추가 edge가 발생하지 않아 이벤트가 더 오지 않을 수 있다.

그래서 애플리케이션은 더 읽을 데이터가 있는데도 read callback이 다시 안불러서 멈춘것처럼 된다.

LT에서는 이 실수를 해도 남아있으면 또 이벤트가 와서 결국 다시 읽게되지만 ET에서는 치명적이다.

```
LT: “남아있으면 또 알려줌”

ET: “새로 생긴 순간만 알려줌 → 그러니 그때 다 비워야 함(EAGAIN까지)”
```

### NioEventLoop

NioEventLoop는 JDK `Selector` 기반으로 즉, Selector.select(...)를 통해 준비된 SelectionKey들을 받아서 채널별 read/write 같은 io이벤트를 처리한다.

- selectStrategy로 지금 select로 블록할지/즉시 돌지를 결정
- select() 수행
- 선택된 key들 처리
- taskQueue 사용자 작업/오프로딩 결과등을 처리

위와같은 흐름으로 동작하고 이 구조 특히 task가 있으면 select를 오래 블록하지 않도록 다시 확인하는 작업이 있다.

**wakeup, selectNowSupplier hasToken()같은 로직으로 io대기와 task 실행을 섞는다.**

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

`selectStategy.calculateStratgy()`로 select() 호출 여부를 결정한다.

busy-wait는 NIO에서는 지원하지 않는다는 주석이 실제로 들어가 있다.

ioRatio로 io vs task 실행 시간을 나눠 쓴느 구조가 NioEventLoop이 있다.

```java
// Netty NioEventLoop.wakeup(...) 실제 동작을 요약한 의사코드
protected void wakeup(boolean inEventLoop) {
  if (!inEventLoop && nextWakeupNanos.getAndSet(AWAKE) != AWAKE) {
    selector.wakeup();  // Selector.select(...)에서 빠져나오게 함
  }
}
```

그리고 이게 차이의 핵심인데 Nio쪽은 JDK Selector의 wakeup을 그대로 사용한다.

### EpollEventLoop

EpollEventLoop는 Linux의 epoll을 JNI로 호출한다. epoll_wait로 이벤트를 받고

wakeup은 **Selector.wakeup() 대신에 eventfd에 write해서 epoll_wait를 깨운다**

- `Native.epollWait(epollFd, events, timeout)`로 이벤트 대기
- 반환된 이벤트 배열을 돌며 채널별 ready 이벤트처리
- taskQueue 처리

EpollEventLoop 소스/레퍼런스 xref포함에 wakeUp, ioRatio, 그리고 epoll wait 기반 구조를 확인이 가능하다

즉 핵심 대기 호출이 Selector.select()가 아니라 Native.epollWait()라는 점이고

Java level SelectionKey Set 대신에 native events 배열을 직접 다루는 쪽으로 간다.

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

(epoll_wait + busy-wait 가능)

epoll쪽은 BUSY_WAIT 케이스를 실제로 처리한다 deadline기반 epollWait가 Native.epollWait로 내려간다.

```java
// Netty EpollEventLoop.wakeup(...) 동작 요약 의사코드
protected void wakeup(boolean inEventLoop) {
  if (!inEventLoop && nextWakeupNanos.getAndSet(AWAKE) != AWAKE) {
    Native.eventFdWrite(eventFd.intValue(), 1L); // epoll_wait(...)를 깨움
  }
}
```

Epoll은 selector.wakeup()이 아니라 eventfd에 write해서 epoll_wait를 깨우는 방식이다.

Epoll의 진짜 syscall 지점은 Native.epollWait0, epollCtlAdd등 JNI에서다.

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

### 어? 그럼 NioEventLoop는 Selector.select 쓰니까 epoll안쓰는거?

아뇨 NioEventLoop도 결국 epoll을 타는데, 그 epoll은 netty가 직접 호출하는 epoll이 아니라

jdk selector 구현이 내부에서 쏘는거다. 

NioEventLoop가 epoll을 호출하는 지점은

```java
int ready = selector.select(timeout)
```

이 Selector.select()가 linux에서 `EPollSelectorImpl`로 구현되어 있으며 내부적으로 epoll을 쓴ㄷ다.

EpollSelectorImpl은 wakeup용 파이프(또는 소켓페어)를 만들고 epoll에 등록ㅎ한다음, doSelect()에서 epoll wait로 이벤트를 받는 구조다

즉 epoll 호출 시점은 jdk 내부쪽인것이다

```java
Netty NioEventLoop
  -> Selector.select()
     -> sun.nio.ch.EPollSelectorImpl.doSelect(...)
        -> EPoll.wait(...) / epoll_wait JNI
```

LT, ET는 원래 epoll의 플래그(EPOLLET)로 결정되는데 Java NIO Selector는 선택된 키 집합 selectedKeys 라는 추상 모델을 제공한다.

리눅스에서 그 내부가 epoll이여도 일반적으로 엣지 트리거를 전제로한 프로그래밍 모델로 노출이 되지 않는다.

즉 애플리케이션 프레임워크 netty nio는 이벤트가 오면 적당히 읽고 남아있으면 다음 select에 또 잡히는 방식으로 레벨 트리거에 가까운 사용경험이 되도록 구현이 되어있다.

ET처럼 EAGAIN까지 반드시 다 비워야 다음 이벤트가 온다는 전제를 하지않는것

Selector.selecT()가 LT를 직접 구현한다라기 보다는 Selector가 노출하는 readiness 모델이 ET(EAGAIN까지 drain필수) 전제가 필수가 아니고 결과적으로 LT처럼 동작하는 사용모델을 제공하는 것이다.