# Elasticsearch Indexing & Vector Search

LSH, CNN등을 위에서 다뤘었는데, 고차원 벡터와 해시 서명을 실제 분산 시스템 환경에서 어떻게 디스크에 저장하고 밀리초 단위로 검색하는지 시스템 엔지니어링 관점에서 알아보자

Elasticsearch는 apache lucene 라이브러리를 코어로 사용하는 분산형 restful 검색 및 분석 엔진이다.

이미지 검색 파이프라인에서 es는 편환된 해시값 lsh이나 고차원 실수 벡터 CNN Feature를 색인하여 보관하고 사용자의 쿼리 이미지가 들어왓을때 가장 유사한 데이터를 초고속으로 찾아내는 데이터베이스 역할을 수행한다.

> Elasticsearch는 노드 서버를 추가하거나 제거할 대 자동으로 분산 sharding and replica 되어 수시템 규모를 탄력적 elastic 조정할 수있는 검색 search 가 결합된 이름으로 유래되었다.
>
> indexing은 책 맨 뒤 찾아보기 처럼 데이터가 저장될 대 검색에 최적화된 특정 자료구조 역색인 또는 그래프로 미리 변환하여 디스크와 메모리에 매핑해두는 행위이다.

### 문제 정의

ES가 해결하고자한 문제는 **B-Tree의 한계를 먼저 알아볼수있는데** RDBMS는 B-Tree 인덱스 기반으로 되어있고 1차원적 숫자나 문자열 크기 비교 및 정렬에는 최적화되어있지만, 다차원 배열(벡터)간의 공간적 유사도(거리)를 구하는 작업에는 전혀 사용할 수 없다.

선형 탐색의 병목도 있는데, 1000만개의 상품 이미지가 있을때 RDBMS에서 거리 계산 수식을 돌리면 모둔 레코드를 디스크에서 읽어와 cpu에서 연산해야하므로 쿼리당 수분에서 수십분이 소요된다.

> B-Tree는 1차원적 데이터 숫자, 문자열등의 대소 크기를 비교하여 트리를 분기하고 정렬하는데 특화된 구조로 딥러닝에서 수출된 수백 수천차원의 벡터는 단일값크기로 비교할 수 없으며 다차원 공간상에서는 모든 요소를 종합한 유클리디안 거리나 코사인 각도를 계산해야하므로 B-Tree의 탐색 로직을 전혀 적용할 수 없다.

### 해결 방식

이미지 검색을 위한 ES의 색인 방식은 크게 두 가지 구조로 나뉜다. 고전적 LSH 방식과 현대 ES가 지원하는 native vector 방식.

#### LSH 해시값을 이용한 역색인 방식

LSH 알고리즘이 2048차원의 벡터 10110 같은 이진 문자열 토큰으로 변환했다면 ES는 이를 일반적인 텍스트 단어처럼 취급하여 역색인 구조에 넣는다.

"어떤 해시 토큰이 어느 문서 document id에 포함되어 있는가"를 매핑해두고 교집합/합집합 비트 연산으로 단숨에 후보군을 추려낸다

#### 고밀도 벡터 dense vector와 HNSW 그래프 방식 (현대적 접근)

최신 ES(7.x 이상)는 해싱 없이 딥러닝에서 나온 2048차원 실수벡터 그대로 dense_vector 타입으로 색인할 수 있다.

내부적으로 HNSW(Hierarchical Navigable Small World) 라는 알고리즘을 통해 벡터들을 계층적 그래프로 메모리에 엮어두고, 마치 스킵 리스트 처럼 상위 계층부터 성큼성큼 건너뛰며 유사 벡터를 찾아낸다

<br>

## 상세 컴포넌트 동작 원리 및 구조화 low-level architecture

### 역색인 Inverted Index 구조와 포스팅 리스트 posting list

Dictionary(해시 토큰 모음)와 Posting List(해당 토큰을 가진 문서 ID 배열)로 구성된다.

- Term 10110 -> [ Doc1, Doc5, Doc102 ]
- Term 11100 -> [ Doc2, Doc5, Doc88 ]

쿼리 이미지의 LSH 해시가 10110, 11100으로 나왔다면 ES는 전체 데이터를 뒤지지 않고 두 포스팅 리스트만 메모리로 가져온다.

**Skip-list, Bitset 교집합 연산:** 두 리스트에서 공통으로 등장하는 문서 Doc5를 찾기 위해, Lucene은 포스팅 리스트 내부에 Skip-list 포인터를 두어 탐색 속도를 $O(\log N)$으로 줄이고 cpu의 `SIMD` 명령어를 활용한 Bitset AND 연산이 초고속 교집합을 구한다.

> SIMD는 Single Instruction, Multiple Data로 하나의 명령어로 여러개의 데이터를 동시에 병렬처리하는 cpu 하드웨어 기술로 일반적인 연산 방식이 반복문을 돌며 데이터를 하나씩 처리하면, SIMD는 긴 데이터 배열(벡터)를 cpu 레지스터에 통째로 올려 한 번의 사이클에 동시에 계산해낸다. es는 lsh 해시 비트맵의 교집합을 구하거나 다차원 벡터 간의 유사도 거리를 연산할 때 SIMD를 적극 활용해 검색속도를 극단적으로 끌어올린다.

### HNSW(Hierarchical Navigable Small World) 알고리즘 벡터 서치

그래프를 생성한다. 벡터 공간 내에서 거리가 가까운 데이터끼리 엣지를 연결하여 거미줄 같은 그래프를 만든다

계층을 이후에 구조화하는데 노드들을 여러 층으로 나눈다.

- **최상위 Layer 3**: 아주 듬성듬성한 소수의 노드만 존재, 진입점 역할
- **Layer 2:** 조금 더 촘촘한 노드들
- **Layer 0**바닥층: 모든 데이터가 조밀하게 연결된 원본 그래프

**탐색 메커니즘**

1. 최상위 레이어의 임의의 노드에서 시작해 쿼리 벡터와 가장 가까운 노드로 이동 (greedy routing) 한다.
2. 더 이상 가까운 노드가 없으면 바로 아래 레이어로 강하 drop down 한다.
3. 이를 바닥층 layer 0의 로컬 최적점에 도달할 때까지 반복한다.

이 방식은 디스크 io대신 메모리 ram 상의 포인터 점프를 극대화하므로, 억 단위의 벡터 데이터에서도 수 밀리초 이내에 nearest neighbor 탐색을 가능하게 만든다.

### Example

상품 이미지를 LSH 해시 토큰으로 저장하는 필드와 HNSW 방식의 원본 Dense Vector로 저장하는 필드를 모두 포함하여 인덱스를 설계하고 검색하는 코드다.

```py
from elasticsearch import Elasticsearch
import numpy as np

# ES 클라이언트 연결
es = Elasticsearch("http://localhost:9200")
index_name = "image_search_index"

def create_index():
    """이미지 검색을 위한 Elasticsearch 인덱스 매핑(스키마) 정의"""
    mapping = {
        "mappings": {
            "properties": {
                "product_id": {"type": "keyword"},
                
                # 1. LSH 방식: 해시 서명들을 단순 문자열 배열(Keyword)로 저장
                # 검색 시 Term 쿼리로 완전 일치 매칭 수행 (역색인 활용)
                "lsh_signatures": {"type": "keyword"},
                
                # 2. HNSW 방식 (최신 ES 권장): 딥러닝 512차원 실수 벡터를 그대로 저장
                # index: true로 설정 시 내부적으로 HNSW 그래프 생성
                "cnn_vector": {
                    "type": "dense_vector",
                    "dims": 512,
                    "index": True, 
                    "similarity": "cosine" # 거리 계산 기준 (코사인 유사도)
                }
            }
        }
    }
    
    if not es.indices.exists(index=index_name):
        es.indices.create(index=index_name, body=mapping)
        print(f"인덱스 '{index_name}' 생성 완료.")

def index_document(product_id, lsh_sigs, vector):
    """추출된 데이터를 ES에 색인 (Indexing)"""
    doc = {
        "product_id": product_id,
        "lsh_signatures": lsh_sigs,       # 예: ["hash_A_101", "hash_B_010"]
        "cnn_vector": vector.tolist()     # Numpy 배열을 리스트로 변환
    }
    es.index(index=index_name, id=product_id, body=doc)

def search_by_lsh(query_signatures):
    """방식 1: LSH 토큰을 이용한 역색인 고속 검색 (Boolean Query)"""
    # 쿼리의 해시 토큰 중 '하나라도' 일치하는 문서들을 스코어 순으로 가져옴 (OR 조건)
    query = {
        "query": {
            "terms": {
                "lsh_signatures": query_signatures,
                "boost": 1.0
            }
        },
        "size": 10 # 상위 10개 상품 반환
    }
    response = es.search(index=index_name, body=query)
    return [hit["_source"]["product_id"] for hit in response["hits"]["hits"]]

def search_by_knn(query_vector):
    """방식 2: HNSW 그래프를 이용한 네이티브 벡터 검색 (k-NN Search)"""
    # k: 최종 반환할 결과 수, num_candidates: 각 샤드에서 HNSW로 탐색할 후보군 수
    query = {
        "knn": {
            "field": "cnn_vector",
            "query_vector": query_vector.tolist(),
            "k": 10,
            "num_candidates": 100
        },
        "_source": ["product_id"] # 반환할 필드 지정 (네트워크 대역폭 최적화)
    }
    response = es.search(index=index_name, body=query)
    return [hit["_source"]["product_id"] for hit in response["hits"]["hits"]]

# ==========================================
# 실행 시뮬레이션
# ==========================================
# 1. 스키마 세팅
create_index()

# 2. 데이터 인덱싱 (서버에 상품이 등록될 때 실행)
sample_vector = np.random.rand(512).astype(np.float32)
# LSH 테이블이 2개라고 가정하여 추출된 서명
sample_lsh_sigs = ["t1_10110", "t2_11100"] 

index_document("item_001", sample_lsh_sigs, sample_vector)
print("상품 데이터 색인 완료.")

# 3. 데이터 검색 (사용자가 쿼리 이미지를 올렸을 때 실행)
# 보통 실무에서는 1차로 LSH를 통해 100개로 후보군을 좁히고, 
# 2차로 해당 후보군에 대해서만 원본 벡터 코사인 유사도(Rescoring)를 계산하여 정확도를 극대화합니다.
lsh_results = search_by_lsh(["t1_10110", "t2_00000"])
print(f"LSH 기반 검색 결과: {lsh_results}")

knn_results = search_by_knn(sample_vector)
print(f"k-NN (HNSW) 기반 검색 결과: {knn_results}")
```