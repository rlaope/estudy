# Routing, Scheduling, Autoscaling, and Multi-LoRA in Model Serving

Let's start by posing some questions.

- A single engine achieved an 80% RadixAttention hit rate, but when the replicas were increased to 4, it dropped to 20%. Why?
- GPU utilization is 95%, but the autoscaler isn't scaling out. **Why shouldn't we scale based on GPU utilization in the first place?**
- If interactive requests from paying customers and batch requests from free users are mixed in the same queue, who should be processed first?
- If 300 clients each want their own fine-tuned model, do we need 300 sets of GPUs?
- When traffic suddenly doubles, how many seconds will it take for a new pod to generate its first token?

The answers to all these questions lie outside the engine itself, as these problems emerge the moment you have 'n' engines.

When considering the performance of a single engine, we focused on batch size, KV cache optimization, kernels, parallelization, PD separation, speculative decoding, and quantization.

This time, we're focusing on cluster performance = deciding where, when, and to how many engines to send requests.

```
[Client]
     ↓
[Gateway / Router]      ← Where to send (cache-aware routing)
     ↓                     ← Accept now, defer, or drop (admission / flow control)
[Engine Replicas × N]        ← Who goes first inside the engine (scheduling policy)
     ↓                     ← Batching, KV, kernels, parallelization, PD separation
[GPU]
     ↑
[Autoscaler]             ← How many N, how long to scale up
                           ← Hundreds of customers with one model (Multi-LoRA)
```

| Case | Scenario | Hardware | Key Considerations (Part 2) |
|------|----------|----------|-------------------|
| A. In-house coding assistant | 200 engineers → scaling to 1,000 | 1 node of 8×H200 → 4 nodes | Morning peak hours, inter-replica cache |
| B. Document QA / RAG | Repeated queries on the same document | 4×H100 (8B model, DP 4) | Routing determines cache hit rate |
| C. Large-scale MoE serving | DeepSeek V4-class | 12 nodes / 96 GPUs, PD separation | Scaling prefill and decode pools separately |
| D. Multi-tenant SaaS (new) | 300 clients, each with their own LoRA | 4×H100 | Adapter management, fairness between tenants |

### Overview

The reasons for running multiple replicas include insufficient capacity, service failure if one dies, or the need for rolling updates.

The problem arises the moment you place a **standard L7 load balancer (round-robin, least-connections)** in front of the replicas. While this approach works well for web servers, it breaks down for LLMs for three reasons:

1.  **Request costs are not uniform**: Web requests usually have similar costs, but for LLMs, a 100-token question and a 32k-token document summary both count as 1 request. Outputs can also vary, mixing 10-token responses with 4,000-token ones. **The number of connections is no longer an accurate measure of load.**
2.  **Engines are stateful**: RadixAttention and prefix caching benefit from the engine remembering KV states from previous requests. If the load balancer treats engines as **interchangeable stateless servers**, this state becomes meaningless.
3.  **True load signals are not at the HTTP level**: Queue depth, KV cache utilization, and the number of currently decoding sequences are internal engine values, which the load balancer currently doesn't know.

**Let's do the math - how caching varies by routing method**

Let's take round-robin as an example. If 4 replicas each cache the same document separately, the memory of 4 GPUs would only amount to 1 GPU's worth of cache. If the same document is always sent to the same replica, then the 4 GPUs can each store different documents, fully utilizing the capacity of all 4 GPUs.

However, if this isn't done, and requests are sent in a round-robin fashion, cache misses will occur, leading to re-computation. This highlights the importance of routing.

Case B: When an 8B model is deployed one per H100 GPU across 4 H100s with DP=4.

If deployed on a single machine first:

```
[1] KV size per token (Llama-3.1-8B)
    2 × 32 layers × 8 KV heads × 128 dimensions × 2 bytes (BF16)
    = 131,072 bytes = 128 KiB / token

[2] KV cache capacity of one replica
    80GB × 0.88 (mem-fraction-static) ≈ 70GB
    - Weights 16GB
    ≈ 54GB → 54GB ÷ 128KiB ≈ 410K tokens

[3] Workload: 200 frequently queried documents × average 8K tokens
    = 1.6 million tokens (this is the "working set" we want in cache)
```

To store all 200 documents, 1.6 million tokens are needed, and a single replica can hold about 1/4 of that.

**If sent via round-robin**

For document A, the first request goes to Replica 1, the second to Replica 2, the third to Replica 3, and so on, in a rotating manner.

- Each replica tries to remember 200 documents (1.6 million tokens), but only has about 410K tokens of cache.
- Older documents are evicted whenever new ones arrive.
- The probability that a queried document is still in the cache is about 25% (cache hit).
- The remaining 75% require re-prefilling the 8K tokens of the document from scratch.

Therefore, if the router sends the same document to the same replica, the cache hit rate can be increased.

| | Round-robin | Same document → Same replica |
|---|---|---|
| Documents one replica needs to remember | 200 (1.6 million tokens) | 50 (400K tokens) |
| Cache space of one replica | For 50 documents (410K tokens) | For 50 documents (410K tokens) |
| Cache hit rate (approx.) | ~25% | Almost all after the first time |
| Documents stored across the entire cluster | 4 replicas store similar, overlapping documents (duplication) | All 200 documents stored without overlap |

<br>

## Cache-Aware Routing - Where to Send

### Cache Affinity vs. Load Balancing

From a cache perspective alone, the answer is simply to send the request to the replica that has the corresponding prefix.

But **what if there's a popular prefix?** For example, if all requests start with the same 3k-token system prompt, a cache-only router would **direct all requests to a single replica**. The other three would be idle, while one's queue would explode.

```
캐시만 보는 라우팅  → 적중률 ↑, 핫스팟 → TTFT p99 폭발 ❌
부하만 보는 라우팅  → 균등 분산, 캐시 무의미 → 프리필 반복 ❌
둘을 섞는다         → 적중률을 조금 양보하고 핫스팟을 피함 ✅
```

> To draw an analogy, if you keep sending customers to a popular restaurant (Store #1), the line in front of it will stretch 50 meters. At some point, the benefit of Store #1 having staff who know the menu, leading to faster orders, will be outweighed by the longer waiting times. The router's role is to calculate this break-even point.

Ultimately, every cache-aware router differs in how it blends these two factors.

**Three Ways a Router Knows Cache State**

| Method | How Cache State is Known | Advantages | Disadvantages | Representative Implementations |
|------|---------------------|------|------|----------|
| **Approximate** | Router records "where it sent what" in its own tree and **assumes** it's still in cache | No engine modification needed, simple | Engine evicts but router doesn't know (drift) | SGLang Model Gateway `cache_aware`, llm-d default scorer, Dynamo `--no-kv-events` |
| **Precise** | Engine **publishes** KV block store/evict **events** and router maintains a global index | Reflects actual cache state | Event bus required, overhead | Dynamo KV Router (default), llm-d precise scorer |
| **Hash** | Hashes prefix for consistent hashing | Stateless, very cheap | Unaware of load, cannot handle hot prefixes | `prefix_hash`, `consistent_hashing` policy |

Which method is correct depends on the workload. The llm-d documentation explains that the default profile uses an approximate scorer that predicts cache location by observing request traffic, and that precise routing, which inspects the actual cache entries of vLLM instances, further increases hit rates and is expected to become the default in the future. Red Hat documentation also summarizes that the approximate method is GA and recommended for most deployments, while the precise method is suitable for workloads where prefix patterns change significantly.

### **A Look Inside SGLang Model Gateway's `cache_aware`**

First, a name clarification: the old name was SGLang Router (SGL Router).

The name changed in December 2025, so the source directory moved from `sgl-router/` to `sgl-model-gateway/`, and the Prometheus metric prefix changed from `sgl_router_*` to `smg_*`. Be aware that if your dashboard queries are written with the old names, they will silently return empty results.

It offers various load balancing strategies such as `random`, `round_robin`, `cache_aware`, `power_of_two`, and `bucket`.

The core `cache_aware` operation looks like this:

```
Request arrives
  │
  ├─ Is the load imbalanced? (Queue difference between busiest and least busy worker exceeds threshold)
  │     └─ YES → Ignore cache, send to worker with shortest queue (hotspot prevention takes precedence)
  │
  └─ NO → Calculate the longest prefix match ratio between each worker's "router-side radix tree" and the request
          │
          ├─ If highest match ratio > cache_threshold (default 0.5)
          │     └─ Send to that worker (aim for cache hit)
          │
          └─ Otherwise → Send to the worker with the smallest tree
                      (= plant the new prefix where there's most cache room)
```

The last branch is important: **where a new prefix is "planted"** determines future hit rates.

If it's planted anywhere, that replica might already be busy when requests with the same prefix later flood in.

`--cache-threshold` is the minimum prefix match ratio for cache routing, defaulting to 0.5, and `--balance-abs-threshold` defaults to 32.

And one thing to remember is that this router's tree is **text-based, not token-based**. The `cache_aware` policy purely looks at prefixes from the router's perspective, using a radix tree over text, not tokens. This means its boundaries might subtly differ from the token-based radix tree inside the engine.

```bash
# 2 workers + cache-aware routing
python -m sglang_router.launch_router \
  --worker-urls http://worker1:8000 http://worker2:8000 \
  --policy cache_aware \
  --cache-threshold 0.5 \
  --balance-abs-threshold 32 \
  --max-concurrent-requests 512
```

### Dynamo KV Router - Viewing as a Cost Function

NVIDIA Dynamo expresses this tension as an **explicit cost function**.

The formula is simple and easy to understand.

The cost formula is `cost = overlap_score_weight * prefill_blocks + decode_blocks`, and the router selects the worker with the lowest cost. If `router_temperature` is set to a non-zero value, softmax sampling is applied to the normalized cost logits, introducing randomness into the selection.

- `prefill_blocks`: The **number of blocks that need to be newly computed** if sent to this worker (total blocks - cache hit blocks). This is the prefill cost, i.e., the TTFT term.
- `decode_blocks`: The number of active KV blocks currently being decoded on this worker, representing the decode load, i.e., the ITL term.
- `overlap_score_weight`: The exchange rate between the two terms.

Let's calculate with hypothetical numbers: a request of 4,096 tokens, with 16-token blocks, means 256 blocks.

```
Worker 1: Cache hit 200 blocks → prefill 56 blocks, active decode 900 blocks (busy)
Worker 2: Cache hit   0 blocks → prefill 256 blocks, active decode 300 blocks (idle)

weight = 1.0
  W1 = 1.0 × 56  + 900 = 956
  W2 = 1.0 × 256 + 300 = 556   ← Selected (sacrificing cache for idle worker)

weight = 3.0
  W1 = 3.0 × 56  + 900 = 1,068
  W2 = 3.0 × 256 + 300 = 1,068   ← Tie

weight = 4.0
  W1 = 4.0 × 56  + 900 = 1,124   ← Selected (tolerating busy worker for cache)
  W2 = 4.0 × 256 + 300 = 1,324
```

You can see how a single `weight` value changes the decision for the same request and cluster state.

The fact that there's no single optimal setting in serving tuning can also be observed at the routing layer.

The tuning direction is intuitive: increasing `weight` above 1.0 prioritizes prefill cost reduction (TTFT improvement), while lowering it below 1.0 prioritizes decode performance (ITL).

For prefill-heavy workloads like long prompts and short generations, increase `weight`. For decode-heavy workloads like short prompts and long generations, decrease `weight`.

The latest version even incorporates hierarchical caching (HBM -> CPU DRAM -> Disk) into the cost. Different weights are assigned to cache hits on the GPU, host memory, and disk, which are then subtracted from the prefill blocks. This means that cache in CPU DRAM is valued less than cache in HBM because it's more expensive to retrieve (H100 HBM approx. 3,350 GB/s vs. PCIe 5.0 approx. 63 GB/s). The break-even point of "recompute or fetch" has effectively been integrated into the router's cost function.

### llm-d and Gateway API Inference Extension - The K8s Standard

There's a movement to standardize routing logic, which is currently built separately for each engine, using an extension to the Kubernetes Gateway API.

This is the Gateway API Inference Extension (GAIE), and llm-d is a stack built on top of it.

The structure is a three-stage pipeline. According to the Red Hat blog, the EPP (Endpoint Picker) inspects all model server pods before sending a request to find the optimal target. The default scorer checks how much of the request's prefix each pod has cached. It selects the pod with the highest score, and in case of a tie, it chooses randomly.

```
Request
 ↓
[Filter]  Remove unusable pods (e.g., draining, different model, no LoRA)
 ↓
[Score]   Sum of scorer-specific scores × weights for each remaining pod
            - prefix-cache-scorer   (cache affinity)
            - queue-scorer          (queue length)
            - kv-cache-utilization  (KV availability)
 ↓
[Pick]    Select highest score
```

The default profile scores pods using `prefix-cache-scorer` and `queue-scorer`, giving higher weight to the `prefix-cache` side. This structure is identical to Dynamo's cost function; the only difference is whether it's expressed as cost minimization or score maximization.

One interesting improvement: **cold request distribution**. llm-d v0.4 directly addressed the problem mentioned above, where planting new prefixes anywhere could later create hotspots. It introduced a scorer that intelligently distributes cold requests, which act as seeds for new prefix caches, preventing hotspots during cache formation. This eliminated the queue oscillations seen in v0.3, reducing P99 TTFT by 50%.

And important news for SGLang users: in v0.8, SGLang entered the precise prefix cache routing path. The `prefix cache` scorer, which was previously vLLM-exclusive, now also works with the SGLang backend, making this optimization engine-agnostic. In the same release, `llm-d-inference scheduler` was renamed to `llm-d-router-endpoint-picker`. Remember both names when searching.

### Routing in PD Disaggregation

In a PD disaggregation architecture, routing decisions occur twice: once in the prefill pool and once in the decode pool. The criteria for these two decisions differ.

```bash
python -m sglang_router.launch_router \
  --pd-disaggregation \
  --prefill http://prefill1:30001 9001 \
  --prefill http://prefill2:30002 \
  --decode  http://decode1:30011 \
  --decode  http://decode2:30012 \
  --prefill-policy cache_aware \
  --decode-policy power_of_two
```

- **Prefill Pool:** Its job is to compute prefix KV cache, so a cache hit means computation is skipped. Therefore, it's set to `cache_aware`.
- **Decode Pool:** KV is **transferred** from the prefill. Prefix caching on decode nodes is less meaningful; the key factors are memory availability and batch size. Thus, `power_of_two`, which randomly picks two and chooses the less busy one, is cheap and sufficient.

> The power-of-two-choices is an old load balancing result: simply picking two random options and choosing the less busy one dramatically reduces the maximum load compared to picking just one randomly. It approaches near-optimality without the cost of inspecting all options.

### Trade-Offs

- **Hot Prefix Hotspots**: An eternal challenge for affinity routing; if the load threshold is set too loosely, p99 can spike.
- **Approximate Method Drift**: The router believes the cache exists, but if the engine has already evicted it, the approximate router's judgment can be wrong, especially under high engine memory pressure.
- **Precise Method Cost**: Introduces new infrastructure (like a KV event bus such as NATS, ZMQ) and new points of failure.
- **Router Replica Inconsistency**: If multiple router replicas are deployed for availability, and each holds a different tree, they might send the same prefixes to different workers. Dynamo mitigates this by sharing request assignments between routers.
- **Text vs. Token Mismatch**: If the router calculates matches based on text and the engine caches based on tokens, the boundaries can misalign.

#### Example - Applying Case B

```
Situation: 4×H100, 8B, DP=4. 200 documents repeatedly queried. Operating with round-robin.
Symptom: sglang cache_hit_rate per replica is 25-30%. It was 80% when tested with a single engine.

Diagnosis: Exactly as per the round-robin calculation at the beginning. Working set 1.6 million tokens vs. 410k tokens per replica.
      Round-robin is replicating the working set 4 times.

Action:
  1. Place Model Gateway with cache_aware in front.
  2. Check prompt structure: Is it [system prompt][document][question] order? (Varying values should be sent to the end for prefix sharing)
  3. Check for popular documents where questions are concentrated → If so, set the balance threshold conservatively.

Expectation: Working set per replica reduces to 400k tokens, fitting within capacity → Hit rate recovery.
Verification: Observe hit rate per replica + queue length per replica together. If only hit rate increases and queues are concentrated on one side, it's a hotspot.
```

<br>

## Admission and Flow Control - Accept, Delay, or Drop?

If a router decides "where to go," then a layer is needed before it to decide "whether to accept it now."

If it's put in a queue, will it eventually be processed? When traffic exceeding capacity arrives, most systems just queue it up, assuming it will be processed eventually.

But let's first experience why this is dangerous with some numbers.

In the simplest queuing model, M/M/1, the queue waiting time is as follows:

```
평균 대기 시간 = ρ / (1 - ρ) × 평균 서비스 시간
ρ = 사용률 (도착률 / 처리율)
```

| Utilization ρ | Waiting Time (multiples of service time) |
|-------------:|---------------------------------------:|
| 50% | 1x |
| 70% | 2.3x |
| 80% | 4x |
| 90% | 9x |
| 95% | 19x |
| 99% | 99x |

A continuous batching engine is not M/M/1, but as batches get larger, the throughput itself increases, so the curve shape remains the same.

Once utilization exceeds a certain point, waiting time increases not linearly, but explosively.

The goal of engine optimization was to fully utilize the GPU, but **the moment you try to run the entire cluster at 95%, you're on the knee of this curve.** Even a 5% spike in traffic doubles the TTFT.

### Solution: It's better to deliberately reject quickly than to slow down everything.

Let's recall goodput. It's the throughput counting only requests that meet SLOs.

```
[방치]   100건 전부 받음 → 큐 폭발 → 100건 전부 TTFT 8초 → goodput 0
[차단]   100건 중 10건 즉시 429 → 90건은 TTFT 0.5초    → goodput 90
```

By dropping 10 requests, 90 were saved. The 10 rejected requests were rejected quickly, so clients can retry or go to another region. This is better than waiting 8 seconds for a timeout.

> It's like emergency room triage. If all patients are admitted in order of arrival, a cardiac arrest patient might die behind a common cold patient. In a saturated state, you must decide who goes first and who goes to another hospital.

### Priority Tiers: InferenceObjective

In GAIE, the resource that expresses this is `InferenceObjective`.

It declares how important a request is.

The gateway's AuthPolicy automatically injects two headers based on this authentication information: `x-gateway-inference-fairness-id` for fair queuing by grouping requests from the same authentication source, and `x-gateway-inference-objective` for priority by matching the InferenceObjective resource.

```yaml
# Conceptual example - check apiVersion and fields for each GAIE version
apiVersion: inference.networking.x-k8s.io/v1alpha2
kind: InferenceObjective
metadata:
  name: ide-autocomplete       # Case A: IDE autocomplete - user waits while typing
spec:
  priority: 100
  poolRef:
    name: coder-pool
---
apiVersion: inference.networking.x-k8s.io/v1alpha2
kind: InferenceObjective
metadata:
  name: chat-standard          # Standard chat
spec:
  priority: 0
  poolRef:
    name: coder-pool
---
apiVersion: inference.networking.x-k8s.io/v1alpha2
kind: InferenceObjective
metadata:
  name: nightly-code-review    # Nightly batch review - can be slow, can be dropped, retriable
spec:
  priority: -10
  poolRef:
    name: coder-pool
```

The entity that determines saturation is the Saturation Detector. The key is that it looks at KV stores and queues, not just HTTP connection counts.

EPP's Flow Control layer is designed to manage physical capacity, such as KV cache and queue saturation metrics, rather than HTTP connection counts.

The handling of negative priority requests varies by mode. In the existing admission mode, they are immediately rejected upon saturation. However, if flow control is enabled, negative priority requests are not immediately rejected; they are held in their dedicated queue and wait until dispatched or expired. If limits are still exceeded, lower priority requests are dropped entirely with a 428 or 503, while normal priority requests continue to be queued.

### Preventing Retries

If 1,000 clients immediately retry simultaneously after receiving a 429, saturation worsens. This is called a retry storm.

- Server: Informs when to retry using the `Retry-After` header.
- Client: Staggers retry times with exponential backoff + jitter (random delay).
- Gateway: Temporarily removes failed workers using a circuit breaker.

SGLang Model Gateway also has this layer. It provides retries with jitter, worker-level circuit breakers, token bucket rate limiting with queuing, and background health checks.

### Trade-Offs

- **Priorities do not reserve capacity**: In Dynamo documentation, priority is not like Kubernetes' PriorityClass or GPU preemption or a mandatory admission policy, and it does not reserve capacity for high-priority requests. For higher tiers to always be faster, you must divide pools by tier or allocate separate capacity.
- **Saturation threshold tuning is difficult**: If too low, it discards valid requests; if too high, it only reacts after the system has already passed its knee and failed. This can also lead to oscillations where the state flips near the threshold.
- **Overkill for single workloads**: If there's only a single workload type with uniform SLA requirements, flow control adds unnecessary complexity.

<br>

## Engine Internal Scheduling Policies - Who Goes First

The router has placed requests into the engine. Now, the question remains **which requests to put into the next batch** from the engine's waiting queue.

Continuous batching only dictates that requests are added immediately when space becomes available; who gets added is a separate issue.

The default is mostly FCFS (First-Come, First-Served). While simple and predictable, it has three problems:

1.  **Cache unaware**: Even if a request with a cached prefix is later in the queue, it processes earlier cache-miss requests first.
2.  **Length unaware**: Short requests can wait behind long requests (head-of-line blocking).
3.  **User unaware**: If one user pushes 1000 requests, all other users will have to wait behind them.

Let's look at solutions for each.

### Cache-Aware Policies - LPM and DFS-weight

SGLang has several policies. Cache-aware policies include LPM (Longest Prefix Match), which prioritizes requests with longer cached prefixes, and DFS-Weight, which improves cache reuse through depth-first weighted traversal. Cache-unaware policies include FCFS, LOF (Longest Output First), and random.

**Why does LPM increase hit rates?** Caches are evicted over time. If requests with prefixes currently in the cache are processed first, those prefixes are utilized before eviction. If delayed, other requests might come in and push them out via LRU.

**DFS-weight** processes requests in the order of a depth-first traversal of a Radix Tree. It **processes requests with the same subtree (common prefix) consecutively** to utilize that subtree while it's in the cache.

Here's an important practical lesson. When comparing FCFS and LPM for shared prefix workloads on H100, LPM and FCFS were statistically similar, with less than a 2% difference. The benefit of LPM only appears under cache pressure; if GPU memory is ample, any policy (not just random) performs equally well. -> Don't over-engineer.

**If the cache can hold everything, order doesn't matter.** Order only becomes meaningful when evictions occur. Therefore, when benchmarking policy comparisons, you need to intentionally create cache pressure by lowering `--mem-fraction-static` to see a difference.

TradeOff: LPM reorders requests to increase cache hits but incurs more scheduling overhead. Also, requests not in the cache might continuously be pushed back, leading to starvation.

### Length-Aware - SJF Approximation and Output Length Prediction

A classic solution to minimize average waiting time is **SJF (Shortest Job First)**, which processes shorter jobs first.

The problem is that in LLMs, the job length (number of output tokens) is not known beforehand. Input length is known, but output only ends when EOS is generated.

Therefore, research focuses on predicting output length. The key observation is that while it's hard to predict the exact value, predicting the relative order is sufficient. "Efficient LLM Scheduling by Learning to Rank" demonstrates that relative ranks of output lengths within a request batch can be predicted using learning to rank, and used this to create a scheduler that approximates SJF more closely than existing methods.

```
FCFS:   [긴 요청 4000토큰] [짧은 요청 50] [짧은 요청 30] [짧은 요청 80]
        → 짧은 요청 셋 다 긴 요청이 끝날 때까지 대기 (배치 슬롯이 꽉 찼다면)

SJF 근사: [짧은 30] [짧은 50] [짧은 80] [긴 4000]
        → 평균 대기 시간 크게 감소, 긴 요청은 조금 더 늦게 끝남
```

TradeOff: This time, long requests can suffer starvation. Actual implementations therefore use aging, which increases priority based on waiting time.

There's also the opposite approach. For batch (offline) jobs, total processing time is more important than average waiting time, so LOF (Longest Output First) can be advantageous. This involves starting long jobs first and then filling empty slots with shorter ones.

### User-Aware - Fairness and VTC

In multi-tenant scenarios (Case D), a very realistic problem is that if one client pushes 10,000 batch requests, FCFS will make the other 299 clients wait behind them.

A common response is **RPM limits**, but this has two issues:

1.  The number of requests itself is not the cost.
2.  Even if there's spare capacity, clients hitting the limit cannot use it -> GPU idles while clients are rejected.

OSDI '24's "Fairness in Serving Large Language Models" directly addressed this problem, defining fairness in LLM serving using a cost function that reflects input/output token throughput, and proposed VTC (Virtual Token Counter), a fair scheduler operating on continuous batching.

The idea comes from network fair queuing. Each tenant has a counter that increases as tokens are consumed, and the scheduler picks the next request from the tenant with the smallest counter.

```py
# VTC 개념 의사코드
counter = defaultdict(float)     # 클라이언트별 "지금까지 받은 서비스량"

def on_arrival(req):
    c = req.client
    if not has_queued(c):        # 새로 오거나, 한동안 안 오다 복귀한 클라이언트
        # 안 쓰는 동안 크레딧을 저축했다가 한꺼번에 쓰는 걸 막는다
        active = [counter[k] for k in backlogged_clients()]
        if active:
            counter[c] = max(counter[c], min(active))
    enqueue(req)

def pick_next():
    c = argmin(counter[k] for k in backlogged_clients())   # 가장 덜 받은 사람
    req = dequeue(c)
    counter[c] += W_IN * req.input_tokens                  # 프리필 비용
    return req

def on_decode_step(running_reqs):
    for r in running_reqs:
        counter[r.client] += W_OUT * 1                     # 출력 토큰 하나씩
```

-   **Token-based:** Counting in tokens rather than requests eliminates unfairness caused by differences in request size.
-   **Different weights for input and output**: Decode tokens are more expensive than prefill tokens, and decoding is memory-bound, so weights can be adjusted for fairness.
-   **Work-conserving**: If others aren't using it, one person can use all of it, preventing the GPU from idling like with RPM limits.
-   **Incrementing counter on return**: Without `max(counter[c], min(active))` in the code above, a client returning after an hour of inactivity could monopolize resources for a while due to accumulated unused service. This mechanism prevents that.

There are also guarantees. The paper provides a rigorous proof of VTC's fairness guarantee, showing it's within 2x of the optimal bound. This means the service difference between any two backlogged clients is bounded by a constant.

### Priority and Preemption

Explicit priorities are also supported by the engine. vLLM starts with `--scheduling-policy priority`, and SGLang with `--enable-priority-scheduling`. When memory pressure occurs, which cache blocks to keep first is determined by cache priority settings like SGLang's `--radix-eviction-policy priority`.

Priorities truly become effective with preemption. If a high-priority request arrives and KV memory is full, a running low-priority request must be preempted. Here, the recalculation vs. swap-out trade-off reappears. The preempted request will later need its KV recomputed or fetched from CPU.

### schedule-conservativeness: How pessimistically to view the future

This parameter in SGLang is often overlooked due to its ambiguous name, but it addresses a fundamental problem of the scheduler.

When adding new requests to a batch, the scheduler must estimate how many more tokens the already running requests will generate.

This is because that much KV memory needs to be reserved. However, it's common for requests to have `max_new_tokens=4096` but actually end at 200 tokens with EOS.

```
너무 낙관적 (최대치보다 훨씬 적게 쓸 거라 가정)
  → 새 요청을 많이 넣음 → 실행 중 요청들이 생각보다 길어짐
  → KV 부족 → 실행 중 요청을 강제로 퇴출(retract) → 재계산 낭비

너무 비관적 (max_new_tokens를 다 쓸 거라 가정)
  → 새 요청을 적게 넣음 → 메모리가 남는데 배치가 작음 → 처리량 손해
```

SGLang's tuning guide offers a practical diagnostic: if requests often set a large `max_new_tokens` but frequently end early with EOS or stop strings, lower `--schedule-conservativeness` to around 0.3. If token utilization is very high and "KV cache pool is full. Retract requests" warnings appear frequently, raise it to around 1.3. Occasional retracts, about once per minute, are acceptable.

#### Policy Selection Guide

| Situation                                   | Policy                                  | Reason                                       |
| :------------------------------------------ | :-------------------------------------- | :------------------------------------------- |
| Many shared prefixes and cache pressure (Case B) | LPM or DFS-weight                       | Utilize before eviction                      |
| Many shared prefixes but ample memory       | FCFS is sufficient                      | Order is meaningless (as per empirical data above) |
| Mixed short/long requests, interactive      | SJF approximation + aging               | Average waiting time                         |
| Offline batch                               | LOF                                     | Total processing time                        |
| Multi-tenant (Case D)                       | VTC-like fair scheduling                | Prevent single tenant monopolization         |
| SLA tiers exist                             | Priority + (if needed) pool separation | Priority is not capacity reservation         |

## Autoscaling

### Why Shouldn't We Scale Based on GPU Utilization?

In web services, habits like scaling out HPA at 70% CPU should not be directly applied to GPUs.

1.  Reason 1: **nvidia-smi GPU utilization does not indicate how busy it is.** This value represents the proportion of time during the sampling interval that at least one kernel was running. It's independent of how many SMs are working or how much memory bandwidth is being used. If a memory-bound kernel, like decoding, runs continuously, it will show 100% utilization.
2.  Reason 2: Continuous batching inherently keeps the GPU busy. As pointed out in llm-d, traditional metrics like CPU/GPU utilization are lagging indicators that reflect saturation *after* it has already occurred. In LLM inference, GPU utilization often stays near 100% during active batching, regardless of the actual load, making it an unreliable signal.

```
요청 10개 처리 중   → GPU 사용률 98%
요청 200개 처리 중  → GPU 사용률 99%
요청 200개 + 대기 300개 → GPU 사용률 99%   ← 이미 SLO 붕괴
```

Therefore, it's better to look at the following signals instead:

| Signal | Meaning | Pros | Pitfalls |
|------|------|------|------|
| Queue Depth (Number of pending requests) | Demand that cannot be processed and is accumulating | Direct evidence of saturation, leading indicator | 0 might not mean "idle" (see below) |
| Number of In-flight Requests | Current concurrency | Directly related to Little's Law | Flattens near engine's maximum capacity |
| KV Cache Utilization | Memory pressure | The real resource limiting batch size | Appears high if prefix cache accumulates (check if it includes evictable cache) |
| TTFT / ITL p95 | User experience | Directly linked to SLO | Lagging indicator, high noise |
| GPU Utilization | Ratio of kernel execution time | Easy | Almost useless as explained above |

llm-d recommends the same approach. Effective LLM autoscaling requires proactive and SLO-aware signals that reflect the actual state of the inference system, such as queue depth, number of in-flight requests, or KV cache pressure.

### Pitfall: Replicas Scaling Down to 0 During Streaming Decode

Consider a situation where both queue depth and RPS are 0, but replicas are actively working. These are requests streaming long outputs.

A pod that received a long context request 25 minutes ago and is currently generating the 4812th token out of 8000 has a request rate of 0 and a queue depth of 0. What if HPA scrapes these metrics, sees no demand, and starts the scale-down timer? We must avoid such mistakes.

#### Countermeasures

-   Always include the number of in-flight requests in scale-down decisions.
-   Implement drain logic when terminating pods: block new requests and allow ongoing requests to complete.
-   Set `terminationGracePeriodSeconds` longer than the maximum generation time (the default 30 seconds will cut off long generations).

### Scale Up Fast, Scale Down Slow

LLM replicas take several minutes to spin up (cold start), and shutting them down clears the cache. Therefore, they are tuned asymmetrically.

```yaml
# HPA behavior - scale up aggressively, scale down slowly
behavior:
  scaleUp:
    stabilizationWindowSeconds: 0        # Immediately on signal
    policies:
    - type: Pods
      value: 2                           # 2 at a time
      periodSeconds: 60
  scaleDown:
    stabilizationWindowSeconds: 600      # Only scale down if idle for 10 minutes
    policies:
    - type: Pods
      value: 1
      periodSeconds: 300                 # One at a time every 5 minutes
```

Example of directly attaching to engine metrics with KEDA

```yaml
# Conceptual example - metric names vary by engine/version
apiVersion: keda.sh/v1alpha1
kind: ScaledObject
metadata:
  name: coder-sglang
spec:
  scaleTargetRef:
    name: coder-sglang
  minReplicaCount: 2            # Don't scale down to 0 (due to cold start)
  maxReplicaCount: 8
  triggers:
  - type: prometheus
    metadata:
      serverAddress: http://prometheus:9090
      query: sum(sglang:num_queue_reqs{deployment="coder-sglang"})
      threshold: "8"            # Scale up if pending requests per replica exceed 8
  - type: prometheus
    metadata:
      serverAddress: http://prometheus:9090
      query: avg(sglang:token_usage{deployment="coder-sglang"})
      threshold: "0.85"         # Scale up if KV exceeds 85%
```

### Scaling with PD Separation - Two Dials

When PD separation is implemented, the prefill pool and decode pool must operate independently.

"Increase the proportion of prefill for workloads with long inputs" means automating the adjustment of that ratio.

Tools like Dynamo's Planner perform this role, adjusting the number of prefill and decode workers and predicting future load using ARIMA, Prophet, etc.

The calculation structure clearly reflects the differences in characteristics between prefill and decode.

```
[Number of Prefill Replicas]
  Predicted Load = Number of requests in next interval × Average input length in next interval / Interval length
  Prefill Replicas = ceil(Predicted Load / Profiled prefill throughput per GPU / Number of GPUs per engine)
  → TTFT is determined by prefill (TTFT = Queue wait + Prefill)

[Number of Decode Replicas]
  Interpolate GPU-per-decode throughput from profile data that satisfies ITL target
  → ITL is determined by decode batch size (Batch↑ → Throughput↑, ITL↑)

[Correction Factor]
  prefill_correction = Actual TTFT / Expected TTFT
  decode_correction  = Actual ITL  / Expected ITL
```

Why are correction factors needed?

The numbers the planner uses to determine the number of replicas (how many prefill tokens one GPU processes per second) are pre-profiled values.

However, the profiling environment and the actual operating environment differ. If these differences are left unaddressed, the number of replicas will consistently be incorrect.

The biggest difference is the prefix cache. Profiling is usually done without a cache, but in actual operation, prefill is skipped as much as the cache hits. Let's look at the numbers (hypothetical values):

```
Profiled value: One prefill GPU = 10,000 tokens per second (measured without cache)
Incoming input: 50,000 tokens per second

[Without correction]
  Planner calculation: 50,000 ÷ 10,000 = 5 prefill replicas
  Actual: Cache hit rate 60% → Actual tokens to compute are 20,000 per second
       → 2 would be enough, but 5 are launched → 3 GPUs are idle

[With correction]
  Actual TTFT is about 0.4 times the expected → prefill_correction ≈ 0.4
  Planner reflects 0.4 in expected load: 50,000 × 0.4 = 20,000
  → 20,000 ÷ 10,000 = 2 prefill replicas
```

The opposite can also happen. If actual inputs are longer than during profiling, or if different requests interfere with each other, the actual TTFT might be worse than expected. In such cases, the correction factor becomes greater than 1, and the planner launches more replicas. Without correction, believing there's enough capacity, situations where SLOs could be violated can be prevented.

In summary, the correction factor is a mechanism that measures **how much the profiled values deviate from reality** through actual measurements and reflects this in replica calculations.

Without this, the planner wastes GPUs when the cache hits well and remains unaware of capacity shortages when the environment degrades.

### Heterogeneous Hardware - Fill Cheaper GPUs First

If the same model can be deployed on H100, A100, and L4, indiscriminately scaling up all pools simultaneously is a waste of time. llm-d's WVA (Workload Variant Autoscaler) addresses this by prioritizing adding capacity to the cheapest variant and removing it from the most expensive variant first, optimizing without violating latency SLOs.

### Headroom Calculation - Can the Autoscaler Keep Up?

Autoscalers are reactive. They launch replicas after observing an increase in demand.

The demand that arrives in the interim must be handled by existing replicas. Let's calculate this.

Assuming we're serving 1,000 engineers, 9 AM when they start work is peak time.

```
[1] Traffic Ramp
    08:50 → 09:05 (over 15 minutes) 0.5 req/s → 10 req/s
    Increase rate ≈ 9.5 / 15 ≈ 0.63 req/s per minute

[2] Time until new capacity enters service (broken down in cold start section)
    Detection: Metric collection 15s + HPA cycle 15s + Decision ≈ 1 min
    Cold Start: Node 3 min + Image 1 min + Weights 1 min + Initialization 1 min ≈ 6 min
    Total ≈ 7 min

[3] Demand increase during those 7 minutes
    0.63 × 7 ≈ 4.4 req/s

[4] Amount one replica can handle within SLO (assumption, value to be measured by benchmark)
    ≈ 3 req/s
    → At the point of peak entry, approximately 2 replicas' worth of headroom should already be available
```

-   **Predictable peaks cannot be caught reactively.** For daily peaks like morning commute times, pre-scale with cron (KEDA cron trigger).
-   **Cold start time determines headroom.** Reducing cold start from 7 minutes to 2 minutes reduces the required spare replicas to less than 1/3, which will be elaborated below.

<br>

## Cold Start - Until a New Pod Emits Its First Token


"A pod takes a long time to start" seems like a single problem, but it actually consists of five stages.

```
[1] 노드 프로비저닝      클라우드에서 GPU 노드 확보        수십 초 ~ 수 분 (재고 없으면 실패)
[2] 컨테이너 이미지 풀    vLLM/SGLang 이미지는 수~십수 GB   수십 초 ~ 수 분
[3] 가중치 로드          스토리지 → (CPU) → GPU           모델 크기 / 실효 대역폭
[4] 엔진 초기화          torch.compile, CUDA 그래프 캡처,  수십 초
                        커널 오토튜닝, KV 풀 할당
[5] 워밍업 & 헬스 체크    첫 요청은 비정상적으로 느림        수 초
```
> The ranges above are for order-of-magnitude sense; you should measure them for each stage in your own environment. Even just good log timestamps will immediately show where the bottleneck is.

### Weight Loading

70B BF16 = 140GB

```
일반 클라우드 블록 스토리지 (~1 GB/s)   → 140초 ≈ 2분 20초
로컬 NVMe (~7 GB/s)                 → 20초
100Gbps 네트워크 (이론 12.5 GB/s)      → 11초
```

However, in reality, theoretical values are not achieved. The default loader reads files sequentially, downloads them to local disk once, then reads them again, and involves multiple copies through the page cache, so it cannot fully utilize the bandwidth, resulting in slower performance.

### Accelerating Weight Loading - Run:ai Model Streamer

The core of the solution is to **read in parallel, bypass the local disk, and send directly to the GPU.**

Run:ai Model Streamer is a prime example; according to NVIDIA's description, it's an open-source Python SDK that concurrently reads model weights from storage and streams them directly into GPU memory.

- On Azure, 8 x H100 VM with Llama-3.1-8B (approx. 15GiB): Run:ai Streamer 3.61 seconds vs. default vLLM loader 15.48 seconds, an improvement of about 4.3x.
- On TPU with Qwen3-Coder-480B FP8: The streamer finished loading in under 280 seconds, while the default loader caching to external storage took over 630 seconds. The memory effect is also significant: the default loader requires more than double the model size in host memory, but the streamer eliminates local file caching, capping peak memory at the model size plus a 40GB buffer.

```bash
# vLLM - S3/GCS/Azure에서 바로 스트리밍
vllm serve s3://my-bucket/models/llama-3.1-70b \
  --load-format runai_streamer \
  --tensor-parallel-size 8 \
  --model-loader-extra-config '{"distributed": true, "concurrency": 32}'
```

Note: The streamer requires SafeTensors weights, so you must check if your model has `.safetensors` files and not just `.bin` files. This is because the two formats have different structures. SGLang also supports the same streamer.

```
.safetensors
  [헤더: 텐서 이름 → dtype, shape, 파일 안 바이트 위치(offset)]
  [텐서 데이터가 그냥 이어 붙어 있음]
  → 헤더만 읽으면 "어느 텐서가 몇 번째 바이트부터 몇 바이트"인지 전부 안다
  → 파일을 여러 구간으로 쪼개 동시에 읽고, 읽는 족족 GPU로 보낼 수 있다

.bin (PyTorch pickle)
  [Python 객체를 직렬화한 것]
  → torch.load로 역직렬화를 끝까지 해야 텐서가 어디 있는지 알 수 있다
  → 구간을 나눠 병렬로 읽는 게 불가능, 스트리밍의 전제가 깨진다
```

The streamer's speed comes from reading in parallel and operating on the GPU, which is possible with the SafeTensors format. Pickle also has security issues as it can execute arbitrary code during loading, so SafeTensors is the standard nowadays.

Most recent public models are distributed in SafeTensors format, but older models or fine-tuned checkpoints saved with `torch.save` internally are often in `.bin` format. If you enable `--load-format-runai_streamer` with such models, loading will fail. It's advisable to include a SafeTensors conversion step in your deployment pipeline beforehand. SGLang also supports the same streamer, and the conditions are identical.

### Reducing Engine Initialization

Even after all weights are loaded, it's not over. Let's recall CUDA graphs.

CUDA graphs need to be fixed in shape, so multiple graphs are pre-captured for each batch size, which then impacts startup time and GPU memory.

- **Persist torch.compile cache**: If you save compilation results to a volume, they can be reused from the second startup onwards.
- **Limit the range of batch sizes to capture:** Set `--cuda-graph-max-bs` to your actual maximum batch size to avoid spending time capturing large batches that are never used.
- **Lazy image loading:** Start with necessary layers before the entire container image is downloaded, e.g., SOCI, stargz families.

### Sleep Mode: Not Shutting Down at All

If cold start is a bottleneck, not shutting down at all is an option. vLLM's Sleep Mode doesn't kill the process; it only frees up GPU memory.

Level 1 moves model weights to CPU memory and discards the KV cache; wake-up times range from 0.1-0.8 seconds for small models to 3-6 seconds for large models. Level 2 discards both weights and the KV cache, leaving only buffers like RoPE scaling tensors in CPU.

The effect is that waking up a sleeping model is 18-20 times faster than launching a new vLLM instance. This is because the process remains alive, preserving the CUDA context, compilation results, and captured graphs. It completely skips four stages of the cold start.

```bash
export VLLM_SERVER_DEV_MODE=1
vllm serve Qwen/Qwen3-8B --enable-sleep-mode --port 8001

curl -X POST 'localhost:8001/sleep?level=1'   # 가중치 → CPU, KV 버림
curl -X POST 'localhost:8001/wake_up'         # 다시 GPU로
```

- **When running multiple models alternately on a single GPU**: This is good for time-sharing several low-traffic models on one GPU.
- **RL training:** When alternating between training and rollouts (inference) on the same GPU, you can put the inference engine to sleep and hand over memory to training.

Endpoints like `/sleep` and `/wake_up` require `VLLM_SERVER_DEV_MODE=1` and should only be exposed on trusted networks. If exposed externally, anyone can put the server to sleep.

### Scale-to-zero is not cheap
 
| Strategy | Idle Cost | First Request Latency | Suitable For |
|------|----------|------------|----------|
| Maintain minimum replicas always | Constant GPU billing | None | Conversational, strict SLO (Case A) |
| Sleep (Level 1) | GPU occupied, weights in CPU RAM | Seconds | Time-sharing multiple models |
| Scale-to-zero + fast loader | 0 | Minutes | Internal tools, batch, low-frequency models |
| Scale-to-zero + default loader | 0 | Several minutes ~ 10 minutes | Virtually unusable for conversational apps |

> Decision criteria: Idle time x GPU cost per hour vs. cost of lost requests and user churn during cold start. In a conversational service, a first user waiting 5 minutes can often be more expensive than several hours of GPU time.

<br>

## Multi-LoRA Serving - 300 Models on a Few GPUs

Let's say 300 clients each want a model fine-tuned with their own data.

For full fine-tuning, each client would have a full set of weights, meaning even for an 8B model, 16GB x 300 = 4.8 TB would be needed. For a 70B model, it would be 42TB. We can't allocate a separate GPU for each client, and most clients have sporadic traffic, leaving GPUs mostly idle.

Training with LoRA changes the situation. The weight matrix W is kept as is, and only two small matrices, B and A, are trained.

```
W' = W + B·A
W: d_out × d_in       (fixed, shared by all)
A: r × d_in           (per client)
B: d_out × r          (per client)
r: rank, usually 8~64    (much smaller than d)
```

### Adapter Size - Let's Calculate It Ourselves

Let's assume LoRA is applied to all 7 modules in Llama-3.1-8B (hidden 4096, intermediate 14336, KV dimension 1024, 32 layers). The number of LoRA parameters for a single matrix is `r x (d_in + d_out)`.

```
q_proj:    4096 → 4096    r × 8,192
k_proj:    4096 → 1024    r × 5,120
v_proj:    4096 → 1024    r × 5,120
o_proj:    4096 → 4096    r × 8,192
gate_proj: 4096 → 14336   r × 18,432
up_proj:   4096 → 14336   r × 18,432
down_proj: 14336 → 4096   r × 18,432
-----------------------------------
Sum per layer                  r × 81,920
× 32 layers                    r × 2,621,440

r = 16 → approx. 42 million parameters × 2 bytes ≈ 84MB
r = 64 → approx. 168 million parameters × 2 bytes ≈ 335MB
```

An r=16 adapter is about 0.5% of the 16GB base model. Even with 300 of them, it's only 25GB, which fits comfortably in CPU RAM.

#### Why Merging Is Not an Option

If we add LoRA to W to create W', the inference cost becomes the same as the base model. However, this means each client has a different model, preventing us from batching requests from different clients together. All batching benefits would be lost.

Instead of merging, we can just compute it like this:

```
y = x·W + (x·A)·B
    ─────   ─────────
    Common     Per client

Even if 64 requests in a batch are from different clients,
  The x·W part performs matrix multiplication for all 64 at once (reads weights once and applies to 64).
  Only the (x·A)·B part needs to use different A and B for each request.
```

The problem is that if we run 64 different small matrix multiplications for the second term as separate kernels for each request, the kernel launch overhead becomes prohibitive.

#### Kernel Launch Overhead

A function running on the GPU is called a kernel.

Even a single matrix multiplication is a kernel. To run a kernel, the CPU must send a command to the GPU each time, saying 'execute this function with this data'. There is a **fixed overhead time, unrelated to the actual computation**, to prepare and send this command, and for the GPU to receive and start it. This is typically a few µs per kernel (varies by driver and framework).

For large matrix multiplications, the computation itself takes hundreds of µs to ms, so a few µs is negligible. However, LoRA's x·A is extremely small, meaning there's too much back-and-forth.

```
x·A  =  (1 × 4096) × (4096 × 16)
     → Computation is so small that the GPU finishes in 1-2µs.
     → But launch time also takes several µs.
     → The time spent "calling to start work" is longer than the actual work time.
```

> It's like a delivery driver returning to the logistics center for the next waybill after each delivery. If the delivery itself takes 10 seconds but the round trip takes 5 minutes, they'd spend all day just going back and forth, which is inefficient. The principle is that it's better to load multiple items onto one truck at once.

Let's roughly calculate how much this adds up to:

```
64 requests × 2 LoRA matrix multiplications (A, B) × 7 modules × 32 layers
  = Approx. 28,000 kernel calls per decode step.
Assuming a launch cost of approx. 5µs per kernel,
  = 28,000 × 5µs ≈ 140ms

A single decode step should typically complete within tens of ms.
→ LoRA launch costs alone make the step several times slower.
```

> This problem arises from not utilizing batching properly.

Therefore, running each request separately is not the answer; a method is needed to group LoRA computations from multiple requests.

### SGMV - Heterogeneous Adapters in a Single Kernel

Punica solved this problem. It introduced a CUDA kernel called SGMD (Segmented Gather Matrix-Vector Multiplication) that groups requests by adapter ID and executes batched GEMM for each group.

```
Requests in batch: [Client A, Client A, Client C, Client B, Client B, Client B, Client A]
                      ↓ Sorted by adapter (segment)
              [A A A | B B B | C]
                      ↓ Within a single kernel, gather and multiply the corresponding A, B for each segment.
              1 kernel launch
```

The heterogeneity cost is smaller than expected. For a batch of 64, when all adapters are different, SGMV takes 116µs. When all are the same, it takes 37µs, about a 3x difference, but this is dwarfed by the base model's attention computation.

Punica achieved a 12x throughput improvement compared to existing multi-LoRA methods and was within 8-12% of vLLM running only the base model (1,044 vs 1,140 tokens/sec for Llama-7B on A100).

S-LoRA added memory management. It keeps all adapters in main memory and only brings adapters used by currently executing requests to the GPU.

It also extended the PagedAttention idea to adapters, allowing KV cache and adapter weights to share the **same page pool** (Unified Paging). As a result, it could serve 2,000 adapters concurrently on a single A100 80GB, whereas vLLM with simple packing would run OOM with more than 5 adapters.

These two research efforts are the foundation of current engine implementations for multi-LoRA. SGLang adopts techniques from S-LoRA and Punica to efficiently support different LoRA adapters for each sequence within a batch.

### Engine Configuration

**vLLM:** GPU slots and a 2-tier CPU cache

```bash
VLLM_ALLOW_RUNTIME_LORA_UPDATING=True \
vllm serve meta-llama/Llama-3.1-8B-Instruct \
  --enable-lora \
  --max-loras 16 \          # Number of adapters concurrently loaded on GPU (= number that can be mixed in one batch)
  --max-lora-rank 16 \      # This value pre-allocates slots (see pitfall below)
  --max-cpu-loras 128       # CPU LRU cache
```

```bash
# Add adapter at runtime (without restart)
curl -X POST localhost:8000/v1/load_lora_adapter \
  -H "Content-Type: application/json" \
  -d '{"lora_name": "tenant-042", "lora_path": "/adapters/tenant-042"}'
```

```bash
python -m sglang.launch_server \
  --model-path meta-llama/Llama-3.1-8B-Instruct \
  --enable-lora \
  --lora-backend csgmv \
  --max-loras-per-batch 16 \
  --max-lora-rank 16 \
  --lora-target-modules all \
  --max-loaded-loras 64
```

- `max_loras_per_batch` is the maximum number of adapters per batch, which affects the amount of GPU memory reserved for multi-LoRA. If memory is insufficient, it should be set lower; the default value is 8.
- Backend: `csgmv` is a Chunked SGMV backend optimized for high concurrency, showing 20-80% latency improvement over the default Triton backend in benchmarks.
- Dynamic loading: Adapters can be loaded and unloaded at runtime using the `/load_lora_adapter` and `/unload_lora_adapter` APIs. In this case, it is recommended to specify `--max-lora-rank` and `--lora-target-modules` at startup.
- Fixed pinning: Frequently used adapters can be pinned to the GPU, but pinning too many can stall requests that are not pinned. Therefore, the number of pinned adapters is limited to `max-loras-per-batch - 1`.

### Caveats

#### 1. `max-lora-rank` is pre-allocated to the maximum value

Slots are not sized per adapter but are allocated based on `--max-lora-rank`. Even if only one out of 300 adapters is r=64 and the rest are r=16, all slots might be allocated for r=64.

```
max-loras 8 × r=16 slots ≈ 8 × 84MB  ≈ 0.7GB
max-loras 8 × r=64 slots ≈ 8 × 335MB ≈ 2.7GB
Difference 2GB ÷ 128KiB/token ≈ lose 15,000 KV cache tokens
```

If one client's value is large, it eats into the number of other concurrent users. Therefore, standardizing the adapter rank policy during the training phase is optimal for serving. The point that model design and serving design are a single decision is reiterated with LoRA.

#### 2. Prefix Cache is Fragmented Per Adapter

This is easy to miss: even with the same system prompt, if the adapters are different, the KV cache will be different.

If LoRA is applied to `k_proj` and `v_proj`, this is obvious. Even if it's only applied to `q/o`, the output (hidden state) of the preceding layers changes, thus altering the KV cache of subsequent layers.

```
300 clients × same 3K system prompt
  For base model: 1 set of 3K token KV → shared by all
  For 300 LoRAs: 300 sets of 3K token KV → Adapter ID must be included in cache key
```

#### 3. Adapter-Aware Routing is Required

Following the same logic as cache-aware routing, if 300 adapters are randomly distributed across 4 replicas, all replicas would have to load all 300, and 4 copies of the prefix cache would be created for each adapter. To save on adapter loading and caching, the same adapter should be routed to a small number of replicas.

#### 4. Adapter Swapping Causes TTFT p99 Spikes

If a request comes for an adapter not in a GPU slot, it must be fetched from CPU, disk, or remote storage, causing latency spikes.

#### 5. Fewer Slots Shrink Batches

If there are 20 client requests mixed in the waiting queue but only 8 slots, the scheduler can only put requests for those 8 adapters into a single batch. The rest wait. Even if memory remains, the batch size won't increase. SGLang's `--enable-lora-overlap-loading` with asynchronous loading mitigates this, but at the cost of fewer opportunities to create prefill batches that group multiple adapters, as adapters are loaded onto the GPU at different times.
