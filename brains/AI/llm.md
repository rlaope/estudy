# LLM Architecture & Inference Mechanics

### Transformer

현대 LLM, GPT Llama, Mistral등은 대부분 Autoregressive Decoder-only 구조를 따른다.

이게 뭐냐면 텍스트를 입력 받아 다음 토큰을 하나씩 예측하는 구조다.

- **Embedding Layer**: 텍스트 토큰을 고차원 벡터 d로 변환한다.
- **Residual Connection (잔차 연결)**: 각 레이어의 출력을 입력에 다시 더해주는 구조이다. x + sublayer(x) 이는 기울기 소실 vanishing gradient를 방지하고 깊은 층에서도 정보가 유실되지 않게한다.
- **Layer Normalization**: 학습의 안정성을 위해 각 층의 출력을 정규화한다. 최근 모델 효율성을 위해 입력을 먼저 정규화하는 Pre-Norm 방식과 RMSNorm을 주로 사용한다.
- **Feed-Forward Network (FNN)**: Attention 이후 각 토큰 벡터를 독립적으로 비선형 변환하느 2층 mlp 구조다. 보통 ReLU대신 SwiGLU 활성화 함수를 사용해 표현력을 높인다.

### Multi-Head Self-Attention (MHSA) 메커니즘

모델이 문맥 내에서 어떤 정보에 집중할지를 결정하는 핵심 수학적 연산이다.

- Q, K, V 각각 쿼리 키 밸류인데, 입력 벡터 x에 각각의 가중치 행렬 wq, wk, wv를 곱해 생성한다

$$Attention(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d_k}}\right)V$$

- $QK^T$: 토큰 간의 유사도(내적)를 계산.
- $\sqrt{d_k}$: 차원이 커질수록 내적값이 커져 Softmax의 기울기가 완만해지는 것을 방지하는 Scaling 인자.

### Positional Encoding

Transformer는 순환 구조 RNN가 없으므로 토큰의 위치 정보를 명시적으로 주어야한다.

RoPE: 절대적 위치 대신 두 토큰 사이의 상대적 거리를 복소수 회전 행렬을 통해 주입한다.

학습시 본것보다 더 긴문장에 대해서도 외삽 능력이 뛰어나며, 거리 감쇠 성질을 가져 멀리 있는 단어보다 가까운 단어의 관계를 더 잘 포착한다.

### Inference Mechanics & Optimization

추론시 성능과 자원 효율을 결정하는 물리적 메커니즘으로

- **KV Caching:** 디코딩 단계에서 t 번째 토큰을 생성할때 1 ~ t - 1까지 계산된 k, v 벡터를 메모리에 미리 저장해두느 ㄴ기법이다. 이를통해 매 스텝마다 전체 문장을 다시 연산하지 않고 새로 추가된 토큰 연산만한다.
- **Decoding Strategies:** 
  - Greedy Search: 가장 확률 높은 토큰만 선택
  - Nucleus Sampling (Top-p): 누적 확률이 p인 후보군 내에서 샘플링하여 문장의 다양성을 확보
- **Quantization**: 가중치 파라미터의 정밀도를 낮추어 메모리 점유율을 줄이는 기법이다. FP16 -> INT8/INT4

### 코드 예시

Transformer Block & Attention을 구현해보면 아래처럼 할 수 있다.

```py
import torch
import torch.nn as nn
import math

class MultiHeadAttention(nn.Module):
    def __init__(self, d_model, num_heads):
        super().__init__()
        self.num_heads = num_heads
        self.d_k = d_model // num_heads
        
        # WQ, WK, WV 가중치 행렬
        self.W_q = nn.Linear(d_model, d_model)
        self.W_k = nn.Linear(d_model, d_model)
        self.W_v = nn.Linear(d_model, d_model)
        self.W_o = nn.Linear(d_model, d_model)

    def forward(self, x, mask=None):
        batch_size, seq_len, d_model = x.size()
        
        # 1. 선형 변환 및 Head 분리
        # (batch, seq_len, num_heads, d_k) -> (batch, num_heads, seq_len, d_k)
        q = self.W_q(x).view(batch_size, seq_len, self.num_heads, self.d_k).transpose(1, 2)
        k = self.W_k(x).view(batch_size, seq_len, self.num_heads, self.d_k).transpose(1, 2)
        v = self.W_v(x).view(batch_size, seq_len, self.num_heads, self.d_k).transpose(1, 2)

        # 2. Scaled Dot-Product Attention 계산
        # $attn\_scores = \frac{QK^T}{\sqrt{d_k}}$
        attn_scores = torch.matmul(q, k.transpose(-2, -1)) / math.sqrt(self.d_k)
        
        if mask is not None:
            attn_scores = attn_scores.masked_fill(mask == 0, -1e9)
            
        attn_weights = torch.softmax(attn_scores, dim=-1)
        
        # 3. Value와 곱함
        output = torch.matmul(attn_weights, v)
        
        # 4. 원래 차원으로 복구
        output = output.transpose(1, 2).contiguous().view(batch_size, seq_len, d_model)
        return self.W_o(output)
```