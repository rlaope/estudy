# Kimi Model Analysis

As of August 12, 2026

> It's good to have prior knowledge of Transformer block configurations, what MoE and active total parameters are, KV cache, GQA MLA speculative decoding, quantization formats, harnesses, and parsers.

Kimi K3 comes with several new names: Kimi Delta Attention, Attention Residuals, Stable LatentMoE, Quantile Balancing, Pre-Head Muon, SiTU.

As you delve into Kimi, you'll encounter the following questions. Let's study with the goal of being able to answer them.

- What has changed between **K2, K2 Thinking, K2.5, K2.6 Code, and K3**, all released within a year?
- What are KDA, AttnRes, Stable LatentMoE, MuonClip, and what problems were they designed to solve?
- Linear attention is an old idea, so why was it incorporated into a 2.8 trillion parameter model?
- What exactly are the downsides of switching to a fixed-size state?
- Who exactly is running a 2.8 trillion open-weight model?

## Quick Math Review Before Starting

A few mathematical formulas will appear later. If you already know them, you can skip this section. However, if you're new to the papers, it's helpful to know the notation for cross-referencing, so I've compiled them here.

| Symbol | Meaning |
|---|---|
| `q` | Query vector. Represents "what is needed right now". |
| `k` | Key vector. "The label attached to this information". |
| `v` | Value vector. "The actual content to be stored". |
| `S` | State matrix. Carries a summary of past information. The core of KDA. |
| `α` | Forgetting rate. Closer to 0 means faster forgetting, closer to 1 means longer retention. |
| `β` | Update strength. Indicates how strongly new information should overwrite existing information. |
| `n` | Number of tokens so far (context length). |
| `d` | Length of a single vector (number of dimensions). |

- `kᵀ` (superscript T) — Transpose, which lays a vertically oriented vector horizontally. When a vertical vector and a horizontal vector are multiplied, like `v kᵀ`, the result is a matrix (table). This operation creates a table from two vectors.
- `I` is the identity matrix; multiplying by it changes nothing. It's equivalent to the number 1. If you see a form like `I - something`, you can interpret it as subtracting 'something' from the original value.
- `Diag(α)` is a matrix with the values of α arranged on its diagonal. Multiplying by this means each dimension is multiplied by its corresponding α. This implies applying a different scaling factor to each dimension.
- `⊙` — Element-wise multiplication. Only multiplies elements at the same position. Different from matrix multiplication.

### Four Frequently Appearing Equations

We'll cover each of these in detail later, so for now, just familiarize yourself with their forms.

#### Writing to and Reading from State

```
쓰기 :  S ← S + v kᵀ
읽기 :  o = S · q
```

`v kᵀ` is "a table with content `v` attached to label `k`", which is then added to `S` to accumulate. When reading, the query `q` is multiplied to retrieve relevant content.

#### Delta Rule (Overwrite)

```
S ← S − β (S·k − v) kᵀ
         └─ Difference between current stored value and new value ─┘
```

Simply adding would mix old and new values, so we correct by subtracting only the difference. `S·k` is "the value read with this label right now", and `v` is "the value that should be".

#### State Transition KDA Final Form

```
A = Diag(α) (I − β k kᵀ)
    └ Per dimension ┘ └─ Overwrite ─┘
      forget differently
```

This matrix is multiplied by the state at each step; the front part handles forgetting, and the back part handles overwriting. The form of this matrix determines what the model can and cannot do.

#### Muon Optimizer

```
M ← μM + G           Accumulate gradients so far
O ← NewtonSchulz(M)  Normalize magnitudes in all directions
W ← W − ηO           Update weights with that
```

`G` is the gradient, `M` is the accumulated gradient, and `η` is the learning rate. The middle line is the core of Muon, and the rest is a typical optimizer.

## Kimi

Kimi is a family of models created by Moonshot AI in Beijing, China. The company name is Moonshot AI, and the product and model name is Kimi.

What distinguishes it from other labs is its continuous occupation of the upper limit of open-weight model size.

When K2 was released with 1T parameters in July 2025, it was the largest among publicly available open-weight models, and when K3 was released with 2.8 trillion in July 2026, it again became the largest.

The official blog stated that for 9 out of the past 12 months, Kimi models have been at the upper limit of open model size.

They are leading to that extent, and this is their strategy. It's important to understand why this is a strategy: being at the size limit attracts serving stack developers voluntarily, as they want to become engines capable of running the latest and largest models.

On the day K3 was released, the ecosystem was so active that vLLM kernels, Docker images, and deployment recipes were already prepared, and NVIDIA and AMD also participated in kernel development.

A second characteristic is that they also open-source their operational infrastructure. Usually, only the model weights are open, and how they are served remains closed. Moonshot has opened everything: the serving platform Mooncake, optimizer scaling reports, the attention architecture paper (Kimi Linear), and kernel implementations (FlashKDA).

The point where this actually had an effect was that with the release of FlashKDA, external contributors created and uploaded H100-optimized versions, and the vLLM team validated them and merged them into the production path within a day. Opening the kernel itself became a channel for performance improvement.

### Open Policy and License

Kimi does not operate separate API-only models and open models. They release the weights of their flagship model as is.

The model used via API is the same as the one downloaded from HuggingFace.

The license is a Modified MIT, with one additional MIT condition: if monthly active users exceed 100 million or monthly revenue exceeds $20 million, the model name must be displayed on the product screen. Below that, it's no different from MIT.

This is close to a clause preventing large corporations from secretly embedding it in their products.

There's a unique decision in their release approach. For K3, there was about a ten-day gap between the model announcement and the release of its weights. This approach was first proposed by the vLLM team and accepted by Moonshot, and the reason is practical.

If the announcement and weight release were to happen on the same day, the model team would be simultaneously finalizing product and API evaluations and stability reviews, causing checkpoints to constantly shift.

From the perspective of the open-source engine team, they would be dealing with a moving target, and deadlines would constantly be pushed back. By separating them, the model team can first fix the final checkpoint, tokenizer, and serving specifications, and the engine team can secure a stable integration period.

| Period | Model | Introduced Features |
|-------|----------------------|-----------------------------------------------------|
| 2025.2 | Moonlight-16B-A3B | Model that validated Muon optimizer at scale |
| 2025.7 | **Kimi K2** | 1.04T / 32B active. Trained with 15.5T tokens using MuonClip, 0 Loss Spikes |
| 2025.10 | Kimi Linear-48B-A3B | First public release of KDA. Prototype of K3 Attention |
| 2025.11 | **K2 Thinking** | Thinking mode. 256K. INT4 QAT. 200-300 consecutive tool calls |
| 2026.1 | **K2.5** | Native multimodal. Agent Swarm and PARL |
| 2026.4 | K2.6 | Hallucination rate improvement (AA-Omniscience 65% → 39%) |
| 2026.6 | K2.7 Code | Coding specialized. Approximately 30% reduction in thinking tokens |
| 2026.7 | **Kimi K3** | 2.8T. KDA + AttnRes + Stable LatentMoE. 1M context |

Although it was released six times within a year, the architecture actually diverged only once. The K2 series, including K2, K2 Thinking, K2.5, K2.6, and K2.7 Code, all use the same core framework.

It has a total of 1.04 trillion parameters, 32 billion active, 8 out of 384 experts + 1 shared, 61 layers, and MLA attention on all layers. The generational difference came from post-training and data, not from the structure.

The structure changed only once, transitioning from K2 to K3.

In the K2 series, the total parameters are 1.04T, with 8 out of 384 experts + 1 shared, 61 layers, MLA on all layers, context from 128->256K, residual connection is standard x + f(x), precision is INT4 weights, and the optimizer is MuonClip.

In the K3 series, total parameters increased to 2.8T, with 16 out of 896 experts + shared, approximately 93 layers, attention is 3 layers of KDA and 1 layer of Gated MLA. Context is 1M, residual connection is Block AttnRes, precision is MXFP4 weights + MXFP8 activations, and the optimizer is Per-Head Muon.

### Goal

#### The Goal of Agents

To understand the architecture, one must first know the goal. Kimi's goal is consistently agents.

K2's subtitle itself was Open Agentic Intelligence, and all subsequent generations moved towards tool calling, long-running tasks, and parallel orchestration.

Adopting this goal automatically brings two things.

**Context becomes longer.** Codebase snapshots, tool definitions, and hundreds of tool call histories accumulate in the full context. Instead of a few conversational turns, hundreds of thousands to a million tokens become the norm.

**The same prefix repeats:** An agent's next turn usually starts by resending the previous turn's prompt as is, and this characteristic later determines serving costs.

#### Traces Left in the Specifications

Kimi didn't just say it was targeting agents; it carved its specifications in that direction. The paper describes what criteria were used to make decisions when creating K2, and two decisions moved in opposite directions.

- **Increased experts to 384:** The referenced model, DeepSeek-V3, had 256, so they increased experts by about 50%. The rationale for this increase was also based on measurements: by fixing the number of active experts at 8 and varying only the total number of experts, performance consistently improved as the number increased, with no limits observed within the measurement range.
- **Reduced attention heads to 64:** The same DeepSeek-V3 had 128, so they cut it in half. Usually, both are increased when scaling parameters, but the reason for reducing heads, as directly stated in the paper, is that while more heads utilize memory bandwidth better and speed up short sentences, the burden on inference increases sharply with longer contexts. In other words, having more heads is beneficial for a few conversational turns, but it was a disadvantage for agent systems that involve processing entire codebases and making hundreds of tool calls.

In other words, they sacrificed the benefits of short contexts for an architecture designed for long contexts.

This is where the decision to go with agents manifested.

Keeping this perspective in mind makes K3's decisions much clearer, and subsequent developments like KDA, AttnRes, and Radix caching redesign all stem from the single goal of making long contexts affordable.

<br>

## Why Decoding is Slow

### Misconception

You might have heard that "attention is O(n²) and gets slow as it gets longer," but this refers to the prefill (reading the prompt all at once) stage. In the decoding stage, where tokens are generated one by one, it's O(n) per step.

And there's something more important: in decoding, what determines the time is not the computational load but the speed of reading data from memory.

Let's take arithmetic intensity as an example, which represents how many byte operations are performed when reading 1 byte from memory.

Taking the H100 as a baseline, its BF16 dense computation performance is approximately 495 TFLOPS per second, and its HBM bandwidth is 3.35 TB per second. Dividing these yields about 150 operations per byte. This value is the break-even point.

```
커널의 산술 강도 > 150 -> 연산 성능에 묶임 (compute-bound)
커널의 산술 강도 < 150 -> 메모리 대역폭에 묶임 (memory-bound)
```

Let's calculate the decoding attention. For head dimension d and context n, we need to read n x d elements for K and V respectively. If it's BF16, each element is 2 bytes, totaling 4nd bytes.

The operations, combining dot products and weighted sums, amount to 4nd operations.

```
산술강도 = 4 x n x d FLOPs / 4 x n x d bytes = 바이트당 약 1회
```

At 1/150th of the break-even point, the computational units are idle over 99% of the time, and the GPU is merely fetching numbers from memory.

To reduce latency here, reducing FLOPs isn't the answer because it's not a computational bottleneck to begin with. There are only two methods:

1. Reduce the bytes read, or
2. Generate more tokens from the bytes read once.

KDA is the former, and speculative decoding is the latter.

And what the amount of reading is proportional to differs.

```
전체 어텐션 : 4nd 바이트 -> 컨텍스트 길이 n에 비례해서 계속 자란다.
선형 어텐션 : 2d² 바이트   →  n과 무관하게 고정
```

If n is 1 million, the difference is orders of magnitude, and this is where K3 stepped in, to endure even with an increased context.

<br>

## KDA - Rewriting the Sequence Axis

KDA stands for Kimi Delta Attention. It is a core component of the K3 architecture, first validated as a separate model called Kimi Linear in October 2025, and then integrated into K3.

### Step 1 - Transforming Cache into State

Instead of storing the entire past, it summarizes it into a single matrix whose size never changes.

```
상태 S : d × d 행렬 하나

쓰기 : 새 토큰이 오면 S를 갱신
읽기 : o = S · q  (질의 벡터를 곱함)

컨텍스트가 1천이든 100만이든 S의 크기는 같음
→ 스텝당 읽는 양이 상수
```

This idea itself is shared by all linear attention families like Mamba, RWKV, and GLA.

What differentiates KDA are two things built on top of this.

### Enabling Overwrites

Naive linear attention only adds new information.

```
S = S + v kᵀ
```

The problem is immediately apparent: writing to the same key twice causes the values to merge.

For example, if you write "meeting room = 3rd floor" and later "meeting room = 5th floor," reading it would result in "8th floor."

In other words, the longer the context, the more the state becomes jumbled and corrupted.

The delta rule operates by erasing before writing.

```
S_t = S_{t-1} − β_t (S_{t-1} k_t − v_t) k_tᵀ
                     └── 지금 저장된 값과 새 값의 차이 ──┘
```

It calculates "what would I get if I read this key now?" and subtracts the difference from the value that should be there.

It's good to keep one perspective in mind here: if we set the objective function as `L(S) = ½‖S k − v‖²`, the gradient becomes `(S k − v) kᵀ`. This is identical to the equation above, meaning the delta rule is performing one step of gradient descent on this objective function with a learning rate β.

From this perspective, related research falls into place.

> An objective function is a mathematical function that quantitatively defines the value to be maximized or minimized in an optimization problem. It serves as a criterion for finding optimal variables or states by maximizing benefits, efficiency, performance, etc., and minimizing costs, losses, errors, etc.

- Changing the objective function -> different memory update rules
- Varying the learning rate β per token -> adaptive updates
- Elevating from 1st order to 2nd order -> momentum or Adam-style state updates
- Running multiple steps -> TTT (Test Time Training) family
- Learning the objective function itself -> Titans family

**Designing update rules for linear attention is equivalent to designing online optimization algorithms.**

Most of the work of transferring knowledge from optimization literature to this field has not yet been done.

Existing models used a single scalar α.

- Mamba2: A single scalar per head or block
- Gated DeltaNet: A single scalar per head
- KDA: α ∈ [0,1]^d, independent forgetting rate for each dimension

Forgetting with a single scalar means the entire state fades at a uniform rate. This implies that information that needs to be retained for a long time and information that is only needed immediately disappear at the same speed.

In a fixed-size state, the state itself is an information budget, and this means the budget is not being used efficiently.

By assigning different forgetting rates to each channel, the budget can be allocated across different time scales. Some dimensions might have long time constants, storing document-level information, while others have short ones, storing only the preceding few words.

That is, Mamba2 used a scalar value per head block to forget past memories, and Gated DeltaNet had all feature dimensions within a head forget or retain past memories at the same rate. KDA, however, uses a **channel-wise diagonal matrix** instead of a scalar, allowing each specific dimension to have its own independent forgetting speed. This is a strategy to quickly erase certain information while independently retaining core information for longer.

Empirical results also exist. In sequence inference tasks like Palindrome, MQAR, and Stack, KDA converged faster and achieved higher accuracy than Gated DeltaNet and Mamba2. MQAR (Multi-Query Associative Recall) is particularly significant as it directly measures computational memory capacity by benchmarking how many key-value pairs can be accurately stored relative to the state size.

There was also a side effect. α acted as a learned positional decay, eliminating the need for separate positional embeddings like RoPE. This is because the natural fading of old information inherently encodes how old it is.

> Promoting a scalar hyperparameter to a vector is a recurring effective technique in this field, offering a significant increase in expressiveness with almost zero additional cost.

### What KDA Cannot Do - Expressiveness Theory

It's common to hear that linear attention is weak at precise retrieval.

While true, this is too vague. We need to understand what it cannot do and how, to know where to compensate.

#### State Transition Matrix Coordinate System

All linear RNN families can be expressed in this form.

```
S_t = A_t · S_{t-1} + (입력 항)
```

Here, A_t is called the state transition matrix.

It's the matrix multiplied when the previous state transitions to the next state.

The structure of this matrix determines the model's expressiveness.

Placing each model in this coordinate system yields the following:

- **Mamba2:** `A_t` is a diagonal matrix, only independently decaying each dimension.
- **DeltaNet:** `A_t = I - β k kᵀ`, with a rank-1 update added to the identity.
- **KDA:** `A_t = Diag(α) (I − β k kᵀ)`: applies channel-wise decay and a rank-1 update.

**A linear RNN with only positive eigenvalues in its transition matrix cannot solve parity.** This holds true no matter how many layers are stacked.

Here, "cannot solve parity" means it cannot be solved with a general 3D formula or rotation method. It implies that a dedicated algorithm is required.

Parity is the problem of determining whether the number of 1s in an input bit string is odd or even. It seems very simple, but it's one of the easiest state tracking tasks. Non-linear RNNs like LSTMs solve it easily.

The reason state tracking is actually important is that all these fall into this category:

- Tracking variable values while following code execution
- Tracking piece positions on a chessboard
- Matching deeply nested parentheses

It's intuitive why it can't be solved. The way to solve parity with a single state is to multiply the state by -1 every time a 1 appears.

The sign needs to be flipped, but if the eigenvalues are confined to the [0, 1] range, flipping is impossible. Only fading is possible.

The Mamba family's design, where gates are set to 0 or 1, meaning they only express "how much to forget," precisely corresponds to this limitation.

### Reflection Changes the Class

A solution can be elegantly achieved in the DeltaNet family:

```
A_t = I − β k kᵀ

β 범위가 [0, 1] →  고유값 [0, 1]    사영에 가까움
β 범위가 [0, 2] →  고유값 [−1, 1]   β=2에서 완전한 반사
```

`I − 2kkᵀ` is what is called a Householder reflection matrix in linear algebra.

It flips a vector across the plane orthogonal to k. If β is allowed up to 2, the delta rule can perform reflections, and at that moment, parity can be solved.

> Explaining what β means in this equation, it's a geometric memory update principle discussed in recent research extending DeltaNet's expressiveness, such as the DeltaProduct model. **Beta signifies an update gate and learning rate that determines how much of the existing memory to erase and overwrite with the currently input information.**

A one-line modification that doubles β changes the class of expressiveness. Both training and inference costs remain the same.

There's an even stronger result. If the transition matrix is a product of matrices of the form `I - vvᵀ` and its eigenvalues are within the range [-1, 1], then that linear RNN can learn any regular language.

The limitation is also clear: it can only represent permutations that swap two elements, which can be expressed by a single Householder reflection.

To solve more complex permutation problems in a single layer, a rank-n transition matrix is required, which is the approach of the DeltaProduct family. The cost is non-diagonal matrix multiplication.

### So, Where Does KDA Stand?

Let's look at KDA's transition matrix again.

```
A_t = Diag(α_t) (I − β_t k_t k_tᵀ)
```

This is a special case of DPLR (Diagonal-Plus-Low-Rank), where a rank-1 update is applied to a diagonal component.

It has higher expressiveness than Mamba2's purely diagonal form, and while more limited than general DPLR, it is much cheaper.

**Here arises an unresolved question:** while the range of α is publicly known to be [0,1], there is no explicit documentation for the range of β.

```
β ∈ [0, 1] 이면  →  고유값이 양수  →  parity를 못 풂
β ∈ [0, 2] 이면  →  반사 가능      →  정규 언어 학습 가능
```

The answer can be found by checking the scale of the activation function that generates beta in the public weights and kernel code. This would take half a day to run, and digging deeper would be endless for now, so let's leave it at that.

For the second question, the eigenvalues of the matrix resulting from multiplying Diag(α) and (I − βkkᵀ) are not simply the product of the eigenvalues of the two matrices.

It needs to be calculated whether channel-wise decay increases expressiveness or binds eigenvalues more strongly to [0, 1]. The first experiment would be to extract the layer-wise eigenvalue distribution from the Kimi Linear checkpoint.

> When looking at a new linear attention model, the first questions to ask are: what is the structure of its transition matrix, and what is the range of its eigenvalues? Most of the remaining design decisions stem from that.

### Hybrid

Reclaiming what was lost.

This involves distributing the structure layer by layer.

K3 addresses the loss of expressiveness by keeping only some layers as full attention. K3 made that choice.

```
K3의 층 배치 (반복 단위)

[ KDA → MoE ] [ KDA → MoE ] [ KDA → MoE ] [ Gated MLA → MoE ]
└──── 값싼 요약 3층 ────────────────────┘ └─ 정밀 검색 1층 ─┘
                 75%                              25%
```

K3 uses MLA every four layers; three of these four layers have no KV cache at all, and only one is responsible for precise global retrieval.

```
KV 캐시  : 네 층 중 한 층만 유지  →  75% 감소
디코딩   : 1M 컨텍스트에서 최대 약 6배
           (토큰당 11.5ms → 1.8ms 수준으로 보고)
```

Let's confirm that 75% precisely corresponds to a 3:1 ratio.

Before simply writing down the reported figures, let's double-check them ourselves.

Even if we make the extreme assumption that full attention layers constitute 25% and the attention cost of KDA layers approaches 0, if attention accounts for all the time, Amdahl's Law dictates a maximum speedup of 1 divided by 0.25 = 4x.

```
이론 상한 4배 < 보고치 6배
```

While 6x is far from sufficient, it can be further explained.

- **Full attention layers are not just plain attention**: They are Gated MLA, and MLA is a structure that already reduces the KV cache by compressing keys and values into a low-rank latent space. Therefore, the cache reduction effect is greater than the layer ratio.
- **Decoding is bandwidth-bound:** As calculated earlier, the number of bytes read determines the time, so a 75% cache reduction is reflected differently from the FLOPs ratio.
- **Different baseline for comparison:** If compared to a model with full attention in all layers, the conditions are not the same.
- **Kernel optimization can contribute separately:** The effect of fused kernels.

> It's a good habit to calculate the upper bound and then try to explain the difference before noting down reported figures, as dissecting the gap between theoretical complexity and actual performance is beneficial.

<br>

## Putting Recurrence on the GPU

There's a wall of sequentiality. Because a mathematically elegant formula is completely different from one that's fast on a GPU.

Since the state update equation requires the state at time t to calculate t+1, if you put it directly on a GPU with thousands of cores, most of the cores will sit idle.

This is why linear attention couldn't leave the lab for a long time.

The solution is **chunk-level parallelization**.

```
청크 크기 C = 64 기준

1) 시퀀스를 64개씩 잘라냄
2) 청크 안 : 순차 갱신을 하나의 조밀한 표현으로 압축해 행렬곱으로 처리
3) 청크 간 : 상태만 순차로 전달

순차 스텝 수 : T개  →  T/64개
연산 구성    : 대부분 행렬곱  →  텐서 코어를 탐
```

The problem is point 2: how to perform sequential updates, or how to compress them into matrix multiplications.

#### What can be compressed

- **WY Representation:** If you multiply rank-1 updates sequentially within a chunk, you get the form `∏(I − β_i k_i k_iᵀ)`. Expanding this product compresses it into a compact form `I − W Yᵀ`. W and Y are two matrices created from the key vectors within the chunk.

This is a numerical linear algebra technique originally developed to handle products of Householder transformations. Since it replaces sequential multiplications with two matrices, the entire chunk can be processed at once.

What's interesting here is that we saw the transition matrix of the delta rule in the previous section was of Householder form, and **the very form chosen for its expressiveness also brought with it the potential for parallelization here.**

> Householder is a concept in linear algebra that refers to a transformation or reflection. It's a transformation that perfectly symmetrically moves (reflects) a vector as if mirrored across a certain plane. When you multiply a vector x by a matrix H, the vector is flipped to the symmetric point on the opposite side of x, relative to the plane perpendicular to v.

- **UT Transformation:** Creating the WY representation requires the inverse of a lower triangular matrix in the middle. Explicitly calculating the inverse generates many operations that are not matrix multiplications, but UT transformation rewrites this part as matrix multiplications. The Kimi Linear report explicitly states that reducing non-matrix multiplication FLOPs was crucial.

**DPLR Specialization:** A general DPLR transition matrix is `Diag(a) + b + cᵀ`, where a, b, and c are all independent variables. KDA binds the variables of the low-rank term, like `b = k ⊙ α`, to the key vector. As the number of independent variables decreases, the number of matrix operations required for chunk expansion significantly reduces, while also maintaining consistency with the classical delta rule form. Measurement results show the kernel is approximately 2 times faster compared to general DPLR.

> ⊙ Hadamard Product: An operation where, given two matrices of the same size, elements in the same position are multiplied one-to-one, without following matrix multiplication rules. \(\text{Diag}()\) is an operation that converts a vector into a diagonal matrix.

**Numerical Stability Handling.** When channel-wise alpha is included, damping factors are continuously multiplied during chunk expansion. If not normalized based on the chunk's starting point, values will decay to zero in long chunks. This handling is a core challenge in implementation difficulty.

> On modern GPUs, the throughput of Tensor Core matrix multiplications and other operations differs by orders of magnitude. When designing a new sequence model, the question to ask is not "how many FLOPs?" but "can this operation be expressed as a matrix multiplication?" If it cannot, no matter how good the theoretical complexity, it will be slow. The transition from Mamba1 to Mamba2 also reflects this lesson.

Moonshot has even released a CUTLASS implementation of this kernel under the name FlashKDA.

<br>

## AttnRes - Re-architecting the Depth Axis

### What Happens When Layers Increase

The above content covers the horizontal axis. Now that the cost of handling long contexts has been resolved, we can scale up the model.

Increasing the depth makes K3 approximately 93 layers deep.

Each layer in a Transformer adds its own computation result to the value passed up from the layer below.

If you imagine 93 layers sequentially adding their results to the same value, a problem becomes apparent.

```
Layer 1 adds, Layer 2 adds, ... Layer 93 adds

-> Values grow larger and get mixed with all sorts of information
-> The signal from Layer 3 becomes increasingly insignificant in the overall sum
-> By Layer 90, it's impossible to discern what Layer 3 contributed
-> The signal propagating back to earlier layers during training also weakens.
```

This isn't an issue with 20-30 layers, but it becomes a problem with 90+ layers.

### Selecting Instead of Adding

K3 replaced this addition with attention, calling it Block AttnRes.

Attention's original purpose is to select what's needed from multiple options, assign weights, and blend them.

This is applied not to words in a sentence, but to the **outputs of preceding layers.**

```
What each layer does

1. Creates a query vector representing "what do I need right now?" (determined by training)

2. Lays out candidates: up to 8 outputs from previous blocks + current block's value = up to 9 total

3. Scores the query against each candidate and applies softmax

4. Receives the blended value (according to scores) as its input.
```

Traditional residuals accumulate contributions from all layers with equal weight. AttnRes selects only what's needed for the current layer.

The deeper the network, the greater the advantage of this selection approach.

The vLLM team compares this implementation to FlashAttention, but it's a much smaller problem since there are only up to 9 candidates.

### It's Not Free

Each layer must continuously read previous outputs, and the residual becomes a value that must be carried along, not just passed through.

This consumes more memory and computation.

And it's slow without a dedicated kernel. A naive implementation would launch separate kernels for reading, writing, merging, and normalization for each of the 93 layers.

Even if the overhead per layer is small, multiplying it by 93 makes it significant.

vLLM combined these into a single operation.

The GPU partitioning method also had to change. As residuals became heavier, the cost of each GPU redundantly holding them increased.

Sequence parallelism, which will be discussed later, was introduced because of this.

> Around the same time, DeepSeek-V4 also revamped its residual connections under the name mHC. The fact that two areas, which had been fixed for 8 years, were overhauled simultaneously is a signal in itself. If you're wondering where to invest, this area is likely the most undervalued right now.

<br>

## Stable Latent MoE - Problems when scaling width

When the number of experts reaches 896, width becomes the problem after depth. K3 increases the number of experts to 896 and activates only 16 shared experts. This is about 1.8 percent of the total.

The rationale for this increase wasn't intuition but measurement. When creating K2, we fixed the number of active experts at 8 and varied the total number of experts. Performance consistently improved with more experts, and no limitations were observed within the experimental range.

The problem is that increasing them causes issues in three areas.

1.  **Expert weight reading time:** As mentioned before, decoding is tied to the time it takes to fetch data from memory, and the same applies here. For each token, the weights of the selected expert must be read from memory, and if the expert is large, this reading accounts for most of the time.
2.  **Inter-GPU communication:** Since 896 experts don't fit on a single GPU, they are distributed across multiple GPUs. This means that for each layer, tokens are sent to their respective GPUs, and results are gathered back, leading to two communication steps. If there are 16 active experts, the communication overhead increases proportionally.
3.  **Workload imbalance:** With as many as 896 experts, some experts become overloaded while others are not called at all. Uncalled experts don't get trained, and the GPU hosting overloaded experts becomes a bottleneck.

### Sending Summaries to Experts

LatentMoE, a structure proposed by NVIDIA, solves the first two problems simultaneously.

```
기존   : 토큰 값을 모델 폭 그대로 전문가에게 넘김
        → 전문가가 계산
        → 결과를 모델 폭 그대로 받음

Latent : 토큰 값을 좁은 차원으로 줄여서 넘김
        → 전문가가 좁은 공간에서 계산
        → 결과를 다시 모델 폭으로 늘림
```

As the vectors handled by experts become narrower, the expert weight matrices also shrink, and the data transferred between GPU cycles decreases. This allows more experts to be activated for the same cost. This is the rationale behind K3 activating 16 experts.

### Preventing Workload Imbalance

The third problem, workload imbalance, needs to be addressed separately.

The traditional method involves adding a penalty to the loss function:

If only specific experts are continuously called, a penalty is applied, and the strength of this penalty is manually determined.

This value is tricky: if set too high, the router sends tokens indiscriminately, preventing specialization.

If set too low, it fails to prevent imbalance.

This value needs to be re-tuned when the model scale changes.

**Quantile Balancing** completely eliminates this value. It ranks experts based on the scores assigned by the router,

and directly determines where to assign tokens from that ranking. The need for a manually set value disappears.

> It's better to eliminate sensitive hyperparameters than to tune them well. If you can directly derive the target from the statistics of the value you're trying to control, do so. This is a recurring pattern from batch normalization to recent research.

### Additional Components

**SiTU** was introduced as an activation function to control activation magnitudes and stabilize ultra-large-scale training, but it created a debt on the serving side.

It includes sigmoid and tanh components, causing derivative values to approach zero when inputs are very large or small, leading to vanishing gradients and high computational costs. This is due to the numerous exponential (exp) operations involving the natural constant.

In vLLM, the MXFP4 optimization path did not support SiTU, falling back to a slower implementation, which required separate workarounds. AMD also had to integrate SiTU into its kernel stack.

**Gated MLA** is responsible for the entire attention mechanism in the hybrid architecture.

It's an MLA with an added gate, and this gate's computation has the property of being able to run concurrently with the main attention computation.

This property is used in kernel optimization, as will be discussed later.

<br>

## Precision - Incorporating Compression into Training

2.8 trillion parameters, when stored in 16-bit, compress to 5.6TB. This is a necessary step.

Typically, training is completed with full precision, and then parameters are reduced to 4-bit only for deployment. However, this means the model is exposed to conditions it has never encountered during training, naturally leading to a drop in accuracy.

QAT (Quantization Aware Training) simulates low-precision operations during training. While weights are stored in high precision, they are reduced to 4-bit for computation, and the expanded value is used. This allows the model to adapt to compression errors during training.

> QAT, or Quantization Aware Training, is a model lightweighting technique that simulates low-precision quantization errors from the training phase. PTQ (Post-Training Quantization) is performed after training.

```
K2 Thinking : 사후학습 단계에서 MoE 부분에 INT4 가중치 압축
              → 생성 속도 약 2배, 모델 크기 약 594GB

K3          : SFT 단계부터 적용
              가중치 MXFP4 + 활성값 MXFP8
```

**Reducing activations is much more difficult.** While weights are fixed once training is complete, activations vary with each input, and occasionally very large values appear. A single large value can crush the rest. The microscaling format, which groups values and applies a separate scale to each group, addresses this problem.

There's one practical point worth noting: **K2 Training measured all its published benchmarks in INT4.** Typically, scores are reported with high precision, and then the service is compressed. This leads to a discrepancy between published scores and actual performance in use. When reviewing vendor data, it's important to check what precision was used for measurement, but most vendors don't disclose it at all.

<br>

## Muon

**Preventing Training Collapse**

In the Kimi lineage, it's worth noting that K2 first created a separate 16B model called Moonlight before building their main model, just to verify that the new optimizer, Muon, would work even with large models.

They did not gamble on running a 1-trillion-parameter model with a new optimizer without that verification.

### Muon vs AdamW

AdamW views weights as a collection of individual numbers. It keeps separate records for each number and adjusts the learning rate for each. It effectively ignores the fact that weights are inherently matrices.

Muon treats weights as entire matrices.

To understand the difference, if training runs for a long time, updates tend to heavily concentrate in only a few directions. Specific directions continue to change significantly, while others barely change at all. This means that while a matrix can represent a vast space, only a small part of it is being utilized.

**Muon normalizes the magnitude of all directions before applying updates.**

It suppresses directions that were changing too much and amplifies those that were changing too little, leading to more uniform training across the entire matrix.

Precisely performing this "magnitude normalization" calculation requires heavy computation, but it's approximated using a method called Newton-Schulz. This involves only a few matrix multiplications, making it fast on GPUs and allowing it to run even with reduced precision.

#### Application Targets

- **Matrix-shaped weights (hidden layers)** -> Muon
- **Linearly arranged values (embeddings, biases)** -> Retain existing AdamW

This is because the concept of aligning directions only applies to matrices.

Two things needed when scaling up large models

Muon was originally an optimizer validated on smaller models, but as Moonshot scaled it up to large-scale training, two issues were identified.

1.  **Weights needed to be prevented from growing indefinitely.** Muon originally lacked this mechanism, but when large models were trained for extended periods, weight magnitudes continuously increased. A damping mechanism was required for stable convergence.
2.  **Update magnitudes needed to be aligned across parameters.** After the process of equalizing directions, the actual update magnitudes varied depending on the matrix shape. Scaling was adjusted for each parameter to unify their magnitudes.

The result was an **approximately 2x improvement in computational efficiency compared to AdamW**. The same performance could be achieved with half the computation.

There was also a tricky point in the implementation: Muon needs to see the entire weight matrix to equalize directions. However, large models often split parameters across multiple GPUs, and naively looking only at each GPU's fragment breaks the algorithm. Moonshot also released a distributed implementation that solves this.

### QK-Clip

Even with a good optimizer, training can still collapse.

A typical failure is a loss spike, and a major cause is runaway attention scores.

Attention calculates scores by multiplying queries and keys, and as training progresses, these scores continuously grow.

K2 team observed scores exceeding 1000 in medium-scale training.

```
점수가 커진다
  → softmax가 한 곳에만 거의 100%를 몰아준다
  → 나머지 위치는 학습 신호를 못 받는다
  → 학습이 멈추거나 숫자가 터진다
```

The typical response is a post-hoc measure: if it explodes, roll back to a checkpoint from a few days ago, lower the learning rate, and restart training.

This carries the super risk of losing several days of computation on thousands of GPUs.

**QK-Clip** pre-calculates how much the scores will grow immediately after each update.

If it seems likely to exceed a threshold (100 in K2's case), it directly reduces the relevant weights on the spot. Instead of indirectly inducing this by adding a penalty to the loss function, it directly modifies the weights.

MuonClip is the combination of Muon and QK-Clip, and with this combination, there were no loss spikes during the training of 15.5 trillion tokens.

At the 1-trillion-parameter scale, this result is considered the most valuable for reproduction in the K2 report.

> Training instability is generally summarized as some value continuously growing out of control. If the stopping point is placed in the loss function, it's indirect and slow; if it's placed directly on the weights, it's definitive. When designing new architectures, the first step is to ask, "What could grow indefinitely here?" and then check if an upper bound can be applied at that point.

### Per-Head Muon

K3 applies Muon separately to each attention head.

This is a natural extension, as Muon's premise is "weights are matrices, so they should be treated as matrices."

The weights of multi-head attention are actually formed by concatenating several independent matrices side-by-side. Processing the concatenated whole blurs the boundaries between heads.

By processing them head by head, each head is managed independently.

> The unit at which an optimizer is applied is also a design variable. There's no reason why one optimizer per model should be optimal; the approach is to align the structural boundaries of parameters with the application boundaries of the optimizer.

<br>

## Serving

This is a detailed and publicly disclosed area from the Kimi documentation, and it will be useful for practitioners.

Let's explore **how the decision to use KDA created specific problems in serving and how they were resolved.**

### Mooncake

First, the Kimi service runs on its own serving platform called Mooncake.

Its paper and code were released in 2024, and it now processes over 100 billion tokens per day across thousands of nodes.

#### Separation of Prefill and Decode

Prefill is the stage where the prompt is read all at once, making computation the bottleneck. Decode is the stage where tokens are emitted one by one, making memory reads the bottleneck.

Mixing tasks with different characteristics on the same server is detrimental to both, so the clusters are separated and configured appropriately for each.

#### Idle Resources as a Cache Store

GPU clusters have idle CPUs, DRAM, and SSDs, which are all bundled together and used as a KV cache store.

Parts that everyone uses, like system prompts, are copied to multiple locations.

#### Conductor

A scheduler called Conductor assigns requests.

It simultaneously checks where the cache is located and how busy each node is.

If it only considers the cache, requests will flock to a specific node.

If it only considers load, the cache cannot be utilized, so balancing these is the essence of the scheduler.

#### Early Rejection

If requests that cannot be completed on time come in, they are rejected proactively based on prediction, without even being accepted.

Most serving research assumes that all requests are processed, but this design stems from the perspective that real services are not like that.

One particular optimization is impressive: the tasks of loading and storing the cache are overlapped with prefill computations on a layer-by-layer basis.

This hides the transfer time behind the computation.

#### Breaking Prefix Caching

Prefix caching is currently the biggest cost-saving measure in serving.

If multiple requests share the beginning of a prompt, that part of the computation is reused.

Earlier, we pointed out that a characteristic of agent workloads is that the next turn sends the previous turn's prompt exactly as it was.

This characteristic dominates costs, with the official Kimi API reporting over 90% hit rate in coding tasks, and prices showing roughly a 10x difference: $0.30 for a cache hit vs. $3.00 for a miss.

However, using KDA makes this fundamentally difficult.

```
전체 어텐션 : 앞부분이 토큰별 K, V 벡터로 남아 있음
             → 블록 단위로 이름표 붙여놓고 통째로 재사용하면 끝

KDA        : 앞부분이 "그 지점의 상태"로만 남아 있음
             → 정확히 그 위치의 상태를 저장해뒀어야 함
             → 앞에서부터 다시 돌리면 캐싱하는 의미가 없음
             → 그렇다고 매 토큰 저장하면 공간이 즉시 터짐
```

The KDA state and the size of one layer are comparable to thousands of tokens worth of KV cache; the state is not small.

Its advantage is that it's fixed regardless of context length, not that it's absolutely small.

A practical implication is that for services where most requests are short, there might be a range where a hybrid approach is actually detrimental.

It's necessary to calculate from what length it becomes beneficial for one's own workload.

#### Separation of Block Size and Caching Unit

There was a problem with the existing vLLM.

vLLM manages the KV cache in block units.

A block is a fixed-size slot that holds a few tokens' worth of cache.

The problem was that this block size served two roles simultaneously:
- The unit for actual memory allocation
- The unit for comparing if the beginning parts of requests are the same

Since the KDA state (summary) is heavy, the block size needs to be set to thousands of tokens.

However, if it's set too large, two requests sharing 99% of a prompt might still suffer a cache miss because their common part doesn't fill a whole block.

Storage is efficient, but reuse becomes impossible.

The solution was to separate these two: blocks are kept large, but summaries can be registered **even at fine-grained positions within a block**.

If a later request matches that position, the summary is copied to its own location, and processing continues. Since it's copied and used, the original remains, and other requests can continue to use it.

Moonshot didn't just solve this problem for its own service but contributed the implementation to the vLLM core.

As a result, this infrastructure can now be used by all models with similar structures.

#### Summary Storage Location

Even with caching enabled, where to store the summary remains a policy issue.

There are two implementations:

1.  Storing at regular intervals. For example, every 32K tokens.

This is controlled by `VLLM_PREFIX_CACHE_RETENTION_INTERVAL`.

A better point is the end of the prompt. Since an agent's next turn sends the previous turn's prompt exactly as it was, the summary here has a very high probability of being reused. vLLM automatically finds and always stores at this position.

2.  Storing when seen. This is a method called Marconi, and its rule is simple:

```
요청 1 : 자기 프롬프트 끝에만 저장. 공유 지점에는 안 남김
요청 2 : KV 캐시는 맞았는데 KDA 요약본이 없어서 미스
        → 이 미스가 "이 앞부분은 실제로 공유되고 있다"는 증거
        → 이제 그 자리에 저장
요청 3 : 재사용
```

The first time it's seen, it's merely information that such a prefix exists. **Only when it's seen a second time is it deemed evidence of being shared, and then the storage space is used.**

This prevents prefixes used only once from evicting cache entries, while frequently repeated ones are automatically promoted. It eliminates the need for humans to predict which parts will be used often.

### Kernel Optimization

K3 fundamentally changed the heavy parts.

Just optimizing the attention kernel is not enough.

Looking at the actual points of optimization reveals where the bottlenecks shifted.

-   **Bundling KDA Layers:** A single KDA layer involves multiple computations. Running them separately incurs the cost of queuing tasks on the GPU every time. With a 3:1 batch, there are many KDA layers, so a small loss per layer directly translates to latency. All were bundled into a single kernel.
-   **Attention Metadata Builder:** Before launching a kernel, index information must be passed to the GPU. This includes where each sequence in the batch starts and ends, where cache blocks are located, and which slots to read from and write to for KDA summaries. This is called attention metadata. The initial K3 implementation used a generic builder for Gated DeltaNet, which was creating items K3 didn't even use, and assembling them by chaining small PyTorch operations. Creating a dedicated K3 builder and combining those operations into a single Triton kernel reduced the time from 70μs to 34μs for batch 1 (a 96% reduction). The bottleneck was not the attention computation itself, but the metadata assembly preceding it. The smaller the batch, the larger the proportion of such fixed overhead.
-   **Dedicated Code for Small Matrix Multiplications:** For small batches, dedicated matrix multiplication units have a preparation phase before use. If the matrix is large, the preparation cost (attention metadata build) pays off, but for small matrices, time is spent mostly on preparation. This resulted in an 8-100% improvement per kernel.

> There was talk that matrix multiplication should be used for speed, but here, dedicated units are avoided. The idea is that processing large blocks and very small ones differently can be more efficient.

**Sequence parallelism** also comes into play here. Due to AttnRes, the residuals became heavy, making it expensive for all GPUs to redundantly hold the entire data. This was changed so that each GPU only holds and processes its own share.

A lesson learned here is that while the theoretical communication volume decreased, NVIDIA's communication library was not optimized for messages of this size, so no actual benefit was observed. Therefore, a custom communication kernel was written, which was 1.7 to 4.5 times faster than the existing one.

> It's common for theoretically calculated communication volumes not to predict actual measurements. Communication libraries are optimized for specific size ranges, and outside those ranges, theoretically cheaper methods can be slower. It needs to be measured.

### Dspark Speculative Decoding

Even with kernel and communication optimizations, a fundamental constraint remains: to generate a single token, all 2.8 trillion active weights must be read. Decoding computation is bound by these memory reads.

Speculative decoding is a technique that allows processing multiple tokens with a single read.

A lightweight, fast draft model proposes several next tokens in advance, and the main model confirms them.

The fast draft model proposes several next tokens in advance, and the main model validates all of them in a single forward pass.

It confirms up to where it's correct and restarts from the point of error, resulting in the same output as if the main model generated it alone (lossless).

The reason this is beneficial is that **the cost of reading weights once is the same whether generating 1 token or validating 7 tokens.** It utilizes idle computational resources to confirm multiple tokens.

DSpark is a separate draft model attached by K3.

#### Why a separate model instead of MTP?

The standard method for creating drafts is **MTP (Multi Token Prediction)**.

During training, the model is instructed to predict not just one next token but several simultaneously. The prediction heads attached at that time are then repurposed as a drafter during inference.

Qwen3.5 series uses this method, and its advantage is that it doesn't require creating a separate model.

The problem is that MTP heads are **autoregressive drafters**; they must generate one token before generating the next based on it.

```
자기회귀 드래프터로 7개를 만들려면
  → 7번 돌아야 함
  → 드래프트 개수를 늘릴수록 드래프팅 비용도 같이 늘어남
  → 개수를 늘려서 얻는 이득이 금방 상쇄됨
```

K3 attached a separate draft model, DSpark, which incorporates three features:

-   **Block Diffusion Backbone:** Generates multiple tokens **all at once in a single parallel pass** instead of sequentially. This applies the same idea as diffusion models generating images in one go to token blocks. As a result, drafting costs remain almost flat even with deeper blocks, directly addressing the limitations of autoregressive methods.
-   **Low-Rank Markov Head:** When tokens are generated all at once, tokens within a block don't know each other. Each might seem plausible, but when concatenated, they can form grammatically incorrect candidates. This head provides internal forward and backward dependencies within the block to improve candidate quality.
-   **Confidence Head:** Predicts the probability that each draft will pass validation in advance. By filtering out hopeless candidates at the scheduling stage, computational resources used for validation can be saved. (This is still under development in vLLM as of now).

The draft was made **MLA native**. The main model uses Gated MLA in its hybrid full attention layers, and the draft model was aligned to have the same attention structure and KV cache layout.

This is to ensure that both the draft and main models operate within the same cache management system in the prefix caching and prefill/decode separation environment created earlier.

The recognition is that **creating a draft model haphazardly leads to problems in serving stack integration.**

```
GB300 NVL72, 배치 1 기준

TP8   : 111 tok/s  →  331 tok/s
TP16  : 118 tok/s  →  370 tok/s   (약 3.14배)
```

The benefit of speculative decoding is determined by the **acceptance rate**.

This is the percentage of tokens proposed by the draft that pass validation by the main model, usually measured as "average tokens confirmed per step."

If this value is close to 1, it means the draft was consistently wrong, yielding no benefit and only incurring additional drafting costs.

The values measured in K3 are:

```
코딩처럼 다음에 뭐가 올지 뻔한 작업  : 스텝당 약 4.73 토큰 수용
창작처럼 예측하기 어려운 작업        : 스텝당 약 2.61 토큰 수용
```

**The more predictable the task, the better the draft performs.** Code has defined syntax and idioms, making it easier to predict, while creative tasks do not. The value of speculative decoding varies greatly depending on whether one's workload falls into the former or latter category.

There's one unresolved issue here: if the draft is wrong in speculative decoding, it must be rolled back. For KV cache, simply truncating the end is sufficient, but KDA summaries are overwritten in place, so they cannot be truncated.

vLLM explicitly states that it uses a different kernel path for speculative decoding, which seems related to this constraint.

<br>

## Deployment

#### Execution Command

```bash
vllm serve moonshotai/Kimi-K3 \
  --tensor-parallel-size 8 \
  --trust-remote-code \
  --load-format fastsafetensors \
  --enable-prefix-caching \
  --enable-auto-tool-choice \
  --tool-call-parser kimi_k3 \
  --reasoning-parser kimi_k3
```

- `--tool-call-parser kimi_k3`: Converts the tool call format generated by the model to OpenAI format. Without it, the model generates tool calls correctly, but the harness doesn't recognize them. This often manifests as the agent not using tools, which can be easily mistaken for a model performance issue.
- `--reasoning-parser kimi_k3`: Separates reasoning tokens from the final response. Without it, the reasoning process is mixed into the answer.
- `--enable-prefix-caching`: **This is off by default for K3.** The hybrid cache design is constantly changing, so it must be explicitly enabled. Considering the 10x price difference seen earlier, running without it is an expensive mistake.
- `--load-format fastsafetensors`: The time to load 2.8 trillion weights is not negligible.

#### Enable Speculative Decoding

```bash
--speculative-config '{"model":"Inferact/Kimi-K3-DSpark","method":"dspark",
  "num_speculative_tokens":7,"attention_backend":"FLASHINFER_MLA",
  "draft_sample_method":"probabilistic","rejection_sample_method":"block"}'
```

Choose the communication backend according to the GPU connection method.

```
GPU 간 통신
  NVLink로 묶여 있으면   → flashinfer_nvlink_one_sided
  네트워크로 묶여 있으면 → deepep_v2

MoE
  TP > 1     → flashinfer_trtllm
  전문가 병렬 → deep_gemm_mega_moe
```

> A harness is a wrapper that encloses a model, provides tools, allows it to read files, and execute commands. Examples include Kimi Code, Claude Code, and Cline. Even with the same model, performance can vary depending on the harness due to differences in system prompts, tool definition formats, and context management methods.

### Checklist, Precautions

**Tool calls must be validated with your own traffic.** This is an item directly warned by the vLLM team. It has been observed that K3 sometimes outputs tool calls in a format the parser doesn't expect, resulting in empty responses. Simple tests might work well in the same environment, but it varies with prompts and execution, making reproduction difficult and more dangerous. Mechanisms should be in place to detect empty results and retry or route to another path.

**All reasoning history must be returned.** K3 was trained to continuously carry forward its previous thoughts. If the harness doesn't pass all past reasoning content, or if a conversation in progress with another model is switched to K3 midway, quality becomes highly unstable. The official documentation recommends against mid-session model switching.

**Excessive autonomy must be curbed with prompts.** K3 was trained primarily on long-running tasks, so when it encounters minor issues or ambiguous instructions, it tends to make decisions on its own without asking the user. For applications where boundaries must be maintained, constraints should be specified in the system prompt or AGENT.md.

**When evaluating, first suspect truncation.** K3 thinks a lot before answering, so if benchmark scores are low, it's often not because it's wrong, but because generation was cut off midway. Increase the reasoning budget, set `max_tokens` generously, and first check if it was truncated.

**The vision encoder is simply replicated.** The K3 vision encoder has a head size of 12, which is evenly divided among 8 GPUs. Since the encoder is less than 1B and the main body is about 2T, it's judged better to copy it to each GPU rather than forcibly dividing it and incurring communication costs.

### Why Chat Templates Are Programs

There's one small but useful decision.

Most models first convert requests into text, then tokenize them.

K3 directly assembles token sequences as programs.

The reason is control tokens. To distinguish system messages, user messages, tool definitions, and tool results, special markers are needed. If processed via text, the boundaries break down when the user's input contains the exact same string as a marker.

From a prompt injection perspective, this design is safer because user text and control markers do not meet in the same space.

<br>

## Agent Swarm and PARL

When agents call tools sequentially, time accumulates with each step.

The kernel optimization and speculative decoding discussed earlier only increase token generation speed; they do not reduce the **tool call round trip itself**.

The same applies even if you scale up the model.

While parallel calls are possible, manually pre-defining sub-agent roles and sequences lacks scalability.

### How It Works

**PARL (Parallel-Agent Reinforcement Learning)** gives the model the ability to create sub-agents and distribute tasks, teaching it how to use this functionality through reinforcement learning.

```
Task Arrives
  ↓
Orchestrator (the trained component)
  ├─→ Sub-agent 1 (simultaneously) → Tool A, B
  ├─→ Sub-agent 2 (simultaneously) → Tool C, D
  ├─→ Sub-agent 3 (simultaneously) → Tool E, F
  └─→ Aggregate Results
```

There are two important design choices, both concerning what *not* to train.

**Sub-agents are kept fixed without training, and only the orchestration is trained.**

This separation avoids two problems simultaneously.

1.  Credit assignment: When results are good, it's hard to distinguish whether it's due to the orchestrator's effective task distribution or the sub-agents' successful execution. This separation helps distinguish between them.
2.  The other is learning instability: if ten sides change simultaneously, the environment constantly shifts for each other, preventing learning from converging.

**It does not assume parallelism is always good.** Some tasks require later steps to receive results from earlier steps before proceeding.

You need to find a file before you can modify it, and you need to run tests to determine what to fix. Forcibly splitting such tasks can lead to sub-agents performing contradictory work or wasted effort.

Therefore, PARL doesn't hardcode rules for splitting; instead, it learns *when*, *what*, and *how* to split tasks through result feedback.

The problem is that learning this judgment is difficult. Sequential processing, while slow, reliably yields an answer.

The benefits of parallelization **only appear much later as a final result, and only occasionally.**

As a result, learning tends to gravitate towards the safer option. While this can be an advantage, being *too* safe in judgment is also a problem.

The result is serial collapse: a phenomenon where, even in situations where parallel calls are possible, the orchestrator gives up on making a judgment and simply calls tasks one by one sequentially.

```
Normal: This task must be sequential, so it proceeds sequentially -> Result of judgment
Failure: No judgment is made, always proceeds sequentially -> serial collapse
```

**The problem is not using the parallel functionality when given,** not sequential execution itself.

What PARL solves is how to train this judgment capability.

```
K2.5 : Max 100 sub-agents, Max 1500 orchestration steps, Time reduced by up to 4.5x
K2.6 : 300 sub-agents, 4000 steps, over 12 hours of continuous autonomous execution (as reported)
```

#### Serendipitous Discovery

An interesting report from the K2.5 paper noted that when reinforcement learning was performed with image data, text performance also improved.

MMLU-Pro / GPQA-Diamond were improved.

This is explained as mutual reinforcement, where, contrary to the common belief that multimodal input sacrifices text performance, it enhances text perception and refines text again.

### Differences from Other Families

**Branches for Solving Long Contexts**

| Branch | Key Strategy | Representative Models | Pros / Cons |
|---|---|---|---|
| Exactly as Is | Full Attention | GPT, Claude families | Highest accuracy / Memory and computational burden with context length |
| Compress | KV Compression + Selective Attention | DeepSeek-V2~V4 | Memory savings / Potential information loss due to compression |
| Transform | State Space / Linear Attention | Mamba2, RWKV, GLA | Efficient for long sequences / Potential degradation in precise retrieval capability |
| Mix | Hybrid Attention | **Kimi K3**, Qwen3.5 | Trade-off between efficiency and accuracy / Increased design and serving complexity |

This is why linear attention is being evaluated as having 'graduated' from research curiosity to frontier production.

#### Kimi vs DeepSeek: Same Problem, Different Answers

Both aim to create 1M contexts cheaply, but their premises are opposite.

| Comparison Item | Kimi K3 | DeepSeek-V4 |
|---|---|---|
| Attention | Many KDA (linear) + Few Gated MLA | Alternating Compressed/Sparse Attention |
| Philosophy | Mostly abandons attention | Retains attention but compresses it |
| Depth Axis | AttnRes | mHC |
| Optimizer | Per-Head Muon | Muon |
| Scale | 2.8T / 16 active out of 896 | 1.6T / 49B active |
| 1M Efficiency | Decoding ~6x, KV 75% reduction | Computation 27%, KV 10% |

1.  1M context is only economical if the attention structure is changed.
2.  To stack layers very deeply, residual connections must be modified.
3.  Muon and 4-bit QAT are becoming defaults for large-scale training.

There are also diverging points: on the sequence axis, one chose transformation, the other chose compression.

- Kimi focuses on size limits and agents, with a single flagship lineup; local execution is virtually impossible. A notable feature is the public release of its serving infrastructure.
- DeepSeek focuses on efficiency and inference, with a flagship-centric lineup; local execution is difficult. Its characteristic is research primarily on attention compression.
- Qwen focuses on lineup breadth, offering a full range from 0.8T to 2.4T. It is strong in local execution and characterized by its ecosystem of derived models.

<br>

## Terminology

## Terminology

| Term | Definition and Mechanism |
|---|---|
| **Active Parameters / Total Parameters** | The amount involved in processing one token vs. the amount that needs to be loaded into memory. The former determines speed, the latter hardware requirements. |
| **MLA** | An attention mechanism that reduces KV cache by compressing keys and values into a narrow space. The main focus of the K2 predecessor generation. |
| **KDA** | Kimi Delta Attention. Fixed-size summary + delta rule (overwrite) + channel-wise forgetting. The KV cache disappears, but precise retrieval is weak, so it's used in a hybrid manner. |
| **Delta Rule** | Instead of adding new information, it corrects by the difference from the currently stored value. Prevents values from being corrupted when writing to the same item again. |
| **Channel-wise Forgetting** | Sets the forgetting rate differently for each dimension. Distinguishes between information to retain for a long time and information to discard quickly. |
| **Hybrid Attention** | A structure that mixes linear attention layers and full attention layers. Both Kimi and Qwen use a 3:1 ratio. |
| **Gated MLA** | The layer responsible for full attention in a hybrid setup. Adds a gate to MLA. |
| **AttnRes** | Instead of simply adding the results of layers, it selects and mixes necessary elements from preceding layers. Prevents signals from being buried in ultra-deep layers. |
| **LatentMoE** | Performs expert computations in a narrow space to reduce weight reading and inter-GPU communication. |
| **Quantile Balancing** | Determines expert assignment by ranking router scores. Removes sensitive hyperparameters. |
| **MuonClip** | Muon optimizer + QK-Clip. Preemptively blocks attention score explosion to prevent loss spikes in ultra-large-scale training. |
| **QK-Clip** | Pre-checks if scores will increase immediately after an update, and if they are likely to exceed a threshold, directly reduces weights. |
| **Per-Head Muon** | Applies Muon separately for each attention head. |
| **SiTU** | An activation function that controls the magnitude of values. |
| **QAT** | Simulates low-precision operations during training to allow the model to adapt to compression errors in advance. Results in less loss than post-training compression. |
| **MXFP4 / MXFP8** | Low-precision formats that divide values into bundles and apply a scale factor to each bundle. Supported by modern GPUs in hardware. |
| **Attention Metadata** | Index information passed to the GPU before launching an attention kernel. Includes sequence range for each request, cache block location, summary slots, etc. If the batch is small, this assembly cost can be greater than the kernel execution time. |
| **Prefix Caching** | Reuses computations for the shared prefix of prompts across multiple requests. Dominates costs in agent workloads. |
| **P/D Separation** | Handles prefill (computation bottleneck) and decode (bandwidth bottleneck) on different servers. |
| **Mooncake** | Kimi's serving platform. P/D separation + cache store built from idle resources + global scheduler. |
| **Marconi Method** | Caches a prefix when it is observed for the second time. Prevents one-time uses from evicting cache entries. |
| **Speculative Decoding** | A lightweight draft model proposes multiple tokens, and the main model validates them with a single forward pass. The result is lossless. Leverages the fact that the cost of reading weights once is the same whether generating 1 token or validating 7. |
| **MTP (Multi-Token Prediction)** | During training, it predicts multiple next tokens simultaneously and reuses those heads as a drafter during inference. No separate model is needed, but since it's autoregressive, increasing the number also increases drafting costs. |
| **Block Diffusion** | A method that generates tokens in blocks all at once, rather than sequentially. Costs remain flat even if the number of drafts increases. |
| **Acceptance Rate** | The proportion of draft proposals that pass main model validation. Expressed as the average number of confirmed tokens per step. Higher for more predictable tasks. |
| **DSpark** | Draft model for K3. Block diffusion + low-rank Markov head + confidence head. Speeds up single-stream decoding by approximately 3x. |
| **PARL** | A method where an orchestrator is taught via reinforcement learning to distribute tasks to sub-agents. Sub-agents are fixed. |
| **Serial Collapse** | A training failure where the system gives up on determining if a situation can be parallelized and always processes sequentially. Sequential processing is normal for tasks where it's appropriate; the problem is not making the determination. |
| **Harness** | A shell that provides a model with access to tools, files, and command execution. The same model can have different performance depending on the harness. |
| **Scaffolding** | The prompt, tool, and loop structure that wraps agent tasks. Significantly influences benchmark scores. |
