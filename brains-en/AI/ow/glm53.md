# GLM-5.3 Analysis

Today, I will study the following points, with a reference date of August 31, 2026:

- GLM-5, 5.1, 5.2, 5.3, and 5.3-Flash all came out within half a year. What is the baseline?
- "The base model remains the same, and only post-training is scaled." What does this mean?
- The 753B Flagship and 320B Flash are both 5.3. Why do they have different architectures?
- What are DSA, IndexShare, IndexPool, mHC, SAO, and slime?
- Isn't GLM MIT licensed? Why is only the flagship model under a proprietary license?
- They claim to have beaten frontier models in cybersecurity benchmarks. How much should we trust those numbers?
- What kind of hardware can run it?

## The GLM Family

### Identity - A Family That Thrives by Leveraging Others' Harnesses

GLM is a model family created by Z.ai (智谱, Zhipu AI), originating from the THUDM lab at Tsinghua University.
Starting with the GLM paper in 2021 and the ChatGLM chatbot in 2023, it has now reached the GLM-5 generation.

What distinguishes it from other families is its **distribution channel**. While Qwen builds its presence through a wide lineup and DeepSeek through architectural papers, GLM thrives by **integrating into coding agents that people are already using.**

The most symbolic example is Anthropic's compatible endpoint:

```
https://api.z.ai/api/anthropic        ← Claude Code, Goose, etc., connect directly
https://api.z.ai/api/coding/paas/v4   ← OpenAI compatible tools (Cline, Kilo, Roo, etc.)
https://api.z.ai/api/v1               ← Codex
```

From the perspective of Claude Code, this endpoint is indistinguishable from the Anthropic API. By simply changing two lines of environment variables, Claude Code can directly use GLM. Instead of creating its own CLI to attract users, it adopts a strategy of flowing its model into the plumbing of existing harnesses.

This strategy is evident in benchmarks. The footnotes of the official GLM-5.3 benchmark state that almost all coding agent categories were measured using `Claude Code 2.1.207`. This means that the frontier claims of a Chinese open-weight model are being measured through American agent software, which **reveals more about the current state of the tooling layer than the model's score itself.**

| Period | Model | Introduced Features |
| --- | --- | --- |
| 2025.7 | **GLM-4.5 / 4.5-Air** | 355B-A32B / 106B-A12B. 23T tokens. Thinking/non-thinking hybrid. MIT |
| 2025.8 | GLM-4.5V | Air-based VLM. 3D vision encoder + 3D-RoPE |
| Late 2025 ~ Early 2026 | GLM-4.6 / 4.7 | Improved versions of 4.5 series. Direct comparison baseline for 5th generation |
| 2026.2 | **GLM-5** | 744B-A40B. 28.5T tokens. **DSA introduced**. 200K context. Entirely trained on Huawei Ascend |
| First half of 2026 | GLM-5.1 | Same architecture as GLM-5, RL redesigned for coding specialization |
| 2026.6.16 | **GLM-5.2** | **IndexShare introduced**. Context 200K → 1,048,576. SAO applied to agentic RL. MIT |
| 2026.8.14 | **GLM-5.3** | Base frozen, only post-training scaled. Cyber capability emergence. Weights announced 2 weeks later |
| 2026.8.26 | **GLM-5.3-Flash** | 320B-A18B. New base. **Sparse + Linear Hybrid Attention**, mHC. Native multimodal. MIT |
| 2026.8.28 | GLM-5.3 weights released | 753B. **Proprietary `glm-5.3` license, not MIT** |

### Open-Closed

Unlike Qwen, which maintains separate open and Max Closed lines, GLM **releases its flagship models as open-weights.** The top-tier model available via API and the model downloadable from Hugging Face are the same.

However, there's a different kind of time lag: GLM-5.3 was first released via API on August 14th, and its weights were released on August 28th.

Z.ai stated that **cyber capabilities developed faster than expected, requiring two weeks for safety evaluation and hardening.**

> From a practical perspective, it's worth remembering this pattern: the release and download dates for GLM models can differ. During this interim period, it's an API model, not an open-weight one. When planning self-hosting, set your timeline based on the date it's uploaded to repositories.

### License

GLM was MIT licensed for a long time, but in August 2026, within a week, the same company applied different licenses to two models.

| Model | License | Status |
| --- | --- | --- |
| GLM-5.3-Flash (320B) | **MIT** (unmodified) | Released August 26 |
| GLM-5.3 (753B) | **`glm-5.3` proprietary license** | Released August 28 |

The body of the `glm-5.3` license largely follows MIT, with additional clauses explicitly permitting use, reproduction, performance, merging, distribution, sublicensing, and sale without restriction, and explicitly including **model weights, parameters, configuration files, inference and training code** in the definition of software.

It also adds a separate line granting rights for execution, deployment, fine-tuning, and creation of derivative works.

```
Clause 2 (Summary)

If the licensee or its affiliates operate a Model as a Service business, and
the combined revenue of the licensee and its affiliates exceeds $10 billion over 12 consecutive months,
they must pass a security review by Z.AI before commercial use.
```

- **Definition of MaaS**: Hosting weights directly and selling API access to third parties that gives them "substantial control over input, parameters, and training data."
- **Exclusions from MaaS**: ① End-user products where model capabilities are embedded only within specific functions or harnesses. ② Simply relaying requests to models hosted by others.

| Model | Total/Active | Context | Multimodal | License | Status |
| --- | --- | --- | --- | --- | --- |
| GLM-4.5 / Air | 355B-A32B / 106B-A12B | 128K | Separate VL model | MIT | Maintenance |
| GLM-5 | 744B-A40B | 200K | Text | MIT | Maintenance |
| GLM-5.1 | 744B-A40B | 200K | Text | MIT | Maintenance |
| GLM-5.2 | 744B-A40B | 1M | Text | MIT | 5.3 base. API automatically routes to 5.3 |
| **GLM-5.3** | **753B-A40B** | **1M** | Text | `glm-5.3` | **Flagship frontier** |
| **GLM-5.3-Flash** | **320B-A18B** | **1M** | **Native (Text, Image, Video)** | **MIT** | **Efficiency frontier** |

The reason why various numbers like 44B, 750B, 753B, 754B appear for what seems to be the same model is:

```
744B  ← GLM-5 Technical Report (arXiv 2602.15763) main text
750B  ← Referred to as "750B-A40B" in SAO paper
753B  ← Aggregated from safetensors in Hugging Face GLM-5.3 model card
754B  ← Community aggregation related to GLM-5.2
```

The difference lies in whether MTP Draft Layer, Embedding, and Router are counted.

No number is wrong; they just count different scopes. **When sizing hardware, you should look at the actual checkpoint file size, not the numbers in the paper.**

### GLM-5.3, GLM-5.3-Flash: Brothers in Name, Different in Bloodline

These two are not like a large and small version of the same model; their **base models are different from the start.**

| Item | GLM-5.3 | GLM-5.3-Flash |
| --- | --- | --- |
| Base | Reuses GLM-5.2 base | **Newly trained base** |
| Pre-training Corpus | 28.5T (inherits GLM-5) | **30T, Multimodal** |
| Total/Active | 753B / 40B | 320B / 18B |
| Number of Layers | 78 | **45** |
| Attention | MLA + DSA + IndexShare | **KDA (Linear) + NoPE Sparse MLA Hybrid + IndexPool** |
| Residual Connection | Standard | **mHC** |
| Experts | 256 routing + 1 shared (8/token) | 288, 8/token |
| Multimodal | Text-only | Native (Image, Video) |
| HF Architecture Tag | `glm_moe_dsa` | `glm5_next` |
| License | `glm-5.3` | MIT |
| API Price (1M tokens) | $1.40 / $0.26 cache / $4.40 | **$0.15 / $0.50** |

The price difference is roughly 10x. According to Z.ai's own performance metrics, Flash generally outperforms GLM-5.2 in benchmarks.

- **If you need top performance and budget is not an issue**: GLM 5.3 still excels in coding agent top scores.
- **If cost-per-intelligence is the goal**: Flash. With 18B active layers and 45 layers, latency is also lower.
- **If you need to handle images and videos**: Only Flash. GLM 5.3 is text-only.
- **If considering local execution**: Definitely Flash. As discussed later, its memory footprint is less than half.
- **If you are self-hosting to sell inference**: Read the license. Flash is MIT, 5.3 is not MIT.

<br>

## Architecture

The GLM-5 generation architecture is built in three layers.

```
GLM-5
  MLA + DSA introduced
  → Foundation for sparse attention

GLM-5.2
  + IndexShare
  → Further reduces the overhead of sparsity

GLM-5.3
  No architectural changes. Only post-training
  → Shifts to a different axis

GLM-5.3-Flash
  Linear + Sparse Hybrid, IndexPool, mHC
  → New base, new experiments
```

### Layer 1: MLA + DSA (GLM-5)

#### Problem to Solve

As context length increases, attention becomes unmanageable. If there are N tokens, Q-K pairs are N^2, so computation grows quadratically, and the number of concurrent users is limited proportionally to the KV cache length.

GLM-5 addresses this problem by combining two approaches:

**MLA (Multi-head Latent Attention)** caches a compressed, lower-dimensional latent space instead of full K/V tensors for each head. This reduces the volume of the KV cache itself.

**DSA (DeepSeek Sparse Attention)** divides attention into **two stages: selection and computation.**

```
① Lightning Indexer
   Lightly calculates relevance scores between the current query and all preceding tokens
   → Selects only the top 2,048 positions

② Main Attention
   Performs normal attention only on the selected 2,048 positions
```

> To draw an analogy, when looking for an answer in a library, instead of looking through all ten thousand books, you first use index cards to select about 20 relevant books, and then you only read those 20 books thoroughly. Skimming the index cards still involves ten thousand operations, but the cost of looking at one card is cheaper than reading a book.

The difference from fixed patterns like sliding windows is that it selects based on content, meaning if a sentence 300 pages back is important now, it can be chosen.

Let's accurately understand the cost structure here:

```
Main Attention: O(L × k)   k = 2,048 fixed  →  Linear with length
Indexer       : O(L²)                       →  Still quadratic
```

The problem here is that the **indexer is still quadratic**. Although the cost per operation is much cheaper, making it less noticeable, running this across 78 layers repeatedly means it's still a significant chunk when the context reaches 1M.

The GLM-5 technical report states that DSA was introduced by **Continued Pre-Training** from a dense base. To avoid the astronomical cost of training sparsely from scratch, a two-stage strategy of "dense warm-up" -> "sparse adaptation" was used.

It reported nearly equivalent performance to MLA models in long-document benchmarks, and when fine-tuned with the same SFT data, training loss and evaluation scores were comparable.

> RL Stability Detail: DSA's indexer selects top-k tokens and computes attention only on that subset. The report states that **the result of top-k is critical for RL stability.** In MoE, this is similar to the problem of preserving activated top-k experts for routing replay to ensure training-inference consistency. If the tokens selected during training and inference differ, training becomes unstable.

### IndexShare GLM-5.2

#### What observation did it start from?

IndexShare researchers observed that adjacent DSA layers select 70-100% overlapping tokens. That is, the indexer in a neighboring layer was finding almost the same positions that the previous layer had just found.

#### Solution

Run the indexer only in one layer and **reuse the list of selected positions** for subsequent new layers.

```
GLM-5.2's repeating pattern (except for the first few layers)

[ full, shared, shared, shared ] × ...
  │     └──────────────────────┘
  │       └ Indexer not run
  │         Borrows the list of positions selected by the previous layer
  └ Layer that runs its own DSA indexer
```

Here, `full` does not mean dense attention, but rather that it runs its own indexer over the entire range, not attention. All layers still perform sparse attention.

And `shared` layers still compute their own queries, attention weights, value combinations, output projections, and FFN updates. The only thing they borrow is the list of which positions to look at. Hidden states and attention outputs continue to change as they pass through layers.

> To draw an analogy, imagine four people in a meeting room each drawing different conclusions from the same stack of documents. One person does the initial screening, identifying the 20 most relevant documents, and the other three then read those 20 documents from their own perspectives.

The reported effect is a 2.9x reduction in FLOPs per token for 1M context.

**Trade-offs and Caveats**
- This is an estimate of computation at that context length, not an indication that end-to-end speed will be 2.9x faster.
- Z.ai itself states that **KV cache memory does not decrease at the same rate.** For 1M tokens, cache capacity, long-context kernels, CPU scheduling, and cache transfer are still significant serving costs.
- For the trained model, this pattern was not added at inference time by attaching a cache, but rather by incorporating it during mid-training with 128K sequences. This gave the surviving indexers a chance to adapt to the dependent out-layers.

> For reference, a separate 30B DSA model experimented with 200K context, showing a maximum 1.82x acceleration for prefill and 1.48x for decode by retaining only a quarter of the indexers. Since these are measurements for different models and lengths, they should not be interpreted as GLM 5.2 figures, but rather as evidence for the reuse idea itself.

#### MTP Layer Reuse Application

GLM-5.2 also revamped its MTP layers for speculative decoding. The first draft step calculates indices, and subsequent draft steps reuse those indices along with the preceding KV cache. This combined rejection sampling with an end-to-end total-variation loss.

In reported ablations, the entire MTP change **increased the average accepted draft length from 4.56 to 5.47 tokens (+20%)**. However, since the table shows cumulative changes, the individual contribution of IndexShare is not isolated.

> **MTP (Multi-Token Prediction):** During training, instead of predicting just one next token, it predicts multiple tokens simultaneously, allowing it to learn more far-sighted representations, which improves long-range consistency. During inference, these layers are directly reused as the draft model for speculative decoding, eliminating the need to create a separate draft model.
> **Speculative Decoding:** A lightweight drafter proposes multiple tokens in advance, and the main model validates them with a single forward pass. The cost of reading weights once is almost the same whether generating 1 token or validating 6, so multiple tokens can be confirmed using the remaining computational resources. "Accepted draft length" refers to the average number of tokens that passed validation in one go, and this number is close to the speedup factor.

### GLM-5.3-Flash Hybrid

Flash re-architected its attention structure while training a new base, **mixing linear attention and sparse attention within a single model for the first time in the GLM series.**

```
GLM-5.3-Flash Attention (45 layers)

KDA Linear Attention Layers        ← Summarizes the past into a fixed-size state. "Remembers"
NoPE Sparse MLA Layers          ← Precisely picks out distant relevant parts with a lightweight indexer. "Searches"
```

The roles are clearly divided: **linear attention handles local dependencies through state modeling, while sparse attention pulls in relevant parts of the global context using an indexer.**

- **KDA (Kimi Delta Attention):** Z.ai adopted a linear attention variant from the Moonshot family as its hybrid linear axis. Delta-rule based linear attention like Gated DeltaNet and KDA are becoming common components in the open-weight community.
- **NoPE (No Positional Encoding):** This involves removing explicit positional encoding from that layer. In a hybrid setup, linear attention layers sequentially update states, so positional information is already carried. Therefore, removing positional encoding from sparse attention layers still maintains context and even facilitates length extrapolation.

#### IndexPool

Using an indexer requires creating and holding separate key vectors for the indexer. For 1M context, this indexer cache itself becomes a burden on latency and memory.

**IndexPool compresses 4 indexer key vectors into 1 using weighted pooling.** This reduces the resolution seen by the indexer by a factor of four, but in return, reduces the indexer cache by a factor of four.

Since the selection stage is primarily about roughly filtering candidates, a slight loss in resolution is judged to have a minor impact on the final attention quality.

#### mHC (Manifold-Constrained Hyper-Connection)

This is an improvement to the residual connection. Standard transformers simply add the output of each block to its input, while hyper-connection variants extend these connections in multiple branches to increase expressiveness. The problem is that leaving them as is can lead to unstable training. mHC places these extended connections under manifold constraints, maintaining stability while improving scaling efficiency.

Z.ai describes this only as an improvement in scaling efficiency and does not provide separate quantitative figures.

##### Effects

Compared to GLM-5.3, Z.ai's own measurements show:

**Attention computation reduced by 3.01x, KV cache size reduced by 4.44x.**

The number of layers also halved from 92 in GLM-4.5 to 45, and active parameters decreased from 32B to 18B.

This design allowed for **similar total parameters (320B vs 355B)** while reducing cost per token.

According to Z.ai's comparison data, Flash has the lowest attention computation per layer and head when compared to GLM-5.3, DeepSeek-V4-Flash, and Kimi K3.

However, its KV cache is still slightly larger than DeepSeek-V4-Flash and Kimi K3, indicating room for improvement.

### So What Did GLM-5.3 Change? - Nothing

This is the core claim of this release: GLM-5.3 uses the same base model as GLM-5.2. All improvements came from post-training.

Architecture, tokenizer, and pre-training are all identical. Only the pipeline after pre-training has changed.

And the results are as follows:

| Benchmark | GLM-5.2 | GLM-5.3 | Change |
| --- | --- | --- | --- |
| Terminal-Bench 3.0 | 4.6 | 28.3 | Approx. 6x |
| FrontierSWE | 67.5 | 78.1 | +10.6 |
| DeepSWE v1.1 | 46.2 | 66.9 | +20.7 |
| SWE-Marathon v1.1 | 19.4 | 42.5 | More than 2x |
| ExploitBench | 24.4 | 54.4 | More than 2x |
| CyberGym | 77.2 | 84.5 | +7.3 |
| Z.ai Code Bench (private) | Baseline | +50% | Self-measured |

**By pouring more RL environment and compute into a frozen base, the agentic terminal coding score increased 6x.**

If this is reproducible, the competitive question for the next year will not be "who pre-trains the largest base?" but "who runs the best post-training pipeline on an existing base?"

Z.ai itself noted that "the intelligence ceiling of this base model may not have been reached yet."

### Post-Training Stack - SAO, slime, Environment

Three components made this possible, all of which were already introduced in GLM-5.2 and are not new to 5.3.

#### SAO - Single-Rollout Asynchronous Optimization

> To clarify the name: secondary sources refer to SAO differently as Scalable Agentic Optimization, Self-play Alignment Optimization, or Self-Adaptive Optimization. The title of the original paper (arXiv 2607.07508, Hou et al., 2026) is Single-Rollout Asynchronous Optimization. The GLM team themselves use it this way.

The problem it aims to solve is that in typical chatbot RL, rewards are immediate. The model responds, and that response can be graded.

However, in real agent tasks like fixing bugs in a repository, rewards may only come after tens to hundreds of steps.

Identifying which of those many steps contributed to the outcome is the credit assignment problem.

Here, an asynchronous problem overlaps:

```
Synchronous RL: Generates an entire batch of rollouts → Updates model → Repeats
            The whole process waits for the slowest rollout

Asynchronous RL: A group of generators continuously streams trajectories
            The trainer updates as they arrive
            GPUs are not idle
```

The problem is that GRPO doesn't fit well with asynchronicity. GRPO typically samples about 8 responses for the same prompt and uses their group average as a baseline. However, a synchronization barrier is hidden within the group, where the entire group must wait for the slowest member, and in an asynchronous environment, that wait quickly leads to stale updates.

This worsens when long agent trajectories are compressed into multiple sub-trajectories. **The number and length of learnable trajectories for the same prompt vary per rollout,** breaking the clean, comparable groups that GRPO relies on. GRPO's fundamental premise is shattered.

#### Solution

SAO, instead of patching the group, **eliminated it.** It trains with 1 rollout per prompt. However, it had to revive what GRPO had discarded.

- **Group sampling -> Single-rollout sampling**
- **Revival of the critic**: Since the free baseline of group average is gone, a real critic must be trained. Critic updates are more frequent, and a frozen-attention technique is used, where only MoE projection is trained while attention is fixed.
- **Bilateral token-level clipping DIS**: Masks diverging gradients to prevent policy lag and training collapse.

#### Results

```
Vanilla GRPO: Collapses at approx. 160 asynchronous steps
SAO         : Trains stably for approx. 1,000 steps
              BeyondAIME 54.8 → 74.8
              Outperforms GRPO-based methods in SWE-Bench Verified, IMOAnswerBench
```

For three years, everyone removed value models, but in long-horizon asynchronous learning, value models have returned.

> **Caution when reading:** The ablation results are from a Qwen3-30B-A4B backbone (core foundation), i.e., a 30B-scale lab. The paper does not disclose the wall-clock or GPU time for GLM-5.2's main training. It's best to consider this as a promising pattern for long-horizon agentic RL, rather than a firmly established truth.

> Ablation, meaning surgical removal, is a research method that verifies the importance of a component or feature in an AI model by removing it and observing the change in performance.

#### slime - Asynchronous RL Infrastructure

This is a post-training framework open-sourced by THUDM.

```
Training: Megatron
Rollout: SGLang
Core: Trajectory generation and model updates do not wait for each other
```

Reported system performance figures for GLM-5.3:
- **2.3x improvement in end-to-end RL training throughput** for long-horizon coding tasks.
- **Training rollout log probability controlled to a divergence level of 1e-7.**

> Why is the second point important? If the inference engine generating trajectories and the trainer learning from them produce different probabilities for the same input, the training will reinforce actions that never actually occurred. In asynchronous RL, this discrepancy quietly accumulates and can collapse training. 1e-7 suggests that the two paths are effectively performing the same calculations.

Both are internal metrics, so there's no external way to verify them. However, they explain where more compute went.

#### Environment - The Real Bottleneck Here

The most interesting part is not the algorithms, but the environment.

Z.ai did not hand-write training tasks. **Research agents collected recurring patterns from real engineering work and synthesized them into executable multi-step environments.** This involves embedding hidden states in the environment that the model must discover through its actions.

What was expanded in 5.3:
- **Expanded long-horizon task environments by tens of times:** Some tasks correspond to several days of continuous engineering work.
- **Increased environment types and diversity.** ML optimization clusters, security analysis frameworks, real development tooling.
- Specifically, in **real ML optimization clusters**, the model was made to use the same compute, storage, documentation, and experimentation systems as human algorithm engineers to achieve measurable end-to-end acceleration.
- **Prevented reward hacking.** Identified and blocked shortcut pipelines that only increased scores without actually completing the task.

<br>

## Cyber Capabilities - The Most Controversial Part of This Release

Z.ai intentionally included vulnerability discovery data and environments in post-training. The expectation was a gradual improvement in security reasoning.

> "We thought this would make the model more capable at vulnerability discovery and reasoning. What surprised us was how quickly that capability developed as we scaled training."

The model began to reason across multiple stages of exploits and consistently planned complete exploit chains.

| Benchmark | GLM-5.2 | GLM-5.3 | Fable 5 (in table: w/ fallback) | GPT-5.6 Sol |
| --- | --- | --- | --- | --- |
| CyberGym (Vulnerability Discovery) | 77.2 | **84.5** | 83.8 | 83.6 |
| ExploitBench (Exploitation) | 24.4 | 54.4 | **78.0** | 76.5 |
| ExploitGym (2h / 6h) | 29 / 39 | 105 / 130 | 181 / 247 | **216 / 293** |

```
"Outperformed frontier models in CyberGym"        → 84.5 vs 83.8  ✓
"23.6 points behind in ExploitBench"               → 54.4 vs 78.0  ✓
"Less than half in ExploitGym 2-hour budget"   → 105 vs 216    ✓
```

CyberGym measures the ability to find and verify flaws in source code, while ExploitBench and ExploitGym measure the ability to escalate those into actual exploits. GLM-5.3 is first on the lower rung of the ladder but significantly behind on the upper rungs.

And Z.ai included this in their release statement:

> "The higher up the exploit chain a benchmark is, the greater the improvement compared to GLM-5.2, and at the same time, the larger the remaining gap with closed frontier models. Capabilities are growing fastest where we are most behind."

It's rare to see such self-criticism in a launch announcement. This is an honest way to read the security section.

### Public Vulnerability Ledger

Z.ai launched a public ledger at `cvd.z.ai` to track AI-assisted vulnerability discoveries through a structured embargo process.

```
Vulnerabilities Discovered    2,436 / 269 open-source projects
Severity         Critical 107 · High 990 · Medium 1,286 · Low 53
               (Critical+High 1,097 cases)
Disclosure Status      53 disclosed, 2,383 embargoed
Oldest one  1981
Average Latency  26.6 years
```

The recent weekly feed of the ledger includes actual assigned CVE IDs such as Linux kernel 6lowpan use-after-free (CVE-2026-64452), WebKit/Safari memory handling bug (CVE-2026-43663), FreeBSD ptrace parameter validation flaw (CVE-2026-45253), GStreamer heap OOB write (CVE-2026-59691), Suricata SMTP/MIME parsing bypass (CVE-2026-57229), and Joomla stored XSS (CVE-2026-48952).

The ledger statistics are Z.ai's own aggregation, and CVE IDs are recorded as actual assigned identifiers, not vendor-generated labels.

**This is a defensive narrative:** the model surfaces flaws, a human-operated embargo program responsibly discloses them, and patches follow through normal CVE procedures. However, as noted in the licensing section, the concern that delayed the release by two weeks did not leave a trace in the license document.

## Benchmarks

| Benchmark | GLM-5.3 | GLM-5.2 | Kimi K3 | DeepSeek-V4 Pro | Qwen3.8-Max | Opus 4.8 | Fable 5 | GPT-5.6 Sol |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Terminal Bench 2.1 | 88.2 | 81.0 | 88.3 | 87.9 | 86.6 | 85.0 | 88.0 | **88.8** |
| Terminal Bench 3.0 | 28.3 | 4.6 | 17.4 | – | – | 21.1 | 33.7 | **34.6** |
| DeepSWE v1.1 | 66.9 | 46.2 | 67.5 | 62.7 | 56.6 | 58.0 | 69.7 | **72.7** |
| NL2Repo | 58.0 | 48.9 | 58.0 | 61.1 | 55.9 | **69.7** | – | – |
| ProgramBench | 19.0 | 9.5 | 17.5 | – | 10.5 | 15.5 | **33.0** | 23.0 |
| FrontierSWE | 78.1 | 67.5 | – | – | – | 66.5 | **88.2** | – |
| SWE-Marathon v1.1 | 42.5 | 19.4 | 48.1 | – | – | **48.8** | 33.1 | 42.5 |
| PostTrainBench | 39.8 | 31.7 | 32.0 | – | – | 32.9 | **41.8** | 36.2 |
| CyberGym | **84.5** | 77.2 | 80.0 | 83.3 | 78.5 | 78.1 | 83.8 | 83.6 |
| ExploitGym (2h/6h) | 105/130 | 29/39 | 36/70 | – | 14/26 | 80/120 | 181/247 | **216/293** |
| ExploitBench | 54.4 | 24.4 | 32.2 | – | 28.8 | 40.0 | **78.0** | 76.5 |
| Toolathlon Verified | 73.0 | 59.9 | **76.5** | 74.1 | 72.5 | 76.2 | 74.7 | 74.9 |
| AutomationBench | **48.2** | 26.2 | 46.7 | 43.2 | 39.8 | 41.0 | 46.2 | 45.8 |
| Agents' Last Exam (CLI) | 28.5 | 23.8 | 27.6 | 25.7 | 27.0 | 25.7 | 23.8 | **28.6** |
| HLE w/ Tools | 62.5 | 54.7 | 59.8 | 60.0 | 56.2 | 57.9 | 63.9 | **64.5** |
| GDPval-AA v2 | **1769** | 1508 | 1682 | 1590 | 1739 | 1588 | 1743 | 1730 |
