# File Descriptor in Network(cc. KeepAlive)

운영체제가 열린 파일, 소켓, 파이프 같은 io리소스를 식별하기 위해 부여하는 정수 번호다.

Linux에서는 다음을 모두 fd로 취급한다.

- 일반 파일
- 디렉토리
- 소켓(tcp,udp)
- 파이프
- epoll 인스턴스
- eventfd
- signalfd

즉, fd는 단순 파일 핸들링이 아니라 모든 io의 포인터 역할을 한다.

### 프로세스가 fd를 사용하는 방식

프로세스는 모든 io 대상에 대해 fd 테이블을 가진다.

예시로 어떤 서버가 tcp 소켓을 1개 accept 하면:

- 프로세스 fd 테이블에 새 entry를 생성
- 커널의 global file table에 참조 증가
- tcp 소켓이 커널 메모리에 실제로 할당됨.

결론: 클라이언트 1명이 연결 = fd 1개 추가 점유율이다.

### FD 고갈

프로세스 또는 시스템 전체에서 더 이상 FD를 새로 할당할 수 없는 상태를 말한다.

원인은 단순하다.

- 서버가 너무 많은 tcp 연결을 동시에 유지한다.
- 죽지 않은 idle connection이 누적된다.
- FD limit이 낮다 (`ulimit -n`)
- 리소스 누수 (fd close 누락)

fd가 고갈되면 다음과 같은 에러가 발생한다.

- EMFILE(per-process FD 초과)
- ENFILE (system-wide FD 초과)
- 서버가 더이상 소켓을 accept하지 못함
- logging 실패
- 파일 open 불가

fd는 핵심 인프라 자원이기 때문에 고갈되면 서버가 죽은 것 처럼 행동하기 시작함.

### FD 제한 값 (가장 중요한 부분)

Linux 에서는 3가지 계층에 FD limit이 존재한다.

시스템 전체 limit `proc/sys/fs/file-max`: 시스템 전체에서 동시에 열 수 있는 파일 핸들의 총량

per-user limit `/etc/security/limits.conf` 내 nofile 여기서 soft/hard 설정 가능

per-process limit `ulimit -n`: 서버 프로세스 한 개가 열수있는 fd 수

대부분의 서버는 기본 1024로 매우 낮고 운영환경에서는 65535~200000정도로 올린다.

### Keep-Alive 환경에서 fd가 고갈되는 이유

설명을 봤으면 쉽게 예측가능하겠지만 http keep-alive는 tcp연결을 오래 유지하는 기능이고 즉 요청 처리후 연결을 닫지 않으니

클라이언트 수가 증가할수록 할당된 fd도 오래 점유되어 fd 점유량이 늘어나게 된다.  idle connection도 fd를 계속 먹고 있으니

다음 시나리오가 많이 터지는데
- 로드가 몰려 keep alive timeout을 길게 설정
- lb나 클라이언트가 비정상적인 연결을 유지
- crawlers, bot, 혹은 느린 클라이언트의 지속 연결
- fd limit을 올리지 않은 서버

그래서 고부하 서버는 fd 고갈 방지를 위해
- `keepalive_timeout`: 적절히 설정
- `max_keepalive_requests`: 제한
- connection pool 크기 제한
- reverse proxy 앞단에서 connection offloading
- h2/h3 연결 갯수 자체를 줄이기
등을 반드시 한다.

fd는 단순히 열린 파일 개수로 측정되는것이 아니라 실제로는 fd가 커널 자원을 참조하고 있어서 부하도 동반한다.

tcp 소켓 하나당 ex로

```
소켓 상태 구조체
send buffer
receive buffer
TCP control block
retransmission timer
SYN queue / accept queue
```

등이 필수적으로 메모리에 유지되며 fd가 많아질수록 커널 메모리 사용량과 context switching 비용이 증가한ㄴ다.

```bash
# process 단위
lsof -p <pid> | wc -l
cat /proc/<pid>/limits

# 전체 시스템 단위
cat /proc/sys/fs/file-nr
cat /proc/sys/fs/file-max

# Prometheus
node_filefd_allocated
process_open_fds
```

즉 fd는 io 리소스 식별자며 서버가 많은 연결을을 유지할수록 fd가 누적되고 keep-alive 환경에서 고갈되기 쉽다.

그렇기에 성능 용량 커넥션 관리에 핵심 자원중 하나로 다루어야한다.