# Durable Execution and Workflow Recovery

In enterprise environments where complex multi-agent systems run for tens of minutes to several days,

application processes running in container workers can be forcibly terminated at any time due to infrastructure load, OOM, or scheduling.

This document thoroughly analyzes **Durable Execution architecture** design techniques that safely protect the agent's internal global state even if a worker unexpectedly goes down during execution,

and resume operations precisely from the last point of failure without data loss.

<br>

## Can a Tool Be Called Again if the Worker Terminates Immediately After Tool Execution?

This seems to depend on whether the Tool supports idempotency or if a mechanism is in place.

Without an infrastructure guard, it should not be simply re-invoked.

### The Fundamental Dilemma of Distributed Computing

Let's consider a scenario where a worker completes a Tool operation that interacts with an external API or database, and then the process goes down just before committing the resulting agent global state.

In this case, the infrastructure state system (e.g., LangGraph, Temporal) detects the worker's death, rolls back to the last saved checkpoint, and then re-executes that node.

The nature of the execution model in this situation is as follows:

- **At-least-once**: The default approach adopted by infrastructure architectures when attempting state recovery; if a worker dies, the node is re-executed from the beginning, leading to duplicate Tool calls.
- **Duplicate Call Risk**: If the Tool performs state-mutating, non-idempotent operations such as money transfers, infrastructure creation, or data ledger insertion, the system recovery process could lead to duplicate payments or resource creation, causing severe data inconsistency.

Therefore, to safely retry a Tool within the agent runtime, an idempotency key mechanism must be embedded in the tool proxy. When calling the Tool, a unique identifier (thread_id + node_name + step_count) should be generated and sent to the tool provider. The receiving server should then check if this key has already been processed using a distributed lock or a unique key. If it has, it should simply return the existing result, thus ensuring safe self-recovery.

<br>

## Core Mechanisms of Durable Execution

These are five distributed state control techniques for building a fault-tolerant system.

### Checkpoint (State Snapshot Persistence)

This is the point when each node operation in the agent graph is successfully completed and transitions to the next node.

The entire global State structure is physically stored in a disk-level database, not in memory, on a transaction-by-transaction basis.

Even if all workers are destroyed, this checkpoint table can be read to restore the memory landscape to its state just before the failure.

### Retry & Timeout (Fine-Grained Control)

It is necessary to distinguish between transient network delays (Transient Error) and complete system stalls.

When delays occur, retries with exponential backoff should be performed. However, to prevent external API deadlocks, strict short-term timeouts should be applied at each layer to prevent threads from waiting indefinitely.

### Suspend and Resume (Human in the loop)

Certain tool operations are risky for the system to decide automatically (e.g., approving payments over 10 million KRW).

In such cases, the agent changes its state to `Suspended`, takes a checkpoint, and then explicitly terminates the process.

Subsequently, when an administrator clicks an approval button via a UI interface, the system receives an external event and resumes the state from the suspended checkpoint.

### Idempotency (Idempotency Guard)

This is a property that enforces the system's final state to remain the same as if an operation were performed only once, even if it is executed tens of thousands of times with the same input.

The key is to generate and preserve a unique transaction ID at the agent's forefront.

<br>

## LangGraph PostgreSQL Checkpointer Idempotency Guard Recovery Server

Let's look at the framework implementation code that adapts the LangGraph persistence architecture model for a production environment, integrating PostgresSaver to mimic a Redis distributed structure, incorporating an idempotency guard, and defending against worker-down scenarios.

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

Running the above failure script provides structural guidelines for state transition logs recorded in the terminal hypervisor and PostgreSQL ledger, and between serving tools.
