# Self-Attention, Multi-Head Attention

Attention solved the bottleneck between the encoder and decoder and significantly improved translation quality.

However, the backbone was still **RNN**, which left two critical limitations from a deep learning architecture perspective.

- **No Parallel Processing (Speed Issue):** RNNs must read words in chronological order (1 2 3 ...). Since the next word can only be processed after the previous word's computation is complete, even with powerful GPUs, true simultaneous computation (parallel processing) is not possible.
- **Still Long Distance (Reference Issue):** Even with attention, understanding the relationship between the first and last words of a sentence still required physically traversing numerous timesteps (words) in between.

The solution began with the idea of "completely abandoning the RNN structure that reads words one by one in sequence, and instead, having all input words look at each other simultaneously to understand their relationships."

This can be seen as the background for the emergence of **Self-Attention**, one of the core concepts of the Transformer.

<br>

## Query, Key, Value (Q, K, V)

Self-attention learns how words within a sentence relate to each other.

To do this, each input word vector is duplicated (linearly transformed) into three different roles. An analogy to a database search system makes this intuitive.

- **Query (Q, Question/Search Term):** What information do I need?
  - In a sentence, the word 'it' plays the role of establishing what noun it refers to.
- **Key (K, Keyword/Title):** What characteristics do I have?
  - In a sentence, the word 'animal' has a tag saying 'I am an animal and a noun'.
- **Value (V, Actual Content/Value):** What is the real meaning I want to convey?

### How it works

1. Calculate the attention score, or similarity score, by performing a dot-product between the Q of a specific word and the K of all words in the sentence to determine their relevance.
2. Pass this score through a softmax function to create weights that sum to 1.
3. The higher the weight (probability), the more of that word's V is brought in to update (weighted sum) the current word's meaning according to the context.

<br>

## Multi-Head Attention

When understanding relationships between words, relying on only one criterion can lead to missing context.

Some words might have important grammatical relationships like subject-verb, while others might be connected to the emotional tone of the sentence.

- **Concept:** Instead of performing computations on the model's entire embedding dimension (512 dimensions) as a whole, it's split into multiple independent heads, for example, 8 of them.
- **Effect:** The 8 heads, like 8 different experts, analyze the sentence from various perspectives—grammar, meaning, anaphora, etc.—and then concatenate the results from each head to grasp a much more three-dimensional and rich context.

### Example

Looking at the tensor flow, we can see how Q, K, and V are derived from the actual input word vector x to generate context.

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

In this way, the Transformer eliminates the sequential dependencies of RNNs and calculates the relationships between all words in a sentence at once, solely through matrix multiplication.

This allows for 100% utilization of GPU's parallel processing capabilities, leading to overwhelmingly fast training speeds and scalability.
