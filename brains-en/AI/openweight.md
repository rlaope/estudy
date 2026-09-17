# Archaeology of OpenWeight Models (Kimi, GLM, Qwen, DeepSeek)

This note aims to understand the implementation techniques and strengths of Qwen, GLM, DeepSeek, and Kimi from scratch.
The reference point is early August 2026, and concepts, principles, and figures may change over time.

### Before We Begin

Before we start, let's lay out a map of the models we'll explore, just to get a feel for them.

- **DeepSeek:** Developed by DeepSeek in Hangzhou, part of the HuanfangQuant family, a master of efficiency. They first prove it with papers and then confirm it with models. DeepSeek-V4-Pro (1.6T/49B), V4-Flash (284B/13B)
- **Qwen**: Developed by Alibaba Cloud, available in all sizes from 0.8B to 400B. Qwen3.6-27B (dense), Qwen3.6-35B-A3B
- **GLM**: Developed by Z.ai, specialized in agent coding with good cost-effectiveness. GLM-5.2 (744B/40B)
- **Kimi**: Moonshot AI, a challenger at the forefront of scale, capable of long contexts and autonomous execution. Kimi K3 (2.8T/104B)

```
2024
 ├─ DeepSeek-V2 ······· MLA + DeepSeekMoE 최초 도입 (236B/21B)
 └─ DeepSeek-V3 ······· 671B/37B, 보조손실 없는 로드밸런싱, MTP, FP8 훈련
                        (H800 278만 시간 = 프론티어 대비 극단적 저비용)
2025
 ├─ 1월  DeepSeek-R1 ··· GRPO 기반 순수 RL 추론. 세계적 충격
 ├─ 5월  Qwen3 ········· 0.6B~235B 전 라인업, thinking/non-thinking 통합
 ├─ 7월  Kimi K2 ······· 1T/32B, MuonClip으로 15.5T 토큰 무사고 학습
 ├─ 7월  GLM-4.5 ······· deep-over-wide MoE, slime RL 인프라 공개
 ├─ 9월  Qwen3-Next ···· Gated DeltaNet 3:1 하이브리드 (선형 어텐션 본격화)
 └─ 하반기 DeepSeek-V3.2 ·· DSA(희소 어텐션) + 라이트닝 인덱서
2026 ★ 세대 교체의 해
 ├─ 1월  Kimi K2.5 ····· 네이티브 멀티모달 + Agent Swarm
 ├─ 1월  DeepSeek 논문 3연발 ·· mHC, Engram, DSA (V4의 설계도 공개)
 ├─ 2월  GLM-5 ········· 744B/40B, DSA 채택, 28.5T 토큰
 ├─ 2월  Qwen3.5 ······· 397B-A17B, 얼리퓨전 멀티모달, 201개 언어
 ├─ 4월  Qwen3.6 ······· 27B dense가 397B 플래그십을 코딩에서 추월
 ├─ 4월  Kimi K2.6 ····· 300 서브에이전트, 12시간 자율 실행
 ├─ 4월  DeepSeek-V4 ··· mHC + Engram + 하이브리드 희소 어텐션, 1M 컨텍스트
 ├─ 6월  GLM-5.2 ······· IndexShare로 1M 컨텍스트 실용화
 └─ 7월  Kimi K3 ······· 2.8T, 세계 첫 오픈 3T급. KDA + AttnRes + LatentMoE
```

<br>

## MoE

It's called Mixture of Experts.

Making models larger makes them smarter. However, making them larger also makes them slower and more expensive. Is it possible to create a model that "knows a lot but doesn't mobilize everything every time"?

The solution is to split the FFN (feed-forward layer) within the transformer block into multiple experts, and a router decides for each token, "Let's send this token to experts 3, 17, and 44."

```
       [토큰]
          ↓
      [라우터] ── 점수 계산 → 상위 k개 선택
          ↓
   ┌──┬──┬──┬──┬─ ... ─┬──┐   256개 전문가
   │  │██│  │██│       │  │   (██ = 이번에 켜진 것)
   └──┴──┴──┴──┴───────┴──┘
          ↓
      [결과 합산]
```

### Diverging Points

Each model has its own diverging points; while they all use MoE, how they use it differs.

1. **Fine-grained Experts**: Early MoE (e.g., Mixtral) activated 2 out of 8 large experts. **DeepSeekMoE** changed this approach by breaking down experts into smaller units and activating more of them. This increases the **number of possible combinations** for the same computational load, leading to better specialization.

For example, it's the difference between calling 2 out of 8 generalist doctors versus calling 8 out of 256 highly specialized doctors. The latter can create more sophisticated combinations.

2. **Shared Expert**: Knowledge commonly needed by all tokens (grammar, general facts) is wasteful to learn by selecting an expert every time. Therefore, one expert is always kept active.

| 모델 | 총 전문가 | 활성 | 공유 전문가 |
|------|----------:|-----:|-------------|
| DeepSeek-V3 | 256 routed | 8 | 1개 있음 |
| Qwen3 MoE | 128 | 8 | 없음 (의도적 제거) |
| Qwen3.6-35B-A3B | 256 | 8 | 1개 있음 |
| GLM-5.2 | 256 | 8 | 1개 있음 |
| Kimi K2 | 384 | 8 | 있음 |
| Kimi K3 | 896 | 16 | LatentMoE 구조 |

Activating 16 out of 896 experts in Kimi K3 represents an extreme sparsity of 1.8%.

### Load Balancing - MoE's Challenge

There's a problem: as the router learns, tokens tend to flock to popular experts.
Only a few experts are continuously trained, while the rest are neglected, leading to wasted model capacity. This is called routing collapse.

The traditional solution is an auxiliary loss, which adds a penalty to the loss function if experts are not used evenly. However, this penalty conflicts with the original goal (accurate prediction), thus **degrading performance.** (If it predicts well, it won't use experts evenly... a contradiction)

Here, each model offered a different solution.

- **DeepSeek-V3:** Auxiliary-loss-free load balancing. Without modifying the loss function, a **bias** term is added to the router scores to slightly lower scores for overloaded experts and raise them for underutilized ones. This method balances without performance degradation and is one of DeepSeek's key inventions and an industry standard.
- **Qwen3:** Global batch load balancing loss: This approach balances experts not at the batch level but globally, further pushing expert specialization.
- **GLM-4.5**: Sigmoid gate + lossless balanced routing. When distributing tokens to each expert, it uses a sigmoid instead of the traditional softmax to increase flexibility through independent probability calculations. Lossless balanced routing removes the auxiliary balancing loss function, which causes performance degradation during model training, allowing for natural balancing without penalties to the loss function. In short, it doesn't use it.
- **Kimi K3**: Quantile Balancing: Directly induces expert distribution from the quantile in the router scores. The key point is the complete elimination of heuristic updates and sensitive parameters. This is because traditional methods become untunable when there are as many as 896 experts.

These techniques come with trade-offs.

- GPU memory is required proportional to the total number of parameters.
- Experts are scattered across multiple GPUs, leading to significant overhead. This is why DeepSeek V3 developed techniques like dualpipe for overlapping communication and computation.
- If the batch size is small, personal usage efficiency is low.

<br>

## Attention Optimization

To extend the context to 1 million tokens, the quadratic problem of attention must be solved (the issue of KV cache growing large).

```
"어텐션이 너무 비싸다"
              │
   ┌──────────┼──────────┐
   ↓          ↓          ↓
[압축하자]  [골라내자]  [기억으로 바꾸자]
   MLA        DSA      Linear Attention
(DeepSeek)  (DeepSeek) (Qwen/Kimi)
```

### MLA (Multi-head Latent Attetnion)

As the KV cache grew, it consumed all available memory.
K and V are compressed into **low-dimensional latent vectors**, stored, and then uncompressed only when needed.

This can be likened to keeping only summaries of meeting documents instead of the originals, and restoring them from the summary when needed. This saves desk space.

While GQA (Group Query Attention) is an approach to share heads, MLA aims to reduce the dimension itself. DeepSeek V2 first introduced it, leading to V3 and V4, and it has also been adopted by GLM-5 and Kimi K3.

> More Detail: The RoPE method is not well-suited for compression (as it represents token positions by rotation). It's good to know that MLA uses a decoupled RoPE to compress while separately preserving positional information.

### DSA (Deepseek Sparse Attention)

It starts with the question, "Do we really need to look at all 1 million tokens?" It begins with the query, "Aren't most requests irrelevant to this current sentence?"
**Before** attention calculation, a lightweight filter is run to **select the top k relevant tokens**, and then precise attention is performed only on those.

This filter is the Lightning Indexer.

```
[100만 토큰]
     ↓
[라이트닝 인덱서]  ← 가볍고 빠른 점수 계산 (FP8, ReLU 기반)
     ↓
[상위 2,048개만 선택]   ← 실제 공개 코드의 k값
     ↓
[정밀 어텐션]      ← 여기만 비쌈
```

When writing a thesis in a library, you don't read all 1 million reference books. It's like asking the librarian to pull out a few thousand books related to your topic and reading only those core ones. (Even 2,000 books seems like a lot, but the key is that the number has been reduced.)

The training method is clever. Instead of training the indexer from scratch:

1. **Warm-up**: The existing model's attention is kept as-is, and only the indexer is trained. The goal is to mimic where the existing attention looked (KL divergence loss), completing it in 1,000 steps with 128K sequences, totaling 2.1 billion tokens.
2. **Main Training:** Then, sparse selection is enabled, and the entire model adapts.

Computational complexity drops from quadratic to virtually linear. Moreover, a side effect is that it filters out **irrelevant information (distractors)**, making it robust to noise.

**The trade-off is the risk of cutting out information** that seems irrelevant now but becomes crucial 20 steps later. In extremely complex multi-step reasoning, a slight performance loss is observed.

DSA has been adopted by DeepSeek-V3.2/V4, GLM-5/5.1/5.2 (GLM leverages DeepSeek's technology — the power of the open ecosystem).

### GLM's Improvement: IndexShare

When GLM-5.2 tried to increase its context from 200k to 1M, a problem arose: the indexer itself running at every layer became expensive.
Only one indexer is run every four layers, and the top k indices from its results are used by the other three layers.

> Instead of multiple teams asking the librarian four times, they ask once and share that list among the four teams.

The effect is a **2.9x reduction** in FLOps per token for a 1M context. Furthermore, even with reduced computation, long-context benchmarks improved compared to the previous version.

### Linear Attention and Hybrid (Replaced by Recurrence)

Fundamentally, it began with the question, "Why do we need to re-examine the entire past every time?"
A method was proposed to carry a summary of the past in a state, similar to RNNs. This completely eliminates the KV cache because the state size remains constant regardless of context length.

> Instead of keeping all meeting minutes (attention), it's like continuously updating and carrying a one-page summary of the current situation. It feels like a respect for the strengths of RNNs.

A representative implementation is GDN (Gated DeltaNet), which combines four elements as a descendant of Mamba-family research.

- **Delta rule:** Updates only the difference between existing memory and new information (error-correcting memory).
- **Exponential gating:** Controls how much memory to forget (prevents saturation).
- **Causal Conv1D:** Captures local context.
- **Q/K L2 normalization:** Used instead of softmax.

**However, pure linear attention has limitations.** If you only carry a summary, you can't retrieve the exact phrase from page 300, line 42. Thus, its precise retrieval capability is diminished.

Therefore, a hybrid approach is necessary.

```
Qwen3-Next / Qwen3.5 / Qwen3.6 의 층 배치 (3:1 패턴)

[GDN][GDN][GDN][Full Attention] × 12회 반복  = 48층
 ↑ 값싼 요약        ↑ 정밀 검색 (KV 캐시 유지)
 75%                25%
```

Three layers follow the flow using an inexpensive recurrent method, and the fourth layer precisely performs full attention. This saves most of the cost while retaining retrieval capability.

In fact, there is theoretical support showing that **hybrids are more advantageous than both pure GDN and pure attention** for certain types of retrieval tasks.

### Kimi's Variation: KDA (Kimi Delta Attention)

Kimi modified GDN in two ways.

1. Qwen3-Next uses **one scalar gate per head**, whereas KDA places a gate on each **channel (feature dimension)**.
2. Instead of standard attention, it uses MLA (+Gate) in the entire attention layer, meaning it's a structure that uses **compression and recurrence simultaneously**.

Moonshot reported that KDA **speeds up decoding by up to 6.3 times** in a 1 million token context.

| 구분 | 압축 (MLA) | 선택 (DSA) | 순환 (GDN/KDA) |
|------|------------|------------|-----------------|
| 아이디어 | 저장할 것을 줄인다 | 볼 것을 줄인다 | 상태로 요약한다 |
| KV 캐시 | 대폭 축소 | 유지되나 접근 축소 | 소멸 |
| 정밀 검색 | 유지 | 대체로 유지 | 약함 → 하이브리드 필수 |
| 대표 | DeepSeek | DeepSeek, GLM | Qwen, Kimi |

This is where the characteristics of the four models diverge: Qwen and Kimi bet on recurrence + hybrid, while DeepSeek and GLM bet on compression + selection.

<br>

## MTP - Multi-Token Prediction

Traditional training is about "predicting the next single token." This has issues with weak signals and slow generation, as it's one token at a time.
Therefore, it starts with the idea of **predicting multiple tokens simultaneously** during training, rather than just one next token.

- **Training Effect:** By forcing the model to look ahead two or three words, it learns more far-sighted representations. This improves long-range consistency.
- **Inference Effect:** This MTP layer is directly used as the **draft model for speculative decoding**. There's no need to create a separate model.

> Analogy — In dictation training, if students are asked to predict "the next three characters" instead of just "the next character," they will understand sentence structure better. And a student trained this way will actually be able to write three characters at a time.
> Speculative Decoding is an acceleration technique where a small, fast model predicts multiple next tokens in advance, and a large model validates them all at once. If the prediction is correct, multiple tokens are confirmed simultaneously.

This technique has been adopted by all (Qwen, Kimi, GLM, DeepSeek). It was validated in DeepSeek-V3 (1.8x acceleration with 2-token MTP) and subsequently adopted by GLM, Qwen3-Next series, and DeepSeek-V4.

GLM 5.2 applied IndexShare to the MTP layer, and added rejection sampling and total-variation loss, increasing the **average acceptance length from 4.56 to 5.47 (+20%)**.

<br>

## Optimizer - AdamW -> Muon

In training ultra-large models, **attention logits (Q·K dot product) can explode**, causing training to fail. This is critical, leading to losses of hundreds of millions to billions of won in time and resources.

The Muon optimizer, in particular, is token-efficient but experiences this phenomenon more frequently.

This is because existing countermeasures were not effective.

- **Logit softcap:** Truncates values. However, it cannot prevent dot products from becoming excessively large before truncation.
- **QK-Norm:** A good method, but not applicable to MLA. This is because MLA does not fully unroll the K matrix during inference.

### Kimi's Solution - MuonClip, QK-Clip

Moonshot combined **QK-Clip**. The logic is simple:
If the maximum attention logit in any head exceeds a threshold τ, the Q and K weights of that head are directly scaled down.

Kimi K2 was trained with τ = 100, and the resulting graph is impressive:

```
최대 로짓
  100 ├──────────────╮            (초반 70k 스텝: 계속 캡에 걸림)
      │              ╰──╮
   30 │                 ╰────────  (이후 자연 안정화. 개입 거의 없음)
      └──────────────────────────→ 스텝
```

Zero loss spikes over 15.5 trillion tokens of training is a remarkable achievement for a trillion-parameter model.

Other large MoE models typically experience 3-5 spikes per trillion tokens, losing 10-20% of their total schedule to rollbacks and debugging each time. This is not the case here.

> Logits refer to the raw linear output values in an AI model before passing through activation functions like Sigmoid or Softmax. Their purpose is to represent the score the model assigns to each category (class), and these values are then converted into probabilities to make a final decision.

As of 2026:

- Kimi: MuonClip → Evolved to Per-Head Muon (independent optimization per head) in K3.
- From GLM-4.5: Muon adopted.
- DeepSeek-V4: Switched from AdamW to Muon.
- Qwen: Predicts and sets optimal learning rates and batch sizes for each model through its own scaling law experiments.

QK-Clip suppresses extreme attention scores. There are concerns that this might lead to a slight performance loss in situations requiring **very sharp focus** (e.g., complex multi-tool calls). It's a trade-off between stability and expressiveness.

<br>

## Reinventing Residual Paths - mHC, AttnRes

As the number of layers reached dozens to hundreds, a single residual path became insufficient.

> Residual Connection is a bypass that adds the input directly to the output of each layer, a fundamental grammar of deep learning since ResNet in 2015. It's like an elevator installed next to the stairs of a 100-story building; if a signal (gradient) had to climb and descend 100 floors only by stairs, it would get exhausted and disappear, but with an elevator, it's delivered intact.

### DeepSeek - mHC(Mainfold-Constrained Hyper-Connections)

First, let's look at the idea of Hyper-Connections (HC), which involves **widening the residual path into multiple branches and allowing them to exchange information**. Performance definitely improved, but there was a problem.

The property of **identity mapping** is broken. The core of residual connections is that if nothing is done, the input comes out as is. However, when branches start mixing, this guarantee is lost. Measurements on a DeepSeek 27B model showed signal amplification exceeding 3000x, sometimes leading to training collapse. Despite this, it was introduced to break the limitations of fixed information flow and maximize the model's expressiveness and training efficiency, but the signal explosion was too critical.

**Solution:** Therefore, in mHC, the mixing matrix for the branches is not arbitrary; instead, a **double stochastic matrix (row sums to 1, column sums to 1)**, known as a Birkhoff polytope, is forcibly projected onto the space. This constraint is implemented using the Sinkhorn-Knopp algorithm.

The Sinkhorn-Knopp algorithm involves:
1. **Unconstrained Parameter Generation:** Allows the model to learn a general weight matrix without constraints.
2. **Positivization:** Applies the exponential function (exp) to make all elements of the matrix positive (greater than 0).
3. **Alternating Normalization:** Divides to make row sums 1, and immediately divides again to make column sums 1, repeating this 20 times.
4. **Birkhoff Polytope** Projection: After these iterations, the matrix mathematically reaches a safe zone where all row and column sums are 1.0, and is then used as the final connection weight.

In other words, it's like casting a powerful black magic spell to force it within 1.0.

> Simply put, a double stochastic matrix means that no matter how complex the plumbing, the total amount of water entering and leaving is always the same, implying that water neither increases nor evaporates.

Consequently, signal amplification was suppressed from 3,000x to less than 2x, with a computational overhead of 6.7%. In 27B experiments, BBH scores increased from 43.8 (baseline) -> 48.9 (HC) -> 51.0 (mHC). This forms the basis of DeepSeek-V4's training.

In summary, traditional residual connections yielded diminishing returns as ultra-large AI models deepened. HC increased expressiveness, but led to signal amplification issues, making the AI too strong and causing signal saturation. mHC was introduced to prevent this.

### Kimi - AttnRes (Attention Residuals)

The same problem was solved in a different way: if mHC widened the residual path, AttnRes selectively connected them. Instead of just receiving the accumulated state from the immediately preceding layer, it **selectively retrieves representations from much earlier layers, weighted by attention scores.**

> If mHC is like adding 4 elevators, AttnRes is a retrieval system that specifies, "I need the 37th-floor meeting minutes right now!" You don't even have to go to the 37th floor!!

According to Moonshot's report, **training efficiency improved by approximately 25% with less than 2% additional cost**, while other sources cite 4% for training and 2% for inference.

These two techniques, though seemingly localized, are foundational technologies that "make it possible to train 2-3 trillion parameter models in the first place." This is why Kimi K3 could reach 2.8T, and why DeepSeek-V4 is stable at 1.6T.

<br>

## Engram (Conditional Memory)

This is a conceptual paper **DeepSeek released in January 2026**.

The problem statement highlights that when recalling "the capital of France is Paris," a transformer **cannot query a database.** Instead, it reconstructs this static fact through computation, mobilizing multiple layers of attention and FFNs. This is an expensive runtime reconstruction of a static lookup table, wasting effective depth, which should be used for high-level reasoning, on trivial tasks.

In other words, it's like asking why waste expensive and smart resources on easy questions. The paper's insight is that **language modeling is not one task, but two.**

https://arxiv.org/abs/2601.07372

- **Dynamic Reasoning:** Truly requires logical combination, multi-step inference, mathematics, code generation, and deep, adaptive computation.
- **Static Recall:** Proper nouns, idioms, grammar, templates, boilerplate, short code patterns—i.e., local, repetitive, and context-independent.

Transformers treat both equally, but Engram argues against this.

### Solution

**If MoE is conditional computation, then Engram is conditional memory.**

It's a modernization of classical N-gram embeddings.

- Takes a sequence of 2-3 tokens and queries a massive embedding table using a **hash function**.
- Completes in O(1) constant time, regardless of table size.
- Since the table address is determined solely by the input tokens, it **knows in advance which row is needed.** -> The GPU can **prefetch** from CPU memory while computing the preceding layers.

> If all LLMs until now were people calculating in their heads, Engram is like giving that person a dictionary on their desk. Things like "Diana, Princess of Wales" can be found in 0.1 seconds by opening the dictionary, whereas reconstructing it in one's head would take 10 seconds. This is extreme optimization. - The whole world is a cache.

### Results

- **Discovered a U-shaped scaling law,** revealing that **75% dynamic reasoning MoE and 25% static lookup Engram** is optimal for sparse capacity.
- 27B model experiment: 3-5 point improvement across knowledge, reasoning, and coding, with complex reasoning increasing from 70 -> 74, and knowledge from 57 -> 61 (%).
- **NIAH (Needle in a Haystack) accuracy 84.2% -> 97%** because separating memory allowed attention to focus on long-range context.
- **Achieved less than 3% throughput loss even when offloading a 100 billion parameter embedding table to system DRAM**.

Looking at the last point, if 100B parameters can be offloaded from a 1.6T model outside the GPU, the barrier to self-hosting decreases. A path to circumvent GPU HBM constraints opens up.

Engram is a core component of **DeepSeek-V4** and, as the paper's authors put it, aims to be an essential modeling primitive for next-generation sparse models. It's a declaration to become the **third fundamental component, following attention and MoE.**

<br>

## Post-Training and RL Infrastructure

It seems the real battleground these days is here; ultimately, those who throw money at it win.
Architectures now converge due to mutual copying, and **the differences actually emerge in post-training.**

#### RLVR - Reinforcement Learning with Verifiable Rewards

Mathematics and coding answers **can be mechanically verified.** There's no need to run a compiler, use a calculator, or have a human grade them, so they can be scaled infinitely. DeepSeek-R1 astonished the world with this.

#### GRPO - Group Relative Policy Optimization

Traditional PPO requires a separate Critic (value) model, which is usually similar in size to the policy model, effectively doubling memory usage.
GRPO eliminated the Critic. Instead, it generates **multiple answers for the same problem (a group) and judges good or bad through relative comparison within the group.**

> Analogy: Instead of creating a grading rubric (Critic), 8 students place their answers side-by-side and are graded relatively, determining "this is better than that." It's much cheaper.

Kimi uses a Self-Critique Rubric reward structure. While verifiable tasks can use RLVR, it started from the premise that **there are no correct answers for tasks like writing design.**

Kimi K2 learns not only from externally defined tasks but also from self-evaluation, using rubric rewards where the model assesses its own output.

It also uses an **agent data synthesis pipeline**. To learn how to use tools effectively, example data of tool usage is needed. However, such data doesn't exist in the world.

Kimi created a pipeline that **systematically generates tool-agent-task-trajectory data in both simulated and real environments**, mass-producing high-quality agent interactions with verifiable ground truths. The advantages of K2-series agents stem from this.

**slime - GLM's RL Infrastructure**: An open-source framework released with GLM-4.5, its core consists of two modes:

| 모드 | 구조 | 적합한 과제 |
|------|------|-------------|
| 코로케이트 + 동기 | 학습과 롤아웃이 같은 GPU | 수학·추론 (롤아웃이 빠름) |
| 디스어그리게이트 + 비동기 | 데이터 생성과 학습을 분리 | 에이전트 (롤아웃이 매우 느림) |

The second point is crucial: agent tasks can take tens of minutes to generate a single trajectory, and in synchronous mode, the GPU would remain idle. Asynchronous decoupling is necessary to keep the GPU saturated, making long-term agent RL practically feasible.

> Infrastructure is capability - This is the fundamental reason why GLM is strong in agentic coding. It's not the architecture, but the training infrastructure.

Expert Training -> Integrated Training (GLM)
1. Reasoning-specialized models, coding-specialized models, and agent-specialized models are each **trained separately with RL.** This allows for full utilization of strong signals from each domain.
2. Then, they are combined into a single general-purpose model through self-distillation.

> It's like training three specialists separately and then transferring their knowledge to one general practitioner.

Qwen extends RL to a million-scale agent environment and uses a curriculum that gradually increases task difficulty. The goal is real-world adaptability.

**Data Rephrasing Kimi**

High-quality text is finite, and simply repeating training multiple times leads to memorization (overfitting).

Kimi uses a strong teacher model to rephrase the same content in different perspectives and styles, creating data with diverse expressions but identical meaning, thereby encouraging understanding instead of memorization.

<br>

## Summary

| 기술 | 해결하는 문제 | 대표 개발자 | 현재 확산도 |
|------|---------------|-------------|-------------|
| 세분화 MoE + 공유 전문가 | 용량 ↑, 계산 ↓ | DeepSeek | 전원 채택 |
| 보조 손실 없는 로드밸런싱 | 라우팅 붕괴 | DeepSeek | 사실상 표준 |
| MLA (압축) | KV 캐시 폭증 | DeepSeek | DeepSeek, GLM, Kimi |
| DSA (선택) | 어텐션 제곱 폭발 | DeepSeek | DeepSeek, GLM |
| GDN/KDA 하이브리드 (순환) | 어텐션 제곱 폭발 | Qwen, Kimi | Qwen 전 세대, Kimi |
| MTP + 투기적 디코딩 | 생성 속도, 장거리 일관성 | DeepSeek | 전원 채택 |
| Muon / MuonClip | 학습 불안정, 토큰 효율 | Moonshot | 전원 전환 중 |
| mHC / AttnRes | 초대형 모델 신호 전파 | DeepSeek / Moonshot | 각자 플래그십 |
| Engram (조건부 메모리) | 계산으로 기억을 재구성하는 낭비 | DeepSeek | DeepSeek-V4 |
| slime / 에이전트 데이터 합성 | 장기 에이전트 학습 | Z.ai / Moonshot | 각자 핵심 자산 |

> DeepSeek's invented technology is adopted by GLM (DSA), Moonshot's invented optimizer is adopted by DeepSeek (Muon), and Kimi also adopts Qwen's hybrid batch (GDN -> KDA). This is the era of great "pakuri" (copying/borrowing). In the open-weight ecosystem, good ideas spread quickly, and as a result, data, RL infrastructure, and execution speed, rather than architecture, create performance and differentiation.

In the next post, I will delve deeper into the anatomy of each model.

- [DeepSeek](http://github.com/rlaope/estudy/blob/master/brains/AI/ow/deepseek.md)
- [Qwen](http://github.com/rlaope/estudy/blob/master/brains/AI/ow/qwen.md)
- [GLM](http://github.com/rlaope/estudy/blob/master/brains/AI/ow/glm.md)
- [Kimi](http://github.com/rlaope/estudy/blob/master/brains/AI/ow/kimi.md)

<br>

## Direct Comparison Table

### Spec Comparison Table (Based on Open Flagships, August 2026)

| 항목 | DeepSeek V4-Pro | Qwen3.6-27B / 3.5-397B | GLM-5.2 | Kimi K3 |
|------|-----------------|------------------------|----------|----------|
| 총 / 활성 파라미터 | 1.6T / 49B | 27B dense / 397B·17B | 744B / 40B | 2.78T / 104B |
| 컨텍스트 | 1M | 256K (YaRN 1M) / 1M | 1M | 1M |
| 어텐션 전략 | MLA + 압축희소 | GDN 하이브리드 3:1 | MLA + DSA + IndexShare | KDA + Gated MLA |
| MoE 구성 | DeepSeekMoE | 256중 8 + 공유 1 | 256중 8 + 공유 1 | 896중 16 (LatentMoE) |
| 옵티마이저 | Muon | 자체 스케일링 법칙 | Muon | Per-Head Muon |
| 잔차 | mHC | 표준 | 표준 | AttnRes |
| 멀티모달 | ✗ 텍스트 | ✓ 네이티브 (얼리퓨전) | ✗ 사실상 텍스트 | ✓ 네이티브 (비디오 포함) |
| 라이선스 | MIT | Apache 2.0 | MIT | Modified MIT |
| 자체 호스팅 | Flash만 현실적 | ✓✓ 가장 쉬움 | 어려움 | ✗ 사실상 불가 |
| 속도 (측정) | ~62 tok/s | 크기별 상이 | ~168 tok/s | ~62 tok/s |
| 출력 가격 (1M당) | $0.87 (Pro) / $0.28 (Flash) | $0.80대 (Coder-Next) | $4.10~4.40 | $15.00 |

> Prices and benchmarks vary significantly depending on the provider and time. Always verify the latest information.

### Character Comparison - What Each Model Prioritizes

| 모델 | 가장 아끼는 것 | 덜 아끼는 것 | 한 줄 철학 |
|------|----------------|--------------|-------------|
| DeepSeek | 비용 (계산·메모리·전력) | 멀티모달, 최상단 몇 % | 압도적 가격, 긴 컨텍스트 경제성 |
| Qwen | 접근성 (사용자의 하드웨어와 지갑) | 오픈 최상단 성능 | 로컬 실행, 다국어, 생태계 |
| GLM | 사람의 개입 (에이전트 자율성) | 멀티모달, 서비스 안정성 이력 | 장기 코딩 에이전트, 속도 |
| Kimi | 아무것도 안 아낌 (스케일 최전선) | 비용, 속도, 배포 용이성 | 최상위 능력, 프론트엔드, 자율 실행 |

```
DeepSeek ──MLA──────────────→ GLM-5.2, Kimi K3
         ──DSA──────────────→ GLM-5/5.1/5.2
         ──보조손실없는 밸런싱──→ 사실상 업계 표준
         ──MTP──────────────→ 전원

Moonshot ──Muon/MuonClip────→ GLM-4.5, DeepSeek-V4

Qwen ────GDN 3:1 하이브리드──→ Kimi (KDA로 개량)

GLM ─────slime (오픈소스)────→ 커뮤니티 RL 인프라
```

Architectural ideas spread within a few months (effectively "pakuri-ed"). Therefore, sustainable differentiation comes not from architecture, but from data curation, RL, infrastructure, and execution speed.

<br>

## Detailed Strengths for Specific Tasks

I will provide recommendations by task type, along with more specific causal explanations.
It would be beneficial to focus on **why** certain models are ranked as they are, as this will make it easier to judge new models when they emerge.

### Coding Agents Across Multiple Files (Refactoring, Bug Fixing, Feature Implementation)

#### **1위 GLM-5.2 / 2위 Kimi K3 / 가성비: DeepSeek V4**

To explain why GLM: This task isn't about doing well once, but about not getting lost over 30-300 steps.
The deep-over-wide architecture is advantageous for multi-step reasoning, 7 trillion tokens of code and reasoning-focused training create domain density, and most importantly, slime's asynchronous RL makes slow, long-term tasks actually trainable. Additionally, its speed of 168 tokens per second reduces the total time spent in iterative loops.

**When to switch to Kimi?** -> When UI/Frontend has a large proportion, 12-hour autonomous execution is needed, or parallel sub-agents are required.

**When to switch to DeepSeek? ->** When budget is the top priority and it's cheaper to compensate for slightly lower accuracy with retries.

### Frontend UI Design Code

**1위는 Kimi K3가 (압 도 적)**

This area cannot be graded by a compiler and has no single correct answer, requiring aesthetic judgment.
Kimi's **self-critique rubric reward can precisely target this problem**, and its native vision allows it to actually see and judge the screens it creates. The #1 ranking in the Frontend Code Arena is a result of this combination.

**Alternative:** GLM-5.2 also ranks #1 in the Design Arena, making it strong. Considering the cost difference, it's worth testing.

### Batch Processing (Summarization, Classification, Extraction, Batch Pipelines)

**1위는 DeepSeek V4-Flash (압도적)**

Such tasks have low individual difficulty and **high volume.** Unit cost is everything. Flash, with 13B active parameters, costs $0.14/input and $0.28/output, making it about 80-95% cheaper than frontier models. This is the result of the triple savings from MLA + DSA + FP8.

**Alternatively,** if you self-host Qwen 3.6-35B-A3B, the token cost becomes zero (excluding hardware depreciation). If the volume is large enough, this option is cheaper.

### Local / On-Premise / Personal GPU

**Qwen3.6-27B** 사실상 유일한 정답급

When 4-bit quantized, it fits into a single consumer GPU with about 17GB, outperforming the 397B flagship in coding. Its dense architecture is advantageous over MoE in a batch 1 environment. It's Apache 2.0, and the Ollama, LM Studio, and GGUF ecosystems are the most robust.

#### Summary by Scale

- 8GB class: Qwen3-8B series
- 16~24GB: Qwen3.6-27B (Q4) — Optimal point
- 32GB+: Qwen3.6-35B-A3B (MoE, fast due to 3B active parameters)
- Multi-GPU server: DeepSeek V4-Flash (284B)
- Data center: GLM-5.2 (744B)
- Supernode: Kimi K3 (2.8T)

### 1 Million Token Ultra-Long Document Codebase

**1순위 GLM-5.2 / 2순위 DeepSeek V4**

Why GLM: IndexShare reduced FLOPs per token by 2.9x at 1M, creating a truly usable 1M context, not just an **advertised 1M**. It's important that long-context benchmarks improved even with reduced computation.

**DeepSeek?** At 1M, the KV cache is 10% of V3.2's, and Engram offloads static knowledge lookup, giving attention more room to focus on long-range context. This increased **NIAH accuracy from 84.2% to 97%**. It's a case of simultaneously addressing accuracy and cost in long contexts.

> Caution: For any model, actually filling 1M tokens will drastically increase latency and cost. It's usually better to reduce the context first with RAG. Long context is a complement to RAG, not a replacement.

### Math, Science Reasoning

**1순위 DeepSeek V4 / 2순위 Qwen3.5 3.7 계열**

Mathematics is an area where **RLVR (Reinforcement Learning with Verifiable Rewards)** works perfectly. DeepSeek paved this way with R1 and integrated it into V4. Qwen3.5 reported strong figures such as GPQA 88.4% and CodeForces Elo 2056 (top 1% programming level).

> Caution: Math benchmarks carry the highest risk of contamination, and famous problem sets might have been included in the training data. Verification should be done with newly created problems (as it could be cheating by copying from answer keys).

### Korean and Multilingual

**1순위 Qwen 계열**

Qwen3 was trained on 119 languages, and Qwen3.5 on 201 languages and dialects. It has long maintained an advantage over Western models, especially in CJK (Chinese, Japanese, Korean). The multilingual tokenizer also has the side effect of splitting Korean more efficiently, **processing the same sentence with fewer tokens.**

> **Practical Tip**: Korean performance differs between understanding and generation. While most models understand well, natural Korean style generation varies greatly. It is essential to compare directly.

### Image and Video Understanding

**1순위 Kimi K2.6/K3(비디오 포함) / 2순위 Qwen3.5 3.6**

Both are **early-fusion native multimodal.** They were trained together from the start, rather than having separate vision adapters attached later, allowing for natural reasoning across text and images.

**GLM and DeepSeek are excluded from this category.** They are virtually text-only.

### Web Browsing, Research Agents

**1순위 Kimi K3 (BrowseComp 강세)**

This is due to the combination of tool call accuracy, long autonomous execution, and parallel sub-agents. Agent Swarm allows exploring multiple search paths simultaneously.

### Creating Dedicated Models with Fine-Tuning

**1순위는 Qwen (Apache 2.0 + 사이즈 선택폭)**

The key to fine-tuning is choosing the right size that fits your data.

Only Qwen offers models from 0.8B to 400B within the same generation. Its license is the most permissive, and it has the most tuning recipes and community resources.

```
멀티모달(이미지/비디오)이 필요한가?
├─ 예 ─→ 로컬에서 돌려야 하나?
│         ├─ 예 ─→ Qwen3.6 계열
│         └─ 아니오 ─→ Kimi K2.6/K3
└─ 아니오
    ↓
자체 호스팅이 필수인가? (규제/보안/비용)
├─ 예 ─→ GPU가 몇 장인가?
│         ├─ 1장 ─→ Qwen3.6-27B ★
│         ├─ 소규모 서버 ─→ DeepSeek V4-Flash
│         └─ 데이터센터 ─→ GLM-5.2
└─ 아니오
    ↓
가장 중요한 것은?
├─ 비용 ────────→ DeepSeek V4-Flash
├─ 코딩 에이전트 ─→ GLM-5.2
├─ 속도/지연 ────→ GLM-5.2
├─ 절대 성능 ────→ Kimi K3
└─ 프론트엔드 ───→ Kimi K3
```

<br>

## Glossary

| 용어 | 원어 | 한 줄 설명 |
|------|------|-------------|
| 게이티드 델타넷 | Gated DeltaNet (GDN) | A representative implementation of linear attention. Summarizes the past into a fixed-size state using delta rule + exponential gating. Used since Qwen3-Next. |
| 공유 전문가 | Shared Expert | An MoE expert that is always active for all tokens. Handles common knowledge. |
| 관할권 / 데이터 주권 | — | The issue of which country's infrastructure data passes through. Critical in regulated industries. |
| 그룹 상대 정책 최적화 | GRPO | Learns by relatively comparing multiple answers to the same problem within a group, without a critic model. Saves memory compared to PPO. |
| KV 캐시 | KV Cache | K·V values stored for reuse during generation. Grows proportionally to length, dominating service costs. |
| KDA | Kimi Delta Attention | Kimi's improved version of GDN. Combines channel-wise gating + Gated MLA. 6.3x faster decoding at 1M. |
| QK-Norm | — | Normalizes Q·K to prevent attention logit explosion. Not applicable to MLA. |
| QK-Clip | — | Directly scales down Q·K weights of a head if its logit exceeds a threshold. Core of MuonClip. |
| 다중 토큰 예측 | MTP | Predicts multiple next tokens simultaneously. Improves long-range consistency ↑ + repurposed as speculative decoding draft. |
| 델타 규칙 | Delta Rule | Updates only the difference between existing memory and new information. Error-correcting memory. |
| 라우터 | Router | A gate in MoE that decides which expert a token is sent to. |
| 라우팅 붕괴 | Routing Collapse | A phenomenon where tokens flock to a few experts, wasting the capacity of the others. |
| 라이트닝 인덱서 | Lightning Indexer | DSA's filter. Quickly selects the top k relevant tokens before attention. |
| 로드 밸런싱 | Load Balancing | Techniques to evenly distribute expert usage. Auxiliary-loss method vs. auxiliary-loss-free method. |
| 롱호라이즌 과업 | Long-horizon Task | Autonomous tasks spanning dozens to thousands of steps. Consistency is more important than capability. |
| mHC | Manifold-Constrained Hyper-Connections | Widens residual paths into multiple branches, stabilized by double stochastic matrix constraint (Birkhoff polytope). DeepSeek-V4. |
| MLA | Multi-head Latent Attention | Compresses K·V into low-dimensional latent vectors for storage. Significantly reduces KV cache. DeepSeek invention. |
| MoE | Mixture of Experts | Activates only a subset of many experts to achieve large capacity with small computation. |
| Muon / MuonClip | — | Optimizer considering matrix structure / Combines with QK-Clip for stabilization. Moonshot invention, spreading in the industry. |
| 바늘 찾기 | NIAH (Needle in a Haystack) | A test to measure how accurately specific information is retrieved from a long document. |
| 버코프 폴리토프 | Birkhoff Polytope | The space of double stochastic matrices. mHC projects onto this to suppress signal amplification. |
| 보조손실 없는 로드밸런싱 | Auxiliary-loss-free | Balances using a bias term in router scores instead of a loss function. No performance degradation. DeepSeek-V3. |
| 사고 예산 | Thinking Budget | A feature to specify the maximum tokens for thinking. Adjusts latency and performance. |
| 선형 어텐션 | Linear Attention | Summarizes the past into a fixed-size state, eliminating the KV cache. Weak for precise retrieval. |
| 손실 스파이크 | Loss Spike | A sudden surge in error during training. Rollback costs are enormous. |
| slime | slime | Z.ai's open-source RL framework. Asynchronous decoupling mode is key for long-term agent training. |
| 싱크혼-크노프 | Sinkhorn-Knopp | An iterative algorithm to make a matrix double stochastic. Used in mHC implementation. |
| 얼리퓨전 | Early Fusion | A multimodal approach that trains text and images together from the start. Contrasts with attaching adapters later. |
| 에이전트 스웜 | Agent Swarm | Parallel coordination of up to 300 sub-agents. Kimi's unique feature. |
| Engram | 조건부 메모리 | O(1) lookup of static knowledge using a hash table. Eliminates the waste of reconstructing memory through computation. DeepSeek. |
| 오픈웨이트 | Open-weight | Only weights are public (data·code are private). All four families in this note fall into this category. |
| YaRN | — | RoPE extension technique. Extends context beyond training length. |
| 유효 깊이 | Effective Depth | The remaining layers a model can actually use for high-level reasoning. Engram's core problem statement. |
| IndexShare | — | Four layers share a DSA indexer. 2.9x FLOPs reduction at 1M. GLM-5.2. |
| AttnRes | Attention Residuals | Selectively retrieves residuals in the depth direction using attention weights. Kimi K3. |
| 양자화 인지 학습 | QAT | Minimizes compression loss by assuming low-bit representation from the training phase. |
| 전문가 세분화 | Fine-grained Experts | Breaks down experts into smaller units and activates more. Increases combination diversity. DeepSeekMoE. |
| 잔차 연결 | Residual Connection | A bypass that adds the input to the layer's output. Fundamental for training deep networks. |
| 정적 패턴 회상 vs 동적 추론 | — | Key distinction in the Engram paper. Separates tasks that can be looked up from those requiring computation. |
| 투기적 디코딩 | Speculative Decoding | Accelerates by having a small model predict in advance and a large model verify. |
| DSA | DeepSeek Sparse Attention | Selects only the top k relevant tokens with an indexer for attention. Quadratic → virtually linear. |
| 프리필 / 디코드 | Prefill / Decode | Reading input (parallel) / Generating output (sequential). Different optimization targets. |
| 활성 파라미터 | Active Parameter | Parameters actually involved in computation per token. Determines cost and speed. |
| RLVR | RL with Verifiable Rewards | Reinforcement learning with mechanically verifiable answers. Infinitely scalable in math and coding. |

<br>

## Common Misconceptions and Pitfalls

### Misconception 1. A larger number of parameters means a better model.

No. Qwen3.6-27B dense outperforms Qwen3.5-397B-A17B in coding benchmarks. There are cases where a 27B model beats a 397B model.

This is because performance is a function of **compute x data quality x training techniques x post-training**, and must be considered holistically. It's a normal phenomenon for smaller models to later outperform larger models that were released earlier.

### Misconception 2. Fewer active parameters means it runs on a personal PC.

Absolutely not. Kimi K3 has 104B active parameters, but the entire 2.8T must be loaded into memory, which is 594GB even when compressed to 4-bit.
Active parameters determine speed, while total parameters determine memory. Judge self-hosting feasibility based on **total parameters.**

### Misconception 3. Are all Chinese models open?

Increasingly, no. Qwen's top-tier Max models are closed, and as of August 2026, the latest available general-purpose Qwen model is 3.6-27B, with stronger 3.7 and 3.8-Max models being API-only.

Kimi K3 was also only accessible via API at the time of its announcement, with weights released later.

### Misconception 4. The MIT license has no restrictions?

Licenses and geopolitics are separate. Z.ai is on the U.S. entity list, and its hosting API routes through Chinese infrastructure. Running weights internally resolves this issue, but using a hosting API is an entirely different decision.

Furthermore, Kimi uses a Modified MIT license (requiring attribution for large-scale services), not pure MIT.

### Misconception 5. Benchmark scores = actual performance.

There are three pitfalls.

1. **Vendor reporting bias:** Figures in technical reports come from settings favorable to their own models. Claims like "beat GPT" are mostly based on vendor's own measurements.
2. **Contamination:** The possibility of benchmark problems being mixed into training data, especially severe in math and coding.
3. **Lag:** Benchmarks reflect reality with a delay of several months.

The solution is simple: Create 20-50 cases with your own business data and compare them directly. It takes half a day and is more accurate than 100 benchmarks.

### Misconception 6. If the context is 1M, you can put in all 1 million tokens.

You can, but:

- **Cost surge (only input tokens are charged)**
- **Latency surge (prefill becomes longer)**
- **Accuracy may decrease (phenomenon of missing middle parts)**

In most cases, it's cheaper and more accurate to extract only relevant parts using RAG. Consider long context as a complement to RAG, not a replacement.

### Misconception 7. Open models are always cheap?

Kimi K3 is extremely expensive at $15 per million output tokens. This is over 17 times the cost of DeepSeek V4 Pro, and open weights and low prices are separate issues.

### Misconception 8. All techniques mentioned in a paper are included in that model.

Just because DeepSeek published papers on mHC, Engram, and DSA doesn't guarantee all three are in V4. Actual implementations can be adjusted, and details should be confirmed with technical reports and model cards. Especially for models like K3, where a complete technical report is not yet available, explanations often rely on third-party analysis, so be cautious.

### Misconception, Pitfall: Model Retirement Risk

Kimi K2.5 was launched in January 2026 and terminated on May 25. DeepSeek's legacy endpoints also retired on July 24.
If you build products relying on hosting APIs, you must check the model's lifecycle policy. Self-hosting eliminates this risk.

### Misconception 10. Output changes due to quantization during serving.

Serverless providers typically serve active values quantized to FP8. The results subtly differ from running the publicly released weights as-is, and this can be the cause of discrepancies when directly comparing benchmarks.
