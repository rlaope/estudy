# Elasticsearch 8.x Native Vector Search와 하이브리드 아키텍처

FAISS를 도입해 벡터 검색을 10ms로 줄였다고 칩시다. 그런데 기획팀에서 나이키 브랜드 중에서 비슷한 이미지를 찾아주세요! 라고 요구한다.

실제 아키텍처에서 FAISS는 오직 실수 배열 (벡터)와 정수 ID만 메모리에 올리는 C++ 라이브러리로, 브랜드명이 나이키라는 텍스트 메타데이터를 저장하거나 필터링할 수 있는 기능 자체가 존재하지 않는다.

반대로 기존 검색엔진 ES7.x 이하는 텍스트 필터링은 완벽하지만 내부 코어 HNSW 알고리즘이 구현되어 있지 않아 임베딩 비교시 전체 문서를 스캔하는 선형탐색 O(N) 병목이 발생한다.

그렇다면 서로 치명적인 결함을 가진 두 시스템을 억지로 이어붙이지 않고 나이키 상품 중 이 이미지와 비슷한 것을 단일 쿼리로 10ms 안에 처리해야한다면

**위 팩트 기반의 시스템 제약에 대한 해답이 Elasticsearch 8.x의 Native Vector Search 기능인것이다.**

딥러닝 임베딩 연산 결과 벡터를 텍스트 검색 엔진의 코어에 직접 색인하여, 벡터 유사도 탐색과 텍스트 핉터링을 물리적으로 동일한 데이터베이스 엔진 내에서 동시에 처리하는 분산 검색 아키텍처다.

- **Native Vector Search**: 외부 플러그인 FAISS 등이나 우회 스크립트 없이, 데이터베이스 코어 엔진 Lucene 단에서 직접 벡터 인덱싱과 탐색 네이티브를 지원하는 기능
- `dense_vector`: 희소 배열 sparse array가 아닌 차원에 실수 float 값이 꽉 차있는 딥러닝 임베딩 결과를 저장하기 위한 전용 데이터 타입
- **Hybrid Search**: 전통적인 키워드 매칭(BM25 스코어)과 임베딩 벡터 매칭(k-NN 스코어)를 결합하여 최종 검색 결과를 도출하는 기법.
- **RRF(Reciprocal Rank Fusion):** 스케일이 서로 다른 두 가지 검색 결과 (BM25 점수와 코사인 유사도 점수)를 합치기 위해, 절대 점수 대신 rank를 기반으로 역수를 취해 새로운 점수를 계산하는 정규화 알고리즘.

<br>

## The Problem

**Two-System 아키텍처 정합성 문제**: 기존에는 텍스트용 ES와 벡터용 FAISS 서버를 이중으로 운영해야했다. 이 경우 post-filtering (FAISS에서 1000개를 먼저 찾고 ES에서 나이키를 거르는 방식 -> 나이키가 0개일 수 있음)이나 pre-filtering(ES에서 나이키 10만개를 찾고 그 데이터에서 FAISS를 10만개 id를 넘겨 벡터 탐색 -> 네트워크 메모리 폭발) 방식 모두 치명적인 검색 누락이나 지연속도를 유발했다.

`script_score`의 선형 탐색 병목이 있는데, 단일 ES 7.x 시스템에서 해결하려 할 때 쓰던 방식으로 조건에 맞는 문서를 추린뒤, 실행 시간에 일일이 내적 dot product를 계산(brute-force) 하므로 데이터가 수십만건이 되어도 cpu 연산량이 폭증해 실서비스 응답시간 SLA를 맞출 수 없었다.

### Solution

**Lucene 코어에 HNSW 자료구조 통합**: ES 8.x부터 내부 코어인 Apace Lucene 엔진에 C/Java 레벨로 HNSW 그래프 알고리즘을 직접 이식해 이로 인해 ES 내부에서 벡터 탐색시 O(logN) 수준의 근사 검색 속도를 달성시켰다.

**Native Pre-filter를 포함한 단일 k-NN 실행**으로 ES는 쿼리 요청을 받으면 가벼운 역색인 inverted index를 통해 나이키가 해당하는 문서들의 BitSet(메모리 마스크)를 0.001초만에 먼저 생성하고 그 후 HNSW 그래프를 탐색할때 이 마스크를 적용해 조건에 맞지 않는 노드를 즉시 건너뛰는 방식으로 정확도와 속도를 동시에 보장한다.

#### 직관적인 예시로 이해해보겠다.

**과거의 아키텍처 시스템 분리 시절에는** 텍스트를 태그를 관리하는 알바생 ES와 옷의 생김새 벡터. 그리고 그걸 관리하는 알바생 FAISS가 물리적으로 존재했다. 나이키 중에서 비슷한 모양을 찾아줘라고 말하면, 태그 알바생이 나이키 리스트를 먼저 뽑아 벡터 알바생에게 명단을 건네주고 벡터 알바생(FAISS)가 자신의 차트와 그 명단을 대조해야 했다. 소통에 병목이 생기고 리스트가 길면 서버가 터진다.

**ES 8.x 네이티브 검색**은 이제 한 명의 통합 알바생 ES가 태그 장부 이미지 창고 안내도 HNSW를 전부 메모리에 들고있고 나이키 중에서 비슷한 모양!하면 장부에서 나이키 bitset 즉 구역을 확인한뒤 창고 안내도를 따라가다 나이크 구역이 아닌 길은 아예 진입조차 안하고 단번에 옷을 가져온다.

<br>

## 상세 동작 원리 및 구조화 

Elasticsearch 8.x 내부에서 Native Vector Search가 처리되는 Low-level 메커니즘이다.

#### 1. **Mapping** 정의 구조 할당 

`type: "dense_vector"` 필드에 `index: true` 속성을 부여하면 Lucene 백엔드에서 해당 필드용 HNSW 그래프를 저장할 디스크/메모리를 세그먼트 단위로 준비한다.

#### 2. **Indexing(색인)**

문서가 인제스트 될 때, 일반 필드는 역색인 트리로 들어가고, 벡터 필드는 HNSW 그래프의 노드로 삽입되어 이웃간의 edge 연결이 된다.

#### 3. Query Execution

- **Pre-filtering:** `knn` 블록 내부에 filter가 선언된 경우 Lucene은 역색인 조회로 조건에 맞는 문서들의 ID를 BitSet(1, 0 배열)으로 캐싱한다
- **마스킹 그래프 탐색**: HNSW는 꼭대기 층부터 탐색을 내겨라 때, 도달한 노드의 ID, BitSet이 1로 켜져있는지 확인하고 0이면 해당 노드의 거리는 무한대 infinity로 취급하여 탐색 스페이스에서 제외한다.

#### 4. Scoring 및 분산 병합

클러스터 내 각 샤드에서 탐색된 top-k 결과들을 코디네이팅 노드로 취합하여, 코사인 유사도 점수를 기반으로 재정렬한 후 클라이언트에게 반환한다.


### Example

인덱싱 매핑과 기본쿼리 예시를 알아보자. ES 클러스터에서 벡터 데이터를 저장하기 위한 공간을 설계 mapping 하고, 쿼리를 던지는 json 구조다.

`script_score`가 아닌 네이티브 knn 객체를 사용한다.

```json
// 1. 인덱스 생성 및 Mapping 정의 (REST API)
PUT /home_shopping_products
{
  "mappings": {
    "properties": {
      "product_id": { "type": "keyword" },
      "brand_name": { "type": "keyword" },
      "image_vector": {
        "type": "dense_vector",
        "dims": 512,
        "index": true,             // Lucene HNSW 인덱싱 활성화
        "similarity": "cosine",    // 코사인 유사도 기반 거리 계산
        "index_options": {         // HNSW 세부 파라미터 튜닝
          "type": "hnsw",
          "m": 16,
          "ef_construction": 100
        }
      }
    }
  }
}

// 2. 단일 벡터 k-NN 검색 쿼리 (Pre-filter 포함)
POST /home_shopping_products/_search
{
  "knn": {
    "field": "image_vector",
    "query_vector": [0.012, 0.453, -0.123, ...], // 512차원 벡터 배열
    "k": 10,                                     // 최종 반환할 문서 수
    "num_candidates": 100,                       // HNSW 탐색 노드 수 (정확도 vs 속도)
    "filter": {                                  // 그래프 탐색 전 적용할 역색인 필터
      "term": { "brand_name": "Nike" }
    }
  },
  "_source": ["product_id", "brand_name"]
}
```

두 번째로 위 코드에서 원리를 알아봤고 사용을 알아봤으니 파이썬 elasticsearch 클라이언트로 실무의 복합 요구사항 텍스트 BM25, 벡터 k-NN + RRF 병합 작업을 단일 쿼리로 처리하는 백엔드 데이터베이스 연동 클래스 예시를 살펴보자


```py
from elasticsearch import Elasticsearch
from typing import List, Dict, Any

class ElasticVectorSearchEngine:
    """
    Elasticsearch 8.x 기반의 하이브리드 검색 처리 엔진 (RRF 지원).
    """
    def __init__(self, es_client: Elasticsearch, index_name: str = "home_shopping_products"):
        self.es = es_client
        self.index_name = index_name

    def hybrid_search_with_filter(
        self, 
        vector_query: List[float], 
        keyword: str, 
        brand_filter: str, 
        top_k: int = 10
    ) -> List[Dict[str, Any]]:
        """
        벡터 유사도와 키워드 매칭을 결합하고 특정 브랜드로 필터링하는 하이브리드 쿼리를 실행합니다.
        """
        
        # 1. k-NN 쿼리 블록 (벡터 유사도 기반)
        # Pre-filter를 적용하여 해당 브랜드의 BitSet이 켜진 노드만 HNSW 그래프에서 탐색
        knn_query = {
            "field": "image_vector",
            "query_vector": vector_query,
            "k": top_k,
            "num_candidates": 50,
            "filter": {
                "term": { "brand_name": brand_filter }
            }
        }
        
        # 2. 전문 검색(Full-text Search) 쿼리 블록 (BM25 형태소 분석 기반)
        match_query = {
            "bool": {
                "must": [
                    { "match": { "product_name": keyword } }
                ],
                "filter": [
                    { "term": { "brand_name": brand_filter } }
                ]
            }
        }

        # 3. ES 8.x 하이브리드 쿼리 바디 구성
        # knn 파라미터와 query 파라미터를 동위에 선언.
        # rrf(Reciprocal Rank Fusion) 블록을 통해 두 검색 엔진의 결과를 순위 기반 정규화하여 병합
        request_body = {
            "knn": knn_query,
            "query": match_query,
            "rank": {
                "rrf": { 
                    "window_size": 50,
                    "rank_constant": 60
                }
            },
            "_source": ["product_id", "brand_name", "product_name"]
        }

        # 4. 쿼리 실행 및 파싱
        response = self.es.search(index=self.index_name, body=request_body)
        
        results = []
        # ES 8.x RRF 결과는 hit['_score']가 절대값이 아닌 병합된 역수 랭크 스코어입니다.
        for hit in response['hits']['hits']:
            results.append({
                "product_id": hit['_source']['product_id'],
                "brand": hit['_source']['brand_name'],
                "name": hit['_source']['product_name'],
                "hybrid_score": hit['_score'] 
            })
            
        return results

# --- 실무 API 라우터 사용 예시 ---
# es_client = Elasticsearch("https://localhost:9200", basic_auth=("elastic", "password"))
# search_engine = ElasticVectorSearchEngine(es_client)
# 
# @app.get("/search/hybrid")
# def search(keyword: str, brand: str):
#     # CLIP Text Encoder를 통해 검색어 벡터화 (약 15ms)
#     query_vector = text_encoder.encode(keyword) 
#     
#     # 하이브리드 검색 실행 (네트워크 I/O 포함 약 30ms 이내)
#     results = search_engine.hybrid_search_with_filter(
#         vector_query=query_vector, 
#         keyword=keyword, 
#         brand_filter=brand
#     )
#     return {"data": results}
```