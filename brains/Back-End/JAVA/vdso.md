# vDSO(virtual Dynamic Shared Object)

vDSO는 리눅스 커널이 유저 공간 메모리에 미리 매핑해두는 작은 공유 라이브러리다.

리눅스 커널이 제공하는 고성능 시스템콜 최적화 메커니즘이며 jvm이 이를 그대로 활용함.

커널로 트랩을 타고 들어가야하는 시스템 콜중에서 안전하게 유저 영역에서 처리가 가능한 것들은 **커널 진입 없이**처리하도록 하기 위해서다.

즉 syscall -> kernel -> 유저모드 복귀

이 과정을 생략시켜 비교적 잦은 호출(시간관련호출등)의 비용을 줄이는 구조임

jvm은 내부에서 시간 조회, 스레드 스케줄링, 난수 seed, gc timestamp등 다양한 곳에서 다음 호출들을 자주 사용한다.

`gettimeofday()`, `clock_gettime()`, `time()`, `clock_getres()`

이 중에서 `clock_gettime(CLOCK_MONOTONIC)`와 같은 호출은 거의 모든 gc단계와 jvm 스케줄링에서 반복적으로 호출된다.

이 함수들이 vDSO를 통해 구현되어 있다면? jvm은 커널 왕복없이 유저모드에서 빠르게 처리한다.

### JVM vDSO

jvm 내부는 시간 기반 연산이 자주 발생하는데 예를들어 몇개 알아보자면

1. gc timestamp
   1. gc 시작 ~ gc 종료 시각
   2. stw
   3. young old gc event timestamp

위 값들은 전부 os::javaTimeNanos(), os::elapsedTime(), os::elapsed_counter() 같은 jvm 내부 함수로 가져오며 내부적으로 clock_gettime()을 호출한다. 이때 vDSO가 활성화되어 있으면 훨신 빠르다

이 밖에도 JIT Compile Scheduling, lock mutex timing thread schedule 등이 있고 시간 기반 연산이 매우 잦은 vm은 vDSO 유무에따라 미세하지만 누적 차이가 상당히 난다.

리눅스에서 `cat /proc/$(pgrep -n java)/maps | grep vdso` 명령을 통해서 vDSO가 활성화 되어있는지 볼수있다

```
7ffe39bff000-7ffe39c00000 r--p 00000000 00:00 0                          [vvar]
7ffe39c00000-7ffe39c01000 r-xp 00000000 00:00 0                          [vdso]
```

보이면 활성화되어있다.

어떤 함수가 vDSO에서 호출되는지 확인하는법은 `objdump -T /usr/lib64/libc.so.6 | grep vdso` 이걸로 보면 된다.

컨텍스트 스위치 같은게 일어나 사용자 모드에서 커널 모드로 전환될때 사용자 스레드의 명령어 캐시 기타 캐시를 비우게 된다. 

이는 사용자 공간 코드가 접근하는 메모리 영역과 커널이 접근하는 메모리 영역이 일반적으로 겹치지 않기 때문이고

컨텍스트 스위치에서 커널 모드로 전환되면 변환 조회 버퍼를 무효화 함으로써 잠재적인 다른 캐시들도 무효화 된다.

호출이 반환될 때 이러한 캐시는 다시 채워져야하고 이러한 전환의 영향은 제어가 사용자 공간을 돌아온 후에도 지속되기 때문에

이러한 비용을 줄이기위해 가상 동적 공유 객체 vDSO를 사용하는것이라고 볼 수 있다.

사용자 공간 내 메모리 영역으로 커널 권한이 실제 필요하지 않은 시스템 호출을 가속화하기 위한 것.

쉽게 말해 굳이 커널레벨 안가도 되는데 왜 가냐 가지마라 이걸로 해라 이런느낌