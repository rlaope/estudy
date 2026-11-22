# GDN(Gated DeltaNet)

GDN을 한줄로 요약하면 **고정 크기 행렬 하나를 메모리로 쓰는 RNN** 인데, 그 메모리를 매 토큰마다 덮어쓰기 + 서서히 잊기 방식으로 갱신하는 레이어다.

위의 문장을 이해하기 위해 아래 순서로 따라가볼 것이다

1. Softmax attention -> Linear attention: 왜 행렬 하나 state로 요약할 수 있는가
2. Linear attention의 문제: 계속 더하기만 해서 메모리 오염
3. 해결책의 두 갈래
   1. **Gating (감쇠):** 오래된 정보를 통째로 흐리게함 -> Mamba2, GLA, RetNet
   2. **Delta rule (덮어쓰기):** 같은 key에 대한 기존 값을 지우고 새 값을 씀 -> DeltaNet
4. 둘을 합친 것 -> Gated DeltaNet
5. 학습은 어떻게 병렬화 하는가 (chunkwise, WY/UT transform)
6. 실제 모델에서 왜 전부 GDN으로 안쓰고 attention과 3:1로 섞는지
7. 서빙할때 kv cache 대신 state를 다루면서 생기는 문제들

수식읽는법은 아래 표를 확인해보자 

| 기호 | 읽는 법 | 뜻 |
|---|---|---|
| q_t, k_t, v_t | t번째 토큰의 query, key, value | 길이 128 정도의 숫자 목록(벡터). 아래 첨자 t는 "몇 번째 토큰인가" |
| o_t | t번째 출력 | 그 레이어가 t번째 토큰에 대해 내놓는 결과 벡터 |
| S_t | t번째까지의 state | 지금까지 본 내용을 요약해 둔 표(행렬). 이 노트의 주인공 |
| S_{t-1} | 바로 전 state | 토큰 하나 전 시점의 표 |
| qᵀk (q^\top k) | q와 k의 내적 | 두 벡터가 얼마나 비슷한지 나타내는 숫자 하나 |
| v kᵀ (v k^\top) | v와 k의 외적 | 두 벡터로 표 하나를 만드는 연산. "k 주소에 v를 적는 메모" 한 장 |
| S q | 표에 벡터 곱하기 | "q 주소로 표를 읽어서 값을 꺼낸다" |
| Σ (\sum) | 다 더하기 | Σ_{i≤t}는 "1번부터 t번 토큰까지 전부 더한다" |
| α (alpha) | 감쇠 | 0~1 사이 숫자. 기존 표에 곱해서 "얼마나 남길지" 정함. 0.9면 90%만 남김 |
| β (beta) | 쓰기 강도 | 0~1 사이 숫자. 새 정보를 "얼마나 세게 덮어쓸지" 정함 |
| I | 단위행렬 | 곱해도 아무것도 안 바뀌는 표. 숫자 1에 해당 |
| ∈ ℝ^(a×b) | ~는 a×b 크기 | "실수 a×b개짜리 표"라는 뜻 |
| O(1), O(n) | 빅오 | 비용이 길이와 상관없음(O(1)) / 길이에 비례(O(n)) |
 
예를들어 linear attention의 핵심 식 

$$S_t = S_{t-1} + v_t k_t^\top$$

은 말로 하면 **지금 표 = 바로 전 표 + 이번 토큰의 메모 한장**이다 뒤에 나오는 GDN식도 결국 전 표를 조금 흐리게(α) 하고, 이번 key 자리에 있던 옛 값을 지우고(β) 새값을 적는다 를 기호로 쓴 것 뿐이다.

<br>

## Linear attention

### softmax를 빼면 생기는 일

일반 attention의 t번째 출력은 다음과 같다

$$o_t = \sum_{i \le t} \frac{\exp(q_t^\top k_i)}{\sum_j \exp(q_t^\top k_j)} v_i$$

여기서 softmax(정규화 포함)를 걷어내고 그냥 내적만 쓰게 되면

$$o_t = \sum_{i \le t} (q_t^\top k_i)\, v_i = \Big(\sum_{i \le t} v_i k_i^\top\Big) q_t = S_t\, q_t$$

괄호 안에 있는 덩어리를 S_t 라고 부르겠다.

S_t는 "t번째 토큰까지 본 내용을 전부 합쳐둔 표(행렬)"이고 크기는 세로 d_v칸 x 가로 d_k 칸이다. (d_v 는 value 벡터 길이, d_k는 key 벡터길이로 보통 128) 원래 식에 쓰여있던 `S_t ∈ ℝ^(d_v × d_k)` 는 "S_t는 실수가 d_v x d_x개 들어있다는 표다" 라는 뜨싱고 이 문장과 같은 말을 기호로 줄여 쓴 것이다

이렇게 묶으면 새 토큰이 올때마다 표에 v_t k_tᵀ 를 더하기만 된다는 아래 식이 나온다.

$$S_t = S_{t-1} + v_t k_t^\top, \qquad o_t = S_t q_t$$

이게 linear attention으로 과거 토큰의 정보가 가 $d_v \times d_k$ 크기 행렬 하나에 들어간다 그래서 디코드시 메모리는 시퀀스 길이와 상관없이 O(1) 연산도 토큰당 O(d²) 이다.

### Fast weight 관점에서

$S_t$를 key를 넣으면 value가 나오는 연상 메모리(associative memory)로 보면 이해가 쉬운데 $v k^\top$을 더하는 건 "k라는 주소에 v를 기록"하는 것이고 $S q$는 "q라는 주소로 읽기"이다.

Schlag et al. (2021)이 이 관점을 정리했고, 이후 DeltaNet 계열은 전부 이 시각으로 설명된다.

### 문제점

계속 더하기만 하니 key들이 서로 직교하지 않으면(d_k보다 토큰 수가 많아지면 당연히 직교할 수 없다.) 같은 주소 근처에 값이 계속 겹쳐 싸힝고 읽을때 잡음이 섞이게 된다.

그래서 순수 linear attention은 언어 모델링 품질 특히 in-context retrieval이 softmax attention보다 확실히 떨어졌다.

## 해결 방향 1: Gating (감쇠)

가잔 단순한 방법은 메 step마다 메모리를 조금씩 줄이는 것이다

$$S_t = \alpha_t S_{t-1} + v_t k_t^\top, \qquad \alpha_t \in (0, 1)$$

- $\alpha$가 상수면 RetNet 스타일
- $\alpha_t$가 입력에 따라 바뀌는 스칼라면 Mamba2
- $\alpha_t$가 key 차원별 벡터면 GLA(Gated Linear Attention)

장점은 이제 문맥이 바뀌었으니 다 잊어라 를 빠르게 할 수 있다는 점이다 단점은 **선택적으로 잊지 못한다**라는 점이다.

감쇠 메모리는 전체가 균일하게 (혹은 채널 단위로) 걸리기 때문에 특정 key에 해당하는 정보만 골라서 지우는 것은 못한다.

## 해결 방향2: Delta Rule (덮어쓰기)

DeltaNet의 갱신 식이다.

$$S_t = S_{t-1} - \beta_t (S_{t-1} k_t - v_t) k_t^\top = S_{t-1}(I - \beta_t k_t k_t^\top) + \beta_t v_t k_t^\top$$

1. $S_{t-1} k_t$: 지금 key로 메모리를 읽으면 나오는 기존 값 $v^{old}$
2. $v_t - v^{old}$: 새로 쓰려는 값과 기존 값의 차이(delta)
3. 그 차이에 $\beta_t$(쓰기 강도, 0~1)를 곱해서 그 주소에 더함

즉 $\beta_t = 1$이고 $k_t$가 단위 벡터면 k 방향에 있던 옛 값을 완전히 지우고 새 값으로 바꾼다 1번의 계속 싸힝기만 하는 문제를 정면으로 해결한다.

### Online regression으로 보기 (중요_

손실 $\mathcal{L}(S) = \frac{1}{2}\|S k_t - v_t\|^2$ 을 생각하면 이 손실에 대해 학습률 $\beta_t$로 SGD 한 스텝을 밟은 겨로가가 정확히 위의 식이다

- Linear attention: 손실이 $-\langle S k_t, v_t \rangle$ 형태 (상관만 키움)
- DeltaNet: 손실이 회귀 오차 (틀린걸 고침)

이 관점후 나오는 TTT(Test-Time Training), Titans, 그리고 각종 변형들이 이해하는 공통 틀이라서 꼭 익혀두는 게좋다.

**레이어 하나 = 추론중에 작은 선형 회귀 모델을 온라인 학습하는 것**이라고 생각하면 된다.

### 3.3 $(I - \beta k k^\top)$의 의미

k가 정규화 되어있으면 $I - \beta k k^\top$ 은 일반화된 Householder행렬이다 고유값은 k 방향으로 1 - $1-\beta$, 나머지 방향으로 1이다 그래서

- k 방향 성분만 줄어들고 나머지는 그대로 두는 선택적 지우기
- 이 때문에 q, k에 L2 정규화기 필수다 정규화를 안하면 고유값이 범위를 벗어나 state가 발산할 수 있기 때문이다.

한가지 더: $\beta$ 범위를 (0, 2)로 넓히면 고유값이 음수가 될 수 있고, 이렇게 하면 parity 같은 state tracking 문제를 풀 수 있게 된다는 이론 결과가 있다. razzi et al., ICLR 2025). 표현력 논의에서 자주 나오니 알아두는게 좋다.

### DeltaNet의 단점

지우기를 하나의 key 방향으로만 해야해서 문맥이 확 바뀌어 메모리 전체를 비우고 싶을때 rank-1 을 수십번 반복해야 겨우 비워지니 빠르게 잊는 능력이 부족하다.

<br>

## GatedDeltaNet = Gating + Deltarule

$$S_t = S_{t-1}\Big(\alpha_t (I - \beta_t k_t k_t^\top)\Big) + \beta_t v_t k_t^\top, \qquad o_t = S_t q_t$$

$\alpha_t \in (0,1)$: 입력 의존 스칼라 감쇠 (Mamba2에서 가져옴). 전체를 빠르게 비울 때 사용
$\beta_t \in (0,1)$: 쓰기 강도. 특정 key만 정밀하게 덮어쓸 때 사용

둘이 역할을 나눠 가진다고 보면 된다.

논문 제목이 "Improving Mamba2 with Delta Rule"인 경우가 이거다.

저자는 Songlin yang, Jan Kautz, Ali Hatamizadh 이고 ICLR 2025에 실렸다

### 구현에서 실제로 쓰는 순서

수식을 그대로 구현하지 않고 보통 이렇게 푼다(flash-linear-attention, vLLM 모두 구현 비슷)

```
S = S * exp(g_t)                  # 1) 먼저 감쇠
v_old = S^T k_t                   # 2) 감쇠된 메모리에서 기존 값 읽기
delta = beta_t * (v_t - v_old)    # 3) 차이 계산
S = S + k_t ⊗ delta               # 4) rank-1 갱신
o_t = S^T q_t                     # 5) 읽기
```

여기서 state를 $d_k \times d_v$로 두느냐 $d_v \times d_k$로 두느냐는 구현마다 다르다. 논문, fla, vLLM 코드를 오가며 읽을때 전치 방향이 헷갈리는 일이 많으니 shape부터 확인하는 습관을 들이면 좋다/

### 레퍼런스 코드 (학습용이고 느림)

```py
import torch
import torch.nn.functional as F

def gdn_recurrent_ref(q, k, v, g, beta, S0=None):
    """
    q, k: [B, H, T, Dk]   (L2 정규화된 상태로 들어온다고 가정)
    v:    [B, H, T, Dv]
    g:    [B, H, T]       log 감쇠 (음수). alpha = exp(g)
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

이 함수를 `fla.ops.gated_delta_rule.chunk_gated_delta_rule` 결과와 비교해보면 오차가 얼마나 나는지 fp32/bf16에 따라 어떻게 달라지는지 밑에서 정밀도 이야기할때 더 와닿을 것이다

<br>

## 학습 병렬화: chunkwise 알고리즘

여기가 GDN 공부에서 가장 어려운 부분이고 동시에 왜 이제 실용화됐냐에 대한 답이다

### 문제

위 점화식은 토큰 단위 순차 연산이라 학습시 gpu를 쓰지 못한다. Linear attention은 $S_t$가 단순 누적합이라 병렬화가 쉬웠지만 delta rule은 $S_{t-1}$에 행렬 $(I - \beta k k^\top)$ 를 곱하기 때문에 단순 누적합이 아니다.

### 핵심 아이디어

시퀀스 길이를 C(보통 64짜리 청크로 나눈다)

- **청크 사이:** state를 순차적으로 넘긴다. 청크 개수 L/C 번만 순차라 괜찮다.
- **청크 내부:** 행렬곱으로 한 번에 계산한다. 여기서 텐서코어를 쓴다.

청크 내부를 행렬곱으로 만들려면 Householder 행렬 C개곱 $\prod (I - \beta_i k_i k_i^\top)$을 압축된 형태로 표현해야 하는데, 이때 쓰는 것이 WY representation

$$\prod_{i=1}^{C}(I - \beta_i k_i k_i^\top) = I - \sum_{i=1}^{C} w_i k_i^\top \quad \text{(꼴로 표현 가능)}$$

$w_i$는 점화식으로 구할 수 있고, 이걸 행렬로 묶으면

$$T = \big(I + \mathrm{tril}(\mathrm{diag}(\beta) K K^\top, -1)\big)^{-1} \mathrm{diag}(\beta), \quad W = TK, \quad U = TV$$

처럼 하삼각 행렬 역행렬 한 번으로 정리된다 UT transform. 그 다음은 일반 행렬곱이다

GDN에서는 여기에 감쇠 $\alpha$가 붙으므로, 청크 내 누적 감쇠 $\gamma_t = \prod_{i \le t} \alpha_i$를 이용한 마스크 $\Gamma_{ij} = \gamma_i / \gamma_j$를 곱해 주는 형태로 확장된다.

복잡도는 대략 O(L·C·d + L·d²)이고, 시퀀스 길이에 선형이다.

### prefill과 decode는 커널이 다름

- **prefill**: chunk(`chunk_gated_delta_rule`). 연산량이 많아 compute bound
- **decode**: 토큰 하나씩 갱신하는 fused recurrent 커널 (`fused_recurrent_gated_delta_rule`) 매 토큰마다 state 전체를 HBM에서 읽고 다시 써야해서 memory-bound

서빙 엔진 코드를 볼 때 이 두 경로가 따로 있다는 걸 알고 보면 더 쉽다.

<br>

## 실제 GDN 레이어에서는 수식 > 부품 

논문 수식만보고 코드를열면 이게 모야 싶은게 많은데 Qwen3-Next / Qwen3.5 기준으로 GDN 블록에 들어가는 부품을 정리하면 아래와 같다.

| 부품 | 역할 | 비고 |
|---|---|---|
| q, k, v projection | 입력을 q, k, v로 | k/q head 수 < v head 수 (GQA와 반대 방향의 비대칭) |
| Short causal conv1d (kernel 4) + SiLU | 바로 앞 몇 토큰 정보를 섞음 | 국소 패턴 담당. 서빙 시 conv state도 따로 저장해야 함 |
| q, k L2 norm | delta rule 안정성 | 3.3절 참고 |
| $\alpha_t$ 계산 | `g = -exp(A_log) * softplus(a + dt_bias)` | Mamba2와 동일한 parameterization |
| $\beta_t$ 계산 | `beta = sigmoid(b)` | |
| Output gate | 출력에 RMSNorm 후 `SiLU(z)` 곱함 | head별 norm |

Qwen3-Next-80B-A3B의 GDN 설정은 key head 16개, value head 32개, head dim 128, conv kernel 4이다. 정확한 값은 hf에 config.json `linear_*` 필드를 확인하자

### State 크기 직접 계산해보기

Qwen3-Next 기준으로

- GDN state: value head 32 x 128 x 128 x 4 byte(fp32) = **2 MiB / 레이어 / 시퀀스**
- Conv state: (16x128x2 + 32x128) 채널 x 3 x 2 byte ≈ 48 KiB / 레이어
- Full attention KV: KV head 2 x head dim 256 x (K, V) x 2 byte = **2KiB / 레이어 / 토큰**

그러면 GDN 레이어 하나의 state가 attention 레이어 하나 KV cache **약 1,024**토큰 분량이다.

모델 전체 GDN 36층 + attention 12층으로 보면 하이브리드는 시퀀스당 고정 약 72 MiB + 토큰당 24KiB이고 같은 구조를 전부 attention으로 만들면 토큰당 96KiB 계산해보면 약 1K 토큰 이하에서는 하이브리드가 메모리를 더 쓴다. 짧은 요청이 많은 워크로드에서는 linear attention이니까 뭊건 메모리 이득이라고 생각하면 틀린다. state를 bf16으로 두면 손익분기점은 절반으로 내려간다.

NVIDIA도 비슷한 계산을 공개했는데 SSM 레이어 하나의 FP32 state가 시퀀스당 4MiB이고 FP8 KV 토큰당 0.5 KiB라서 state하나가 attention KV cache 8K 토큰 분량과 같다고 정리했다.

<br>

## 왜 GDN만 쓰지않고 attention과 섞을까?

### 고정 크기 메모리 한계

state는 크기가 고정이므로 넣을 수 잇는 정보량에 상한이 있다. 긴 문서에서 정확한 문자열 하나를 찾아오는 류의 작업 (needle-in-a-haystack, 정확한 복사, 긴 리스트 lookup)은 원리적으로 불리하다. 반대로 GDN은 순서/상태 추적에 강한 귀납 편향이 있다.

그래서 현재 업계 표준 해법은 **GDN 3: full attention 1** 비율의 하이브리드다 attention이 정확한 회상, GDN이 압축된 맥락과 상태 누적을 나눠 맞는 구조다.

#### 주요 모델

| 모델 | 시기 | linear 레이어 | 섞는 attention | 비율 |
|---|---|---|---|---|
| Qwen3-Next-80B-A3B | 2025.09 | GDN | Gated Attention (GQA) | 3:1, 48층 중 36층이 GDN |
| Kimi Linear 48B-A3B | 2025.10 | KDA (GDN 변형) | MLA | 3:1 |
| Qwen3.5-397B-A17B | 2026.02 | GDN | Gated Attention (GQA) | 3:1, 60층 중 45층이 GDN |
| Olmo Hybrid 7B | 2026.03 | GDN | MHA | 3:1 (Olmo 3의 sliding window 층을 GDN으로 교체) |
| Kimi K3 | 2026.07 | KDA | MLA | Kimi Linear와 같은 비율로 추정 |

Qwen 쪽은 Qwen3.5 이후 Qwen3.6, Qwen3.8 까지 같은 아키텍처 (`Qwen3_5ForConditionalGeneration`)를 공유하고 있다. 즉 GDN 하이브리드는 이제 실험이 아니라 qwen 메인라인의 기본 구조다.

### KDA(Kimi Delta Attention)은 무엇이 다른가

GDN의 감쇠 $\alpha_t$는 head당 스칼라 하나. KDA는 이걸 key 차원별 벡터(채널 단위 감쇠)로 바꿨다.

GLA가 linear attention에서 했던걸 GDN에 한 샘이다. 대신 일반적인 DPLR(diagonal-plus-low-rank) 형태로 그대로 두면 chunk 계산이 비싸지므로 연산량을 줄인 전용 chunk 알고리즘을 같이 제시했다.

Moonshot은 공정 비교에서 full attention(MLA)를 이겼다고 주장했고, kv cache는 75퍼 줄이고 1M context에서 decode 처리량은 최대 6배라고 보고했다. 커널은 fla(`fla/ops/kda`)와 CUTLASS 기반 FlashKDA로 공개되어 있다.

### 반대사례: MiniMax M2

MiniMax는 M1/01 에서 Lightning Attention(linear) 하이브리드를 썼는데, M2에서는 다시 **full attention으로 돌아갔다** 사전 학습 리드가 직접 이유를 정리한 글이 있는데 요지는 다음과 같다.

- 작은 규모와 공개 리더보드에서는 하이브리드가 full attention 만큼 좋아보였지만 규모를 키우고 복잡한 멀티홉/에이전트 작업으로 가니 차이가 드러났다.
- 진짜 병목은 아키텍처보다 평가 체계의 한계 (기존 벤치로는 차이를 못잡음)
- linear/sparse attention은 인프라가 덜 성숙했다: 저정밀도 state, prefix caching, speculative decoding 문제가 남아있다.

이에 대해 Songlin Yang은 MiniMax의 실험 방식이 linear attention을 과소평가했다고 반론했다 양쪽 다 읽오보고 판단하는걸 권장. 이 논쟁은 linear attention 이 품질 손실 없이 공짜 점심인가에 대한현재 업계 온도를 보여줌

### 또 다른 갈래: Sparse/Compressed attention (deepseek)

DeepSeek는 linear attention 대신 kv를 압축하거나 일부만 골라 보는 방향을 택했다 V3.2에서 DSA(학습된 indexer로 top-k 토큰 선택)를 넣었고 2026년 4월 V4에는 CSA(Compressed Sparse Attention)와 HCA(Heavilty Compressed Attention)을 섞은 하이브리드 1M 컨텍스트를 지원한다. NVIDIA 정리에 따르면 V3.2 대비 토큰당 추론 FLOPs 73%, KV cache 메모리 90%를 줄였다.

정리하면 2026년 현재 긴 컨텍스트 효율화는 크게 세 진영

- **Linear(GDN/KDA) 하이브리드: Qwen, Moonshot, Ai2**
- **Sparse/Compressed attetnion: DeepSeek** 
- **Full attention 유지 + 다른데서 효율화: MiniMax M2**

<br>

## 서빙 관점

서빙 엔진은 원래 토큰마다 늘어나는 KV cache를 페이지 단위로 관리하도록 설계되어있다 (PagedAttention).

GDN 하이브리드는 여기에 **요청마다 고정 크기인 state**라는 전혀 다른 종류의 메모리를 끼워 넣는다. 그래서 생기는 문제를 하나씩 보겠다.

### 메모리 관리: hybrid kv cache manager

vLLM V1은 레이어를 종류별로 그룹으로 나누고(full attention gropu, GDN/Mamba 그룹), 각각 관리하는 hybrid KV cache manager를 쓴다. 재미있는 점은 **attention 레이어의 논리 block size를 자동으로 조정해서 attention 페이지 하나와 GDN state 하나가 같은 바이트 크기를 차지하도록 맞춘다는 것이다.** 그래야 하나의 메모리 풀에서 같이 할당할 수 있다.

### Prefix Caching이 어려워짐

kv cache는 토큰 위치별로 저장되니까 공통 prefix가 있으면 그 부분 블록만 재사용하면 된다. GDN state는 **지금까지 전부를 요약한 값 하나**라서 중간 위치 state를 꺼낼 방법이 없다. 재사용하려면 prefix마다 state 스냅샷을 따로 저장해야하고 촘촘히 저장할수록 메모리가 다시 시퀀스에 비례해 늘어난다.

현재 엔진들의 대응은 다음과 같다.

- 일정 간격, 혹은 블록 경계에 정렬해서만 state를 저장 (vLLM `--mamba-cache-mode align`)
- 프롬프트/턴 경계에서 저장 (에이전트처럼 턴 구조가 뚜렷한 워크로드에 유리)
- 캐시 히트 지점 이후 짧은 꼬리만 다시 계산하는 방식 등 연구가 계속 나오는중

MiniMax가 M2 글에서 실서비스 대화는 캐시 히트율이 높아서 새 아키텍처는 이걸 잘 처리해야한다라고 지적한게 바로 이 부분이다.

### Speculative decoding이 어려워진다

일반 attention에서 draft 토큰 k개를 검증한 뒤 거절된 토큰의 kv만 버리면 된다. GDN state는 draft까지 이미 섞여서 갱신되어버리기 때문에 부분 롤백이 안된다. 해법은 크게 두가지

- 검증 중 draft 토큰마다 중간 state를 스냅샷해두고 수락된 위치의 것을 선택 (메모리/대역폭 비용 큼)
- state를 갱신하지 않고 draft 입력만 버퍼에 쌓아 두었다가 수락된 만큼만 다시 반영 (NVIDIA ReplaySSM 같은 방식)

Qwen3.5처럼 MTP(multi-token prediction) 헤드를 기본 탑재한 모델이 많아서 이 문제는 실 서비스에 직접 영향을 준다 2026년 관련 논문 tree 구조 검증, MTP 전용 설계등이 많이 나왔으니 서빙쪽에 관심이 있으면 깊게 파봐도 된다.

### Decode는 memory-bound, 그리고 정밀도

decode시 GDN은 매 토큰마다 state전체(Qwen3-Next 기준 레이어당 2MiB)를 읽고 쓴다. 배치가 작으면 attention보다 GDN레이어가 latency 병목이 되는 구간도 있다.

정밀도도 문제인데 state는 계속 누적 갱신되기에 오차가 쌓인다 kv cache는 FP8로 줄이는게 흔하지만 GDN state는 기본적으로 fp32로 두는 경우가 많고 낮추면 품질이 흔들린다. MiniMax도 linear attention은 저정밀도 state저장에 훨신 민감하다를 첫번재 과제로 꼽았다

### 그 밖에도

- **Chunked prefill:** 오히려 자연스럽고 청크 끝 state만 넘기면 된다.
- **Tensor parallel:** head 단위로 state를 나눈다. k head수와 v head수가 다르므로 TP 크기 제약을 확인하자
- **P/D disagrgregation:** prefill 노드에서 decode 노드로 kv 뿐만 아니라 GDN state와 conv state도 보내야한다 대신 이쪽은 시퀀스 길이와 무관한 고정 크기라 긴 컨텍스트에서는 전송량이 줄어든다.
- **CUDA graph:** decode 경로의 state 버퍼 인덱싱 방식 때문에 엔진마다 구현이 꽤 다르다.


### 9. 최근 연구 흐름 (더 보고 싶으시면)
1. 표현력 이론: 음수 고유값으로 state tracking 가능 (Grazzi et al.), Householder 곱을 여러 번 하는 DeltaProduct, KDA 표현력 분석(NeurIPS 2026)
2. 게이트 세분화: Gated DeltaNet-2 — GDN과 KDA 모두 "지우기와 쓰기가 같은 스칼라 β에 묶여 있다"는 한계를 지적하고, 채널 단위 erase gate와 write gate를 분리
3. Mamba-3: SSM 쪽에서도 계속 개선 중이고, 위 논문들에서 비교 대상으로 같이 나온다
4. Olmo Hybrid: 완전 공개 모델로 "같은 레시피에서 attention만 바꾼" 통제 비교를 제공해서, 하이브리드 효과를 공정하게 보고 싶을 때 가장 좋은 자료입니다. 이론(표현력)과 실험(스케일링)을 같이 다룬다.