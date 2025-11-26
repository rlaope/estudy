# System Call Interface

리눅스에서 User Space, Kernel Space 기능을 사용하는 유일한 통로가 System Call Interface이다.

애플리케이션이 하드웨어나 os 핵심 기능에 접근할 때 반드시 System Call을 통해야하며, 이 과정은 

단순 함수 호출이 아니라 cpu 권한 모드 전환과 커널 내부 실행이 포함된 무거운 작업이다.

저번에도 알아봤지만 system call이 필요한 이유는 user space(일반 애플리케이션)은 보안과 시스템 안정성을 위해서

하드웨어 커널 데이터에 직접 접근할 수 없다. 예를들어 디스크 접근이나 네트워크 통신 메모리 할당, 프로세스 생성 등이 안된다.

이 모두가 User space에서는 불가능하므로, system call을 사용하게 된다.

**커널에게 해야할 일을 위임하는 공식적인 요청과정 바로 system call이다**

### syscall 호출 과정

애플리케이션에서 `read(fd, buf, size)`를 호출한다고 가정해보겠다.

겉으로는 그냥 함수처럼 보이지만 실제 내부에서는 다음처럼 훨씬 복잡하게 움직인다.

#### libc의 wrapper 함수 호출

- User Space에서는 glibc에서 제공하는 read() 함수를 호출한다. 하지만 glibc는 실제 read가 아니라 단지 system call 번호를 cpu 레지스터에 넣고 커널로 요청을 올리는 포장(wrapper) 일 뿐이다.

#### cpu 권한 상승

-  read 함수는 단순한 함수 호출이 아니라 CPU를 Ring 3 -> Ring 0으로 전환하는 작업을 수행한다.
-  기존 방식은 `int 0x80` 인터럽트, 현대 방식(x86-64): `syscall` 명령어, ARM64는 `svc #0`  
-  이상하게 생기긴 했지만 이 명령어들이 실행되는 순간 cpu는 특권 모드로 전환해 kernel space로 진입한다.  

#### 커널의 system call dispatcher

커널은 system call 번호에 따라 어떤 커널 함수를 실행할지 분류한다.

예를들어서

- 0: read
- 1: write
- 2: open
- 39: getpid
- 57: fork
- 63: uname

이 번호 테이블들을 system call table이라고 부르고 dispatcher라고 부르는애가 read의 커널 내부 함수인 sys_read()를 호출한다

#### 커널 내부 로직 실행

sys_read는 다음을 수행한다

1. 사용자 메모리(buf)가 유효한지 검증한다
   1. 악성 프로세스가 커널 메모리를 읽으려고 해도 막아야 하기 때문이다.
   2. copy_from_user()로 안전하게 복사한다.
2. 파일 디스크립터가 유효한지 확인한다.
3. 커널 파일 시스템(VFS)으로 접근한다.
4. 파일 데이터를 가져오기 위해 디스크 드라이버에 요청한다.
5. 그결과 User Space 버퍼로 다시 복사한다.

이 모든 과정은 User Space 함수 호출보다 몇십에서 몇백배 더 비싸다.