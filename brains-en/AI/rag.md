# RAG Advancement Strategies

When introducing AI products in the field, the biggest hurdle is the quality of RAG.

A system that worked well with a few text files in a controlled test environment starts hallucinating or referencing irrelevant documents the moment it encounters a client's massive and fragmented data.

We need to design a robust data pipeline to bridge this gap.

### Advanced RAG Beyond Chunking

Early RAG models (Native RAG) simply cut documents into fixed character lengths (chunking), embed them, store them in a vector DB, and then search using cosine similarity.

However, in an enterprise environment, this approach alone has clear limitations.

#### Hybrid Search

Vector search (Semantic Search) is excellent at finding semantic similarity, but it is very weak at exact keyword matching (Exact Match) for client-specific modeling (e.g., ALF-2024-X), specific error codes, or proper nouns. To compensate for this, hybrid search, which combines traditional keyword-based search (Lexical Search) like BM25 with vector search to merge results, is essential.

#### Re-ranking

After quickly retrieving dozens of relevant document chunks through an initial search, this process re-evaluates contextual relevance and ranks them using more accurate models like cross-encoders. It's a technique that resolves the trade-off between search speed (recall) and accuracy (precision).

#### Query Expansion and Routing

This technique involves not directly putting a user's short and ambiguous question into search, but instead passing it through an LLM once to expand it into multiple specific synonym queries (query rewrite), or establishing a routing layer to determine whether the question should query a database or internal documents.

### Designing a Pipeline for Processing Client-Specific Domain Data

Client data comes in various forms, such as PDFs, Confluence documents, Jira tickets, and internal databases, and often includes unstructured data like tables and images.

#### Data Extraction and Preprocessing (ETL for RAG)

Accurately parsing complex tables or multi-column layouts within PDFs is extremely important. If data within a table is corrupted, AI will never be able to provide correct answers. Recently, parsers that recognize document structures (headings, paragraphs, tables) and convert them into Markdown format have been introduced in preprocessing, or strategies are used to convert table data into JSON format for chunking.

#### Metadata Filtering

Documents are stored in a vector DB not just with plain text, but also with metadata such as category, creation date, department name, and document status. Applying pre-filtering with metadata during search can significantly reduce the search space, thereby increasing accuracy and reducing costs.

- **Role-Based Access Control (RBAC) in RAG**: This is the most sensitive aspect in a B2B SaaS environment, especially for enterprise messengers like Channel Talk. Financial data documents accessible only to executives should not be used as a source for answering questions from regular employees. When storing document chunks, access permission metadata must be enforced, and tenant isolation must be guaranteed at the system architecture level to ensure that only documents matching the permissions of the user requesting the query are returned during search.
- **Data Freshness and Synchronization (Freshness, CDC/Webhook)**: When a client's refund policy or manual is updated, the contents of the vector DB must be synchronized without delay. This requires designing an event-driven pipeline that detects changes in the data source, invalidates existing embedding data, and then re-learns it. Referring to outdated data and providing incorrect information can lead to critical failures.
