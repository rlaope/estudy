# Kernel Space vs User Space

### Kernel Space

Kernel Space는 운영체제 커널이 실행되는 특권 영역이다.

cpu 권한 레벨로는 Ring 0(가장 높은 권한)에서 동작한다.

커널은 기본적으로 다음 역할을 수행하는데.

- 프로세스 관리 (스케줄링, context switch)
- 메모리 관리 (page table, virtual memory, numa, tlb)
- 파일 시스템 관리 (VFS -> ext4/xfs ...)
- 네트워크 스택 처리 (tcp/ip)
- 디바이스 드라이버 운영
- 보안 정책 처리 (LSM, seccomp 등)

즉, **하드웨어 접근과 시스템 자원 관리**는 모두 커널이 수행하는 작업이며 사용자 프로세스는 직접 접근할 수 없다.

Kernel Space Code Example
- Linux Kernel 그 자체
- 드라이버 (NIC Driver, 블록 디바이스 드라이버)
- 시스템 호출 핸들러
- TCP/IP stack (netfilter, conntrack 포함)

### User Space

User Space는 일반 애플리케이션이 실행된느 비특권 영역으로 권한레벨은 Ring 3

- 웹서버 (Nginx, Spring Boot, NodeJS)
- CLI (ls, grep, bash)
- Docker daemon
- JVM, Python Interpreter etc..

UserSpace에서 직접 cpu memory disk 네트워크 하드웨어에 접근할 수 없으며 모든 하드웨어 요청은 반드시 Kernel Space를 통해야한다.


### 왜 구분하는가?

사용자 프로세스가 잘못된 포인터 접근으로 커널 메모리를 망가트리면 시스템 전체가 다운될 수 있다.

따라서 커널은 자신을 보호하기 위해 사용자 공간을 격리해놓는데 즉 시스템 안정성 때문에

그리고 유저 영역에서는 악성 코드가 섞일수도 있으니 커널은 반드시 Ring 3에 격리한다.

직접 하드웨어를 조작할 수 없게 함으로써 권한 상승을 방지 즉 보안 때문이다.

그리고 커널은 고성능 io 처리(nic, block io, tcp stack)를 수행해야 하므로 전용 영역에서 인터럽트 처리, DMA 제어 메모리 매핑등을 최적화 한다.


### System Call

User Space -> 커널 기능 사용 -> 다시 User Space 복귀

이때 필수적으로 system call이 발생한다.

예르르들어 `write(fd, buf, size)` 이 콜은

1. user space에서 write 호출
2. system call 번호를 레지스터에 넣고 커널로 진입
3. cpu ring3 -> ring0로 전환 (privileged mode)
4. 커널이 디바이스 드라이버에게 io를 요청함
5. 작업이 완료 후 user space 복귀 (ring0 -> ring3)

중요한 점은 이 과정에서 context switch + 권한 전환 비용이 발생함.

그래서 고성능 서버에서는 다음을 줄이는게 중요한데.
1. syscall 횟수 줄이기
2. epoll 사용하기
3. sendfile(zero-copy)
4. io_uring 사용

서버 성능 최적화랑 연결되는 방법이다.

```
┌───────────────────────────────┐
│           User Space          │
│  App, JVM, Python, Nginx      │
│  ────────────────┐            │
└──────────────────┼────────────┘
                   ▼  System Call
┌──────────────────┴────────────┐
│          Kernel Space          │
│ Scheduler, Memory Manager      │
│ TCP/IP stack, VFS, Drivers     │
└────────────────────────────────┘
                   ▼
             Hardware (CPU, NIC, Disk)
```

### 위에서 나온 용어 몇개 정리

**인터럽트 처리 Interrupt Handling**이란 하드웨어가 cpu에게 지금 이것을 처리해야한다고 신호를 보내는것  

ex. NIC가 패킷을 받아 패킷이 도착했다는 인터럽트 발생, 디스크가 읽고 데이터가 준비됐다는 인터럽트, 키보드 입력 키 눌렸다는 인터럽트  

인터럽트가 없으면 데이터가 왔는지 계속 폴링으로 알아야해서 성능이 떨어짐.


**DMA(Direct Memory Access)** 제어란

DMA는 디바이스가 cpu를 거치지 않고 메모리에 직접 데이터를 읽고 쓰는 기술이다.

NIC이 패킷을 받는 과정을 예로 들어볼 수 있는데.
1. NIC이 DMA를 이용해 시스템 메모리에 패킷을 복사한다.
2. NIC이 CPU에게 인터럽트를 날린다.
3. CPU는 복사된 메모리 주소만 읽어서 처리한다.

DMA가 없던 시절엔 하드웨어 -> cpu -> memory 로 복사가 일어나 cpu 부하가 매우 컸다.

DMA가 생겨서 하드웨어 -> 메모리로 작동해 cpu는 복사 작업에서 해방되고 고속 io 구현의 핵심이 된다.

**메모리 매핑이란** 커널이 특정 메모리 영역(파일, 디바이스 메모리 등)을 프로세스 가상 주소 공간에 그대로 연결해주는 것이다.

mmap()으로 파일을 메모리에 매핑하면 read() 호출 없이도 메모리를 읽으면 곧 파일 데이터가 된다.

tcp socket도 커널 버퍼가 mmap 기반으로 관리된다.

왜 중요하냐면 read/write 없이 메모리를 직접 읽는 구조라 성능이 향상되고 zero copy 기술이라 성능이 향상된다.

물론 프로세 메모리는 기본적으로 다른 메모리와 공유되지 않지만, 공유가 필요한 상황에서 copy하기 보다는 공유가 비용이 싸다고 생각하면 쉽다. open read write lseek같은 함수를 mmap하나로 줄일수있어서 좋다.

### syscall 횟수 줄이기

user space -> kernel space로 진입할때마다 system call 비용이 발생한다 read write recv send 호출마다

Ring3 -> Ring0으로 전환하고 컨텍스트 스위칭에 커널 검증, 커널 버퍼 작업등이 발생해 비싸다.

그래서 이런것들을 줄여야 성능이 좋아진다.

epoll 사용을 알아보자 기본적으로 socket io는 아래처럼 동작한다.

```scss
read()  
데이터 없으면 블록됨  
또 read()
또 read()
```

이러면 수천 수만개의 소켓을 감시할때는 불가능하다.

하지만 epoll은 소켓에 이벤트가 생길때만 알려준다.
- 새패킷 도착
- 연결끊김
- 쓰기 가능

즉 필요할때만 notify -> syscall 횟수 급감

그래서 고성능 서버 네티같은 애들은 epoll 기반이다.

그리고 두번째로 sendfile(zero copy)에 대해서도 알아보자.

일반적으로 파일 -> 네트워크 전송 과정은 아래와 같다.

1. 파일 -> 커널 버퍼
2. 커널버퍼 -> user space 버퍼
3. user space 버퍼 -> 커널 네트워크 버퍼
4. 네트워크 카드를 통해서 전송

즉 3번의 복사가 일어난다. 그러나 sendfile은? 파일 -> 커널 -> NIC으로 바로 보내도록 해준다.

즉 user space의 복사단계가 없는 제로카피로 cpu 사용량이 감소한다.

nginx, kafka, envoy 등 모두 sendfile을 사용한다.

마지막으로 io_uring(리눅스 최신 io api)에 대해서 알아보자

과거 io는 syscall을 필요로 했다. epoll + read/write 방식

근데 io uring은 다음을 가능하게 한다.
1. user space에서 커널 큐에 직접 io요청이 가능함
2. completion ring에서 결과를 폴링
3. syscall 없이 수십 수백개의 io 실행 가능
4. read write 호출 자체가 사라짐

즉 기존의 리눅스 io의 거의 모든 병목을 제거한 구조로 가장 빠른 리눅스의 io방식이다.

redis postgre nginx 등도 지원하기 시작했다.