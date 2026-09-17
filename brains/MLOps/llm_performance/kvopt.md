# KV Cache와 Batching 최적화

LLM 서빙을 모니터링하다보면 흔히 발생하는 기이한 현상이 있다.

GPU 사용률 (GPU Utilization)은 90퍼센트 이상을 찍고 있어서 장비가 터지려고 하는데

정작 초당 처리되는 토큰수는 바닥을 치는 현상이다.

이 모순을 해결하는 것이 바로 vLLM과 SGLang같은 현대적 LLM 서빙 엔진의 핵심 존재 이유다.

## GPU 사용률은 높은데 처리량 낮은 이유가?

결론부터 말하면, GPU가 유의미한 연산 (Tensor Core를 활용한 행렬 곱)을 하느라 바쁜게 아니라, 메모리 데이터를 옮기고 대기하는 I/O 병목(Memroy-bound)에 갇혀 있거나, 비효율적인 메모리 할당으로 인해 배치를 크게 키우지 못했기 때문이다.

1. **메모리 대역폭의 한계 (Memory Bandwidth Bound)**: Decode 단계는 극단적인 memory-bound 작업이다. 가중치와 KV Cache를 HBM에서 연산 코어로 가져오는 시간동안 GPU 유닛들은 일을 안하고 기다리기만한다. 하지만 GPU 모니터링 도구 nvidia-smi 등 메모리 버스가 작동하고 있으면 GPU가 열심히 일하고 있다 (Utilization 95%)로 판단한다. 즉, 연산 가동률이 아니라 **메모리 버스 가동률**이 찍힌 것이다.
2. **메모리 파편화로 인한 배치 크기 제약**: 전통적인 서빙 방식에서는 각 요청이 사용할 최대 토큰 길이 (ex. 2048 tokens)만큼의 KV Cache 메모리를 미리 고정된 크기 static allocation으로 할당했다. 사용자가 실제로 10토큰만 입력하고 50토큰만 출력하더라도 나머지 1988토큰의 공간이 남아 낭비가 되는것이다. 이로 인해 정작 동시에 사용자를 늘려 배치를 키우고싶어도 VRAM OOM(Out of Memory)이 발생해 배치를 키우지 못하고 결국 Throughput이 낮아진다.

### Batch 크기는 어떻게 결정하는가?

최적의 batch 크기는 GPU VRAM이 허용하는 한 가장 크게 잡는것이 기본 원칙이다.

배치가 커질 수록 GPU의 가중치 하나를 읽어와서 여러 요청의 연산에 재사용할 수 있으므로 Compute-bound 특성에 가까워저 처리량이 폭발적으로 증가할 수 있기 때문이다.

단, 배치를 무작정 키우면 OOM이 나거나 큐 대기 시간 증가로 인해 TTFT(첫 토큰 지연 시간)가 SLA 기준을 초과하게 된다. 따라서 하드웨어 메모리 낭비 없이 100퍼센트 활용할 수 있는 기술들이 필요하다.

<br>

## 5대 핵심 최적화 기술의 원리 이해

### PagedAttention (vLLM 핵심)

운영체제 OS의 가상 메모리 페이징 기법을 KV Cache 관리 시스템에 도입한 기술이다.

KV Cache를 고정된 작은 크기의 block (ex. 16 tokens) 단위로 쪼개고 물리적으로 흩어진 VRAM 공간에 가상 블록 테이블을 통해 매핑한다.

메모리를 연속 공간으로 할당할 필요가 없으므로, 사전 할당으로 인한 내부 파편화 (Internal Fragmentation)와 가변 길이로 인한 외부 파편화 (External Fragmentation)를 거의 0%에 가깝게 줄여낸다. 낭비되던 VRAM의 60~80%를 회수하여 **배치 크기를 2~4배 이상 키울 수 있게 만든다.**

---

### Continuous Batching (Iteration-level Scheduling)

기존의 Static Batching은 배치 내의 모든 요청이 완전히 끝날 때까지 새로운 요청을 받지 못하는 반면, Continuous Batching은 **토큰 생성 주기 Iteration 단위로 배치를 유동적으로 변경한다.**

하나의 토큰 생성이 끝나는 지점마다 종료된 요청(EOS)은 배치에서 즉시 빼내고 큐에서 대기중이던 신규 요청의 Prefill 단계를 끼워넣는다.

먼저 끝난 유저 때문에 GPU가 놀거나 긴 문장을 쓰는 유저 때문에 전체 시스템이 대기하는 비효율을 완벽히 제거한다.

---

### Prefix Caching (Radix Attention)

프롬프트의 공통된 앞부분(Prefix)에 대한 KV Cache를 메모리에 보관해두고 재사용하는 기술이다

시스템 프롬프트, Few shot 예시, RAG Context등 여러 유저가 공유하는 텍스트의 KV Cache를 트리구조로 관리한다. 동일한 Prefix를 가진 요청이 오면 Prefill 연산을 통째로 건너뛰고 캐시에서 바로 읽어온다.

Multi-turn 대화나 RAG 기반 서비스에서 TTFT를 거의 0ms에 가깝게 단축시키며 대량의 Prefill 연산량을 절감한다.

---

### Chunked Prefill

대형 프롬프트가 들어왔을 때 발생하는 Prefill 병목을 지우기 위해 Prefill 요청을 여러개의 작은 덩어리로 쪼개어 처리하는 기법이다.

아주 긴 프롬프트를 한 번에 처리하려면 막대한 연산량이 들기 때문에 기존 Decode 중이던 배치들이 모두 멈춰서서 ITL(Inter-Token Latency)이 튀게된다. 이를 방지하기 위해 Prefill을 512 토큰 등의 Chunk로 나누어, 기존 Decode 연산들과 함께 배치에 묶어서 조금씩 나누어 처리한다.

**효과:** 긴 프롬프트 유저가 진입해도 기존 유저들의 서비스 경험 ITL 지연 방지를 해치지 않고 연산을 스무스하게 스케줄링할 수 있다.

<br>

## Hands-on vLLM 엔진 설정 및 튜닝 실전

vLLM 아키텍처에서 위의 옵션들을 튜닝하여 처리량을 극대화하는 프로덕션 레벨아규먼트 설정을 알아보자

```bash
# vLLM 최적화 서빙 인스턴스 실행 스크립트
python3 -m vllm.entrypoints.openai.api_server \
    --model meta-llama/Meta-Llama-3-8B-Instruct \
    --port 8000 \
    --gpu-memory-utilization 0.95 \
    --block-size 16 \
    --max-num-seqs 256 \
    --enable-chunked-prefill True \
    --max-num-batched-tokens 2048 \
    --enable-prefix-caching
```

- `--gpu-memory-utilization 0.95`: 모델 가중치를 제외한 나머지 VRAM중 90%를 전부 page attention 블록풀로 할당하는 설정
- `--block-size 16`: pagedAttention의 1블록당 토큰수다. 16또는 32가 CUDA 정렬 성능상 가장 효율적이다
- `--max-num-seqs 256`: 한 배치에 동시에 묶을 수 있는 최대 요청수로 Throughput을 올리려면 하드웨어가 버티는 한 크게 잡아야한다.
- `--enable-chunked-prefill True`: Prefill과 Decode의 공존을 활성화하여 큰 프롬프트 유저 유입시 서비스 끊김을 방지한다.

<br>

## KV Cache 사용량, Batch설정별 성능 비교 보고서

아래 벤치마크 결과는 전통적인 서빙 아키텍처 Hugging Face Native + Static Batching와 현대 최적화 엔진 vLLM (PagedAttention + Continuous Batching) 간의 리소스 효율성 및 처리량 차이를 정량적으로 분석한 결과다.

### 성능 비교 메트릭스 Llama-3-8B, NVIDIA A100 80GB 단일 장비 기준

| 실험군 | 배칭 전략 | KV Cache 방식 | Max Batch 설정 | 실제 VRAM 단편화율 | 최고 Throughput (tokens/sec) | P99 ITL (토큰간 지연) |
|---------|----------|---------------|---------------:|-------------------|-----------------------------:|----------------------:|
| A (대조군) | Static Batch | Naive (연속 할당) | 16 | 65.4% (낭비 심함) | 180 | 12.5 ms |
| B (배칭 개선) | Continuous | Naive (연속 할당) | 32 | 61.2% | 420 | 35.6 ms |
| C (메모리 혁신) | Continuous | PagedAttention | 128 | 3.8% (최적) | 1,850 | 18.2 ms |
| D (풀 최적화) | Continuous | Paged + Chunked | 256 | 4.1% | 2,420 | 14.1 ms (안정적) |

### 데이터 해석 및 인사이트

- **단편화 제거의 위력 A vs C**
  - 전통적인 Native 할당 방식에서는 동시 사용자를 겨우 16명만 받아도 VRAM 가상 OO 한계에 부딪혔다. 실제로 쓰이지 않는 허수의 프롬프트 영역까지 메모리를 독점했기 때문
- 반    면 PagedAttention(C)를 도입하면 메모리 단편화율 3.8% 수준으로 억제되어 물리 메모리의 끝까지 활용이 가능해진다. 배치를 128까지 키울 수 있게 되면서 Throughput이 180에서 1850 tokens/sec으로 약 10배 폭발한다.
- **Chunked Prefill의 효과 C vs D**
  - 단순히 배치 크기만 극대화한 C군의 경우에는 가끔 대형 프롬프트가 들어오면 전체 배치가 멈칫해 P99 ITL이 18.2ms까지 튀는걸 확인할 수 있다.
  - D군처럼 **Chunked Prefill을 활성화**시 신규 요청의 프롬프트 연산을 쪼개어 기존 처리중인 응답 연산 사이에 고르게 버무려줌으로써 최대 배치 크기를 256까지 확보하면서도 토큰간 지연시간을 14.1ms 수준으로 부드럽게 유지가 가능하다.
