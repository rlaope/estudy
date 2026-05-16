# CUDA, Tensor Core

질문으로 시작해보자, cpu는 분기 예측 (Branch Prediction)과 거대한 L1/L2 캐시를 통해 단일 스레드의 지연시간을 최소화하는데 집중한다. 

반면 GPU는 수천 개의 스레드 컨텍스트를 스위칭하며 메모리 지연 시간을 숨기는 처리량 최적화 머신이다. 그렇다면 GPU 코어 내부에서 전통적인 스칼라 연산을 담당하는 CUDA코어와 AI연산의 핵심인 Tensor 코어는 물리적인 트랜지스터 레벨에서 어떤 구조적 차이를 가질까?

GPU 구조를 이해하려면 폰 노이만 아키텍처의 한계를 극복하기 위한 설계 철학 차이를 먼저 인지해야한다.

- **CUDA Core (Streaming Processor)**: 본질적으로 1클럭당 1개의 스칼라 연산(ex, Fused Multiply-Add, FMA)을 수행하는 단순한 산술 논리 연산장치 ALU와 부동소수점 연산장치 FPU의 결합체다. CPU의 SIMD(AVX) 명령어와 유사하게 동작하지면 GPU는 이를 하드웨어 스레드 단위로 쪼개어 수천개를 동시에 실행하는 SIMT(Single Instruction, Multiple Threads) 아키텍처를 취한다.
- **Tensor 코어 (Matrix Processing Unit)**: 행렬의 곱셈과 덧셈(Matrix Multiply-Accumulate, MMA)을 단일 하드웨어 명령어 레벨에서 수행하도록 설계된 특수 목적의 하드웨어 가속기 (시스톨릭 어레이 구조)이다. 1차원 스칼라 연산의 반복을 2차원 공간적 연산으로 전환하여 1클럭당 수십-수백개의 부동소수점 연산을 한번에 처리한다.


**물리적/구조적 병목 현상 정의를 해보자**

기존 CUDA 코어만으로 거대한 신경망 행렬 곱셈을 연산할 때 발생하는 물리적 병목은 단순히 ALU의 개수 부족이 아니라, **명령어 처리 오버헤드**와 **레지스터 대역폭 고갈**에 존재한다.

- **명령어 패치/디코드 전력 낭비 (Instruction Overhead)**: 4x4 행렬 곱셈을 CUDA 코어(스칼라)로 처리하려면 64번의 곱셈과 48번의 덧셈, 즉 최소 100개가 넘는 FMA 명령어가 필요하다. CPU나 GPU가 명령어를 L1 l-Cache에서 Fetch하고 Decode 하는 과정에서 막대한 에너지를 소모하는데, 실제 수학 연산에 쓰는 전력보다 명령어를 해석하는데 쓰이는 전력이 훨씬 더 커지는 오버헤드 병목이 발생한다.
-  **레지스터 파일 대역폭 병목**: CUDA 코어는 매 연산마다 피연산자를 레지스터에서 읽고, 결과를 다시 레지스터에 쓴다. 4 x 4 행렬 연산을 스칼라로 쪼개어 실행하면 수백번의 레지스터 읽기/쓰기 RW가 발생한다. 레지스터 파일은 SRAM으로 구성되어 매우 빠르지만, 면적이 크고 전력 소모가 극심하여 데이터 버스 병목을 유발한다.

<br>

## 해결 방식

NVIDIA는 위에서 정의한 병목들을 돌파하기 위해서 스칼라 중심의 CUDA 연산을 버리고, 하드웨어 공간 배열을 활용한 Tensor 코어 (Systolic Array 구조 활용)을 도입했다.

- **혼합 정밀도 Mixed Precision 및 HMMA 명령어 도입**: Tensor Core는 `HMMA`라고 하는 Half-Precision Matrix Multiply Accumulate 라는 단일 하드웨어 명령어를 사용한다.  $D = A \times B + C$ 연산을 단 1클럭(또는 파이프라인 소수 클럭)만에 수행한다. 명령어 1번만 Fetch, Decode 하면 되므로 프론트엔드 오버헤드가 극적으로 감소한다.
- **데이터 재사용을 통한 레지스터 접근 최소화**: Tensor 코어 내부는 연산기 MAC들이 Grid 형태로 물리적 배선이 연결되어있다. 하나의 MAC에서 계산된 부분합이 레지스터 파일로 돌아가지 않고 즉시 인접한 다음 MAC의 입력으로 흘러들어간다 이를 Systolic Flow라고 하고, 이를 통해서 레지스터 RW 횟수를 1/10 수준으로 줄이고 칩의 전력대비 성능을 극대화 했다(Perf/W)

### 동작 원리

행렬곱셈 GEMM 워크로드가 실행될 때, 데이터가 물리적 메모리를 거쳐 Tensor 코어에 도달하기까지의 경로를 상세히 추적한다.

1. **HBM에서 L2 캐시로의 이동:** GPU의 수천 개 코어 (SM, Streaming Multiprocessor)가 작업을 할당받으면, 초광대역 버스 (예: 5,120-bit 메모리 버스)를 통해 HBM(고대역폭 메모리)에 저장된 가중치 Weight와 입력 행렬 (Activation) 데이터가 GPU 다이 (Die) 중앙에 위치한 거대한 공유 L2 캐시를 로드시킨다.
2. **L2 캐시에서 L1/Shared Memoryh 로드**: 각 SM(Streaming Multiprocessor) 내부의 워프 스케줄러 (Warp Scheduler)가 메모리 로드 명령을 내린다. 행렬을 잘게 쪼갠 타일 단위의 데이터가 L2에서 SM 내부의 L1 캐시겸 공유 메모리(Shared Memory)로 물리적 이동한다. 이 공유 메모리는 cpu의 l1 데이터 케시와 달리 프로그래머가 수동으로 데이터 배치를 통제할 수 있는 초고속 SRAM 공간이다.
3. **공유 메모리에서 레지스터로의 ldmatrix 전송:** Tensor 코어가 연산을 시작하기 직전, ldmatrix 명령어가 실행된다 (Load Matrix) 이 명령어는 일반적인 스칼라 로드와 달리 공유 메모리에 있는 2D 형태의 데이터 블록을 워프(32개의 스레드 묶음)내의 레지스터 파일들에 나누어 일괄적으로 꽂아 넣는다.
4. **레지스터에서 Tensor 코어로의 데이터 인젝션:**: `HMMA` 명령어가 디코드되면 레지스터 파일에 저장된 FP16 or BF16 형식의 A, B 피연산자 행렬 데이터가 크로스바 스위치를 타고 Tensor 코어의 시스톨릭 어레이 입력단으로 라우팅된다.
5. **Tensor 코어 내부의 공간적 연산 (Systolic Flow)**: 클럭이 뛸때마다 데이터는 어레이 내부의 MAC 유닛을 폭포수 처럼 통과한다. A 행렬 데이터는 가로로, B 행렬 데이터는 세로로 흐르며 교차점에서 FP16 정밀도로 곱해진다. 그 곰ㅂ셈 결과는 하단의 덧셈기로 전달되어 FP32 정밀도로 누적 Accumulate 된다. 연산 도중에는 레지스터 파일에 접근하지 않는다.
6. **결과 라이트백(Write-back)**: 최종적으로 계산된 4x4 혹은 8x4 크기의 FP32 부분합 행렬 D가 워프 레지스터 파일로 한 번에 쓰여진다. 이후 이 결과는 다시 공유 메모리를 거쳐 HBM으로 저장된다.

### 정량적 성능 지표와 수학적 모델링

CUDA 코어와 Tensor 코어의 하드웨어 연산 능력을 수학적으로 모델링하여 차이를 정령화 해보자 (NVIDA Ampere A100 기준)

- **CUDA 코어 FP32 FLOPS 계산식**: $\text{FLOPS}_{\text{CUDA}} = (\text{SM 개수}) \times (\text{SM당 CUDA 코어 수}) \times (\text{클럭 속도}) \times 2 \text{ (FMA 연산)}$
A100의 경우: $108 \text{ SMs} \times 64 \text{ Cores/SM} \times 1.41 \text{ GHz} \times 2 \approx 19.5 \text{ TFLOPS}$
- **Tensor 코어 FP16/FP32 FLOPS 계산식**: Tensor 코어는 1클럭당 $4 \times 4 \times 4 = 64$개의 FMA(128 연산)를 수행합니다. A100의 3세대 Tensor 코어는 SM당 4개가 있으며 파이프라인 구조가 확장되어 클럭당 처리량이 더 높다.
$\text{FLOPS}_{\text{Tensor}} = (\text{SM 개수}) \times (\text{SM당 Tensor 코어 수}) \times (\text{클럭 속도}) \times (\text{클럭당 연산 횟수})$
A100의 경우: $108 \text{ SMs} \times 4 \text{ Cores/SM} \times 1.41 \text{ GHz} \times 512 \approx 312 \text{ TFLOPS}$

결론적으로 Tensor 코어를 활성화하는순간 트랜지스터 클럭을 올리지 않고도 명령어 오버헤드와 레지스터 접근을 생략한 구조적 혁신으로 행렬 연산이 약 16배정도 비약적으로 상승한다 19 vs 312

<br>

## 레이어 프로파일링 설정

소프트웨어 PyTorch, CUDA 계층에서 이 거대한 하드웨어를 어떻게 통제하고 상태를 관측하는지 알아보자

**Tensor 코어를 강제 활성화**하는 방법이 있다.

과거에는 수동으로 FP16 캐스팅을 했었는데, 현재는 pytorch에서 AMP(Automatic Mixed Precision)를 통해 하드웨어 데이터 패스를 Tensor 코어로 유도한다.

```py
import torch
# 1. Ampere 이상 아키텍처에서 FP32 입력을 TF32(Tensor Float 32) 형식으로 
# Tensor 코어에 태우도록 허용 (성능 3~4배 증가, 정밀도 손실 미미)
torch.backends.cuda.matmul.allow_tf32 = True 

# 2. BFloat16을 사용하여 Tensor 코어 데이터 패스 명시적 활성화
with torch.autocast(device_type="cuda", dtype=torch.bfloat16):
    output = model(input_tensor) # 내부 GEMM 연산이 HMMA 명령어로 컴파일됨
```

**Nsight Compute를 통한 하드웨어 파이프라인 프로파일링**

CUDA 코어와 Tensor 코어의 실제 점유율을 확인하려면 NVIDIA Nsight Compute ncu 명령을 사용해 GPU 파이프라인 메트릭을 추출한다.

- `sm__inst_executed_pipe_fma.avg.pct_of_peak`: 전통적인 CUDA 코어(FMA 파이프라인)의 활용률. 이 값이 높고 아래 값이 낮다면 AI 모델이 비효율적으로 스칼라 연산을 하고 있다는 증거.
- `sm__inst_executed_pipe_tensor.avg.pct_of_peak`: Tensor 코어 파이프라인의 활용률. 최적화된 대규모 언어 모델(LLM) 학습 시 이 값이 60~80% 이상으로 유지되어야 병목이 없는 상태.

**메모리 정렬 제약을 알아보면** 텐서 코어에서 하드웨어 버스는 8 x 8, 16 x 16 등의 청크 단위로 데이터를 빨아드린다 따라서 파이토치 모델의 Linear 레이어 `in_features`나 `out_features` 그리고 `batch_size`가 8배수 또는 16배수가 아닐경우 컴파일러는 어쩔 수 없이 Tensor 코어를 포기하고 느린 CUDA 코어로 Fallback 하거나 0으로 패딩하는 오버헤드를 발생시킨다. CPU 엔지니어가 캐시라인 정렬을 신경쓰듯 AI 엔지니어는 행렬 차원의 8/16 배수 정렬을 반드시 신경써야한다.