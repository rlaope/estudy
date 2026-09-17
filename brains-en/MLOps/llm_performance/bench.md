# LLM Serving Benchmarks and Performance Metrics

When deploying and operating LLM services, it's risky to rely solely on Average Latency, a typical performance metric for general web services.

This is because LLM inference, unlike typical REST APIs, **has fluctuating computational loads depending on the input, and the mechanism for generating the first and last tokens is entirely different.**

## Why LLM Services Should Be Evaluated Using Metrics Other Than Average Response Time (E2E Latency).

Traditional web services have relatively consistent data sizes and processing logic, whether 100 or 1000 users make requests. However, LLMs are different.

-   **Same Response Time, Completely Different User Experience**: Let's assume two requests both took exactly 10 seconds for their total response time (E2E Latency).
    -   User A: The first character appears in 0.2 seconds, and text is naturally typed on the screen for 10 seconds.
    -   User B: The screen freezes for 9 seconds, then all sentences appear in the last 1 second.
-   **The Trap of Averages**: In both cases, the system logs show the same 10 seconds, but User B might feel the service is down or is more likely to churn. Therefore, LLM performance evaluation should not rely on a single metric but instead **precisely measure the user's perceived time and the continuity of token generation by breaking them down.**

<br>

## Defining Key Performance Metrics for LLM Serving

Let's explore the six key metrics that must be included when optimizing LLM serving architectures and configuring Prometheus and Grafana dashboards.

### TTFT (Time To First Token)

This is the time from when a user sends a prompt until **the first token is generated and reaches the client.**

It's a metric that combines prefill stage latency and network overhead, and it determines the perceived responsiveness of the service for the user.

### ITL (Inter-Token Latency)

This is the time taken between consecutive tokens (the interval until the next token appears).

It's the latency of the decode stage. If ITL becomes longer than a human's reading speed, it creates a choppy, stuttering feeling, so it's usually **ideal to keep it below 50ms.**

### TPS (Tokens Per Second)

Not transactions. This is the total number of tokens generated per second. It's divided into two perspectives:

-   **Per-user TPS:** The number of tokens a single user receives per second ($\approx 1 / ITL$)
-   **System TPS:** The total number of tokens processed per second across the entire instance

### E2E Latency (End to End Latency)

This is the total time elapsed from the start of a request until the final End-of-Sequence (EOS) token is received.

It is calculated as $TTFT + (ITL \times \text{number of generated tokens})$.

### Throughput vs Goodput

**Throughput** is the total number of tokens (input + output) processed by the system per unit of time. It represents infrastructure cost-effectiveness and resource efficiency.

**Goodput** is the processing volume of **valid tokens that actually create business value** among the generated tokens. For example, requests where the user closes the window while reading (Aborted Request) or meaningless outputs truncated due to Max Token limits are counted in Throughput but excluded from Goodput.

<br>

## Load Test Implementation: vLLM Benchmark & Locust

The vLLM engine provides its own benchmark script for load testing.

To monitor actual user behavior patterns, load testing tools like Locust or k6 are integrated for system validation.

### Utilizing vLLM's Internal Benchmark Tool (Based on ShareGPT Dataset)

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

### Streaming API Load Test Script Using Locust

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

This is an example benchmark report derived from testing the vLLM (Llama-3-8B) environment to its hardware limits by increasing concurrent users (virtual users).

### Benchmark Data Summary (Based on GPU: NVIDIA A100 80GB 1GPU)

| 동시 사용자 수 (Concurrency) | 평균 TTFT (ms) | P99 TTFT (ms) | 평균 ITL (ms) | System Throughput (tokens/sec) | 성공률 (Success Rate) |
|-----------------------------|---------------:|--------------:|--------------:|-------------------------------:|----------------------:|
| 1명                         | 45             | 52            | 8.5           | 115                            | 100%                  |
| 10명                        | 68             | 85            | 12.1          | 820                            | 100%                  |
| 50명                        | 120            | 210           | 18.4          | 2,450                          | 100%                  |
| 100명 (임계점)              | 350            | 780           | 24.5          | 3,100                          | 100%                  |
| 200명 (과부하)              | 1,250          | 3,400         | 55.0          | 3,150                          | 98.2% (Timeout 발생)  |

### Results Analysis and Insights

-   **Effectiveness of Continuous Batching (1 to 100 users)**
    -   When concurrent users increase from 1 to 100, the system's overall throughput increases approximately 27 times, from 115 tokens/sec to 3,100 tokens/sec.
    -   This is because vLLM's **Continuous Batching** architecture efficiently groups idle GPU compute units for simultaneous processing (iteration-level scheduling). Up to this point, as concurrent users increase, GPU cost-effectiveness is maximized.
-   **Saturation Threshold: 100 -> 200 Users**
    -   Around 200 concurrent users, Throughput growth stalls (3,100 -> 3,150), and TTFT and ITL begin to skyrocket.
    -   **Cause:** New incoming requests must wait in the queue to perform prefill, causing the P99 tail latency for TTFT to extend to 3.4 seconds. Simultaneously, existing users' KV Caches fill up GPU memory, leading to extreme memory bandwidth contention for new token generation, and ITL exceeds 55ms, a level where readers can perceive stuttering.

In conclusion, the maximum cost-effective capacity range for this single GPU instance is between **80 and 100 concurrent users.**

If the service's SLA (Service Level Agreement) defines `P99 TTFT < 500ms` and `ITL < 30ms`, then a multi-node load balancing architecture should be designed to trigger Grafana alerts before concurrent users exceed 100 and replicate GPU instances via Kubernetes HPA.
