# LSH(Locality-Sensitive Hashing)

LSH is an **Approximate Nearest Neighbor (ANN) search algorithm designed to increase the probability that similar vectors have the same hash value**, instead of comparing all data one by one, when searching for high-dimensional vector data (e.g., 2048-dimensional CNN feature vectors) in a database.

Let's look at the origin of the name:

-   **Locality**: Refers to a local state where vectors are close and similar in vector space.
-   **Sensitive**: Unlike general hash functions like SHA-256, where even a 1-bit difference in input results in a completely different output (Avalanche Effect), LSH is sensitive to data similarity and is intentionally designed to cause hash collisions for similar data.
-   **Hashing**: The process of converting data into a fixed-length, short identifier bucket address.

### Problem Addressed

**Curse of Dimensionality and Exploding Computation**: If there are 10 million product data items, each represented by a 2048-dimensional vector, and a user uploads a photo, it would require 10 million Euclidean/cosine distance calculations.

This has a time complexity of $O(N \times D)$, making it impossible to respond within tens of milliseconds in real-time.

**Limitations of Tree-based Indexes**: Traditional spatial partitioning search trees like KD-Trees suffer from a phenomenon where all nodes must be traversed if the data dimension exceeds 20. This effectively makes the speed identical to a brute-force full search.

### Solution Approach

**"Let's sacrifice 100% perfect accuracy to achieve 99% accuracy, but in return, drastically boost search speed by over 1,000 times."**

The vector space is randomly split in half multiple times (random projection). Two vectors that are close to each other have a very high probability of belonging to the same region each time they are split. By recording which region they belong to as binary bits (0 or 1), a signature is created. Buckets are then formed by grouping data with identical signatures.

During a search, instead of examining all 10 million items, distances are calculated only with the hundreds of data points in the bucket that share the same signature as the query vector, and the results are returned.

## Structuring the Detailed Component Operation Principle

This section structures the operational mechanism of LSH, specifically random projection-based LSH, excluding cosine similarity, into three stages.

### 1. **Random Projection**

Based on geometric principles, a random hyperplane passing through the origin is drawn in the $D$-dimensional space where the data exists. This hyperplane divides the space, with one side assigned 1 and the other 0.

A random vector $R$ is generated, and its dot product with the input vector $V$ is computed. If the dot product is greater than or equal to 0, it's assigned 1; otherwise, it's assigned 0.

$$h(V) = \begin{cases} 1 & \text{if } V \cdot R \ge 0 \\ 0 & \text{if } V \cdot R < 0 \end{cases}$$

> The dot product is an operation that multiplies corresponding components of two vectors and then sums the results, yielding a scalar.

The narrower the angle between two vectors (i.e., the more similar they are), the lower the probability that a random hyperplane will separate them, thus increasing the chance of being assigned the same bit.

### 2. **Hashing Signature Generation**

A single hyperplane can only divide space into two regions. Therefore, K random hyperplanes are generated.

Projecting vector V onto K hyperplanes yields a K-bit array, e.g., if K = 5, something like 10110.

This K-bit string becomes the vector's signature and bucket number. This creates $2^K$ bucket spaces.

### 3. **Amplification (Accuracy Boosting Technique: AND / OR Configuration)**

Using hash collisions introduces two problems:

One is False Positive: dissimilar items accidentally end up in the same bucket. The other, more critical, is False Negative: truly similar items are accidentally separated by a single hyperplane, causing their signatures to differ by 1 bit and sending them to different buckets, leading to a search failure.

To address this, multiple hash tables (multi-tables) are used.

-   **AND Condition (Splitting Buckets)**: Within a single table, increasing the bit length K makes it harder for signatures to be perfectly identical, thus making it stricter (reduces False Positives, increases speed).
-   **OR Condition (Increasing Tables)**: L such tables are created. During a search, if there's a collision in even one of the L tables for the same bucket, all items are brought in as candidates. This reduces False Negatives and improves recall.
-   **Mathematical Probability**: When the probability of a hash collision based on the similarity of two vectors is p, using L tables and K bits results in a final probability of inclusion in the candidate set of $1 - (1 - p^K)^L$. This forms an S-shaped probability curve, allowing for remarkably effective filtering of highly similar data.

### Example

Let's look at an implementation of random projection using numpy. This code implements the low-level process of LSH hashing and bucketing high-dimensional vectors using numpy, without relying on libraries, to clearly understand the internal mechanism. FAISS, Annoy, or Elasticsearch's `dense_vector` are used.

```py
import numpy as np
from collections import defaultdict

class RandomProjectionLSH:
    def __init__(self, vector_dim, num_bits=8, num_tables=3):
        """
        :param vector_dim: 입력 벡터의 차원 수 (예: CNN 결과 2048차원)
        :param num_bits: K값. 몇 개의 랜덤 평면을 사용할 것인가? (버킷 식별자 길이)
        :param num_tables: L값. OR 조건을 위한 해시 테이블의 개수
        """
        self.dim = vector_dim
        self.num_bits = num_bits
        self.num_tables = num_tables
        
        # 1. 무작위 초평면(Hyperplanes) 생성
        # 형태: (num_tables, num_bits, vector_dim)
        # 정규분포(Gaussian)에서 난수를 추출하여 방향성이 완전히 랜덤한 평면의 법선 벡터들을 만듭니다.
        np.random.seed(42) # 재현성을 위한 시드 고정
        self.hyperplanes = np.random.randn(self.num_tables, self.num_bits, self.dim)
        
        # 2. 해시 테이블 초기화 (리스트 안에 딕셔너리 구조)
        # 각 테이블은 { '10110100': [벡터인덱스1, 벡터인덱스3...], ... } 형태를 가집니다.
        self.hash_tables = [defaultdict(list) for _ in range(self.num_tables)]
        
        # 원본 데이터를 저장할 리스트
        self.database = []

    def _compute_hash(self, vector):
        """벡터 하나를 받아 num_tables개의 해시 서명(이진 문자열)을 반환합니다."""
        signatures = []
        for i in range(self.num_tables):
            # 벡터와 해당 테이블의 랜덤 평면들을 한 번에 내적 (행렬 곱)
            # vector: (D,), hyperplanes[i]: (K, D) -> 결과: (K,)
            projections = np.dot(self.hyperplanes[i], vector)
            
            # 내적 결과가 0보다 크면 1, 아니면 0으로 변환 (불리언 -> 정수)
            binary_bits = (projections >= 0).astype(int)
            
            # [1, 0, 1, 1] 배열을 '1011' 형태의 문자열(서명)로 합침
            signature = ''.join(map(str, binary_bits))
            signatures.append(signature)
        return signatures

    def index_vector(self, vector, vector_id):
        """데이터베이스에 벡터를 색인(Indexing)합니다."""
        self.database.append((vector_id, vector))
        
        # 벡터를 해싱하여 각 테이블의 알맞은 버킷에 ID를 집어넣습니다.
        signatures = self._compute_hash(vector)
        for i, sig in enumerate(signatures):
            self.hash_tables[i][sig].append(vector_id)

    def query(self, query_vector):
        """쿼리 벡터와 비슷한 벡터 ID들을 버킷에서 찾아 반환합니다."""
        signatures = self._compute_hash(query_vector)
        candidate_ids = set() # 중복 제거를 위한 Set
        
        # L개의 테이블을 돌면서 OR 조건으로 후보군을 긁어모읍니다.
        for i, sig in enumerate(signatures):
            if sig in self.hash_tables[i]:
                # 해당 서명을 가진 버킷 내의 모든 데이터를 후보군에 추가
                candidate_ids.update(self.hash_tables[i][sig])
                
        return list(candidate_ids)

# --- 실행 및 테스트 ---
# 1. 128차원의 벡터 데이터를 가정
D = 128
lsh = RandomProjectionLSH(vector_dim=D, num_bits=10, num_tables=5)

# 2. 가상의 상품 벡터 10,000개 색인 (Indexing)
print("10,000개의 임의 벡터를 색인 중...")
for idx in range(10000):
    vec = np.random.randn(D)
    lsh.index_vector(vec, vector_id=f"item_{idx}")

# 3. 검색 (Querying)
# 원본 데이터베이스에서 첫 번째 벡터에 약간의 노이즈만 섞어서 쿼리 벡터 생성 (유사한 이미지 가정)
query_vec = lsh.database[0][1] + np.random.randn(D) * 0.1 

# 전체 10,000개를 다 검사하지 않고, LSH를 통해 후보군만 빠르게 추림
candidates = lsh.query(query_vec)

print(f"\n검색 완료!")
print(f"전체 10,000개 중 거리 계산(비교)을 수행할 후보군: {len(candidates)}개로 압축됨.")
print(f"후보군에 정답(item_0)이 포함되어 있는가? : {'item_0' in candidates}")

# (이후 실무에서는 뽑힌 후보군들하고만 코사인 유사도를 정밀 계산하여 Top-K를 정렬 후 반환합니다.)
```
