# DeepSeek V4.1 Flash Archaeology

While analyzing DeepSeek, let's address the following questions:
- There are many versions like V4-Lite, V4-Pro, V4-Flash-0731, V4-Pro-0813, but which ones are still active?
- Its name is "Flash," but does that mean it surpasses the previous generation's flagship, "Pro"?
- What are CED, CSA2, Engram, DSpark, and SWA Bounded Replay?
- It's 552B, but why are there two active parameter counts, 8B and 16B?
- Why is an 890-byte KV cache headline-worthy?
- If the same model's score changes by 9 points when the harness is switched, how should benchmarks be interpreted?
- It's said there's no chat template, so how should it be invoked?

Let's explore these questions.

## The DeepSeek Family

### Identity - A Family that Solves Cost Structures with Architecture

DeepSeek is a research lab that originated from the High-Flyer lineage in Hangzhou, China.

It has served as an architectural reference for the open-weight community, starting with V2 MLA in 2024, followed by V3 large-scale MoE in late 2024, the release of R1 inference in 2025, and V3.2's DSA.

What distinguishes it from other families is **how it defines problems**.

While Qwen competes with the breadth of its lineup and GLM with a deployment strategy of adopting others' harnesses, DeepSeek, in each generation, identifies precisely where the inference cost bottleneck lies and relentlessly optimizes only that.

```
V2   : Large KV cache        → MLA (KV compression into a low-dimensional latent space)
V3   : High cost per parameter → Large-scale fine-grained MoE + load balancing without auxiliary loss
V3.2 : Attention grows quadratically     → DSA (top-k selection with Lightning Indexer)
V4   : 1M context is expensive  → CSA + HCA (27% FLOPs, 10% KV compared to V3.2)
V4.1 : Agents are input-heavy → CED + CSA2 + FP4 KV (KV 890 bytes/token)
```

The title of the V4.1-Flash technical report is **Pushing the Limits of KV Cache Compression** - it highlights the cache, not intelligence.

This demonstrates the nature of DeepSeek's technology.

### Why Optimize KV Cache?

Because long-horizon agents have changed the nature of serving workloads.

Typical chatbot-like services tend to ask short questions and give long answers. However, coding agents explore entire repositories, call tools, read results again, and call them again. In each turn, the entire trajectory so far needs to be **prefilled** again.

This means hundreds of turns where the output is a few hundred tokens, and the input is hundreds of thousands of tokens.

In such workloads, the actual costs are concentrated in three areas:

```
① Prefill computation: The cost of re-inserting the entire context every turn.
② HBM (GPU memory): A larger KV cache reduces the number of concurrent users.
③ SSD storage: The cost of storing KV on disk for cache hits.
```

DeepSeek directly stated in an X announcement: "Cache hit charges often account for a large portion of agent costs; compressing the cache significantly reduces these costs." All architectural choices in V4.1-Flash target these three items.

### Open Policy - MIT

While Qwen uses Apache 2.0, Kimi uses Modified MIT, and GLM has started applying its own licenses to its flagships, DeepSeek still uses pure MIT, opening up even its base models with no revenue gates, no UI attribution requirements, and no restrictive usage clauses.

| Date | Model | Introduced |
| --- | --- | --- |
| 2024.5 | DeepSeek-V2 | **MLA** Introduced. Compressed KV cache into a low-dimensional latent space. |
| 2024.12 | DeepSeek-V3 | 671B-A37B. Fine-grained MoE + shared experts. Load balancing without auxiliary loss. |
| 2025.1 | DeepSeek-R1 | Inference model. Disclosed thought processes. |
| Late 2025 | DeepSeek-V3.2 | Introduced **DSA (DeepSeek Sparse Attention)**. Lightning Indexer. |
| 2026.3.9 | DeepSeek-V4-Lite | 200B. Technical preview of V4 architecture (CSA+HCA). |
| 2026.4.24 | **DeepSeek-V4-Pro / V4-Flash** | 1.6T-A49B / 284B-A13B. 1M context. **CSA + HCA**, mHC, Muon. MIT. |
| 2026.7.24 | — | Deprecated `deepseek-chat` / `deepseek-reasoner` legacy aliases. |
| 2026.7.31 | V4-Flash-0731 | In-place upgrade of `deepseek-v4-flash` route. |
| 2026.8.13 | V4-Pro-0813 | V4-Pro Official GA. |
| 2026.8.16 | — | Switched to peak/off-peak dual pricing. |
| **2026.9.10** | **DeepSeek-V4.1-Flash** | **CED, CSA2, FP4 KV, Engram, DSpark. Native multimodal. 45T tokens. MIT.** |

As of the V4.1-Flash release, the API model list was streamlined simultaneously with its public release. No API changes are needed.

```
Official Model ID  →  deepseek-flash        ← This name will be used going forward.

Deprecated (alias remains, routed to V4.1-Flash)
  deepseek-v4-flash
  deepseek-v4-flash-vision-exp

Routing after grace period
  deepseek-v4-pro
  → Maintained only until September 14, 2026, 12:00 Beijing Time (UTC 04:00, 13:00 KST)
  → After this, all traffic will be routed to V4.1-Flash at V4.1-Flash pricing.
  → A temporary bridge until V4.1-Pro is released.
```

If you were using V4 Pro, prices will drop significantly. Based on off-peak rates, Pro's $0.022 / $0.66 / $1.98 will become Flash's $0.003 / $0.15 / $0.60, representing reductions of approximately 86% / 77% / 70% respectively.

However, this is not a discount but a migration. Since the base model is a newly trained model with a different architecture, prompt responses, tool call patterns, and output styles may differ. Even if the raw capability increases, it's advisable to test production traffic before September 14th.

### Lineup

| Item | V4-Flash | V4-Pro | V4.1-Flash |
| --- | --- | --- | --- |
| Total Parameters | 284B | 1.6T | **552B (Backbone)** |
| Active Parameters | 13B | 49B | **8B (Prefill) / 16B (Decode)** |
| Layers | — | — | **40 (Encoder 20 + Decoder 20)** |
| Attention | CSA + HCA | CSA + HCA | **Pure CSA2** |
| Global KV Cache | Baseline | — | **890 bytes/token (approx. 1/4)** |
| Context | 1M | 1M | 1M |
| Max Output | 384K | 384K | 384K |
| Multimodal | Separate Vision Exp | Text-only | **Native (Image)** |
| Thought Mode | Non-Think / High / Max | Same | **Continuous value 1~100** |
| Pre-training | — | — | **45T Multimodal Tokens** |
| License | MIT | MIT | MIT |

### Parameter Count, Memory

```
552B  ← "backbone parameters" in the model card body
196B  ← Engram conditional memory module (accounted separately)
485B  ← Hugging Face safetensors aggregated display
475GiB ← Actual checkpoint size (FP8 + FP4 expert mix, 48 shards)
```

Be aware that there are differences due to mixed storage precision depending on whether the backbone, Engram, and vision encoder are counted separately or not.

When sizing hardware, you should look at the checkpoint file size, not any paper's numbers.

### Flash, but Stronger than Pro?

This is an easily misunderstood point due to the name: V4.1-Flash is a low-cost tier in the V4.1 family, yet it simultaneously surpasses the **previous generation's flagship, V4-Pro**.

Comparing the base models:

| Benchmark | V4-Flash-Base (284B/13B) | V4-Pro-Base (1.6T/49B) | V4.1-Flash-Base (552B/8B·16B) |
| --- | --- | --- | --- |
| MMLU-Pro (EM) | 68.3 | 73.5 | **74.1** |
| SuperGPQA (EM) | 46.5 | **53.9** | 53.1 |
| SimpleQA-Verified (EM) | 30.1 | **55.2** | 42.3 |
| BigCodeBench (Pass@1) | 56.8 | 59.2 | **60.6** |
| HumanEval (Pass@1) | 69.5 | 76.8 | **79.4** |
| GSM8K (EM) | 90.8 | 92.6 | **93.0** |
| MATH (EM) | 57.4 | **64.5** | 61.1 |
| LongBench-V2 (EM) | 44.7 | **51.5** | 45.2 |

**In world knowledge and coding, it catches up to V4-Pro-Base, with 1/3 the total parameters and 1/4 the active parameters.**

However, in areas that genuinely differentiate, such as memorized factual knowledge (SimpleQA 42.3 vs 55.2) and long-document comprehension (LongBench-V2 45.2 vs 51.5), the 1.6T Pro still leads. This is a familiar pattern where reducing parameters first impacts these areas.

> It's also important to note that V4.1-Pro has not yet been released. DeepSeek itself described V4.1-Flash as the smallest model in the new architectural family, designed to scale to larger models. What we are seeing now is the minimum size of the new architecture.

<br>

## Architecture

### CED (Causal Encoder-Decoder) Halves Prefill

In a typical decoder transformer, every layer computes its own KV. If there are 40 layers, a single prompt token is processed 40 times, and 40 layers' worth of KV accumulates.

However, in agent workloads, prompts are primarily read. If you input 300,000 tokens of repository code, do those tokens require the same deep processing as tokens that will be generated later?

### Solution

The 40 layers are **split into 20 causal encoder layers and 20 decoder layers**. The decoder does not compute its own global KV. Instead, for each layer, it extracts all 20 decoder layers' worth of global KV from projection weights and a single final hidden state of the encoder.

```
                    ┌─ Prompt tokens stop here
                    ↓
Input → [ Encoder 20 layers ] ─ Final Hidden State
                              │
                              ├→ Decoder Layer 1 Global KV (Projection)
                              ├→ Decoder Layer 2 Global KV (Projection)
                              ├→ ...
                              └→ Decoder Layer 20 Global KV (Projection)

              [ Decoder 20 layers ] → Output Generation
```

To use an analogy, instead of twenty people each reading a thick document from scratch, the first twenty people create one summary, and the next twenty people interpret and use that summary in their own ways. Instead of reading the original twenty more times, the summary is transformed in twenty different ways.

The origin of this idea comes from YOCO (You Only Cache Once) research; DeepSeek didn't invent it, but it's an instance where it was actually implemented in a frontier-level MoE.

#### Effect - Why There Are Two Active Parameter Counts

```
Prefill (Input Processing)  : Only passes through 20 encoder layers  → 8B active per token
Decode (Output Generation)  : Encoder + entire decoder → 16B active per token
```

The reason the model card lists two active parameter counts, 8B / 16B, is due to this structure. Prefill computation is effectively halved, which directly translates to cost savings in input-heavy workloads.

### SWA Bounded Replay - No SSD Usage

With CED, global KV is extracted from the encoder, but **Sliding Window Attention (SWA)** still operates across all layers. The window size is 128 tokens.

The problem arises during a cache miss: to restore the decoder's SWA state, it would traditionally have been stored somewhere, typically on an SSD.
> SWA (Sliding Window Attention): An attention mechanism that only considers the most recent N tokens instead of the entire context. It's used in a division of labor where global attention handles distant context and SWA handles immediate local context. Here, the window size is 128 tokens.

#### Solution

The solution is to recompute rather than store. Since the window is 128 tokens, replaying only the last 128 tokens accurately restores the SWA state.

```
Old:  Persistently store SWA KV on SSD  →  Storage cost + transfer latency
V4.1:  Recompute only the last 128 tokens on cache miss
       → Recomputation amount is fixed at 128 tokens, not "number of layers × window size"
```

At the deployment level, it's divided as follows:

```
SWA KV   : A distributed pool created by dedicating 10% of host DRAM, TTL of several minutes.
Global KV  : Guaranteed lifespan of 72 hours.
```

Consequently, the KV capacity persistently stored on SSD becomes approximately 1/8 of V4-Flash. This is a solution to the SSD storage bottleneck, one of the three bottlenecks mentioned earlier.

### CSA2 - Layer Axis Shares Cache.

V4 used a mix of CSA and HCA. V4.1-Flash uses pure CSA2, shifting the compression direction from the sequence axis to the layer axis.

> **CSA:** Compressed Sparse Attention, which first compresses the KV cache and then selects (sparse) only the core tokens needed for the question or current situation. It's a structure that focuses on important long-term memories or highly relevant specific areas, reducing unnecessary computational costs while maintaining accuracy.
>
> **HCA:** Heavily Compressed Attention, which compresses very strongly by creating only one KV at regular intervals (e.g., every 128 tokens). It performs dense attention on the entire compressed history data without selecting the top k. It broadly and cheaply scans the overall flow or general meaning of the context rather than focusing on specific details.

One of three modes is statically assigned to each attention layer; it's determined during training and doesn't change during inference.

| Mode | Own main KV | Own indexer K | Top-K Indices | Function |
| --- | --- | --- | --- | --- |
| **Full** | Computes | Projects here | **Selects anew** | Does everything itself |
| **Reindex** | Reuses previous Full | Reuses previous Full | **Rescores with own indexer Q** | Re-selects the same data from its own perspective |
| **Reuse** | Reuses previous Full | Reuses | **Keeps previous as is** | Skips the indexer entirely |

**All layers retain their own main Q and SWA KV.** Only KV and index selection are shared.

To use an analogy, when an investigation team has narrowed down 20 candidates from a pile of documents:
- **Full**: Creates a new pile of documents and selects new candidates.
- **Reindex**: Uses the same pile of documents but re-selects, saying, "These 20 are relevant based on my interests."
- **Reuse:** Receives both the document pile and the 20 candidates as-is, and only interprets them individually.

**Actual Configuration**

```
18 Encoder CSA2 Layers
  Compression ratio 2, 3 groups of 6 layers each
  Each group = Full 1 + Reuse 5

20 Decoder Layers
  Compression ratio 1, 5 groups of 4 layers each
  Group 1 = Full 1 + Reuse 3
  Groups 2-5 = Reindex 1 + Reuse 3
```

Only 3 encoder layers and 5 decoder layers actually run the indexer anew. The rest use what others have selected.

> Compared to GLM-5.2's IndexShare, the idea is similar but the sophistication differs. IndexShare had two types: full, shared, shared, shared. CSA2 introduces an intermediate "reindex" step, creating the option to share memory but allow each to decide what to focus on. This reduces the quality loss from sharing.

#### TOP-512

The number of items selected by sparse attention is the top 512, which is much more aggressive than the 2,048 in GLM/DeepSeek V3.2 series. This makes the quality of the indexer's selection crucial.

### Hierarchical Sparse Indexer - Breaking the Quadratic Term of the Indexer

#### Remaining Problem

The attention in sparse attention becomes linear, O(Lxk), but the indexer still has to scan the entire context, making it O(L²). This becomes a problem again when the context reaches 1M.

#### Solution

In the decoder, the first full layer creates a candidate pool, and subsequent reindex layers only score within that pool.

```
First Full Layer
  Scores the entire causally visible range (only here is it O(L²))
  → Candidate pool composition: 2,048 blocks of 8 positions each = maximum 16,384 positions

Subsequent Reindex Layers
  Scores only within 16,384 items (fixed regardless of context length)
  → Selects Top-512 from here
```

**The indexing cost for deep layers is capped independently of context length. Whether it's 1M or 100K, the Reindex layer sees 16,384 candidates.**

This structure yields the following figure: **When the context increases 250-fold from 4K to 1M, single-token decode FLOPs increase by 1/4.**

### FP4 KV Cache

The main KV cache is quantized to **E2M1 format (4-bit)**, and an **E4M3 scale is attached every 16 channels**. This follows NVIDIA's NVFP4 method but without a global scale.

> **E2M1 / E4M3:** Floating-point numbers represented by Exponent and Mantissa bits. E2M1 has 2 exponent bits, 1 mantissa bit, and 1 sign bit, totaling 4 bits. Since the precision of values is very low, 16 channels are grouped, and an 8-bit E4M3 value is separately assigned as the scale for that group to compensate for the representable range.

The important point is that **this was introduced as Quantization-Aware Training (QAT) during the post-training phase.** Instead of simply compressing an already trained model to 4 bits later, the model was made to adapt to the 4-bit cache during training. This results in half the storage compared to V4's FP8 cache.

### Summary - 890 bytes/token

The combination of the above factors yielded this result:

```
Global KV Cache: 890 bytes per token

  vs DeepSeek-V4-Flash  approx. 1/4
  vs DeepSeek-V1        approx. 1/437
```

And the two axes at the deployment level are summarized as follows:

```
HBM (GPU Memory) Requirement  : 1/4 of the previous generation
SSD Persistent Storage Requirement : 1/8 of the previous generation
```

Even if you fill 1M tokens, the global KV is only around 890MB. That's insane, legendary.

The reason this is amazing is that it quadruples the number of concurrent sessions that can be handled by a single GPU, which directly impacts the cost per token.

### Single-Pass mHC

mHC (Manifold-Constrained Hyper-Connections) was already included in V4.

It's a technique that extends residual connections into multiple branches while maintaining stability through manifold constraints.

It's a component that stably trained a 1.6T MoE with 6.7% overhead by limiting signal amplification to less than 2x.

V4.1-Flash's Single-Pass mHC **shifts the input mixing coefficients one block forward.** This allows multiple operations to be processed by a unified **Mega-mHC kernel**, halving activation memory traffic.

This is not an optimization that changes the computation results, but a rearrangement of order to group the same computational kernels into one.

### Engram - 196B Conditional Memory

It's a separate module attached to layers 1 and 14, possessing **196B parameters and accessed sparsely via token-based lookups.**

If MoE routing is about "which expert should compute this," Engram is like a cache for "what stored knowledge should be retrieved for this token." It's a lookup, not a computation.

> Note: Engram was originally a separate research paper and was discussed online in conjunction with V4, but was not actually implemented in V4. It was actually implemented in V4.1-Flash. This distinction must be made when reading V4-related materials.

In terms of parameter accounting, this is calculated separately from the 552B backbone, which is a major source of the numerical confusion seen earlier.

### DSpark - Speculative Decoding

It's a semi-autoregressive draft generation + confidence schedule verification structure. After pre-training, the **backbone is frozen and trained separately.**

- **Speculative Decoding:** A technique where a lightweight drafter proposes multiple tokens in advance, and the main model verifies them in a single forward pass. Since the cost of reading weights once is nearly the same whether generating one token or verifying multiple, it uses leftover computational resources to confirm multiple tokens.
- **Semi-autoregressive draft:** Instead of generating tokens one by one sequentially, the drafter outputs multiple tokens at once, reducing the latency of the draft itself.
- **Confidence schedule verification:** The drafter adjusts its proposals to be longer for confident sections and shorter for less confident ones, reducing the waste of always proposing 'n' tokens only to have most rejected.

? The community, based on GGUF repository notation, indicates public checkpoints use DSpark n = 3.

### MoE Configuration

```
Per MoE Layer
  1 Shared Expert
  384 Routing Experts
  6 Active Routing Experts per Token
```

This is a consistent pattern since DeepSeek V3: one shared expert plus multiple fine-grained routing experts. If it's 6 out of 384, it's a very sparse routing at about 1.6%.

> Qwen3.5 uses 10 out of 512 + 1 shared, and GLM-5.2 uses 8 out of 256 + 1 shared. The point of finely splitting experts and having a single shared expert is the same.

### Multimodal Early Fusion

```
DeepSeek-ViT (Trained from scratch)
  2D-RoPE
  3x3 Pixel Unshuffle Downsampling
      ↓
2-layer MLP Projector
      ↓
Visual Embedding → Processed with Text Embedding
```

**The core idea is that the language model processes the image model together from the beginning of pre-training.** This is not late fusion, where a vision adapter is added after the text model is fully built, but rather a method where both are learned in the same representational space from the start.

> Thanks to this, a separate vision model is not needed, and indeed, `deepseek-v4-flash-vision-exp` was deprecated in this release.

The multimodal scores for the base model are as follows:

```
MMMU-Pro (4-shot)     56.5
CVBench (4-shot)      77.9
DocVQA (LLM-Judge)    95.6
RefCOCO-avg (0-shot)  86.0
```

**Document understanding (DocVQA 95.6) and spatial grounding (RefCOCO 86.0) are particularly high.** This ability to read screenshots and pinpoint coordinates is a prerequisite for computer-using agents.

### Pre-training

```
Corpus     45T Multimodal Tokens (Text:Multimodal = 7:1)
Trained from scratch with sparse attention sequence length 64K
           ★ No dense warmup
Context extended to 1M at 34T token mark
```

"No dense warmup" is a notable point. GLM-5 used a two-stage strategy when introducing DSA, continuously pre-training from a dense base.

The reason was that training sparsely from the beginning was astronomically expensive. DeepSeek, this time, did it sparsely from the start.

### Post-training

There were no algorithmic changes. The model card stated that the **post-training recipe follows the standard SFT >> RL >> OPD (On-Policy Distillation) paradigm without algorithmic modifications, and all substantial changes are in the data pipeline.**

Three changes:

1. Gradually scaling large-scale automated synthesis of verifiable agent task environments and data task rollout.
2. RL in dual scaffold transformation, Claude Code, Codex, OpenCode, Pi, mini-SWE, DeepSeek Harness -> Simultaneous training across multiple harnesses instead of fitting to a single one.
3. On-policy distillation from over 40 teacher models.

> OPD: A method where the teacher model aligns its probability distribution on trajectories generated by the student model itself. Unlike off-policy distillation, which merely imitates data created by the teacher, this approach minimizes distribution shift issues as corrections occur in the state distribution actually reached by the student.

GLM5.3 measured all coding agent benchmarks on a single Claude Code harness, while DeepSeek ran RL on multiple harnesses.

### Harnesses

DeepSeek released the full results of running this model on 8 scaffolds.

Conditions: DeepSWE v1.1 uses N=8 per task, Terminal Bench 2.1 uses N=3, Linux container, `temperature=1.0`, `top_p=0.95`, 1M context, max steps 500 per agent, Terminal-Bench 2.1 network access blocked.

| Benchmark | Claude Code | Codex | OpenCode | Pi | mini-SWE | DSH Minimal | DSH Standard | DSH PTC |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| DeepSWE v1.1 (Resolved) | 69.8 | 65.6 | 65.5 | 66.2 | **74.2** | 72.6 | 70.5 | 67.6 |
| Terminal-Bench 2.1 (Pass@1) | 88.0 | 84.1 | 85.0 | 86.1 | 90.3 | **90.6** | 85.8 | 85.8 |

#### 1. **The highest scores came from lightweight harnesses.**

DeepSWE 74.2 is mini-SWE, and Terminal-Bench 90.7 is DSH Minimal; both are minimalist categories with fewer tools.

> DSH Minimal is a preset of the DeepSeek Harness (DSH), providing only a persistent bash shell and an `str_replace` editor, with a single system prompt: "You are a helpful software engineer assistant." The agent doesn't even know it's running within DSH.
> **Standard** is a full set equipped with file editing, shell, file web search, skills, plans, goals, sub-agents, and workflows. PTC and Creator are additional presets tailored for other task profiles.

#### 2. Providing more tools can actually worsen performance.

DSH Minimal drops from 72.6 to Standard 70.5 to PTC 67.6. The community's observation is that as tools increase, the schema and prompt exposed in the first turn become longer, and the number of choices expands, making ability selection less predictable.

> The community summarizes this as GA DeepSeek V4 being overfitted to Minimal, but a more accurate description is that the V4 series' first-turn tool prompt environment is unusually sensitive, and this observation aligns with the fact that public code agent evaluations were measured on DSH Minimal.

#### 3. Still performs well on other harnesses.

Scores of 69.8/88.0 on Claude Code are not significantly different from the highest values, suggesting the effectiveness of dual-scaffold RL.

> Practical Principle: Benchmarks comparing models solely by name are meaningless, so let's record both the model and harness settings. Tool access, permissions, context configuration, and agent loops all alter the path a model takes to complete a task.

## Benchmarks

### Frontier Comparison by Max Thought Budget

Based on `reasoning_effort=100`, `temperature=1.0`, `top_p=0.95`, bold values indicate the top performer in that row.

| Benchmark | Opus-5.0 | GPT-5.6 Sol | Kimi K3 | GLM-5.3 | DS-V4-Pro | DS-V4-Flash | DS-V4.1-Flash |
| --- | --- | --- | --- | --- | --- | --- | --- |
| GPQA Diamond | 93.4 | **94.1** | 92.9 | 88.1 | 92.4 | 89.9 | 90.9 |
| HLE | **56.3** | 44.5 | 43.5 | 42.0 | 42.7 | 37.8 | 36.8 |
| Codeforces (Rating) | — | — | — | — | 3348 | 3289 | **3471** |
| MathArena Apex | — | — | **65.6** | — | 65.3 | 58.6 | **65.6** |
| Terminal-Bench 2.1 | 89.1 | 88.8 | 88.3 | 88.2 | 87.9 | 82.7 | **90.6** |
| Terminal-Bench 3.0 | **43.3** | 34.4 | 17.7 | 28.3 | 11.8 | 7.6 | 30.0 |
| Terminal-Bench 4.0 | **51.8** | 39.9 | 12.6 | 37.9 | 12.4 | 7.0 | 31.2 |
| DeepSWE v1.1 | 74.0 | 73.0 | 67.5 | 66.9 | 62.7 | 54.4 | **74.2** |
| ProgramBench | **37.0** | 23.0 | 17.5 | 19.0 | 15.5 | — | 20.3 |
| NL2Repo-Bench | **75.3** | 56.8 | 58.0 | 58.0 | 61.5 | 54.2 | 64.0 |
| CyberGym | — | 84.5 | 80.0 | 84.5 | 83.3 | 76.7 | **88.1** |
| SEC-Bench Pro | — | **74.3** | — | — | 56.4 | 30.9 | 62.8 |
| ExploitGym | 22.1 | **33.7** | — | 15.0 | 5.4 | 1.8 | 15.3 |
| HLE w/ tools | 63.6 | — | 59.8 | 62.5 | 60.0 | 51.5 | **63.9** |
| AutomationBench | 50.3 | 45.8 | 46.7 | 48.8 | 43.2 | 37.7 | **54.8** |
| Agent's Last Exam | 28.6 | 26.7 | 27.6 | 28.5 | 25.7 | 25.2 | **31.8** |
| Chartography w/ tools | **84.0** | 79.9 | 68.1 | — | — | — | 78.9 |
| BabyVision w/ tools | **94.1** | 88.9 | 85.7 | — | — | — | 89.6 |
| ZeroBench-main w/ tools | 52.0 | **53.0** | 41.0 | — | — | — | 49.0 |
