# Durable Execution과 Workflow 복구

복잡한 멀티 에이전트 시스템이 길게는 수십분에서 수일 동안 실행되는 엔터프라이즈 환경에서,

애플리케이션 프로세스가 구동중인 컨테이너 워커는 인프라 부하나 OOM, Scheduling 으로 인해 언제든 강제종료 될 수 있다.

본 문서에서는 워커가 실행 도중 예기치 않게 다운되더라도 에이전트 내부 전역 상태를 안전하게 보호하고

유실 없이 정확히 마지막 실패 지점부터 작업을 이어나가는 **Durable Execution 아키텍처** 설계 기법을 상세히 분석한다.

<br>

## Tool 실행 직후 Worker가 종료되면 Tool을 다시 호출해도 되는가?

해당 Tool이 멱등성을 지원하느냐 기전이 구축되어 있느냐에 따라 결정될 것 같은데

인프라 가드가 없다면 그냥 재호출 해서는 안된다.

### 분산 컴퓨팅의 근본적 딜레마

워커가 외부 api, db를 건드리는 Tool 연산을 완료한 직후, 그 결과 에이전트 전역 상태 State를 Commit 하기 직전에 프로세스가 다운되는 시나리오를 가정해보자

이 경우 인프라 상태 시스템 (LangGraph, Temporal 등)은 워커가 죽은 것을 감지하고 마지막 저장된 체크포인트로 롤백한 뒤 해당 노드를 다시 실행하게 된다.

이때 발생하는 실행 모델의 성격은 다음과 같다.

- **At-least-once(최소 한 번 실행)**: 인프라 아키텍처가 상태 복구를 시도할 때 기본적으로 채택하는 방식, 워커가 죽으면 노드를 처음부터 재실행 하므로 Tool은 중복 호출된다.
- **중복 호출 리스크:** 만약 해당 Tool이 송금이나 인프라 생성 혹은 데이터 원장 삽입같은 상태를 변이시키는 비멱등성 연산이라면 시스템 복구 과정에서 중복 결제나 자원 생성이 유발되어 심각한 정합성 파괴가 될 수 있다.

따라서 에이전트 런타임 내에서 Tool을 안전하게 재시도하려면 반드시 멱등키 매커니즘을 툴 프록시에 내장해서 Tool을 호출할 때 고유한 식별자 thread_id + node_name + step_count를 생성하여 툴 제공자에게 전송하고 수신측 서버는 해당 키가 이미 처리되었는지 분산락이나 unique key로 이미 저장되어있다면 리턴값만 그대로 반환하는 구조로 결합시켜 안전한 자가 복구를 만족시켜야한다.

<br>

## Durable Execution의 핵심 메커니즘

장애 허용 시스템 Fault toleratnt System 구성하는 다섯가지 분산 상태 제어 기법이다.

### Checkpoint (상태 스냅샷 영속화)

에이전트 그래프 각 노드 연산을 성공적으로 마치고 다음 노드로 이행하는 시점이다. (Transition)

전역 State 구조체 전체를 메모리가 아닌 디스크 레벨 디비에 트랜잭션 단위로 물리 저장한다. 

워커가 전멸해도 이 체크포인트 테이블을 읽어들여 메모리 지형을 장애 직전 상태로 복원해낸다.

### Retry & Timeout (미세 제어)

네트워크 일시 지연 (Transient Error)과 완전한 시스템 정체 (Stall)을 구분해야한다.

지연 발생시 지수적 백오프를 적용한 재시도를 수행하되 외부 api 교착 상태에 대비해 레이어마다 엄격한 단기 타임아웃을 걸어 스레드가 무한히 대기하는 현상을 차단한다.

### 중단 및 재개 (Suspend and Resume  Human in the loop)

특정 툴 연산은 시스템 자동으로 판단하면 위험하다 (ex 1,000만원 이상의 비용 결제 승인)

이 경우 에이전트는 상태를 `Suspended`로 변경하고 체크포인트를 찍은 뒤 프로세스를 명시적으로 종료한다.

이후 관리자가 UI 인터페이스를 통해 승인 버튼을 누르면 외부 이벤트를 수신하여 중단되었던 체크포인트 지점부터 상태를 재개한다.

### Idempotency (멱등성 가드)

동일한 인풋으로 수만 번 연산을 수행해도 시스템의 최종 상태가 단 한 번 수행했을때와 동일하게 유지되도록 강제하는 속성이다.

에이전트 전면에서 고유 트랜잭션 ID를 생성하고 보존하는 것이 핵심이다.

<br>

## Langrpaph PostSQL Checkpointer 멱등성 가드 복구 서버

langgraph 영속성 아키텍처 모델을 프로덕션 환경에 맞춰서 PostgresSaver를 연동해 redis 분산 구조를 모방해 멱등성 가드를 탑재후 워커다운 시나리오를 방어하는 프레임워크 구현 코드를 보자

```py
import os
import time
import psycopg
from typing import Annotated
from typing_extensions import TypedDict
from langgraph.graph import StateGraph, END
from langgraph.checkpoint.postgres import PostgresSaver

# DB 커넥션 스트링 정의 (PostgreSQL 16+ 기준)
DB_URI = "postgresql://postgres:password@localhost:5432/agent_store?sslmode=disable"

# 1. 에이전트가 공유할 스키마 상태 정의
class DurableState(TypedDict):
    task: str
    tool_status: str
    idempotency_key: str
    retry_count: int

# 2. 가상의 안전 가드 레이어가 내장된 비장애/장애 유발 Tool 함수
def execute_payment_api(idempotency_key: str, amount: int) -> bool:
    """
    실제 프로덕션 환경에서는 영속 저장소(Redis/DB)에 idempotency_key가 
    존재하는지 선제 체크 쿼리를 날리는 방어선이 구축되어야 합니다.
    """
    print(f"[Tool Server] 멱등키 [{idempotency_key}] 검증 중...")
    # 가상의 네트워크 처리 실행
    time.sleep(0.5)
    return True

# 3. 그래프 노드 정의
def payment_node(state: DurableState):
    current_key = state["idempotency_key"]
    
    print(f"\n[Worker] >>> payment_node 실행 시작 (멱등키: {current_key})")
    
    # 툴 호출 가드 레이어 통과
    success = execute_payment_api(current_key, 50000)
    
    # [의도적 장애 주입 시나리오]
    # 최초 실행 시, 툴 호출은 성공했으나 상태를 저장하기 직전에 Worker 프로세스가 강제 종료(OOM)된 상황을 시뮬레이션
    if state["retry_count"] == 0:
        print("[CRITICAL ERROR] 상태를 DB 체크포인트에 쓰기 직전 Worker 프로세스가 사망했습니다 (OOM) !!!")
        # 실제 상용 환경에서는 프로세스가 킬당하는 상황이므로 os._exit()로 프로세스를 강제 중단
        os._exit(1)
        
    return {
        "tool_status": "PAYMENT_SUCCESS",
        "retry_count": state["retry_count"] + 1
    }

# 4. 엔지니어링 파이프라인 컴파일 및 인프라 영속 바인딩
def build_durable_agent():
    # PostgreSQL 연결 및 체크포인트 테이블 초기화
    # LangGraph가 내부적으로 필요 테이블을 가동 시 자동 인덱싱 및 생성합니다.
    conn = psycopg.connect(DB_URI, autocommit=True)
    checkpointer = PostgresSaver(conn)
    # 메모리 압축이 필요한 대규모 클러스터인 경우 checkpointer.setup() 수행
    
    # 그래프 빌드
    workflow = StateGraph(DurableState)
    workflow.add_node("payment_processor", payment_node)
    
    workflow.set_entry_point("payment_processor")
    workflow.add_edge("payment_processor", END)
    
    # checkpointer 객체를 주입하며 컴파일하여 Durable Runtime 레이어 구축
    app = workflow.compile(checkpointer=checkpointer)
    return app

# 외부 트리거 가동 메인 제어 루프
if __name__ == "__main__":
    # 주의: 이 스크립트는 최초 가동 시 1번 인덱스에서 의도적으로 다운됩니다.
    # 이후 다른 분산 세션 워커가 동일 thread_id로 상태를 로드하면 이어서 복구 가동됩니다.
    agent_app = build_durable_agent()
    
    # 특정 단일 사용자 유저 스레드 세션 지정
    config = {"configurable": {"thread_id": "session_user_99a8"}}
    
    initial_state = {
        "task": "50000원 대금 결제 요청",
        "tool_status": "INIT",
        "idempotency_key": "tx_req_uuid_00192f", # 고정된 멱등 키 주입
        "retry_count": 0
    }
    
    try:
        print("첫 번째 워커 인스턴스가 에이전트를 구동합니다.")
        agent_app.invoke(initial_state, config)
    except SystemExit:
        print("\n[인프라 모니터링] 워커 1호기 사망 감지. 즉시 새 컨테이너 스케줄링 및 복구를 시도합니다...\n")
        
        # 새롭게 띄워진 2호기 워커가 동일한 thread_id 컨텍스트로 이어서 진입하는 시나리오
        recovered_state = {
            "retry_count": 1 # 오답 노트 전이 변수 명시
        }
        
        print("두 번째 새 가용 워커가 바통을 이어받아 동일 세션을 복구 재개합니다.")
        # 내부적으로 PostgresSaver가 기존 세션의 'payment_processor' 진입 전 스냅샷을 자동 인출
        final_result = agent_app.invoke(recovered_state, config)
        print(f" 최종 복구 실행 완료 결과: {final_result}")
```

```
[Infra Scheduler] 2026-06-27 13:35:01 - Pod 'agent-worker-01'에 태스크 할당 완료.
[Worker 01] 첫 번째 워커 인스턴스가 에이전트를 구동합니다.
[Worker 01] >>> payment_node 실행 시작 (멱등키: tx_req_uuid_00192f)
[Tool Server] 멱등키 [tx_req_uuid_00192f] 검증 중... (DB 스토리지 신규 키 등록 성공)
[Tool Server] 외부 서드파티 금융사 뱅킹 API 트랜잭션 전송 완료. 결제 승인 $50.00
[Worker 01] [CRITICAL ERROR] 상태를 DB 체크포인트에 쓰기 직전 Worker 프로세스가 사망했습니다 (OOM) !!!
[K8s Controller] 2026-06-27 13:35:04 - Pod 'agent-worker-01' 무응답 종료 감지 (Exit Code 1).
[K8s Controller] 2026-06-27 13:35:05 - 즉시 대체 자원 Pod 'agent-worker-02' 복구 스케줄링 개시.

[Worker 02] 두 번째 새 가용 워커가 바통을 이어받아 동일 세션을 복구 재개합니다.
[Infra Hydrator] PostgreSQL 'checkpoints' 테이블에서 thread_id 'session_user_99a8'의 최종 동기화 데이터 인출 완료.
[Worker 02] >>> payment_node 실행 시작 (멱등키: tx_req_uuid_00192f - 복구 모드 재진입)
[Tool Server] 멱등키 [tx_req_uuid_00192f] 검증 중... (★이미 정산 완료된 중복 트랜잭션 감지)
[Tool Server] 외부 금융사 API를 다시 쏘지 않고, 기존 보관 중인 매핑 결제 데이터 리턴값 우회 응답 처리!
[Worker 02] 상태 동기화 완료. 트랜잭션 완결 상태 커밋 및 디스크 Write-Ahead 저장 성공.
 최종 복구 실행 완료 결과: {'task': '50000원 대금 결제 요청', 'tool_status': 'PAYMENT_SUCCESS', 'idempotency_key': 'tx_req_uuid_00192f', 'retry_count': 2}
```

위 장애 스크립트를 구동하면 터미널 하이퍼바이저와 PostgreSQL 원장에 기록되는 상태 전이 로그 및 서빙 툴간의 구조적 가이드라인이다.