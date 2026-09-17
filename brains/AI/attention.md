# Self-Attention, Multi-Head Attention

어텐션은 인코더와 디코더 사이의 병목을 해결하고 번역 품질을 크게 높였다.

하지만 뼈대는 여전히 **RNN**이였고 이는 딥러닝 아키텍처 관점에서 두 가지 치명적인 한계가 남아있다.

- **병렬 처리 불가 (속도 문제):** RNN은 단어를 시계열 순서대로 (1 2 3 ...) 읽어야한다. 앞 단어 연산이 끝나야 다음 단어를 처리할 수 있으므로, 아무리 좋은 gpu를 써도 동시 연산 (병렬 처리)을 제대로 할 수 없다.
- **여전히 먼 거리 (참조 문제)**: 어텐션을 쓴다고 해도 문장 첫 단어와 끝 단어의 연관성을 파악하려면 여전히 그 사이의 수 많은 타임스텝(단어들)을 물리적으로 거쳐야만 했다.

해결책으로는 "단어를 순서대로 하나씩 읽는 RNN 구조를 아예 버리고, 입력된 모든 단어가 서로 한번에 쳐다보며 관계를 파악하게 만들자" 라는 아이디어로 시작했다.

이를 트랜스포머의 개념중 하나인 **Self-Attention**의 등장 배경이라 볼 수 있다.

<br>

## Query, Key, Value (Q, K, V)

셀프 어텐션은 문장 내부의 단어들이 서로 어떤 관계를 맺고있는지 스스로 학습한다.

이를 위해 입력된 각 단어 벡터를 세 가지 다른 역할로 복제 (선형 변환)한다. 데이터베이스 검색 시스템에 비유하면 직관적이다.

- **Query (Q, 질문/검색어)**: 나는 어떤 정보가 필요한가.
  - 문장에서 it(그것) 이라고 하는 단어가 내가 가리키는 명사가 무엇인지를 기준 세우는 역할이다.
- **Key (K, 키워드/제목)**: 나는 어떤 특징을 가지고 있는가
  - 문장에서 animal 이라는 단어가 나는 동물이고 명사야 라는 이름표를 달고있는 것이다.
- **Value (V, 실제 내용/값)**: 내가 진짜 전달하려는 의미는 무엇인가?

### 작동 원리

1. 특정 단어의 Q를 문장 내 모든 단어 K와 Dot-product 하여 얼마나 관련이 있는지 어텐션 스코어 즉 유사도 점수를 구한다.
2. 이 점수를 softmax 함수에 통과시켜 합이 1이되는 가중치로 만든다.
3. 가중치(확률)이 높을수록 해당 단어의 V를 더 많이 가져와서 현재 단어의 의미를 문맥에 맞게 새롭게 업데이트(가중합)한다.

<br>

## Multi-Head Attention

단어 간의 관계를 파악할 때 한 가지 기준만 있으면 놓치는 문맥이 생긴다.

어떤 단어는 주어-동사 같은 문법적 관계가 중요할 수 있고, 어떤 단어는 문장의 감정선과 연결될 수 있다.

- **개념:** 모델의 전체 임베딩 차원 (512 차원)을 통째로 연산하지 않고, 여러개 예를들면 8개씩 독립적인 헤드로 쪼개낸다.
- **효과:** 8개의 헤드가 마치 8명의 서로 다른 전문가처럼 문장을 다른 관점에서 문법, 의미, 지시대명사등 분석하고 이후 각 헤드가 분석한 결과를 하나로 이어붙여 훨씬 입체적이고 풍부한 문맥을 파악해낸다.

### Example

실제 입력 단어 벡터 x에서 어떻게 Q, K, V가 파생되어 문맥을 생성하는지 텐서의 흐름을 보면

```py
import torch
import torch.nn as nn
import torch.nn.functional as F
import math

class SelfAttention(nn.Module):
    def __init__(self, embed_dim):
        super().__init__()
        # 하나의 입력 벡터에서 Q, K, V를 각각 뽑아내기 위한 선형 계층 (가중치 행렬)
        self.query = nn.Linear(embed_dim, embed_dim)
        self.key   = nn.Linear(embed_dim, embed_dim)
        self.value = nn.Linear(embed_dim, embed_dim)
        
        self.embed_dim = embed_dim

    def forward(self, x):
        # x: 입력 문장 텐서 [Batch_size, Seq_len, Embed_dim]
        
        # 1. 각 단어 벡터를 Q, K, V 공간으로 투영(Projection)
        Q = self.query(x)
        K = self.key(x)
        V = self.value(x)
        
        # 2. 어텐션 스코어 계산 (Q와 K의 내적)
        # K를 전치(Transpose)하여 행렬 곱을 수행 -> 단어들 간의 모든 쌍(Pair) 유사도 계산
        scores = torch.matmul(Q, K.transpose(-2, -1))
        
        # 3. 스케일링 (수학적 안정성 및 기울기 소실 방지를 위해 차원 수의 제곱근으로 나눔)
        scores = scores / math.sqrt(self.embed_dim)
        
        # 4. Softmax로 어텐션 가중치(분포) 산출
        attn_weights = F.softmax(scores, dim=-1)
        
        # 5. 가중치와 V를 행렬 곱하여 최종 문맥 벡터 도출
        context_vector = torch.matmul(attn_weights, V)
        
        return context_vector, attn_weights
```

이처럼 트랜스포머는 RNN의 순차적 연결고리를 없애고 오직 행렬 곱색 matrix multiplication만 으로 문장 전체의 단어 관계를 한 번에 계산한다.

덕분에 gpu의 병렬 처리 능력을 100퍼센트 활용할 수 있게되어 압도적인 학습속도와 스케일업이 가능해졌다.