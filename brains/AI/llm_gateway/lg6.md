# LLM API 응답 가공

멀티 테넌트 B2B AI 서비스에서 여러 고객사나 사내 부서들이 하나의 시스템을 통해 무수히 많은 LLM API를 호출할 때 매월 클라우드 벤더 (OpenAI, vLLM Infra) 등으로부터 청구되는 수천만원의 요금을 어떻게 각 테넌트의 실제 사용량과 비례하여 1원 단위로 오차없이 재청구 할 수 있을까? 또한, 이 과금 데이터가 중앙 정산 시스템 (Billing System)으로 넘어갈때 누락이나 포맷 에러를 방지하려면 어떤 표준화 작업이 필요할까?

LLM 서비스의 수익성과 인프라 비용 통제를 책임지는 가장 중요한 백엔드 파이프라인이 바로 **과금 정산 Chargeback 파이프라인이다**. 이를 구현하기 위해서는 비정형화된 LLM 응답 데이터에서 과금의 기준이 되는 핵심 지표를 추출하고 이를 시스템이 신뢰할 수 있는 정형 데이터로 변환하는 계층이 필수적이다.

- **Chargeback (내부 과금/재청구 모델)**: 중앙 IT 부서나 플랫폼 인프라가 부담한 총 비용을 시스템을 실제로 소비한 각 테넌트 (부서, 고객사, 프로젝트)의 리소스 점유율에 따라 투명하게 분배하고 정산하는 재무적으로 엔지니어링하는 프로세스다. LLM 환경에서 토큰이 과금의 절대적인 단위가 된다.
- **메타데이터 추출(Metadata Extraction)**: 텍스트 생성을 위해 호출된 LLM API의 응답 페이로드에는 클라이언트에게 보여줄 텍스트 본문(content) 이외에도 소요된 `prompt_tokens`, `completion_tokens`, `total_tokens` (각각 입력, 출력, 전체)등의 메타데이터가 포함되고 이를 본문가 분리하여 캡처하는 기술이 메타데이터 추출이다.
- **스키마 정규화**: 추출된 파편화된 토큰 정보와 서버 컨텍스트에 존재하는 테넌트 ID, 모델 ID, 호출 타임스탬프 등을 조합하여 다운스트림 (데이터 웨어하우스 혹은 카프카 등등)에서 파싱 에러 없이 100퍼센트 처리할 수 있도록 엄격하게 타입이 정의된 json 문서 구조로 가공하는 과정이다.

<br>

## 문제 정의

단순히 응답값에서 토큰 숫자를 빼내어 db에 저장하려 할 때, 실무 아키텍처에서는 다음과 같은 데이터 정합성 결함과 예외상황이 발생한다.

- **스트리밍 (Server-Sent Events) 응답의 메타데이터 은닉**: 클라이언트의 체감 지연 시간을 줄이기위해 LLM 응답 스트리밍 모드로 설정하면, 청크 단위로 데이터가 조각나서 전달된다. 대부분의 LLM 공급자는 토큰 사용량을 데이터 중간 청크에 포함하지 않고 오직 마지막 청크의 특정 필드에만 실어 보낸다. 이를 중간에 낚아채지 못하면 과금 데이터가 영구 유실된다.
- **LLM 공급자간 응답 페이로드 파편화**: OpenAI, Anthropic, 사내 vLLM 서버등 각 모델 제공자마다 토큰 사용량을 표기하는 JSON Key (ex. `usage`, `token_usage`, `amazon-bedrock-invocationMetrics`)와 계층 구조가 완전히 다르다. 이를 일관된 스키마로 매핑하지 않으면 중앙 전산 시스템에 장애가 발생한다.
- **인증 컨텍스트와 응답 데이터의 분리**: 토큰 정보는 LLM의 응답 객체에 들어왔지만 누가 tenant id 호출 했는가에 대한 정보는 api로 들어온 HTTP Request 헤더나 미들웨어 `request.state`에 존재한다. 비동기 환경에서 이 두개의 서로 다른 출처 데이터를 정확히 하나의 트랜잭션으로 결합해야하는 상태 관리에 어려움이 존재한다.

### 해결 방식

- **Adapter 패턴을 통한 파싱 추상화:** LLM 공급자별로 다르게 응답되는 원시 페이로드를 입력받아. 공급자 타입에 맞는 파서를 동적으로 호출하여 표준화된 토큰 딕셔너리로 반환하는 어뎁터 계층을 구현한다.
- **Pydanic 기반의 강타입 과금 이벤트 스키마 강제**: 파이썬의 단순 딕셔너리 대신 pydantic 모델을 사용해 `ChargebackEvent` 스키마를 정의하고 누락된 필드가 있거나 타입이 일치하지 않으면 인스턴스화 단게에서 에러를 발생시켜, 오염된 데이터가 과금 db로 유입되는 것을 원천 차단한다.
- **비동기 제네레이터 (async generator)를 활용한 최종 청크 후킹:** 스트리밍 응답을 클라이언트에게 yield로 밀어내는 과정(proxying)에서, 응답의 끝단 DONE 또는 마지막 객체를 감지하는 순간 메타데이터를 추출하고 Request Context의 테넌트 ID와 결합하여 비동기로 과금 이벤트를 발행한다.

<br>

## 상세 동작 원리 및 구조화

다이어그램 없이 애플리케이션 메모리와 네트워크 IO레벨에서 발생하는 데이터 매핑 및 가공 흐름을 단계별로 분석한다.

1. **컨텍스트 격리 및 식별:** 클라이언트의 api 호출이 프레임워크 FastAPI에 인입되면 인증 미들웨어가 HTTP 헤더(`Authorization` or `X-API-Key`)를 파싱하여 해당 요청의 주체인 `tenant_id`를 추출한다 이 값은 현재 요청 스레드 안정성을 보장하는 `request.state.tenant_id` 공간에 메모리 적재된다
2. **LLM 추론 및 원시 메타데이터 수신**: 컨트롤러가 외부 LLM API에 추론을 요청하고 응답을 수신한다. 이때 응답 본문 최 한단에는 `{"usage": {"prompt_tokens": 150, "completion_tokens": 50, "total_tokens": 200}}` 형태의 원시 JSON 트리거가 포함되어 있다.
3. **Usage 데이터 추출 및 어뎁터 통과**: 비즈시느 로직은 클라이언트에게 보낼 실제 텍스트 메시지 부분과 `usage` 메타데이터 노드를 분리한다. 분리된 `usage` 노드는 사전에 정의된 파서 함수로 전달되어 시스템 내부 표준 변수 (ex. input_tokens, output_tokens) 로 매핑된다.
4. **스키마 인스턴스화 (Schema Instantation)**: 1번 단계에서 보관중인 `tenant_id`와 3번 단계에서 확보된 `input_tokens`, `output_tokens` 그리고 환경 변수에서 가져운 `model_id` (적용될 단가 계산의 기준)를 하나의 pydantic 클래스 생성자에 주입한다. 이때 생성 타임스탬프와 고유 트랜잭션 ID가 자동으로 발급되어 객체에 각인된다.
5. **JSON 직렬화 및 메시지 발행**: 유효성 검사를 완벽히 통과한 Pydantic 인스턴스는 `.model_dump_json()` 메서드를 통해 바이트/문자열 형태의 JSON으로 직렬화된다. 이 문자열은 과금 시스템이 구독하고 있는 kafka topic이나 redis stream or BackgroundTasks를 통한 비동기 db insert를 통해 영속화 계층으로 안전하게 전송된다.

### Example

다양한 컨텍스트 변수를 모아서 하나의 정형화된 과금 딕셔너리로 조립하고 json 문자열로 변환하는 기초적인 원리다.

```py
import json
from datetime import datetime
import uuid

# 1. 미들웨어 등에서 추출되어 보관 중인 Request Context
current_tenant_id = "tenant-marketing-01"
current_model = "gpt-4-turbo"

# 2. 외부 LLM API(OpenAI 등)로부터 받은 가상의 원시 응답 JSON
llm_raw_response = {
    "id": "chatcmpl-123",
    "choices": [{"message": {"content": "Hello! I am an AI."}}],
    "usage": {
        "prompt_tokens": 10,
        "completion_tokens": 7,
        "total_tokens": 17
    }
}

# 3. 토큰 추출 및 테넌트 맵핑을 통한 Chargeback 스키마 조립
def generate_chargeback_event(tenant_id: str, model_id: str, raw_response: dict) -> str:
    # 안전한 추출 (KeyError 방지)
    usage = raw_response.get("usage", {})
    
    chargeback_data = {
        "event_id": str(uuid.uuid4()),
        "timestamp": datetime.utcnow().isoformat() + "Z",
        "tenant_id": tenant_id,
        "model_id": model_id,
        "metrics": {
            "input_tokens": usage.get("prompt_tokens", 0),
            "output_tokens": usage.get("completion_tokens", 0),
            "total_tokens": usage.get("total_tokens", 0)
        }
    }
    
    # 4. 다운스트림 시스템을 위한 JSON 포맷으로 직렬화
    return json.dumps(chargeback_data)

# 실행 결과
chargeback_json = generate_chargeback_event(current_tenant_id, current_model, llm_raw_response)
print(chargeback_json)
```

위에는 추출이고 FastAPI 환경을 가정해 Pydantic을 이요한 엄격한 스키마 검증과 의존성 주입을 통해 Request State에서 테넌트 정보를 가져오고 이벤트 객체를 생성하는 구조를 보면

```py
import uuid
from datetime import datetime, timezone
from pydantic import BaseModel, Field
from fastapi import FastAPI, Request, BackgroundTasks

app = FastAPI()

# 1. 중앙 과금 시스템(Billing DB/Kafka)과 약속된 엄격한 Pydantic 스키마 정의
class TokenMetrics(BaseModel):
    input_tokens: int = Field(..., ge=0, description="프롬프트 토큰 수")
    output_tokens: int = Field(..., ge=0, description="생성된 토큰 수")
    total_tokens: int = Field(..., ge=0, description="전체 토큰 수")

class ChargebackEvent(BaseModel):
    event_id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    timestamp: str = Field(default_factory=lambda: datetime.now(timezone.utc).isoformat())
    tenant_id: str = Field(..., description="과금 대상 부서 또는 고객사 ID")
    model_id: str = Field(..., description="사용된 LLM 모델명 (단가 맵핑용)")
    metrics: TokenMetrics

# 2. 비동기 환경에서 과금 이벤트를 외부 시스템(예: Kafka, DB)으로 전송하는 가상의 워커
async def publish_chargeback_event(event_json: str):
    # 실제 환경에서는 aiokafka, boto3(Kinesis) 등을 사용하여 전송
    print(f"[Billing Pipeline] 전송 완료: {event_json}")

# 3. 메인 엔드포인트
@app.post("/v1/completions")
async def generate_text(request: Request, bg_tasks: BackgroundTasks):
    # A. 미들웨어가 request.state에 주입해둔 테넌트 식별자 확보
    # (실무에서는 Header나 JWT 파싱 미들웨어를 거친 상태)
    tenant_id = getattr(request.state, "tenant_id", "default_untracked_tenant")
    target_model = "claude-3-opus-20240229"
    
    # B. LLM 호출 및 응답 수신 모사 (실제는 Anthropic/OpenAI 비동기 클라이언트 호출)
    mock_llm_response = {
        "content": "실무 레벨의 과금 파이프라인 데이터입니다.",
        "usage": {"input_tokens": 105, "output_tokens": 45} # Anthropic 스타일
    }
    
    # C. 공급자 독립적인 파싱 및 스키마 인스턴스화
    try:
        # Pydantic 모델을 통해 누락된 필드가 없는지, 타입이 맞는지 런타임 검증
        chargeback_event = ChargebackEvent(
            tenant_id=tenant_id,
            model_id=target_model,
            metrics=TokenMetrics(
                input_tokens=mock_llm_response["usage"].get("input_tokens", 0),
                output_tokens=mock_llm_response["usage"].get("output_tokens", 0),
                total_tokens=(
                    mock_llm_response["usage"].get("input_tokens", 0) + 
                    mock_llm_response["usage"].get("output_tokens", 0)
                )
            )
        )
        
        # Pydantic 객체를 최종 JSON 문자열로 직렬화
        validated_json_payload = chargeback_event.model_dump_json()
        
        # D. 사용자 응답 속도를 저하시키지 않도록 BackgroundTasks에 넘겨 비동기 발행
        bg_tasks.add_task(publish_chargeback_event, validated_json_payload)
        
    except Exception as e:
        # 과금 객체 생성 실패 시(데이터 누락 등) 알람 발생 로직 (Sentry 등)
        print(f"[Error] Chargeback 생성 실패: {str(e)}")

    # E. 클라이언트에게는 텍스트만 깔끔하게 반환
    return {"message": mock_llm_response["content"]}
```