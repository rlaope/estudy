# Tool Calling과 MCP 기반 Tool 설계

LLM에 도구(Tool)을 쥐여주는 것은 매우 강력하지만, 그만큼 위험하다.

텍스트를 예측하는 확률 모델 LLM과 엄격한 타입과 규칙을 요구하는 소프트웨어 API 사이에서 Impedance Mismatch를 해결하는것. 이것이 이번장의 핵심 주제라고 볼 수 있다.

## LLM이 Tool을 잘못 선택하거나 인자를 잘못 생성했을때 어떻게 방어할 것인가?

LLM은 본질적으로 비결정적이라 아무리 프롬프트를 잘 작성해도 다음과 같은 할루시네이션이 발생할 수 있다.

1. **존재하지 않는 도구 호출:** `search_web` 도구만 주었는데 자기 마음대로 `delete_database`라는 도구를 호출하려고 시도함
2. **타입 불일치**: `limit` 인자로 정수 10을 요구했는데 문자열 `ten` or `10개` 를 보냄.
3. **권한 및 범위 초과:** 1 ~ 100건만 조회해야하는 API에 `limit: 100000`을 요청하여 DB 부하를 유발함.

이러한 문제를 방어하기 위해 엔지니어는 시스템을 신뢰하지 않는 구조 (Zero Trust)

이 방향으로 설계해 나아가야한다. 애플리케이션 레벨에서 1차적으로 강력한 Schema Validation을 수행하고

실패시 Exception을 발생시켜 시스템을 죽이는 대신, **에러 메시지를 텍스트로 포장하여 LLM에게 다시 돌려주어 Self correction 하도록 유도한다.**

<br>

## MCP

과거에는 OpenAI, Anthropic, OSS 모델마다 Tool을 정의하고 호출하는 JSON 형식이 모두 달랐다.

이를 해결하기 위해 등장한 것이 MCP(Model Context Protocol) 이다.

MCP는 AI 모델(클라이언트)과 데이터 소스/도구(서버)가 통신하는 방식을 표준화한 오픈소스 프로토콜이고 마치 컴퓨터가 주변 기기를 연결하는 AI를 위한 USB-C 포트와 같다.

**아키텍처 Client-Server** 모델이며

- **MCP Host(Client)**; LLM 에이전트 (Claude Desktop, LangChain, Cursor 등)
- **MCP Server**: 로컬 또는 원격에 띄워진 Tool Provider. DB 연동, 파일 시스템 접근, 사내 API통신등을 담당한다.
- **Transport:** 표준 입출력 stdio 또는 HTTP 위에서 동작하며 마이크로서비스간 통신에 쓰이는 gRPC와 유사한 역할을 LLM 생태계에서 수행합니다.

**장점:** 한 번 MCP Server로 도구를 만들어두면 어떤 LLM이나 에이전트 프레임워크를 사용하든 코드를 수정할 필요 없이 즉시 연결해서 사용이 가능하다.

<br>

## 방어적 Tool 설계의 4대 원칙

안전한 Tool Server를 구축하기 위해 다음 네 가지 요소를 스키마 레벨에서 강재해야한다.

1. **명시적인 입력/출력 Schema (Pydantic):** 타입, 기본값, 열거영을 엄격히 정의한다.
2. **친절한 description (LLM을 위한 주석):** 변수명만 쓰는것이 아니라 description 필드에 LLM이 이해하기 쉬운 가이드라인을 적어주어야한다. (ex. "날짜는 반드시 YYYY-MM-DD 포맷 이어야함.")
3. **Graceful Error Return (에러 캡슐화):** 유효성 검사 실패시 `HTTP 500 Error`를 던지는 대신, `Error: limit은 100을 넘을 수 없습니다. 값을 줄여서 다시 시도해주세요` 라는 문자를 반환하여 Agent의 State에 반영되도록 한다.
4. **Timeout 및 격리 Sandboxing:** 외부 API 호출이 무한히 지연되는 것을 막기 위하여 모든 tool 실행에 엄격한 타임아웃을 세팅해둔다.

<br>

## MCP Tool Server 구현 및 배포

가상의 사내 데이터베이스를 안전하게 조회하는 Tool을 최신 FastMCP(MCP Python SDK)와 Pydantic을 활용해 구현한 예제다.

```py
from mcp.server.fastmcp import FastMCP
from pydantic import BaseModel, Field, field_validator
import asyncio

# 1. MCP Server 인스턴스 생성 (이름 부여)
mcp = FastMCP("EnterpriseDataServer")

# 2. 강력한 방어선: Pydantic Schema 정의
class QueryUserArgs(BaseModel):
    # Enum을 통한 허용값 제한
    department: str = Field(
        ..., 
        description="조회할 부서명. 허용되는 값: 'engineering', 'sales', 'hr'"
    )
    # Range Bound (최소/최대 제한)
    limit: int = Field(
        default=10, 
        ge=1, 
        le=100, 
        description="가져올 최대 사용자 수. 1에서 100 사이의 정수만 허용됨."
    )
    # 추가 커스텀 검증 로직
    @field_validator('department')
    def check_department(cls, v):
        allowed = ['engineering', 'sales', 'hr']
        if v.lower() not in allowed:
            raise ValueError(f"'{v}'는 알 수 없는 부서입니다. {allowed} 중에서 선택하세요.")
        return v.lower()

# 3. Tool 등록 및 Timeout/Error Handling 설정
@mcp.tool(description="사내 부서별 사용자 목록을 데이터베이스에서 조회합니다.")
async def query_users(args: QueryUserArgs) -> str:
    try:
        # Timeout 강제 적용 (예: 5초)
        async with asyncio.timeout(5.0):
            # 실제 DB 쿼리가 들어갈 자리 (가상 로직)
            await asyncio.sleep(1) # 네트워크 지연 시뮬레이션
            
            return f"Success: '{args.department}' 부서에서 {args.limit}명의 사용자 정보를 조회했습니다."
            
    except asyncio.TimeoutError:
        # LLM에게 재시도를 유도하는 에러 반환
        return "Error: 데이터베이스 응답 시간이 초과되었습니다. 쿼리 조건을 단순화하여 다시 시도하세요."
    except Exception as e:
        # 시스템 에러를 숨기지 않고 LLM의 컨텍스트로 전달
        return f"Error: 도구 실행 중 문제가 발생했습니다. {str(e)}"

# 4. 서버 실행 (stdio 모드로 실행되어 LLM 에이전트 프로세스와 파이프 통신)
if __name__ == "__main__":
    mcp.run()
```

### 예상 결과 및 LLM의 자가 수정 루프

만약 LLM이 사용자의 "엔지니어링 팀우너 준부 다 찾아줘"라는 모호한 요청을 받고 다음과 같은 인자를 생성했다고 가정해보자

#### 첫번째 시도 - 실패

- **LLM 생성 JSON:** `{"department": "engineering", "limit": 9999}`
- **MCP Server 처리 결과:** Pydantic Validator가 즉시 개입하여 요청 차단
- **LLM에게 반환되는 텍스트:** `"ValidationError: limit 값이 100을 초과할 수 없습니다. (현재 값: 9999). 값을 수정하여 다시 도구를 호출하세요."`

### 두번째 시도 - 성공

- **상태 변이:** 반환된 에러 메시지가 Agent State(`messages`)에 추가되고 LLM은 이를 읽고 자신의 실수를 인지한다.
- **LLM 생성 JSON:** `{"department": "engineering", "limit": 100}`
- **MCP Server 처리 결과:** 검증 통과
- **최종 반환:** `"Success: 'engineering' 부서에서 100명의 사용자 정보를 조회했습니다."`

<br>

## 산출물: Schema Validation이 적용된 MCP Tool Server 가이드

실제 운영 환경에 MCP Tool Server를 배포할 때 점검해야할 보안 아키텍처 체크리스트 산출물이다.

| 검증 단계 | 적용 기술 / 스택 | 아키텍처 설계 지침 |
|----------|------------------|--------------------|
| 입력 인자 검증 | Pydantic Field, Validator | LLM은 종종 타입을 무시합니다. 숫자 자리에 문자열이 들어와도 파싱할 수 있도록 Pydantic의 강력한 Type Coercion을 활용하고, ge, le 등 경계값을 필수로 설정하세요. |
| 권한 인가 (AuthZ) | JWT, Context Injection | Tool 호출 시 LLM 자체의 권한이 아닌, **'현재 LLM을 사용 중인 End-User의 권한(Token)'**을 MCP Server로 패스스루(Pass-through)하여 접근 제어 모델을 구축해야 데이터 유출을 막을 수 있습니다. |
| 예외 처리 전략 | Try-Except Wrapping | Tool 내부의 HTTP 404, 500 예외가 시스템 전체 프로세스를 중단시키지 않도록 하세요. 모든 예외는 Catch하여 LLM이 읽을 수 있는 평문(Plain text) 에러 가이드로 변환하여 리턴해야 합니다. |
| 통신 프로토콜 | MCP stdio / SSE | 로컬 에이전트(개발 환경)에서는 프로세스 파이프인 stdio를 사용하고, 클라우드 분산 환경에서는 API 게이트웨이 뒤에 SSE(Server-Sent Events) 기반 MCP 서버를 구축하여 gRPC처럼 빠르고 스트리밍 가능한 통신망을 구성합니다. |