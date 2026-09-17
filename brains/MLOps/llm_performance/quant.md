# 4. Quantization과 Multi-GPU Parallelism

LLM의 파라미터가 크기가 수백억 개(70B, 405B등)로 커짐에 따라, 이를 단일 GPU에 올리는 것은 물리적으로 불가능하거나 가성비가 극도로 떨어지는 일이 되었다.

인프라 비용을 현실적인 수준으로 낮추면서 서비스 처리량을 극대화하기 위한 두 가지 핵심 축이 바로 **양자화(Quantization)**와 **다중 GPU 병렬 처리(Multi-GPU Parallelism)**이다.

## 핵심 질문: 모델 품질 저하를 최소화하면서 GPU Memory와 추론 비용을 얼마나 줄일 수 있는가

**메모리와 비용의 직결:** Llama-3-70B 모델을 원래 포멧(FP16, 파라미터당 2바이트)으로 로드하려면 가중치 자체에만 $70 \times 2 = 140\text{ GB}$의 VRAM이 필요하다. 여기에 추론을 위한 KV Cache 공간까지 감안하면 최소 NVIDIA A100(80GB) 2대 또는 H100 2대가 필수적이다. 이는 곧 막대한 인프라 비용으로 이어진다.

**트레이드오프의 극복:** 가중치를 4비트(INT4)로 양자화하면 모델 크기는 $70 \times 0.5 = 35\text{ GB}$로 줄어들어, 단일 A100(40GB/80GB) 장비 1대만으로 서빙이 가능해진다.

장비대수가 절반 이하로 줄어드므로 **추론 비용은 60~70% 이상 절감된다.** 과거에는 4비트 전환시 모델의 지능(Perflexity)이 무너지는 현상이 있었으나. 최근의 **AWQ(Activation-aware Weight Quantization)**나 **GPTQ** 같은 진보된 알고리즘 덕분에 실제 대화 품질(정확도) 저하를 1~2% 이내로 방어할 수 있게 되었다.

<br>

## 데이터 타입 Quantization 기법 분석

양자화는 연속적인 고정밀도 실수 Float 표현을 이산적인 저정밀도 표현 Integer or 소형 Float으로 매핑하여 메모리를 아끼는 기술이다.

### 데이터 타입별 특징

- **FP16 vs BF16 (16-bit)**: 기본 학습 및 추론 데이터 타입이다. FP16은 소수점 정밀도가 높지만 표현 범위가 좁아 Gradient Overflow가 발생하기 쉽다. BF16은 FP32와 동일한 지수부(Exponent) 범위를 가져 LLM 학습 ㅂ및 추론 안정성에 훨씬 뛰어나다.
- **FP8 (8-bit)**: Hopper(H100) 아키텍처부터 본격 지원하는 포맷으로 정수형 양자화 INT8와 달리 소수점 표현이 유지되므로 수치적 안정성이 높고 변환 오버헤드 Dequantization 없이 하드웨어 가속을 직접 받기 때문에 최근 트렌드다.
- **INT8 vs INT4(정수형 양자화)**
  - **Weight-Only 양자화:** 가중치만 저정밀도로 저장하고, 연산 시점에만 FP16으로 복원하여 연산한다. 메모리 대역폭은 획기적으로 아끼지만 복원 연산 오버헤드가 존재한다.
  - **Weight-Activation(W8A8) 양자화**: 가중치와 활성화 함수 출력값 모두를 정수로 변환하여 변환 없이 직접 하드웨어 INT8 텐서 코어로 연산한다. 구현이 까다롭지만 연산 속도 자체가 빨라진다.

### 고급 양자화 알고리즘 GPTQ vs AWQ

**GPTQ(Layer-wise Quantization)**은 역전파 연산 없이 입력데이터에 따른 하중 오차를 최소화하기 위해서 레이어 단위로 가중치를 깎아나가는 방식이다. 연산량이 다소 많지만 고정된 데이터셋 기준 최적화가 우수하다.

**AWQ(Activation-aware Weight Quantization)**은 모든 파라미터가 똑같이 중요하지 않다는 점에 착안했고 모델의 Activation 값을 관찰해 **상위 1% 중요한 가중치 Salient Weight는 FP16 정밀도로 보존 나머지 99%는 4비트로 깎아내는** 방식이다. 품질 저하가 극도로 적어 실제 프로덕션 환경에서 가장 선호된다.


<br>

## 다중 GPU 병렬 처리 아키텍처

모델이 단일 GPU 메모리를 초과하거나 처리량을 늘리기 위해 다중 GPU로 묶을때 NCCL(NVIDIA Collective Communication Library) 기반의 세 가지 병렬화 전략이 사용된다.

### Data Parallelism (DP)

동일한 모델 가중치를 모든 GPU에 각각 복제해두고, 서로 다른 입력 배치 데이터를 나누어 병렬 처리한다.

추론 관점에서 모델이 단일 GPU에 들어갈 수 있을 때 사용하며, Throughput을 선형적으로 올리기위한 복제전략 replication에 가깝다.

### Tensor Paralleism (TP) - Intra Node 최적

하나의 레이어 내 행렬 곱 연산 Y = X x W 자체를 쪼개어 여러 GPU가 동시에 계산하게 한다.

- **Column Parallel**: 가중치 행렬 W를 열 방향으로 쪼개어 각각 연산한뒤 결과를 옆으로 붙인다. Concat
- **Row Parallel**: 가중치 행렬을 행 방향으로 쪼개어 연산한뒤 GPU간 All-Reduce 통신을 통해 더한다.

매 Transformer 블록마다 GPU간 동기화 통신 All-Reduce이 빈번하게 일어난다.

따라서 NVLink와 같이 대역폭이 극도로 높은 단일 서버 내(Intra-node) GPU 환경에서만 효율적이며 네트워크 스위치를 거치는 노드간 확장에는 적합하지 않다.

### Pipeline Parallelism (PP) - Inter Node 최적

모델의 레이어들을 순서대로 쪼개어 GPU에게 분배하는 방식 (ex. 1~20번 레이어는 GPU0, 21~40은 GPU1)

앞 단계의 gpu 출력이 다음 gpu의 입력이 되므로 구조적으로 노는 시간인 Pipeline bubble이 발생한다.

이를 줄이기 위해 마이크로 배치 스케줄링이 필수적이다.

통신량이 상대적으로 적어 네트워크 스위치로 연결된 서로 다른 서버간 병렬화에 적합하다.

<br>

## Hands-on: vLLM을 활용한 AWQ 양자화 및 Tensor Parallel 서빙

vLLM 엔진은 오픈소스 생태계에서 양자화, 멀티 GPU 병렬화를 가장 직관적인 인터페이스로 지원한다.

아래는 Llama-3-70B 모델을 AWQ 4비트로 양자화된 버전으로 로드하고 단일 서버내 4개의 GPU에 Tensor Parallelism을 적용해 배포하는 것이다.

```bash
# vLLM을 이용한 Multi-GPU Tensor Parallel + AWQ 서빙 인스턴스 가동
python3 -m vllm.entrypoints.openai.api_server \
    --model TechForge/Meta-Llama-3-70B-Instruct-AWQ \
    --quantization awq \
    --tensor-parallel-size 4 \
    --gpu-memory-utilization 0.90 \
    --port 8000 \
    --trust-remote-code
```

실행 로그 및 하드웨어 토폴로지 확인 예시 NCCL 초기화 과정

```
INFO 2026-06-20 12:30:15] api_server.py:150] vLLM 최적화 서빙 엔진 가동 시작.
INFO 2026-06-20 12:30:18] config.py:420] 양자화 전략 확정: AWQ (4-bit Weight-Only)
INFO 2026-06-20 12:30:20] pynccl.py:85] NCCL 통신 환경 초기화 중... (Tensor Parallel Size: 4)
INFO 2026-06-20 12:30:25] pynccl.py:120] 4개의 GPU 간 NVLink 토폴로지 감지됨. All-Reduce 통신망 연결 완료.
INFO 2026-06-20 12:30:30] model_loader.py:65] 파라미터 로드 중: 분할된 70B 모델 가중치 로드 성공 (GPU당 약 10.5 GB 점유)
INFO 2026-06-20 12:30:42] api_server.py:210] Uvicorn 서버 가동 완료: http://localhost:8000
```

### 모델크기 정확도 속도 비용간 비교표

아래 메트릭스는 대형 언어 모델 Llama-3-70B를 기준으로 다양한 정밀도와 하드웨어 아키텍처 토폴로지를 구성했을 때 발생하는 엔지니어링 지표를 종합 비교 분석한 정량적 산출물이다.

| 구성 (Precision & Parallelism) | 필요 최소 하드웨어 사양 | 총 가중치 VRAM (GB) | 가용 KV Cache 영역 | Perplexity (지능 손실률) | 시스템 최대 Throughput | 인프라 비용 지수 (원/시간) |
|--------------------------------|-------------------------|--------------------:|--------------------|-------------------------|-----------------------:|--------------------------:|
| FP16 / BF16 (No Parallelism) | 불가능 (OOM) | 140 GB | 0% | Baseline (0.00) | 측정 불가 | 가동 불가 |
| FP16 / BF16 (TP = 4, No PP) | A100 80GB × 4대 | 140 GB (GPU당 35) | 매우 여유로움 | Baseline (0.00) | 1,800 tokens/s | 100% (기준점) |
| FP8 (W8A8) (TP = 2, No PP) | H100 80GB × 2대 | 70 GB (GPU당 35) | 여유로움 | +0.02 (거의 없음) | 2,450 tokens/s | 75% |
| INT8 (BitsAndBytes) (TP = 2, No PP) | A100 80GB × 2대 | 70 GB (GPU당 35) | 여유로움 | +0.05 (미미함) | 1,120 tokens/s | 50% |
| INT4 (AWQ) (No Parallelism) | A100 80GB × 1대 | 35 GB | 보통 | +0.12 (방어 성공) | 410 tokens/s | 25% (극 가성비) |
| INT4 (AWQ) (TP = 2, No PP) | A100 40GB × 2대 | 35 GB (GPU당 17.5) | 매우 여유로움 | +0.12 | 920 tokens/s | 35% |