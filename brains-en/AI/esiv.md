# Elasticsearch Indexing & Vector Search

We've discussed LSH, CNN, etc., above. Let's explore, from a system engineering perspective, how high-dimensional vectors and hash signatures are stored on disk in a real distributed system environment and retrieved in milliseconds.

Elasticsearch is a distributed, RESTful search and analytics engine that uses the Apache Lucene library as its core.

In an image search pipeline, ES indexes and stores converted hash values (LSH) or high-dimensional real-valued vectors (CNN Features), and when a user's query image comes in, it acts as a database that ultra-fast finds the most similar data.

> Elasticsearch's name is derived from the combination of its ability to automatically distribute sharding and replicas when node servers are added or removed, allowing for elastic adjustment of system scale, and its search capabilities.
>
> Indexing is the act of, like an index at the back of a book, pre-converting data into a specific data structure optimized for search, such as an inverted index or a graph, and mapping it to disk and memory when it is stored.

### Problem Definition

The problem ES aims to solve can first be understood by looking at **the limitations of B-Trees**. RDBMSs are based on B-Tree indexes, which are optimized for comparing and sorting one-dimensional numbers or string sizes, but are completely unusable for calculating spatial similarity (distance) between multi-dimensional arrays (vectors).

There's also a bottleneck with linear search. If there are 10 million product images, running a distance calculation formula in an RDBMS requires reading all records from disk and performing computations on the CPU, which can take several minutes to tens of minutes per query.

> B-Trees are structures specialized in comparing the magnitude of one-dimensional data like numbers and strings to branch and sort trees. However, vectors of hundreds or thousands of dimensions exported from deep learning cannot be compared by a single value size, and in multi-dimensional space, Euclidean distance or cosine angle, which combine all elements, must be calculated. Therefore, B-Tree search logic is completely inapplicable.

### Solution Approach

ES's indexing methods for image search are broadly divided into two structures: the classical LSH method and the native vector method supported by modern ES.

#### Inverted Indexing using LSH Hash Values

If the LSH algorithm converts a 2048-dimensional vector into binary string tokens like `10110`, ES treats them like ordinary text words and places them into an inverted index structure.

It maps "which hash token is included in which document ID" and instantly filters candidate sets using intersection/union bitwise operations.

#### Dense Vector and HNSW Graph Method (Modern Approach)

Modern ES (7.x and above) can index 2048-dimensional real-valued vectors directly from deep learning as `dense_vector` types, without hashing.

Internally, through an algorithm called HNSW (Hierarchical Navigable Small World), vectors are linked into a hierarchical graph in memory, and similar vectors are found by skipping through higher layers, much like a skip list.

<br>

## Detailed Component Operation Principles and Structure low-level architecture

### Inverted Index Structure and Posting List

It consists of a Dictionary (collection of hash tokens) and a Posting List (array of document IDs containing that token).

- Term 10110 -> [ Doc1, Doc5, Doc102 ]
- Term 11100 -> [ Doc2, Doc5, Doc88 ]

If the LSH hashes for a query image are `10110` and `11100`, ES does not search through all data, but only loads these two posting lists into memory.

**Skip-list, Bitset Intersection Operation:** To find common documents like `Doc5` in both lists, Lucene places Skip-list pointers within the posting lists to reduce search speed to $O(\log N)$, and a Bitset AND operation utilizing CPU's `SIMD` instructions calculates the intersection at ultra-high speed.

> SIMD stands for Single Instruction, Multiple Data. It is a CPU hardware technology that processes multiple data items in parallel with a single instruction. While typical computation processes data items one by one in a loop, SIMD loads a long data array (vector) entirely into CPU registers and computes it simultaneously in a single cycle. ES actively utilizes SIMD to drastically boost search speed when calculating intersections of LSH hash bitmaps or computing similarity distances between multi-dimensional vectors.

### HNSW (Hierarchical Navigable Small World) Algorithm Vector Search

It creates a graph. It connects data points that are close in vector space with edges, forming a spiderweb-like graph.

It then structures layers, dividing nodes into multiple levels.

- **Top Layer 3**: Contains only a sparse few nodes, acting as entry points.
- **Layer 2:** Slightly denser nodes.
- **Layer 0** (Bottom Layer): The original graph where all data is densely connected.

**Search Mechanism**

1. Starts from an arbitrary node in the top layer and moves to the node closest to the query vector (greedy routing).
2. If no closer node is found, it drops down to the layer immediately below.
3. This is repeated until a local optimum is reached in the bottom layer (Layer 0).

This method maximizes pointer jumps in RAM instead of disk I/O, enabling nearest neighbor search within a few milliseconds even with hundreds of millions of vector data points.

### Example

This code designs an index and performs searches, including both a field for storing product images as LSH hash tokens and a field for storing original dense vectors using the HNSW method.

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
