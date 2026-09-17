# 모듈형 RAG 파이프라인 및 통합 인터페이스 설계

전사적으로 여러 제품팀 (A는 사내위키, B팀은 CS 고객이력, C팀은 상품 리뷰)에서 각자의 도메인 데이터로 RAG 시스템을 구축하려고 한다.

만약 각 팀이 직접 파이썬 랭체인을 설치하고, 벡터 데이터베이스 인스턴스를 따로 띄우며, 임베딩 파이프라인을 바닥부터 직접 개발해야 한다면 어떻게 될까?

전사적인 인프라 자원 낭비는 물론, 유지보수 불가 및 RAG 품질의 하향 평준화가 일어날 것이다.

이를 방지하고 사내 어느팀이든 레고 블록 조립하듯 즉각적으로 RAG를 도입할 수 있게 하려면 어떻게 플랫폼을 설계해야할까?

**위 문제를 해결하기 위해 도입하는 아키텍처가 모듈형 RAG 파이프라인 Modular RAG Pipeine 이다.**

데이터 수집, 임베딩, 벡터 검색, 프롬프트 조합, LLM 생성이라는 복잡한 파이프라인 플랫폼 팀이 중앙에서 표준화된 모듈로 분리하여 개발하고, 타 팀에서는 단순화된 통합 API 형태로 제공하는 엔지니어링 방법론이다.

<br>

## 문제 정의

각 제품팀이 RAG 컴포넌트를 파편화하여 개별 구현함으로 인해 발생하는 전사적인 기술 부채와 인프라 중복 투자가 발생할 수 있는 문제가 존재하고, 시스템 응답 지연 시간이나 검색 품질을 중앙에서 통제하고 개선할 수 없는 한계가 존재했다.

예를들면 A팀은 구형 텍스트 임베딩 모델을 사용하고 B팀은 최신 모델을 사용하면서 부서별로 검색 품질이 제각각이 되거나 A팀은 한 달동안 개발한 PDF 문서 파싱 및 벡터화된 로직을 B팀은 전혀 사용할 수 없어 동일한 코드를 처음부터 다시 작성해야하는 문제다.

### 문제 해결 방식

- **컴포넌트의 논리적 분리 (Decoupling)**: 전체 RAG 과정을 Ingestion(문서 청킹 임베딩), Retrieval(벡터 검색 및 재랭킹), Generation (프롬프트 주입 후 LLM 호출)의 독립된 모듈로 분리하고 각 모듈 인터페이스를 통해서만 통신하게 하여, 추후 검색 엔진이나 LLM이 변경되어도 다른 모듈에 영향을 주지 않도록 결합도를 낮춘다.
- **통합 추상화 레이어 (Facade API) 제공**: 플랫폼 팀은 복잡한 내부 로직을 캡슐화 해야한다. 타 제품 팀 개발자들은 내부 HNSW가 도는지, 어떤 임베딩 모델이 도는지 알 필요 없이, 사내 SDK나 REST API를 통해 `PlatformRAG.ask(query="question", domain="wiki")`라는 한 줄의 코드로 표준화된 RAG 결과를 반환받는다.

플랫폼 팀이 구축한 공통 RAG 인프라를 여러 팀이 호출하여 사용하는 데이터 및 제어 흐름이다.

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

1. **라우팅 및 권한 확인**: 타 팀의 요청이 들어오면 `Domain Router`가 해당 팀이 접근 권한이 있는 데이터 인덱스 `index_cs_history` or `index_wiki`를 식별한다.
2. **Retrieval(검색)**: Retrieval Module은 질문을 텍스트 임베딩 서비스 Triton으로 보내 벡터로 변환한 뒤 중앙 공유 Vector DB(Elasticsearch)에서 관련 문서를 추출한다. 필요시 Re-ranker를 거쳐 문서의 우선순위를 재정렬한다.
3. **Generation (생성)**: Generation Module은 검색된 문서들과 사용자의 질문을 Prompt Template Manager에게 전달하여 시스템 프롬프트를 조립하고 LLM Gateway로 요청을 넘겨 최종 답변을 생성한다.

### Example

플랫폼 백엔드 내부에서 각 기능을 독립된 클래스 모듈로 분리하고 이를 DI 형태로 조립하여 사용하는 설계 원리를 보면

```py
from typing import List, Dict

class BaseRetriever:
    def retrieve(self, query: str, top_k: int) -> List[str]:
        raise NotImplementedError

class BaseGenerator:
    def generate(self, context: List[str], query: str) -> str:
        raise NotImplementedError

# 구체적 모듈 구현 elasticsearch
class ElaticsearchRetriever(BaseRetriever):
    def __init__(self, es_client, index_name):
        self.es = es_client
        self.index = index_name
    
    def retrieve(self, query: str, top_k: int = 3) -> List[str]:
        # 내부적으로 임베딩 및 ES k-NN 검색 수행 로직 (생략)
        print(f"[{self.index}] 인덱스에서 '{query}' 관련 문서 검색 중...")
        return ["문서 내용 1", "문서 내용 2"]

# 3. 구체적인 모듈 구현 (LLM Gateway 연동 모듈)
class GatewayGenerator(BaseGenerator):
    def __init__(self, gateway_url):
        self.gateway_url = gateway_url
        
    def generate(self, context: List[str], query: str) -> str:
        prompt = f"Context: {context}\nQuestion: {query}\nAnswer:"
        print("LLM Gateway로 컨텍스트 및 질문 전송하여 답변 생성 중...")
        # API 호출 로직 (생략)
        return "RAG 기반 최종 답변입니다."

# 4. 파이프라인 조립기 (Orchestrator)
class ModularRAGPipeline:
    def __init__(self, retriever: BaseRetriever, generator: BaseGenerator):
        self.retriever = retriever
        self.generator = generator
        
    def run(self, query: str) -> str:
        # 모듈 1: 검색
        context = self.retriever.retrieve(query)
        # 모듈 2: 생성
        answer = self.generator.generate(context, query)
        return answer

# 내부 시스템에서의 조립 예시
# wiki_retriever = ElasticsearchRetriever(es, "wiki_data")
# llm_gen = GatewayGenerator("http://llm-gateway:4000")
# wiki_rag_pipeline = ModularRAGPipeline(wiki_retriever, llm_gen)
```

RAG 파이프라인 복잡성을 완전히 걷어내고, 사내 타 개발팀이 rest api를 통해 직관저그올 RAG플랫폼을 사용하는 형태 코드도 보자

타 팀은 내부 아키텍처를 몰라도 도메인 이름과 쿼리만으로 고품질 RAG 시스템을 즉시 서비스에 연동할 수 있다.

```py
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class RAGRequest(BaseModel):
    domain: str  # 접근하려는 데이터셋 (예: "hr_wiki", "cs_manual")
    query: str
    top_k: int = 3

@app.post("/api/v1/rag/ask")
def platform_rag_ask(request: RAGRequest):
    """사내 공통 RAG 처리 엔드포인트"""
    
    # 1. 도메인별 설정된 파이프라인 로드 (라우팅)
    pipeline = get_pipeline_for_domain(request.domain)
    if not pipeline:
        raise HTTPException(status_code=404, detail="등록되지 않은 도메인입니다.")
    
    # 2. 캡슐화된 RAG 파이프라인 실행
    answer = pipeline.run(query=request.query)
    
    # 3. 메타데이터(참고 문서 등)와 함께 결과 반환
    return {
        "status": "success",
        "domain": request.domain,
        "answer": answer,
        "metadata": {
            "retrieved_docs_count": request.top_k,
            # 실제 서비스에서는 추적을 위한 trace_id 등을 포함
            "trace_id": "req-12345" 
        }
    }
```

그리고 실무에서는 redis 젖아 조회 로직을 직접 구현하지 않고도 LangGraph가 제공하는 `Checkpointer` 객체 (RedisSaver, PostgreSaver등) 컴파일 시점에 주입하여 모든 io를 프레임워크에 위임한다.

```py
from langgraph.graph import StateGraph
from langgraph.checkpoint.postgres import PostgresSaver
from psycopg_pool import ConnectionPool

# 1. State와 Node 정의 (이전과 동일)
workflow = StateGraph(AgentState)
workflow.add_node("llm_node", llm_reasoning_node)
# ... 노드 및 엣지 연결 생략 ...

# 2. 프로덕션용 Checkpointer 연결 (PostgreSQL 예시)
# DB 커넥션 풀을 생성하고 프레임워크에 주입합니다.
pool = ConnectionPool("postgresql://user:pass@host/db")
checkpointer = PostgresSaver(pool)

# 3. 그래프 컴파일 (이 시점에 상태 자동 저장 로직이 내장됨)
app = workflow.compile(checkpointer=checkpointer)

# 4. 실무 API 호출 방식
# 개발자는 단순히 thread_id만 config로 넘기면, 
# DB에서 상태를 불러오고, 노드 실행 후 다시 덮어쓰는 전 과정이 자동화됩니다.
config = {"configurable": {"thread_id": "thread_12345"}}
final_state = app.invoke({"messages": ["고객 데이터 조회해줘"]}, config=config)
```