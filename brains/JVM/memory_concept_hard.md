# HotSpot JVM ZGC Barrier, NUMA 메모리 할당 및 Safepoint 동작

Java 21 최신 LTS 기준으로 ZGC Generational 도입과 NUMA 아키텍처 고도화, Safepoint 메커니즘들의 로우 레벨 동작을 os, 하드웨어, c++ 런타임 소스코드 레벨에서 해부해보자.

## ZGC의 Colered Pointers, Load/Store Barriers

Java 21에서 ZGC는 Generational ZGC(JEP 439)가 도입되면서 큰 변화를 겪었다. 기존에는 로드 베리어만 작동했지만, 이제는 young/old 세대를 구분하고 세대간 참조(Remembered Set)을 추적하기 위해 **Store Barrier**가 매우 중요한 역할을 하게 되었다.

### Color Pointers

ZGC는 64비트 포인터 공간 중 객체의 실제 메모리 주소를 가리키는 데 일부(예: 44~48)비트를 사용하고 남은 상위 비트들을 메타데이터(color)로 사용한다.

- **비트 구성:** `Marked0`, `Marked1`, `Remapped`, `Finalizable` 등의 상태를 나타낸다
- **하드웨어/OS 매핑:** 과거 ZGC는 동일한 물리적 페이지에서 여러 가상 주소를 매핑하는 Multi-mapping(Virtual Memory Aliasing) 기법을 사용했지만 최신 아키텍처 x64의 Address Marking이나 AArch64의 TBI-TopByteIgnore 를 지원하는 환경에서는 os레벨의 꼼수없이 cpu의 하드웨어 기능을 활용해 메타데이터 비트를 무시하고 실제 주소로 디레퍼렌싱한다.

### Load/Store Barrier

Barrier는 JIT Compiler(C2)가 기계어를 생성할 때 객체 참조를 읽거나 쓸 때 강제로 삽입하는 코드 쪼가리다.

**Fast Path (Assembly level inline code)**라고 모든 메모리 접근에 C++ 런타임을 호출하면 성능이 붕괴될 수 있다. 따라서 JIT 컴파일러는 HotSpot의 `ZBarrierSetC2` 클래스를 통해 Fast Path를 어셈블리로 직접 인라인한다.

- **Load Barrier**: 포인터를 로드할 때 현재 ZGC의 Good Color 마스크와 포인터 색상의 비트를 비교해 `bitwise AND` or `CMP` 한다.

```
mov r10, [rax + offset]      ; 메모리에서 포인터 로드
test r10, [Good_Color_Mask]  ; 현재 올바른 상태(Remapped 등)인지 비트 검사
jnz slow_path_stub           ; 색상이 맞지 않으면(Bad Color) Slow path로 점프
; 색상이 맞으면 그대로 진행 (Fast Path)
```

### Slow Path

`jnz`로 인해 slow path로 빠지게 되면 어셈블리 스텁은 HotSpot 내부의 C++ 런타임 함수를 호출한다

- `src/hotspot/share/gc/z/zBarrierSetRuntime.cpp` 파일 내의 함수들(예: `ZBarrierSetRuntime::load_barrier_on_oop_field_preloaded`)이 호출한다.
- **Load Barrier Slow Path**: 객체가 relocation(이동)중이라면 forwarding table을 참조해 새 주소를 찾아내고(heal), 포인터의 색상을 `Remapped`로 업데이트한 뒤 최신주소를 반환한다. 이 과정에서 CAS 연산이 사용되어 멀티스레드 경합을 안전하게 처리한다
- **Store Barrier (Generational ZGC)**: 객체의 필드에 새로운참조를 쓸 때 발생한다. old객체가 young 객체를 가리키게 되는지를 검사하고 만약 그렇다면 해당 old 객체가 위치한 메모리의 영역 주소를 ZGC의 RememberedSet에 기록한다 `ZRememberedSet::record_store` 하여 다음 young gc때 root로 사용할 수 있도록

<br>

## TLAB & PLAB 할당 흐름과 os page / numa 아키텍처 결합

객체 할당은 JVM 성능의 알파이자 오메가인데 ZGC나 G1 모두 기본적으로 Bump-the-pointer 방식을 사용하며

이를 스레드 로컬 환경으로 가져온것이 TLAB(Thread Local Allocation Buffer)과 PLAB(Promotion Local Allocation Buffer)이다.

- TLAB (Application Thread): Java 애플리케이션이 객체를 new로 생성할때 사용하고 `ThreadLocalAllocbuffer` C++ 클래스가 관리하며 lock없이 자신의 버퍼 내에서 포인터만 증가시켜 bump 할당한다.
- PLAB (GC Worker Thread): GC가 살아남은 객체를 다른 영역 young -> old 혹은 evacutation 시 복사 promotion/relocation 할때 gc 스레드들이 사용하는 로컬 버퍼다. 이 역시 gc 스레드간의 락 경합을 없애기 위함이다.

**OS 메모리 페이지 및 NUMA**는 단순히 메모리를 할당하는 것을 넘어 물리적 하드웨어 NUMA랑 매핑하는것을 알아보는 목적인데.

`+UseNUMA` 옵션이 켜져있다면 HotSpot의 행동은 다음과 같다.
1. **가상 메모리 예약 및 ZPage 할당**: TLAB이 꽉 차서 새로운 TLAB을 요구하면 JVM은 힙 young gen에서 일정 크기의 청크 (zgc라면 ZPage)를 스레드에게 내어준다. 이떄 핫스팟 메모리 관리자는 ZPage가 현재 스레드가 실행중인 cpu의 numa 노드에 할당되기를 원한다
2. **OS First-Touch Policy**: 리눅스 등 현대 OS는 `mmap`으로 메모리를 할당받아도. 실제 물리적 RAM(Page Frame)에는 매핑되지 않은 가상 상태고 해당 메모리 주소에 처음으로 쓰기 작업이 발생할때(page fault), os는 해당 쓰기 작업을 수행한 스레드가 속한 cpu 코어의 로컬 numa 노드 물리 메모리를 할당한다. 이를 first-touch policy라고 한다/
3. **HotSpot의 개입(`os:numa_make_local`):** C++레벨에서 보면 hotspot은 새로운 TLAB 영역을 할당받으면 해당 영역에 0을 채우는 Zeroing 작업을 하는데 이는 보안 목적도 있지만 **실행중인 슬데ㅡ가 직접 메모리를 터치하게 만들어 os가 TLAB의 물리적 페이지를 현재 cpu의 numa 노드에 강제로 할당하도록 유도하는 고도의 테크닉이다**
4. **PLAB의 NUMA Locality**: Generational ZGC에서 GC 스레드가 old gen으로 promote 시킬때 PLAB을 사용하는데 이때도 ZGC의 numa-aware allocator는 가급적 원본 객체 혹은 실행중인 gc 스레드와 가까운 NUMA 노드의 메모리 zpage를 PLAB으로 할당하여 메모리 접근 지연시간을 최소화 시킨다.

<br>

## Safepoint의 Polling Page 메커니즘(OS Page Fault 기반)

safepoint는 gc, deoptimization, thread dump등을 위해 모든 java 스레드를 안전한 상태에서 멈추게 하는 메커니즘이다.

인터럽트 signal 없이 스레드를 어떻게 멈추는가? 에 대한 해답이 바로 polling page와 os의 메모리 보호 page fault 메커니즘 결합이다

### Polling Page의 초기화 및 JIT 컴파일

1. **Polling Page 할당**:  JVM 부트 스트랩시 `os::init_2` 등의 c++ 초기화 코드에서 `mmap`을 통해 특정 메모리 페이지 safepoint polling page를 하나 할당받는다. 평상시에 이 페이지는 읽기 가능한 상태다.
2. **JIT Compiler Polling Code 삽입**: c1,c2 컴파일러는 메서드 복귀 return 지점이나 루프의 끝 backedge등 전략적 위치에 polling instruction을 삽입한다.
   1. `test eax, [rip + polling_page_address]` (특정 레지스터 값을 polling page 주소의 값과 비교하거나 단순히 읽는 행위)
   2. 평상시에는 이 페이지가 읽는게 가능하므로 이 어셈블리 명령어는 캐시 히트시 거의 0에 가까운 오버헤드로 쓱 지나간다. 소프트웨어적 if문 체크보다 빠름

### Safepoint 도달 메커니즘

VM Thread(JVM 내부의 글로벌 제어 스레드)가 gc등을 위해 safepoint를 발동시켜야할 때 다음일이 일어난다.

1. VM Thread `os::make_polling_page_unreachable()` C++ 함수를 호출한다. 이 함수는 내부적으로 리눅스 시스템콜인 `mprotect(polling_page_addr, page_size, PROT_NONE)`을 실행한다. 이 순간 polling page의 읽기 쓰기 불가 상태로 변환된다 PROT_NONE
2. **Java 스레드의 Page Fault(SIGSEGV) 발생:** 애플리케이션 스레드들이 코드를 실행하다가 JIT이 삽입해둔 test 명령어를 만나 (위의 예시) polling page를 읽으려고 시도한다. 하지만 방금 vm thread가 이 페이지의 권한을 PROT_NONE으로 바꿨으니 하드웨어 MMU에서 Page Fault가 발생하고 os는 해당 스레드에 `SIGSEGV` 시그널을 날린다.
3. HotSpot Signal Handler의 개입(`os_linux_x86.cpp` - `JVM_handle_linux_signal`): 일반적인 C++ 프로그램이라면 SIGSEV를 맞고 크래시가 나겠지만 핫스팟 은 자체 커스텀 시그넘 핸들러를 등록해두었다
   1. 핸들러 로직: "방금 발생한 SIGSEGV의 메모리 주소 info->si_addr가 우리가 설정한 polling page가 일치하는지 확인"
   2. 일치한다면? -> 이것은 에러가 아니라 safepoint 요청임을 인지함.
4. **스레드 블로킹 및 safepoint 동기화**: 시그널 핸들러는 프로그램 코드를 크래시시키지 않고 현재 스레드의 상태를 `_thread_in_Java`에서 `_thread_blocked`로 바꾸고 c++ 내부의 모니터/뮤텍스를 획득하려고 시도하며 스스로를 park(대기) 상태로 만들고 모든 java 스레드가 이 함정에 빠져 블로킹되면 vm thread는 비로소 safepoint가 달성되었음을 인지하고 gc등의 작업을 시작한다.


이처럼 hotspot은 `if (need_safepoint)` 같은 소프트웨어 브랜치 검사의 오버헤드를 피하기 위해, 하드웨어 메모리 보호 유닛 MMU과 OS의 page fault 메커니즘을 런타임에 극한으로 활용(Zero-cost 평상시 오버헤드)하는 우아한 해킹을 구현중이다.