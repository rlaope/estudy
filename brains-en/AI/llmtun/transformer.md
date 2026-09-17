# Understanding Transformer and Causal LM Architectures

The Decoder-only Transformer architecture, which forms the foundation of large language models (LLMs), is a deterministic computational system that maps sequence data into a high-dimensional vector space and calculates the next token and its probability distribution using an autoregressive method.

This document analyzes the mathematical essence of autoregressive language models and how their unique component structures impact the model's computational bottlenecks and weight updates, focusing on low-level mechanisms.

## Why Can LLMs Generate Conversations, Inferences, and Code by Only Predicting the Next Token?

From a mathematical perspective, next-token prediction is the process of learning the conditional probability distribution $P(x_t | x_1, x_2, \dots, x_{t-1})$.

The reason this seemingly simple single objective function transfers to high-level general cognitive abilities lies in

**the compression of sequence space and the learning of nonlinear manifolds.**

1.  **Compression of World Representation:** To accurately predict the next word in vast text sequences found on the internet (human conversations, mathematical logical developments, compilable source code), the model must encode not only the statistical consistency within the context but also the underlying rules that generated that text (physical laws, programming syntax, logical causality) into its internal parameter space in the form of weight matrices.
2.  **Context Conditioning:** The autoregressive structure has a feedback loop where the output of the previous step is combined as an input variable for the next step. When predicting the t-th token, the model accumulates interaction information from the t-1 tokens in a vector space through high-dimensional nonlinear transformations (Attention, FFN). In this process, context information is mapped not as a fixed static vector but as a fluid dynamic sub-space, leading to emergent computational capabilities such as instruction following and multi-step reasoning (in-context learning).

<br>

## Key Mechanisms and Bottlenecks of Decoder-only Architectures

The performance and hardware inference speed of Decoder-only Transformers are governed by the dimensional structure of weight matrices and memory I/O patterns.

> Causal LMs are language models trained to predict the next token by only looking at previous tokens. GPT, Llama, Qwen, and Mistral are all Causal LMs.

| Model Type                   | Representative Models           | Training Objective          |
| :--------------------------- | :------------------------------ | :-------------------------- |
| **Causal LM (Decoder-only)** | GPT, Llama, Qwen, Mistral, Gemma | Next Token Prediction       |
| **Masked LM (Encoder-only)** | BERT, RoBERTa, DeBERTa, ELECTRA | Masked Token Prediction     |
| **Seq2Seq LM (Encoder-Decoder)** | T5, FLAN-T5, BART, UL2          | Input → Output Generation   |
| **Prefix LM / Variants**     | GLM family, some U-PaLM experiments | Generation after Prefix     |

### Causal Self-Attention and Causal Masking

Causal Self-Attention is an infrastructure guard that prevents future token information from interfering with the prediction probability of the current token.

Early models like BERT processed the entire sentence (both preceding and succeeding words) simultaneously to understand context.

In models like GPT, which must predict the next word to continue a conversation, allowing them to see future words would be like letting them peek at the answer sheet during an exam, making learning impossible.

By applying masking that overwrites future token positions with $-\infty$ during matrix operations, the model is forced to predict the next word using only 'words that have appeared so far'.

After the matrix multiplication of the Query matrix $Q \in \mathbb{R}^{B \times H \times T \times d}$ and the Key matrix $K \in \mathbb{R}^{B \times H \times T \times d}$, and before applying Softmax, the upper triangular matrix region is masked with $-\infty$ (in actual computation, -1e9 or the smallest representable negative number).

$$Score = \text{Softmax}\left(\frac{QK^T}{\sqrt{d}} + M\right)V, \quad M_{ij} = \begin{cases} 0 & \text{if } i \geq j \\ -\infty & \text{if } i < j \end{cases}$$

This operation incurs $O(T^2)$ spatial and temporal complexity, exacerbating large-scale data transfer bottlenecks between GPU SRAM and High Bandwidth Memory (HBM) as sequence length T increases.

### Rotary Position Embedding (RoPE)

Modern Causal LMs like Llama, Qwen, and Mistral adopt **RoPE**, which performs rotation matrix and complex number multiplication operations on pairs of Query and Key vector matrix channels in a 2D complex plane, instead of adding absolute positions (Absolute PE).

The traditional method of absolute position embedding involved adding a fixed position vector (e.g., position 1 for the first word vector, position 2 for the second word vector) to inform the model about word order.

**The problem that arose here was** that if a model was trained with a token length of 4,000, when a long sentence of 8,000 tokens was encountered in actual service, the addition of an unseen 8,000th position vector would completely disrupt the model's computational landscape.

Instead of adding absolute numbers, this method rotates (Rotary) word vectors by a specific angle for each dimension. Because it only remembers the relative distance (angle difference) between words, it can flexibly handle positional relationships even if the sentence length exceeds the training range.

$$R_{\Theta, m}^d = \text{diag}\left(R_{\theta_1, m}, R_{\theta_2, m}, \dots, R_{\theta_{d/2}, m}\right), \quad R_{\theta_i, m} = \begin{pmatrix} \cos m\theta_i & -\sin m\theta_i \\ \sin m\theta_i & \cos m\theta_i \end{pmatrix}$$

RoPE guarantees a mathematically derived limit where the upper bound of the Attention Score exponentially decreases as the distance between tokens increases (Long-range Decay), allowing the model to flexibly preserve the continuity of relative positional relationships even when exceeding a fixed context length.

### SwiGLU Activation Function-based FFN (Feed-Forward Network)

Replacing the ReLU/GELU-based FFN of the traditional Vanilla Transformer, the SwiGLU architecture, which gates the outputs of two linear projection layers with a Swish function, has become standard.

Previously, simple one-dimensional activation functions like ReLU and GELU, which zero out values below a certain threshold, were used in artificial neural networks to extract data features.

**The problem that arose was** that the function forms were too simple to represent coding contexts or complex mathematical causal relationships that LLMs needed to handle, limiting the model's ability to learn high-dimensional knowledge in depth.

SwiGLU combined a gating structure that uses the results of two weight matrix operations as data for a single gate and multiplies them together. By constructing a neural network with a flexible curved shape, the model became better able to precisely represent complex logical structures.

$$\text{SwiGLU}(x) = \left( \text{Swish}_{1}(xW_{gate}) \otimes xW_{up} \right) W_{down}$$

SwiGLU mitigates the vanishing gradient problem and expands the nonlinear representation capacity of the operational dimension, accelerating loss convergence speed relative to parameters. However, it increases VRAM weight loading capacity because it must maintain three weight matrices simultaneously ($W_{gate}, W_{up}, W_{down}$).

### RMSNorm (Root Mean Square Normalization) and Pre-LN

To ensure training and numerical stability, RMSNorm, which completely eliminates the mean calculation part of LayerNorm and normalizes solely based on the Root Mean Square (RMS), is preferred.

Previously, both the mean and variance of the data were calculated to equalize the range of values, and then LayerNorm (this normalization was placed as Post-LN) was applied at the output end after all operations were complete.

The problem was that as model parameters grew to tens of billions, the operation of calculating the mean itself caused hardware GPU bottlenecks, and learning signals calculated in later layers failed to propagate to earlier layers, leading to disconnections.

By boldly omitting the computationally wasteful mean and only calculating the Root Mean Square (RMS), speed was increased. Simultaneously, placing this normalization block as Pre-LN just before the start of operations created a pathway for learning signals to flow uninterrupted to lower layers without interference.

$$\text{RMSNorm}(x) = \frac{x}{\sqrt{\frac{1}{d}\sum_{i=1}^d x_i^2 + \epsilon}} \odot \gamma$$

By combining this normalization block with a Pre-LN structure, placed directly before entering the Transformer layer, it ensures that gradients flow directly to the input layer without loss through the residual connection pathway during backpropagation, preventing gradient explosion at the scale of tens of billions of parameters.

### Hardware Impact of KV Cache

During the autoregressive inference phase, this technique involves keeping already computed past token key-value tensors resident in HBM and directly reusing them for the next step's computation.

This eliminates computational overhead by converting large matrix multiplication operations into vector-matrix multiplication operations, but as the context window grows, it occupies a significant amount of VRAM, becoming a factor that causes OOM (Out Of Memory).

Previously, each time a new word was generated, the model would re-examine all words from the initial input up to the newly generated one, and relationships were re-evaluated from scratch every time.

If a conversation grew long, accumulating 2000 words, the model would have to restart matrix multiplication operations with all 2000 words for the 2001st word, causing the speed to slow down with each character generated.

By not discarding the core computational data (Key Value tensors) of past words and pre-caching them in GPU VRAM, when generating the next word, only the operation for the single newly added word needs to be performed, and then combined with the existing cache. This allows real-time conversation speed to be maintained even as sentences grow longer.

<br>

## The code below is a set of modules that precisely implement Causal Masking, RMSNorm, and SwiGLU operational mechanisms within a single PyTorch framework, based on the architectural philosophies of Llama and nanoGPT.

```py
import torch
import torch.nn as nn
import torch.nn.functional as F
import math

class RMSNorm(nn.Module):
    """Llama 및 Qwen 아키텍처에서 표준으로 사용하는 RMSNorm 레이어 구현"""
    def __init__(self, dim: int, eps: float = 1e-6):
        super().__init__()
        self.eps = eps
        self.weight = nn.Parameter(torch.ones(dim))

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        # x shape: [Batch, Time, Dim]
        variance = x.pow(2).mean(-1, keepdim=True)
        return x * torch.rsqrt(variance + self.eps) * self.weight

class SwiGLUFFN(nn.Module):
    """Gated Linear Unit 구조 기반의 SwiGLU Feed-Forward Network"""
    def __init__(self, dim: int, hidden_dim: int):
        super().__init__()
        self.w_gate = nn.Linear(dim, hidden_dim, bias=False)
        self.w_up = nn.Linear(dim, hidden_dim, bias=False)
        self.w_down = nn.Linear(hidden_dim, dim, bias=False)

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        # Swish(xW_gate) * xW_up 연산 수행 후 Down projection
        gate_output = F.silu(self.w_gate(x)) # SiLU는 PyTorch의 Swish 구현체
        up_output = self.w_up(x)
        return self.w_down(gate_output * up_output)

class CausalSelfAttention(nn.Module):
    """KV Cache 인터페이스 및 인과적 마스크가 내장된 Multi-Head Attention"""
    def __init__(self, dim: int, n_heads: int, max_seq_len: int = 2048):
        super().__init__()
        assert dim % n_heads == 0
        self.n_heads = n_heads
        self.head_dim = dim // n_heads
        
        # Q, K, V 행렬 합산 프로젝션 레이어
        self.c_attn = nn.Linear(dim, 3 * dim, bias=False)
        self.c_proj = nn.Linear(dim, dim, bias=False)
        
        # 하한 마스크 행렬 버퍼 사전 등록 (상삼각 행렬 소거용)
        self.register_buffer(
            "bias", 
            torch.tril(torch.ones(max_seq_len, max_seq_len)).view(1, 1, max_seq_len, max_seq_len)
        )

    def forward(self, x: torch.Tensor, past_kv: tuple = None) -> tuple:
        B, T, C = x.size()
        
        # Q, K, V 분할 연산 [B, T, C] -> [B, T, 3*C]
        q, k, v = self.c_attn(x).split(C, dim=2)
        
        # 멀티헤드 차원 분할 및 Transpose [B, T, H, d] -> [B, H, T, d]
        q = q.view(B, T, self.n_heads, self.head_dim).transpose(1, 2)
        k = k.view(B, T, self.n_heads, self.head_dim).transpose(1, 2)
        v = v.view(B, T, self.n_heads, self.head_dim).transpose(1, 2)
        
        # KV Cache 로직 인터페이스 처리
        if past_kv is not None:
            past_k, past_v = past_kv
            k = torch.cat([past_k, k], dim=2) # 시간 축(T) 기준 텐서 결합
            v = torch.cat([past_v, v], dim=2)
        current_kv = (k, v)
        
        total_T = k.size(2)
        
        # Scaled Dot-Product Attention 연산
        att = torch.matmul(q, k.transpose(-2, -1)) * (1.0 / math.sqrt(self.head_dim))
        
        # 인과적 마스킹 행렬 적용
        att = att.masked_fill(self.bias[:, :, total_T-T:total_T, :total_T] == 0, float('-inf'))
        att = F.softmax(att, dim=-1)
        
        y = torch.matmul(att, v) # [B, H, T, d]
        y = y.transpose(1, 2).contiguous().view(B, T, C) # 차원 복원
        
        return self.c_proj(y), current_kv

# 단일 가동 및 차원 정합성 검증 테스트
if __name__ == "__main__":
    B, T, C = 2, 10, 512
    x_input = torch.randn(B, T, C)
    
    # 컴포넌트 선언
    norm = RMSNorm(dim=C)
    attn = CausalSelfAttention(dim=C, n_heads=8)
    ffn = SwiGLUFFN(dim=C, hidden_dim=1376)
    
    # 1. Pre-LN 구조 기반 Attention Forward
    norm_x = norm(x_input)
    attn_out, kv_state = attn(norm_x)
    x_res1 = x_input + attn_out # 1차 잔차 연결
    
    # 2. Pre-LN 구조 기반 FFN Forward
    norm_x2 = norm(x_res1)
    ffn_out = ffn(norm_x2)
    final_output = x_res1 + ffn_out # 2차 잔차 연결
    
    print("=== 차원 정합성 프로파일링 결과 ===")
    print(f"입력 텐서 구조: {x_input.shape}")
    print(f"최종 출력 텐서 구조: {final_output.shape}")
    print(f"캐싱된 Key 텐서 구조 [B, H, T, d]: {kv_state[0].shape}")
```

| Model Family | Base Position Encoding & Base Frequency (RoPE Base Theta) | Normalization Layer Type & Position | FFN Activation Function & Dimension Scaling (Activation & Hidden Expansion) | Attention Architecture Type (KV Cache Optimization) | Input vs. Output Operation Characteristics & Bottleneck Section (Bottleneck Profile) |
| :--------------------------- | :------------------------------------------------ | :---------------------------------- | :---------------------------------------------------------- | :------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Llama 3 (8B / 70B)           | RoPE (Theta = 500,000)                            | RMSNorm (Pre-LN structure)          | SwiGLU (Hidden Dim ≈ 3.5×d)                                 | GQA (Grouped-Query Attention)<br>1 KV pair mapped per 8 Qs | Memory-bound (Decode phase)<br>Key challenges include controlling precision limits due to high RoPE frequency and preventing KV Cache memory fragmentation when entering ultra-large context regions. |
| Qwen 2 (7B / 72B)            | RoPE (Theta = 1,000,000)                          | RMSNorm (Pre-LN structure)          | SwiGLU (Hidden Dim ≈ 3.6×d)                                 | GQA (Grouped-Query Attention)<br>GQA fully adopted even for smaller model lines | Compute-bound / Memory-bound crossover<br>Optimization of Attention operation kernels is essential due to the extreme expansion of RoPE base frequency to support up to 128k context extension. |
| Mistral (7B v0.3)            | RoPE (Theta = 100,000 / v0.3 improvement)         | RMSNorm (Pre-LN structure)          | SwiGLU (Hidden Dim ≈ 3.5×d)                                 | SWA (Sliding Window Attention) + GQA<br>Fixed local window size control | I/O-bound mitigation structure<br>The Sliding Window technique forcibly fixes the physical upper limit of the KV Cache, specializing in securing high-density serving throughput. |
