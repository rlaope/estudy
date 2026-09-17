# LLM 에이전트 아키텍처

### ReAct (Reasoning and Acting) 패턴 및 Tool Use (Function Calling)

ReAct는 LLM이 단순한 텍스트 생성기를 넘어 시스템을 제어하는 **컨트롤러 즉 두뇌 역할을 하게 만드는 핵심 추론 프레임워크다**

#### 동작원리 (Thought -> Action -> Observation)

에이전트는 사용자의 요청을 받으면 전저 생각 thought를 한다.

무엇을 해야할지 계획을 세우고, 외부 도구가 필요한지 판단한다.

그 이후 외부 시스템 api를 호출하는 action 행동을 한다. 

api의 결과값 상태 코드 json 응답등을 관찰 observation 한 뒤에 다시 생각하여 최종 답변을 도출하거나 추가 행동을 취한다. 이 루프를 목표 달성시까지 반복한다.

#### Tool Use (Function Calling)

과거에는 LLM의 텍스트 출력을 정규식 regax로 파싱해서 함수를 실행해야 했다.

지금의 function calling은 LLM이 우리가 정의한 json schema(함수 이름, 파라미터 타입, 필수 여부)를 정확히 이해하고, **결정론적인** json 객체를 반환하도록 강제하는 기능이다.

비정형 자연어 세계와 정형화된 엔터프라이즈 시스템 db, CRM API를 연결하는 가장 확실한 브리지인 것.

**Hallucination in Arguments**: LLM이 존재하지 않는 파라미터를 지어내거나, 타입 캐스팅 오류 string을 기대하는데 integer를 반환한다거나 하는 경우가 빈번한데, 이에 대해 엄격한 validation 레이어가 필수적이다.

**api 장애 및 지연 (latency)**: 고객사 api가 타임아웃이 나서 500에러를 반환햇을때 observation이 즉 실패했을때, 에이전트가 api가 응답하지 않습니다 라고 포기할것인지 retry를 할것인가 fallback 시나리오를 태울것인지 thought를 프롬프트와 시스템 아키텍처 레벨에서 설계해야한다.

### 단일 에이전트 한계와 multi-agent 시스템 설계

프로젝트 초기에는 거대한 프롬프트 하나에 모든 도구 (db 검색, 이메일 발송, crm 업데이트 등)를 쥐여주는 단일 에이전트 구조로 시작한다. 하지만 비즈니스가 복잡해지면 모놀리식 아키텍처와 동일한 한계에 부딪히는데.

**단일 에이전트의 한계가 있다.**

1. **Context Overflow & Attention Dilution**: 프롬프트가 길어지고 툴이 많아질수록 LLM의 주의력이 분산되어 엉떵한 툴을 호출하거나 핵심 지시사항을 잊어버린다
2. **보안 및 권한 통제**: 단일 에이전트는 모든 권한을 가지고 있으므로 프롬프트 인젝션에 매우 취약하다.
3. **병목 현상**: 모든 테스크를 순차적으로 처리하므로 레이턴시가 기하 급수적으로 증가한다.

**Multi-Agent 시스템 설계**:  
단일 에이전트를 마이크로서비스 처럼 목적별로 잘개 쪼개서 해결할 수 있는데. 예를들어 의도파악 라우터 에이전트, db 조회 전용 에이전트, 답변 검수 및 교정 에이전트로 나누는 식

- **토폴로지 설계**: Hierachical(계층형) 매니저 에이전트가 작업을 분할하여 워커 에이전트들에게 지시하고 취합하는 구조다
  - **Network/Collbaorative(협력형)**: 에이전트들이 서로 대화하며 상태를 업데이트하는 구조다 LangGraph, AutoGen 등의 프레임워크를 활용한다.

**오케스트레이션과 상태 관리**: 에이전트 간에 데이터를 어떻게 주고 받을것인가(LangGraph의 State Graph) 무한 루프에 빠지지 않도록 depth limit을 어떻게 설정할 것인가등이 과제일 것이다.


### 에이전트의 기억 memory 관리 전략: short-term vs logn-term

LLM은 상태가 없는 stateless 함수다. 따라서 연속된 대화나 과거의 정보를 활용하려면 우리가 기억을 주입 injection 해주어야한다.

- **Short-term Memory (단기 기억)**
  - 개념은 현재 세션내의 대화 기록 context window 내에 들어가는 정보다.
  - 전략은 대화가 길어지면 token limit을 초과하므로 가장 오래된 대화를 버리는 sliding window 방식이나 이전 대화 내용 자체를 요약(summarization)하여 저장하는 방식을 사용한다. redis 같은 inmemory db를 활용함
- **Long-term Memory 및 Vector DB**
  - **개념**: 세션이 종료되어도 영구적으로 유지되어야하는 정보, 고객의 취향이나 과거 구매 이력 사내 메뉴얼같은 것들
  - **VectorDB의 역할**: 텍스트를 고차원 숫자 벡터로 변환하여 저장한다. 사용자가 질문을 하면 질문도 벡터화해서 수학적 거리가 가장 가까운 Consine Similarity 등 과거의 기억이나 문설르 찾아내어 LLM에게 전달하고 이것이 RAG의 기본 뼈대다. pincecone, milvus 혹은 postgresql의 pgvector 확장을 주로 사용한다.

**Entity Memory (객체 기억)**: 단순한 의미 기반 검색 vector 만으로는 부족하다. 특정 고객의 등급, 남은 포인트 같은 명확한 팩트는 vector db가 아닌 rdbms나 graph db entity형태로 명시적으로 저장하고 관리 업데이트 삭제하는 하이브리드 전략이 필수적이다.

**Cache Invalidation**: 장기기억이 변경되었을 때 예를들어 고객의 주소가 바뀐다거나 하면 vector db내의 이전 임베딩 데이터를 어떻게 무효화하고 업데이트할 것인가 시스템 신뢰성을 결정한다.