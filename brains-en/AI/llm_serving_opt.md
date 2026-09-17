# Mastering the LLM Inference Serving Engine SGLang

Before reading this article, I'll start by posing several questions.

- Why was DeepSeek so obsessed with compressing the KV cache?
- Where do the actual benefits come from when training with MTP (Multi-Token Prediction)?
- From a user's perspective, how many milliseconds difference does GLM-5.2's context length increase from 4.56 to 5.47 make?
- The statement that 1M context is practical, not just for advertising – **what was measured?**

The answers to all these questions lie in the **inference serving** layer.

Model architecture is reverse-engineered to fit serving constraints, and the serving engine is the layer that realizes that architecture on hardware.

Only by looking at both layers together can one truly understand why they were built that way.

> Practice - In this article, the practice assumes a hypothetical scenario, and while few people can immediately access an 8-card H200 node, if working in such an environment is the goal, it's appropriate to train in advance on what decisions to make in that environment. Commands and settings are written as they are in reality, but results cite actual measurements from public benchmarks and vendor reports.

| Case | Scenario | Hardware | Key Concern |
|------|--------------------------|-------------------|-------------------|
| A. In-house Coding Assistant | 200 engineers, conversational | 8×H200 (1 node) | Latency (TTFT) |
| B. Document QA / RAG Service | Repeated queries on the same document | 4×H100 | Prefix reuse |
| C. Large-scale MoE Serving | DeepSeek V4-class, traffic surge | 12 nodes / 96 GPUs | Throughput and cost |

<br>

## Serving Engine

Why is a separate engine needed??

You can run models with Hugging Face `transformers`.

However, if you actually try this method, you'll find that it collapses even with just eight concurrent users.

This is because the default generation routine for `transformers` looks like this:

```
요청을 하나 받는다 -> 처음부터 끝까지 생성한다 -> 다음 요청을 받는다
```

From a GPU perspective, it's like **only one car driving on an 8-lane highway**. GPUs are designed for parallel computation with thousands of cores, but if they only process one request, they can barely utilize that parallelism.

In actual measurements, GPU utilization hovers around 30-40%.

A serving engine **fills up this highway**. That's all it does, and even though that's all it does, it's difficult.

### Current Landscape as of August 2026

```
[ 사용 규모 ]
   개인 ──────────────────────────────────────→ 초대형

  llama.cpp        vLLM              SGLang          TensorRT-LLM
  Ollama       (범용 기본값)      (프리픽스/에이전트)   (NVIDIA 극한)
     │              │                  │                  │
  단일 사용자    가장 넓은 하드웨어    RadixAttention     컴파일 필요
  CPU 가능      pip 한 줄 설치      PD분리 + 대규모EP   Blackwell 최적
```

| Engine | Developer | Nature | Version as of 2026.6 |
|------|-----------|------|------------------|
| vLLM | UC Berkeley Sky Computing Lab | General-purpose default. Originator of PagedAttention | 0.23.0 |
| SGLang | LMSYS / xAI affiliated researchers | Specialized for structured execution. RadixAttention | 0.5.13 |
| TensorRT-LLM | NVIDIA | NVIDIA-exclusive extreme optimization. Engine pre-compilation | 1.2.1 |
| LMDeploy | Shanghai AI Lab | Fast support for Chinese models | — |
| llama.cpp / Ollama | Community | Personal use, CPU/Mac capable | — |
| TGI | Hugging Face | Entering maintenance mode in December 2025 | (Not recommended for new adoption) |

> Before diving in, it's worth noting that vLLM, SGLang, and TensorRT-LLM all implement continuous batching, Paged KV Cache, and FP8 quantization, so the superficial feature gap has completely disappeared. In 2024, there was meaning in comparing "vLLM does this, SGLang does that," but now the selection criteria are not features but architectural philosophy and hardware.

When choosing an optimization stack, **hardware becomes the first filter**. SGLang officially supports NVIDIA, AMD, Intel Xeon, Google TPU, Ascend NPU, and more. TensorRT-LLM, by design, is NVIDIA-exclusive, compiling CUDA engines and optimized for Blackwell-class (GB200, GB300) hardware.

If there's even a slight possibility of using AMD MI300 or Ascend, let's exclude TensorRT-LLM from consideration.

**Workload becomes the second filter.** This article will focus on SGLang, and this is where SGLang's raison d'être emerges: workload.

#### What makes SGLang different?

Most serving stacks assume that **all requests are independent of each other**. While this is a valid assumption for simple chat APIs, actual workloads in 2026 are not like that.

- Multi-turn conversations -> What if the entire previous turn is a prefix for the next request?
- RAG -> The same document appears repeatedly for dozens of questions
- Agents -> The same system prompt + tool definition repeats with every call
- Self-consistency sampling / Tree search -> Branching into multiple paths from the same prompt

SGLang **treats this first-class redundancy as a first-class resource.** Even after a request finishes, it doesn't discard the KV cache but stores it in a Radix Tree. If the next request uses the same prefix, the entire computation is skipped. This is RadixAttention, which we'll explore below.

In public benchmarks, SGLang showed approximately 29% higher throughput compared to vLLM for workloads with high prefix sharing (RAG, multi-turn chat) on H100 Llama 3.1 8B. Conversely, for workloads where all requests are completely unique, this advantage was zero, making this conditionality key to understanding SGLang's value.

### Sense of Scale

SGLang is adopted by xAI, NVIDIA, AMD, Google Cloud, Oracle Cloud, LinkedIn, Cursor, and others, and is large enough to generate trillions of tokens in production daily.

<br>

## The Physics of Performance

Let's first look at three core facts:

1. LLM decoding is slow not because of insufficient computation, but because it's **slow to read weights from memory**.
2. Therefore, increasing batch size **almost freely boosts throughput.**
3. What prevents increasing batch size is the memory consumed by the KV cache.

The problem defined in these three sentences is the entire reason for the existence of serving engines.

### PD (Prefill, Decode)

The process of an LLM generating a response might seem like a single, homogeneous task, but it's actually divided into two stages with completely different characteristics.

```
[Prefill]
Input: "Please review the following code: ..." (2,000 tokens)
  ↓
Pass 2,000 tokens through the model all at once
  ↓
Calculate K and V for each of the 2,000 tokens and store them in the KV cache
  ↓
Generate the first output token

[Decode]
1st output token → pass through model → 2nd token
2nd output token → pass through model → 3rd token
...   (repeat 500 times)
```

The key difference is the **number of tokens processed per model pass**.

- Prefill processes 2,000 tokens at once -> **parallel processing is possible.**
- Decode processes 1 token at a time -> **sequential, parallel processing is not possible.**

This is where the true nature of the GPU emerges: a GPU is a machine that **performs matrix multiplications in parallel.**

In prefill, a 2000 x 5120 matrix is multiplied by weights, and since it's a large matrix multiplied by a large matrix, GPUs love this task.

In decode, a 1 x 5120 vector is multiplied by the weight matrix. The weights still need to be read entirely, but the amount of computation performed with those weights is only 1/2000th.

> Imagine it takes 10 minutes to place a huge dictionary on your desk. Prefill is like placing the dictionary once and finding 2000 words at once, taking 0.3 seconds per word. Decode is like placing the dictionary, finding just one word, putting it away, and then placing it again for the next word. That's 10 minutes per word.

- Prefill is: compute-bound, a computationally intensive task where the GPU's tensor cores are saturated.
- Decode is: memory-bandwidth intensive, memory-bound. The arithmetic intensity of batch 1 decode is near 1 FLOP/Byte on the H100 roofline, and linear attention architectures are even lower.

Arithmetic intensity is **how many operations are performed for every 1 byte read from memory**.

The H100's tensor cores can handle hundreds of operations per byte, but decode only does 1. **This means over 99% of GPU performance is idle.**

Three things follow from this fact:

1. To increase decode speed, we need to reduce memory reads, not computation -> This is why quantization (FP8, INT4) is so effective. Halving the weight size halves the read time.
2. There's a way to utilize idle computational resources for free. -> We'll learn about batching and speculative decoding below, which stem from this.
3. Prefill and decode consume different resources, so separating them is beneficial -> This is covered in PD separation.

### KV Cache

This is the most important single concept in this article.

When generating the 500th token in the decode stage, attention needs to refer to the K and V of the preceding 2,499 tokens. If recalculated every time, the same computation would be repeated 500 times.

Storing the once-calculated K and V in GPU memory for reuse is what the KV cache does.

Size formula:

```
KV Cache Size = 2 × L × H_kv × D × S × B × (bytes/element)

  2     : Key and Value (two)
  L     : Number of layers
  H_kv  : Number of KV heads (fewer than query heads if GQA)
  D     : Head dimension
  S     : Sequence length (context)
  B     : Number of concurrent requests (batch)
  bytes : 2 for BF16, 1 for FP8
```

Let's calculate it directly: a 70B-class model is deployed on an 8 x H200 node. With 80 layers, 8 KV heads, 128 head dimensions, BF16, and one user using a 32K context:

```
2 x 80 x 8 x 128 x 32,768(32k) x 1 x 2 bytes
= 2 x 80 x 8 x 128 x 32,768 x 2
= 10,737,418,240 bytes
≈ 10.7GB
```

Each user requires 10.7GB of KV cache space (wow!).

An H200 card has 141GB of memory, so 8 cards have 1,128GB. If the 70B model weights in BF16 are 140GB, then 988GB remains. Dividing this by the 10.7GB we just calculated:

```
988 GB ÷ 10.7 GB = approx. 92 users
```

If 200 engineers simultaneously use a 32K context, half of them will have to wait.

What this calculation reveals is that this single calculation can highlight the necessity of various design choices at once.

| Technique from previous notes | Which term in the formula does it affect? | Effect |
|--------------------------------|-------------------------------------------|--------|
| GQA | Reduces H<sub>kv</sub> | 8 heads shared → 1/8 cache |
| MLA (DeepSeek) | Replaces H<sub>kv</sub> × D with compressed latent dimension | Significantly reduces cache |
| Linear Attention / GDN (Qwen, Kimi) | Removes S dependency (fixed-size state) | Cache size independent of length |
| FP8 KV Cache | Changes bytes/element from 2 → 1 | Halves cache |
| DSA (Sparse) | Maintains cache itself but reduces read amount | Saves bandwidth |

The reason DeepSeek insisted on MLA is now clear. If the 92-user capacity in the above calculation becomes 184 users, the same GPU generates double the revenue. That's the background behind SGLang achieving a cost of $0.20 per million output tokens when serving DeepSeek with 96 H100s, which is about 1/5th of the official DeepSeek Chat API.

The cost of the KV cache means it cannot be eliminated. Eliminating it would lead to full recalculation for every token, making it much slower. Therefore, **managing it well is the only way, and management techniques are needed.**

### Memory Hierarchy - Why Offloading Alone Isn't Enough

If the KV cache doesn't fit entirely in GPU memory, couldn't we put it in CPU memory? This is a natural idea and a technique actually used. However, one must understand the bandwidth differences.

The hierarchy has three tiers: hot KV blocks being actively generated are kept in GPU HBM with approximately 3.35 TB/s bandwidth. Warm blocks that are complete but potentially reusable are moved to CPU DRAM via PCIe 5.0 at about 63 GB/s. Cold blocks go to NVMe at 7 GB/s.

```
GPU HBM   ████████████████████████████████████  3,350 GB/s   (Base 100%)
CPU DRAM  ▌                                         63 GB/s   (1.9%)
NVMe SSD  ▏                                          7 GB/s   (0.2%)
```

PCIe 5.0 is only 2% of HBM bandwidth. Moving a 50GB KV cache takes 15 milliseconds on HBM, but 800 milliseconds if fetched from CPU DRAM.

In workloads where cached tokens overwhelmingly outnumber new tokens, this bottleneck is critical. In a QA scenario where a 32-token question is posed to a 65K-token document, Llama-3.1-405B needs to transfer 33GB via PCIe, which takes 500 milliseconds.

So, the idea of offloading and reusing the KV cache to save on prefill is valid, but if the transfer time is slower than recalculation, it's a loss. Knowing this break-even point is practical thinking.

Criteria for judgment:

```
Recalculation Cost ≈ (Number of tokens × Model FLOPs) / GPU compute performance
Transfer Cost      ≈ (KV Cache Bytes) / Link Bandwidth

If transfer is cheaper → Offloading is beneficial
If recalculation is cheaper → Just recalculate (vLLM's default preemption behavior)
```

Indeed, vLLM supports both recalculation and swap modes for preempted sequences. Recalculation is cheaper for short contexts, while swapping is better for long ones, so the default switches based on context size.

Introducing a hierarchical cache significantly increases system complexity and can cause tail latency (p99) spikes during cache misses. **For workloads with low prefix reuse rates, it's a net loss.**

### Metrics

Serving performance cannot be described by a single number; at least four metrics must be considered together.

#### TTFT (Time To First Token)

This is the latency from the moment a request is sent until the first character appears on the screen.

It's the queue waiting time + prefill time, meaning **it's the performance of the prefill stage.**

This is where users mostly perceive slowness. In a streaming UI, the perceived wait ends the moment the first character appears.

**Target feeling:** For interactive use, under 300ms is optimal. Over 1 second feels inconvenient, and over 3 seconds causes frustration and abandonment. (Though adding animations can make 3 seconds tolerable.)

#### TPOT / ITL (Time Per Output Token / Inter-Token Latency)

The interval between tokens. These two terms essentially refer to the same thing.

This determines the performance of the decode stage, batch size, model size, and quantization.

Compared to human reading speed, which is roughly 10-15 tokens per second, an ITL of 50ms (20 tokens per second) feels faster than human reading speed.

#### Goodput - An Often Overlooked Metric

Throughput when counting only requests that satisfy the SLO.

Optimizing only for throughput might lead to infinitely increasing batch sizes, which then degrades individual users' TTFT and ITL.

Simply put, if 10,000 tokens per second were processed, but half of those were requests where users closed the window while waiting, it's meaningless.

If the number of dishes cooked per hour in a restaurant is throughput, then goodput is the number of dishes served before customers get angry.

**Fundamental Tension Between Throughput and Latency**

```
Batch Size ↑
  → GPU Utilization ↑ → Throughput ↑ ✅
  → Computation per step ↑ → Individual Request ITL ↑ ❌
  → Queue Wait Time ↑ → TTFT ↑ ❌
```

**The essence of serving tuning is to find the point on this curve that matches one's SLO**, and there is no such thing as a single "best performance" setting. It varies by situation.

Therefore, the techniques discussed below address problems such as:

- GPU idling because decode is memory-bound
- Prefill blocking decode, causing latency spikes
- KV cache memory fragmentation
- Recalculating the same prefix repeatedly
- Insufficient GPU memory
- Kernels not fully utilizing hardware
- Model not fitting on a single GPU
- Prefill and decode interfering with each other
- Utilizing idle computational resources during decode
- Slow weight reading
- Needing to reliably extract JSON

<br>

## Batching - Techniques to Fill the GPU

**Since compute resources are left over, let's process multiple items concurrently.**

### Static Batching

Let's start with the simplest method. Gather 8 requests into one batch, and once they all finish, receive the next 8.

```
Time →
Request1 ████████████████████████ (500 tokens)
Request2 ████                     (50 tokens)  ← Occupies slot even after finishing
Request3 ██████                   (80 tokens)  ← Occupies slot even after finishing
Request4 ████████                 (120 tokens) ← Occupies slot even after finishing
      └──────────────────────┘
      The entire batch waits for Request 1
      Requests 5-8 wait in the queue throughout this time
```

LLMs are characterized by **extremely imbalanced output lengths.** What if a request that answers "Yes" and a request that writes 300 lines of code are in the same batch? Static batching means the **entire batch is held hostage by the slowest request.**

Furthermore, slots for completed requests are **computed empty.** The GPU performs operations on padded spaces just the same.

In static batching, GPU utilization typically hovers around 30-40%.

In conclusion, no production serving engine will be using static batching in 2026; let's understand it for conceptual purposes, now that we know its problems.

### Continuous Batching

If waiting for the entire batch to finish was the problem, it starts with **swapping in new requests as soon as slots become available.**

The scheduling unit is changed from a batch to an iteration (one model pass), which is why it's also called **Iteration-level Scheduling**.

```
Every iteration:
  1. Remove completed sequences from the batch and return results
  2. Insert new requests from the waiting queue into empty slots
  3. Pass the entire batch through the model once
  4. Repeat
```

vLLM's scheduler evaluates three queues after each forward pass.

waiting for prefill to start, running actively decoding, and swapped where KV blocks are evicted to CPU memory to free up GPU space for higher-priority tasks.

```
Time →
Slot1 ███(RequestA)██████████████(RequestE)███████
Slot2 ████████(RequestB)████(RequestF)████(RequestI)█
Slot3 ██(RequestC)███████████████████(RequestG)███
Slot4 ██████(RequestD)█████(RequestH)█████████████
      ↑ The next request enters as soon as a slot becomes available
```

The key insight is that **within one iteration, all sequences advance by exactly one token.**

Even if the total length of each sequence differs, **the size of one step is the same.** Therefore, there's no issue in swapping participants at each step.

Switching from simple static batching to PagedAttention-based continuous batching can increase GPU utilization from 30-40% to 75-90%, leading to a 2-4x increase in output tokens per GPU hour.

- Since the scheduler must run every iteration, **CPU overhead is introduced instead.** This is the background for the zero-overhead scheduler that will be mentioned later in SGLang.
- Because batch configurations constantly change, **CUDA Graph Capture** becomes tricky.
- The ITL of individual requests can fluctuate depending on the status of other requests. Latency prediction becomes difficult.

vLLM, SGLang, and TensorRT-LLM all virtually support this standard.

### Chunked Prefill - Preventing Latency Spikes Caused by Long Prompts

Even with continuous batching enabled, latency spikes persist. The cause is the **difference in size between prefill and decode.**

Suppose a user pastes a 32K token document. This request's prefill is a massive task, processing 32,000 tokens at once. During this time, the GPU is tied up with just this one.

```
    [32K Prefill = 400ms]
20 other users decoding →  ████████ (receive no tokens for 400ms)
```

This is called head-of-line blocking, a situation where one user causes the ITL for all users to slow down (spiking up to 400ms here).

To solve this, chunked prefill is used. The method involves not processing the prefill all at once, but **splitting it into fixed-size chunks** and mixing each chunk into the same batch as the decode steps of other requests.

```
Iteration 1: [Prefill Chunk 1 (8K)] + [20 Decodes]
Iteration 2: [Prefill Chunk 2 (8K)] + [20 Decodes]
Iteration 3: [Prefill Chunk 3 (8K)] + [20 Decodes]
Iteration 4: [Prefill Chunk 4 (8K)] + [20 Decodes]
```

Two issues are resolved simultaneously:

1.  **Latency Equalization:** If 400ms is divided into four 100ms chunks, other users receive tokens in between.
2.  **Leveraging Free Computation:** Batches consisting only of decode operations are memory-bound, leaving compute resources available (Fact 1 in Chapter 1). By inserting prefill chunks, **these leftover compute resources are utilized for free.** Prefill and decode have different bottleneck resources, so mixing them complements each other.

In vLLM v0.18.0 (V1), the default chunk size for online serving is 8,192 tokens (controlled by `--max-num-batched-tokens`), a significant increase from the older version's 512. Without chunked prefill, a single 32K token prefill can block the GPU for hundreds of milliseconds, causing latency spikes for all concurrent requests.

As a real-world example from the tuning ladder, with Llama 3.1 8B Instruct / A100 80GB / ShareGPT trace, adding chunked prefill increased throughput from the default 4,200 tok/s to 4,800 tok/s.

**Trade-offs**
- If chunks are too small, prefill efficiency itself decreases, as small matrix multiplications are inefficient for the GPU.
- If chunks are too large, blocking reappears.
- **It can actually be detrimental for offline batch jobs.** This is because it reduces prefill efficiency when latency is not a concern.

### Example: In-house Coding Assistant

Engineers paste entire files, and prompt lengths vary extremely from 500 tokens to 40,000 tokens. **Chunked prefill is not an option, but a necessity.** Without it, someone asking a short question would experience several seconds of delay due to another's 40K paste.

```bash
# vLLM
--enable-chunekd-prefill --max-num-batched-tokens 8192

# SGLang (enabled by default, chunk size adjustment)
--chunekd-prefill-size 8192
```

Key batching configuration values to adjust in practice are listed below:

| Setting | What it controls | Increase | Decrease |
|---|---|---|---|
| `--gpu-memory-utilization` | VRAM ratio for KV cache pool (vLLM default 0.90) | Concurrent users ↑ | OOM headroom ↑ |
| `--max-num-batched-tokens` | Max tokens per iteration (vLLM V1 default 8192) | Throughput ↑ | ITL ↓ (improves) |
| `--max-num-seqs` | Max concurrent sequences (vLLM V1 default 1024) | Throughput ↑ | Individual latency ↓ (improves) |

On dedicated instances, `--gpu-memory-utilization` can be increased up to 0.95, and for high-throughput batch workloads, increasing `--max-num-batched-tokens` to 8192 or 16384 is recommended.

> Caution! - Increasing `--gpu-memory-utilization` to values like 0.98 will likely result in OOM errors. This is because active memory, CUDA Graph Buffer, and fragmentation headroom are needed. Therefore, 0.90 -> 0.95 is a safe range, and anything beyond that should be increased gradually with real-world measurements.

<br>

## KV Cache Management

In Chapter 1, I mentioned that KV cache limits batch size. The techniques in this chapter **increase the number of concurrent users by several times**.

### PagedAttention - Using KV Cache like OS Virtual Memory

This technique was developed by vLLM, and all current engines are based on it.

#### Problem

How should KV cache be allocated? A naive approach is to **pre-allocate a contiguous block of memory for each request, equal to the maximum context length.**

However, the problem is that most requests don't use the entire length.

```
요청 A에 128K 길이만큼 예약  ████████████████████████████████
실제 사용                    ████                            ← 나머지 전부 낭비
```

Traditional inference engines pre-allocate the entire maximum context length for all requests. Since most requests don't use the full context window, 60-80% of the allocated VRAM is wasted.

On top of this, **external fragmentation** occurs. As requests finish, gaps appear in the memory. New requests demand large contiguous blocks, so allocation can fail even if the total free space is sufficient.

#### Mechanism

It directly adopts the **virtual memory paging** concept from operating systems.

Just as a process accesses non-contiguous physical frames via a logical page table, each active sequence in vLLM accesses the KV cache in non-contiguous physical blocks of GPU memory via a logical block table.

Each physical block in the vLLM KV cache pool holds the key and value tensors for 16 contiguous tokens. (This is the default value, adjustable with `--block-size`.)

```
논리 뷰 (시퀀스가 보는 것)         물리 뷰 (실제 GPU 메모리)
시퀀스 A: [블록0][블록1][블록2]  →  [#7][#3][#12]  ← 흩어져 있어도 무방
시퀀스 B: [블록0][블록1]         →  [#1][#9]
                                    ↑ 빈 블록은 즉시 재사용
```

Why does it work?
- Blocks are allocated only when needed, eliminating the waste of pre-allocation.
- Since the block size is fixed at 16 tokens, external fragmentation is inherently avoided, and internal fragmentation is limited to a maximum of 15 tokens.
- Blocks are immediately returned to the pool when a request finishes.

PagedAttention allocates only the necessary blocks at the time of request as tokens are generated, and deallocates them immediately upon request completion, allowing 2-4 times more concurrent requests to be processed on the same GPU.

Trade off
- The attention kernel needs to follow the block table to read KV, requiring a specialized kernel. A naive implementation would actually be slower. - This will be covered in more detail when discussing kernels in Chapter 5.
- If the block size is small, management overhead is high; if it's large, internal fragmentation becomes an issue.

While vLLM is the originator, SGLang also uses a Paged Layout, and it's essentially a fundamental premise for all modern engines.

### Prefix Caching

This is about not computing the same thing twice. PagedAttention eliminated memory waste, but **computational waste remains**.

```
요청 1: [시스템 프롬프트 2000토큰][질문 A]
요청 2: [시스템 프롬프트 2000토큰][질문 B]
요청 3: [시스템 프롬프트 2000토큰][질문 C]
         ↑ 완전히 동일한데 세 번 프리필
```

In agent workloads, system prompts + tool definitions often exceed 5,000 tokens. **This means 5,000 tokens are being recomputed with every call.**

KV blocks are cached using the **hash of the token content** as the key, on a block-by-block basis. When a new request arrives, block hashes are compared from the beginning, reusing matching parts and computing only from the divergence point.

**Why does it work?:**

KV is causal, meaning that the K and V of any token depend only on the preceding tokens and are independent of subsequent tokens. Therefore, if prefixes are identical, the KV for that part is **bit-for-bit identical**, and reusing it does not change the result.

> This property is entirely what enables prefix caching. No matter what comes after, the KV of the preceding part remains unchanged.

If I had to pick one of the most valuable flags for most workloads, it would be `--enable-prefix-caching`. If there's a system prompt shared by all requests, this alone can yield a 30% throughput improvement. In the tuning ladder we saw earlier, the 4,800 -> 5,500 tok/s range corresponds to this item.

Trade off
- The prefix must match exactly. The moment you include a timestamp or username in the system prompt, the **hit rate drops to 0.**
- Since the cache occupies memory, an eviction policy is needed (to remove unused items when it overflows).

> Tips - When designing prompts, put the unchanging parts first and the changing parts last. This single principle often improves TTFT by several times. For example, changing `"Current time now you are an assistant.." ` to `You are an assistant ... Current time now..` allows the entire initial part to be cached. Avoid starting with mutable values.

### RadixAttention - SGLang Core

RadixAttention takes the prefix caching mentioned above a step further, and **this is the raison d'être of SGLang**.

Typical prefix caching is implemented with a **flat hash table.** However, the actual reuse patterns in workloads are not flat but **tree-shaped.**

```
                    [시스템 프롬프트]
                          │
        ┌─────────────────┼─────────────────┐
   [대화 1턴]         [대화 1턴']       [대화 1턴'']
        │                 │
   ┌────┴────┐       ┌────┴────┐
[2턴 A]  [2턴 B]  [2턴 C]  [2턴 D]     ← 멀티턴 대화의 분기
```

- Multi-turn conversations: The tree deepens as turns accumulate.
- Self-consistent sampling: Drawing 8 answers from the same prompt -> 8 branches from one node.
- Agent tree search: Trying multiple paths and backtracking.
- RAG: Branching per question from the same document node.

It's difficult to manage this structure efficiently with a flat cache.

#### Mechanism

Even after a generation request finishes, the KV cache is not discarded but stored in a Radix Tree for both the prompt and the generation result. This data structure enables efficient prefix search, insertion, and eviction. By combining this with an LRU eviction policy and cache-aware scheduling, the cache hit rate is improved.

A **radix tree** is a space-efficient variant of a trie (prefix tree). Unlike a typical tree, the edges of a radix tree can be labeled with variable-length sequences of elements rather than single elements, a characteristic that significantly increases efficiency.

```
트라이 (간선 = 토큰 1개)         라딕스 트리 (간선 = 토큰 시퀀스)
root                             root
 └─"당"                           └─"당신은 도움이 되는 어시스턴트입니다"
    └─"신"                            ├─"오늘 날씨는?"
       └─"은"                         └─"파이썬 코드를 짜줘"
          └─ ... (수천 노드)      ← 노드 수가 극적으로 줄어듦
```

The actual data structure looks like this:

```py
class TreeNode:
    children: dict[TreeNode]   # 자식 노드들
    parent: TreeNode           # 부모
    key: RadixKey              # 이 간선의 토큰 시퀀스
    value: torch.Tensor        # 대응하는 KV 캐시 인덱스
    lock_ref: int              # 참조 카운트 (사용 중이면 축출 금지)
```

`lock_ref` is practically important because **nodes currently in use by a request should not be evicted, so they are protected by a reference count.**

The operational flow is as follows:

```
새 요청 도착
   ↓
라딕스 트리에서 접두사 매칭 (가장 긴 공통 접두사 탐색)
   ↓
매칭된 부분 → KV 캐시 그대로 재사용 (프리필 건너뜀)
매칭 안 된 부분 → 새로 계산하고 트리에 노드 삽입
   ↓
GPU 메모리 부족 시 → LRU로 리프 노드부터 재귀적 축출
```

Eviction is implemented with an LRU policy that recursively removes leaf nodes, and removing from the leaves first is crucial because **deleting an intermediate node invalidates all its children below it.**

Why Tree??
- Partial matching is natural. How much overlap exists can be determined with a single tree traversal.
- Branching is free. It elegantly handles conversational branches, allowing agents to explore alternative paths without recomputing past attention weights.
- Eviction units are semantic. Discarding from the leaves (the most recent detailed branches) is advantageous for hit rate.

#### Cache-Aware Scheduling

RadixAttention's performance doesn't come from the data structure alone; **the scheduler must cooperate for it to perform well.**

For a single worker, requests in the queue are sorted by the length of their matched prefix.

> Why is this beneficial? If requests sharing a large prefix are executed consecutively, the shared node will be fully utilized before it's evicted. Conversely, if they are executed out of order, the node might be evicted and then needed again, leading to recomputation. The hit rate can differ even with the same cache.

When scaled to multiple servers, it becomes a **cache-aware load balancer, SGL Router**.

If requests are distributed round-robin, identical prefixes scatter across different servers, rendering the cache ineffective. Requests with the same prefix should be sent to the same server.

**Numerical Improvement Metrics**
- 5x higher throughput due to KV cache reuse and parallelism.
- Workloads with over 60% prefix overlap see significantly reduced TTFT with a 75-95% cache hit rate, while workloads with only completely unique prompts gain no benefit.
- For multimodal models, the hash of the input image can be used as a key in the radix tree to reuse the KV cache for the same image, resulting in up to 6x higher throughput in such benchmarks.

Trade off
- **Net loss if no sharing:** Without sharing, when there are no common radixes, RadixAttention incurs a small overhead. For simple workloads without repeating prefixes, it's better to use vLLM.
- The token sequence, including special tokens, must match exactly. The tree structure adds a slight memory overhead compared to a flat array, and finding eviction candidates incurs heap operation costs.
- Partial pages at the end of a sequence are not cached. This design ensures page-level sharing and efficient memory allocation.

### Example

```bash
# 축출 정책 선택 (기본 LRU)
--radix-eviction-policy lru # lru lfu fifo mru filo priority

# 페이지 정렬 (기본 1 = 정렬 x)
--page-size 16

# 디버깅/콜드스타트 벤치마크용으로끄기
--disable-radix-cache
```

> Why is `disable-radix-cache` needed? When benchmarking, if the cache is enabled, subsequent runs will show unrealistically fast numbers. It's turned off to measure cold start performance.

### Case Study - Document QA / RAG Service

Let's say there's a service where users ask different questions about the same 20K-token contract document.

```
요청 구조: [시스템 프롬프트 500][계약서 본문 20,000][질문 30]
                    └──────── 20,500 토큰 공유 ────────┘  └ 30토큰만 다름
```

The **sharing rate is 99.85%**. This is precisely the kind of workload for which RadixAttention exists.

The first question prefills 20,500 tokens, and subsequent questions only compute 30 tokens, reducing TTFT from hundreds of milliseconds to tens of milliseconds.

By attaching an **SGL Router** and routing requests with the same document ID to the same worker, the hit rate is maintained even across multiple servers.

### Hierarchical Cache and Distributed KV Store

This is about scaling beyond the GPU, but the radix tree also ultimately has **GPU memory size as its upper limit.** In the case study above, if there are 1,000 types of documents, they cannot all be loaded onto the GPU. If there are multiple servers, **each only knows its own cache, leading to duplication.**

Therefore, the mechanism is to push the KV cache to layers outside the GPU.

KV blocks are moved in order from GPU HBM -> CPU RAM -> SSD -> Redis, S3, Mooncake, etc., and only relevant blocks are fetched when needed using high-throughput connectors. Multiple vLLM instances can share a single KV cache pool, and since the LMCache server runs as an independent daemon, the cache state is preserved even if the engine dies and restarts.

- LMCache: Hierarchical KV offloading layer integrated with vLLM.
- Mooncake: KVCache-centric distributed architecture, Transfer Engine + Store.
- NIXL / RDMA: High-speed KV transfer between nodes.

The Mooncake Transfer Engine was officially integrated into vLLM in December 2024 and SGLang in April 2025, and FAST received the 2025 Best Paper Award.

Particularly important is non-prefix reuse (CacheBlend).

Prefix caching had a fundamental limitation. In RAG, a characteristic was that **the order of retrieved chunks changes every time.**

```
요청 1: [시스템][청크 A][청크 B][질문]
요청 2: [시스템][청크 C][청크 A][질문]   ← 청크 A가 두 번째 위치
```

Chunk A's KV depends on what precedes it, so if its position changes, prefix caching cannot be used.

CacheBlend therefore reuses KV blocks at arbitrary positions by selectively recomputing only a few tokens to restore quality. This is the solution for RAG, which stitches together new retrieved chunks every time.

> Why it works (intuition) - Chunk A's KV doesn't completely change based on its position. Most of it remains similar, and only a few tokens near the boundaries change significantly. Recomputing just those few tokens approximately restores it.

This three-tiered structure allows a single H100 80GB to handle far more concurrent users than GPU memory alone would permit, but at the cost of increased latency for cold block access.

The **bandwidth cliff** calculated above applies directly here: **if recomputation is cheaper than transfer, offloading is a loss.** One must calculate the break-even point for their workload before adoption.

#### SGLang's Direction

SGLang's Q2 2026 roadmap includes making hierarchical cache and hybrid attention native features. It also aims to support flexible session control for agentic workloads. This means directly addressing the issues that break prefix caching in agent workloads (long sessions, frequent branching, tool result insertion).

## SGLang

Now that we've looked at RadixAttention and similar, let's explore the other half of SGLang.

SGLang comes from Structured Generation Language.

The original paper title is **Efficient Execution of Structured Language Model Programs**, and the word **program** here is key.

### Not a Request, but a Program??

Traditional serving engines viewed **one request = one prompt = a complete unit**.

However, actual LLM applications look like this.

```py
# Actual application scenario
document = read(file)
summary = LLM(f"Summarize {document}")
questions = LLM(f"Generate 5 questions from {summary}")
for question in questions:                    # ← Branches into 5 here
    answer = LLM(f"{document}\nAnswer {question}")  # ← Prefill the document 5 times again!
evaluation = LLM(f"Which of {answers} is the best?")
```

In the example above, from the engine's perspective, these become **8 unrelated requests.** This leads to prefilling the document 6 times. However, from a human perspective, it's clearly a single program with a structure.

SGLang's starting point is this realization. Instead of treating each prompt as an isolated request, it aligns inference execution with actual user patterns.

```
┌────────────────────────────────────────┐
│  Frontend (DSL)                        │
│  - Informs the engine about the program's structure         │
│  - Primitives like gen, fork, join, select │
└────────────────┬───────────────────────┘
                 │ Structural Information
┌────────────────▼───────────────────────┐
│  Runtime                                  │
│  - RadixAttention (uses structure for cache reuse) │
│  - Compressed FSM (uses structure for decoding acceleration)       │
│  - Zero-overhead scheduler                  │
└────────────────────────────────────────┘
```

> The key is that when the frontend **informs the runtime about the structure**, the runtime transforms that structure into an optimization opportunity, which is the co-design approach.

The practical reality is, frankly, **most teams don't use the SGLang DSL.** They connect via an OpenAI-compatible API and only leverage RadixAttention and runtime optimizations. This is still viable.

The frontend always observes the entire prompt runtime, and the runtime automatically performs prefix matching and reuse caching. **RadixAttention works automatically even without the DSL**, so the DSL is an optional choice that provides additional benefits when using explicit branching (fork), for example.

### Let's Create a Compressed FSM, Jump-Forward Decoding Structure (for free)

As SGLang's second representative technique, structured output is covered in detail in Chapter 9, but since this technique is unique to SGLang, we'll look at it again here.

#### Problem

Let's say we want to force JSON output. The traditional method uses a **FSM (Finite State Machine)** to, at each step, keep only the tokens allowed by the current state and set the probabilities of the rest to 0.

This token-by-token approach is inefficient when there's an opportunity to decode multiple tokens at once. For example, a constant sequence like `{"summary": "` spans multiple tokens in regular decoding, requiring several decoding steps, even though there's only one valid next token during decoding.

```
{  "  summary  "  :  "     ← 7 model passes
↑ Only 1 choice at each position
```

**The answer is fixed, but we're asking the model 7 times.**

SGLang's compressed FSM can overcome this limitation. The runtime analyzes the FSM, compresses adjacent single-transition edges into a single edge, and recognizes when multiple tokens can be decoded together.

Multiple tokens on a compressed transition edge can be decoded in a single forward pass, significantly accelerating decoding.

```
Regular FSM:   (s0)-{→(s1)-"→(s2)-s→(s3)-u→(s4)-m→ ... (7 steps)
                 ↓ Compresses single transition edges
Compressed FSM:   (s0)---{"summary": "--→(s7)               (1 step)
```

**Jump-forward decoding** leverages this compression at runtime.

When the next output can be deterministically inferred from the grammar based on the current input, it skips LLM decoding and sampling, directly tokenizes that output, and appends it to the context.

**Why does this work?**
- It's information-theoretically obvious: if the next token is uniquely determined, its entropy is 0. Asking the model about something with zero entropy is a waste of time. The grammar already knows the answer, so we can just use it.

> To use an analogy, when filling out a form with a fixed format, if item names like 'Name' and 'Date of Birth' are already printed, the traditional method is to repeatedly instruct to write 'Name: Kim Ddaeng-ddaeng', whereas jump-forward decoding is like using the pre-printed form.

How much improvement is there when the next few tokens are inevitable due to grammar? Closing curly braces, field names, and fixed JSON punctuation can be emitted by the engine in one go without a model forward pass, meaning heavily structured outputs can be generated with fewer model calls than the number of tokens.

This leads to a **surprising result**: heavily structured outputs can be faster than unconstrained generation. While constraints are usually thought of as overhead, here it's the opposite.

**Trade-offs**
- Applies only to **structures expressible by regular expressions/grammar**. There's no benefit within free-text fields.
- Building a compressed FSM incurs compilation costs (a burden if the schema changes frequently).
- Applying grammar constraints itself adds per-token overhead, and its impact varies with grammar complexity.

### Zero-overhead CPU Scheduler

Earlier, I mentioned CPU overhead as a trade-off for continuous batching.

This is because a Python scheduler runs at each iteration, clearing the queue, forming batches, and updating the Radix Tree.

The problem is that **the GPU might be idle during this time**.

```
Traditional method (serial):
[CPU scheduling 3ms][GPU forward 8ms][CPU scheduling 3ms][GPU forward 8ms]...
                   ↑ GPU idle 3ms      ↑ GPU idle 3ms
GPU utilization = 8/11 = 73%
```

This ratio worsens for smaller models and smaller batches. For an 8B model, if the forward pass takes 3ms and scheduling takes 3ms, **the GPU only works half the time.**

#### Mechanism

**Overlap scheduling and GPU execution**. While the CPU prepares the next batch, the GPU computes the current batch.

```
Overlapping method:
GPU: [Forward N][Forward N+1][Forward N+2]...   ← Never idle
CPU:   [Prepare N+1] [Prepare N+2] [Prepare N+3]...   ← Prepares one step ahead
GPU utilization ≈ 100%
```

The composition of the next batch can **mostly be determined without knowing the full output of the current batch.** While it's not certain which requests will finish, it's possible to pre-calculate who to put in the waiting queue and which KV blocks to use, thus it's a **speculative preparation structure that corrects if it goes awry.**

SGLang provides RadixAttention for prefix caching, a zero-overhead CPU Scheduler, Prefill-Decode separation, speculative decoding, continuous batching, PagedAttention, tensor/pipeline/expert/data parallelism, structured output, chunked prefill, quantization (FP4/FP8/INT4/AWQ/GPTQ), and multi-LoRA batching.

The zero-overhead scheduler was introduced in v0.4.

Trade-offs
- Implementation complexity is high. In fact, for Q2 2026, the SGLang roadmap included a refactoring goal to simplify the Stage->Worker->Executor->Engine structure to Stage->Engine, reducing approximately 33,000 lines to 10,000 lines and lowering the request path depth from 8-10 to 6, all without sacrificing accuracy or performance.
- This means work is underway to remove the complexity accumulated for performance.

### SGLang's Current Position as of August 2026

In October 2025, TPU native execution was added with the SGLang-Jax backend, and in November 2025, SGLang Diffusion extended its scope to accelerate video and image generation.

As of the July 2026 release, it supports GB300/B300, RTX PRO 6000 Blackwell server edition, DGX Spark, and Jetson Thor, and provides FP8 precision support on Hopper and above.

This article focuses on goals such as full compatibility and production-level reliability across P/D separation, all parallelization methods, and speculative decoding.

An interesting item here is the support for SGLang Gateway as a DP scheduler for RL framework rollouts, and in gRPC mode, the gateway actively repairs the SGLang server's KV Cache to improve routing decisions.

> While GLM slime enabled long-term agent RL with asynchronous rollouts, SGLang is the engine that actually runs those rollouts. GLM-4.5's core is powered by the open-source slime framework released by THUDM, and slime is SGLang native, which **can be seen as a significant change, becoming part of the inference engine's learning infrastructure.**

<br>

## Attention Kernel

To actually speed up architectures like MLA, DSL, and Gated DeltaNet,
GPU kernels are necessary. This chapter will explain why.

### FlashAttention - Rescuing Attention from Memory

Why is attention slow? A naive attention implementation looks like this:

```
S = Q @ K.T          # Writes an [N, N] matrix to HBM
P = softmax(S)       # Reads [N, N] again, writes again
O = P @ V            # Reads [N, N] yet again
```

The N, N matrix is the problem. If N=8,192, there are 67 million elements, and this involves **repeated writes and reads to HBM.** HBM access is the biggest bottleneck, and attention doing it three times is the issue.

While the GPU performs computations quickly, the memory transfer process for exchanging intermediate results becomes the bottleneck.

#### Tiling and Online Softmax

FlashAttention demonstrated that tiling and online softmax can reduce HBM reads and writes from quadratic to linear with respect to sequence length, achieving a measured 2-4x speed improvement.

Two ideas were combined:
- **Tiling:** Instead of creating the entire [N, N] matrix, Q and K are split into small blocks and processed within GPU SRAM (on-chip high-speed memory). SRAM is much faster than HBM but is very small, in the tens of KB range, so processing must be done in blocks.
- **Online Softmax:** Softmax originally requires seeing the entire row for normalization. But what if you **carry around the cumulative maximum and cumulative sum and update them incrementally?** This allows for accurate results even when processing in blocks.

```
Process Block 1 → Update (partial sum, partial max)
Process Block 2 → Rescale previous results based on new max and accumulate
...
Complete accurate softmax without ever bringing the entire thing into memory
```

This works because there's a trade-off: **you do a bit more computation to save memory.**

Since GPUs have ample compute but limited memory, it's beneficial to make the trade-off of doing more computation to reduce memory access.

FlashAttention-2 achieved 50-73% of theoretical peak performance on A100, and FlashAttention-3 utilized Hopper's asynchronous features TMA, WGMMA, and FP8 quantization to achieve 840 TFLOPS (85% utilization) on H100.

There is the problem of **having to rewrite it for each hardware.** The changes in FA2, FA3, and FA4 are not algorithmic but rather **rewrites to utilize the asynchronous instructions and low-precision tensor cores of new hardware.** This is why kernel development is difficult.

### FlashInfer

It's easy to confuse with FlashAttention, but their purposes are different. FlashInfer is a kernel library for serving.

#### Why Just Fast Attention Isn't Enough

FlashAttention speeds up attention computation. However, serving engines have requirements that it alone cannot meet.

- The kV cache has a paged layout, so it must be read by **following a block table.**
- Load balancing is needed even if sequence lengths within a batch are **all different.**
- Variations are needed to handle **compressed KV** like in MLA.
- It's beneficial to process requests sharing a prefix **all at once.**

FlashInfer extends to an inference operator library, handling block-sparse KV attention (used by vLLM's PagedAttention), multi-head latent attention (MLA, a compressed KV variant in the DeepSeek family), and more.

FlashInfer separates the plan/run stages of attention computation, scheduling variable-length input computations at the plan level to alleviate load imbalance issues.

In terms of memory efficiency, it provides cascade attention for hierarchical KV caching and implements Head-Query fusion for GQA acceleration.

**Cascade Attention** is interesting.

#### ex.

Let's assume 32 requests share a system prompt of about 2000 tokens, with each having its own 200 tokens appended.

Naively, attention on 2000 tokens would mean reading the shared part 32 times.

**Cascade:** The shared part is read only once, processed for 32 queries, and then the unique parts are processed separately and combined.

This allows the two results to be accurately merged thanks to the associative property of online softmax.

This is the kernel counterpart of RadixAttention, saving computation with caching and memory reads with cascading.

SGLang routes attention through FlashInfer by default on both Hopper and Blackwell. In vLLM, FlashInfer is the default on Blackwell (B200/B300), while FlashAttention is the default on Hopper (H100, H200), and FlashInfer is optional.

#### Why Do New Architectures Need New Kernels?

| Architecture | Why standard kernels don't work | What's needed |
|----------|-------------------------|------------|
| MLA (DeepSeek) | KV is a compressed latent vector, so the data layout differs from standard attention | FlashMLA / FlashInfer MLA path |
| DSA (DeepSeek, GLM) | The indexer must gather and attend to only selected non-contiguous top-k tokens | Sparse gather kernel |
| Gated DeltaNet (Qwen) | Not attention, but recurrent state update. Requires chunk-wise parallel algorithm | Triton kernel for Flash Linear Attention |
| KDA (Kimi) | GDN + channel-wise gating + MLA mix | Dedicated kernel |

Indeed, to support Qwen3-Next, vLLM integrates Flash Linear Attention's Triton kernel and introduces a hybrid KV Cache manager that handles both linear attention layers and full attention layers, avoiding fragmentation and maximizing GPU utilization.

> A crucial lesson here is the weight of "day-0 support" when a new model emerges: if the architecture is new, kernels and memory managers must be rewritten. The day-0 support for sparse attention in SGLang and DeepSeek-V3.2 is the result of significant engineering collaboration.

As kernel library dependencies increase, **installation and version management become complex.** Support status varies by hardware, and cases where a particular kernel doesn't work with a certain combination frequently occur in practice.

### CUDA Graphs - Eliminating Kernel Execution Overhead

#### Problem

The cost of calling kernels means that a single model pass involves the sequential execution of hundreds or thousands of CUDA kernels.

The act of instructing the CPU and GPU to execute each kernel incurs microsecond-level costs, and for 1,000 kernels, this alone amounts to several milliseconds.

If a decode step takes 8ms and kernel launch overhead is 3ms, approximately 37% is wasted.

#### Solution

The kernel call sequence is **recorded (captured) once to create a graph**, and then the entire graph is replayed with a single command for optimization.

```
Without recording (every step)
CPU: [Instruct Kernel1][Instruct Kernel2][Instruct Kernel3] ... [Instruct Kernel1000]   ← 1,000 instructions
GPU:   └Execute┘    └Execute┘    └Execute┘           └Execute┘

After recording
CPU: [Instruct Graph Replay]                                        ← 1 instruction
GPU: [Kernel1][Kernel2][Kernel3] ... [Kernel1000]                       ← GPU executes sequentially on its own
```

#### Why Does This Solution Work?

Because the order and types of kernels are identical at every step.

Since the model structure is fixed, the list of kernels called at the 1,000th decode step is exactly the same as at the 1st step.

What changes is only **the content of the memory addresses holding the data, not the call order itself.**

This is why repeating the same instruction 1,000 times was wasteful, and also why recording it once is sufficient.

> It's like a supervisor on an assembly line, where the process is the same every day, repeatedly shouting "process 1, process 2, process 3" a thousand times. Since the order is always the same, they can just leave a work instruction sheet and say "do it this way" once. Of course, there would be a cost to writing the instruction sheet.

+ CUDA graphs require a fixed shape. However, continuous batching changes the batch size at each iteration. This contradicts the part above about why the solution works.

Therefore, a workaround is to pre-capture multiple graphs for frequently used batch sizes like 1, 2, 4, 8, 16, 32... and use the closest one, padding the remaining slots.

In other words, the constraint that the shape must be fixed is resolved by pre-creating all usable shapes.

#### Tradeoff

- Graph capture during server startup takes **tens of seconds to several minutes.**
- Captured graphs occupy GPU memory -> reducing the share for KV cache.
- When debugging, turning it off with `--enforce-eager` (vLLM) / `--disable-cuda-graph` (SGLang) makes root cause analysis easier but slows down performance.

FlashInfer kernels are designed to be capturable with CUDAGraph and torch.compile, supporting low-latency inference.

This is why kernel libraries must explicitly guarantee this compatibility.

<br>

## Parallelization Strategies - Splitting Models Across Multiple GPUs

The previous content doesn't fit on a single GPU. DeepSeek V4-Prox is 1.6T, and Kimi K3 is 2.8T. **How to split them** is the topic of this chapter, and each splitting method has different communication patterns and bottlenecks.

### TP, Tensor Parallelism

A single weight matrix of the model can be larger than GPU memory.

And we want to use the computational power of multiple GPUs **simultaneously for a single request.**

#### Solution - Splitting the Matrix

The matrix is split, each GPU computes a portion, and the results are combined.

```
원래:  Y = X @ W          (W는 [5120 × 20480])

TP=4:  W를 열 방향으로 4등분 → W1, W2, W3, W4  (각 [5120 × 5120])
       GPU0: Y1 = X @ W1
       GPU1: Y2 = X @ W2
       GPU2: Y3 = X @ W3
       GPU3: Y4 = X @ W4
       → all-gather로 [Y1|Y2|Y3|Y4] 결합
```

Attention is **split by head.** For example, 64 heads are divided among 4 GPUs, with 16 heads each.

#### Effects and Trade-offs

The advantage is reduced latency; since the computation for a single request is distributed across 4 GPUs, individual responses become faster.

However, communication occurs at every layer, requiring all-reduce and all-gather, which directly depends on the inter-GPU link bandwidth.

```
NVLink (노드 내부)  : 수백 GB/s ~ TB/s   → TP 가능
InfiniBand (노드 간): 수십~수백 Gb/s      → TP 하면 통신에 잡아먹힘
```

> TP should only be used within a node. Cross-node TP is always a bad choice. For a single node with 8 x H200, TP=8 is the upper limit.

### PP, Pipeline Parallelism

**This is a solution that divides layers by depth.**

It's a technique where an 80-layer model is distributed across 4 nodes, with 20 layers per node.

```
노드0: 층 1-20  → 노드1: 층 21-40 → 노드2: 층 41-60 → 노드3: 층 61-80
       └─ Only activations are passed (small) ─┘
```

#### Effects and Trade-offs

Inter-node communication is much smaller than with TP; only a single activation tensor needs to be passed at layer boundaries.

Suitable for inter-node scaling.

The trade-off is pipeline bubbles. When Node 0 is working, Nodes 1-3 remain idle.

```
시간 →
노드0: [배치1][배치2][배치3][배치4]
노드1:       [배치1][배치2][배치3][배치4]
노드2:             [배치1][배치2][배치3]
노드3:                   [배치1][배치2]
       └Bubble─┘                        └Bubble┘
```

Splitting into smaller micro-batches reduces bubbles but cannot eliminate them entirely. It's less used in inference than TP, primarily when memory is severely limited.

(If TP splits model weights into multiple parts, PP splits request processing in parallel.)

### EP, Expert Parallelism - Core of MoE Serving

Models like Qwen, Kimi, GLM, and DeepSeek are all MoE, making this the most crucial parallelization strategy in practice.

#### Problem: Why shouldn't MoE be split with TP?

DeepSeek-V3 series MoE layers have 256 experts, activating 8 per token.

What happens if these 256 experts are divided using TP?

TP **splits the weight matrix** of each expert, placing fragments on all GPUs.

Then, regardless of which expert is selected, all GPUs must participate, leading to **full communication every time**. This is inefficient.

#### Solution: Assigning Experts Wholly

There is a solution where experts are assigned wholly to GPUs.

```
EP=32일 때: GPU 하나당 전문가 8개씩 담당

Tokens select experts #17, #93, #201, ...
  ↓
[all-to-all communication] Each token is sent to its responsible GPU
  ↓
Each GPU computes with its assigned expert
  ↓
[all-to-all communication] Results are returned to their original positions
```

Since an expert's computation is independent, if it's placed entirely on one GPU, it completes within that GPU. Communication is only for routing tokens, which is much cheaper than moving weights.

Another big advantage is that with fewer experts per GPU, the memory burden per GPU decreases, and thus more space becomes available for the KV cache. This directly translates to batch size and throughput.

#### Trade-offs

The trade-offs are the all-to-all hell and load imbalance. All-to-all itself is very expensive because every GPU communicates with every other GPU. This is the biggest bottleneck for MoE serving, which is why dedicated communication libraries like DeepEP have emerged.

The characteristic of **imbalanced expert load** is also a problem. In real traffic, certain experts are chosen much more frequently. Only the GPUs responsible for those experts become overloaded, while others remain idle.

The solution is EPLB (Expert Parallelism Load Balancer), which replicates popular experts across multiple GPUs or rebalances batches.

#### Effect

SGLang, on 12 Atlas Cloud nodes (each with 8 x H100), achieved 52.3k input tokens/sec and 22.3k output tokens/sec per node for a 2,000-token input sequence, using prefill-decode separation and large-scale expert parallelism.

For local deployment, it costs $0.20 per million output tokens, about one-fifth the cost of the official DeepSeek chat API. Compared to using pure tensor parallelism with the same resources, output throughput can be up to 5 times higher.

This number demonstrates how much difference the choice of parallelization strategy makes, **up to 5 times compared to using only TP on the same hardware.**

SGLang supports PD separation and large-scale EP, including the full functionality of DeepEP, DeepGEMM, and EPLB.

Consequently, EP is optimized by differentiating the criteria for weight splitting and the resulting communication methods.

MoE is structurally composed of a common Attention network + numerous expert FFN networks.

General parallelization TP **splits all layers, including Attention and FFN, and then all GPUs share fragments of a single giant neural network matrix.**

EP **splits only FFN units independently**. For example, if there are 8 experts and 8 GPUs, each GPU stores one expert entirely, and the common Attention area is handled by replicating it with DP instead of splitting it.

### DP Attention, Data Parallel Attention - MLA-Specific Optimization

MLA stores KV in a compressed format. However, if attention is split using TP, each GPU might redundantly store this compressed KV. This means the memory saved by compression is lost again through replication.

Therefore, attention is treated separately. The attention layer is processed with DP (Data Parallel) instead of TP. This means each GPU handles different requests, performing attention entirely, and only cooperates with EP in the MoE layer.

```
Attention Layer: GPU0=requests 1~16, GPU1=requests 17~32, ... (each holds its own KV)
MoE Layer:    expert parallelism with all-to-all
```

The trade-off here is a disadvantage in small batches.

Data parallel attention is not recommended for low-latency, small-batch use cases because it is optimized for high-throughput scenarios with large batches.

> This is because DP processes independent sets of requests per GPU, so **if there are few requests, GPUs remain idle. TP, however, divides a single request among multiple GPUs, so all GPUs work even with small batches. The optimal choice depends on traffic volume.**

```bash
# SGLang에서 DeepSeek 계열 서빙 시
--enable-dp-attention --dp-size 8
```

### Parallelization Selection Guide

| Strategy | Splitting Axis | Communication Volume | Suitable Scope | Caution |
|---|---|---|---|---|
| TP | Matrix (width) | High (per layer) | Within node | Disaster if cross-node |
| PP | Layer (depth) | Low | Across nodes | Pipeline bubble |
| EP | Expert | all-to-all | MoE essential | Load imbalance |
| DP Attention | Request | None (attention layer) | MLA + Large batch | Unsuitable for small batches |

### Example

#### Case 1, 8 x H200 1 node, 70B class

```bash
--tp-size 8 # Since it's within a node, TP 8 is safe with NVLink.
```

#### Case 2 (12 nodes, 96 GPUs, DeepSeek V4-class MoE)

```
Prefill instances: Multiple small ranks, e.g., TP=4
Decode instances: Large-scale EP (tens of ranks) + DP Attention
Connected by PD separation <- More on this below
```

In GB200 NVL72 experiments, large-scale EP with 48 ranks was used for decode, and for prefill, 4 ranks per instance for high-precision settings and 2 ranks for low-precision. The key is the asymmetry: **small ranks for prefill, large ranks for decode.** Let's explore why below.

<br>

## PD Separation - Separating Prefill Decode

Let's start with the question: **Isn't it inherently strange to run two tasks with different characteristics on the same GPU?**

### Why Separate?

Let's look at what chunk prefill alone cannot solve.

Chunk prefill could reduce interference from decode operations when a large prompt came in, but fundamentally, **prefill and decode have diametrically opposite optimal settings.**

- **Prefill**: compute-bound, optimal batch size can be small. It already has many tokens, small TP is optimal for parallelization, TTFT is the latency metric, and quantization gain is moderate.
- **Decode**: memory-bound, optimal batch size is large (must fill the GPU). Large EP is the optimal batch, ITL is the latency metric, and quantization gain is significant.

If they are on **the same GPU, you end up with a compromise that is not optimal for either side.**

#### The solution is to divide the GPU pool by role.

```
[Request]
                       ↓
              ┌────────────────┐
              │ Prefill Instance │  TP=4, Small Batch
              │  (2~6 nodes)   │  Compute Intensive
              └────────┬───────┘
                       │ KV Cache Transfer ★
                       ↓
              ┌────────────────┐
              │ Decode Instance  │  EP=48, Large Batch
              │  (10~12 nodes) │  Memory Intensive
              └────────┬───────┘
                       ↓
                  [Streaming Output]
```

**The core challenge is KV cache transfer.** The several GBs of KV generated by prefill must be moved to the decode nodes, and if this is slow, the separation loses its meaning.

This is where the **Mooncake Transfer Engine** comes in. Mooncake is a transfer engine used for KV Cache transfer for prefill-decode separation, employing techniques similar to DeepEP to support NVLink.

This solution works because each pool can be independently tuned to its characteristics.

- Prefill Pool: Scale to TTFT SLO, small batch, TP small
- Decode Pool: Maximize throughput, large batch, large EP, aggressive quantization
- **Ratio can be adjusted according to traffic:** For workloads with long inputs, increase the proportion of prefill; for short inputs and many requests, increase the proportion of decode.

Out of 14 nodes provided by GB200 NVL72, 12 were used for decode and the rest for prefill, roughly simulating a real-world scenario where 6 prefill nodes and 12 decode nodes are used when there are 18 nodes.

One can get a sense that **a prefill 1: decode 2 ratio** is a common starting point.

#### Effects - Most Impressive Value Examples in this Chapter

| Environment | Performance | Multiplier |
|------|------|------|
| 96 H100s, PD + Large EP | 52.3k input / 22.3k output tok/s per node | 5x output compared to TP |
| GB200 NVL72, PD + Large EP | 7,583 decode tok/s per GPU | 2.7x compared to H100 |
| GB200 + FP8 Attention + NVFP4 MoE | 26,156 input / 13,386 output tok/s per GPU | 3.8x prefill, 4.8x decode compared to H100 |

When using FP8 Attention and NVFP4 MoE, SGLang achieved 26,156 prefill and 13,386 decode tokens/second per GPU on DeepSeek V3/R1 for a 2000-token input sequence, which is 3.8x and 48x faster, respectively, compared to H100 settings. With traditional BF16 Attention and FP8 MoE, it still achieved 18,471 and 9,087 tokens/second.

> Let's pay attention to the last two lines. Lowering precision alone results in prefill going from 18,471 to 26,156 (approx. 1.4x), which shows why quantization is demonstrated in the next chapter.

TradeOff
- **System complexity significantly increases:** Requires two types of clusters, a transfer layer, and routing logic.
- **KV transfer becomes a new bottleneck and a new point of failure.** Without RDMA/NVLink-grade networking, the benefits disappear.
- **It's a net loss at a small scale.** If you only have 8 GPUs, dividing them into 3 for prefill and 5 for decode would make both inefficient.

> Rule of thumb: Only consider this when all three conditions are met: at least tens of GPUs, stable high traffic, and high-speed interconnect.

<br>

## 투기적 디코딩 Speculative Decoding

Decoding is memory-bound, leaving compute resources idle.

Speculative decoding is an inference optimization technique that accelerates text generation in large language models (LLMs) by having a fast, lightweight draft model predict multiple subsequent tokens, and then a slower but accurate target model validates them in a single parallel operation to maximize speed.

Decoding generates only one token at a time. However, to generate that single token, **hundreds of GBs of model weights are all read from memory.** If we're going to read them anyway, **can't we confirm multiple tokens at once?**

The idea is to load only the weights of a tiny model to generate a draft, and then the target model validates it in a single parallel operation at the end.

> The point is to do it with a few GBs, a few GBs, a few GBs, a few GBs, and then hundreds of GBs, rather than hundreds of GBs, hundreds of GBs, hundreds of GBs, hundreds of GBs.

The solution begins with "**proposing and validating all at once.**" The idea emerged to propose γ draft tokens, have the target model validate them in a single pass, accept α matching prefixes, emit α+1 tokens, and restart drafting from the first rejected token.

```
Draft (small and fast): Proposes 6 tokens: "the cat sat on the mat"
                     ↓
Target (large and accurate): Simultaneously validates 6 tokens in one forward pass
                     ↓
Matches up to "the cat sat on" → 4 accepted + 1 generated by target = 5 tokens emitted
Mismatch from "the" → Restart drafting from here
```

Crucially, since all draft tokens undergo target model validation before emission, speculative decoding accelerates inference without altering the final output quality.

This is not an approximation but precisely lossless. This property makes speculative decoding special.

The cost for the target model to simultaneously validate 6 positions in one forward pass is almost the same as validating 1 position.

This is because both read the entire weights once. Although the computational load is 6 times higher, computation resources are abundant, so the time increase is negligible.

> The difference between taking 10 minutes to put a dictionary on the desk and, once it's there, looking up 6 words versus 1 word, is negligible.

#### 해법의 변형 - 드래프트를 만드는 방법들

Common approaches include small draft models, multi-token prediction (MTP), Medusa-style multi-head prediction, and feature-level drafting methods like EAGLE-3, DFlash, and DSpark.

| Method | Mechanism | Feature |
|------|----------|-----:|------|
| Small Draft Model | Runs a separate small model of the same family | More advantageous with larger targets |
| MTP | Reuses multi-token prediction heads attached during training | No separate model needed |
| Medusa | Attaches multiple prediction heads in parallel | Simple to implement |
| EAGLE-3 | Lightweight drafter that takes target's hidden states as input | Production standard by 2026 |
| n-gram | Finds and copies repeating patterns in context | Strong in code editing |

> DeepSeek-V3's inclusion of MTP training as a goal, and GLM-5.2's improvement of the MTP layer to increase acceptance length from 4.56 to 5.47, are all part of this section's discussion. The heads attached during training become the drafters during serving. **Model design and serving design were a single decision.**

### 효과의 함정 - 배치가 커지면 이득이 사라진다.

A systematic evaluation of 5 speculative decoding variants (n-gram, EAGLE, EAGLE-3, draft model, MTP) across 4 models and 6 workloads in vLLM showed that EAGLE achieved up to 1.96x speedup for Llama-3-70B at batch size 1, but this dropped to 1.21x at batch size 128. At high concurrency, the system becomes compute-bound, reducing the idle GPU capacity available for speculation.

**This is the core trade-off of speculative decoding.**

```
Batch 1    : 99% idle compute resources → Speculation is free → 2x acceleration ✅
Batch 128  : Compute resources already saturated → Speculation takes resources → 1.2x ⚠️
Batch 256+ : Speculation can result in net loss ❌
```

It is recommended to skip EAGLE-3 if the batch size exceeds 32. At high concurrency, the acceptance rate of draft heads decreases, leading to the cost of two forward passes per step.

Long context decoding exceeding 32K tokens also suffers from a reduced acceptance rate due to the difficulty of context prediction by draft heads.

Validation accounts for 42-95% of execution time, with the target model's forward pass being the bottleneck.

### EAGLE-3, 3.1

The EAGLE family is typically the standard, primarily because **the drafter observes the target's internal state.**

Instead of simply running a separate small model, it takes the target model's hidden states as input to predict the next tokens, thus **aligning much better with the target.**

EAGLE 3.1, released in May 2026, fixed an interesting issue. The EAGLE team identified the cause of this vulnerability as attention drift.

This is a phenomenon where, as speculative depth increases, the drafter gradually shifts its attention from sync tokens towards the tokens it generated.

```bash
# SGLang
--speculative-algorithm EAGLE3 \
--speculative-draft-model-path <드래프터 경로> \
--speculative-num-steps 5 \
--speculative-eagle-topk 8
```

Set `speculative_num_steps` to 5 and `speculative_eagle_topk` to 8. With an acceptance rate of 0.82 and N-5, approximately 3.5-4 accepted tokens are generated per step.

SGLang's RadixAttention KV Cache is fully compatible with EAGLE-3 draft validation.

> Caution! - Drafters must be trained separately for each model. As of 2026, pre-trained EAGLE-3 heads exist for Llama 4 Scout/Maverick, Qwen 2.5 75B, DeepSeek V3 671B, GLM-5.1, etc. Using the official EAGLE training repository, custom heads can be trained in approximately 2-4 hours with 4 x H100s. First, check if a drafter exists for the model you intend to use.

- **Interactive, low concurrent user scenarios:** Highly recommended, as the small batch size maximizes benefits.
- **RAG, medium concurrency:** Re-evaluate batch size; enable if it's below 32.
- **Large batch serving:** Generally detrimental; it's better to disable it in throughput optimization scenarios.

### 양자화

Decoding is slow because of the **time it takes to read weights from memory.**

So, what if we make the weights smaller? It would be faster.

| Method | Target | Bits | Feature |
|------|----------|-----:|------|
| FP8 | Weights + Activations | 8 | Hardware acceleration on Hopper or newer. Small quality loss |
| NVFP4 / MXFP4 | Weights (mainly MoE) | 4 | Blackwell generation. Block-wise scale sharing |
| AWQ | Weights | 4 | Protects important channels based on activation magnitude |
| GPTQ | Weights | 4 | Sequentially quantizes layer by layer, minimizing error |
| KV Cache FP8 | KV Cache | 8 | Halves the byte term in the 1-chapter formula |
| QAT | From training phase | 4 | Least loss after compression |

This solution works for two reasons:

1. **LLM weights have high redundancy.** Most values are concentrated in a narrow range, so reducing bits results in less expressiveness loss than expected.
2. **Quantization reduces both memory and computation simultaneously.** 4-bit weights take 1/4 the read time, and hardware like Blackwell can perform calculations faster with low-precision tensor cores.

**AWQ is sometimes better than GPTQ,** starting from the observation that weights corresponding to large activation values are important, and thus protects those channels. This is because it doesn't treat all weights equally. Such a snowball effect creates good synergy when it works well.

- On GB200, switching from BF16 attention + FP8 MoE to FP8 attention + NVFP4 MoE improves prefill from 18,471 -> 26,156 and decode from 9,087 -> 13,386 tokens/second, an improvement of approximately 1.4x.
- Adding FP8 KV Cache in the tuning ladder improves performance from 6,200 -> 7,100 tok/s (approx. 15%).
- Kimi K2.6's INT4, based on QAT, reported **approximately 2x inference speed and 50% GPU memory reduction** with negligible quality loss.

### TradeOff

- Quality degradation varies by workload. Degradation not seen in general chat often appears in math or coding, so it must be measured with your own evaluation set.
- Serving providers sometimes secretly quantize. Serverless hosting typically quantizes activations to FP8, leading to subtle differences compared to running public weights directly. This is the first thing to suspect when benchmarks are not reproducible.
- Hardware support is essential. FP8 requires Hopper or newer, and NVFP4 requires Blackwell. Even if a GPU without support claims to support them, it might be software emulation and could actually be slower.
- Indeed, release notes often list known issues where FP8 models fail on specific hardware.

<br>

## Structured Output - Reliably Receiving JSON

As agents and tool calls become standard, **whether a model outputs valid JSON** has become synonymous with system stability.

This leads to a problem: asking the model to "only output JSON" via a prompt is probabilistic. What if the model adds explanations at the beginning, omits a closing brace at the end, or creates fields not in the schema? This can lead to parsing errors and unstable applications.

Even with a 99% success rate, 1 million requests a day means **10,000 failures.** If an agent runs 20 steps, and each step has a 99% success rate, the overall success rate is 82%.

### Solution

The solution is to use constrained decoding. This involves **setting the probability of grammatically impossible tokens to zero** in the model's output probability distribution.

```
Current state: {"name": "김
Possible next tokens: arbitrary characters, or a closing double quote
Impossible: [ { } , : etc. structural tokens

  → Mask impossible tokens in the logit vector with -inf
  → Sampling will only yield valid tokens
```

The probability becomes a structural guarantee, not a request, achieving 100%.

### Implementation Methods

#### 1. FSM Approach - Outlines

For a given regular expression or grammar, it pre-calculates an index that maps each state of the automaton to a set of allowed tokens, thereby reducing step-by-step validation lookups to a single operation.

As a tradeoff, Outlines pioneered the FSM approach but faced issues with complex schema compilation times, ranging from 40 seconds to over 10 minutes. Among the engines tested in the JSONSchemaBench benchmark, it had the lowest compliance rate, primarily due to these timeout problems.

A fundamental limitation is that FSM engines fully pre-calculate token validity but cannot handle recursion and can have long compilation times. Schemas with nested objects repeating to arbitrary depths cannot be represented by an FSM because they are finite-state.

#### CFG Approach - XGrammer, llguidance

This approach uses Context-Free Grammars (CFG) and pushdown automata to handle recursion.

XGrammer's core idea is to **divide tokens into two categories.**

- **Context-independent tokens:** These tokens have their validity determined regardless of the current parsing stack. For example, a regular string token inside `"` in JSON grammar is always valid, whether it's nested three levels deep in an object or inside an array. This **can be pre-calculated.**
- **Context-dependent tokens:** These tokens require inspecting the token stack to determine their validity. A closing brace is a prime example; it's only valid if there's an open object, otherwise, it's a syntax error. These must be checked at runtime.

```
Current state: {"user": {"name": "김
Stack: [ object → object → string ]

Token "철"  → Context-independent. Always OK inside a string. (Candidate for pre-calculation)
Token "}"   → Context-dependent. Must close the string first, so currently impossible.
Token "\""  → Context-dependent. Closing a string, so currently possible.
```

If the vocabulary has 150,000 tokens, **the overwhelming majority are context-independent.** Structural tokens like parentheses or quotes are only a few dozen, and the rest are all content. Therefore, out of 150,000 tokens, only a few dozen or hundreds need to be decided in real-time.

XGrammer pre-calculates the context-independent parts of the mask and overlaps context-dependent checks with GPU execution, reporting an overhead of less than 40 microseconds per token for JSON schemas and context-free grammars.

Its successor, XGrammer2, replaced the predecessor's PDA/stack-based parser with an Earley parser and added optimizations like JIT compilation, achieving faster CFG compilation and mask generation.

#### Jump Forward - SGLang

This is orthogonal to the above methods. This technique is orthogonal to the constrained decoding adopted by XGrammer, and XGrammer has shown that by adding support for jump-forward decoding, combining the two further improves efficiency.

```
Constrained decoding: Blocks impossible tokens (accuracy)
Jump-forward: Skips certain tokens (speed)
      ↓
Combination: Accurate and fast
```

### New Requirements in the Agent Era - Dynamic Structured Generation

The problem is that the schema changes every time.

For tool calls, **the schema changes with each request.** The valid JSON structure varies depending on which tool the user has activated. It can be pre-compiled.

#### Current Status - Maturity by Backend

In the XGrammer-2 evaluation, the combination of SGLang v0.5.3.post3 + Outlines v0.2.11 did not support dynamic structured generation like tool calls at all. While llguidance v1.2.0 supports it, issues such as empty output or language drift from English to other languages occurred with Qwen3-0.6B.

> Practical Lesson - Don't just pick based on a 'structured output support' checkbox. Static schemas and dynamic schemas have entirely different requirements, and maturity varies by backend. If you're building an agent, you must test with dynamic cases.

- A few fixed JSON schemas XGrammar (default)
- Recursive/complex schemas XGrammar or llguidance (CFG required)
- Tool calls (dynamic schemas) XGrammer family, must be validated with real-world measurements
- Output with a very high structural proportion SGLang + Jump-Forward

Tradeoffs
- Grammar compilation cost (a burden if schemas change frequently)
- Per-token overhead varies with grammar complexity
- **Quality Impact:** Constraints can distort the model's natural distribution. Especially if the model's preferred expressions are blocked, it might resort to strange workarounds. If the schema is too restrictive, content quality can suffer.

<br>

## Real-world Example

Let's learn tuning with a hypothetical example. Let's reassemble the concepts learned above into a real decision-making sequence.

The GPU practice environment... well, it seems possible if you have a lot of money. But let's just assume it and try.

### Tuning Order Chart

When you encounter a performance problem, you shouldn't just tweak any setting. There's an **order of effect size.**

```
0단계  워크로드를 측정한다          ← 이걸 건너뛰면 나머지가 전부 추측
  ↓
1단계  엔진과 병렬화를 고른다        ← 나중에 바꾸려면 재설계
  ↓
2단계  연속 배칭 + 청크 프리필       ← 기본값. 안 켜져 있으면 켠다
  ↓
3단계  접두사 캐싱 / RadixAttention  ← 워크로드가 맞으면 단일 최대 이득
  ↓
4단계  양자화 (FP8부터)             ← 하드웨어가 되면 거의 공짜
  ↓
5단계  배치 파라미터 튜닝            ← SLO에 맞춰 곡선 위 지점 선택
  ↓
6단계  투기적 디코딩                ← 저동시성일 때만
  ↓
7단계  PD 분리 / 대규모 EP          ← 수십 GPU 이상에서만
  ↓
8단계  계층 KV 캐시                 ← 재사용률이 높고 GPU가 부족할 때만
```

A real tuning example for Llama 3.1 8B, A100 80GB, ShareGPT is as follows:

Baseline 4,200 -> Chunked Prefill 4,800 -> Prefix Caching 5,500 -> max-num-batched-tokens tuning 6,200 -> FP8 KV Cache 7,100 tok/s

Starting from the baseline, it's a 1.7x improvement in four steps. And all these four steps are at the **level of modifying configuration files**, not writing code.

### Step 0 - Workload Measurement (Important)

Before tuning, you need to know four numbers about your traffic:

1.  Input length distribution (median P95). Needed to determine prefill burden. If this value is large, chunked prefill is essential, and PD separation should be considered.
2.  Output length distribution. Needed to determine decode burden. Batching and quantization are important here.
3.  Prefix sharing rate. Needed to determine caching gains. This is the basis for choosing SGLang (Radix).
4.  Concurrency (peak). Needed to determine batch size. This is where speculative decoding's profitability is decided.

**How to measure prefix sharing rate**

```py
# Sample prompts from production logs to measure common prefix length
def measure_sharing_rate(prompts, tokenizer):
    tokens = [tokenizer.encode(p) for p in prompts]
    total_tokens = sum(len(t) for t in tokens)

    # Sorting makes adjacent prefixes overlap
    tokens.sort()
    shared_tokens = 0
    for i in range(1, len(tokens)):
        common = 0
        for a, b in zip(tokens[i-1], tokens[i]):
            if a != b: break
            common += 1
        shared_tokens += common

    return shared_tokens / total_tokens
```

The criterion is that workloads with over 60% prefix overlap see a 75-95% cache hit rate.

```
Sharing rate 0-20%   → vLLM is sufficient. RadixAttention offers little benefit.
Sharing rate 20-60%  → Both are worth trying.
Sharing rate 60%+    → SGLang strongly recommended.
```

### Example - In-house Coding Assistant

Situation
- 200 engineers, 25 peak concurrent connections
- Prompts 500 ~ 40,000 tokens (file paste)
- System prompt + in-house coding standards = 3,000 tokens (shared by all)
- Hardware 8 x H200 (141GB x 8 = 1,128GB)
- SLO: **TTFT p95 < 1 second, ITL < 50ms**

#### Step 0 Measurement Results

```
Input median 2,100 / p95 28,000 tokens
Output median 400 / p95 1,800 tokens
Prefix sharing rate approx. 35% (system prompt + frequently opened files)
Peak concurrency 25
```

#### Step 1 Engine and Parallelization

- The sharing rate is ambiguous at 35%, but if agent-like tool calls are expected to increase, the sharing rate will likely go up, so SGLang might be chosen.
- Since it's within a node, `--tp-size 8`.

#### Step 2 Batching

- p95 input is 28,000 tokens -> **Chunked prefill is essential**. If not, the questioner will pause for several seconds.

#### Step 3 Caching

- RadixAttention is enabled by default in SGLang.
- **Prompt redesign:** Move variable values (timestamp, username) after the system prompt -> the entire 3,000 tokens become cacheable.

#### Step 4 Quantization

- H200 is Hopper -> supports FP8, both weights and KV cache are FP8.
- With FP8 KV cache, the official byte count per entry goes from 2 -> 1 -> **2x concurrent users**.

#### Step 5 Batch Parameters

- Since there's an ITL 50ms SLO, batch size shouldn't be increased indefinitely.
- Based on concurrency 25, set `--max-running-requests` generously, but equalize latency with `--chunked-prefill-size`.

#### Step 6 Speculative Decoding

- Concurrency 25 -> 32 or less, so it's a profitable range. Apply EAGLE-3.
- Since it's a code editing workload, n-gram is also a candidate (many repetitive patterns).

#### Steps 7-8 - 8 GPUs are not the scale for PD separation, so skip.

The final configuration is as follows.

```bash
python -m sglang.launch_server \
  --model-path <model> \
  --tp-size 8 \
  --chunked-prefill-size 8192 \
  --kv-cache-dtype fp8_e5m2 \
  --quantization fp8 \
  --speculative-algorithm EAGLE3 \
  --speculative-draft-model-path <drafter> \
  --speculative-num-steps 5 \
  --speculative-eagle-topk 8 \
  --mem-fraction-static 0.90 \
  --enable-metrics \
  --host 0.0.0.0 --port 30000
```

```
Baseline                          1.0×
+ Chunked Prefill (latency stabilization) Significantly improves TTFT p95
+ Prefix Caching (prompt redesign)    Saves 3,000 prefill tokens
+ FP8 Weights + KV               ~2x concurrent users
+ EAGLE3 (low concurrency)             ITL ~1.5-2×
```

### Case 2 - Document QA / RAG

Situation
- 20K token contracts, multiple users asking different questions
- 1,000 document types, 80% of traffic surges on 50 popular documents
- Hardware 4 x H100

#### Key Decisions

**Sharing rate is overwhelmingly high.** Questions about the same document are 99% shared. -> SGLang

However, there's a problem. If there are 1,000 document types, not all can be loaded onto GPU memory. Let's try to keep the 50 popular documents alive in the cache and prevent them from being evicted.

Based on this solution, we'll review in three steps:

1.  **Cache-aware routing -** Hash document IDs to always send them to the same worker. With 4 servers, each server would handle only 250 types, leading to a 4x improvement.
2.  **Tiered Cache -** Offload KV for unpopular documents to CPU DRAM. Break-even calculation is essential: measure how many GB a 20K token KV is and how many ms it takes via PCIe, then check if it's cheaper than recomputing.
3.  **CacheBlend -** If it's true RAG where search chunk order changes every time, prefix caching becomes ineffective. Non-prefix reuse is needed.

The point to note is that this workload is overwhelmingly prefill-heavy, with 20,000 input tokens and 300 output tokens. Therefore:

-   Speculative decoding offers little benefit (short decode).
-   **Resources should be concentrated on prefill optimization.**
-   In extreme cases, PD separation, which involves increasing prefill-dedicated instances, is also a consideration.

### Case 3 - Large-scale MoE Serving

- DeepSeek V4 1.6T / 49B active or GLM-5.2 744B / 40B active
- 12 nodes x 8 GPUs = 96 cards NVLink / InfiniBand
- Goal: **Minimize cost, prioritize throughput over individual latency**

- Speculative decoding is a disadvantage due to large batches.
- PD separation is essential.
- Large-scale EP is also essential.
- DP Attention is suitable for large batches.
- Optimization goal is throughput / cost rather than TTFT.

Components

```
Prefill instances: 4 nodes, multiple TP=4 ranks
Decode instances: 8 nodes, EP=48 class + DP Attention
KV Transfer: Mooncake Transfer Engine (RDMA/NVLink)
Load Balancing: EPLB for expert imbalance correction
Quantization: FP8 Attention + NVFP4/FP8 MoE
```

#### Expected Performance

In a 96 H100 configuration, 52.3k input / 22.3k output tokens per node, with a cost of $0.20 per million output tokens, achieving up to 5x output throughput compared to pure TP.

**Conclusion from a cost perspective:** If the official API costs $0.87 ~ $1.10 per million output tokens, then the self-hosting cost of $0.20 is about a 4-5x difference. However, this is when GPUs are always saturated; if traffic halves, the cost doubles.

> The real variable for self-hosting's break-even point is utilization: whether there's enough traffic to keep 96 cards busy 24/7. If not, the API is cheaper.

### Benchmarking Methodology - How to Generate Your Own Numbers

Let's point out five common mistakes.

1.  No warm-up. The first request is abnormally slow due to CUDA graphs, JIT compilation, and kernel auto-tuning, so it's recommended to discard at least dozens of initial requests.
2.  Not turning cache off or on. Repeatedly sending the same prompt results in a 100% RadixAttention hit rate, yielding fantastic numbers. **To measure cold start, use `--disable-radix-cache`**. To simulate real operations, **you must reproduce the actual traffic distribution.**
3.  Only looking at averages. A system with an average TTFT of 400ms but a p99 of 8 seconds is a failed system. Always look at **p50 / p95 / p99** together.
4.  **Applying load only with fixed concurrency.** Real traffic arrives in bursts, closer to a Poisson distribution. Fixed concurrency tests cannot reproduce queue overflow situations.
5.  **Optimizing only total throughput. Remember goodput from Section 1.4.** Requests that violate SLO should not be counted.

For metrics to measure, SGLang is compatible with Prometheus and provides a `/metrics` endpoint.

-   `sglang_num_queue_reqes` queue depth
-   `sglang_token_usage` KV cache utilization
-   `sglang_cache_hit_rate` (Radix Attention effect)
-   `sglang_time_to_first_token_seconds` TTFT
-   In conjunction with this, use `nvidia-smi dmon -s pum -d 10` to monitor GPU utilization and memory pressure.

#### How to Read Dashboards: A Few Diagnostic Examples

-   In symptoms of low GPU utilization and long queues, possible causes are scheduling/memory issues, not GPU scarcity. Check if `token_usage` is low.
-   If `token_usage` is 100% and queues are long, it indicates insufficient KV cache. Quantization, increasing `mem-fraction`, and TP expansion are recommended.
-   Low `cache_hit_rate`: This is a prompt design issue. Check if variable values are at the beginning.
-   Spikes in TTFT p99: Check if long prefill is blocking, if chunked prefill is enabled, and the chunk size.
-   Irregular ITL, batch configuration changes: Adjust `max-num-seqs`.

Remember one diagnostic principle:

"If GPU utilization is low but the request queue is long, the cause is not automatically a weaker model or smaller GPU, but rather scheduling, cache allocation, and workload patterns should be suspected first."

### GPU Capacity Planning - How Many Cards to Buy

We'll do a hypothetical calculation. Knowing how to do these calculations is quite important.

If the goal is 20 requests per second at peak, with an average input of 3,000 / output of 500 tokens for a 70B-class model, targeting TTFT p95 of 1 second:

```
[1] Required Throughput
    Input: 20 req/s × 3,000 = 60,000 tok/s
    Output: 20 req/s × 500   = 10,000 tok/s

[2] Number of Concurrent Live Requests (Little's Law)
    Average request duration ≈ TTFT + (output 500 × ITL 40ms) = 0.5 + 20 = 20.5 seconds
    Concurrent requests = 20 req/s × 20.5s ≈ 410 requests

[3] KV Cache Requirement (Section 1.2 formula, average context 3,500 tokens, FP8)
    Per request ≈ 2 × 80 × 8 × 128 × 3,500 × 1 byte ≈ 0.57 GB
    410 requests × 0.57 GB ≈ 234 GB

[4] Model Weights (FP8 70B) ≈ 70 GB

[5] Total Memory Required ≈ 234 + 70 + 20% buffer ≈ 365 GB
    → 3 H200 (141GB) cards would be sufficient for memory

[6] Throughput Verification ← This is where most setups fall short
    10,000 tok/s output with 3 cards? → 3,333 tok/s needed per GPU
    This number is tight for 70B FP8 decode → Quantization recommended
```

Memory and throughput are ultimately separate constraints, and throughput is usually the first bottleneck.

Sometimes, one might look only at memory and conclude it's a KV cache issue, only to find out that quantization is actually needed.

Also, Little's Law states `Number of concurrent requests = Arrival rate x Average residence time`. The longer the output, the longer the residence time, and the longer the residence time, the more KV cache is needed. Output length is a hidden variable determining KV memory.

| Term | Definition and Mechanism |
|------|--------------------------|
| goodput | Throughput counting only requests that satisfy SLO. If only total throughput is considered, batches might be excessively enlarged, sacrificing individual users. Thus, goodput is the metric corresponding to actual business value. |
| CUDA Graph | Records hundreds to thousands of kernel execution sequences once and replays them with a single command. Eliminates kernel launch overhead. Requires fixed shapes, so multiple graphs are pre-captured and padded per batch size. Consumes startup time and GPU memory. |
| CacheBlend | A technique for reusing KV blocks at non-prefix positions. Selectively recomputes only a few tokens to restore quality. If it's true RAG where search chunk order changes every time, prefix caching becomes ineffective. Non-prefix reuse is needed. |
| DP Attention | Processes attention layers with data parallelism instead of TP. Each GPU handles different requests and holds its own KV, so MLA's compression benefits are not offset by replication. Exclusive to large batches, unsuitable for small batches. |
| DeepEP / DeepGEMM / EPLB | Respectively, an all-to-all communication library for MoE, a low-precision GEMM kernel, and an expert load balancer. A trio that makes large-scale EP practical. |
| EAGLE-3 / 3.1 | A lightweight drafter that takes the target model's hidden states as input. High acceptance rate due to good alignment with the target. 3.1 corrects attention drift with FC normalization, increasing long-context acceptance length by up to 2x. |
| EP (Expert Parallelism) | Assigns MoE experts entirely to GPUs and routes tokens all-to-all. Reduces memory burden per GPU, creating more KV cache headroom. Bottlenecks are all-to-all communication and expert load imbalance. |
| FlashAttention | Performs attention within SRAM without creating an [N,N] intermediate matrix, using tiling + online softmax. Reduces HBM access from quadratic to linear. Requires rewriting for each hardware generation (FA2/3/4). |
| FlashInfer | A serving-specific kernel library. Provides block-sparse KV attention, MLA, cascaded attention, plan/run separated load balancing, etc. Not a superior version of FlashAttention, but a different category. |
| ITL / TPOT | Inter-token latency. A decode performance metric. Below 50ms, it surpasses human reading speed. |
| KV Cache | K·V values stored for reuse during generation. Size = 2 × layers × KV heads × head dimension × length × batch × bytes. Directly determines the number of concurrent users and service cost. |
| LMCache | Tiered KV offloading layer (HBM→DRAM→SSD→Remote). An independent daemon, so the cache survives even if the engine dies, and multiple instances share the cache pool. |
| Little's Law | Number of concurrent requests = Arrival rate × Average residence time. A fundamental tool for capacity planning. Longer outputs lead to longer residence times and require more KV memory. |
| Mooncake | A KV Cache-centric distributed architecture. Composed of a Transfer Engine (inter-node KV transfer) and Store (distributed storage). One of the transport layer standards for PD separation. |
| PagedAttention | Applies OS virtual memory to KV cache. Allocates fixed blocks of 16 tokens only when needed and maps them with a logical block table. Completely prevents external fragmentation, 2-4x concurrent requests. |
| PD Separation | Places prefill and decode in separate GPU pools. Each has its optimal parallelization and batch settings. KV transfer becomes a new bottleneck. Beneficial only with tens of GPUs + high-speed interconnect conditions. |
| RadixAttention | Stores KV cache in a radix tree to automatically reuse prefixes between requests. Evicts from leaves using LRU, protects in-use nodes with `lock_ref`, and enhances hit rate with cache-aware scheduling. A net loss if there's no sharing. |
| SGL Router | Cache-aware load balancer. Sends requests with the same prefix to the same worker, maintaining hit rate even across multiple servers. Round-robin makes caching meaningless. |
| TP / PP | Tensor Parallelism (splits matrices, high communication, node-internal only) / Pipeline Parallelism (splits layers, low communication, bubbles occur). |
| TTFT | Time to first token. Determined by queue wait + prefill. Most of the user-perceived latency. |
| XGrammar | A CFG-based constrained decoding engine. Divides tokens into context-independent/dependent. The former is precomputed, the latter is processed overlapping with GPU execution, achieving less than 40µs per token. As of March 2026, it's the default backend for major engines. |
| Rejection Sampling | In speculative decoding, a procedure to accept/reject draft tokens to match the target distribution. Thanks to this, the final output is exactly lossless. |
| Tiered Cache | Places KV across 3 tiers: HBM/DRAM/NVMe. Bandwidth drops sharply to 3,350 / 63 / 7 GB/s, so it's crucial to calculate if it's cheaper than recomputation. |
| Memory-bound | A state where memory bandwidth, not computation, is the bottleneck. LLM decoding falls into this category. The fundamental reason why quantization, batching, and speculation work. |
| Arithmetic Intensity | Number of operations performed per byte of memory. Decode is near 1 FLOP/Byte, meaning most of the GPU's performance is idle. |
| Preemption | When memory is insufficient, moving an executing sequence out. There are recomputation and CPU swap methods, with the advantageous one depending on context length. |
| Continuous Batching | Iteration-unit scheduling. New requests are immediately added to the batch as soon as space becomes available. GPU utilization 30-40% → 75-90%. |
| Jump-forward Decoding | If the next token is uniquely determined by grammar, it's emitted directly without a model call. Outputs with rich structure can be completed with fewer model calls than token count. |
| Prefix Caching | Reuses KV for identical prefixes. Valid due to KV's causality (dependence only on preceding tokens). Requires exact matches, so prompt design is crucial. |
| Zero-overhead Scheduler | Overlaps CPU scheduling and GPU execution to eliminate GPU idle time. More effective for smaller models and smaller batches. |
| Chunked Prefill | Breaks long prefill into chunks and intersperses them into decode batches. Eliminates head-of-line blocking + utilizes leftover compute resources from memory-bound decode. |
| Cascaded Attention | Computes attention for a shared prefix only once, applies it to multiple queries, and merges with unique parts. Accurate due to the associativity of online softmax. |
| Speculative Decoding | Draft proposes multiple tokens → Target validates them at once. Lossless. Benefits disappear with larger batches (as it becomes compute-bound). |
| Prefill / Decode | Input batch processing (compute-bound) / Output sequential generation (memory-bound). Optimal settings are diametrically opposed, creating an incentive for separation. |

<br>

## Misconceptions and Pitfalls

### Buying more GPUs will solve it.

While not entirely wrong, if the GPU dashboard shows 60% utilization but the request queue is long, the bottleneck isn't the model or the GPU, but the inference engine.

In an area where a single configuration change can yield a 2-4x improvement, doubling the hardware is the most expensive solution.

### High throughput means user satisfaction.

Increasing batch size continuously boosts throughput.

However, the ITL (Interaction-to-Latency) for each user continuously deteriorates.

Optimizing for throughput and user satisfaction are opposing goals. Measure with goodput.

### Is SGLang faster than vLLM?

Only conditionally true. RadixAttention has overhead when there's no sharing, and for simple generation workloads without repeated prefixes, vLLM is better.

"Which engine is faster?" is the wrong question; "What is the prefix sharing rate of my workload?" is the right one.

### Since 1M context is supported, I can just input 1M.

This was a warning from previous notes, but to be more specific from a serving perspective:

- **KV cache explodes.** If you plug S = 1,000,000 into the formula, a single user will monopolize multiple GPUs.
- **Prefill takes seconds to tens of seconds.** TTFT SLO will be missed.
- Latency for all other users spikes. Chunked prefill cannot completely prevent this.

In most cases, reducing it with RAG is cheaper, faster, and more accurate.

### Is speculative decoding always better to keep on?

The acceleration, which was 1.96x at batch 1, drops to 1.21x at batch 128.

In large batches, draft computations take the place of honest computations.

Consider turning it off if the batch size exceeds 32.

### Quantization slightly degrades quality.

"Slightly" varies by workload; degradation unnoticeable in general conversation

becomes very apparent in math, coding, and long inference chains.

Also, serving providers offering APIs sometimes quantize silently, which is why public weights and outputs often yield different results.

### PD separation is a cutting-edge technology, so it should be adopted.

The impressive numbers for PD separation come from an H100 scale of 96 cards; doing this with 8 GPUs makes both pools inefficient, actually slowing things down.

Scale is the condition.

### Five reasons why benchmarks are not reproducible

1. Insufficient warm-up - CUDA graph capture, JIT compilation
2. Cache state differences - second run is always faster
3. Quantization differences - silent quantization by serving providers
4. Kernel backend differences - numbers vary depending on FlashInfer or FlashAttention
5. Input distribution differences - 2,000-token fixed input benchmarks may differ from reality

### The true meaning of day-0 support

An announcement of support on the day a new model is released means it *runs*, not that it's optimized.

New architectures require new kernels, and kernel optimization takes weeks to months. Don't mistake initial performance figures for final performance.

### Judging the whole by changing just one setting

Performance tuning involves interactions.

If you change the chunk prefill size, you need to adjust the optimal batch size; if you enable quantization, KV cache capacity increases, allowing for larger batches.

This is why the numbers in the tuning ladder are presented cumulatively. **Change one thing at a time, but readjust other parameters each time.**
