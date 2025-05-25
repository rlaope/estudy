# epoll

`epoll`은 리눅스의 고성능 이벤트 기반 I/O 감시 시스템이다.

네트워크 서버나 고속 파일 처리기에서 논블로킹 i/o를 효율적으로 감시하고 처리하기 위해 반드시 이해해야할 핵심 기술이다.

**기존 I/O 이벤트 감시 방식에는 대표적으로 다음이 있다.**
- `select()`: 모든 fd를 배열로 넘기고 커널이 순회, fd수 1024 제한, 순차 검색 O(n)
- `poll()`: `struct pollfd[]` 배열로 전달, 동적 크기 허용 그러나 여전히 순차검색 O(n)
- `epoll`: 를 커널에 등록하고 변경사항만 감지, 이벤트 기반 O(1), 스케일에 강하다

epoll은 수천, 수만개의 파일 디스크립터를 감시할 수 있는 방식의 이벤트 감지를 가능하게 하며 고성능 네트워크 서버 사실상 표준이다.

epoll은 커널 내부에 감시할 fd를 저장하는 **별도의 자료구조**를 생성해 놓는다.

앱은 이 자료구조에 fd를 등록만 시켜놓고, 변경이 생길 때만 알림을 받는다 (이벤트 기반)

흐름
1. `epoll_create()` -> epoll 인스턴스 생성 (fd 반환) - 객체 생성
2. `epoll_ctl()` -> 감시할 파일 디스크립터 등록 (EPOLLIN, EPOLLOUT, EPOLLET, etc) - 등록/수정/삭제
3. `epoll_wati()` -> 이벤트가 발생할 때까지 블록/리턴됨 - 이벤트 감시

**epoll의 핵심 모드**
- LT(Level Triggered): 기본 모드, 이벤트가 계속 유지되면 매번 감지된다.
- ET(Edge Triggered): 변화가 생긴 순간 1번만 알림, 고속 서버에 사용, 버퍼 다 읽어야함
> ET 모드는 논블로킹 소켓과 함께 사용해야하며, while 루프로 완전히 읽혀야한다.

### epoll LT vs ET 왜 while루프가 필요한가

- ET는 변화가 생긴 순간 1번만 알림을 하기 때문에 버퍼가 비워질때까지 읽지 않으면 (while 루프가 없으면) 데이터 손실이 발생할 수 있다.

예를 들어 클라이언트가 한 번에 8kb이상 데이터를 보내고 서버가 1kb씩 읽을때 ET모드에서는 while루프가 없으면 데이터가 손실된다.

**확인 포인트**
- `read()`가 한 번만 호출되면 데이터 일부만 도착하고 나머지는 알림이 안온다. (ET 모드)
- `while(read(fd, buf, sizeof(buf)) > 0)` 루프가 필요하다

```c
if (event.events & EPOLLIN) {
    while ((n = read(fd, buf, sizeof(buf))) > 0) {
        // 처리
    }
    if (n == 0 || (n < 0 && errno != EAGAIN)) {
        close(fd); // 종료
    }
}
```

### epoll + nonblocking socket - O_NONBLOCK

epoll은 비동기 방식이므로 블로킹 소켓을 사용하면 epoll_wait() 외에 read/write가 blcoking이 걸려 성능이 저하된다.
- socket() -> accept() -> read()에서 O_NONBLOCK 없이 사용하게 된다면
	- `read()`가 대기 상태에서 블로킹 되어 전체 서버가 멈춤

### epoll 내부구조 - 리눅스 커널 `/proc` 트리

epoll은 커널 내부에서 별도의 자료구조로 fd를 관리한다고 했었다. 어떤 자료구조로 동작하는지 개념을 익혀보겠다.

`/proc` 트리에서 epoll의 존재를 확인하고 소스코드 레벨의 흐름을 봐보자

`/proc/PID/fdinfo/N`을 통해 epoll FD의 내부 상태를 확인해보자

```bash
ls -l /proc/$(pgrep your_server_bin)/fd
cat /proc/$(pgrep your_server_bin)/fdinfo/N
```

epoll은 커널 내에 다음과 같은 구조로 구현되어 있다.

```c
struct eventpoll {
	spinlock_t lock; 
	struct rb_root rbr; // fd 관리 레드블랙 트리
	struct list_head rdlist; // 이벤트가 발생한 ready list
	wait_queue_head_t wq; // epoll wait에서 대기할 때 사용하는 큐
	struct file *file;  // epoll fd 자체
}
```
- 이벤트 감시용 fd는 `epoll_create()`로 생성되고
- `epoll_ctl()`로 트리에 등록이 되며
- 이벤트 발생시 rdlist에 추가가 되며
- `epoll_wait()`호출시 rdlist에서 꺼낸다.
	- epoll_wait에서는 내부적으로 do_epoll_wait() -> ep_poll() 순으로 호출된다.

### `/proc`

리눅스는 각 프로세스의 열린 파일 디스크립터 정보를 보통 `/proc/PID/` 경로에 저장한다.

epoll역시 fd이기 때문에 이 구조에 나타난다.

```bash
ps aux | grep your_epoll_server
```

```bash
ls -l /proc/<PID>/fd
```

```bash
# result
3 -> socket:[12345]
4 -> anon_inode:[eventpoll]
```
> anon_inode:\[eventpoll]은 epoll fd임을 의미한다 커널 내부의 익명 inode


해당 epoll fd 상세 정보를 보고싶으면 `cat /proc/<PID>/fdinfo/4` 를 쳐보면 된다.
```makefile
pos:     0
flags:   020000002
mnt_id:  12
tfd:     5 events: 1 data: 5
tfd:     6 events: 1 data: 6
```

- tfd: targetFD(감시 대상)
- events: 감시하는 이벤트 (EPOLLIN=1)
- data: 사용자 정의 데이터 or fd

등록된 감시 대상들이 epoll 내부에 어떻게 저장되어 있는지 확인 가능

 `cat /proc/sys/fs/epoll/max_user_watches` 이 명령어로 아래와 같은것들을 확인가능하다
- 한 사용자당 epoll에 등록할 수 있는 최대 감시 항목 수.
- 기본값은 1,000,000개이며, 이를 초과하면 `epoll_ctl`이 `ENOSPC` 반환
