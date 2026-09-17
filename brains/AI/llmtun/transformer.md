# Transformer와 Causal LM 구조 이해

대규모 언어 모델 LLM의 근간을 이루는 Decoder-only Transformer 아키텍처는 시퀀스 데이터를 고차원 벡터 공간으로 매핑하고 Autoregression 방식으로 다음 토큰과 확률 분포를 계산하는 결정론적 연산 시스템이다.

본 문서에서는 자가회귀 언어 모델의 수학적 본질과 고유한 컴포넌트 구조가 모델의 연산 병목 및 가중치 업데이트에 미치는 영향을 로우레벨 메커니즘 중심으로 분석한다.

## LLM은 왜 다음 토큰 예측만으로 대화, 추론, 코드 생성이 가능한가?

수학적 관점에서 다음 토큰 예측은 조건부 확률 분포 $P(x_t | x_1, x_2, \dots, x_{t-1})$를 학습하는 과정이다.

매우 단순해보이는 이 단일 목적 함수 (Objective Function)가 고차원의 일반적인 인지능력으로 전이되는 이유는

**시퀀스 공간의 압축과 비선형 매니폴드 (Manifold) 학습에 있다.**

1. **세계적 표상(World Representation)의 압축:** 인터넷에 존재하는 방대한 텍스트 시퀀스(인간의 대화, 수학적 논리 전개, 컴파일 가능한 소스코드)의 다음 단어를 정확히 예측하려면 모델은 컨텍스트 내부의 통계적 정합성뿐만 아니라 그 텍스트를 생성한 기저의 규칙 (물리 법칙, 프로그래밍 문법, 논리적 인과관계)을 자신의 내부 파라미터 공간에 가중치 행렬의 형태로 압축하여 인코딩해야한다.
2. **컨텍스트 조건화(Context Conditioning):** 자가회귀 구조는 이전 단계의 출력이 다음 단계의 입력 변수로 결합되는 피드백 루프를 가진다. t번째 토큰을 예측할때 모델은 t - 1 개 토큰들의 상호작용 정보를 벡터 공간에서 고차원 비선형 변환 (Attention, FFN)을 통해 누적한다. 이 과정에서 컨텍스트 정보가 고정된 정적 벡터가 아니라, 유동적인 동적 서브스페이스(Sub-space)로 매핑되므로 지시문 수행(Instruction Following)이나 다단계 추론(In context Learning)같은 창발적(Emergent) 연산 능력이 발행된다.

<br>

## Decoder-only 아키텍처의 핵심 메커니즘과 병목

Decoder-only Transformer의 성능과 하드웨어 추론 속도는 가중치 행렬의 차원 구조 및 메모리 I/O 패턴에 의해 지배받는다.

> Casual LM은 이전 토큰만 보고 다음 토큰을 예측하도록 학습된 언어 모델이다. GPT, Llama, Qwen, Mistral은 모두 Casual LM

| 모델 유형                            | 대표 모델                            | 학습 목표        |
| -------------------------------- | -------------------------------- | ------------ |
| **Causal LM (Decoder-only)**     | GPT, Llama, Qwen, Mistral, Gemma | 다음 토큰 예측     |
| **Masked LM (Encoder-only)**     | BERT, RoBERTa, DeBERTa, ELECTRA  | 가려진 토큰 예측    |
| **Seq2Seq LM (Encoder-Decoder)** | T5, FLAN-T5, BART, UL2           | 입력 → 출력 생성   |
| **Prefix LM / 변형**               | GLM 계열, 일부 U-PaLM 실험             | Prefix 이후 생성 |


### Causal Self-Attention과 인과적 마스킹 (Casual Masking)

Casual Self-Attention은 미래의 토큰 정보가 현재 토큰의 예측 확률에 개입하지 못하도록 차단하는 인프라 가드다.

기존 BERT 같은 초기 모델은 문장 전체 (앞단어와 뒷단어 모두)를 동시에 보고 맥락을 파악했다.

다음 단어를 예측하여 대화를 이어나가야하는 모델(GPT)에서 미래 단어를 미리 볼수있게 두면 답지를 미리 보고 시험을 치는 꼴이 되어 학습이 성립하지 않는다.

미래의 토큰 위치를 행렬 연산 단계에서  $-\infty$ 값으로 덮어버리는 마스킹을 적용해 모델은 오직 '지금까지 나온 단어들로만' 다음 단어를 맞추도록 강제한다.

Query 행렬 $Q \in \mathbb{R}^{B \times H \times T \times d}$와 Key 행렬 $K \in \mathbb{R}^{B \times H \times T \times d}$의 행렬곱 연산 이후 Softmax를 취하기전에 상삼각 행렬 Upper Triangular Matrix 영역을  $-\infty$ (실제 연산 시에는 -1e9 또는 표현 가능한 가장 작은 음수)로 마스킹한다.

$$Score = \text{Softmax}\left(\frac{QK^T}{\sqrt{d}} + M\right)V, \quad M_{ij} = \begin{cases} 0 & \text{if } i \geq j \\ -\infty & \text{if } i < j \end{cases}$$

이 연산은 $O(T^2)$의 공간 및 시간 복잡도를 유발하므로, 시퀀스 길이 T가 길어질수록 GPU의 SRAM, 고대역폭 메모리 HBM간 대규모 데이터 전송 병목을 심화시킨다.

### 회전 위치 임베딩 RoPE: Rotary Position Embedding

Llama, Qwen, Mistral등 현대 Casual LM은 절대적 위치를 더해주는 방식 (Absolute PE) 대신, Query, Key 벡터 행렬의 채널 쌍을 2차원 복소수 평면 상의 회전 행렬 (Rotation Matrix)과 복소수 곱 연산을 하는 **RoPE를 채택한다.**

단어의 순서를 모델에게 알려주기 위해서 첫 번째 단어 벡터에는 1번 위치 벡터를, 두 번째 단어에서는 2번 위치 벡터를 고정값으로 더해주는 방식이 절대적 위치 임베딩이라는 기존 방식이다.

**여기서 발생한 문제는** 모델이 4,000개의 토큰 길이로 학습되었다면, 실제 서비스에서 8,000개짜리 긴 문장이 들어왔을 때 본 적 없는 8,000번 위치 벡터가 더해지면서 모델의 연산 지형이 완전히 망가진다.

절대적인 숫자를 더하는 대신 단어 벡터를 차원별로 특정 각도만큼 회전 (Rotary) 시키는 방식을 쓴다. 단어간의 상대적인 거리 (각도 차이)만 기억하기 때문에 문장 길이가 학습 범위를 넘어 길어지더라도 위치 관계를 유연하게 처리할 수 있다.

$$R_{\Theta, m}^d = \text{diag}\left(R_{\theta_1, m}, R_{\theta_2, m}, \dots, R_{\theta_{d/2}, m}\right), \quad R_{\theta_i, m} = \begin{pmatrix} \cos m\theta_i & -\sin m\theta_i \\ \sin m\theta_i & \cos m\theta_i \end{pmatrix}$$

RoPE는 토큰간의 거리가 멀어질수록 Attention Score의 상한선이 지수적으로 감소하는 수학적 유도 한계를 보장(Long-range Decay) 하므로, 모델이 고정된 컨텍스트 길이를 초과하더라도 상대적 위치관계의 연속성을 유연하게 보존할 수 있도록 만든다.

### SwiGLU 활성화 함수 기반 FFN (Feed-Forward Network)

기존 Vanilla Transformer의 ReLU/GELU 기반 FFN을 대체하여, 두 개의 선형 투영 (Linear Projection) 레이어의 출력을 Swish 함수로 게이팅 하는 SwiGLU 구조가 표준으로 안착했다.

기존에는 인공신경망에서 특정 값 이하를 0으로 쳐내는 ReLU, GELU 같은 단순한 일차원 활성화 함수를 사용해 데이터의 특징을 추출했다.

**발생한 문제는** LLM이 다루어야하는 코딩 문맥이나 복잡한 수학적 인과관계를 표현하기에는 함수의 형태가 너무 단순하여, 모델이 고차원 지식을 깊이 있기 학습하는데 한계가 있었다.

두 개의 가중치 행렬 연산 결과를 하나의 Gate로 쓰고있는 데이터로 써서 서로 곱해주는 Gating 구조를 SwiGLU는 결합했다. 유연한 곡선 형태를 가진 신경망을 구성함으로써, 모델이 복잡한 논리 구조를 더 잘 정밀하게 표현하게 되었다.

$$\text{SwiGLU}(x) = \left( \text{Swish}_{1}(xW_{gate}) \otimes xW_{up} \right) W_{down}$$

SwiGLU는 그라디언트 소실 문제를 완화하고 연산 차원의 비선형 표현 영역을 확장하여 파라미터 대비 손실 수렴 속도를 가속화하지만 세개의 가중치 행렬을 동시에 유지해야하므로 VRAM 가중치 적재 용량이 증가한다. ($W_{gate}, W_{up}, W_{down}$)

### RMSNorm(Root Mean Square Normalization)과 Pre-LN

훈련과 수리적 안정성을 확보하기 위해 LayerNorm의 평균 계산 파트를 완전히 소거하고 오직 제곱평균제곱근 RMS만을 기준으로 정규화하는 RMSNorm이 선호된다. 

기존에는 데이터의 평균과 분산을 모두 계산하여 값의 범위를 고르게 맞춘뒤 LayerNorm, 모든 연산이 끝난 출력단에 이 정규화 Post-LM을 배치했다.

문제는 모델의 파라미터가 수백억개로 커지자 매번 평균을 구하는 연산 자체가 하드웨어 GPU 병목을 유발했고, 뒤쪽 레이어에서 계산된 학습 신호가 앞쪽 레이어까지 전달되지 못하고 끊기는 현상이 발생했다.

계산 낭비인 평균을 과감히 생략하고 제곱평균제곱근RMS 만을 구하도록 하여 속도를 올린것이다. 동시에 이 정규화 블록을 연산 시작 직전 Pre-LN에 배치하여 학습 신호가 방해받지않고 하위 레이어까지 끊김없이 흐르도록 통로를 뚫어준 것

$$\text{RMSNorm}(x) = \frac{x}{\sqrt{\frac{1}{d}\sum_{i=1}^d x_i^2 + \epsilon}} \odot \gamma$$

이 정규화 블록을 Transformer 레이어 진입 직전에 배치하는 Pre-LN 구조를 결합함으로써 역전파시 그라디언트가 잔차연결 통로를 통해 손실 없이 입력 레이어까지 직접 흐르도록 보장해 수백억개 파라미터 스케일에서의 그레디언트 폭발을 차단한다.

### KV Cache의 하드웨어적 영향

자가회귀 추론 단계에서 이미 연산이 완료된 과거 토큰 key value 텐서를 HBM에 상주시켰다가 다음 스텝의 연산에 직접 재사용하는 기법이다.

이는 대형 행렬곱 연산을 백터-행렬 곱 연산으로 전환해 계산 오버헤드를 소거하지만 컨텍스트 윈도우가 커질수록 엄청난 VRMA 점유율을 차지해 OOM 유발인자가 된다.

기존에는 다음 단어 한개를 생성할때마다 처음 입력된 단어부터 방금 생성된거까지 확인했고 관계를 매번 처음부터 새로 개선했다.

대화가 길어져서 2000단어가 쌓여있으면 2001 번째 단어를 위해 2000개의 단어와 행렬 곱연산을 다시 시작해야하므로 글자가 한 자씩 나올때마다 속도가 느려진다.

과거 단어들이 가졌던 핵심 연산데이터 Key Value 텐서를 버리지않고 gpu vram에 미리 caching해둔뒤 다음 단어를 만들때 새로 추가된 단어 1개에 대한 연산만 수행하고 기존 캐시만 합치면 되기때문에 문장이 길어져도 실시간 대화 속도를 유지할 수 있다.

<br>

## 아래 코드 Llama 및 nanoGPT 아키텍처 사상을 기반으로 Causal Masking, RMSNorm, SwiGLU 연산 메커니즘을 pytorch 단일 프레임워크로 정밀 구현한 모듈 세트다.

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

| 아키텍처 계열 (Model Family) | 기본 위치 인코딩 및 기저 주파수 (RoPE Base Theta) | 정규화 레이어 유형 및 위치 (Normalization) | FFN 활성화 함수 및 차원 배율 (Activation & Hidden Expansion) | Attention 아키텍처 유형 (KV Cache Optimization) | 입력 대비 출력 연산 특성 및 병목 구간 (Bottleneck Profile) |
|-----------------------------|---------------------------------------------------|--------------------------------------------|-------------------------------------------------------------|--------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Llama 3 (8B / 70B) | RoPE (Theta = 500,000) | RMSNorm (Pre-LN 구조) | SwiGLU (Hidden Dim ≈ 3.5×d) | GQA (Grouped-Query Attention)<br>8개 Q당 1개 KV 쌍 매핑 | Memory-bound (Decode 페이즈)<br>초고거대 컨텍스트 영역 진입 시 높은 RoPE 주파수로 인한 정밀도 한계 제어 및 KV Cache 메모리 파편화 방어가 주 과제. |
| Qwen 2 (7B / 72B) | RoPE (Theta = 1,000,000) | RMSNorm (Pre-LN 구조) | SwiGLU (Hidden Dim ≈ 3.6×d) | GQA (Grouped-Query Attention)<br>소형 모델 라인까지 GQA 전면 도입 | Compute-bound / Memory-bound 교차<br>최대 128k 컨텍스트 확장을 지원하기 위해 RoPE 기저 주파수를 극단적으로 확장함에 따른 Attention 연산 커널 최적화 필수. |
| Mistral (7B v0.3) | RoPE (Theta = 100,000 / v0.3 개량) | RMSNorm (Pre-LN 구조) | SwiGLU (Hidden Dim ≈ 3.5×d) | SWA (Sliding Window Attention) + GQA<br>로컬 윈도우 크기 고정 제어 | I/O-bound 완화 구조<br>Sliding Window 기법을 통해 KV Cache의 물리적 상한선을 강제로 고정함으로써 고밀도 서빙 처리량(Throughput) 확보에 특화. |