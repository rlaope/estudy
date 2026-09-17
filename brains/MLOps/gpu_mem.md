# GPU 내부 메모리 계층 구조

CPU 소프트웨어 엔지니어는 변수를 선언할 때 그 데이터가 물맂거으로 L1 캐시에 들어갈지 L2 캐시에 들어갈지 전혀 신경쓰지 않는다.

모든 것을 하드웨어가 알아서 최적화 해주기 때문이다. 하지만 GPU 커널을 작성하는 엔지니어는 왜 데이터를 어느 메모리 영역 (Register, Shared Memory, Global Memory)에 할당할지 명시적으로 코딩해야만 할까?

CPU와 GPU는 목적이 다르기 때문에 실리콘 다이(Die) 위에 SRAM을 배치하고 사용하는 방식 자체가 아예 다르다

### CPU 메모리 계층 (투명한 캐시, Transparent Cache)

L1, L2, L3 캐시는 프로그래머에게 물리적 주소 공간으로 노출되지 않는다.

하드웨어 프리페처(Prefetcher)와 캐시 일관성 프로토콜(Cache Coherence)이 이 메모리 접근 패턴을 실시간으로 분석하여, 자주 쓰이는 데이터를 DRAM에서 자동으로 복사한다.

지연 시간을 줄이는 것이 유일한 목적이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FJyySF%2FbtsDIzjsJ1T%2FAAAAAAAAAAAAAAAAAAAAANe7cfFyCqtq0Uptqcxr5WUJ23sgPcP8NauuH0cEtpMx%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1780239599%26allow_ip%3D%26allow_referer%3D%26signature%3DKMI5iRFftKi8vmWiTktA0Elw50c%253D)


### GPU 메모리 계층 (명시적 메모리, Explict Memory)

GPU는 수십만개의 스레드를 동시에 처리하므로 자동화된 캐시 관리 로직을 넣을 실리콘 면적과 전력이 부족하다. 대신 프로그래머가 직접 제어할 수 있는 고속 SRAM 영역을 노출 시킨다.

- **VRAM (Global Memory)**: HBM이나 GDDR 기술로 구현된 GPU의 주 메모리다. 대역폭은 넓지만 레이턴시가 수백 사이클에 다한다.
- **Shared Memory (공유 메모리)**: SM 내부에 위치한 프로그래머가 직접 주소를 할당하고 제어할 수 있는 L1 수준의 초고속 SRAM이다.
- **Register File (레지스터 파일)**: CPU 코어당 수십 개의 불과한 레지스터와 달리, GPU의 SM에는 수만개의 32-bit 레지스터가 거대한 물리적 배열 형태로 존재한다.

<br>

## 문제 정의

GPU가 대규모 병렬 스레드로 실행할 때, 전통적인 CPU 캐시 아키텍처를 그대로 가져다 쓰면, 시스템이 물리적으로 붕괴한다.

- **Cache Thrashing에 의한 대역폭 고갈**: 만약 GPU에 CPU와 같은 자동 L1 캐시를 달아놓고 수만개의 스레드가 각자 다른 데이터를 요구하면, 한정된 캐시 라인에 기존 데이터를 끊임없이 밀어내고 새로 적재하는 Eviction이 발생하고, Hit Rate가 0에 수렴하여 VRAM 대역폭만 소모하게 된다.
- **컨텍스트 스위칭의 물리적 저장 공간 부족:** CPU는 스레드를 교체할 때 레지스터의 값을 메인 메모리로 백업한다. GPU는 매 클럭마다 Warp, 32개 스레드를 교체해야하는데 이때 레지스터의 값을 VRAM으로 왕복 백업한다면 메모리 버스가 마비된다.
- **데이터 의존성으로 인한 VRAM 왕복 지연:** 행렬 곱셈시 하나의 스레드 블록 내에 있는 256개의 스레드가 동일한 데이터 Tile을 반복해서 읽어야하고, 이를 매번 VRAM에서 읽어오면 막대한 전력 소모와 지연 시간이 누적된다.

### Solution

이러한 물리적 한계를 극복하기 위해서 GPU 아키텍처는 캐시의 제어권을 하드웨어에서 소프트웨어(개발자)로 넘겼다.

- **초거대 레지스터 파일 정적 할당:** GPU는 SM 내부에 수백 KB에 달하는 거대한 레지스터 파일을 배치한다. 스레드 블록이 SM에 할당될 때, 컴파일러가 계산한 만큼의 레지스터를 각 스레드에 고정적으로 분할할당 해버린다. (Static Partitioning) 스레드가 stall에 빠져도 레지스터 값은 메모리로 방출되지 않고 물리 칩셋에 그대로 유지되므로 0클럭 컨텍스트 스위치가 가능해진다.
- **소프트웨어 관리형 L1 Cache (Shared Memory)**: 하드웨어가 짐작해서 데이터를 L1에 올리는 대신, 개발자가 `__shared__` 키워드를 사용해 VRAM의 특정 블록을 SM 내부의 SRAM 으로 명시적으로 복사하도록 강제한다. 블록 내의 모든 스레드는 이 공유 메모리를 통해 VRAM 접근 없이 데이터를 수십번 재사용할 수 있다.

<br>

## 하드웨어 데이터 패스 및 메모리 계층 동작 원리

VRAM에(HBM)에 저장된 행렬 데이터가 스레드 연산 유닛 ALU에 도달하기까지의 물리적 데이터 패스를 추적해보면

1. **VRAM에서 L2 캐시로의 이동:** GPU 커널이 `Global Memory` 로드 명령을 내리면, 메모리 컨트롤러가 HBM에서 데이터를 읽어 GPU 칩 중앙에 위치한 L2 캐시로 가져온다. L2 캐시는 모든 SM이 공유하는 유일한 자동 캐시 영역이다.
2. **L2 캐시에서 SM 내부 SRAM으로 적재 (Shared Memory)**: 스레드 블록 내의 스레드들은 협력하여 (Coalesced Memory Access) L2 캐시에서 데이터를 읽어 자신들이 속한 SM 내부의 `Shared Memory` 영역으로 복사한다. 이때 데이터는 SM 외부를 벗어나지 않는 로컬 데이터가 된다.
3. **동기화 장벽 Barrier**: 공유 메모리에 데이터가 다 올라올 때까지 연산이 꼬이지 않도록 `__syncthreads()` 하드웨어 장벽 명령어를 통해 256개 스레드의 타이밍을 정렬한다.
4. **Shared Memory에서 Register File로 이동:** 각 스레드는 자신이 이번 클럭에 연산할 특정 Operand를 Shared Memory에서 자신의 개별 Register File로 로드한다.
5. **연산, VRAM Write-back**: 레지스터의 데이터를 ALU or Tensor Core에 쏘아 보내 연산한다. 연산이 완료된 결과값은 다시 레지스터 -> 공유 메모리를 거치거나, 곧바로 L2 캐시를 통과하여 VRAM으로 영구 기록된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FTJ04m%2FbtsDHq1A6YG%2FAAAAAAAAAAAAAAAAAAAAACdjLLwh46x7kkLodJYEgvTPZw0wovKvMoMKJZ2bSEZS%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1780239599%26allow_ip%3D%26allow_referer%3D%26signature%3Dxq6HW0XVN7F0zvH4iIoWT%252F6lvSU%253D)

<br>

## 메트릭 및 프로파일링 로그 분석

메모리 계층 설계나 실패 병목을 진단하기 위해 NVIDA 툴체인이 출력하는 로우레벨 로그와 메트릭을 분석한다.

#### `ptxas` 컴파일러 경고 (Register Spiling 로그)

```
ptxas info : Used 255 registers, 8 bytes smem, 16 bytes cmem[0]
ptxas warning : Spills 32 bytes to local memory
```

위 로그를 해석해보면 커널 함수 내에 너무 많은 지역변수를 선언하여 하드웨어가 스레드당 할당할 수 있는 최대 레지스터 한계에 도달한 것이다 (위 예제에선 255)

넘쳐난 32바이트의 데이터는 이름만 Lcoal Memory일 뿐 실제 물리적 위치는 가장 느린 VRAM으로 쫒겨나게 된다 이게 Spill.

이 경고가 뜨면 해당 커널의 성능은 수십 배 하락할 수 있다.


#### Nsight Compute `ncu` Shared Memory 뱅크 충돌 메트릭

- `l1tex__data_bank_conflicts.sum`: 공유 메모리는 대역폭을 극대화하기 위해 32개가 독립적인 뱅크 구조로 나뉘어 있다. 만약 워프 내의 32개 스레드가 하필 동일한 뱅크에 있는 각기 다른 주소를 동시에 요청하면 하드웨어 병목이 발생하여 접근이 직렬화된다. 이 카운터값이 높으면 메모리 주소 할당 배열에 패딩을 추가하여 뱅크를 어긋나게 물리적으로 재정렬 해야한다.
- `sm__occupancy.avg.pct_of_peak`: 레지스터 파일이나 공유 메모리를 스레드 블록이 너무 크게 점유하면, 남은 SRAM 공간이 부족하여 다음 블록을 SM에 올리지 못한다. 이 지표가 낮으면 레지스터 SRAM 사용량을 다이어트 해야한다.

### 프로파일링/설정

백엔드 및 AI 인프라에서 메모리 계층의 물리적인 한계를 통제하기 위해서는

#### 런치 바운드를 통한 레지스터 강제 제한 `__lauch_bounds__`

커널의 레지스터 사용량이 너무 많아 Occupancy(SM내 활성 스레드 비율)가 떨어지는 것을 방지하기 위해 컴파일러에게 이 커널은 블록당 256개의 스레드를 쓸 것이고, SM당 최소 4개의 블록은 유지해야하니 알아서 레지스터 최대치를 억제하라 라고 강제 명령을 내린다.

```
// 블록당 256개 스레드, SM당 최소 4개 블록 유지를 강제하여 Occupancy 확보
__global__ void __launch_bounds__(256, 4) my_matmul_kernel(float* A, float* B, float* C) {
    // 컴파일러는 이 블록 내부의 지역 변수를 최소화하여 레지스터 초과를 막고,
    // 필요하다면 명령어 재배치를 통해 최적화합니다.
}
```

파이토치 환경에서 Triton 컴파일러로 gpu 커널을 작성할 때 명시적으로 SRAM 크기를 조율할수도있다.

```python
@triton.jit
def matmul_kernel(a_ptr, b_ptr, c_ptr, ...):
    # 1. HBM(VRAM)에서 포인터 연산으로 데이터를 로드
    # 이 과정에서 블록 크기(BLOCK_SIZE)만큼의 데이터가 SRAM(Shared Memory)으로 자동 할당됨
    a_block = tl.load(a_ptr + offsets, mask=mask)
    
    # 2. 레지스터 레벨의 지역 변수에 누적 연산 진행
    accumulator = tl.zeros((BLOCK_SIZE_M, BLOCK_SIZE_N), dtype=tl.float32)
    accumulator += tl.dot(a_block, b_block)
    
    # 3. 최종 결과를 다시 VRAM으로 전송
    tl.store(c_ptr + offsets, accumulator, mask=mask)
```