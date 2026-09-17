# Agent 구성요소와 State Model 설계

LLM 애플리켕이션을 개발하다보면 단순한 프롬프트 엔지니어링의 한계에 부딪히는 순간이 온다.

"검색을 먼저 하고 결과가 부족하면 다른 키워드로 다시 검색한 뒤 정보를 요약해서 답변해줘"

위와 같은 다단계 논리 구조를 안정적으로 실행하려면 새로운 패러다임이 필요하다.

매번 프롬프트를 칠 수 있는 노릇은 아니니까 혹여나 빠지기라도하면 공통지침이 무산되는 것이니.

## 단순 LLM Chain과 상태를 가진 Agent System은 무엇이 다른가?

전통적인 LangChain이나 파이프라인 구조 Chain와 현대적인 Agent System의 가장 큰 차이는 상태 보존 Statfulness과 순환 구조 Cyclic Execution에 다.

| 구분 | 단순 LLM Chain (Stateless Pipeline) | Stateful Agent System (LangGraph 기반) |
|------|------------------------------------|----------------------------------------|
| 실행 흐름 | 단방향(DAG). A → B → C로 끝남. | 순환형(Cyclic). A → B → C → A 반복 가능. |
| 의사 결정 | 개발자가 미리 정해둔 순서대로만 실행됨. | LLM이 현재 '상태'를 보고 다음 행동을 직접 결정(Routing). |
| 에러 복구 | 중간 단계(예: API 호출 실패)에서 에러 발생 시 전체 파이프라인 중단. | 실패 시 상태(Error Log)를 기록하고, LLM이 다른 도구를 선택하여 재시도(Retry). |
| 데이터 공유 | 이전 단계의 출력이 다음 단계의 입력으로만 전달됨. | 전역 상태(Global State) 객체를 통해 모든 노드가 과거의 생각, 행동, 결과물에 접근 가능. |
| 중단 및 재개 | 불가능. 한 번 실행하면 끝까지 가야 함. | Human-in-the-loop. 특정 단계에서 멈추고(Interrupt) 사용자 승인 후 재개 가능. |

단순 Chain이 일회성 함수라면 Agent System은 메모리와 제어 흐름을 가진 하나의 운영체제 프로세스처럼 동작한다.

<br>

## Agent 구성 요소 The Nodes

복잫반 문제를 해결하는 Agent는 하나의 거대한 프롬프트로 동작하지 않는다.

책임을 분리 (Separation of Concerns)하여 여러 개의 전문화된 Node로 나눈다.

1. **Planner (계획자):** 사용자의 복잡한 요청을 분석하여 작은 하위 작업으로 분할한다.
2. **Router (라우터):** 현재 상태를 평가하여 다음에 어떤 노드로 이동해야하는가를 결정하는 에지 Conditional Edge 역할을 한다. (ex. 도구 호출 -> Tool Executor, 검색 필요 -> Retriever, 완료 -> END)
3. **Tool Executor (도구 실행자):** LLM이 생성한 JSON 파라미터를 기반으로 실제 외부 api(web search, db query, code execution)를 호출한다.
4. **Retriever (검색자):** RAG 파이프라인과 연동되어 VectorDB or 외부 Knowledge base에서 문맥을 가져온다.
5. **Memory (메모리):** State 객체 내의 대화 기록 (shrot-term)과 요약된 핵심 정보 (long-term)를 영속적으로 보관한다.
6. **Validator (검증자/Reflector):** Tool Executor나 Retriever의 결과물이 사용자 질문을 해결하기에 충분한지, 혹은 Hallucination이 없는지 스스로 검증한다. 통과하지 못하면 피드백을 추가해 다시 Planner or Router로 돌려보낸다.

<br>

## Agent State Schema 설계 (Pydantic & TypedDict)

Agent System의 심장은 상태이다. State

LangGraph에서는 Python의 `TypedDict`를 사용해 그래프를 통과할때마다 노드들이 읽고 쓸 데이터 구조를 엄격하게 정의한다.

이때 LLM의 구조화된 출력 (Structured Output)을 검증하기 위해 `Pydantic`을 혼용한다.

```py
from typing import Annotated, List, Sequence, Optional
from typing_extensions import TypedDict
from pydantic import BaseModel, Field
import operator
from langchain_core.messages import BaseMessage

# 1. Pydantic을 이용한 LLM 출력 검증용 스키마 (Router 및 Validator 용도)
class RouteDecision(BaseModel):
    next_action: str = Field(description="다음에 실행할 액션: 'search', 'execute_tool', 'respond', 'retry'")
    reason: str = Field(description="해당 액션을 선택한 이유")
    tool_name: Optional[str] = Field(default=None, description="실행할 도구의 이름")
    tool_args: Optional[dict] = Field(default=None, description="도구 실행에 필요한 파라미터")

# 2. TypedDict를 이용한 LangGraph 전역 State 스키마
class AgentState(TypedDict):
    # Annotated와 operator.add를 사용하면 노드가 메시지를 반환할 때 기존 리스트에 '추가(Append)' 됩니다.
    messages: Annotated[Sequence[BaseMessage], operator.add]
    
    # 사용자의 원본 요청
    user_query: str
    
    # Planner가 쪼갠 하위 작업 리스트
    sub_tasks: List[str]
    
    # 현재 실행 중인 도구와 그 결과
    current_tool: Optional[str]
    tool_results: List[dict]
    
    # Validator가 판단한 에러 및 피드백 루프 카운트
    errors: List[str]
    iteration_count: int
```

### 상태 설계시 엔지니어링 고려사항

- **Reducer(`operator.add`)**: messages 필드는 값을 overwrite 하는 것이 아니라 누적 append 되어야 한다. 그래야 agent가 과거에 자신이 무슨 도구를 썼고 어떤 실패를 겪었는지 기억할 수 있다.
- **명시적인 제어 변수**: `iteration_count` or `errors` 같은 변수를 State에 두는 이유는 **무한 루프(Infinite Loop)를 방지하기 위함이다.** 루프가 5번을 넘어가면 강제로 프로세스를 종료하는 로직을 Router에 구현해야한다.

<br>

## 실행 그래프 구축

정의된 State를 바탕으로 노드들을 연결하여 방향성 그래프를 만든다.

```py
from langgraph.graph import StateGraph, END

# 1. 그래프 초기화 (정의한 AgentState를 주입)
workflow = StateGraph(AgentState)

# 2. 노드 추가 (각 함수는 AgentState를 입력받아 수정된 부분집합을 반환)
workflow.add_node("planner", plan_tasks_node)
workflow.add_node("router", routing_node)
workflow.add_node("tool_executor", execute_tool_node)
workflow.add_node("validator", validate_result_node)

# 3. 엣지 연결 (실행 흐름 정의)
workflow.set_entry_point("planner")
workflow.add_edge("planner", "router")

# 4. 조건부 엣지 (Router의 결정에 따라 동적으로 분기)
def route_condition(state: AgentState) -> str:
    # Router 노드가 결정한 next_action 값을 읽어 분기
    last_message = state["messages"][-1].content
    if "execute_tool" in last_message:
        return "tool_executor"
    elif "respond" in last_message:
        return "validator"
    return END

workflow.add_conditional_edges(
    "router",
    route_condition,
    {
        "tool_executor": "tool_executor",
        "validator": "validator",
        END: END
    }
)

# Tool 실행 후에는 결과를 들고 다시 Router(또는 LLM)로 돌아가서 다음 행동을 결정 (순환 구조)
workflow.add_edge("tool_executor", "router")

# Validator에서 통과하면 END, 실패하면 다시 Router로 피드백 전달
def validation_condition(state: AgentState) -> str:
    if state["errors"]: # 에러가 있다면
        return "router"
    return END

workflow.add_conditional_edges(
    "validator",
    validation_condition,
    {
        "router": "router",
        END: END
    }
)

# 5. 그래프 컴파일 (Durable Runtime 완성)
app = workflow.compile()
```

<br>

## Agent State Schema와 실행 그래프 동작 예시

위 코드를 시각적으로 표현하면 agent는 단방향 파이프라인이 아닌 **조건에 따라 유동적으로 툴을 사용하고 스스로 반성ㅇ하는 생태계로 동작한다.**

### Agent 실행 흐름 및 상태변화 ex

1. **[Planner]**: `sub_tasks` state 업데이트 -> [`1. 애플 주가 검색`, `2. 결과 한국어 3줄 요약`]
2. **[Router]**: LLM이 판단하여 `Search API` 도구 호출 결정 및 `messages`에 state 누적.
3. **[Tool Executor]**: 외부 api 호출, `tool_results` State 업데이트 -? `[{"tool": "search", "result": "Apple closed at $195.00..."}]`
4. **[Router]**: 검색 결과를 보고 정보가 충분함을 인지. 요약 플모프트 작성후 `validator`로 라우팅
5. **[Validator]**: 요약된 응답이 한국어 3줄 조건을 만족하는지 검사.
   1. 실패시 errors state 업데이트 -> "2줄로만 요약되어 3줄로 다시 작성하세요" 루프백 실행
   2. 성공시 최종 메시지 반환 후 END 도달

에이전트 아키텍처에서 가장 중요한 것은 **각 단계 LLM이 서로 상태 객체 JSON을 공유하여 협력하는 구조를 체계적으로 잡아주는 것이다.**

