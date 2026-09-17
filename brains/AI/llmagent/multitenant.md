# 멀티 테넌트(Multi-tenant) LLM 플랫폼 환경과 자원 격리

사내 여러팀이 단일 RAG/Agent 플랫폼을 공유할 때, 한 팀의 트래픽 폭주로 전체 플랫폼 외부 LLM API 한도 Rate Limit가 초과되어 다른 팀 서비스까지 마비되는 현상을 어떻게 막고, 매월 발생하는 수천만원의 토큰 비용을 부서별로 어떻게 정확히 청구할 수 있을까?

이 문제를 해결하는 아키텍처가 **멀티 테넌시 기반의 자원 격리 및 관측성 파이프라인이다. 하나의 물리적 ai 플랫폼을 여러 태넌트가 완전히 독립된 시스템처럼 사용하도록 논리적으로 분리하고 통제하는 기술이다.**

- **Multi-tenancy**: 하나의 물리적 소프트웨어 인스턴스(플랫폼)를 여러 사용자 그룹 마치 자신들만의 전용 서버인 것처럼 독립적으로 사용하는 아키텍처, LLM 환경에서는 API 키, 분당 최대 토큰량, 사용자 컨텍스트를 논리적으로 분할하는 것을 의미한다.
- **Token Bucket Algorithm**: rate limiting을 구현하는 핵심 알고리즘으로 단순히 1분에 100번 호출 가능으로 막는것이 아니라 ,바구니에 토큰을 일정 속도로 채워두고 요청이 들어올 때마다 해당 텍스트 길이에 비례하는 토큰을 차감하는 방식이다 일시적인 트래픽 폭주는 허용하면서도 장기적인 과부하를 정밀하게 통제할 수 있다.
- **LLM Observability**: 시스템의 외부 출력값 로그만 보고 내부 상태를 유추하는 개념, 전통적인 APM과 달리 LLM 관측성은 비용 token usage, 프롬프트 체인의 실행순서 trace, 검색된 문맥의 질 context, 응답의 지연시간 time to first token이라는 4차원 데이터를 하나로 묶어서 추적하는것이 핵심이다.

<br>

## 문제 정의

단일 인증 정보로 외부 LLM을 공유하는 구조 탓에 특정 팀의 과도한 사용이 다른 팀의 서비스 마비로 직결되는 노이지 네이버 문제가 존재했고

블랙박스 형태의 기존 로깅 시스템으로는 부서별 프롬프트 효율성 분석 및 인프라 비용 성산이 불가능한 시스템적 한계가 있었다.

ex. A팀이 주말동안 테긋트 대량 요약 스크립트를 잘못 돌려 OpenAI의 분당 요청 제한 RPM/TPM을 모두 소진해버렸고, 이로 인해서 정상적으로 운영중이던 B팀의 고객센터 챗봇까지 연쇄적으로 다운되었다. 게다가 로그에는 단순한 HTTP 200/500 상태 코드만 남아있어 월말에 청구된 수백만원의 토큰 과금액을 어느 팀이 유발했는지 기술적으로 증명할 수 없는 문제다.

### Solving

- **API Gateway 레벨의 동적 제어 및 토큰 버킷 격리**: 플랫폼 앞단에 위치한 gateway가 들어오는 요청의 header를 검사하여 테넌트를 식별한다. 메모리 DB에서 각 테넌트별로 토큰 버킷을 생성하여 할당된 쿼터를 초과하는 즉시 해당 테넌트의 요청만 429 too many requests 상태 코드로 차단하고 나머지 테넌트는 보호한다.
- **OpenTelemetry 기반의 비동기 분산 추적(Distributed Tracing) 로깅**: LLM 호출이 완료되면 gateway는 응답 데이터에서 usage 객체(프롬프트 토큰, 완성 토큰)를 추출한다. 이를 단순히 텍스트 파일로 남기지 않고 누가 (teneant ID), 어떤 흐름으로 (trace ID), 얼마나 썼는지를 json 형태의 OpenTelemetry 표준 스펙으로 구조화해 Langfuse나 Elasticsearch 같은 관측성 특화 db에 비동기로 쏘아 보낸다.

<br>

## 상세 동작 원리 및 구조화

멀티 테넌트 환경에서 gateway가 트래픽을 제어하고, 비용 정산 및 분석을 위해 구조화된 json 로그를 생성하여 적재하는 논리적 흐름을 보면

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

이 데이터 스키마가 존재해야만 플랫폼 팀이 부서별 과금 대시보드와 프롬프트 최적화 리포트를 만들 수 있다.

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

토큰 버킷 기반의 테넌트 격리 로직을 보자

단순 횟수 RPM 제한이 아니라 텍스트 길이 TPM을 예측하여 테넌트의 자원을 제어하는 알고리즘 원리다.

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

FastAPI 기반의 Gateway에서 헤더로 테넌트를 식별하고 사용자 응답을 방해하지 않고 비동기 토큰 사용량을 로깅하는 코드도 보자

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