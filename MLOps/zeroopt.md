# ZeRO Optimizer (DeepSpeed, FSDP)

수백억개의 파라미터를 가진 거대 언어 모델 LLM을 학습하려면 테라바이트 단위의 메모리가 필요하다.

하지만 최고급 GPU인 A100, H100 조차, VRAM 용량은 80GB에 불과한다.

단일 GPU 메모리 모델 조차 다 올라가지 않는 물리적 한계를 분산 시스템 엔지니어들은 어떻게 극복할까?

GPU 분산 학습 모델에서 여러 장치에 배치할 때 발생하는 메모리 낭비를 물리적으로 제거하는 것이 **ZeRO(Zero Redundancy Optimizer)** 아키텍처 핵심이다.

- **모델 상태 (Model States)**: 학습시 GPU HBM 공간을 영구적으로 점유하는 3대 핵심 데이터다.
  - **Paramters**: 모델의 가중치 데이터 $W$
  - **Gradients**: Backward시 계산된 미분값 ($\nabla W$)
  - **Optimizer States**: Adam등 옵티마이저가 유지하는 모멘텀과 분산 데이터
- **전통적인 데이터 병렬화 DP(Data Parallel)**: 모든 GPU가 모델 상태의 완벽한 복사본 100%를 동일하게 HBM에 보유하고 학습할 데이터 Batch만 쪼개어 연산하는 방식이다.
- **ZeRO**: Microsoft DeepSpeed 팀이 고안한 아키텍처로 모든 GPU가 모델의 복사본을 중복해서 갖는 대신, 전체 모델 상태를 N개의 GPU 개수만큼 물리적으로 조각내어 Shard 각 GPU의 HBM에 분산 저장하는 기술이다. PyTorch의 FSDP(Fully Sharded Data Parallel)도 이 ZeRO-3 개념의 네이티브 구현체다.

<br>

## 물리적/구조적 현상 정의

전통적인 Data Parallel DP방식을 사용할 때 HBM에서 발생하는 치명적인 메모리 중복 및 OOM 병목이다.

**옵티마이저 상태 메모리 폭식:** 파라미터가 70B LLaMA 모델을 16 bit(FP16, 2bytes)로 학습한다고 가정해보자

- 파라미터: $70 \times 10^9 \times 2 \text{ Bytes} \approx 140 \text{ GB}$
- 기울기: 파라미터와 동일한 크기 $\approx 140 \text{ GB}$
- Adam 옵티마이저 (FP32 기준): 모멘텀과 분산을 모두 저장해야 하므로 파라미터 크기의 4배 $\approx 280 \text{ GB}$ 총 560GB의 HBM이 피룡하고 80GB GPU 1장으로는 어림도없고 8장을 묶어 전통적인 DP를 돌려도 모든 GPU가 560GB를 통째로 가져가야하므로 하드웨어 런칭 즉시 CUDA OOM 에러가 발생하며 커널이 죽는다.

메모리 장벽에 의한 모델 확장성 한계: tensor 코어는 놀고있는데, VRAM에 데이터를 적재할 공간이 없어 학습 자체를 시작할 수 없는 하드웨어적 교착상태에 빠진다.

### Solution

ZeRO는 통신 대역폭(NVLink, RDMA)를 희생하는 대신, HBM 공간의 중복을 제거하여 연산가능한 모델의 크기를 무한대로 확장한다. 분할 수준에 따라 3단계로 나뉜다.

- **ZeRO-Stage 1(옵티마이저 상태 분할):** 가장 많은 메모리를 차지하는 옵티마이저 상태만 GPU 개수 만큼 쪼개어 나누어 가진다. 메모리 사용량이 크게 줄어든다.
- **ZeRO-Stage 2(기울기 분할 추가):** 옵티마이저 상태와 더불어, 역전파가 끝난 기울기 데이터도 N등분 하여 각 GPU가 자신의 담당 구역만 소유한다.
- **ZeRO-Stage 3 / FSDP (파라미터 분할까지 완벽 적용)**: 파라미터 가중치까지 모조리 쪼개어 분산시킨다 단일 gpu안에는 전체 모델의 1 / N 조각만 존재하고 HBM 용량의 물리적 한계까지 완벽하게 파괴되면서 GPU를 클러스터에 추가할 수록 적재할 수 있는 모델 크기가 선형적으로 커진다.

### RDMA

서버 A에서 서버 B로 보낼때 일반적인 TCP/IP 소켓 통신을 할때 `유저 공간 버퍼 -> os tcp ip stack -> nic`를 거치고 상대방 서버에서도 역순으로 커널을 거쳐 올라간다. 이 과정에서 cpu가 개입하고 복사가 발생하면서 느려지는데,

RDMA Remote Direct Memory Access는 이 OS 커널과 CPU를 완전히 Bypass하는 네트워크 기술이다.

서버 A의 랜카드가 서버 A의 메모리에서 데이터를 직접 퍼다가 서버 B의 랜카드를 통해 서버 B 메모리 RAM. GPU VRAM 특정 주소에 다이렉트로 꽂아버린다. CPU는 전혀 관여하지 않으므로 컨텍스트 스위칭 오버헤드가 없고 핑이 극단적으로 낮아진다. AI 클러스터 처럼 여러대의 서버에 있는 GPU들끼리 데이터를 교환할때 이 기술이 없으면 네트워크 병목 때문에 아예 굴러가지 않는다.

### 파라미터와 기울기

- **parameter(Weight, $W$)**: AI 모델이 가지고 있는 수많은 변수 가중치들이다.
- **기울기(Gradient, $\nabla W$)**: 오차를 줄이기 위해 각 파라미터를 어느 방향으로 얼만큼 수정해야하는지 알려주는 업데이트 지시값이다.

예를들어 행렬 형태의 파라미터 1억개가 있다면 학습 과정에서 1번 파라미터는 +0.01로 올리고 2번 파라미터는 -0.05 내리고 하는식으로 1억개 파라미터 각각에대한 수정 지시값이 정확히 1:1로 짝지어져야한다.

즉 $\text{새 파라미터} = \text{기존 파라미터} - (\text{학습률} \times \text{기울기})$ 라는 연산을 해야하므로 기존 파라미터 행렬과 기울기 행렬의 shape와 개수가 완벽하게 똑같아야지만 수학적으로 성립하고 그래서 파라미터가 140GB면 담은 기울기도 140GB가 된다.


<br>

## 하드웨어 데이터 패스 및 메모리 계층 동작 원리

가장 극단적인 분할 **ZeRO-3(FSDP)** 환경에서 1/N 조각만 가진 GPU가 어떻게 전체 네트워크 연산 foward backward를 끊임없이 수행하는지 추적해보겠다. (8-GPU 가정)

1. **순전파 진입 전 대기**: GPU0은 모델의 1번 레이어 파라미터중 1/8 조각만 HBM에 가지고 있고 연산을 하려면 나머지 7/8이 필요하다
2. **All-Gather 통신 트리거 (JIT Fetch)**: 연산코어 SM을 기반으로 1번 레이어가 계산을 시작하기 직전 하드웨어 통신망(NVLink or RoCEv2)를 통해 나머지 7대의 GPU에게 파라미터 조각을 요청한다. 네트워크 인터페이스 카드 NIC, PCIe/NVLINK 스위치를 타고 파라미터들이 GPU 0번의 HBM 빈공간으로 쏟아져 들어온다.
3. **Tensor 코어 연산:** 1번 레이어의 파라미터가 100% 온전하게 조립된 짧은 찰나의 순간 Tensor 코어가 `HMMA` 명령을 통해 행렬 곱셈 연산을 폭풍처럼 수행한다.
4. **메모리 폐기 Eviction/Free**: 1번 레이어의 연산이 끝나는 즉시, GPU는 0번은 남에게서 빌려온 7/8 파라미터 조각들을 HBM에서 물리적으로 삭제하여 메모리를 비운다.
5. **Backward Pass 및 Reduce-Scatter**: 역전파시에도 동일하게 파라미터를 임시로 가져와 Gather 기울기를 계산한다. 계산된 전체 기울기는 다시 각 담당 GPU에 분산시켜 보내고 자신은 남의 기울기 조각을 즉시 삭제한다.
6. 결론저긍로 HBM을 절약한 대가로 노드 간 끊임없는 파라미터 패킷 이동이 발생한다. 즉 GPU 메모리 용량 병목을 네트워크 대역폭 병목으로 치환한 방식인 것이다.

<br>

### 로우레벨 메트릭 및 프로파일링 로그 분석

DeepSpeed를 사용하여 학습을 구동할 때, 노드 메모리의 물리적 상태를 진단할 수 있는 터미널 레벨의 초기화 로그다

```
[INFO] [logging.py:96:log_dist] [Rank 0] DeepSpeed info: version=0.10.0, git-hash=unknown, git-branch=unknown
[INFO] [engine.py:3213:_configure_zero_optimizer] ZeRO Stage 3 is enabled.
[INFO] [engine.py:3214:_configure_zero_optimizer] Partitioning Parameters, Gradients, and Optimizer States.

# 파라미터 분할 후 단일 GPU의 메모리 할당 상태 리포트
[INFO] [memory_stats.py:45] OOM Catch: False
[INFO] [memory_stats.py:46] --------------------------------------------------
[INFO] [memory_stats.py:47] Allocated Memory (after ZeRO-3 init): 
[INFO] [memory_stats.py:48]     Total Model Parameters: 70.00 B
[INFO] [memory_stats.py:49]     Total Allocated per GPU: 18.25 GB
[INFO] [memory_stats.py:50]     Remaining HBM Capacity: 61.75 GB
[INFO] [memory_stats.py:51] --------------------------------------------------
```

단일 GPU로는 140GB가 넘어 로드조차 불가능했던 70B 모델이 ZeRO Stage 3 초기화 직후 단 18.25GB의 HBM만 점유 (`Allocated per GPU`) 하고 안착했다. 

나머지 61GB의 여유 공간은 런타임에서 All-Gather로 다른 GPU의 조각을 임시로 가져오거나 활성화 데이터를 담는 버퍼로 쓰이게 될것이다.

분산 시스템에서 OOM없이 정상 궤도에 올랐음을 증명한다.

<br>

### 실무 적용 레이어 및 프로파일링 설정

AI 인프라 환경에서 분산 학습 스크립트를 구동할때, DeepSpeed 엔진이 주입하는 JSON 설정 파일 (`ds_config.json`)을 통해 하드웨어 동작 방식을 직접 통제한다.

```json
{
  "train_batch_size": 256,
  "gradient_accumulation_steps": 4,
  "zero_optimization": {
    "stage": 3,
    "contiguous_gradients": true,
    "overlap_comm": true,
    "reduce_scatter": true,
    "reduce_bucket_size": 500000000,
    "allgather_bucket_size": 500000000,
    "offload_optimizer": {
      "device": "cpu",
      "pin_memory": true
    },
    "offload_param": {
      "device": "nvme",
      "nvme_path": "/mnt/nvme_storage",
      "buffer_count": 5,
      "buffer_size": 100000000
    }
  },
  "fp16": {
    "enabled": true
  }
}
```

- `stage: 3`: 파라미터, 기울기, 옵티마이저 모두 분할해 HBM 한계를 해제
- `overlab_comm: true`: 하드웨어 최적화 핵심이다 Tensor 코어가 L번째 레이어 연산을 수행하는동안, 백그라운드 네트워크 스레드가 L _ 1 번째 레이어 파라미터를 미리 All-Gather 통신으로 가져온다. 연산과 통신을 중첩(overlapping)시켜 네트워크 대역폭 병목을 마스킹한다.
- `offload_optimizer`, `offload_param` (ZeRO-Infinity): GPU 개수가 부족해도 분할을 해도 HBM이 꽉 찬다면, 안 쓰는 조각 호스트 CPU의 시스템 RAM (`device: "cpu"`) 이나 심지어 메인보드에 꽂힌 NVMe SSD(`device: "nvme"`)로 강제 offload시킨다. 학습 속도는 PCIe 버스를 타기 때문에 느려지지만 물리적인 VRAM 한계를 뛰어넘어 초거대 모델을 억지로 구동시킬 수 있는 엔지니어링 생존 기법이다.

