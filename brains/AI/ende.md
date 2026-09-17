# Positional Encoding, Encoder-Decoder

셀프 어텐션을 통해 우리는 RNN의 순차적 굴레를 벗어나 단어들의 관계를 한 번에 별렬로 계산할 수 있게 되었다.

하지만 여기에서는 치명적인 부작용이 존재하게 되는데,

**순서 정보의 상실 Permutation Invariance**이다. 셀프 어텐션은 행렬 곱셈일 뿐이고, 모델 입장에서는 I love you, You love me를 입력받았을때, 단어의 위치를 알려주지 않으면 두 문장이 수학적으로 결괏값이 동일하게 나온다. 단어들이 줄을 서서 들어오는게 아니라 한 방에 우르르 들어오기 때문이다.

해결책의 방향은 "입력되는 단어 벡터(임베딩)에 너는 첫번째 단어야 너는 두번째 단어야 하고 위치 정보 (번호표)를 수학적으로 각인시키자"라는 것에서 나왔고, 이것이 포지셔널 인코딩 Postional Encoding이다.

<br>

## Positional Encoding

가장 단순한 학습 방법은 1,2,3... 처럼 정수 인덱스를 더해주는 것이지만, 문장이 길어지면 값이 무한정 커져 모델의 학습을 방해한다. 트랜스포머는 이를 해결하기 위한 Sine, Cosine 주기 함수를 사용한다.

아날로그 시계를 상상해보면 초침은 빨리돌고 분침은 중간, 시침은 천천히 도는데 이 세 바늘의 각도(위치)만 알면 우리는 정확히 몇시 몇분 몇초인지 알 수 있다. 이처럼 서로 다른 주기를 가진 파동들을 겹쳐서 각 위치마다 절대 겹치지 않는 고유한 수학적 지문(바코드)를 만들어 단어 벡터에 더해준다.

수학적 수식으로는 위치 $pos$와 임베딩 차원의 인덱스 i에 대해 포지셔널 인코딩은 다음과 같이 정의된다.

$$PE_{(pos, 2i)} = \sin\left(\frac{pos}{10000^{2i/d_{model}}}\right)$$

$$PE_{(pos, 2i+1)} = \cos\left(\frac{pos}{10000^{2i/d_{model}}}\right)$$

이렇게 생성된 위치 벡터는 원래의 단어 임베딩 벡터와 더해져 모델의 입력으로 들어간다.

<br>

## 전체 아키텍처: 인코더와 디코더의 조립

이제 번호표 PE(Positional Encoding)를 단 단어들이 트랜스포머 본체에 들어간다.

- **인코더**: 깊은 이해
  - `멀티 헤드 어텐션 -> 피드 포워드 신경망 FFN`의 층 layer를 여러번 반복한다.
  - 각 층을 지날때마다 Residual Connection과 Layer Normalization를 거친다. 결과적으로 인코더는 입력 문장의 모든 단어 관계가 완벽하게 계산된 풍부한 문맥 지도를 출력한다.
- **디코더**: 미래를 가린 채 번역
  - **마스크드 셀프 어텐션 Maksed Self-Attetion**: 디코더는 단어를 생성할 때 미래의 단어를 미리 훔쳐보면 안된다. 정답을 보고 푸는 꼴이니까, 그래서 현재 시점 이후 단어들과 어텐션 스코어를 가려버린다. (인피니티 값으로)
  - **크로스 어텐션 (Encoder Decoder Attetion)**: 디코더의 핵심인데 여기서 **Query(Q)**는 Decoder가 지금까지 생성한 단어가 나오고 Key, Value는 인코더가 넘겨준 문맥 지도에서 나온다 내가 지금 사과라는 단어를 번역했는데(Q), 다음 단어를 쓰려면 원본 문장 K, V의 어디를 봐야하는지를 묻는 과정이다.

### Example

파이토치로 주기 함수를 이용해 지문(바코드)를 생성하고 임베딩에 더하는 것을 보자

```py
import torch
import torch.nn as nn
import math

class PositionalEncoding(nn.Module):
    def __init__(self, d_model, max_len=5000):
        # d_model: 모델의 임베딩 차원 (예: 512)
        # max_len: 모델이 처리할 수 있는 최대 문장 길이
        super().__init__()

        # [max_len, d_model] 크기의 빈 텐서 생성 (여기에 위치 번호표를 채울 예정)
        pe = torch.zeros(max_len, d_model)
        
        # 위치 인덱스 pos 생성: [0, 1, 2, ..., max_len-1]
        position = torch.arange(0, max_len, dtype=torch.float).unsqueeze(1)
        
        # 주기를 결정하는 분모(div_term) 계산 (exp와 log를 이용해 최적화하여 계산)
        # 10000^(2i/d_model) 과 수학적으로 동일함
        div_term = torch.exp(torch.arange(0, d_model, 2).float() * (-math.log(10000.0) / d_model))
        
        # 짝수 인덱스(2i)에는 Sine 함수 적용
        pe[:, 0::2] = torch.sin(position * div_term)
        # 홀수 인덱스(2i+1)에는 Cosine 함수 적용
        pe[:, 1::2] = torch.cos(position * div_term)
        
        # pe를 [Batch, Seq_len, d_model] 형태로 더하기 위해 차원 추가
        pe = pe.unsqueeze(0)
        
        # 모델의 파라미터(가중치)는 아니지만, 상태로 저장해야 하므로 register_buffer 사용
        self.register_buffer('pe', pe)

    def forward(self, x):
        # x: 단어 임베딩 벡터 [Batch, Seq_len, d_model]
        # 입력 벡터 x에 길이에 맞는 포지셔널 인코딩 값을 더해줌
        x = x + self.pe[:, :x.size(1), :]
        return x

# --- 실행 예시 ---
d_model = 512
seq_len = 50   # 50단어짜리 문장
batch_size = 2

# 가상의 단어 임베딩 텐서 생성
word_embeddings = torch.randn(batch_size, seq_len, d_model)

# 포지셔널 인코딩 모듈 통과
pos_encoder = PositionalEncoding(d_model=d_model)
encoded_x = pos_encoder(word_embeddings)

print(f"입력 임베딩 형태: {word_embeddings.shape}")
print(f"위치 정보가 더해진 출력 형태: {encoded_x.shape}")
```