# 근사 검색 알고리즘, HNSW (ANN & FAISS)

벡터 데이터베이스 Elasticsearch나 고속 검색 라이브러리 FAISS의 코어 역할을 하고있는 알고리즘들을 다루어보겠다.

완벽한 100% 정확도를 위해서 서버가 뻗는 대신에 99%의 정확도를 유지하면서 검색 속도를 수백 배 끌어올리는 인덱싱 아키텍처이다.

- **ANN(Approximate Nearest Neighbor) 근사 최근접 이웃**: 벡터 공간에서 쿼리 벡터와 가장 가까운 K개의 벡터를 찾을때 대략적으로 가까운 것들을 극도로 빠르게 찾아내는 알고리즘의 총칭이다.
- **HNSW(Hierachical Navigable Small World)**: ANN 알고리즘 생태계를 평정한 알고르짐으로, 노드 데이터들을 여러 층 그래프로 연결하여 탐색한다.
- **FAISS(Facebook AI Similarity Search)**: 메타 ai 리서치 팀에서 c++로 개발한 초고속 밀집 벡터 검색 라이브러리로. HNSW를 비롯한 다양한 ANN 알고리즘을 손쉽게 사용이 가능하다 .

#### 명칭의 유래

- **Approximate (근사)**: 전수조사 exact를 포기하는 대신, 약간의 오차를 허용하여 검색 성능 throughtput, memory 효율을 얻어낸다는 trade-off
- **HNSW(계층적 탐색 가능한 좁은 세상)**: `Hierarchical` 아파트 층수처럼 여러겹의 레이어로 구성되어 있다. Navigable은 이웃 노드끼리 링크, 간선으로 연결되어있기 때문에 길찾기가 가능하고 small world 여섯다리만 건너면 세계사람을 다 안다는 좁은 세상 네트워크 이론을 차용해 아무리 먼데이터라도 몇 번의 점프만으로 도달할 수 있음을 의미한다.

## The Problem

**Exact k-NN** 기존에는 쿼리 벡터가 1개 들어오면 db에 있는 100만개의 상품과 벡터와 일일이 코사인 유사도를 계산 브루트포스 해야했다. 트래픽이 몰리면 cpu가 버티질 못하니 검색에 1~2초가 넘게 걸려 실서비스가 불가능했다.

**LSH의 정확도 한계**도 존재했다 기존에 사용하던 LSH는 벡터를 단순히 16비트나 32비트의 짧은 문자열로 잘라 해시 테이블에 넣는 방식이고 속도는 빠르지만 정교한 실수 float 값을 몽뚱그려 압축하기 때문에, 디테일이 생명인 홈쇼핑 상품의 경우 진짜 정답을 엉뚱한 해시통에 넣어버려 영원히 찾지 못하는 치명적 문제가 있다.

### Solution

**그래프 기반 탐색으로 해결했다.** HNSW는 데이터를 해시 테이블에 쑤셔넣지 않고, 데이터간의 관계를 거미줄 처럼 엮은 그래프를 만든다.

멀리 건너뛰는 고속도로 층부터 정밀하게 탐색하는 골목길 층까지 여러 레이어를 두어 최적의 길을 찾아 나선다.

예를들어 직관적인 예시를 들어보자.

100만벌의 옷이있는 옷가게 거대 창고에서 빨간 린넨 셔츠를 찾아야한다

- **Exact k-NN**이라면 전수조사. 즉 알바생이 창고 입구부터 100만벌의 옷을 하나하나 꺼내 비교한다. 정확도는 100%겠지만 오래걸린다.
- **LSH** 우편번호 느낌으로 옷들이 비슷한 색깔별로 던져져있기 때문에 빨간색 보관함을 뒤질것이고 빠르긴한데 직원이 실수로 린넨 셔츠를 주황색 보관함에 넣어놨다면 못찾게 된다. 즉 정확도가 70 정도고 검색시간이 좀 더 짧아진느낌
- **HNSW (네비게이션 그래프)**: 알바생이 창고에 층별 안내도를 그려놨다.
  - 3층(고속도로): 여름옷 구역으로 크게 점프
  - 2층(국도): 여름옷 중에서도 셔츠 구역
  -  1층(골목길): 셔츠 구역 안에서 내 주변에 있는 빨간옷들 10벌만 비교 단 10벌만 비교하고도 정확도가 높음 (정확도 98 검색시간도 0.01초정도)

## 상세 동작 원리 및 구조화

HNSW의 내부 아키텍처는 공간층으로 나눈 다층 그래프 탐색 multi-layer graph navigation 이다.

#### 1. **색인 indexing 단계

최하단 레이어 layer 0에서는 100만개의 전체 데이터가 촘촘하게 이웃끼리 연결된다.

위로 올라갈수록 확률을 통해 살아남은 소수의 허브들만 듬성듬성 연결된다 layer 1, layer 2...

#### 2. 검색 단계

진입점 entry point 가장 꼭대기 층 노드가 몇 개 없는 층에서부터 탐색을 시작한다.

그리디 서치를 하는데 현재 층에서 쿼리 벡터와 가장 가까운 노드로 이동해서 더 이상 가까워질 이웃이 없으면 바로 아래층 layer로 내려간다.

최하단도달 layer 0에 도달하면, 이미 쿼리와 매우 가까운 곳에 진입한 상태로 그 주변 이웃들만 정렬해 최종 top-k를 반환한다. ㅇ이 과정에서 복잡도는 O(log(N)) 으로 수렴한다.

### Example

파이토치로 뽑은 벡터를 FAISS, HNSW 인덱스에 넣고 초고속으로 탐색하는 기본적인 로직이다 FAISS는 오직 numpy, float32 타입만 취급한다는 점이 핵심ㅇ이다.

```py
import faiss
import numpy as np
import torch

def understand_faiss_hnsw(query_tensor, db_tensors):
    """
    차원(d)이 512인 PyTorch 벡터들을 FAISS HNSW 인덱스에 넣고 검색합니다.
    """
    # 1. PyTorch Tensor -> Numpy Array 변환 (FAISS 필수 조건)
    # CPU 메모리로 옮기고 연속적인 float32 배열로 캐스팅합니다.
    db_vectors = db_tensors.cpu().numpy().astype('float32')
    query_vector = query_tensor.cpu().numpy().astype('float32')
    
    d = db_vectors.shape[1] # 벡터 차원 수 (예: 512)
    
    # 2. HNSW 인덱스 생성
    # 32는 노드당 연결할 최대 이웃의 수(M)를 의미합니다. (파라미터 튜닝 요소)
    index = faiss.IndexHNSWFlat(d, 32)
    
    # 거리 측정 방식을 코사인 유사도와 동일한 효과를 내는 L2 거리(내적)로 설정
    index.hnsw.efConstruction = 40 # 색인 시 탐색 깊이 (정확도 vs 빌드 속도)
    
    # 3. 데이터 색인 (메모리에 그래프 빌드)
    index.add(db_vectors)
    print(f"HNSW 그래프에 총 {index.ntotal}개의 벡터 색인 완료")
    
    # 4. 검색 파라미터 세팅 및 검색
    index.hnsw.efSearch = 64 # 검색 시 탐색 깊이 (정확도 vs 검색 속도)
    k = 5 # 상위 5개
    
    # distances: 쿼리와 가장 가까운 노드들의 거리 값 반환
    # indices: 해당 노드들의 DB 내 배열 인덱스(ID) 반환
    distances, indices = index.search(query_vector, k)
    
    print(f"찾아낸 Top-{k} ID   : {indices[0]}")
    print(f"해당 ID의 거리 값 : {distances[0]}")
```

실제 서버 환경에서 벡터를 매번 색인하지 않고 디스크에 인덱스를 write_index 및 로드 read_index 하는 코사인 유사도 검색을 위한 사전 정규화를 수행하는 래퍼 클래스를 예시로 또 봐보겠다.

```py
import faiss
import numpy as np

class HNSWSearchEngine:
    """실무 환경에서 사용하는 FAISS 기반 벡터 저장/검색 엔진"""
    
    def __init__(self, dimension: int = 512, m: int = 32, ef_search: int = 128):
        self.dimension = dimension
        # L2 거리를 기반으로 하는 HNSW 인덱스 생성 (코사인 유사도용)
        self.index = faiss.IndexHNSWFlat(dimension, m)
        self.index.hnsw.efConstruction = 100
        self.index.hnsw.efSearch = ef_search
        
    def _normalize(self, vectors: np.ndarray) -> np.ndarray:
        """
        [매우 중요] FAISS는 내장 코사인 유사도 검색이 까다롭습니다.
        하지만 벡터를 L2 정규화(길이를 1로 만듦) 한 뒤 L2 거리(Euclidean)를 구하면, 
        수학적으로 코사인 유사도 순위와 완벽하게 동일해집니다.
        """
        norms = np.linalg.norm(vectors, axis=1, keepdims=True)
        # 0으로 나누는 것 방지
        norms = np.where(norms == 0, 1e-10, norms)
        return vectors / norms

    def build_index(self, product_ids: list, vectors: np.ndarray, save_path: str = "hnsw.index"):
        """전체 상품 벡터를 정규화하여 그래프에 올리고 파일로 저장합니다."""
        assert len(product_ids) == vectors.shape[0]
        
        normalized_vecs = self._normalize(vectors).astype('float32')
        self.index.add(normalized_vecs)
        
        # 실무에서는 인덱스(0, 1, 2...)와 실제 상품 ID(item_99)를 매핑하는 딕셔너리를 따로 저장해야 합니다.
        self.id_map = {i: pid for i, pid in enumerate(product_ids)}
        
        # 재시작 시 1초 만에 복구하기 위해 인덱스를 디스크에 덤프
        faiss.write_index(self.index, save_path)
        print(f"✅ HNSW 인덱스 저장 완료: {save_path} (총 {self.index.ntotal}건)")

    def load_index(self, load_path: str = "hnsw.index"):
        """서버 부팅 시 디스크에서 인덱스를 메모리로 로드합니다."""
        self.index = faiss.read_index(load_path)
        print(f"✅ HNSW 인덱스 로드 완료 (총 {self.index.ntotal}건)")

    def search(self, query_vector: np.ndarray, top_k: int = 10):
        """실시간 검색 쿼리를 처리합니다."""
        # 쿼리 벡터 역시 정규화 필수
        query_normalized = self._normalize(query_vector).astype('float32')
        
        # 검색 수행 (수 밀리초 소요)
        distances, indices = self.index.search(query_normalized, top_k)
        
        results = []
        for dist, idx in zip(distances[0], indices[0]):
            if idx != -1: # -1은 찾지 못했음을 의미
                results.append({
                    "product_id": self.id_map.get(idx, "Unknown"),
                    # L2 거리를 직관적인 점수로 변환 (거리가 짧을수록 점수가 높게)
                    "score": round(float(1 / (1 + dist)), 4) 
                })
        return results

# --- 실무 API 서버 활용 예시 ---
# hnsw_db = HNSWSearchEngine(dimension=512)
# hnsw_db.load_index()
#
# @app.post("/search/vector")
# def vector_search(query: str):
#     query_vec = clip_encoder.get_vector(query) # Step 2에서 만든 CLIP 객체
#     results = hnsw_db.search(np.array([query_vec]), top_k=20)
#     return {"items": results}
```