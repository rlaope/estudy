# LLM Serving Benchmark와 성능 지표

LLM 서비스를 배포하고 운영할 때, 일반적인 웹 서비스의 성능 지표인 평균 응답시간 Average Latency만을 기준으로 삼는것은 위험하다.

LLM 추론은 일반적인 REST API와 달리 **입력값에 따라 연산량이 요동치고 첫 토큰과 마지막 토큰이 나오기까지 메커니즘이 완전히 다르기 때문이다.**

## 왜 평균 응답시간 (E2E Latency)가 아니라 다른 지표로 LLM 서비스를 판단하는가.

전통적인 웹서비스는 100명이 요청하든 1000명이 요청하든 반환되는 데이터의 크기와 처리 로직이 비교적 일정하다. 그러나 LLM은 다르다

- **동일한 응답 시간, 전혀 다른 사용자 경험**: 어떤 두 요청의 전체 응답 시간 (E2E Latency)이 똑같이 10초가 걸렸다고 가정해보겠다.
  - A 사용자: 0.2초만에 첫글자가 보이고 10초동안 화면에 텍스트가 자연스럽게 타이핑된다.
  - B 사용자: 9초동안 화면이 멈춰있다가 마지막 1초에 모든 문장이 나온다.
- **평균의 함정**: 두 경우 모두 시스템 로그에는 똑같은 10초지만 B사용자는 서비스가 다운되었다고 느낄 수 있거나 이탈할 확률이 높다. 따라서 LLM 성능 평가는 단일 수치가 아닌 **사용자의 체감 시간과 토큰 생성의 연속성을 쪼개어 정밀하게 측정해야한다.**

<br>

## LLM 서빙 핵심 성능 지표 정의

LLM 서빙 아키텍처를 최적화하고 Prometheus와 Grafana의 대시보드를 구성할 때 반드시 포함해야하는 6대 핵심 지표를 알아보자.

### TTFT (Time To First Token)

사용자가 프롬프트를 전송한 시점부터 **첫 번째 토큰이 생성되어 클라이언트에 도달할때까지 시간이다.**

Prefill 단계의 지연시간과 네트워크 오버헤드가 합쳐진 지표이기도하며, 사용자가 체감하는 서비스의 반응속도를 결정하는 지표다.

### ITL (Inter-Token Latency)

연속적인 토큰과 토큰 사이에 걸리는 시간 (다음 토큰이 나오기까지의 간격)이다.

Decode 단계의 지연 시간이고. 사람이 글을 읽는 속도보다 ITL이 길어지면 뚝뚝 끊기는 느낌 Stuttering을 주게 되므로, 보통 **50ms 아래로 유지하는것이 이상적이다.**

### TPS (Tokens Per Second)

transaction아니다. 초당 생성되는 토큰의 총량이다. 두 가지 관점으로 나뉜다.

- **Per-user TPS:** 단일 사용자가 초당 제공받는 토큰 수  ($\approx 1 / ITL$)
- **System TPS:** 인스턴스 전체에서 처리하는 초당 총 토큰 수

### E2E Latency (End to ENd Latency)

요청 시작부터 마지막 종료 토큰 EOS를 받을 때까지의 총 소요 시간이다.

$TTFT + (ITL \times \text{생성된 토큰 수})$로 계산된다.

### Throughput (처리량) vs Goodput (유효 처리량)

**Throughput**은 시스템이 단위 시간초당 처리한 전체 토큰수 input + output이다. 인프라와 가성비와 자원 효율성을 대변한다.

**Goodput**은 생성된 토큰 중 **실제 비즈니스 가치를 창출한 유효 토큰의 처리량이다.** 예를들어 사용자가 중간에 읽다가 창을 닫아버린 요청 (Aborted Request)이나 Max Token 제한에 걸려 잘려나간 무의미한 출력은 Throughput에는 잡히지만 Goodput에서는 제외된다.

<br>

## 부하 테스트 구현: vLLM Benchmark & Locust

vLLM엔진은 자체적으로 부하테스트를 위한 벤치마크 스크립트를 제공한다.

실제 사용자 행동 패턴을 모니터링하기 위해 Locust나 k6 같은 부하테스트 툴을 연동해 시스템 검증을 한다.

### vLLM 내부 벤치마크 도구 활용 (ShareGPT 데이터셋 기준)

```bash
# vLLM 소스코드에 포함된 벤치마크 스크립트 실행 예시
python3 benchmarks/benchmark_serving.py \
    --backend vllm \
    --model /models/Llama-3-8B-Instruct \
    --dataset-name sharegpt \
    --dataset-path ./ShareGPT_V3_unfiltered_cleaned_split.json \
    --num-prompts 1000 \
    --request-rate 10.0 # 초당 10개 요청 주입 (Concurreny 제어 가능)
```

### Locust를 이용한 Streaming API 부하 테스트 스크립트

```py
# python -m locust -f locustfile.py
from locust import HttpUser, task, between
import json

class LLMUser(HttpUser):
    wait_time = between(1, 3) # 유저당 요청 간격 1~3초 무작위

    @task
    def generate_stream(self):
        payload = {
            "model": "meta-llama/Meta-Llama-3-8B-Instruct",
            "messages": [{"role": "user", "content": "Write a long essay about quantum computing."}],
            "stream": True, # 스트리밍 활성화
            "max_tokens": 128
        }
        
        headers = {"Authorization": "Bearer EMPTY", "Content-Type": "application/json"}
        
        with self.client.post("/v1/chat/completions", json=payload, headers=headers, catch_response=True, stream=True) as response:
            if response.status_code == 200:
                # 첫 토큰 타임스탬프 기록을 위한 로직 구현 가능
                for line in response.iter_lines():
                    if line:
                        pass # 스트리밍 데이터 청크 처리
                response.success()
            else:
                response.failure(f"Status code: {response.status_code}")
```

동시 사용자 concurrent users (가상 유저)를 늘려가며 vLLM(Llama-3-8B) 환경에서 하드웨어 한계점까지 테스트했을때 도출되는 벤치마크 결과 보고서예시다

### 벤치마크 데이터 요약 (GPU: NVIDIA A100 80GB 1GPU 기준)

| 동시 사용자 수 (Concurrency) | 평균 TTFT (ms) | P99 TTFT (ms) | 평균 ITL (ms) | System Throughput (tokens/sec) | 성공률 (Success Rate) |
|-----------------------------|---------------:|--------------:|--------------:|-------------------------------:|----------------------:|
| 1명                         | 45             | 52            | 8.5           | 115                            | 100%                  |
| 10명                        | 68             | 85            | 12.1          | 820                            | 100%                  |
| 50명                        | 120            | 210           | 18.4          | 2,450                          | 100%                  |
| 100명 (임계점)              | 350            | 780           | 24.5          | 3,100                          | 100%                  |
| 200명 (과부하)              | 1,250          | 3,400         | 55.0          | 3,150                          | 98.2% (Timeout 발생)  |


### 결과 분석 및 인사이트

- **Continuous Batching의 효과 (1명 -> 100명 구간)**
  - 동시 사용자가 1 -> 100명으로 증가할 때 시스템 전체 처리랑 throughput이 115 tokens/sec에서 3,100 tokens/sec으로 약 27배 증가한다.
  - 이는 vLLM의 **Continous Batching** 아키텍처가 노는 GPU 연산 유닛들을 효율적으로 묶어서 한 번에 처리(Iteration-level scheduling) 하기 때문이다. 이 구간까지는 동시 사용자가 늘어날수록 gpu 가성비가 극대화된다.
- **포화 임계점 100 -> 200명 구간**
  - 동시 사용자 200명쯤 되자 Throughput의 성장은 멈추고 3,100 -> 3,150, TTFT와 ITL이 폭등하기 시작한다.
  - **원인:** 신규 진입 요청들은 Prefill을 수행하기 위해 Queue에서 대기해야하므로 TTFT 꼬리 지연시간 P99가 3.4초까지 밀리게 된다. 동시에 기존 처리중인 유저들의 KV Cache가 GPU 메모리를 가득 채우면서 새로운 토큰 생성을 위한 메몰 ㅣ대역폭 경쟁이 극에 달해 ITL이 독자가 끊김을 느낄 수 있는 수치 55ms를 초과한다.

결론적으로 해당 단일 GPU 인스턴스 최대 수용 가능한 가성비 구간은 **동시 사용자 80~100** 사이이다.


서비스의 SLA(Service Level Agreement) 기준으로 `P99 TTFT < 500ms`, `ITL < 30ms`를 정의했다면, 동시 사용자 100명을 넘기전에 grafana 알람을 발생시키고 k8s의 HPA를 통해 GPU 인스턴스를 복제하도록 다중 노드 로드밸런싱 아키텍처를 설계해야한다.,