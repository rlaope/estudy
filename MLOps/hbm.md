# HBM, Roofline 모델

> 연산 병목인가, 메모리 병목인가?

질문으로 시작해보자.

현대 GPU 연산 유닛 Tensor 코어는 1초에 수백조번의 부동 소수점 연산을 (TFLOPS) 수행할 수 잇을정도로 발전했다.

그렇다면 LLM을 추론할 때 연산 속도를 더 높이는 최신 GPU를 도입해도 성능 향상이 미미할 때가 있다면, 그 이유는 무어싱ㄹ까?

왜 시스템은 항상 메모리가 데이터를 가져오기를 **기다리고 있을까**

AI 시스템의 성능 최적화를 위해서는 소프트웨어가 처리하는 연산량과 메모리 데이터 전송량 사이의 물리적 상관 관계를 정량화 해야한다.

- **산술 강도 (Arithmetic Intensity, AI)**: 1 byte 데이터를 물리적 메모리에서 연산 코어로 가져왓을 때, 해당 데이터로 몇 번의 부동 소수점 연산 (FLOPs)을 진행하는 지 나타내는 비율이 산술 강도다. (FLOPs/Byte)
- **루프라인 모델**: Roofline Model은 하드웨어의 최대 연산 성능 Compute Roof와 최대 메모리 대역폭 Memory Roof 를 기준으로 현재 실행중인 워크로드(알고리즘)가 하드웨어 한계 대비 어느정도 성능을 내고 있으며, 병목의 원인이 연산인지 메모리인지 시각적/수학적으로 진단하는 평가 모델이다.
- **HBM(High Bandwidth Memory)**: 기존 GDDR 메모리의 대역폭 한계를 극복하기 위해, DRAM Die를 수직으로 적층하고 GPU 다이와 매우 가까운 실리콘 인터포저 (Silicon Interposer) 위에 올려 데이터 버스의 폭을 극단적으로 넓힌 차세대 메모리 아키텍처다.

<br>

## 문제 정의

하드웨어 진화 과정에서 연산 유닛의 발전 속도가 메모리 대역폭의 발전 속도를 압도하면서 치명적인 구조 병목이 발생했다.

- **GDDR의 Pin 및 Trace(배선)의 한계**: GDDR 메모리는 PCB(메인보드) 기판을 통해 GPU와 연결된다. 물리적인 칩의 외각선 한계로 인해 데이터 핀을 384개나 (384-bit) 이상 늘리기 매우 어렵다. 대역폭을 높이려면 클럭 주파수를 올려야하는데, 이는 전력 소모와 발열을 기하급수적으로 증가시킨다.
- **메모리 장벽 (Memory Wall) 현상**: 딥러닝 워크로드 중 특히 트랜스포머 모델의 추론 단계는 산술 강도가 매우 낮다. 즉 거대한 가중치 데이터를 메모리에서 한 번 읽어온 뒤 단순 곱셈 한 번만 하고 버리는 과정을 반복한다. 하드웨어 연산기가 아무리 빨라도 데이터를 공급받지 못해 GPU가 유휴 상태에 빠지는 Memory-Bound 상태가 시스템 주된 병목으로 고착화 되어있다.

### Solution

이러한 메모리 병목(Rooflinke의 경사면 한계)을 극복하기 위해 물리적인 칩 패키징 아키텍처를 전면 재설계했다.

- **2.5D 패키징 및 TSV (Through SIlicon Via):** DRAM 칩들을 PCB에 나열하지 않고 위로 쌓아 올린뒤 3D, 머리카락보다 얇은 미세한 구리 기동 TSV으로 칩에 직접 구멍을 뚫어 수천 개의 상하 배선을 연결한다.
- **실리콘 인터포저 (Silicon Interposer):** GPU칩과 HBM칩을 일반 메인보드가 아닌 미세 공정에 적용된 실리콘 칩 인터포저 위에 나란히 배치한다 (2.5D)이를 통해 GPU와 메모리간의 물리적 거리가 밀리미터 단위로 압축된다.
- **초광대역 저속 버스 (Ultra-Wide Bus)**: HBM은 384-bit 버스 대신 4096-bit 또는 5120-bit의 거대한 버스 인터페이스를 가지고, 클럭 속도를 GDDR 보다 낮추어 전력 소모와 발열을 잡으면서도, 한 번에 이동하는 데이터의 차선을 수십배로 늘려 초당 테라바이트 단위의 메모리 대역폭 상향을 이뤄냈다.

<br>

## 동작 원리

HBM 구조에서 데이터가 물리적으로 어떻게 GPU 내부로 진입하는지 패키징 레이어 관점에서 추적한다.

1. **DRAM 셀 배열 I/O**: HBM 스택의 가장 꼭대기 층에 있는 DRAM 셀에서 데이터 읽기 요청이 처리된다.
2. **TSV 관통:** 데이터 PCB의 외곽 핀으로 나가는 대신, 칩 자체에 뚫려있는 TSV(Through Silicon Via) 마이크로 채널을 타고 수직으로 빠르게 하강하여 HBM의 가장 아래층인 베이스 로직 다이에 도달한다.
3. **마이크로범프(Microbump) 통과:** 베이스 다이의 밑면에는 일반적인 칩 핀보다 훨씬 조밀한 수천 개의 마이크로범프가 있다. 데이터는 이 범프를 거쳐 실리콘 인터포저로 진입한다.
4. **실리콘 인터포저 라우팅:** 실리콘 인터포저 내부에 새겨진 수천 가닥의 초미세 배선을 타고 수 밀리 미터 옆에 위치한 GPU 다이 메모리 컨트롤러 PHY로 직접 전달된다.
5. **GPU L2 캐시 및 크로스바 진입**: GPU 메모리 컨트롤러를 통과한 데이터는 즉시 GPU 다이 정중앙을 가로지르는 대형 크로스바 스위치를 통해 거대한 L2 캐시에 안착하며 이후 SM으로 분배된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2Fbel0bR%2FbtsI8knD7TP%2FAAAAAAAAAAAAAAAAAAAAAPYSTskg_LzjUNsuf4e43WTrnBhjvhiVmmEw52K6IRoU%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1780239599%26allow_ip%3D%26allow_referer%3D%26signature%3DwnMXvial9w8%252F6DrDzpyj%252FxO341k%253D)

### profiling

NVIDIA Nsight Compute `ncu`를 사용해 특정 커널이 Compute-Bound인지 Memory-Bound인지 식별하는 Roofline 분석 로그다.

```bash
root@ai-node01:~# ncu --metrics sm__throughput.avg.pct_of_peak_sustained_elapsed,dram__throughput.avg.pct_of_peak_sustained_elapsed --section SpeedOfLight_RooflineChart python inference.py

==PROF== Connected to process 15024
==PROF== Profiling "attention_kernel" - 1 of 1

Section: GPU Speed Of Light Roofline Chart
---------------------------------------------------------------------- --------------- ------------------------------
Metric Name                                                                Metric Unit                    Metric Value
---------------------------------------------------------------------- --------------- ------------------------------
sm__throughput.avg.pct_of_peak_sustained_elapsed                             %                              12.45
dram__throughput.avg.pct_of_peak_sustained_elapsed                           %                              94.82
---------------------------------------------------------------------- --------------- ------------------------------

[Warning] This kernel is Memory Bound. 
The memory bandwidth utilization (94.82%) is significantly higher than the compute utilization (12.45%). 
Arithmetic Intensity is 0.82 FLOPs/Byte, which falls under the Memory Roof slanted line.
Consider using operator fusion or shared memory to increase data reuse.
```

- `dram__throughput 94.82%`: HBM 물리적 최대 대역폭 한계치 (A100기준 1.5TB/s)의 약 95퍼센트를 사용하고 있고 메모리 컨트롤러가 Starvation 상태다.
- `sm__throughput 12.45%`: 연산코어 Tensor/CUDA는 최대 성능이 12.45%만 가동되고 있다는 뜻이고 데이터가 도착하기를 기다리며 노는 시간이 많다는 뜻이다.

결론적으로 이 커널은 완벽한 Memory Bound 상태이고 이 상황에서는 GPU의 클럭을 오버클럭하거나 Tensor 코어수가 더 많은 GPU로 교체해도 성능은 오르지 않는다.

오직 HBM3, HBM3e 등 메모리 대역폭이 더 넓은 하드에ㅜ어로 교체하거나 산술강도 AI를 높이는 소프트웨어 최적화만이 답이다.

추가로 시스템 엔지니어나 ai 엔지니어가 메모리 병목을 피하고 산술 강도를 억지로 높여 roofline의 우측 compute - bound 영역으로 밀어내려면 다음 최적화 기법을 써볼 수 있따

#### 연산자 융합 Operator Fusion

여러개의 개별 연산 `MatMul` ->  `Scale` -> `Mask` -> `Softmax`를 별도로 싱행하면, 매 단계에 HBM에서 데이터를 읽고 쓰는 IO가 발생하여 심각한 Memory-Bound에 빠진다. FlashAttention이나 Triton 커널을 사용하여 이 단계들을 하나의 거대한 커널로 합치면 중간 결과를 HBM에 쓰지않고 SM 내부의 초고속 SRAM에 보존한채 연산을 끝낼 수 있다. 이를 통해 산술 강도의 분모를 급감시켜 병목을 해소한다.

FLOPs/Byte에서 Byte가 줄어드니까 

#### Pytorch JIT 컴파일러 활용 (torch.compile)

PyTorch 2.0 이상의 `torch.compile` 메커니즘을 사용하면 내부적으로 OpenAI Triton이 백엔드로 작동하여 자동으로 커널 퓨전을 수행한다.

```py
import torch

def memory_bound_logic(x, y):
    # 파이썬 레벨에서는 3번의 HBM R/W가 발생하는 비효율적 코드
    a = torch.sin(x)
    b = torch.cos(y)
    return a + b

# 컴파일을 통해 3개의 연산을 단일 CUDA 커널로 병합(Fusion)
# HBM 로드 1회, 내부 SRAM 연산 후 HBM 저장 1회로 I/O 최소화
optimized_logic = torch.compile(memory_bound_logic)

x = torch.randn(10000, 10000, device="cuda")
y = torch.randn(10000, 10000, device="cuda")
output = optimized_logic(x, y)
```

#### 양자화를 통한 데이터 대역폭 최적화

가중치와 활성화 함수 데이터를 FP16(2Bytes)에서 INT8(1Byte) 또는 FP8(1Byte)로 양자화하면, HVM에서 읽어와야할 데이터량이 절반으로 줄어든다.

이는 물리적 메모리 대역폭을 2배로 늘린것과 동일안 효과를 내어 Roofline 모델상에서 워크로드 위치를 Compute Bound 영역으로 이동시킨다.

<br>

## GDDR

위에서 언급한 GDDR Graphics Double Data Rate는 우릭 흔히 아는 그래픽 카드에 들어가는 비디오 메모리 VRAM 이다.

집에서 쓰는 RTX 3080, 4090이나 플레이스테이션 같은 게임기에 들어가는 렘이 모두 GDDR인거임

보통 컴퓨터 조립할때 메인보드에 꽂는 시금치 램픙ㄹ DDR(DDR4, DDR5)라고 부르는데 이름은 비슷하지만 설계 목적은 다르다.

- **DDR(CPU용 램): 반응속도 올인**: 소량의 데이터를 지연없이 빠르게 실어 나르는데에 최적화 되어있고 CPU가 이리저리 코드를 건너뛰며 일을 할 때 빠르게 응답해야하니까
- **GDDR (GPU용 램) 데이터 처리량 올인**: GPU는 모니터 화면에 수백만개의 픽셀을 동시에 칠해야하니까 한 번에 엄청나게 많은 데이터를 쏟아붇는데 최적화 되어있다 반응속도는 DDR보다 약간 느리지만 데이터의 양이 나르는게 압도적으로 많다.

HBM과는 무슨 관계냐면 GDDR도 최신 모델은 빠른데, AI 모델이 수백억개의 파라미터를 가지게 되면서 GDDR도 벅차게 된거고 GDDR은 메모리 칩을 그래픽 카드 기판 PCB위에 평면으로 쫙 깐 구조였고 그러다 보니 Bus 384-bit 늘리기가 물리적으로 힘들었다 라는 문제가 있었고

이 평면적 한계를 극복한게 메모리칩을 위로 쌓고 4000개 이상으로 차선을 뚫고 gpu 코어 옆에 붙이자에서 나온게 HBM이다.

GDDR은 게이밍 일반 작업용 GPU mem이고 HBM은 ai 연산 특수 메모리라고 생각하면 됨