# Multi-tenant LLM Platform Environment and Resource Isolation

When multiple internal teams share a single RAG/Agent platform, how can we prevent one team's traffic surge from exceeding the external LLM API rate limit for the entire platform, thereby paralyzing other teams' services, and how can we accurately bill tens of millions of won in monthly token costs to each department?

The architecture that solves this problem is a **multi-tenancy-based resource isolation and observability pipeline. It is a technology that logically separates and controls a single physical AI platform so that multiple tenants can use it as if they were completely independent systems.**

- **Multi-tenancy**: An architecture where multiple user groups independently use a single physical software instance (platform) as if it were their dedicated server. In an LLM environment, this means logically partitioning API keys, maximum tokens per minute, and user contexts.
- **Token Bucket Algorithm**: A core algorithm for implementing rate limiting. Instead of simply limiting calls to 100 per minute, it fills a bucket with tokens at a steady rate and deducts tokens proportional to the text length for each incoming request. This allows for temporary traffic bursts while precisely controlling long-term overload.
- **LLM Observability**: The concept of inferring internal system states by only looking at external output logs. Unlike traditional APM, LLM observability focuses on tracking four-dimensional data—cost (token usage), prompt chain execution order (trace), quality of retrieved context, and response latency (time to first token)—as a unified whole.

<br>

## Problem Definition

Due to the structure of sharing external LLMs with a single set of credentials, a noisy neighbor problem existed where excessive usage by one team directly led to service paralysis for other teams.

Existing black-box logging systems had systemic limitations, making it impossible to analyze prompt efficiency per department or calculate infrastructure costs.

e.g., Team A accidentally ran a large-scale text summarization script over the weekend, exhausting OpenAI's per-minute request limits (RPM/TPM). This caused a cascading failure, bringing down Team B's customer service chatbot, which was operating normally. Furthermore, the logs only contained simple HTTP 200/500 status codes, making it technically impossible to prove which team caused the millions of won in token charges billed at the end of the month.

### Solving

- **Dynamic Control and Token Bucket Isolation at the API Gateway Level**: A gateway positioned at the front of the platform inspects the headers of incoming requests to identify the tenant. It creates a token bucket for each tenant in an in-memory DB, and as soon as an allocated quota is exceeded, only that tenant's requests are blocked with a 429 Too Many Requests status code, protecting other tenants.
- **OpenTelemetry-based Asynchronous Distributed Tracing Logging**: Once an LLM call is completed, the gateway extracts the usage object (prompt tokens, completion tokens) from the response data. Instead of simply logging this to a text file, it structures who (tenant ID), what flow (trace ID), and how much was used into a JSON-formatted OpenTelemetry standard specification and asynchronously sends it to observability-specific databases like Langfuse or Elasticsearch.

<br>

## Detailed Operating Principles and Structuring

Looking at the logical flow where a gateway controls traffic in a multi-tenant environment and generates and stores structured JSON logs for cost settlement and analysis:

```mermaid
graph TD
    ClientA[A팀: CS 챗봇] -->|Tenant: team-cs| Gateway[Platform API Gateway]
    ClientB[B팀: 사내 위키] -->|Tenant: team-wiki| Gateway
    
    subgraph "Platform Multi-tenant Layer"
        Gateway --> Auth[Tenant Router]
        Auth --> RateLimiter[Redis Rate Limiter\n(Token Bucket 검사)]
        RateLimiter -->|할당량 내| LLM_Call[LLM API Call]
        RateLimiter -.->|할당량 초과| Reject[429 차단 및 보호]
        LLM_Call --> Logger[Async Observability Logger]
    end
    
    LLM_Call <-->|추론 및 토큰 소비| ExternalLLM[OpenAI / vLLM]
    Logger -->|구조화된 JSON 비동기 전송| ObservabilityDB[(Elasticsearch / Langfuse)]
```

This data schema must exist for the platform team to create departmental billing dashboards and prompt optimization reports.

```json
{
  "trace_id": "req-98765-abcd",
  "tenant_id": "team-cs",
  "project_name": "refund_bot_v2",
  "timestamp": "2026-04-22T15:10:22Z",
  "request_details": {
    "model_routed": "gpt-4-turbo",
    "latency_ms": 1250,
    "status_code": 200
  },
  "usage_metrics": {
    "prompt_tokens": 1500,
    "completion_tokens": 250,
    "total_tokens": 1750,
    "estimated_cost_usd": 0.0225
  }
}
```

Let's look at the token bucket-based tenant isolation logic.

This is the algorithmic principle for controlling tenant resources by estimating text length (TPM) rather than just RPM limits based on call count.

```py
import redis
import time

redis_client = redis.Redis(host='localhost', port=6379, db=1)

def check_tenant_token_bucket(tenant_id: str, estimated_tokens: int) -> bool:
    """
    Redis 고정 윈도우 방식으로 테넌트별 분당 토큰(TPM) 한도를 논리적으로 격리합니다.
    (실무에서는 더 정교한 Leaky Bucket이나 Token Bucket 알고리즘을 사용합니다.)
    """
    MAX_TPM_QUOTA = 50000  # 팀당 분당 5만 토큰 제한
    current_minute = int(time.time() / 60)
    
    # 테넌트와 시간을 결합한 논리적 격리 키 (예: quota:team-cs:2839402)
    redis_key = f"quota:{tenant_id}:{current_minute}"
    
    # 트랜잭션(Atomic)으로 토큰 사용량 누적
    current_usage = redis_client.incrby(redis_key, estimated_tokens)
    
    # 해당 분(minute)의 첫 요청일 때만 TTL(60초) 설정하여 메모리 누수 방지
    if current_usage == estimated_tokens:
        redis_client.expire(redis_key, 60)
        
    # 누적 사용량이 할당된 쿼터를 넘어서면 해당 테넌트만 차단
    if current_usage > MAX_TPM_QUOTA:
        return False # Noisy Neighbor 차단 성공
        
    return True # 정상 통과
```

Let's also look at the code for a FastAPI-based Gateway that identifies tenants via headers and logs asynchronous token usage without disrupting user responses.

```py
from fastapi import FastAPI, HTTPException, Header, BackgroundTasks
import json

app = FastAPI()

def async_log_to_observability_db(log_data: dict):
    """
    사용자 API 응답 지연을 막기 위해 백그라운드 스레드에서 로그를 DB에 적재합니다.
    실무에서는 ELK 스택의 Bulk API나 Langfuse SDK를 통해 큐(Queue) 방식으로 전송합니다.
    """
    print(f"[Observability Logger] DB 적재 완료: {json.dumps(log_data)}")

@app.post("/api/v1/platform/chat")
async def platform_chat(
    prompt: str, 
    background_tasks: BackgroundTasks,
    x_tenant_id: str = Header(...) # 클라이언트가 헤더로 자신의 팀 ID를 명시 (필수)
):
    # 1. 테넌트 권한 및 Token Bucket Rate Limit 검사
    # 프롬프트 길이를 대략 계산하여 예산을 먼저 확인 (글자 수 / 4 ≈ 토큰 수)
    estimated_tokens = len(prompt) // 4 
    if not check_tenant_token_bucket(x_tenant_id, estimated_tokens):
        raise HTTPException(status_code=429, detail="팀별 분당 토큰 할당량을 초과했습니다.")
        
    # 2. 실제 LLM 추론 엔진 호출 (OpenAI API 또는 자체 호스팅 vLLM)
    # response = llm_engine.generate(prompt)
    mock_response = {
        "text": "RAG 기반 응답입니다.",
        "usage": {"prompt_tokens": estimated_tokens, "completion_tokens": 150, "total": estimated_tokens + 150}
    }
    
    # 3. 비용 정산(Chargeback)을 위한 관측성 JSON 로그 구조화
    log_payload = {
        "trace_id": "req-123", # 추적 ID (실제로는 uuid 등 생성)
        "tenant_id": x_tenant_id,
        "model": "gpt-4-turbo",
        "usage": mock_response["usage"]
    }
    
    # 4. 클라이언트에게는 즉시 응답을 반환하고, 로그 전송은 백그라운드 워커에 위임(Async)
    background_tasks.add_task(async_log_to_observability_db, log_payload)
    
    return {"response": mock_response["text"]}
```
