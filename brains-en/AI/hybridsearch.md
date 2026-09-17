# Elasticsearch 8.x Native Vector Search and Hybrid Architecture

Let's say you reduced vector search to 10ms by introducing FAISS. But the planning team asks, "Please find similar images among Nike brand products!"

In a real architecture, FAISS is a C++ library that only loads float arrays (vectors) and integer IDs into memory. It inherently lacks the functionality to store or filter text metadata like 'Nike' as a brand name.

Conversely, while existing search engines like ES7.x and below are perfect for text filtering, they don't have the core HNSW algorithm implemented. This leads to a linear search O(N) bottleneck, scanning all documents when comparing embeddings.

So, if you need to process "similar images among Nike products" within 10ms with a single query, without forcibly gluing together two systems with critical flaws...

**The answer to the system constraints based on the facts above is Elasticsearch 8.x's Native Vector Search feature.**

It's a distributed search architecture that directly indexes deep learning embedding vectors into the core of a text search engine, simultaneously handling vector similarity search and text filtering within the physically same database engine.

- **Native Vector Search**: A feature that natively supports vector indexing and search directly within the Lucene core engine of the database, without external plugins like FAISS or workaround scripts.
- `dense_vector`: A dedicated data type for storing deep learning embedding results, where float values densely fill the dimensions, unlike sparse arrays.
- **Hybrid Search**: A technique that combines traditional keyword matching (BM25 score) and embedding vector matching (k-NN score) to produce final search results.
- **RRF (Reciprocal Rank Fusion):** A normalization algorithm that calculates a new score by taking the reciprocal based on rank, instead of absolute scores, to combine two search results with different scales (BM25 score and cosine similarity score).

<br>

## The Problem

**Two-System Architecture Consistency Problem**: Previously, it was necessary to operate two separate servers: ES for text and FAISS for vectors. In this scenario, both post-filtering (finding 1000 items in FAISS first, then filtering for Nike in ES -> potentially resulting in 0 Nike items) and pre-filtering (finding 100,000 Nike items in ES, then passing 100,000 IDs to FAISS for vector search -> network memory explosion) methods caused critical search omissions or latency.

There's a linear search bottleneck with `script_score`. When attempting to solve this in a single ES 7.x system, after filtering documents that meet the conditions, dot products were calculated individually at runtime (brute-force). This led to a surge in CPU computation even with hundreds of thousands of data points, making it impossible to meet real-service response time SLAs.

### Solution

**HNSW Data Structure Integration into Lucene Core**: Starting with ES 8.x, the HNSW graph algorithm was directly ported to the internal core, the Apache Lucene engine, at the C/Java level. This achieved approximate search speeds of O(logN) for vector search within ES.

**Single k-NN Execution with Native Pre-filter**: When ES receives a query request, it first generates a BitSet (memory mask) of documents corresponding to 'Nike' in 0.001 seconds via a lightweight inverted index lookup. Then, when traversing the HNSW graph, this mask is applied, immediately skipping nodes that don't match the condition, thereby ensuring both accuracy and speed.

#### Let's understand with an intuitive example.

**In the past era of separated architecture systems**, there were physically separate part-time workers: ES managing text tags, and FAISS managing clothing appearance vectors. If you asked to "find similar shapes among Nike products!", the tag part-timer would first extract a list of Nike items and hand it over to the vector part-timer. The vector part-timer (FAISS) then had to compare their chart with that list. This created communication bottlenecks, and if the list was long, the server would crash.

**ES 8.x native search** now has a single, integrated part-timer, ES, holding all the tag ledgers, image warehouse guides, and HNSW in memory. If you ask "similar shapes among Nike!", it checks the Nike bitset (i.e., the section) in the ledger, then follows the warehouse guide, completely avoiding paths that are not in the Nike section, and retrieves the clothes instantly.

<br>

## Detailed Operation Principle and Structuring

This is the low-level mechanism by which Native Vector Search is processed within Elasticsearch 8.x.

#### 1. **Mapping** Definition Structure Allocation

When the `index: true` attribute is assigned to a `type: "dense_vector"` field, the Lucene backend prepares disk/memory for storing the HNSW graph for that field, on a segment-by-segment basis.

#### 2. **Indexing**

When a document is ingested, general fields go into the inverted index tree, and vector fields are inserted as nodes in the HNSW graph, establishing edge connections between neighbors.

#### 3. Query Execution

- **Pre-filtering:** If a filter is declared within the `knn` block, Lucene caches the IDs of matching documents as a BitSet (1, 0 array) via an inverted index lookup.
- **Masked Graph Traversal**: When HNSW traverses from the top layer downwards, it checks if the ID of the reached node in the BitSet is set to 1. If it's 0, the distance to that node is treated as infinity, excluding it from the search space.

#### 4. Scoring and Distributed Merging

The top-k results found in each shard within the cluster are aggregated at the coordinating node, re-sorted based on cosine similarity scores, and then returned to the client.

### Example

Let's look at an example of indexing mapping and a basic query. This is a JSON structure for designing the mapping to store vector data in an ES cluster and for submitting queries.

It uses the native `knn` object, not `script_score`.

```json
// 1. Create Index and Define Mapping (REST API)
PUT /home_shopping_products
{
  "mappings": {
    "properties": {
      "product_id": { "type": "keyword" },
      "brand_name": { "type": "keyword" },
      "image_vector": {
        "type": "dense_vector",
        "dims": 512,
        "index": true,             // Activate Lucene HNSW indexing
        "similarity": "cosine",    // Distance calculation based on cosine similarity
        "index_options": {         // HNSW detailed parameter tuning
          "type": "hnsw",
          "m": 16,
          "ef_construction": 100
        }
      }
    }
  }
}

// 2. Single Vector k-NN Search Query (with Pre-filter)
POST /home_shopping_products/_search
{
  "knn": {
    "field": "image_vector",
    "query_vector": [0.012, 0.453, -0.123, ...], // 512-dimensional vector array
    "k": 10,                                     // Number of documents to return
    "num_candidates": 100,                       // Number of HNSW search nodes (accuracy vs. speed)
    "filter": {                                  // Inverted index filter to apply before graph traversal
      "term": { "brand_name": "Nike" }
    }
  },
  "_source": ["product_id", "brand_name"]
}
```

Next, having understood the principles and usage from the code above, let's look at an example of a backend database integration class using the Python Elasticsearch client to handle complex real-world requirements like text BM25, vector k-NN + RRF merging in a single query.

```py
from elasticsearch import Elasticsearch
from typing import List, Dict, Any

class ElasticVectorSearchEngine:
    """
    Hybrid search processing engine based on Elasticsearch 8.x (RRF supported).
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
        Executes a hybrid query combining vector similarity and keyword matching, filtered by a specific brand.
        """
        
        # 1. k-NN Query Block (vector similarity based)
        # Apply Pre-filter to search only nodes with the brand's BitSet enabled in the HNSW graph
        knn_query = {
            "field": "image_vector",
            "query_vector": vector_query,
            "k": top_k,
            "num_candidates": 50,
            "filter": {
                "term": { "brand_name": brand_filter }
            }
        }
        
        # 2. Full-text Search Query Block (BM25 morphological analysis based)
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

        # 3. Construct ES 8.x Hybrid Query Body
        # Declare knn and query parameters at the same level.
        # Merge results from two search engines via rank-based normalization using the rrf (Reciprocal Rank Fusion) block
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

        # 4. Execute and Parse Query
        response = self.es.search(index=self.index_name, body=request_body)
        
        results = []
        # ES 8.x RRF results have a merged reciprocal rank score, not an absolute score, in hit['_score'].
        for hit in response['hits']['hits']:
            results.append({
                "product_id": hit['_source']['product_id'],
                "brand": hit['_source']['brand_name'],
                "name": hit['_source']['product_name'],
                "hybrid_score": hit['_score'] 
            })
            
        return results

# --- Real-world API Router Usage Example ---
# es_client = Elasticsearch("https://localhost:9200", basic_auth=("elastic", "password"))
# search_engine = ElasticVectorSearchEngine(es_client)
# 
# @app.get("/search/hybrid")
# def search(keyword: str, brand: str):
#     # Vectorize search query via CLIP Text Encoder (approx. 15ms)
#     query_vector = text_encoder.encode(keyword) 
#     
#     # Execute hybrid search (approx. within 30ms including network I/O)
#     results = search_engine.hybrid_search_with_filter(
#         vector_query=query_vector, 
#         keyword=keyword, 
#         brand_filter=brand
#     )
#     return {"data": results}
```
