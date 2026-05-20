# PCIe 레인 구조, GPUDirect Storage, NUMA Optimization

질문으로 시작해보겠다. "리눅스 파일 시스템에 저장된 테라바이트급 AI 학습 데이터를 GPU VRAM으로 밀어 넣을 때, 왜 이 데이터는 반드시 호스트 CPU의 메인 메모리 RAM을 거쳐야 하는 것일까? I/O 장치끼리 CPU의 허락 없이 직접 데이터를 주고받을 수는 없을까?"

단일 서버 Node 내에서 스토리지, NIC, 그리고 GPU 간의 물리적 통신은 **메인보드의 PCIe(Peripheral Component Interconnect Express)** 버스를 통해서 이루어진다.

- **PCIe 레인 (Lanes) 및 토폴로지**: 메인보드의 모든 고속 장치를 연결하는 직렬 버스 표준이다. GPU는 보통 16개의 레인 (x16)을 사용하여 CPU와 연결되며, PCIe Gen4 기준 양방향 64GB/s의 대역폭을 가진다.
- **NUMA (Non-Uniform Memory Access, 비균등 메모리 엑세스)**: 멀티 소켓 (2개 이상의 cpu) 서버에서 각 CPU는 자신만의 로컬 RAM 뱅크와 PCIe 슬롯을 가진다. 특정 cpu가 다른 cpu에 연결된 메모리나 gpu에 접근하려면 소켓 간 인터커넥트(UPI/QPI) 버스를 한번 더 건너가야하므로 레이턴시가 급증하는 아키텍처다.
- **GPUDirect Storage(GDS)**: 스토리지 (NVMe SSD)에 있는 데이터가 CPU 시스템 메모리 RAM을 거치지 않고 PCIe 스위치를 통해 곧바로 GPU의 VRAM에 꽂히도록 데이터 패스를 숏컷으로 뚫어주는 NVIDIA 하드웨어 기술이다.


<br>

## 물리적/구조적 병목

전통적인 POSIX I/O 구조를 사용하여 대용량 데이터셋을 GPU로 넘길 때 시스템 레벨에서는 세 가지 치명적인 병목 현상이 중첩되어 발생한다.

- **바운스 버퍼 복사 병목**: 리눅스 커널은 NVMe에서 데이터를 읽으면 먼저 커널 공간의 page cache에 저장하고, 이를 유저 공간의 버퍼 복사 `memcpy` 를 한다. 이후 CUDA 드라이버가 이를 다시 GPU VRAM으로 복사하고 한 데이터가 PCIe 버스를 두 번(스토리지 -> RAM -> GPU)이나 왕복하면서 PCIe 대역폭을 절반으로 깎아먹고 호스트의 CPU 사이클을 낭비한다.
- **GPU Starvation (데이터 공급 부족)**: cpu가 데이터를 복사하고 가상 메모리 매핑을 관리하는 오버헤드 때문에 연산 속도를 따라가지 못한다 1초에 수천장의 이미지를 처리할 수 있는 GPU가 데이터가 도착하지 않아 연산을 멈추고 멍하니 대기하는 현상이 발생한다.
- **NUMA 횡단 페널티**: 만약 NVMe SSD는 CPU 0번 소켓에 연결되어 있고, GPU는 CPU 1번 소켓에 연결되어 있다면, 데이터 호스트 RAM으로 올라온 뒤 좁디 좁은 CPU간 통신 버스 UPI를 건너가야한다. 이 경우 대역폭은 30~50%로 하락하고 레이턴시틑 폭증한다.

### Solution

이러한 OS 계층과 하드웨어 라우팅 비효율을 제거하기 위해 GPUDirect(P2P DMA) 기술이 도입되었다.

- **피어투피어(P2P) DMA(Direct Memory Access)**: 원래 DMA는 장치가 cpu 개입 없이 RAM에 접근하는 기술이다. P2P DMA는 한걸음 더 나아가 호스트 RAM 조차 배제하고 PCIe 버스에 꽂힌 두 장치(NVMe, GPU)가 서로의 메모리 주소 (BAR, Base Address Regsiter)에 직접 데이터를 읽고 쓰는 기술이다.
- **커널 VFS 우회(cuFile API)**: NVIDIA는 기존의 파일 읽기 시스템 콜 (`read()`) 대신 `cuFile` 이라는 전용 API를 제공한다. 이를 통해 리눅스 커널의 복잡한 파일 시스템 계층을 우회하고 NVMe 컨트롤러에 하드웨어 I/O 커맨드를 직접 하달한다.

### 하드웨어 데이터패스 메모리 계층 동작 원리

GPU Direct Storage가 활성화 되었을때 데이터가 메인보드 회로 레벨에서 이동하는 정확한 경로다.

1. **메모리 주소 매핑:** 유저 애플리케이션 (pytorch)이 데이터를 받을 GPU VRAM의 물리적 주소를 PCIe 공간(BAR1)에 노출시키고, 이 주소를 NVMe 컨트롤러에 통보한다.
2. **I/O 커맨드 발급**: 호스트 CPU는 데이터 복사에 전혀 관여하지 않는다. 단지 NVMe 컨트롤러의 Submission Queue에 SSD 특정 블록 데이터를 방금 알려준 GPU BAR 주소로 쏘아라 라는 제어 명령만 내리고 다른 스레드 작업으로 넘어간다.
3. **데이터의 PCIe 스위치 라우팅**: NVMe 장치가 데이터를 PCIe 레인으로 밀어낸다. 이 전기적 패킷은 CPU 소캣(Root Complex)까지 올라가지 않는다. 메인보드 하단에 위치한 **PCIe 스위치 칩셋**이 패킷의 목적지 주소를 검사하고. "어? 목적지가 시스템 RAM이 아니라 GPU네"라고 판단하여 스위치 내부에서 패킷의 방향을 꺾어 곧바로 GPU 슬롯으로 향하게 한다.
4. **VRAM 안착**: 패킷은 GPU 내부의 메모리 컨트롤러를 통해 HBM에 저장된다. 바운스 버퍼가 완전히 제거되었으므로 대역폭 낭비가 0이 되어 레이턴시는 하드웨어 한계치까지 감소한다.

<br>

## System lowlevel metric profiling

베이메탈 서버나 인스턴스에 접속했을때, 다중 GPU와 NUMA 노드, NIC, 스토리지 간에 물리적 배선 상태 토폴리지를 진단하는 예시를 보자

```bash
root@ai-server:~# nvidia-smi topo -m

        GPU0    GPU1    NIC0    CPU Affinity    NUMA Affinity
GPU0     X      PIX     NODE    0-15,32-47      0
GPU1    PIX      X      NODE    0-15,32-47      0
NIC0    NODE    NODE     X      16-31,48-63     1

Legend:
  X    = Self
  SYS  = Connection traversing PCIe as well as the SMP interconnect between NUMA nodes (e.g., QPI/UPI)
  NODE = Connection traversing PCIe as well as the interconnect between PCIe Host Bridges within a NUMA node
  PHB  = Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)
  PIX  = Connection traversing up to a maximum of 1 PCIe bridge (PCIe Switch)
```

- `GPU0`, `GPU1`의 관계(`PIX`): 두 GPU는 같은 PCIe 스위치 PIX에 묶여있고 이들끼리는 CPU를 거치지 않고 p2p 통신이 가능하다는 뜻이고 통신속도가 빠르다.
- `GPU0`과 `NIC0`(네트워크 카드)의 관계(`NODE`): 서로 다른 NUMA 노드 (GPU는 NUMA 0, NIC는 NUMA 1)에 묶여있다. 데이터가 이 둘 사이를 이동하려면 `SYS/NODE` 즉 호스트 CPU간의 버스를 건너야하므로 치명적인 NUMA 횡단 페널티가 발생한다. GPU Direct RDMA를 사용할 떄 최악의 성능이 나온다.

추가로 백엔드 인프라에서 학습 프로세스를 진행할때,

하드웨어 토폴리지를 고려해 운영체제 레벨에서 자원을 강제 할당하는 튜닝 방식이 있다.

`numactl`을 이용해 토폴로지 인식 프로세스 바인딩 (Affinity Pinning)이라고 하는데.

위 토폴로지 로그에서 확인했듯, 특정 GPU를 사용할때는 해당 GPU와 물리적으로 동일한 NUMA에 연결된 CPU코어와 시스템 RAM만 사용하도록 OS 스케줄러를 pinning 해야한다.

```bash
# 잘못된 실행 (OS가 스레드를 NUMA 0과 1에 멋대로 스케줄링하여 병목 발생)
python train.py --gpu 0

# 최적화된 실행: 
# GPU 0번은 NUMA 0번에 묶여 있으므로, 
# --cpunodebind=0 (CPU 코어도 0번 노드만 사용)
# --membind=0 (메모리 뱅크도 0번 노드만 사용) 옵션을 강제하여 횡단 페널티 원천 차단
numactl --cpunodebind=0 --membind=0 python train.py --gpu 0
```

**GDS 활성화 여부 모니터링도 할 수 있는데.**

추가로 NVIDIA에서 제공하는 GDS 벤치마크 툴 `gdsio` 를 통해서 스토리지 IO가 실제 cpu 바운스 버퍼를 우회하는지 검증한다.

커널의 `O_DIRECT` 플래그와 `cuFile` 이 정상적으로 맞물리면 cpu utilization은 5% 미만으로 떨어지면서 io 처리량은 GB/s 단위로 폭증하는 것을 시스템 모니터링 지표로 확인할 수 있다.