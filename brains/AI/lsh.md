# LSH(Locality-Sensitive Hashing)

LSH는 고차원 벡터 데이터 예를들면 2048차원의 CNN 특징 벡터를 데이터베이스에서 검색할 때, 모든 데이터를 일일이 비교하는 대신 **서로 비슷한 벡터들이 동일한 해시값을 가질 확률을 높이도록 설계된 근사 최근접 이웃 검색 알고리즘 (ANN, Approximate Nearest Neighbor)이다.**

명칭에 유래를 알아보자면,

- **Locality 지역성 / 유사성**: 벡터 공간 상에서 서로 거리가 가깝고 유사한 local 상태를 의미한다.
- **Sensitive (민감한)**: 일반적인 해시 함수 sha-256등은 입력값이 1비트만 달라도 결과가 완전히 달라지지만(Avalanche Effect), LSH는 데이터의 유사성에 민감하게 반응하여 비슷한 데이터는 의도적으로 해시 충돌(collision)을 일으키도록 설계되었다.
- **Hashing**: 데이터를 고정된 길이의 짧은 식별자 버킷 주소로 변환하는 과정

### 해결하고자 한 문제

**차원의 저주(Curse of Dimensionality)와 연산량 폭발**: 상품 데이터가 1,000만개이고 각각이 2048차원 벡터라면, 사용자가 사진을 올렷을 때 이를 1,000만번 유클리디안/코사인 거리 연산을 해야한다.

이는 $O(N \times D)$의 시간복잡도를 가지며 실시간 수십 ms이내에 응답이 불가능하다.

**Tree 기반 인덱스의 한계**: KD-Tree 같은 전통적인 공간 분할 검색 트리는 데이터의 차원이 20차원을 넘어가면 모든 노드들이 다 뒤져야하는 현상이 발생한다. 사실상 완전탐색 브루트포스와 속도가 똑같아진다.

### 해결 방식

**"100% 완벽한 정확도를 포기하고 99%의 정확도를 얻는 대신, 검색 속도를 1,000배 이상 극단적으로 끌어올리자."**

벡터 공간을 무작위로 여러 번 반으로 쪼갠다(랜덤 투영). 비슷한 위치에 있는 두 벡터는 반으로 쪼개질때마다 같은 구역에 속할 확률이 매우 높다. 이렇게 어느 구역에 속하는가를 이진수 비트 0, 1로 기록하여 서명 signature를 만들고 서명이 똑같은 데이터끼리들만 모아둔 bucket을 생성한다.

검색시에는 1,000만개를 모두 보지않고 쿼리 벡터와 서명이 동일한 버킷에 들어있는 수백 개의 데이터와만 거리를 계산하여 결과를 반환한다.

## 상세 컴포넌트 동작 원리 구조화

LSH 그 중에서도 코사인 유사도를 제외한 random projection 기반 LSH의 동작 메커니즘을 3단계로 구조화한다.

### 1. **Random Projection (무작위 초평면 투영)**

기하학적 원리로 데이터가 존재하는 $D$차원 공간에 원점을 지나는 무작위의 평면을 그린다. 이 평면을 기준으로 한쪽은 1, 반대쪽은 0으로 나눈다.

랜덤한 벡터 $R$을 생성하고, 입력벡터 $V$와 내적(Dot Product)를 한다. 내적값이 0보다 크면 1 작으면 0을 부여한다.

$$h(V) = \begin{cases} 1 & \text{if } V \cdot R \ge 0 \\ 0 & \text{if } V \cdot R < 0 \end{cases}$$

> 내적은 두 벡터에 대응하는 성분끼리 곱한 후 그 결과를 모두 더하는 연산으로 스칼라로 나오는 계산법이다.

두 벡터가 이루는 각도가 좁을수록(비슷할 수록) 무작위 평면이 두 벡터 사이를 갈라놓을 확률이 낮아지므로 동일한 비트를 부여받을 확률이 높아진다.

### 2. **Hashing Signature Generation**

평면 1개로는 공간을 2개밖에 못나눈다. 따라서 무작위 평면 K개를 생성한다.

벡터 V를 K개의 평면에 투영하면 K개의 비트배열이 나온다 ex. K = 5, 10110 같은

이 K-bit 문자열이 해당 벡터의 서명(signature) 이자 버킷 번호가 된다. 이때 $2^K$개의 버킷 공간이 생성된다.

### 3. **Amplification (정확도 증폭 기법: AND / OR 구성)**

해시 충돌을 이용하다 보니 두 가지 문제가 생긴다.

하나는 False Positive(오탐): 안비슷한데 우연히 같은 버킷에 잡히는거 하나는 False Negative(미탐) 더 치명적인건데 진짜 비슷한데 우연히 평면 하나가 사이를 갈라놔서 서명이 1비트로 달라져 다른 버킷으로 감 즉 검색 실패

이를 해결하기 위해 여러 개의 해시 테이블 multi-tables를 운영한다.

- **AND 조건 (버킷 쪼개기)**: 한 테이블 내에서 비트 길이 K를 늘리면 서명이 완벽하게 같아지기 어려우므로 깐깐해진다. (False Positive 감소, 속도 증가)
- **OR 조건 (테이블 늘리기)**: 이런 테이블을 L개 만든다. 검색시 L개의 테이블중 단 하나라도 동일한 버킷에 대한 충돌이 있으면 모두 후보군 candidate로 가져온다 False Negative가 감소, 재현율 향상
- **수학적 확률**: 두 벡터의 유사도에 따른 해시 충돌 확률이 p일때 L개의 테이블과 K개의 비트를 사용할 경우 최종적으로 후보군에 포함될 확률은 $1 - (1 - p^K)^L$이라는 S자 형태의 확률 곡선을 그리게 되며 유사도가 높은 데이터만 기가막히게 잘라낼 수 있다.

### Example

numpy를 활용한 random projection 구현을 알아보겟다. 내부 메커니즘을 명확히 이해하기 위해, 라이브러리를 쓰지 않고 고차원 벡터를 LSH 해싱하고 버킷화 하는 low-level 과정을 numpy로 구현한 코드다. FAISS, Annoy or Elasticearch의 dense_vector를 사용한다.

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