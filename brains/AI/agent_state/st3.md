# Checkpointing 적용

애플리케이션 서버가 재시작되거나 분산 환경에서 여러 서버 인스턴스가 동작할 때, 특정 사용자의 에이전트 상태를 어떻게 서버간 공유하고 영구적으로 보존할 수 있을까.

특히 바이너리 데이터가 아닌 가독성있는 JSON 형태로 상태를 관리하려면? 어떤 계층적 설계가 필요할까

LangGraph의 영속성 아키텍처는 Checkpointer라는 추상화 계층을 통해 구현된다 대표적으로 RedisSaver는 인메모리 데이터 구조 저장소 redis를 백엔드로 에이전트의 상태를 실시간으로 저장하고 복구하는 고성능 체크포인팅 컴포넌트다.

- **RedisSaver**: LangGraph 그래프의 각 노드가 실행을 마칠 때마다 현재의 State 스키마 데이터를 redis의 특정 key에 저장하는 객체다. 분산 환경에서 세션 공유를 가능하게 하는 외부 영속성 계층이다.
- **JSON 직렬화**: 파이썬 객체 형태의 상태 데이터를 네트워크 전송 및 저장이 가능한 JSON 형태로 변환하는 과정이다. 기본적으로 많은 체크포인터는 파이썬 전용인 Pickle 방식을 사용하지만, 타 언어 호환성이나 가독성을 위해 JSON 형식을 채택하는 경우가 많다.
- **Checkpoint**: 특정 시점(노드 실행 직후)의 에이전트 상태 스냅샷이다. 여기에는 메시지 내역, 전역 변수값, 다음 실행할 노드의 위치 정보가 포함된다.
- **Thread Configuration**: thread_id를 포함하는 설정 객체 redis 내부에서 데이터를 구분하는 인덱스 역할을 수행하며 이를 통해 무상태 서버가 특정 사용자의 이전 문맥을 정확히 찾아낼 수 있다.

<br>

## 문제 정의

기본적인 MemorySaver를 사용하거나 적절한 직렬화 전략 없이 외부 저장소를 연동할 경우 다음과 같은 기술적 부채와 운영상의 제약이 발생한다.

- **상태 데이터의 휘발성 및 확장성 결여**: `MemorySaver`는 서버의 RAM에 데이터를 저장하므로 프로세스 종료시 모든 데화 내역이 유실된다. 다중 노드 서버 환경에서 세션 ㅅ티키니즈가 보장되지 않으면 사용자의 요청이 다른 서버로 갈때 문맥이 끊기는 현상이 발생한다.
- **Pickle 직렬화의 보안 및 호환성 문제**: 파이썬의 기본 pickle 방식은 역직렬화시 임의 코드가 실행될 수 있는 보안 취약점이 있으며, redis에 저장된 데이터를 관리자가 직접 조회하거나 타 언어로 작성된 대시보드에서 파싱하기가 어렵다.
- **원자적 쓰기 부재에 따른 상태 손실**: 에이전트가 복잡한 병렬연산을 수행할 때 여러 노드의 결과가 동시에 redis에 기록되려하여 데이터 경합이 발생해 최종 상태 객체가 깨질 위험이 있다. 따라서 체크포인터는 트랜잭션 수준의 원자성을 보장해야한다.

### Solution

- **Redis 기반 외부 영속성 주입**: `langgraph-checkpoint-redis` 라이브러리를 활용해 `RedisSaver` 객체를 생성하고 이를 통해 애플리케이션 메모리가 아닌 외부 redis 인스턴스에 상태를 물리적으로 격리 저장하여 서버 가용성과 관계없는 영속성을 확보한다.
- **커스텀 json serializer 적용:** pydantic 모델이나 `langchain_core`의 `load/dump` 유틸리티를 활용하여 복잡한 LangCahin 메시지 객체를 정형화된 json 구조로 변환하는 직렬화 로직을 주입한다. 이를 통해 redis 내부 데이터를 텍스트 형태로 유지하며 높은 가독성과 상호 운용성을 확보한다.
- **컴파일 타임 체크포인터 바인딩**: 그래프를 빌드하는 시점에 `compile(checkpointer=redis_saver)` 형식으로 주입한다. 이렇게 하면 개발자가 노드 내부에서 수동으로 save 명령을 내릴필요 없이 프레임워크가 노드 실행 완료 이벤트를 가로채어 자동으로 redis에 체크포인트를 기록한다.

<br>

## 상세 동작 원리 및 구조화

```mermaid
sequenceDiagram
    participant Client as Client
    participant Router as FastAPI
    participant Graph as LangGraph Engine
    participant Node as Agent Node
    participant Saver as RedisSaver
    participant Redis as Redis Server

    Client->>Router: 1. POST /invoke (thread_id: 123)
    Router->>Graph: 2. graph.invoke(config={"thread_id": "123"})
    
    Note over Graph,Saver: [상태 복구 페이즈]
    Graph->>Saver: 3. 이전 체크포인트 조회 요청
    Saver->>Redis: 4. GET checkpoint:123
    Redis-->>Saver: 5. 직렬화된 JSON 반환
    Saver->>Graph: 6. JSON 역직렬화 및 State 객체 복원
    
    Note over Graph,Node: [노드 실행 페이즈]
    Graph->>Node: 7. 복원된 State 주입 및 노드 실행
    Node-->>Graph: 8. State 업데이트 딕셔너리 반환
    Graph->>Graph: 9. 내부 리듀서를 통해 최신 State 병합
    
    Note over Graph,Redis: [영속화(Checkpointing) 페이즈]
    Graph->>Saver: 10. 노드 실행 완료 및 저장 트리거
    Saver->>
```

redis 내부에서 데이터가 어떻게 구조화되어 저장되고, 노드 실행시마다 어떤 생명주기를 거치는지 물리적 메모리 및 네트워크 레벨에서 분석한다.

1. **연결 풀(Connection Pool) 초기화**: 애플리케이션 시작시 redis py로 redis 연결풀 생성후 RedisSaver는 이 연결풀을 점유해 대기
2. **그래프 실행 및 thread_id 식별**: 클라이언트 요청이 들어오면 `config={"configurable": {"thread_id": "..."}}`로 특정 세션 지정
3. **노드 실행 완료 및 상태 가로채기**: 특정 노드 agent_node의 연산이 종료되면 LangGraph 런타임은 반환된 State 업데이트값을 확인함
4. **객체 직렬화 파이프라인**: State 내부의 BaseMessage 객체들이나 커스텀 변수들이 직렬화기로 전달되고 여기서 복잡한 파이썬 클래스들은 json 문자열로 변환된다.
5. **Redis HSET/SET**: thread_id를 키의 일부로 사용하여 redis 데이터에 기록후 일반적인 `checkpoint:<threadid>`와 같은 네이밍 컨벤션으로 데이터는 바이너리 세이프한 문자열 형태로 저장시킨다.
6. **버전 관리 및 체크포인트트리**: LangGraph는 단순히 덮어쓰기 뿐만 아니라 실행 경로의 분기를 지원하기 위해 타임스탬프나 체크포인트 ID를 서브키로 관리할 수 있고 이를 통해 과거 특정 시점으로 상태를 되돌리는 time travel이 가능하다.
7. **응답 후 자원 해제**: 기록 완료후 redis connection을 풀로 반환하고 최종 응답이 전송된다.

### Example

redis를 직접 핸들링해 json으로 상태를 저장후 복구하는 로직의 핵심 흐름을 보여주는 예제다.

```py
import json
import redis
from typing import Dict, Any

# 1. Redis 연결 설정
r = redis.Redis(host='localhost', port=6379, db=0, decode_responses=True)

# 2. 가상의 에이전트 상태 (LangGraph State 모사)
current_state = {
    "messages": [
        {"role": "user", "content": "오늘 날씨 어때?"},
        {"role": "assistant", "content": "서울은 맑음입니다."}
    ],
    "next_step": "weather_api_call"
}

# 3. JSON 직렬화 및 저장 로직
def save_checkpoint(thread_id: str, state: Dict[str, Any]):
    checkpoint_key = f"checkpoint:{thread_id}"
    # 객체를 가독성 있는 JSON 문자열로 변환
    serialized_data = json.dumps(state, ensure_ascii=False)
    # Redis에 원자적으로 저장
    r.set(checkpoint_key, serialized_data)
    print(f"[Save] Thread {thread_id} 상태 저장 완료")

# 4. 데이터 로드 및 역직렬화
def load_checkpoint(thread_id: str) -> Dict[str, Any]:
    checkpoint_key = f"checkpoint:{thread_id}"
    raw_data = r.get(checkpoint_key)
    if raw_data:
        return json.loads(raw_data)
    return {}

# 실행 테스트
tid = "user_1234"
save_checkpoint(tid, current_state)
restored_state = load_checkpoint(tid)
print(f"복구된 데이터: {restored_state['messages'][-1]['content']}")
```

`langgraph-checkpoint-redis` 라이브러리를 사용해 실제 그래프에 RedisSaver를 주입하고 비동기 환경에서 작동하는 프로덕션 레벨 코드까지 보면

```py
import os
from typing import Annotated, TypedDict
from langchain_openai import ChatOpenAI
from langgraph.graph import StateGraph, START, END
from langgraph.checkpoint.redis import RedisSaver
from redis.asyncio import ConnectionPool

# 1. 상태 스키마 정의
class AgentState(TypedDict):
    input: str
    history: Annotated[list, lambda x, y: x + y]

# 2. 노드 정의
def call_model(state: AgentState):
    # 실제 환경에서는 LLM 호출 로직이 들어감
    return {"history": [f"AI response to: {state['input']}"]}

# 3. Redis 연동 및 체크포인터 초기화
# 프로덕션에서는 환경 변수를 통해 Redis URL을 관리합니다.
REDIS_URL = os.getenv("REDIS_URL", "redis://localhost:6379")
pool = ConnectionPool.from_url(REDIS_URL)

# RedisSaver는 내부적으로 직렬화를 처리하며, 
# 필요 시 커스텀 직렬화기를 생성자에 주입할 수 있습니다.
checkpointer = RedisSaver(pool)

# 4. 그래프 구축 및 체크포인터 주입
builder = StateGraph(AgentState)
builder.add_node("agent", call_model)
builder.add_edge(START, "agent")
builder.add_edge("agent", END)

# compile 시점에 영속성 계층을 프레임워크에 주입
# 이제 모든 노드 실행 결과는 자동으로 Redis에 기록됩니다.
graph = builder.compile(checkpointer=checkpointer)

# 5. 비동기 실행 엔드포인트 모사
async def run_session(thread_id: str, user_input: str):
    config = {"configurable": {"thread_id": thread_id}}
    
    # 실행 시마다 Redis에서 이전 상태를 자동으로 로드함
    async for event in graph.astream(
        {"input": user_input, "history": []}, 
        config, 
        stream_mode="values"
    ):
        print(f"Current State in Thread {thread_id}: {event}")

# 사용 예시: 동일한 thread_id로 두 번 호출하면 데이터가 Redis에 누적됨
# await run_session("session_001", "첫 번째 질문")
# await run_session("session_001", "두 번째 질문 (이전 문맥 유지)")
```