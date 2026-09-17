# Modular RAG Pipeline and Integrated Interface Design

Across the enterprise, multiple product teams (Team A for internal wiki, Team B for CS customer history, Team C for product reviews) are trying to build RAG systems with their respective domain data.

What if each team had to directly install Python LangChain, spin up separate vector database instances, and develop embedding pipelines from scratch?

This would lead to not only wasted enterprise-wide infrastructure resources but also unmaintainability and a downward leveling of RAG quality.

To prevent this and enable any team within the company to adopt RAG instantly, like assembling LEGO blocks, how should the platform be designed?

**The architecture introduced to solve the above problem is the Modular RAG Pipeline.**

It is an engineering methodology where a platform team centrally develops complex pipelines—data collection, embedding, vector search, prompt orchestration, and LLM generation—by separating them into standardized modules, and then provides them to other teams in the form of a simplified, integrated API.

<br>

## Problem Definition

There was a problem where each product team's fragmented and individual implementation of RAG components could lead to enterprise-wide technical debt and redundant infrastructure investment, along with limitations in centrally controlling and improving system response times or search quality.

For example, Team A might use an older text embedding model while Team B uses the latest one, resulting in inconsistent search quality across departments, or Team B might be unable to use the PDF document parsing and vectorization logic developed by Team A over a month, forcing them to rewrite the same code from scratch.

### Solution Approach

- **Logical Decoupling of Components**: The entire RAG process is separated into independent modules—Ingestion (document chunking and embedding), Retrieval (vector search and re-ranking), and Generation (prompt injection and LLM call). Communication occurs only through each module's interface, reducing coupling so that future changes to the search engine or LLM do not affect other modules.
- **Provision of an Integrated Abstraction Layer (Facade API)**: The platform team must encapsulate complex internal logic. Developers from other product teams do not need to know whether internal HNSW is running or which embedding model is being used; they receive standardized RAG results with a single line of code, `PlatformRAG.ask(query="question", domain="wiki")`, via an internal SDK or REST API.

This is the data and control flow where multiple teams call and use the common RAG infrastructure built by the platform team.

```mermaid
graph TD
    ClientA[A팀 Application: CS 챗봇] -->|REST API| RAG_API[RAG Platform API Gateway]
    ClientB[B팀 Application: 사내 위키] -->|REST API| RAG_API
    
    subgraph "Modular RAG Platform (Core)"
        RAG_API --> Router[Domain Router]
        Router --> Retriever[Retrieval Module]
        Router --> Generator[Generation Module]
        
        Retriever --> Embedder[Embedding Service]
        Retriever --> ReRanker[Cross-Encoder Re-ranker]
        
        Generator --> PromptMgr[Prompt Template Manager]
        Generator --> LLM_Gateway[LLM Gateway - Step 1]
    end
    
    subgraph "Shared Infrastructure"
        Embedder --> Triton[Triton Server: BGE-m3]
        Retriever --> VectorDB[(ES 8.x Vector Index)]
        LLM_Gateway --> SaaS_LLM[OpenAI / vLLM]
    end
```

1. **Routing and Authorization**: When a request from another team comes in, the `Domain Router` identifies the data index (`index_cs_history` or `index_wiki`) that the team has access to.
2. **Retrieval**: The Retrieval Module sends the query to the Triton text embedding service to convert it into a vector, then extracts relevant documents from the central shared Vector DB (Elasticsearch). If necessary, it reorders the document priorities via a Re-ranker.
3. **Generation**: The Generation Module passes the retrieved documents and the user's query to the Prompt Template Manager to assemble the system prompt, then forwards the request to the LLM Gateway to generate the final answer.

### Example

Looking at the design principle where each function within the platform backend is separated into independent class modules and assembled using DI:

```py
from typing import List, Dict

class BaseRetriever:
    def retrieve(self, query: str, top_k: int) -> List[str]:
        raise NotImplementedError

class BaseGenerator:
    def generate(self, context: List[str], query: str) -> str:
        raise NotImplementedError

# Concrete module implementation: Elasticsearch
class ElaticsearchRetriever(BaseRetriever):
    def __init__(self, es_client, index_name):
        self.es = es_client
        self.index = index_name
    
    def retrieve(self, query: str, top_k: int = 3) -> List[str]:
        # Internal logic for embedding and ES k-NN search (omitted)
        print(f"Searching for documents related to '{query}' in [{self.index}] index...")
        return ["문서 내용 1", "문서 내용 2"]

# 3. Concrete module implementation (LLM Gateway integration module)
class GatewayGenerator(BaseGenerator):
    def __init__(self, gateway_url):
        self.gateway_url = gateway_url
        
    def generate(self, context: List[str], query: str) -> str:
        prompt = f"Context: {context}\nQuestion: {query}\nAnswer:"
        print("Sending context and query to LLM Gateway to generate answer...")
        # API call logic (omitted)
        return "This is the final RAG-based answer."

# 4. Pipeline Assembler (Orchestrator)
class ModularRAGPipeline:
    def __init__(self, retriever: BaseRetriever, generator: BaseGenerator):
        self.retriever = retriever
        self.generator = generator
        
    def run(self, query: str) -> str:
        # Module 1: Retrieval
        context = self.retriever.retrieve(query)
        # Module 2: Generation
        answer = self.generator.generate(context, query)
        return answer

# Example of assembly in an internal system
# wiki_retriever = ElasticsearchRetriever(es, "wiki_data")
# llm_gen = GatewayGenerator("http://llm-gateway:4000")
# wiki_rag_pipeline = ModularRAGPipeline(wiki_retriever, llm_gen)
```

Let's also look at the code where the RAG pipeline complexity is completely removed, and other development teams within the company use the RAG platform intuitively via a REST API.

Other teams can instantly integrate a high-quality RAG system into their services with just a domain name and a query, without needing to know the internal architecture.

```py
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class RAGRequest(BaseModel):
    domain: str  # Dataset to access (e.g., "hr_wiki", "cs_manual")
    query: str
    top_k: int = 3

@app.post("/api/v1/rag/ask")
def platform_rag_ask(request: RAGRequest):
    """Common RAG processing endpoint for internal use"""
    
    # 1. Load domain-specific pipeline (routing)
    pipeline = get_pipeline_for_domain(request.domain)
    if not pipeline:
        raise HTTPException(status_code=404, detail="Unregistered domain.")
    
    # 2. Execute encapsulated RAG pipeline
    answer = pipeline.run(query=request.query)
    
    # 3. Return results with metadata (e.g., referenced documents)
    return {
        "status": "success",
        "domain": request.domain,
        "answer": answer,
        "metadata": {
            "retrieved_docs_count": request.top_k,
            # In a real service, include trace_id for tracking, etc.
            "trace_id": "req-12345" 
        }
    }
```

And in practice, instead of directly implementing Redis storage and retrieval logic, you can inject LangGraph's `Checkpointer` objects (like RedisSaver, PostgresSaver) at compile time, delegating all I/O to the framework.

```py
from langgraph.graph import StateGraph
from langgraph.checkpoint.postgres import PostgresSaver
from psycopg_pool import ConnectionPool

# 1. Define State and Node (same as before)
workflow = StateGraph(AgentState)
workflow.add_node("llm_node", llm_reasoning_node)
# ... Node and edge connection omitted ...

# 2. Connect Checkpointer for production (PostgreSQL example)
# Create a DB connection pool and inject it into the framework.
pool = ConnectionPool("postgresql://user:pass@host/db")
checkpointer = PostgresSaver(pool)

# 3. Compile the graph (automatic state saving logic is embedded at this point)
app = workflow.compile(checkpointer=checkpointer)

# 4. Practical API call method
# Developers simply pass the thread_id in the config,
# and the entire process of loading state from the DB, executing nodes, and overwriting it again is automated.
config = {"configurable": {"thread_id": "thread_12345"}}
final_state = app.invoke({"messages": ["Retrieve customer data"]}, config=config)
```
