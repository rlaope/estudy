# QWEN Model Analysis

> As of August 10, 2026

When you decide to start using Qwen, you'll soon run into the following issues:

- Qwen3, Qwen3.5, Qwen3.6, Qwen3.8 came out within half a year, so which one is the standard?
- 27B dense and 35B-A3B are from the same generation, but what's the difference, and when should I use which one?
- There are terms like Gated DeltaNet, early fusion, and MTP, but what are they?
- They say it beat Claude in official benchmarks, but how much can I trust those numbers?
- Which coding harness should I use to get the best performance from the model?

Let's try to understand these points with the goal of being able to answer them.

<br>

## Qwen Family

### Identity - A Family That Wins with Breadth

Qwen is a family of models created by Alibaba Cloud, known in Chinese as Tongyi Qianwen and in English as Qwen.

What distinguishes it from other families is the **breadth of its lineup**. DeepSeek focuses on a single flagship model, and Moonshot concentrates on pushing the upper limits of scale. Qwen, in contrast, covers the entire spectrum, from 0.8B to 2.4T, within the same generation.

Qwen3.5 Generation Size Spectrum (Released Feb-Mar 2026)
```
0.8B  2B  4B  9B  27B    35B-A3B   122B-A10B      397B-A17B
 │    │   │   │    │         │          │              │
휴대폰 ────────── 소비자 GPU ──── 워크스테이션 ──── 데이터센터
```

This breadth is a strategic choice, enabling organizations with any hardware to find a solution within Qwen.

As a result, fine-tuning, derivative models, and community tools have heavily gravitated towards Qwen, making it a leader in this space.

### Open-Closed

```
오픈 트랙 (Apache 2.0, 가중치 공개)
  Qwen3 → Qwen3-Next → Qwen3.5 → Qwen3.6 → (Qwen3.8-27B 예정)

클로즈드 트랙 (API 전용)
  Qwen3-Max → Qwen3.5-Plus/Flash → Qwen3.7-Max → Qwen3.8-Max
```

The top-tier performance Max series was API-only, while the open-source line operated separately, one generation behind.

However, they promised to open the weights of Max-grade models for the first time with the release of Qwen3.5-Max in August 2026, and also announced that Qwen3.8-27B would be open-sourced. While repositories would be uploaded, it wasn't usable locally until the license and model card were released.

| Date | Model | Introduced Features |
|---|---|---|
| 2025.5 | **Qwen3** | dense 0.6B~32B + MoE 30B-A3B, 235B-A22B. Integrated thinking/non-thinking and thought budget. 36T tokens, 119 languages |
| 2025.7 | Qwen3-Coder-480B-A35B | Agentic coding specialization. Qwen Code CLI released concurrently |
| 2025.9 | **Qwen3-Next-80B-A3B** | First introduction of Gated DeltaNet 3:1 hybrid. 9.3% of Qwen3-32B training cost |
| 2026.2.3 | Qwen3-Coder-Next | 256K context, compatible with various harnesses |
| 2026.2.16 | **Qwen3.5-397B-A17B** | Hybrid across the entire generation. Early-fusion multimodal, 512 experts, 201 languages |
| 2026.2~3 | Qwen3.5 small wave | 122B-A10B, 35B-A3B, 27B, 9B, 4B, 2B, 0.8B |
| 2026.4.16 | **Qwen3.6-35B-A3B** | Achieved SWE-bench 73.4 with 3B active parameters |
| 2026.4.22 | **Qwen3.6-27B** | Dense 27B surpassed the previous generation's 397B flagship in coding |
| 2026.5~6 | Qwen3.7-Max / Plus | Closed |
| 2026.8.3 | **Qwen3.8-Max** | 2.4T / 95B active, 1M context. Open weights announced |

Generations changed three times in half a year.

This pace is the most annoying aspect when working with Qwen, and Chapter 10 discusses how to address it.

### License

The open line is Apache 2.0. DeepSeek GLM is MIT, which has virtually no restrictions, while Kimi is Modified MIT, requiring attribution if MAU exceeds 100 million or monthly revenue exceeds 20 million USD.

Apache 2.0 explicitly includes patent provisions, making it one of the easiest licenses to pass corporate legal review. When creating and redistributing derivative models, only the original license needs to be cited.

<br>

## Lineup

| Generation | Representative Model | Context | Status |
|---|---|---|---|
| Qwen3 | 0.6B~32B dense, 30B-A3B, 235B-A22B | 32K~128K | Maintenance. 14B/32B dense is still used as it's not in subsequent generations |
| Qwen3-Next | 80B-A3B | 256K | Testbed for hybrid architecture |
| Qwen3.5 | 0.8B~397B-A17B | 262K native, 1M with YaRN | Stable. vLLM·SGLang support mature |
| **Qwen3.6** | 27B dense, 35B-A3B | 1M native | **Current forefront of the open line** |
| Qwen3.7 / 3.8 | Max, Plus | 1M | Closed (3.8 open announced) |

> 80B-A3B refers to MoE (Mixture of Experts) where only 3 billion (3B, Active 3 Billion) out of a total of 80 billion parameters are activated during token processing.

The latest open-weight model available now is Qwen3.6. As of now.

### Purpose-specific Series

Beyond general-purpose models, Qwen also releases purpose-specific series.

Without understanding this structure, one might inefficiently extract embeddings using a general-purpose model.

| Series | Purpose | Representative Model | Notes |
|---|---|---|---|
| **General-purpose** | Conversation, Inference, Coding | Qwen3.6-27B, 35B-A3B | Multimodal built-in since 3.5 |
| **Coder** | Agentic Coding | Qwen3-Coder-Next, Qwen3-Coder-480B-A35B | 256K context, emphasis on harness compatibility |
| **Embedding** | Search Vector Generation | Qwen3-Embedding 0.6B / 4B / 8B | MRL support, instruction awareness |
| **Reranker** | Search Result Reranking | Qwen3-Reranker 0.6B / 4B / 8B | Cross-encoder |
| **VL-Embedding / Reranker** | Multimodal Search | Qwen3-VL-Embedding, Reranker | Document Image Search |
| **Image** | Image Generation·Editing | Qwen-Image series | Separate line |

If building RAG, three models are needed, not just one general-purpose model.

An embedding model selects candidates, a reranker sorts them, and a general-purpose model writes the answer.

The trick is to choose the appropriate size for each stage.

### Hardware Specifications by Size

Let's look at the approximate memory requirements based on Q4_K_M quantization.

Actual values vary depending on context length.

```
| Model | Q4_K_M Size | Minimum Hardware | Purpose |
|---|---|---|---|
| Qwen3.5-0.8B | Approx. 0.6 GB | Phone, Raspberry Pi | On-device classification, routing |
| Qwen3.5-2B | Approx. 1.5 GB | 8GB Laptop | Simple summarization, embedded |
| Qwen3.5-4B | Approx. 2.8 GB | 8GB GPU | Large batch, fine-tuning based |
| Qwen3.5-9B | Approx. 6 GB | 12GB GPU | General chat, tool calling |
| **Qwen3.6-27B** | **Approx. 16.8 GB** | **24GB GPU (RTX 3090/4090)** | **Local coding. Standard for this segment** |
| Qwen3.6-35B-A3B | Approx. 20 GB | 32GB Unified Memory | Multimodal + coding, speed priority |
| Qwen3.5-122B-A10B | Approx. 68 GB | 2×A100 80GB | Mid-size server |
| Qwen3.5-397B-A17B | Approx. 220 GB | 8×H100 class | Data center |
```

> Q4_K_M is a standard notation appended to filenames when quantizing LLM models into GGUF format, meaning it's compressed with 4-bit precision (Q4), uses the advanced K-quant algorithm (K), and balances for medium-quality (M).

A single 24GB GPU is a core segment of the Qwen ecosystem, and Qwen3.6-27B was designed to target this space. According to community reports, it achieves 25 tokens per second on a 32GB RAM MacBook.

### Two Options in the Same Generation: 27B dense and 35B-A3B

Qwen 3.6 released two models around the same time. They have different characteristics, and here's how to choose between them:

| Item | Qwen3.6-27B | Qwen3.6-35B-A3B |
|---|---|---|
| Structure | dense | MoE (Active 3B) |
| SWE-bench Verified | 77.2 | 73.4 |
| MMLU-Pro | 86.1 (based on 3.5-27B) | 85.2 |
| GPQA Diamond | — | 86.0 |
| AIME 2026 | — | 92.7 |
| VITA-Bench (General-purpose Agent) | 41.8 (based on 3.5-27B) | 35.6 ← Weakness |
| Memory | 16.8 GB | 20 GB |
| Batch 1 Speed | Normal | Fast |
| Deployment Complexity | Low | Additional considerations like MoE offloading |

**Summarizing selection criteria based on the above:**

- **Individual developer, single GPU, coding-focused ->** 27B dense. In batch 1, the advantage of MoE active parameters is small, leaving only the burden of loading all total parameters into memory. Deployment is also simplified.
- **Serving with many concurrent requests and multimodal needs ->** 35B-A3B. As batch size increases, the computational advantage of active 3B becomes more apparent.
- **General-purpose agent tasks ->** VITA-Bench 35.6 is lower than 27B (41.8) of the same generation or Gemma4-31B (43.0). If agent tasks other than coding are primary, other candidates should be considered.

There's an interesting backstory to this choice. When a Qwen executive surveyed which model should be open-sourced, 27B dense was overwhelmingly ranked first, but 35B-A3B was actually released first. 27B came out six days later. The community prefers dense models because they can run without additional optimizations like MoE offloading.

<br>

## Architecture

Let's explore Qwen's architecture, understanding the problems it solved, its underlying principles, and its implications for actual deployment.

### Qwen3 Generation - The Complete Transformer

Qwen3, released in May 2025, did not invent a new architecture.

Instead, it precisely combined verified techniques available at the time.

```
Qwen3 Block Configuration

Input → RMSNorm → GQA (+ QK-Norm) → Add Residual
     → RMSNorm → SwiGLU FFN or MoE → Add Residual → Output
       ↑ Pre-Norm Placement
```

- **GQA:** A technique where attention heads share KV in groups to reduce KV cache.
- **SwiGLU:** A gated activation function that offers good expressiveness with the same number of parameters.
- **RoPE:** Encodes position by rotating vectors, naturally expressing relative distances.
- **RMSNorm + Pre-Norm:** Normalizes value magnitudes and places them before the block to stabilize deep networks.

Two changes occurred when transitioning from **Qwen2** to 3.

**QKV bias was removed, and QK-Norm was added.** By applying normalization to Q and K, the inner product attention logits do not explode. If logits become too large during training, softmax concentrates on a single point, causing gradients to vanish, and potentially leading to training collapse. QK-Norm prevents this issue at its root.

**Shared experts were removed from MoE.** Qwen2.5-MoE always had shared experts that were always active, but Qwen3 removed them. Instead, it used **global batch load balancing loss** to push experts to specialize more distinctly.

> However, this decision was later reversed. Qwen3.5 reintroduced shared experts with a configuration of 10 routing + 1 shared expert out of 512 experts. This seems to be because as the number of experts grew to 512, the redundant learning waste of putting common knowledge needed by all tokens into each expert became severe. This is an example of an architectural decision being reversed based on scale.

### Integrated thinking / non-thinking and Thinking Budget

The direction Qwen3 actually proposed anew is this.

#### Existing Problem

Operating reasoning models and general models separately incurs double the deployment cost, and users have to choose which one to call each time. However, it's wasteful to run simple calculations like 1+1 through a reasoning model that processes thousands of tokens, just as it's wasteful for a general model to give an immediate answer to a complex proof.

#### Solution

Integrate both modes into a single model and allow switching between them per request. A **thinking budget** can be attached, allowing a cap to be specified, such as "for this problem, think for up to 2000 tokens."

```py
# vLLM / SGLang OpenAI 호환 API에서
{
  "model": "Qwen/Qwen3.6-27B",
  "messages": [...],
  "chat_template_kwargs": {"enable_thinking": True},
  "max_tokens": 4096
}
```

In Ollama, it can be fixed in the Modelfile.

```
FROM qwen3:8b
PARAMETER think false      # 항상 즉답 모드
# PARAMETER think true     # 항상 사고 모드
```

Thinking tokens are billed as output tokens and increase latency.

Turning off thinking for tasks that don't require reasoning, such as classification or extraction, immediately reduces cost and latency.

Conversely, for math or code debugging, it must be enabled to improve accuracy.

It is standard to configure it differently for each task.

Following this design, it has become an industry standard, with GLM's high/max effort and Qwen3.80Max's low/high/xhigh belonging to the same family.

### Gated DeltaNet Hybrid

This is the core area where Qwen's architecture placed its bets, introduced in Qwen3-Next (September 2025) and applied across the entire Qwen3.5 and 3.6 generations.

#### Problem to Solve

When the context becomes long, attention becomes unmanageable.

For N tokens, Q-K pairs increase quadratically (N²), and the KV cache grows proportionally to the length, limiting the number of concurrent users.

DeepSeek opted for MLA by compressing KV or DSA by selecting tokens to attend to.

**Qwen took a different path.**

#### How it Works

Instead of re-examining the entire past, it carries a summary of the past in a fixed-size state. This is a **similar idea to RNNs, but uses modern update rules.**

> Analogy - If traditional attention is like keeping and reviewing all meeting minutes, linear attention is like carrying a one-page summary of the current situation. Even if the meeting lasts ten hours, the summary remains one page, so the desk doesn't get cluttered.

**Gated DeltaNet - GDN** refines the way this summary is updated.

- **Delta Rule:** Updates only with the difference between existing memory and new information, acting as an error-correcting memory.
- **Exponential Gating:** Controls what and how much to forget, preventing state saturation.
- **Causal Conv1D:** Captures local context from the immediately preceding few tokens.
- **Q/K L2 Normalization:** Used instead of softmax to prevent value explosion.

The delta rule is key; naive linear attention simply adds new information, causing the state to quickly become muddled.

The delta rule calculates **how different new information is from what is already remembered** and reflects only that difference, thus correcting memory without overwriting it.

#### Limitations of Pure Linear Attention and Hybrid Approach

If only a summary is carried, precise requests like "the phrase on page 300, line 42" cannot be retrieved.

In other words, its precise retrieval capability is limited, so it was addressed by mixing layers.

```
Qwen3.5-397B-A17B's Actual Layer Placement (Total 60 Layers)

15 × [ GDN→MoE, GDN→MoE, GDN→MoE, Gated Attention→MoE ]
       └──────── 3 Cheap Summary Layers ────────┘ └ 1 Precise Retrieval Layer ┘
              75%                              25%
```

Three layers follow the flow with linear attention, and the fourth layer uses full attention for precise targeting. This **saves most of the cost while retaining retrieval capabilities.**

Here's the detailed configuration of each layer in Qwen3.5.

```
Gated DeltaNet Layer
  64 V heads, 16 QK heads, 128 head dimension

Gated Attention Layer
  32 Q heads, 2 KV heads, 256 head dimension, 64 RoPE dimension
  ↑ Only 2 KV heads = very aggressive GQA. KV cache in this layer is also minimized.
```

Even the Gated Attention layer is designed to minimize KV heads to just two, meaning that even the 25% of full attention layers are maximally reduced.

#### Actual Effects

According to Qwen's official announcement:

```
Decoding Throughput (vs. Qwen3-Max)
  32K context  : 8.6x
  256K context : up to 19x
Deployment VRAM      : 60% reduction
```

**The throughput multiplier increases as the context length grows.** This directly demonstrates the nature of linear attention. While attention scales quadratically with length, linear attention operates independently of length, thus the gap widens over time.

The training benefits are also significant. Qwen3-Next-80B-A3B achieved equivalent performance with **9.3% of the training compute of Qwen3-32B**. This means $100,000 becomes $9,300.

#### Trade-offs

**Theoretical weaknesses remain in precise retrieval.**

While 25% of full attention layers compensate, it's not perfect. For tasks requiring precise citation of specific details from very long documents, it's worth comparing against pure attention models.

**Serving stack support is slow.** New architectures require new kernels and new KV cache managers. For vLLM to support Qwen3-Next, it had to integrate Flash Linear Attention's Triton kernel and introduce a hybrid KV cache manager that manages both linear and full attention layers. There's a gap of weeks to months between a new generation being "functional" and "optimized."

### MoE Configuration

Let's look inside Qwen3.5-397B-A17B MoE.

```
512 experts
Active per token: 10 routing + 1 shared
Expert intermediate dimension: 1,024
-> Active parameters 17B / Total 397B = 4.3%
```

This approach involves splitting experts into smaller pieces and activating many of them.

Compared to early MoE, which activated 2 out of 8 large experts, this offers a much greater number of combinations, leading to better specialization.

> To use an analogy, it's the difference between sending 2 out of 8 generalist doctors to a hospital versus calling 10 out of 512 highly specialized doctors. The latter allows for precise combinations like cardiology + radiology + pediatrics.

The 35B-A3B pushes this idea to the extreme, activating only 3B (8.6%) out of 35B.

This ratio allows a coding agent to run even on an RTX 3060-class card.

### MTP

Qwen3.5 and later are trained with MTP.

MTP, or Multi-Token Prediction, trains the model to predict not just the next token, but several tokens simultaneously.

By forcing it to look ahead two or three words, it learns more far-sighted representations, which improves long-range coherence.

During inference, these MTP layers are directly used as the **draft model for speculative decoding**. No separate model needs to be created.

In speculative decoding, the draft model proposes multiple tokens, and the main model validates them in a single forward pass.

The cost of reading weights once is almost the same whether generating one token or validating six, so multiple tokens can be confirmed with the remaining computational resources.

Let's look at the measured effects in local execution. Using MTP-enabled GGUF in llama.cpp, Qwen3.6-35B-A3B shows a **27-29%** improvement over the standard **146 tok/s**.

However, the VRAM trade-off varies quite a bit depending on the settings. On a 16GB card, increasing the draft depth reduces the available context.

```
Measured Recommended Values for 16GB VRAM

Qwen3.6-27B dense
  --spec-draft-n-max 2 + q8 KV  → Fastest
  --spec-draft-n-max 1 + q5 KV  → Most context headroom

Qwen3.6-35B-A3B (MoE)
  --spec-draft-n-max 1 is only practical
  With q8 KV, average context is only 15K
  Increasing to 3-4 only consumes VRAM, speed does not scale proportionally
```

> GGUF (Georgi Gerganov Unified Format) is a **file format** created to efficiently store and execute LLM weights and metadata in a single file.
> It aims to reduce VRAM usage by supporting various quantizations with a single file structure, enabling operation on lower hardware specifications, and is primarily used in local AI execution programs like llama.cpp or Ollama.

**MTP benefits are greater in MoE** because the calculation for MTP heads is relatively cheaper compared to the full forward pass due to sparse routing.

### Early Fusion Multimodal

Starting with Qwen3.5, text, images, and videos are trained together from the beginning.

**The traditional method, late fusion,** was easier to build by creating a text model and then attaching an adapter with a vision encoder. However, this often resulted in awkward inference because the two modalities existed in separate representation spaces.

Early fusion mixes image tokens and text tokens into the same sequence from the pre-training stage. The model handles both within a single representation space from the outset.

> If late fusion is like learning Korean completely and then adding an interpreter, early fusion is like learning two languages together from childhood. The latter naturally allows for thought that blends both languages.

Qwen's official announcement states that early fusion training **maintains performance equivalent to previous generations in text tasks while outperforming Qwen3-VL, a dedicated vision model, in visual understanding.**

They claim to have avoided the common trade-off where adding multimodal capabilities degrades text performance.

**Deployment Note:** If you only intend to use text, not loading the vision encoder is advantageous for memory initialization time.

```bash
vllm serve Qwen/Qwen3.5-397B-A17B-FP8 \
  --language-model-only \        # 비전 인코더 미로드
  --reasoning-parser qwen3 \
  --enable-prefix-caching
```

### Reinforcement Learning at the Scale of a Million Agent Environments

This is one of the three pillars highlighted in Qwen3.5's official introduction.

They state that they **expanded reinforcement learning to a million-scale agent environment and applied a curriculum that gradually increases task difficulty.**

**This is necessary because to learn how to use tools effectively,** example data on tool usage is needed. Such data is scarce on the internet, so the only scalable method is to create a large number of simulation environments and let the model learn through trial and error.

The difficulty curriculum is based on the human learning approach: if only difficult tasks are given from the start, there are no successful examples, reward signals are sparse, and learning does not progress. Starting with easier tasks ensures meaningful signals at each stage.

### Strong to Weak Distillation

When creating smaller models, instead of training them separately from scratch, they are taught to mimic the output of a larger model.

This provides much more information than simply giving a single correct token, by providing the **entire probability distribution** that the larger model assigned to each token.

"The answer is A" teaches less than "A is 70%, B is 20%, C is 8%...".

This is one reason why Qwen's smaller models are stronger than competing models of the same size. A 0.8B model partially inherits the knowledge of a 397B model.

### Context Extension - YaRN

Qwen3.5 open models have a native context of 262,144 tokens, extended to 1,010,000 tokens with YaRN.

Qwen 3.6 has a native 1M context.

**YaRN:** RoPE represents positions as rotation angles. For long positions not seen during training, rotations can become excessive, confusing the model. YaRN increases the rotation period, mapping long positions to angles within the training range, contributing to token extension.

```bash
# vLLM에서 YaRN 적용
vllm serve Qwen/Qwen3.5-397B-A17B-FP8 \
  --hf-overrides '{"rope_scaling":{"rope_type":"yarn","factor":4.0,
                   "original_max_position_embeddings":262144}}' \
  --max-model-len 1010000
```

There's a principle for choosing the factor: factor 2.0 covers approximately 524K, and 4.0 covers approximately 1M.

**Increasing the factor degrades quality in short contexts.**

It's important to set the factor to the minimum required by the actual workload. If you unconditionally set it to 4.0 just because 1M is supported, most short requests will suffer.

> The factor (scaling factor, s) is a multiplier that determines how many times the original trained maximum context length will be extended. The default formula is (maximum context length to extend / model's original pre-trained context length). If a model that originally supported 4K (4,096) tokens wants to extend to 128K (131,072) tokens, the factor value would be 128K/4K, which is 32.

<br>

## Causality of Strengths, Known Weaknesses

### Local Execution - This Category Virtually Dominates

Qwen3.6-27B, at approximately 16.8GB using Q4_K_M, fits on a single RTX 3090 or 4090 while achieving SWE-bench Verified 77.2. It even surpasses the previous generation's open flagship, Qwen3.5-397B-A17B, in coding benchmarks.

> The RTX 4080 is equipped with 16GB~ VRAM, so to load the Qwen3.6 model along with its KV cache, high-efficiency quantization like 4-5 bit IQ4_XS or Q4_K_M must be used. In this case, the token generation speed (TPS) records approximately 15-35 TPS. Quantization below 4 bits is essential, and if the context length is set long, the KV cache size can swell to GBs, potentially causing a sharp drop in TPS due to offloading if VRAM is exceeded. Using the `-ctk q8_0 -ctv q8_0` options to compress the KV cache to 8 bits can save VRAM and increase throughput.

If we look at why these results appear, step by step:

```
1. Later generation, so training data and post-training techniques are improved
        ↓
2. Inherits capabilities of superior models through strong-to-weak distillation
        ↓
3. Hybrid attention allows even small models to handle long contexts
        ↓
4. Dense structure is advantageous over MoE in batch 1 (personal use) environments
   → MoE requires loading all total parameters into memory
     If the batch is small, the computational advantage of active parameters is not realized
        ↓
5. Apache 2.0 + the most robust GGUF/quantization ecosystem
   → Runs on Ollama, LM Studio, llama.cpp immediately upon release
```

Number 5 is underestimated; often, "does it run on my hardware right now?" is more important than a 2-point benchmark score,

It's also a significant asset to have people who have already encountered issues when problems arise. The metrics of high download counts and numerous derivative models overcome this tedious advantage.

### Multilingual - 201 Languages

Qwen3 expanded to 119 languages, and Qwen 3.5 to 201 languages and dialects. As of writing this note, it has the broadest language coverage among open models.

For Korean users, there are two practical implications.

1.  **CJK Processing Quality**: Superiority in Chinese, Japanese, and Korean compared to Western models has long been maintained, due to the high proportion of these languages in the training data.
2.  **Tokenizer Efficiency**: Qwen3.5's vocabulary has been expanded to accommodate 248,320 multilingual entries. If the vocabulary covers multiple languages well, the same Korean sentence can be expressed with fewer tokens. Fewer tokens mean reduced cost and latency, and more content fits into the same context window.

> Qwen tends to define language support broadly, so quality is not guaranteed for all 201 languages, and the variance is greater for low-resource languages. It should not be viewed with the same standard for languages with sufficient data, like Korean, and those without.

Korean performance should be evaluated by separating comprehension and generation. While most models perform well in comprehension, there's significant variation in generating natural Korean prose. It is highly recommended to directly compare writing styles for your specific use case.

### Training and Inference Efficiency

Qwen3-Next-80B-A3B achieved equivalent or better performance with 9.3% of Qwen3.32B's compute, and Qwen 3.5 delivered 8.6x decoding throughput at 32K and up to 19x at 256K compared to Qwen3-Max.

This efficiency stems **solely from hybrid attention.** Since 75% of the layers are linear attention, those layers have no KV cache and are linear in computational length. The longer the context, the greater the benefit.

From a deployment perspective, this means serving more concurrent users with the same GPU, which directly translates to cost per token.

### Weaknesses and Controversies

#### Test Question Bias

One researcher analyzed Qwen-based model embeddings and reported that they are **located in a highly test-question-centric branch, clearly distinguishing them from other foundational models like LLaMA and Gemma.** This suggests the possibility that the training data contained a significant amount of text in the format of test evaluations (cheating).

This provides a clue to explain the gap between benchmark scores and real-world usage experience. Since most benchmarks are in test format, models familiar with test formats have an advantage. Real-world tasks are not in test format.

Do not take benchmark scores at face value; you should evaluate them directly with 20-50 cases in your own work format. While this principle applies to all models, it is particularly relevant for Qwen.

#### Relative Weakness in General-Purpose Agent Capabilities

Qwen3.6-35B-A3B scored 35.6 on VITA-Bench, lower than its contemporary 27B (41.8) or Gemma4-31B (43.0). Strong performance in coding benchmarks does not directly translate to general-purpose agent capabilities.

**Coding agents and general-purpose agents are different capabilities.** Coding is easier to apply reinforcement learning with verifiable rewards, leading to significant returns from focused investment. General-purpose agent tasks often have ambiguous correct answers, so the same approach doesn't work.

#### Thought Loops

A repeatedly mentioned issue in community reports is the phenomenon of the model circling the same reasoning in thought mode, consuming tokens.

**Response**: It is practical to explicitly set a thought budget limit and route the thought mode to be toggled on or off depending on the task difficulty.

#### Frequent Generation Turnover Rate

Within 1 year and 3 months, Qwen3 → Next → 3.5 → 3.6 → 3.7 → 3.8 were released.

It's difficult to decide which generation to adopt as an in-house standard, and fine-tuned models quickly become outdated.

**Response:** Instead of chasing generations, manage evaluation sets as assets. When a new model is released, run the same evaluation set and only migrate if there's a significant improvement. Without an evaluation set, you can't know if switching to a new model each time actually leads to improvement.

#### Top-Tier Accessibility

Qwen3.7-Max and Qwen3.8-Max are API-only, meaning the best performance obtainable with open weights is always one generation behind.

The opening of Max-tier for Qwen3.8 was announced, but it's not yet available as of now..

<br>

## Selection Guide by Task

### Local Coding - Single GPU

**Qwen3.6-27B (Q4_K_M, approx. 16.8GB)**

Achieves SWE-bench Verified 77.2 on a single 24GB GPU.

It's dense, so it runs without MoE offloading.

```bash
# Ollama
ollama run qwen3.6:27b

# llama.cpp (MTP 포함 GGUF 권장)
llama-server -m Qwen3.6-27B-Q4_K_M-MTP.gguf \
  --spec-draft-n-max 2 \
  --cache-type-k q8_0 --cache-type-v q8_0 \
  -c 32768 -ngl 99
```

If you have 32GB or more memory, 35B-A3B is also an option. This is better if multimodal capabilities are needed or if generation speed is more critical.

> `--spec-draft-n-max 2` When using speculative decoding or MTP models, this specifies the maximum number of words a light and fast model can predict and generate at once. Setting it too high increases the probability of the smaller model making incorrect predictions, so 2 or 3 has been set as the ideal value for speed improvement.
>
> `--cache-type-k q8_0 and --cache-type-v q8_0` (KV cache quantization) compresses (quantizes) the KV cache data capacity, where AI remembers context, to an 8-bit (q8_0) size. This nearly eliminates memory loss while halving the VRAM occupied by the cache.
>
> `-ngl`, or Number of GPU Layers, is a value that makes the AI model's operations processed by the faster GPU instead of the CPU. Entering a number sufficiently larger than the total number of layers, such as 99 or -1, is commonly used as a maximum acceleration setting to load all possible layers onto the GPU.
>
> `-c` refers to the context size, which is the maximum number of tokens an AI can remember and understand at once in a conversation or document. It is set to values like 4096, 8192, 32768 because, in computer science, using powers of 2 for memory allocation and processing maximizes efficiency. Think of the binary system.

### Server Coding Agent

**Qwen3-Coder-Next or Qwen3.6-35B-A3B**

The Coder series was developed with harness compatibility as an explicit goal.

Trained to adapt to various scaffold templates with a 256K context, it integrates with claude code, qwen code, qoder, kilo, trae, cline, and others.

```bash
vllm serve Qwen/Qwen3-Coder-Next \
  --tensor-parallel-size 2 \
  --enable-auto-tool-choice \
  --tool-call-parser qwen3_coder \
  --enable-prefix-caching \
  --port 8000
```

`--tool-call-parser qwen3_coder` is important; it's a parser that converts the tool call format generated by the model into the OpenAI format. If not specified, the harness tool will not recognize the call.

### Large-Scale Batch Processing

**Qwen3.5-4B or 9B**

For low-difficulty, high-volume tasks like classification, extraction, and summarization, smaller models are often sufficient.

As discussed in previous case studies, model downsizing yields greater multiples than any configuration tuning.

Let's first go through the validation process.

```bash
# Compare candidates with a human-labeled validation set
python eval_batch.py \
  --models Qwen3.5-27B Qwen3.5-9B Qwen3.5-4B Qwen3.5-2B \
  --testset labeled_500.jsonl \
  --metrics accuracy,rouge-l
```

Run this to select the smallest model that meets the required quality.

### RAG

Three models can be used in combination.

```
User Query
    ↓
Qwen3-Embedding-0.6B ── Vectorization → Search top 100 results from vector DB
    ↓
Qwen3-Reranker-4B ──── Re-rank 100 results → Select top 5
    ↓
Qwen3.6-27B ────────── Generate answer with selected documents
```

**Reason for separating embedding and reranker:** Embeddings are dual encoders that can vectorize questions and documents separately for pre-indexing. They are fast but have lower precision. Rerankers are cross-encoders that evaluate questions and documents together. They are accurate but slow, so they cannot be applied to the entire document set.

> Think of it like the relationship between resume screening and interviews: resumes (embedding) reduce 10,000 people to 100, and interviews (reranker) reduce 100 people to 5. You can't interview all 10,000 people.

Qwen3-Embedding-8B **ranked first on the MTEB multilingual leaderboard with 70.58 points (as of June 5, 2025)**, while Qwen3-Reranker-4B scored 69.76 on MTEB-R, and the 8B version achieved 77.45 on Chinese CMTEB_R and 81.22 on code search MTEB-Code.

**Size selection:** Since embeddings must process the entire corpus at the indexing stage, it's practical to start with 0.6B and scale up as needed. Rerankers are only applied to the top 100 results, so using a 4B model is less burdensome.

Both models support **instruction awareness**.

You can tailor it to a domain by adding instructions like "Judge relevance from the perspective of legal contract analysis." In Qwen's official evaluations, using instructions was advantageous for most downstream tasks.

The embedding model supports **MRL (Matryoshka Representation Learning)**, allowing users to define the final vector dimension. This means you can adjust vector database storage costs and search quality.

### Multimodal

**Qwen3.5 or higher general-purpose models - Early fusion means no separate VL model is needed.**

Official figures for Qwen3.5-397B-A17B.

```
Document Processing   OmniDocBench 1.5  90.8   OCRBench 93.1   CC-OCR 82.0
Video Understanding   VideoMME(subtitles) 87.5   VideoMMMU 84.7   MLVU 86.7
Math Vision   MathVision 88.6   MathVista(mini) 90.3   We-Math 87.9
UI Manipulation     ScreenSpot Pro 65.6   OSWorld-Verified 62.2   AndroidWorld 66.8
```

**Document OCR and UI manipulation are strong suits.** The ability to find clickable coordinates from a screenshot (ScreenSpot, OSWorld) is a prerequisite for computer-using agents.

Settings for multimodal serving.

```bash
vllm serve Qwen/Qwen3.5-397B-A17B-FP8 \
  -dp 8 --enable-expert-parallel \
  --mm-encoder-tp-mode data \        # Vision encoder in data parallel
  --mm-processor-cache-type shm \    # Shared memory caching for pre-processing results
  --reasoning-parser qwen3 \
  --enable-prefix-caching
```

`--mm--processor-cache-type shm` allows skipping preprocessing when the same image appears repeatedly.

This is highly effective in workloads like document QA, where multiple questions refer to the same image.

### Fine-tuning Based Models

**Entire Qwen lineup is good; Qwen's advantage is greatest for this purpose.**

The key to fine-tuning is choosing an appropriate size that matches your data volume.

If you have 1,000 data points and fine-tune a 70B model, it will overfit. If you have 1 million data points and use a 1B model, it will lack capacity.

Only Qwen offers models from 0.8B to 400B within the same generation, making this choice possible.

Another significant point is that being Apache 2.0 means there are no restrictions on commercial redistribution of derived models.

### Edge On-Device

**Qwen3.5-0.8B/2B**

Handles routing, intent classification, and simple summarization on phones or embedded devices. Creating a hierarchical structure where only complex requests are passed to server models can significantly reduce costs and latency.

### Selection Flowchart

```
Is multimodal (image/video) needed?
├─ Yes ─→ Qwen3.5 or higher general-purpose model (3.6-35B-A3B or 3.5-397B)
└─ No
    ↓
Is it part of a search pipeline?
├─ Vectorization ─→ Qwen3-Embedding (from 0.6B)
├─ Re-ranking ─→ Qwen3-Reranker (4B)
└─ Answer Generation ─→ Go below
    ↓
Where will it run?
├─ Single 24GB GPU ─→ Qwen3.6-27B ★
├─ 32GB+ Unified Memory ─→ Qwen3.6-35B-A3B
├─ 8GB or less / Edge ─→ Qwen3.5-2B / 4B
├─ Multi-GPU Server ─→ Qwen3.5-122B-A10B
└─ Data Center ─→ Qwen3.5-397B-A17B
    ↓
What is the task nature?
├─ Coding Agent ─→ Coder series or 3.6-27B
├─ Large-Scale Batch ─→ Smallest possible size after validation
├─ General-purpose Agent ─→ Check VITA-Bench weakness. Compare with other families.
└─ Highest performance needed ─→ Qwen3.8-Max (API only)
```

<br>

## Harnesses and Serving Stacks

A **harness** is a wrapper that encloses a model, provides it with tools, and executes commands to read files.

In coding agents, examples include claude code, qwen code, cline, and opnecode.

Even with the same model, performance varies depending on the harness. This is because system prompts, tool definition formats, and context management methods differ.

In benchmarks from NVIDIA's Polar: Agentic RL on Any Harness at Scale paper (May 2026), a Qwen3.5-4B based model showed the best coding performance with the Qwen-Code harness.

This implies that combinations where the model and harness come from the same team are advantageous.

The reason is that QwenCode officially states that it has adjusted its prompts and function call protocols to match the Qwen model. When the tool call format used in the model's post-training aligns with the format expected by the harness, friction is reduced.

> **Practical Principle:** When changing models, review the harness as well. Benchmark scores are measured with a specific harness, and rankings can change with different harnesses.

### Qwen code

A terminal agent released by the Qwen team alongside Qwen3-Coder, which forked Google Gemini CLI and adjusted prompts and function call protocols to fit the Qwen model.

```bash
npm i -g @qwen-code/qwen-code@latest

# Connect local vLLM/SGLang server to Qwen Code
export OPENAI_BASE_URL="http://127.0.0.1:8000/v1"
export OPENAI_API_KEY="dummy"
export OPENAI_MODEL="Qwen/Qwen3.6-27B"

qwen
```

### Recommended Configurations by Serving Stack

**vLLM - Qwen3.5-397B-A17B**

Text-only, 8 GPU data parallel

```bash
vllm serve Qwen/Qwen3.5-397B-A17B-FP8 \
  -dp 8 \
  --enable-expert-parallel \
  --language-model-only \
  --reasoning-parser qwen3 \
  --enable-prefix-caching
```

- `-dp 8 --enable-expert-parallel`: For MoE, assigning experts entirely is advantageous for communication.
- `--language-model-only`: Saves memory by not loading the vision encoder when only using text.
- `--reasoning-parser qwen3`: Separates and returns reasoning tokens in the response.
- `--enbale-prefix-caching`: Reuses system prompts repeatedly.

**If in a Blackwell (GB200) environment, NVFP4 checkpoints are recommended.**

```
vllm serve nvidia/Qwen3.5-397B-A17B-NVFP4 \
  -dp 4 --enable-expert-parallel \
  --language-model-only --reasoning-parser qwen3 \
  --enable-prefix-caching
```

NVFP4 is natively processed by Blackwell's 5th generation Tensor Cores.

Hopper does not support it, which would make it slower, so using FP8 is recommended.

For **low-latency workloads**, a combination of enabling MTP speculative decoding and disabling prefix caching is recommended.

> **NVFP4** checkpoints are compressed and optimized AI model weights and state files in a 4-bit floating-point format developed by NVIDIA. They minimize model accuracy loss while reducing capacity, maximizing speed and efficiency with the latest hardware.

```
MTP-1 Speculative Decoding
  → Reduces TPOT (Time Per Output Token) with high acceptance rate
  → However, throughput decreases under heavy load
```

This is only beneficial when concurrency is low, so understand your traffic characteristics and tune accordingly.

#### SGLang

```bash
python -m sglang.launch_server \
  --model-path Qwen/Qwen3.6-27B \
  --tp-size 2 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --tool-call-parser qwen3_coder \
  --reasoning-parser qwen3 \
  --chunked-prefill-size 4096 \
  --enable-metrics --enable-cache-report
```

- `--tool-call-parser qwen3_coder`: Without this, the harness cannot recognize tool calls.
- **Hybrid architecture models (Qwen3-Next, 3.5, 3.6)** require a KV cache manager that handles both linear and full attention layers, so you must first check if your engine version supports that generation.

#### llama.cpp / Ollama - local

```bash
# Use GGUF with MTP (check for MTP in filename)
llama-server \
  -m Qwen3.6-27B-Q4_K_M-MTP.gguf \
  --spec-draft-n-max 2 \
  --cache-type-k q8_0 --cache-type-v q8_0 \
  -c 32768 \
  -ngl 99 \
  --host 0.0.0.0 --port 8080
```

Based on 16GB VRAM, the empirically recommended values are as previously summarized: 27B dense is fastest with `--spec-draft-n-max 2` + q8 KV, and if more context headroom is needed, reduce to 1 + q5 KV.

### Combination Summary

```
| Scenario | Model | Serving | Harness |
|---|---|---|---|
| Personal Local Coding | Qwen3.6-27B Q4_K_M | Ollama / llama.cpp | Qwen Code, Cline, OpenCode |
| Team Server Coding | Qwen3-Coder-Next | vLLM / SGLang | Qwen Code, Cline |
| RAG Service | Embedding 0.6B + Reranker 4B + 3.6-27B | SGLang (HiCache) | Custom Pipeline |
| Multimodal Agent | Qwen3.5-397B-A17B | vLLM (mm option) | Custom or OpenCode |
| Large Batch | Qwen3.5-4B/9B | SGLang (batch maximization) | Not required |
| Best Performance | Qwen3.8-Max | API | Qwen Code, Claude Code etc. |
```

<br>

## Official Benchmark Data

### Qwen3.5-397B-A17B Official Figures

These are values released by the Qwen team and distribution partners.

### Language and Reasoning

| Benchmark | Score | What it measures |
|---|---:|---|
| MMLU-Pro | 87.8 | 57 fields of expert knowledge |
| GPQA Diamond | 88.4 | Graduate-level scientific reasoning |
| AIME 2026 | 91.3 | Competitive mathematics |
| LiveCodeBench v6 | 83.6 | Competitive programming |

### Coding and Agent

| Benchmark | Score |
|---|---:|
| SWE-bench Verified | 76.4 |
| SWE-bench Multilingual | 69.3 |
| SecCodeBench | 68.3 |
| TAU2-Bench | 86.7 |
| BFCL-V4 (Tool Calling) | 72.9 |
| BrowseComp | 69.0 / 78.6 (depending on strategy) |
| WideSearch | 74.0 |
| Tool Decathlon | 38.3 |
| MCP-Mark | 46.1 |
| HLE (Tool Use) | 48.3 |

### Multimodal

| Benchmark | Score |
|---|---:|
| OmniDocBench 1.5 | 90.8 |
| OCRBench | 93.1 |
| CC-OCR | 82.0 |
| VideoMME (with captions) | 87.5 |
| VideoMMMU | 84.7 |
| MLVU | 86.7 |
| MathVision | 88.6 |
| MathVista (mini) | 90.3 |
| ScreenSpot Pro | 65.6 |
| OSWorld-Verified | 62.2 |
| AndroidWorld | 66.8 |

Note the two reported values for **BrowseComp**: 69.0 and 78.6.

It's the same model and the same benchmark, but there's a 9.6-point difference depending on the strategy.

This shows that agent benchmark scores are a function not only of the model's pure intelligence but also of the **scaffolding and context management methods** used.

### Qwen3.6 Generation

| Benchmark | Qwen3.5-27B | Qwen3.6-27B | Qwen3.6-35B-A3B |
|---|---:|---:|---:|
| SWE-bench Verified | 75.0 | **77.2** | 73.4 |
| MMLU-Pro | 86.1 | — | 85.2 |
| GPQA Diamond | 85.5 | — | **86.0** |
| AIME 2026 | 92.6 | — | 92.7 |
| VITA-Bench | **41.8** | — | 35.6 |
| MMMU | — | — | 81.7 |
| OmniDocBench | — | — | 89.9 |
| RefCOCO (Spatial Intelligence) | — | — | 92.0 |
| VideoMMMU | — | — | 83.7 |
| ODInW13 | — | — | 50.8 |
| Artificial Analysis Intelligence Index | — | — | 43 |

It achieves almost the same math and science scores as these two models with 3B and 27B active parameters.

AIME 92.7 vs 92.6, GPQA 86.0 vs 85.5

On the other hand, it lags behind by 3.8 in SWE-bench and 6.2 in VITA-Bench.

This pattern suggests that while math and science problems can often be solved with a single accurate inference, agent tasks require maintaining state across multiple steps, making fewer active parameters a disadvantage in the latter.

The Artificial Analysis intelligence index of 43 is significantly ahead when compared to the median of 15 for open-weight models of similar size.

### Qwen3.8-Max

These are values announced by Alibaba with its release on August 3, 2026.

| Benchmark | Qwen3.8-Max | Comparison Target |
|---|---:|---|
| Terminal-Bench 2.1 | 86.6 | GPT-5.6 Sol 88.8 / Opus 4.8·Fable 5 84.6 |
| SWE-bench Pro | 67.7 | Fable 5 80.0 |
| FrontierSWE | 73.5 | Fable 5 88.8 |
| DeepSWE 1.1 | 56.6 | (3.7-Max is 21.6) |
| PaperBench | 93.0 | 1st place |
| IFBench | 82.8 | 1st place |
| GPQA Diamond | 92.6 | (3.7-Max 92.4) |
| OSWorld-Verified | 86.1 | GPT-5.6 Sol Max 83.2 / Fable 5 85.0 / Gemini 3.1 Pro 76.2 |
| Parametric CAD Bench | 91.5 | |
| OmniDocBench 1.5 | 92.1 | |
| CoWorkBench | 74.8 | |
| WideSearch | 81.9 | |
| JobBench | 53.4 | (3.7-Max 31.3) |

Specifications and Pricing

```
총 파라미터   2.4T
활성 파라미터  95B
컨텍스트     1,000,000 토큰 (일부 자료는 983,616)
최대 출력    131,072 토큰
사고 모드    항상 켜짐. low / high / xhigh 조절
가격        입력 $2 / 출력 $6 / 캐시된 입력 $0.25 (100만 토큰당)
API 호환    OpenAI 사양 + Anthropic 사양
```

The Qwen3.8-Max announcement is good training for reading vendor benchmarks.

**Two facts are simultaneously true in the same table.**

```
"Terminal-Bench 2.1에서 Claude Opus 4.8과 Fable 5를 앞섰다"   → 86.6 vs 84.6  ✓
"SWE-bench Pro에서 Fable 5에 크게 뒤진다"                     → 67.7 vs 80.0  ✓
"FrontierSWE에서 Fable 5에 크게 뒤진다"                        → 73.5 vs 88.8  ✓
```

Points to note when reading benchmarks are as follows.

1. **Only the former makes headlines.** When reading vendor announcements, it's necessary to get into the habit of looking at both favorable and unfavorable rows.
2. **No independent verification.** As of its release on August 3, there were no figures from independent evaluation bodies, including Artificial Analysis, and it had not yet appeared on community leaderboards. The difference between vendor tables and independent measurements becomes the real story.
3. **Different baselines for multimodal tables.** Alibaba's multimodal comparison uses Qwen3.7-Plus as the baseline, not Qwen3.7-Max. Since Plus is a lower tier than Max, the generational improvement appears larger than it actually is.

Vendor Benchmark Checklist

```
□ 누가 측정했나
  자기 모델 점수와 경쟁사 점수를 같은 팀이 냈다면,
  경쟁사 점수 쪽에 더 큰 의심이 필요합니다.

□ 어떤 하네스와 설정인가
  에이전트 벤치마크는 하네스에 민감합니다.
  BrowseComp 69.0 vs 78.6이 그 예입니다.

□ 비교 대상이 같은 등급인가
  Plus를 기준선으로 삼고 Max와 비교하면 격차가 부풀려집니다.

□ 불리한 행이 표에 있는가
  전부 이겼다는 표는 유리한 행만 골랐을 가능성이 큽니다.

□ 독립 측정치가 나왔나
  보통 며칠에서 몇 주 걸립니다. 그때까지는 판단 보류.

□ 사고 예산이 얼마였나
  xhigh로 측정한 점수와 low로 쓰는 실사용은 다릅니다.
```

### What's ultimately needed is your own evaluation set.

20-50 of your own business cases are more accurate than 100 benchmarks.

This is especially true for Qwen, which has suspicions of test-format data bias.

```py
#!/usr/bin/env python3
"""blind_eval.py — 모델 후보 블라인드 비교"""
import json, random, asyncio, aiohttp

MODELS = {
    "A": "http://localhost:8001/v1",   # Qwen3.6-27B
    "B": "http://localhost:8002/v1",   # Qwen3.6-35B-A3B
    "C": "http://localhost:8003/v1",   # 비교 대상
}

async def run_one(session, base_url, prompt):
    async with session.post(f"{base_url}/chat/completions", json={
        "model": "local", "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.0, "max_tokens": 2048,
    }) as r:
        d = await r.json()
        return d["choices"][0]["message"]["content"]

async def main(cases_path, out_path):
    cases = [json.loads(l) for l in open(cases_path)]
    async with aiohttp.ClientSession() as s:
        with open(out_path, "w") as out:
            for c in cases:
                results = {}
                for key, url in MODELS.items():
                    results[key] = await run_one(s, url, c["prompt"])
                # 라벨을 섞어서 평가자가 어느 모델인지 모르게 함
                labels = list(results.keys())
                random.shuffle(labels)
                out.write(json.dumps({
                    "case_id": c["id"],
                    "prompt": c["prompt"],
                    "criteria": c["criteria"],
                    "outputs": [{"shown_as": i + 1, "hidden_model": k,
                                 "text": results[k]} for i, k in enumerate(labels)],
                }, ensure_ascii=False) + "\n")

asyncio.run(main("my_cases.jsonl", "blind_results.jsonl"))
```

Evaluation cases are created in this format.

```json
{"id": "code-01",
 "prompt": "다음 Python 함수의 버그를 찾아 수정하세요:\n...",
 "criteria": "인덱스 오프바이원 오류를 지적하고, 수정된 코드가 실행되어야 함"}
```

<br>

## Glossary

| Term | Definition and Mechanism |
|---|---|
| **Active Parameters** | Parameters that actually participate in computation when processing a single token in MoE. Determines speed and computational cost. Conversely, **memory consumption is determined by the total parameters**, so "35B-A3B means it's as light as a 3B model" is a misunderstanding. |
| **dense vs MoE** | A structure where all parameters are activated every time vs. a structure where only some are activated. Dense is advantageous for batch size 1, while MoE is advantageous for large batches. |
| **Gated DeltaNet (GDN)** | A type of linear attention. Summarizes the past into a fixed-size state, consisting of delta rule (updates only differences) + exponential gating (selective forgetting) + Conv1D (local context) + Q/K L2 normalization. KV cache is eliminated, but precise retrieval is weak. |
| **Hybrid Attention** | A structure that mixes linear attention layers and full attention layers. Qwen uses a 3:1 ratio. A compromise that saves cost while retaining retrieval capabilities. |
| **Gated Attention** | The layer responsible for full attention in hybrid attention. Qwen3.5 minimizes cache by reducing KV heads to 2 even here. |
| **Shared Expert** | An expert in MoE that is always active for all tokens. Responsible for common knowledge. Removed in Qwen3 but revived in Qwen3.5 (512 experts). |
| **Global Batch Load Balancing Loss** | A penalty that encourages even expert usage, calculated globally rather than per batch. An approach that pushes specialization further. |
| **QK-Norm** | A technique that applies normalization to Q and K to prevent attention logit explosion. Ensures training stability. |
| **MTP (Multi-Token Prediction)** | Simultaneously predicts multiple next tokens during training. Improves long-range consistency and reuses that layer as a drafter for speculative decoding during inference. |
| **Speculative Decoding** | A draft proposes multiple tokens, and the main model verifies them at once. Lossless. Only beneficial when concurrency is low. |
| **Early Fusion** | Learning image/video tokens and text tokens together in the same sequence from the pre-training stage. Contrasts with late fusion, where adapters are attached later. |
| **thinking budget** | The upper limit of tokens to use for internal reasoning before answering. A knob to directly control cost and latency. |
| **YaRN** | An extension technique that increases RoPE's rotation period to handle contexts longer than the training length. Larger factors degrade short-context quality. |
| **strong-to-weak distillation** | A smaller model learns by imitating the output probability distribution of a larger model. The entire distribution contains more information than a single correct answer. |
| **MRL** | Matryoshka Representation Learning. A training method that allows users to truncate the dimension of embedding vectors. Enables balancing storage cost and quality. |
| **Instruction-Aware Embedding** | A method that provides instructions like "from what perspective to view similarity" when vectorizing. Useful for domain-specific applications. |
| **Dual Encoder vs Cross Encoder** | Vectorizing questions and documents separately (fast, pre-indexable) vs. judging them together (accurate, slow). A structural difference between embedding and reranker. |
| **harness** | A wrapper that provides tools, file access, and command execution to a model. The same model can have different performance depending on the harness. |
| **Tool Call Parser** | A server-side configuration that converts the tool call format output by the model into an OpenAI-compatible format. Qwen uses `qwen3_coder`. |
| **Inference Parser** | A configuration that separates and returns thought tokens from the final response. Qwen uses `qwen3`. |
| **scaffolding** | A prompt, tool, and loop structure that wraps an agent to enable it to perform tasks. Greatly influences agent benchmark scores. |

<br>

## Misconceptions and Pitfalls

### Is 35B-A3B as lightweight as a 3B model?

Active parameters determine speed, while total parameters determine memory.

35B-A3B is fast because its computational load is at a 3B level, but it requires loading the entire 35B memory. Based on Q4, this is about 20GB.

Judge self-hosting feasibility by total parameters.

### Is MoE always more efficient than dense?

For batch size 1 (personal local use), dense models are often the only viable option.

The advantage of MoE emerges **when multiple requests utilize different experts, filling the GPU**, and there's no such effect if there's only one request.

On the other hand, the burden of loading all parameters into memory remains.

This is why Qwen3.6-27B was preferred by the community.

### If it codes well, does it also perform agent tasks well?

Qwen3.6-35B-A3B is strong with a SWE-bench score of 73.4, but its VITA-Bench score is 5.6, which is lower than its contemporaries like 27B (41.8) or Gemma4-31B (43.0).

This means coding can be verified by a compiler, allowing for large-scale reinforcement learning, which leads to significant returns on focused investment.

For general agent tasks, the correct answer is often unclear, so the same approach doesn't work. These two capabilities should be measured separately.

### Since it's 1M Context, can I just put in 1M?

Increasing the YaRN factor allows handling long contexts, but it degrades the quality of short contexts.

It should be set to the minimum required length to match the actual workload.

And if you actually fill 1M, KV will explode, and prefill will take several to tens of seconds, so in most cases, reducing it with RAG is cheaper and more accurate.

## Additional Pitfalls

If you serve without `--tool-call-parser qwen3_coder`, the model generates tool calls normally, but the harness doesn't recognize them. This symptom appears as the agent not using tools, which can easily be mistaken for poor model performance.

Similarly, without `--reasoning-parser qwen3`, reasoning tokens are included in the final response.

### Serving Hybrid Models with Older Engines

Regarding **serving hybrid models with older engines**:

Qwen3-Next and later require a KV cache manager that handles both linear attention layers and full attention layers.

If the engine doesn't support that generation, loading will fail, or performance will be significantly degraded even if it loads.

### Confusing MTP GGUF and GGUF

To use speculative decoding in llama.cpp, you need to download a GGUF file with 'MTP' in its filename.

Giving `--spec-draft-n-max` to a regular GGUF has no effect.

MTP also uses additional VRAM. On a 16GB card, 35B-A3B with `--spec-draft-n-max 1` reduces the average context to around 15K.

### Deploying NVFP4 Checkpoints on Hopper

NVFP4 is natively processed by Blackwell's 5th-generation Tensor Cores.

Hopper H100/H200 lacks hardware support, so it's emulated and becomes slower than FP8.

You should choose checkpoints appropriate for your hardware: FP8 for Hopper, NVFP4 for Blackwell.

### Blindly Following Generations

At a pace where new generations emerge every six months, constantly switching to the latest version consumes excessive development time for validation.

Manage your evaluation set as an asset and only migrate when significant improvements are confirmed.

<br>

### Serving Argument Summary

```bash
# Common
--reasoning-parser qwen3              # Separate reasoning tokens
--tool-call-parser qwen3_coder        # Parse tool calls
--enable-prefix-caching               # Reuse prefix

# MoE
--enable-expert-parallel              # vLLM
--enable-ep-moe --ep-size N           # SGLang

# Multimodal
--language-model-only                 # Don't load vision encoder when only using text
--mm-encoder-tp-mode data             # Vision encoder data parallel
--mm-processor-cache-type shm         # Cache pre-processing results

# Context extension
--hf-overrides '{"rope_scaling":{"rope_type":"yarn","factor":2.0,
                 "original_max_position_embeddings":262144}}'
--max-model-len 524288

# Quantization (hardware-specific)
Hopper    : FP8 checkpoint
Blackwell : NVFP4 checkpoint
```

llama.cpp local recommended values (16GB VRAM)

```bash
# Qwen3.6-27B dense — Speed priority
--spec-draft-n-max 2 --cache-type-k q8_0 --cache-type-v q8_0

# Qwen3.6-27B dense — Context priority
--spec-draft-n-max 1 --cache-type-k q5_1 --cache-type-v q5_1

# Qwen3.6-35B-A3B MoE
--spec-draft-n-max 1 --cache-type-k q8_0 --cache-type-v q8_0
# Average context ~15K. Deeper drafts consume only VRAM
```

### Model Selection Summary

```
Local 24GB GPU, coding        → Qwen3.6-27B (Q4_K_M, 16.8GB)
Local 32GB+, multimodal        → Qwen3.6-35B-A3B
Server, agentic coding         → Qwen3-Coder-Next
Server, multimodal + agent    → Qwen3.5-397B-A17B
Large batch                  → Qwen3.5-4B / 9B (after validation)
Edge, on-device            → Qwen3.5-0.8B / 2B
Search vectorization                → Qwen3-Embedding-0.6B
Search reranking                → Qwen3-Reranker-4B
Highest performance (API)            → Qwen3.8-Max
```
