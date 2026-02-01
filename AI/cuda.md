# CUDA

### Compute Unified Device Architecture

CUDA는 NVIDIA가 설계한 병렬 컴퓨팅 플랫폼이자 프로그래밍 모델이다.

과거에는 gpu를 그래픽 렌더링 용으로만 사용했으나, 이를 일반적인 연산 GPGPU(General Purpose computing on Graphics Processing Units)에 사용할 수 있도록 c cpp등의 언어를 확장해 하드웨어에 직접 제어할 수 있게 만든 인터페이스다.

- 복잡한 gpu 하드웨어 명령어를 프로그래머가 익숙한 cpp 문법으로 다룰 수 있고 
- 수천개의 스레드를 하드웨어 연산 유닛에 효율적으로 분배하며
- cpu, gpu간 데이터 전송 및 gpu 내부 메모리 계층을 제어한다.

### CUDA 소프트웨어 스택 구조

CUDA는 단순히 언어가 아니라 하드웨어와 소프트웨어를 잇는 다층적 구조로 이루어져있다.

1. **CUDA Libraries:** 최적화된 함수 집합 (cuBLAS: 선형대수, cuDNN: 딥러닝 등).
2. **CUDA Runtime API:** 메모리 할당, 커널 실행 등 고수준 관리 기능을 제공하며 프로그래머가 주로 사용함
3. **CUDA Driver API:** 하드웨어에 더 가까운 저수준 을 제공 runtime api가 내부적으로는 driver api 호출함
4. **GPU Driver:** 운영체제와 gpu 하드웨어 사이의 통신을 담당한다.

### CUDA 컴파일 과정

CUDA 코드는 cpu에 서 실행되는 host code와 gpu에서 실행되는 device code가 섞여있다.

이를 처리하기 위해 전용 컴파일러인 nvcc를 사용한다.

1. 분리: nvcc가 .cu 파일을 열어 읽어 host, device 코드를 분리한다.
2. host compilation: host코드는 일반적으로 cpp 컴파일러(gcc, msvc)가 컴파일한다
3. device compilation: device코드는 ptx(parallel thread execution) 라는 중간 단계의 어셈블리 언어로 변환된다.
   1. 최종적으로 특정 gpu 아키텍처(Ampere, Hopper등)에 최적화된 바이너리인 SASS(Source Absolute Set of Standard)로 변환되어 실행된다.

### Kernel 실행 모델

kernel은 gpu에서 병렬로 실행되는 함수의 최소 단위다.

cpu가 호출하며, 수천 개 이상의 스레드가 동일한 코드를 각기 다른 데이터셋에 대해서 실행한다.

gpu는 스레드를 효율적으로 관리하기 위해 계층적인 구조를 가진다.

1. **thread**: 연산의 최소 단위며 각자 register, pc를 가짐
2. **block**: thread group 계층이며 공유 메모리와 스레드 집합, 동일 sm(steraming multiprocessor)에 할당된다./
3. **grid**: grid NDRange계층이며 하나의 커널 호출로 실행되는 전체 블록/스레드 그룹의 집합이다.

### host device 통신

cpu와 gpu는 물리적으로 분리된 메모리 공간을 갖는다. Discrete GPU 기준.

이들 사이의 데이터 흐름은 성능 병목의 주요 원인이다.

- 메모리 복사 레이턴시(PCle Overhead): 외부 gpu 시스템에서는 데이터 전송시 PCle 버스를 이용한다. 전송 대역폭은 VRAM 대역폭보다 현저히 낮으므로 빈번한 Memcpy는 연산 이득을 상쇄한다.
- 통합 메모리 (Unified Memory, Apple Silicon): Mac의 M시리즈 칩은 cpu와 gpu가 동일한 물리적 RAM을 공유한다.
  - zero copy: 포인터만 전달해서 데이터를 이동시키지 않고 공유할 수 있어서 복사 레이턴시가 거의 없다
  - visibility: 다만 데이터의 가시성을 보장하기 위한 캐시 플러시 제어는 여전히 필요함

### 동기화 및 메모리 가시성

gpu는 비동기로 방식으로 동작한다.

cpu가 커널을 실행하라고 명령을 내린 직후, 커널 완료를 기다리지 않고 다음 명령을 수행한다. 

따라서 정확한 결과를 위해 동기화 필수적이다.

**스레드 수준 동기화(Barrier)**
- CUDA: `__syncthreads()`
- Metal: `threadgroup_barrier(mem_flags::mem_threadgroup)`
- 목적은 공유 메모리에 데이털ㄹ 쓰고 읽을 때 발생할 수 있는 RAW(Read After Write) Hazard를 방지하기 위해서 

동일한 블록 thread group 내의 스레드들이 특정 지점에 도달할 때까지 대기하게 된다.

**커널 수준 동기화**  
- 커널 A의 결과가 B의 입력일때 커널 A가 완전히 종료되었음을 보장해야한다.
- Command Queue: 명령을 순차적으로 큐에 삽입해 실행 순서를 보장한다.
- Fence/Events: 서로 다른 큐나 스트림 간의 실행 의존성을 하드웨어적 신호를 주고받아 제어한다.

