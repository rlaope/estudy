# Low Level, JVM Async Programming

### 비동기(Async)

비동기는 명령을 실행하고 결과를 기다리지 않고 다른 일을 계속할 수 있는 구조를 의미한다.

주로 디스크, 네트워크 IO 등에서 자주 사용되며 cpu 리소스를 효율적으로 활용할 수 있게 한다.

- 동기(sync): 작업을 순차적으로 수행, 완료 전까지 블로킹
- 비동기(async): 작업을 요청하고 대기하지 않음, 콜백 등으로 응답
- 병렬(Parallel): 여러 작업을 동시에 수행함
- 멀티스레딩: 할 프로세스 내 여러 실행 흐름

### thread, context switching

- OS에서 프로그램은 프로세스 단위로 실행되며, 스레드는 그 내부의 실행 단위다.
- Java의 CompletableFuture, Kotlin의 Coroutine은 내부적으로 스레드를 생성하거나, OS 스레드 (executor)를 사용한다.
- **Context Switching**은 커널 레벨에서 레지스터, 스택, 프로그램 카운터 등을 저장/복원하는 동작으로 시간이 많이 든다.
- 비동기는 컨텍스트 스위칭과 같은 것을 최대한 피하며, cpu가 놀고있는 상황을 최소화시켜 효율적으로 자원을 활용할 수 있다. (스케일링 비용 감소, 적은 스레드로 더 많은 트래픽 처리가 가능함)
- 대신에 blocking call이 async/non-blocking system에 존재한다면 더욱 큰 병목이 발생할 수 있어 주의해야한다.

async의 단점으로는 순차적이지 않기때문에 실행 예측이 불가능하고 디버깅이 어려우며 예외처리도 복잡하다

코루틴/비동기 context 유출/메모리 누수 위협도 존재한다. 테스트 로깅과 추적도 어렵기 때문에

비동기 기술을 제대로 검증하지 않고 사용하면 오히려 처리량이 떨어질 수도 있기때문에 가파른 러닝커브가 존재한다.


### Non Blocking I/O

Blockiing IO는 호출한 작업을 진행하는동안 다른 작업을 실행할 수 없는. 즉 제어권이 넘어간 것을 의미하고 

Non Blocking IO는 제어권이 호출자에게 아직 있어 다른 작업을 진행할 수 있는 특징이다.

- **블로킹 I/O**
	- `read(fd)` 호출 -> 커널이 데이터 올 때 까지 유저 스레드 멈춤 -> cpu 낭비
- **논블로킹 I/O + 이벤트 기반**
	- 유저 스레드는 `read(fd)` 호출 -> 커널에게 데이터가 오면 알려달라고 등록
	- 커널은 데이터가 준비되면, `epoll`, `kqueue`, `IOCP`등을 통해 알림
	- 유저 스레드는 콜백 또는 폴링 구조로 후속 작업을 진행함
> Java NIO, Netty, Kotlin Coroutine 등에서 이 구조를 사용함


- `epoll`은 리눅스 커널에서 제공하는 **I/O 이벤트 감시 메커니즘이다.**
	- `select()`, `poll()`과 달리 수천개의 fd를 O(1) 성능으로 감시가 가능하다.
	- `epoll_create()`로 epoll() 인스턴스를 생성해 `epoll_ctl()`로 관심있는 소켓(fd)를 등록한다. 그리고 `epoll_wait()`로 이벤트가 올 때까지 대기한다 (논블로킹)
```c
int epfd = epoll_create(0);
epoll_ctl(epfd, EPOLL_CTL_ADD, sockfd, &event);
epoll_wait(epfd, events, MAX_EVENTS, timeout);
```

Netty, Undertow, Vert.x 같은 서버는 내부적으로 이 구조를 사용해 싱글 스레드로도 수천 연결을 유지한다.

- `IOCP`는 윈도우 기반의 완전 비동기 I/O 모델로 `WSARecv` `WriteFileEx` 등은 작업으로 등록하고 완료되었을 때 콜백으로 알려준다.
	- `CreateIoCompletionPort()`로 포트를 생성하고 fd를 바인딩한다.
	- I/O 작업을 등록(ReadFileEx)
	- I/O 완료시 커널이 Worker Thread에 알림을 준다
> Java의 Windows에서 `AsynchronouseSocketChannel`은 내부적으로 IOCP를 활용한다.

<br>

### Java, Coroutine의 구현 비교

Java(CompletableFuture)는 ForkJoinPoll(스레드풀) 기반으로 실행단위가 실제 스레드며 스레드 전환 비용이 존재한다. suspend 동작은 스레드 대기로 이루어진다. 그렇기때문에 thread가 blcok되면 성능 저하가 발생한다.

Coroutine은 상태머신 기반으로 유저모드 스레드가 실행단위며 전환 비용은 Stackless coroutine으로 매우 저렴하다. suspend 동작도 함수 상태를 저장후 다음 상태로 점프한다.

그래서 코루틴이 더 경량 실행 단위를 가지고 있어 더 많은 실행단위를 가지고 있고 더욱 읽기 쉽다는 특징도 가지고 있다. sequential 한 코드 문법으로 async non blocking을 구현할 수 있으니 말이다.

그렇지만 호환성 문제가 발생할 수도 있고 CompletableFuture는 코루틴 의존성 없이도 사용이 가능하며 병렬 처리나 단순 비동기 체이닝에는 여전히 강력한 특징도 가지고 있다.

Kotlin을 잘 도입중이라면 코루틴.. 쓰는게 맞는 것 같긴하다.