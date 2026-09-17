# Java AIO

Java NIO 패키지에서 제공하는 비동기(Asynchronous) I/O API다.

I/O 작업을 수행할 때 블로킹되지 않으므로 다른 작업을 수행할 수 있다.

> NIO와 달리 CompletionHandler 인터페이스를 사용해 I/O 작업이 완료될 때 까지 알림을 받는다.




### AIO 지원 
- AsynchronousChannel 클래스
- AsynchronousSocketChannel 클래스
- AsynchronousServerSocketChannel 클래스
- AsynchronousFileChannle 클래스
- callback, future 지원


`Thread pool`, `epoll`, `kqueue` 등의 이벤트 알림 `system call`을 사용한다.

I/O가 준비되었을 때, Future 혹은 callback으로 비동기적인 로직이 처리가 가능하다.


### AsynchronousSocketChannel

클라이언트 소켓을 위한 비동기 채널이다.

### AsynchronousServerSocketChannel

서버(TCP) 소켓을 위한 비동기 채널이다.

### AsynchronousFileChannel

파일 읽기 쓰기 조작을 위한 비동기 채널이다.

<br>

## CompletionHandler



Java AIO API에서 비동기 I/O 작업 완료를 처리하는 인터페이스다.

이 인터페이스를 구현하여 I/O 작업이 완료될 때 호출되는 콜백 메서드를 정의할 수 있다.

CompletionHandler는 제네릭 타입들을 가지고 있는데, 그 타입은
1. I/O 작업의 결과 타입
2. I/O 작업 결과를 처리할 때 사용할 객체의 타입

이렇게 두가지다.

이 인터페이스를 구현하면 I/O 작업이 완료될 때 자동으로 호출되는 completed() 메서드와 I/O 작업 중 예외가 발생했을 때 호출되는 failed() 메서드를 정의할 수 있다.

이를 통해 비동기 I/O 작업을 처리할 때 **콜백 방식**으로 처리할 수 있습니다.