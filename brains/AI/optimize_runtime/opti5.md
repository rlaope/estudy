# Continuous Batching 알고리즘

질문으로 시작해보자, 이전 주제에서 다룬 Dynamic Batching은 여러 요청을 모아 한 번에 처리하는 훌륭한 기술이였다. 그런데 LLM 텍스트 생성에 4개의 요청을 묶음으로 만들었을때 3개는 10글자만 생성하고 끝났는데 나머지 1개가 1000글자를 생성해야한다면 어떻게 될까

전통적인 배칭 방식에서는 가장 긴요청이 끝날때까지 앞서 끝난 3개의 GPU 연산 슬롯이 텅 빈채로 대기해야하는 문제가 존재한다.

이 거대한 비효율을 타파하기 위해 등장한 기술이 continuous batching 또는 iteration-level scheduling 이다.

- **Static/Dynamic Batching의 한계**: request level에서 한 번 묶인 배치는 모든 요청이 끝날 때까지 해제되지 않고 LLM은 각 요청마다 생성되는 텍스트 길이가 천차만별이므로 낭비가 심하다.
- **Continuous Batching (Iteration-Level)**: 단어가 한 글자 토큰이 생성될 때마다 스케줄러가 배치의 상태를 검사하고, 생성이 끝난 요청은 그 즉시 배치에서 방출하고 대기열에 있던 새로운 요청을 빈자리에 즉각 투입하여 gpu를 100% 가동하는 알고리즘이다 vLLM, TGI등의 핵심 엔진이 채택한 방식이다.

<br>

## 문제 정의

즉, 기존 서빙 엔진은 reqest 단위로 스케줄링을 했기 때문에 배치 내에서 가장 긴 응답 길이에 전체가 묶이는 **Head-of-Line Blocking** 현상이 존재했다.

예를들면, 배치사이즈가 4인 gpu 환경에서 3개의 슬롯이 연산을 마치고 놀고 있는데도, 남은 1개의 요청이 끝날때까지 큐에 대기중인 수십명의 새로운 사용자는 gpu에 진입조차 못하고 대기(혹은 타임아웃)을 겪게된다.

### 해결 방식

- **Token-by-Token 스케줄링 검사**: 토큰 단위 검사인데 LLM이 Forward Pass(1토큰 생성 연산)을 1회 마칠때마다 스케줄러가 개입하여 EOS(End of Sentence) 토큰이 나온 시퀀스를 확인한다.
- **즉각적인 상태 전환 (Eviction & Injection)**: EOS가 발생한 요청의 결과는 클라이언트에게 즉시 스트리밍으로 반환 완료하고 해당 VRAM을 해제한다. 동시에 Pending Queue에 있던 새 요청의 프롬프트를 빈자리에 밀어 넣어 다음 Forward Pass에 합류시킨다. 이를 통해 gpu 유휴시간을 0에 가깝게 만든다.

#### Forward Pass

forward pass는 순전파로 데이터가 신경망 입력층으로 들어와 은닉층의 행렬 연산을 거친뒤 최종 예측값을 도출해내는 직진 방향의 연산 흐름으로 LLM 서빙 관점에서는 1회의 forward pass를 마쳤다는 것은 모델 전체 레이어를 한 번 통과하여 다음 단계 토큰 1개를 예측해냈다는 것을 의미한다.

보통 순전파 이후 오차를 계산해 거꾸로 돌려보내는 역전파를 통해 가중치를 업데이트한다. 하지만 실제 서비스를 운영하는 api 서버에서는 역전파가 필요없고 오직 forward pass만 무한히 반복하여 고객의 요청을 처리해야하고 이 차이를 코드레벨에서 명확히 통제하지 않으면 프레임워크가 불필요한 메모리 gradient를 계속 잡고있어 vram 누수와 oom 가능성이 있다

- **Prefill Phase 프롬프트 처리**: 사용자의 질문 전체를 묶어 한 번의 무겁고 거대한 순전파로 처리해 전체 문맥 kv cache 계산으로 서빙
- **Decode Phase (토큰 생성)**: prefill 결과 바탕으로 1토큰을 생성할 때마다 1회의 가벼운 forward pass를 반복 실행해 앞서 다룬 continuous batching은 바로 이 단계에서 효율을 극대화한다.

<br>

## 상세 동작 원리 및 구조화

전통적인 배치와 Continuous Batching의 GPU 슬롯 활용도를 비교한걸 보자

```mermaid
gantt
    title Traditional Batching vs Continuous Batching
    dateFormat  s
    axisFormat %S
    
    section Traditional (낭비 발생)
    Req 1 (10토큰) :active, a1, 0, 2s
    Req 2 (10토큰) :active, a2, 0, 2s
    Req 3 (30토큰) :active, a3, 0, 6s
    GPU 슬롯 1 유휴 상태 (Idle) :crit, c1, 2s, 6s
    GPU 슬롯 2 유휴 상태 (Idle) :crit, c2, 2s, 6s
    새로운 요청 진입 불가 :c3, 2s, 6s

    section Continuous Batching (즉시 투입)
    Req 1 (10토큰) :active, b1, 0, 2s
    Req 2 (10토큰) :active, b2, 0, 2s
    Req 3 (30토큰) :active, b3, 0, 6s
    새 요청 Req 4 즉시 투입 :done, b4, 2s, 6s
    새 요청 Req 5 즉시 투입 :done, b5, 2s, 5s
```

### Example

엔진 내부에서 매 이터레이션 토큰 생성마다 동작하는 스케줄링 로직의 파이썬 의사 코드를 보며 이해를 해보자

```py
class ContinuousBatchingScheduler:
    def __init__(self, max_batch_size=4):
        self.max_batch_size = max_batch_size
        self.waiting_queue = []  # 대기 중인 새 요청
        self.active_batch = []   # 현재 GPU에서 연산 중인 요청 슬롯

    def step(self):
        """매 토큰 생성(Forward Pass) 직후 호출되는 스케줄링 로직"""
        
        # 1. 종료된 요청 방출 (Eviction)
        retained_batch = []
        for req in self.active_batch:
            if req.is_finished(): # EOS 토큰이 나왔거나 최대 길이에 도달했는지 확인
                print(f"[{req.id}] 생성 완료. 배치에서 제거 및 결과 반환.")
            else:
                retained_batch.append(req)
        self.active_batch = retained_batch

        # 2. 빈자리만큼 새 요청 투입 (Injection)
        available_slots = self.max_batch_size - len(self.active_batch)
        while available_slots > 0 and self.waiting_queue:
            new_req = self.waiting_queue.pop(0)
            self.active_batch.append(new_req)
            print(f"[{new_req.id}] 빈 슬롯에 새 요청 즉시 투입.")
            available_slots -= 1

        # 3. 구성된 새로운 배치로 다음 1토큰 동시 추론 진행 (GPU Forward Pass)
        if self.active_batch:
            # model.forward(self.active_batch)
            pass
```

보통은 위처럼 직접 스케줄러를 짜지는 않고 프로덕션 vLLM의 Continuous Batching 효율을 극대화하기 위해 엔진의 스케줄링 한계치를 제어하는 설정 코드다.

```py
from vllm.engine.arg_utils import AsyncEngineArgs
from vllm.engine.async_llm_engine import AsyncLLMEngine

# Continuous Batching 스케줄러가 최적의 결정을 내릴 수 있도록 제어값을 부여
engine_args = AsyncEngineArgs(
    model="meta-llama/Llama-3-8B",
    
    # [핵심 파라미터 1] max_num_seqs
    # Continuous Batching 큐에서 '동시에 GPU 연산 슬롯에 올릴 수 있는 최대 문장(요청) 개수'
    # 이 값이 너무 크면 VRAM OOM이 발생하고, 너무 작으면 배치에 빈자리가 생겨 Throughput이 떨어짐
    max_num_seqs=256,
    
    # [핵심 파라미터 2] max_num_batched_tokens
    # 한 번의 스텝(Iteration)에서 처리할 수 있는 총 토큰 수의 합 (Prompt 토큰 + 생성 토큰)
    # 새 요청(프롬프트)이 중간에 Inject 될 때, 이 토큰 수 한계를 넘지 않는 선에서만 투입을 허용함
    max_num_batched_tokens=8192,
    
    # 앞서 학습한 PagedAttention과 결합되어 VRAM을 효율적으로 재활용
    gpu_memory_utilization=0.9
)

# 비동기 엔진 기동
engine = AsyncLLMEngine.from_engine_args(engine_args)
```

Continuous Batching은 내부적으로 요청을 매우 빠르게 넣고 빼기 때문에, 외부 api 서버 입장에서는 gpu가 얼마나 힘들어하는지 알 수 없다.

트래픽 폭주시 엔진 내부 스케줄러 대기열 pending queue를 모니터링하여 무중단 스케일아웃을 수행하기 위한 백그라운드 옵저버빌리티 패턴을 보면

```py
from fastapi import FastAPI
import asyncio
from prometheus_client import Gauge
import logging

logger = logging.getLogger(__name__)
app = FastAPI()

# 1. Prometheus 메트릭 정의 (그라파나 대시보드 연동용)
# 대기 중인 요청 수, 현재 처리 중인 요청 수, GPU 캐시 사용률을 추적합니다.
METRIC_PENDING_REQUESTS = Gauge('vllm_pending_requests', 'Number of requests waiting in queue')
METRIC_RUNNING_REQUESTS = Gauge('vllm_running_requests', 'Number of requests currently in continuous batch')
METRIC_KV_CACHE_USAGE = Gauge('vllm_kv_cache_usage_percent', 'GPU KV Cache usage percentage')

async def log_continuous_batching_stats(engine):
    """엔진 내부의 스케줄러 통계를 주기적으로 빼내어 메트릭으로 노출하는 백그라운드 태스크"""
    while True:
        try:
            # vLLM 엔진 내부의 현재 스케줄링 상태(Stats) 강제 조회
            stats = await engine.get_decoding_stats()
            
            # 메트릭 게이지 업데이트
            METRIC_PENDING_REQUESTS.set(stats.num_requests_waiting)
            METRIC_RUNNING_REQUESTS.set(stats.num_requests_running)
            METRIC_KV_CACHE_USAGE.set(stats.gpu_cache_usage)
            
            # 특정 임계치 초과 시 경고 로깅 (Auto-scaling 트리거 조건으로 활용 가능)
            if stats.num_requests_waiting > 100:
                logger.warning(f"🚨 Continuous Batching Queue 포화 상태! 대기 중: {stats.num_requests_waiting}건")
                
        except Exception as e:
            logger.error(f"메트릭 수집 중 오류: {e}")
            
        await asyncio.sleep(2.0) # 2초마다 수집

@app.on_event("startup")
async def startup_event():
    # 애플리케이션 시작 시 백그라운드 모니터링 루프 실행
    asyncio.create_task(log_continuous_batching_stats(engine))
```

- Continuous Batching이 만능은 아니라 `max_num_seqs`의 한계에 도달하면 결국 waiting queue에 요청이 쌓이는건 마찬가지라 블랙박스로 두지 ㅇ낳고 get_decoding_stats()와 같은 내부 api 폴링으로 메트릭 추출을 해내야한다.
- 이렇게 수집된 vllm_pending_requests 지표는 k8s의 HPA 설정시 custom metric으로 cpu/mem 이외 실제 대기중인 큐 길이 기준으로 ai 서버를 자동 증설하는 기준으로도 사용할 수 있다.