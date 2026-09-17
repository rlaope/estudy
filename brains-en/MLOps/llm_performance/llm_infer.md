# Understanding Transformer Inference Execution Structure

Training LLMs and performing inference in a service environment are entirely different engineering challenges.

Let's analyze the step-by-step flow of the forward pass, memory profile, and the fundamental causes of bottlenecks that occur internally when a Transformer model processes a single request.

## Why are Training and Inference Bottlenecks Different, and Where Do They Slow Down?

When working with AI models, one might ask, "GPU computational power was crucial during training, so why is VRAM bandwidth emphasized during inference?"

### Training Bottleneck: Compute-bound (Computational Limit)

During training, matrix multiplication operations are performed in parallel across the entire sequence length.

Since a massive number of tokens are computed at once, the GPU's arithmetic logic units (ALUs) work continuously, and GPU FLOPS (floating-point operations per second) become the bottleneck.

### Inference Bottleneck: Memory-bound (Memory Bandwidth Limit)

In contrast, inference has an **autoregressive** characteristic.

That is, it predicts one next word based on the previous words.

To generate this single word, the entire model weights, amounting to tens of GBs, must be pulled from the GPU's HBM (High Bandwidth Memory)
into SRAM (fast memory next to the compute cores).

While the computation itself is minimal, the time taken to fetch data (Memory Bandwidth) is much longer, causing the GPU compute cores to idle while waiting for data.

<br>

## Two-Phase Structure of Transformer Inference: Prefill -> Decode

From the user's prompt input to the final response output, Transformer inference is broadly divided into two phases.

### Phase 1. Prefill (Prompt Processing)

**The process of reading and understanding the input question at once**

- **Operation**: Processes all input tokens from the user's prompt in parallel with a single forward pass.
- **Main Purpose**: Understands the context of all input tokens and generates a KV Cache (Key-value Cache) to be reused in the subsequent Decode phase, storing it in GPU memory.
- **Bottleneck Characteristic (Compute-bound)**: Since a long input sequence is processed with matrix multiplication in one go, GPU utilization is very high. Computational load increases sharply as the prompt gets longer.
- **User Experience**: The time taken until this phase completes is TTFT (Time To First Token). This is the waiting time until the user sees the first character.

### Phase 2. Decode (Token Generation)

**The process of generating the next word, one character at a time**

- **Operation**: Utilizes the KV Cache computed in Prefill, taking only the single newly generated token as input to predict the next single token.
- **Main Purpose**: Repeats this process until the user-defined maximum length (Max Tokens) is reached or an end-of-sequence (EOS) token appears.
- **Bottleneck Characteristic (Memory-bound)**: To process just one token, billions of parameters must be read from HBM. The arithmetic intensity is very low, making it structurally inefficient and slow.
- **User Experience**: This affects TPOT (Time Per Output Token), which is the time taken to generate one token, and determines the perceived typing speed for the user.

### Streaming Execution Structure

In a real service environment, the system does not wait for the decode phase to complete before returning the entire sentence.

Instead, each time a token is generated in the Decode Phase, it is immediately streamed to the client web application to minimize the user's perceived latency.

<br>

## Attention Computation Load and GPU Memory Profiling

The key factors determining inference performance and GPU memory usage are "KV Cache size and the length N of input/output tokens".

### Attention Computation Structure and Complexity

The formula for Scaled Dot-Product Attention, the core of the Transformer, is as follows.

$$Attention(Q, K, V) = softmax(\frac{QK^T}{\sqrt{d_k}})V$$

Here, the computational load has an $O(N^2)$ complexity with respect to the sequence length N.

Therefore, the Input Token length exponentially impacts the computation time of the Prefill phase.

### Calculating GPU Memory Size Occupied by KV Cache

To avoid recomputing the key and value for previous tokens at each Decode step, they are stored in memory.

The memory in Bytes for the KV Cache for a single request is calculated as follows.

$$Memory_{KVCache} = 2 \times 2 \times N_{layers} \times N_{heads} \times d_{head} \times L \times Batch$$

- $2$: Key and Value two tensors
- $2$: Bytes for Float16/BFloat16 (16-bit = 2 Bytes)
- $N_{layers}$: Number of Transformer blocks (layers)
- $N_{heads}$: Number of Attention heads
- $d_{head}$: Dimension size of each head
- $L$: Total sequence length processed so far (Input Length + Generated Length)
- $Batch$: Batch size (usually 1 for a single request)

Since L increases by 1 each time an output token is generated, the size of the KV Cache, as the decode phase progresses,

**increases linearly.** This can lead to out-of-memory errors or overhead in cache management when generating long sentences.

To address this, advanced techniques like PagedAttention vLLM have emerged.

<br>

## Hands-on PyTorch & Hugging Face Code Verification

This is an example that explicitly separates the Prefill and Decode phases using PyTorch to verify the theory and profile execution time and memory.

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

Even though prefill processed 6 words at once, it took 15ms, whereas decode processed just 1 word but took 8ms. This means the per-token processing efficiency of decode is significantly lower.

It can also be observed that as decode progresses, the KV cache accumulates, increasing memory allocation.

In AI engineering, it is crucial to accurately understand this Transformer architecture and its dual bottleneck phenomena.

This is because the bottleneck section changes completely depending on whether the prompt is a very long summarization task or the output is a long code generation task.

Therefore, appropriate inference engines like vLLM, TGI, TensorRT LLM, and optimization techniques must be used according to the purpose.
