# RNN에서 어텐션까지의 발전 과정

RNN은 텍스트와 같은 시계열(시퀀스) 데이터를 처리하기 위해서 고안된 신경망 구조다.

작동방식은 입력 시퀀스 스 $x_1, x_2, \dots, x_T$를 타임 스텝 단위로 순차적으로 처리한다.

**은닉 상태 갱신**으로 각 타임스텝 $t$에서 이전 단계의 은닉상태인 $h_{t-1}$과 현재 입력값 $x_t$를 받아 현재 상태 $h_t$를 계산한다.

결과적으로 마지막 타임스텝의 은닉승태 $h_T$에는 시퀀스 전체를 순차적으로 연산하며 누적된 정보가 담기게 된다.

## Seq2Seq

Seq2Seq는 두개의 RNN(Encoder and Decoder)을 결합하여 가변 길이의 입력 시퀀스를 다른 형태의 출력 시퀀스로 매핑하는 모델이다.

- **Encoder:** 입력 문장의 단어들을 순차적으로 연산한 후, 최종 타임스텝의 은닉 상태 $h_T$를 출력한다. 이를 전체 문장의 정보를 담은 고정 길이 컨텍스트 벡터 (Context Vector, C)로 사용한다.
- **Decoder:** 인코더가 넘겨준 단일 벡터 C를 초기 은닉 상태로 전달받아, 출력 시퀀스 $y_1, y_2, \dots, y_{T'}$를 순차적으로 생성한다.

### 순차 처리 및 압축 구조의 한계

이러한 RNN 기반 Seq2Seq 구조에는 두 가지 치명적인 수학적, 아키텍처적 결함을 가진다.

- **장기 의존성 (Long-term Dependnecy) 및 기울기 소실**: RNN은 타임스텝을 거칠때마다 동일한 가중치 행렬을 곱한다. BPTT(Backpropagation Through Time)시 역전파 거리가 길어지면 초기 입력에 대한 기울기가 0에 수렴하여 Vaninshing Gradient, 문장이 길어질수록 모델이 앞부분의 데이터를 유실하게된다.
- **고정 크기 병목 Fixed Size Bottleneck:** 입력 시퀀스의 길이가 10이든 100이든, 항상 차원이 고정된 단일 벡터 $C$에 모든 정보를 덮어쓰며 압축하므로 필연적으로 세부 정보 손실이 발생한다.

<br>

## Attention 메커니즘

어텐션은 Decoder가 출력 단어를 생성할 때마다 고정된 벡터 $C$ 하나에만 의존하는 병목을 제거한다.

대신 Encoder의 모든 타임 스텝 은닉 상태  $h_1, \dots, h_T$에 직접 접근하여 매 스텝마다 동적인 컨텍스트 벡터를 생성한다.

### 수학적 연산 과정

1. **스코어 계산:** 디코더의 현재 은닉 상태 $s_t$와 인코더의 각 상태 $h_i$를 내적(Dot-product)하여 각 단어 간의 연관성(유사도)을 구한다.

$$e_{ti} = s_t^\top h_i$$

2. **가중치 산출 softmax:** 연산도니 스코어를 0~1 사이의 확률값으로 변환하여, 현재 타임스텝에서 인코더의 어떤 상태  $h_i$에 가중치를 둘지 결정한다.

$$\alpha_{ti} = \frac{\exp(e_{ti})}{\sum_{k=1}^T \exp(e_{tk})}$$

3. **동적 컨텍스트 벡터 생성:** 인코더의 상태들에 가중치를 곱해 모두 더한(가중합) 새로운 벡터 $c_t$를 디코더의 입력(참조)으로 사용한다.

$$c_t = \sum_{i=1}^T \alpha_{ti} h_i$$

<br>

## Example

아래는 디코더가 인코더의 전체 은닉 상태 `encoder_outputs`를 매번 참조하여 동적 컨텍스트 벡터를 생성하는 어텐션 연산 코드다.

```py
import torch
import torch.nn as nn
import torch.nn.functional as F

class AttentionMechanism(nn.Module):
    def __init__(self):
        super().__init__()

    def forward(self, decoder_hidden, encoder_outputs):
        # decoder_hidden: [Batch, 1, Hidden_dim] (디코더의 현재 상태 s_t)
        # encoder_outputs: [Batch, Seq_len, Hidden_dim] (인코더의 전체 상태 h_1 ... h_T)

        # 1. 어텐션 스코어 계산 (내적 연산)
        # [Batch, 1, Hidden_dim] x [Batch, Hidden_dim, Seq_len] -> [Batch, 1, Seq_len]
        attn_scores = torch.bmm(decoder_hidden, encoder_outputs.transpose(1, 2))

        # 2. 어텐션 가중치 산출 (Softmax)
        attn_weights = F.softmax(attn_scores, dim=-1)

        # 3. 동적 컨텍스트 벡터 도출 (가중합)
        # [Batch, 1, Seq_len] x [Batch, Seq_len, Hidden_dim] -> [Batch, 1, Hidden_dim]
        context_vector = torch.bmm(attn_weights, encoder_outputs)

        return context_vector, attn_weights
```