# GDN(Gated DeltaNet)

To summarize GDN in one sentence, it is an **RNN that uses a fixed-size matrix as memory**, and this memory is updated at each token by overwriting + gradually forgetting.

To understand the above sentence, we will follow the order below.

1.  Softmax attention -> Linear attention: Why can it be summarized as a single matrix state?
2.  Problem with Linear attention: Memory pollution due to continuous accumulation
3.  Two branches of solutions
    *   **Gating (attenuation):** Blurs old information entirely -> Mamba2, GLA, RetNet
    *   **Delta rule (overwriting):** Deletes existing values for the same key and writes new ones -> DeltaNet
4.  Combining both -> Gated DeltaNet
5.  How is training parallelized (chunkwise, WY/UT transform)?
6.  Why real models don't use GDN exclusively but mix it with attention at a 3:1 ratio
7.  Problems arising from handling state instead of KV cache during serving

Let's check the table below for how to read the formulas.

| Symbol | How to read | Meaning |
|---|---|---|
| q_t, k_t, v_t | query, key, value of the t-th token | A list of numbers (vector) of length about 128. The subscript t means "which token number" |
| o_t | t-th output | The resulting vector that the layer outputs for the t-th token |
| S_t | state up to the t-th token | A table (matrix) summarizing the content seen so far. The main subject of this note |
| S_{t-1} | previous state | The table from one token earlier |
| qᵀk (q^\top k) | inner product of q and k | A single number indicating how similar two vectors are |
| v kᵀ (v k^\top) | outer product of v and k | An operation that creates a table from two vectors. "One page of memory writing v at address k" |
| S q | multiplying a vector by a table | "Reads the table at address q and retrieves the value" |
| Σ (\sum) | sum all | Σ_{i≤t} means "sum all tokens from 1 to t" |
| α (alpha) | attenuation | A number between 0 and 1. Multiplied by the existing table to determine "how much to keep". 0.9 means 90% is kept |
| β (beta) | write strength | A number between 0 and 1. Determines "how strongly to overwrite" new information |
| I | identity matrix | A table that doesn't change anything when multiplied. Corresponds to the number 1 |
| ∈ ℝ^(a×b) | ~ is a×b size | Means "a table containing a×b real numbers" |
| O(1), O(n) | Big O | Cost is independent of length (O(1)) / proportional to length (O(n)) |

For example, the core equation of linear attention

$$S_t = S_{t-1} + v_t k_t^\top$$

is, in words, **current table = previous table + one memory page for this token**. The GDN equation that follows is also just a symbolic representation of slightly blurring the previous table (α), deleting the old value at the current key's position (β), and writing a new value.

## Linear attention

### What happens when softmax is removed

The t-th output of general attention is as follows:

$$o_t = \sum_{i \le t} \frac{\exp(q_t^\top k_i)}{\sum_j \exp(q_t^\top k_j)} v_i$$

If we remove softmax (including normalization) here and just use the inner product,

$$o_t = \sum_{i \le t} (q_t^\top k_i)\, v_i = \Big(\sum_{i \le t} v_i k_i^\top\Big) q_t = S_t\, q_t$$

I will call the term inside the parentheses S_t.

S_t is "a table (matrix) that aggregates all content seen up to the t-th token" and its size is d_v rows x d_k columns. (d_v is the value vector length, d_k is the key vector length, usually 128). The original expression `S_t ∈ ℝ^(d_v × d_k)` means "S_t is a table containing d_v x d_k real numbers," and it is a shorthand notation for this sentence.

When grouped this way, the following equation shows that v_t k_tᵀ is simply added to the table whenever a new token arrives.

$$S_t = S_{t-1} + v_t k_t^\top, \qquad o_t = S_t q_t$$

This is linear attention, where information from past tokens is stored in a single $d_v \times d_k$ matrix. Thus, memory during decoding is O(1) regardless of sequence length, and computation is O(d²) per token.

### From the perspective of fast weights

It's easier to understand if you view $S_t$ as an associative memory that outputs a value when a key is input. Adding $v k^\top$ means "recording v at address k," and $S q$ means "reading from address q."

Schlag et al. (2021) organized this perspective, and all subsequent DeltaNet-family models are explained from this viewpoint.

### Problems

Since values are continuously added, if keys are not orthogonal to each other (which is naturally impossible if the number of tokens exceeds d_k), values will continuously overlap near the same address, leading to noise when reading.

Consequently, pure linear attention showed a definite drop in language modeling quality, especially in-context retrieval, compared to softmax attention.

## Solution Direction 1: Gating (Attenuation)

The simplest method is to gradually reduce memory at each step.

$$S_t = \alpha_t S_{t-1} + v_t k_t^\top, \qquad \alpha_t \in (0, 1)$$

-   If $\alpha$ is constant, it's RetNet style
-   If $\alpha_t$ is a scalar that changes based on input, it's Mamba2
-   If $\alpha_t$ is a vector per key dimension, it's GLA (Gated Linear Attention)

The advantage is that it can quickly "forget everything" when the context changes. The disadvantage is that it **cannot forget selectively**.

Since attenuation memory is applied uniformly (or channel-wise) across the entire memory, it cannot selectively erase information corresponding to a specific key.

## Solution Direction 2: Delta Rule (Overwriting)

This is DeltaNet's update equation.

$$S_t = S_{t-1} - \beta_t (S_{t-1} k_t - v_t) k_t^\top = S_{t-1}(I - \beta_t k_t k_t^\top) + \beta_t v_t k_t^\top$$

1.  $S_{t-1} k_t$: The existing value $v^{old}$ obtained by reading memory with the current key
2.  $v_t - v^{old}$: The difference (delta) between the new value to be written and the existing value
3.  Multiply that difference by $\beta_t$ (write strength, 0~1) and add it to that address

That is, if $\beta_t = 1$ and $k_t$ is a unit vector, it completely erases the old value in the k direction and replaces it with a new value. This directly solves the problem of continuous accumulation mentioned in point 1.

### Viewing as Online Regression (Important)

Considering the loss $\mathcal{L}(S) = \frac{1}{2}\|S k_t - v_t\|^2$, the result of taking one SGD step with learning rate $\beta_t$ for this loss is exactly the equation above.

-   Linear attention: Loss is of the form $-\langle S k_t, v_t \rangle$ (only increases correlation)
-   DeltaNet: Loss is regression error (corrects what's wrong)

This perspective provides a common framework for understanding TTT (Test-Time Training), Titans, and various subsequent variations, so it's good to be familiar with it.

**You can think of one layer as online learning a small linear regression model during inference**.

### 3.3 Meaning of $(I - \beta k k^\top)$

If k is normalized, $I - \beta k k^\top$ is a generalized Householder matrix. Its eigenvalues are $1 - \beta$ in the k direction and 1 in other directions. Therefore,

-   Selective erasure that reduces only the component in the k direction and leaves others unchanged
-   For this reason, L2 normalization for q and k is essential. Without normalization, eigenvalues can go out of bounds, causing the state to diverge.

One more thing: if the range of $\beta$ is extended to (0, 2), eigenvalues can become negative, and theoretical results show that this can solve state tracking problems like parity (Razzi et al., ICLR 2025). This often comes up in discussions about expressiveness, so it's good to know.

### Disadvantages of DeltaNet

Since erasure must be done in the direction of a single key, when the context changes drastically and you want to clear the entire memory, it takes dozens of rank-1 repetitions to barely clear it, thus lacking the ability to forget quickly.

## GatedDeltaNet = Gating + Delta Rule

$$S_t = S_{t-1}\Big(\alpha_t (I - \beta_t k_t k_t^\top)\Big) + \beta_t v_t k_t^\top, \qquad o_t = S_t q_t$$

$\alpha_t \in (0,1)$: Input-dependent scalar attenuation (taken from Mamba2). Used to quickly clear the entire memory.
$\beta_t \in (0,1)$: Write strength. Used to precisely overwrite only specific keys.

You can think of them as having divided roles.

This is the case where the paper title is "Improving Mamba2 with Delta Rule".

The authors are Songlin Yang, Jan Kautz, Ali Hatamizadh, and it was published in ICLR 2025.

### Actual Order of Operations in Implementation

Instead of implementing the formula directly, it's usually solved like this (implementations in flash-linear-attention and vLLM are similar):

```
S = S * exp(g_t)                  # 1) First attenuate
v_old = S^T k_t                   # 2) Read existing value from attenuated memory
delta = beta_t * (v_t - v_old)    # 3) Calculate difference
S = S + k_t ⊗ delta               # 4) Rank-1 update
o_t = S^T q_t                     # 5) Read
```

Here, whether the state is set as $d_k \times d_v$ or $d_v \times d_k$ varies by implementation. When reading across papers, fla, and vLLM code, it's easy to get confused by transpose directions, so it's good practice to check the shape first.

### Reference Code (for training, slow)

```py
import torch
import torch.nn.functional as F

def gdn_recurrent_ref(q, k, v, g, beta, S0=None):
    """
    q, k: [B, H, T, Dk]   (Assumed to be L2 normalized)
    v:    [B, H, T, Dv]
    g:    [B, H, T]       log attenuation (negative). alpha = exp(g)
    beta: [B, H, T]       0~1
    return o: [B, H, T, Dv], S: [B, H, Dk, Dv]
    """
    B, H, T, Dk = k.shape
    Dv = v.shape[-1]
    S = torch.zeros(B, H, Dk, Dv, dtype=torch.float32, device=q.device) if S0 is None else S0
    o = torch.empty(B, H, T, Dv, dtype=torch.float32, device=q.device)
    q, k, v = q.float(), k.float(), v.float()
    for t in range(T):
        S = S * g[:, :, t].exp()[..., None, None]
        kt, vt, qt = k[:, :, t], v[:, :, t], q[:, :, t]
        v_old = torch.einsum('bhkv,bhk->bhv', S, kt)
        delta = beta[:, :, t, None] * (vt - v_old)
        S = S + torch.einsum('bhk,bhv->bhkv', kt, delta)
        o[:, :, t] = torch.einsum('bhkv,bhk->bhv', S, qt)
    return o, S
```

If you compare this function with the result of `fla.ops.gated_delta_rule.chunk_gated_delta_rule`, you'll better understand the error margin and how it changes with fp32/bf16 when we discuss precision below.

## Training Parallelization: Chunkwise Algorithm

This is the most challenging part of studying GDN and simultaneously the answer to why it has only now become practical.

### Problem

The recurrence relation above involves token-wise sequential operations, preventing GPU usage during training. While linear attention was easy to parallelize because $S_t$ is a simple cumulative sum, the delta rule is not a simple cumulative sum because it multiplies $S_{t-1}$ by the matrix $(I - \beta k k^\top)$.

### Core Idea

Divide the sequence length into C (usually chunks of 64).

-   **Between chunks:** States are passed sequentially. It's acceptable because it's sequential only L/C times (number of chunks).
-   **Inside chunks:** Calculated at once with matrix multiplication. Tensor Cores are used here.

To make the chunk internal calculation a matrix multiplication, the product of C Householder matrices $\prod (I - \beta_i k_i k_i^\top)$ must be expressed in a compressed form, and the WY representation is used for this.

$$\prod_{i=1}^{C}(I - \beta_i k_i k_i^\top) = I - \sum_{i=1}^{C} w_i k_i^\top \quad \text{(can be expressed in this form)}$$

$w_i$ can be found using a recurrence relation, and when grouped into a matrix,

$$T = \big(I + \mathrm{tril}(\mathrm{diag}(\beta) K K^\top, -1)\big)^{-1} \mathrm{diag}(\beta), \quad W = TK, \quad U = TV$$

it simplifies to a single lower-triangular matrix inverse, the UT transform. The next step is general matrix multiplication.

In GDN, since attenuation $\alpha$ is added, it is extended by multiplying a mask $\Gamma_{ij} = \gamma_i / \gamma_j$ using the cumulative attenuation within the chunk, $\gamma_t = \prod_{i \le t} \alpha_i$.

The complexity is approximately O(L·C·d + L·d²), which is linear with sequence length.

### Prefill and decode use different kernels

-   **Prefill**: chunk (`chunk_gated_delta_rule`). Compute-bound due to high computational load.
-   **Decode**: Fused recurrent kernel that updates one token at a time (`fused_recurrent_gated_delta_rule`). Memory-bound because the entire state must be read from and written back to HBM for each token.

It's easier to understand serving engine code if you know that these two paths are separate.

## In actual GDN layers, equations > components

If you open the code after only looking at the paper's equations, you'll find many things that make you wonder. Based on Qwen3-Next / Qwen3.5, the components included in a GDN block are summarized below.

| Component | Role | Notes |
|---|---|---|
| q, k, v projection | Projects input to q, k, v | Number of k/q heads < number of v heads (asymmetry opposite to GQA) |
| Short causal conv1d (kernel 4) + SiLU | Mixes information from the immediate preceding tokens | Handles local patterns. Conv state must also be stored separately during serving |
| q, k L2 norm | Delta rule stability | Refer to section 3.3 |
| $\alpha_t$ calculation | `g = -exp(A_log) * softplus(a + dt_bias)` | Same parameterization as Mamba2 |
| $\beta_t$ calculation | `beta = sigmoid(b)` | |
| Output gate | Multiplies output by `SiLU(z)` after RMSNorm | Per-head norm |

The GDN configuration for Qwen3-Next-80B-A3B is 16 key heads, 32 value heads, head dim 128, and conv kernel 4. For exact values, check the `linear_*` fields in config.json on HF.

### Directly calculating state size

Based on Qwen3-Next:

-   GDN state: value head 32 x 128 x 128 x 4 byte (fp32) = **2 MiB / layer / sequence**
-   Conv state: (16x128x2 + 32x128) channels x 3 x 2 byte ≈ 48 KiB / layer
-   Full attention KV: KV head 2 x head dim 256 x (K, V) x 2 byte = **2 KiB / layer / token**

Thus, the state of one GDN layer is equivalent to approximately **1,024** tokens of KV cache for one attention layer.

Considering a model with 36 GDN layers + 12 attention layers, the hybrid approach uses a fixed ~72 MiB per sequence + 24 KiB per token. If the same structure were entirely attention, it would be 96 KiB per token. Calculations show that for sequences under approximately 1K tokens, the hybrid uses more memory. In workloads with many short requests, assuming linear attention always provides memory benefits is incorrect. If the state is kept in bf16, the break-even point is halved.

NVIDIA also published similar calculations, concluding that one FP32 state of an SSM layer is 4 MiB per sequence, and FP8 KV is 0.5 KiB per token, meaning one state is equivalent to 8K tokens of attention KV cache.

## Why mix GDN with attention instead of using GDN exclusively?

### Fixed-size memory limitations

Since the state has a fixed size, there's an upper limit to the amount of information it can store. Tasks like finding an exact string in a long document (needle-in-a-haystack, exact copy, long list lookup) are fundamentally disadvantaged. Conversely, GD
