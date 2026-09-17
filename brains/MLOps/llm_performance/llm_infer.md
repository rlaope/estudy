# Transformer 추론 실행 구조 이해

LLM을 학습시키는 것과 서비스 환경에서 추론을 수행하는 것은 완전히 다른 엔지니어링 과제이다.

Transformer모델이 단일 요청을 처리할 때 내부에서 발생하는 

Forward Pass의 단계적 흐름과 메모리 프로파일 그리고 병목 현상의 근본적인 원인을 분석해보자

## 학습과 추론의 병목은 왜 다르며 각각 어디서 느려지는가

AI 모델을 다루다 보면, "학습할 때는 gpu 연산 능력이 중요했는데, 추론할때는 왜 VRAM 대역폭 타령을 할까?" 라는 질문을 던질 수 있다.

### 학습(Training)의 병목: Computed-bound (연산량의 한계)

학습시에는 전체 시퀀스 길이에 대해 병렬로 행렬 곱 연산(Matrix Multiplication)이 수행된다.

막대한 양의 토큰을 한 번에 계산하므로 gpu의 연산유닛 ALU가 쉬지 않고 일하며, GPU FLOPS(초당 부동소수점 연산 횟수)가 병목이 된다.

### 추론(Inference)의 병목: Memory-bound (메모리 대역폭의 한계)

반면 추론은 **자기회귀(Autoregression)** 특성을 가진다.

즉, 이전 단어들을 바탕으로 다음 단어 하나를 예측한다.

이 단어 하나를 생성하기 위해 수십 GB에 달하는 모델 가중치 전체를 GPU HBM(고대역폭 메모리)에서  
SRAM(연산 코어 옆의 빠른 메모리)로 끌어와야한다.

연산 자체는 매우 적지만, 데이터를 끌어오는 시간 Memory Bandwidth이 훨씬 오래걸려 GPU 연산 코어가 데이터를 기다리며 놀게된다.

<br>

## Transformer 추론의 2단계 구조 Prefill -> Decode

사용자의 프롬프트가 입력되어 최종 응답이 출력되기까지, Transformer의 추론은 크게 두 가지 페이즈로 나뉜다.

### Phase1. Prefill (Prompt Processing)

**입력된 질문을 한 번에 읽고 이해하는 과정**

- **동작 방식**: 사용자가 입력한 프롬프트 input tokens 전체를 한 번의 Forward Pass로 병렬처리한다.
- **주요 목적**: 입력된 모든 토큰의 context를 파악하고, 이후 Decode 단계에서 재사용할 KV Cache(Key-value Cache)를 생성하여 GPU 메모리에 저장한다/
- **병목 특성 (Compute-bound)**: 입력된 긴 시퀀스를 한 번에 행렬곱 연산하므로 GPU 활용도 Utilization가 매우 높다. 프롬프트가 길어질수록 연산량이 급증한다.
- **사용자 경험**: 이 단계가 끝날때까지 걸리는 시간이 바로 TTFT(Time To First Token)이다. 즉 사용자가 첫 번째 글짜를 보기까지 대기하는 시간이다.

### Phase2. Decode (Token Generation)

**한 글자씩 다음 단어를 생성하는 과정**

- **동작 방식**: Prefill에서 계산해둔 KV Cache를 활용해, 방금 생성된 토큰 1개만 입력받아 다음 토큰 1개를 예측한다.
- **주요 목적**: 사용자가 설정한 최대 길이(Max Tokens)에 도달하거나 종료 토큰 EOS이 나올 때 까지 이 과정을 반복한다.
- **병목 특성 (Memory-bound)**: 단 1개의 토큰을 처리하기 위해 수백억개의 파라미터를 HBM에서 읽어와야한다. 연산 강도 Arithmetic Intensity가 매우 낮아 구조적으로 비효율적이며 느리다.
- **사용자 경험**: 토큰 1개를 생성하는데 걸리는 시간인 TPOT(Timte Per Output Token)에 영향을 미치며, 사용자가 느끼는 글자가 타자 쳐지는 속도를 결정한다.

### Streaming 실행 구조

실제 서비스 환경에서는 decode 단계가 완료될때까지 기다렸다가 전체 문장을 반환하지 않는다.

Decode Phase에서 토큰이 하나 생성될 때마다 클라이언트 웹 앱으로 즉시 Streaming 전송하여 사용자의 체감 지연시간을 극소화한다.

<br>

## Attention 연산량과 GPU Memory 프로파일링

추론 성능과 GPU 메모리 사용량을 결정하는 핵심은 "KV Cache의 크기와 입출력 토큰의 길이 N"이다.

### Attention 연산 구조와 복잡도

Transformer의 핵심인 Scaled Dot-Product Attention의 수식은 다음과 같다.

$$Attention(Q, K, V) = softmax(\frac{QK^T}{\sqrt{d_k}})V$$

여기서 연산량은 시퀀스 길이 N에 대해서 $O(N^2)$ 복잡도를 가진다.

따라서 Input Token 길이는 Prefill 단계의 연산 시간에 기하 급수적인 영향을 미친다.

### KV Cache가 차지하는 GPU Memory 크기 계산

매 Decode 스텝마다 이전 토큰들의 key,value 값을 다시 계산하지 않기 위해 메모리에 저장해둔다.

단일 요청에 대한 KV Cache의 메모리 Bytes는 다음과 같이 계산된다.

$$Memory_{KVCache} = 2 \times 2 \times N_{layers} \times N_{heads} \times d_{head} \times L \times Batch$$

- $2$: Key와 Value 두 개의 텐서
- $2$: Float16/BFloat16의 바이트 수 (16-bit = 2 Bytes)
- $N_{layers}$: Transformer 블록(레이어) 수
- $N_{heads}$: Attention 헤드 수
- $d_{head}$: 각 헤드의 차원 크기
- $L$: 현재까지 처리된 전체 시퀀스 길이 (Input Length + Generated Length)
- $Batch$: 배치 사이즈 (단일 요청이므로 보통 1)

output token이 생성될때마다 L이 1씩 증가하므로 KV Cache의 크기는 decode 단계가 진행됨에 따라

**선형적으로 증가하게 된다.** 이로 인해 긴 문장을 생성할 때 메모리 부족이 발생하거나 캐시 관리에 부하가 생긴다.

이를 해결하기 위해 PagedAttention vLLM같은 고급 기술들이 등장했다.

<br>

## Hands-on PyTorch & Hugging Face 코드 검증

이론을 확인하기 위해 파이토치를 이용해 Prefill, Decode 단계를 명시적으로 분리하고 실행시간과 메모리를 프로파일링 하는 예제다.

```py
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
import time

# 1. 모델 및 토크나이저 로드 (CUDA 환경)
model_id = "gpt2-large" # 예시용 소형 모델
device = "cuda" if torch.cuda.is_available() else "cpu"

tokenizer = AutoTokenizer.from_pretrained(model_id)
model = AutoModelForCausalLM.from_pretrained(model_id, torch_dtype=torch.float16).to(device)

prompt = "AI engineering is the practice of"
inputs = tokenizer(prompt, return_tensors="pt").to(device)

# 메모리 초기화 및 측정 시작
torch.cuda.empty_cache()
torch.cuda.reset_peak_memory_stats()
start_mem = torch.cuda.memory_allocated()

# ==========================================
# [Phase 1] Prefill 단계 분석
# ==========================================
torch.cuda.synchronize()
t0 = time.perf_counter()

with torch.no_grad():
    # prompt 전체를 한 번에 통과시키고 past_key_values(KV Cache)를 반환받음
    outputs = model(**inputs, use_cache=True)

torch.cuda.synchronize()
prefill_time = (time.perf_counter() - t0) * 1000 # ms 단위

# 첫 번째 토큰 추출
next_token_logits = outputs.logits[:, -1, :]
next_token = torch.argmax(next_token_logits, dim=-1).unsqueeze(-1)
kv_cache = outputs.past_key_values # 생성된 KV Cache

prefill_mem = torch.cuda.memory_allocated() - start_mem

# ==========================================
# [Phase 2] Decode 단계 분석 (1 Token 생성)
# ==========================================
torch.cuda.synchronize()
t1 = time.perf_counter()

with torch.no_grad():
    # 직전 생성된 토큰 1개와 KV Cache만 입력으로 제공
    decode_outputs = model(next_token, past_key_values=kv_cache, use_cache=True)

torch.cuda.synchronize()
decode_time = (time.perf_counter() - t1) * 1000 # ms 단위
decode_mem = torch.cuda.memory_allocated() - start_mem

# 결과 출력
print(f"--- 추론 실행 구조 프로파일링 결과 ---")
print(f"[Prefill] 소요 시간 (TTFT): {prefill_time:.2f} ms")
print(f"[Prefill] 추가 메모리 할당 (KV Cache 등): {prefill_mem / 1024**2:.2f} MB")
print(f"[Decode] 1토큰 생성 시간 (TPOT): {decode_time:.2f} ms")
print(f"[Decode] 누적 메모리 증가량: {decode_mem / 1024**2:.2f} MB")
```

```
--- 추론 실행 구조 프로파일링 결과 ---
[Prefill] 소요 시간 (TTFT): 15.42 ms
[Prefill] 추가 메모리 할당 (KV Cache 등): 2.15 MB
[Decode] 1토큰 생성 시간 (TPOT): 8.31 ms
[Decode] 누적 메모리 증가량: 2.18 MB
```

prefill은 6개의 단어를 한번에 처리했음에도 15ms가 걸렸고 반면 decode는 단어 1개를 처리했는데 8ms가 소요되었다 즉 토큰당 처리 효율은 decode가 훨씬 떨어진다.

decode가 진행될수록 kv cache가 축적되어 메모리 할당량이 증가하는 것도 확인이 가능하다.

AI 엔지니어링에서 이러한 트랜스포머 구조와 이원적 병목 현상을 정확히 이해해야한다.

프롬프트가 매우 긴요약 작업인지 출력결과가 긴 코드 생성작업인지에 따라 병목 구간이 완전 달라지기 때문이다.

목적에 맞는 적절한 추론엔진 vLLM, TGI, TensorRT LLM과 최적화 기법을 써야한다.