# SGLang Serving Casebook

> Reference Version: SGLang 0.5.13 / August 2026
> The benchmark figures and logs in these notes are simulated values and may differ from actual measurements.
> They are derived by reverse-calculating the scaling factors and hardware specifications from public benchmarks and are not absolute values. **Use them only to understand the changes, directions, and magnitudes of tuning before and after.**

### Cases Covered

1.  **In-house Coding Assistant**, 8xH200, TTFT p95 constraint, key techniques: chunked prefill, speculative decoding, replica splitting
2.  **Document QA / RAG**, 4xH200, prefix reuse constraint, key techniques: cache-aware routing, HiCache
3.  **Nightly Batch Document Processing**, 2xH100, throughput-dominated constraint, model downsizing, batch maximization
4.  **Structured Output API**, 2xH100, schema compliance rate-dominated constraint, XGrammer, jump-forward
5.  **Large-scale MoE Serving**, 16 Node x H200, cost-dominated constraint, EP, DP Attention, PD Disaggregation

Each case proceeds in the same order.

```
Requirements and SLO → Traffic Profile Measurement → Model Selection → Capacity Calculation
    → Initial Startup → Benchmark → Diagnosis → Tuning Round Iteration → Final Configuration
```

## Common Tools

### Workload Profile - Metrics to Secure Before Tuning

-   **Input Length Distribution (Median, p95):** Determines prefill burden and chunk size.
-   **Output Length Distribution:** Determines decode burden and KV cache residency time.
-   **Prefix Sharing Rate:** Determines the magnitude of RadixAttention's benefit.
-   **Peak Concurrency:** Determines speculative decoding's profitability and batch parameters.

If production logs are available, you can extract them with the following script:

```py
#!/usr/bin/env python3
"""profile_workload.py — Extract workload profile from logs"""
import json, statistics
from transformers import AutoTokenizer

def profile(log_path, model_path):
    tok = AutoTokenizer.from_pretrained(model_path)
    ins, outs, seqs = [], [], []

    with open(log_path) as f:
        for line in f:
            r = json.loads(line)
            t_in = tok.encode(r["prompt"])
            ins.append(len(t_in))
            outs.append(len(tok.encode(r["completion"])))
            seqs.append(t_in)

    # Prefix sharing rate: sum of common prefix lengths of adjacent pairs after sorting / total tokens
    seqs.sort()
    shared = 0
    for a, b in zip(seqs, seqs[1:]):
        c = 0
        for x, y in zip(a, b):
            if x != y: break
            c += 1
        shared += c

    return {
        "입력 중앙값": statistics.median(ins),
        "입력 p95":    sorted(ins)[int(len(ins) * 0.95)],
        "출력 중앙값": statistics.median(outs),
        "출력 p95":    sorted(outs)[int(len(outs) * 0.95)],
        "접두사 공유율": round(shared / sum(ins), 3),
    }

if __name__ == "__main__":
    import sys
    for k, v in profile(sys.argv[1], sys.argv[2]).items():
        print(f"{k:12s}: {v}")
```

Prefix Sharing Rate Criteria
```
0 ~ 0.20   RadixAttention offers almost no benefit. No comparative advantage over vLLM.
0.20 ~ 0.60  Worth benchmarking both.
0.60 이상    SGLang has a clear advantage. Expect 75-95% cache hit rate.
```

### Benchmark Execution

```bash
# Online benchmark simulating actual traffic distribution
python -m sglang.bench_serving \
  --backend sglang \
  --host 127.0.0.1 --port 30000 \
  --dataset-name $random \
  --random-input-len 2048 \
  --random-output-len 512 \
  --random-range-ratio 0.5 \
  --num-prompts 500 \
  --max-concurrency 25 \
  --warmup-requests 30 \
  --output-file result.jsonl
```

-   `--max-concurrency`: Sets the upper limit for concurrent requests to match the peak, preventing latency metrics from becoming meaningless due to infinite concurrency.
-   `--random-range-ratio`: Randomizes lengths with a uniform distribution, resolving the discrepancy between fixed-length measurements and reality.
-   `--warmup-requests`: Excludes CUDA graph capture or JIT compilation from the initial requests to prevent skewing the average and introducing outliers.
-   `--dataset-name sharegpt`: Uses actual conversation distribution; otherwise, prefix sharing characteristics are not reflected.

To measure cold start performance, launch with `--disable-radix-cache` or clear the cache before each run.

```bash
curl -X POST http://127.0.0.1:30000/flush_cache
```

#### Output

```
============ Serving Benchmark Result ============
Backend:                                 sglang
Traffic request rate:                    inf
Max request concurrency:                 25
Successful requests:                     500
Benchmark duration (s):                  128.43
Total input tokens:                      1048576
Total generated tokens:                  262144
Request throughput (req/s):              3.89
Input token throughput (tok/s):          8164.21
Output token throughput (tok/s):         2041.05
Total token throughput (tok/s):          10205.26
Concurrency:                             24.81
----------------End-to-End Latency----------------
Mean E2E Latency (ms):                   6382.14
Median E2E Latency (ms):                 5904.33
---------------Time to First Token----------------
Mean TTFT (ms):                          842.17
Median TTFT (ms):                        612.44
P99 TTFT (ms):                           4380.92
---------------Inter-Token Latency----------------
Mean ITL (ms):                           31.24
Median ITL (ms):                         28.41
P95 ITL (ms):                            72.18
P99 ITL (ms):                            148.63
Max ITL (ms):                            982.51
==================================================
```

Establishing a reading order can speed up diagnosis.

```
1. Concurrency vs Max request concurrency (actual concurrent requests being processed vs. allowed maximum)
  -> If Concurrency is higher than Max request concurrency, it means the server cannot handle all requests (queue or memory issue).
  -> If Concurrency is low compared to max, it means system resources are ample and load is light.

2. Median TTFT vs p99 TTFT
  -> If the ratio is 3x or more, suspect blocking by long prompts or chunked prefill.

3. Median ITL vs P99 ITL / Max ITL
  -> If Max ITL is several hundred ms, the scheduler is stalled at a specific step; preemption or prefill insertion is the cause.

4. Output token throughput
  -> Cost metric, baseline for comparison between tuning rounds.
```

### Server Metrics

```bash
# Prometheus endpoint (--enable-metrics required)
curl -s http://127.0.0.1:30000/metrics | grep -E \
  'sglang_(num_running_reqs|num_queue_reqs|token_usage|cache_hit_rate|gen_throughput)'
```

```conf
sglang_num_running_reqs{model="..."} 24.0
sglang_num_queue_reqs{model="..."} 0.0
sglang_token_usage{model="..."} 0.61
sglang_cache_hit_rate{model="..."} 0.34
sglang_gen_throughput{model="..."} 2041.05
```

-   `num_queue_reqs` being close to 0 is normal; if it deviates, it means requests are queuing up, indicating insufficient capacity or a scheduling limit.
-   `token_usage` normal range is 0.5~0.85; approaching 1.0 signifies KV cache saturation -> preemption.
-   `cache_hit_rate`: Normal range depends on the workload; if it's low compared to the sharing rate, it indicates a prompt design issue (poor caching, variable values at the beginning).

If `token_usage` is close to 1.0 and `num_queue_reqs` is also high, KV cache shortage is the bottleneck. If GPU utilization is low but the queue is long, it's a scheduling issue. Distinguishing these two is the starting point for diagnosis.

### Tuning Order

Let's follow the order of effect size.

```
1. Model Selection          Minimum model suitable for task difficulty          Up to several times
2. Parallelization Batch    TP size and replica count splitting                 2-3x
3. Chunked Prefill          Stabilizes p99 TTFT                                 Latency stabilization
4. Cache Strategy           RadixAttention, routing, HiCache                    Depends on sharing rate
5. Quantization             FP8 weights + FP8 KV                                1.3-1.5x
6. Batch Parameters         Select point on SLO curve                           10-30%
7. Speculative Decoding     Only for low concurrency                            1.5-2x
8. PD Disaggregation / Large-scale EP  Only for tens of GPUs or more            Up to 5x
```

<br>

## In-house Coding Assistant

### Requirements

-   **Users:** 200 engineers
-   **Peak Concurrent Requests:** 25
-   **Interface:** IDE Plugin, Streaming
-   **Hardware:** 8xH200 SXM (141GB, 4.8TB/s) single node
-   **SLO:** **TTFT p95 1,000ms**, **ITL Median < 50ms**

### Profiling

Current `profile_workload.py` execution results:

```
입력 중앙값  : 2100
입력 p95     : 28000
출력 중앙값  : 400
출력 p95     : 1800
접두사 공유율 : 0.35
```

Input p95 is 13 times the median, indicating a bottleneck. This is due to the user pattern of pasting entire files. Without chunked prefill, short requests get stuck behind long ones.

A sharing rate of 0.35 is borderline. The system prompt and internal coding conventions (3,000 tokens) are fully shared, while the rest vary. Introducing agent-based tool calls would increase this value, justifying the choice of SGLang.

### Model Selection

Let's compare the candidate set.

**Candidate 1. Qwen3.6-27B, dense, FP8 weights 27GB, coding performance SWE-bench Verified 77.2, batch 1 decode fast:**

**Candidate 2. Qwen3.6-35B-A3B: MoE (active 3B), FP8 weights 35GB, lower coding performance than 27B. Batch 1 decode very fast:**

**Candidate 3. GLM-4.5-Ari, MoE 106B/12B, FP8 weights 106GB, higher coding performance than 27B, batch 1 decode moderate:**

#### Qwen3.6-27B Selected

Concurrency 25 is much smaller than the H200 threshold batch size (206). This is a memory-bound region, so performance is determined by the **number of weight bytes to read**.

The active parameter advantage of MoE appears with large batches, but here the batch is small, so the advantage is minor. Instead, it incurs the burden of loading all total parameters into memory.

GLM-4-5-Air has higher coding performance, but even with FP8, it's 106GB, reducing KV cache headroom. Its slower decode makes it difficult to meet the ITL SLO compared to 27B.

### Capacity Calculation

Calculate KV cache for Qwen3.6-27B attention configuration (48 layers, 8 KV heads, 128 head dimensions).

```
KV per token (BF16) = 2 × 48 × 8 × 128 × 2 bytes = 196,608 bytes ≈ 192 KB
KV per token (FP8)  = 96 KB

Average context 3,000 tokens × 25 concurrent requests
  BF16: 3,000 × 192 KB × 25 ≈ 14.4 GB
  FP8 :                       ≈  7.2 GB

p95 scenario (28,000 tokens × 25 requests)
  BF16: 28,000 × 192 KB × 25 ≈ 134 GB
  FP8 :                       ≈  67 GB
```

With 8xH200 = 1,128GB, subtracting 27GB for weights leaves ample headroom, so memory is not a constraint.

The constraint in this case is latency, not capacity, so tuning should focus on latency.

### Round 0 - Initial Startup

```bash
python -m sgalng.launch_server \
    --model-path Qwen/Qwen3.6-27B \
    --tp-size 8 \
    --host 0.0.0.0 --port 30000 \
    --enable-metrics
```

```
python -m sglang.bench_serving --backend sglang --port 30000 \
  --dataset-name random --random-input-len 2100 --random-output-len 400 \
  --random-range-ratio 0.1 --num-prompts 500 --max-concurrency 25 \
  --warmup-requests 30
```

```
============ Serving Benchmark Result ============
Max request concurrency:                 25
Successful requests:                     500
Benchmark duration (s):                  128.43
Request throughput (req/s):              3.89
Output token throughput (tok/s):         1554.21
Total token throughput (tok/s):          9721.44
Concurrency:                             24.81
---------------Time to First Token----------------
Mean TTFT (ms):                          842.17
Median TTFT (ms):                        612.44
P99 TTFT (ms):                           4380.92
---------------Inter-Token Latency----------------
Mean ITL (ms):                           31.24
Median ITL (ms):                         28.41
P99 ITL (ms):                            148.63
Max ITL (ms):                            982.51
==================================================
```

-   **TTFT p95:** Estimated around 3,200ms (p99 is 4381), which is outrageously above the < 1000ms target. ❌
-   **ITL Median:** 28.41ms, which is below 50ms, so it passes. ✅

P99 TTFT is 7 times the median, and Max ITL is 982ms, indicating that a request being decoded was stalled for nearly 1 second at one step. The cause is a long prompt prefill occupying the entire batch.

### Round 1 - Chunked Prefill

```diff
python -m sglang.launch_server \
    --model-path Qwen/Qwen3.6-27B \
    --tp-size 8 \
+   --chunked-prefill-size 4096 \ # added
    --host 0.0.0.0 --port 30000 \
    --enable-metrics
```

```
Output token throughput (tok/s):         1498.63     (-3.6%)
---------------Time to First Token----------------
Median TTFT (ms):                        648.11      (+5.8%)
P99 TTFT (ms):                           1621.05     (-63.0%)
---------------Inter-Token Latency----------------
Median ITL (ms):                         27.92
P99 ITL (ms):                            61.44       (-58.7%)
Max ITL (ms):                            118.27      (-88.0%)
```

Interpreting the changes, TTFT dropped from 4,381ms to 1,621ms, and Max ITL went from 982ms to 118ms.

The 28,000-token prefill was split into seven 4,096-token chunks and inserted between decode batches, eliminating the period where one request monopolized the GPU.

In exchange, throughput dropped by 3.6%, and median TTFT slightly increased. This is because splitting the prefill results in smaller matrix operations for each chunk, slightly reducing GPU efficiency.

**This is a trade-off that reduced tail latency by 63% at the cost of 3.6% throughput.** For this case, where latency is the SLO, it's a favorable decision.

**+Also tried reducing chunk size to 2,048.**

```
--chunked-prefill-size 2048
P99 TTFT (ms):                           1584.22     (-2.3% vs 4096)
Output token throughput (tok/s):         1402.11     (-6.4% vs 4096)
```

Tail improvement is only about 2%, but throughput suffers 6%. Diminishing returns begin at 4096, so let's stop here.

### Round 2 - FP8 Quantization

```diff
python -m sglang.launch_server \
    --model-path Qwen/Qwen3.6-27B \
    --tp-size 8 \
    --chunked-prefill-size 4096 \
+   --quantization fp8 \
+   --kv-cache-dtype fp8_e5m2 \
+   --mem-fraction-static 0.90 \
    --host 0.0.0.0 --port 30000 \
    --enable-metrics
```

```
Output token throughput (tok/s):         1971.44     (+31.5%)
---------------Time to First Token----------------
Median TTFT (ms):                        482.30      (-25.6%)
P99 TTFT (ms):                           1184.67     (-27.0%)
---------------Inter-Token Latency----------------
Median ITL (ms):                         21.08       (-24.5%)
P99 ITL (ms):                            48.92
```

All metrics improved simultaneously. This is because this region is memory-bound.

Weights decreased from 54GB (BF16) to 27GB (FP8), halving the bytes read at each decode step, and decode time was almost halved accordingly.

**Quality verification must be done concurrently.** Quantization loss might not be visible in general conversations but can appear in code.

```bash
# Compare with BF16 using 50 internal code cases
python eval_codegen.py --baseline http://bf16-server:30000 \
                       --candidate http://fp8-server:30000 \
                       --cases internal_50.jsonl --blind
```

```
Blind evaluation of 50 cases
  BF16 superior  : 6 cases
  FP8 superior   : 5 cases
  Equivalent     : 39 cases
  Compile failed : BF16 1 case / FP8 1 case
→ No significant difference. FP8 adopted.
```

### Round 3 - Speculative Decoding

Concurrency 25 is well below the threshold batch size of 206, meaning there are still computational resources available.

Speculative decoding utilizes this spare capacity.

```diff
+   --speculative-algorithm EAGLE3 \
+   --speculative-draft-model-path Qwen/Qwen3.6-27B-eagle3 \
+   --speculative-num-steps 5 \
+   --speculative-eagle-topk 8 \
+   --speculative-num-draft-tokens 6 \
```

```
Output token throughput (tok/s):         3284.90     (+66.6%)
---------------Time to First Token----------------
Median TTFT (ms):                        511.44      (+6.0%)
P99 TTFT (ms):                           1247.02     (+5.3%)
---------------Inter-Token Latency----------------
Median ITL (ms):                         12.63       (-40.1%)
P99 ITL (ms):                            34.18
```

Let's check the acceptance rate in the server logs.

```
[2026-08-07 14:22:11] Speculative decoding stats:
  accept_length_mean: 3.82 / 6
  accept_rate: 0.637
  draft_overhead_ratio: 0.118
```

Interpreting the changes, generating one token now involves reading weights equivalent to validating 6 tokens.

An average of 3.82 out of 6 proposed by the draft are accepted, making the effective tokens per step 3.8 times higher. Even after subtracting the 11.8% cost of running the draft model, ITL dropped by 40%.

TTFT increased by 5-6%. This is because the draft model also performs prefill.

This is acceptable as it's within the SLO margin.

**Let's measure the break-even point by increasing concurrency.**

```
Concurrency   Speculative OFF (tok/s)   Speculative ON (tok/s)   Ratio
  8          712               1584          2.22×
 25         1971               3285          1.67×
 64         3844               4912          1.28×
128         5901               5734          0.97×   ← Reversal
```

From concurrency 128 onwards, speculative decoding becomes detrimental.

This is because as it approaches compute-bound, running the draft model takes away resources from honest computation.

This means that if peak concurrency increases, this setting needs to be re-evaluated and should be an operational metric to monitor.

### Round 4 - TP8 Single Instance vs TP2 x 4 Replicas

Up to this point, 8 GPUs were grouped into a single instance. This configuration benefits individual latency by distributing the computation of a single request across 8 GPUs, but all-reduce communication occurs at each layer, and GPUs are idle when batches are small.

Let's split the 8 GPUs into four TP2 instances and group them with a router for comparison.

```bash
# 4 workers (GPU 0-1, 2-3, 4-5, 6-7)
for i in 0 1 2 3; do
  CUDA_VISIBLE_DEVICES=$((i*2)),$((i*2+1)) \
  python -m sglang.launch_server \
    --model-path Qwen/Qwen3.6-27B \
    --tp-size 2 \
    --chunked-prefill-size 4096 \
    --quantization fp8 --kv-cache-dtype fp8_e5m2 \
    --mem-fraction-static 0.90 \
    --speculative-algorithm EAGLE3 \
    --speculative-draft-model-path Qwen/Qwen3.6-27B-eagle3 \
    --speculative-num-steps 5 --speculative-eagle-topk 8 \
    --port $((30001+i)) --enable-metrics &
done

# Cache-aware router
python -m sglang_router.launch_router \
  --worker-urls http://127.0.0.1:30001 http://127.0.0.1:30002 \
                http://127.0.0.1:30003 http://127.0.0.1:30004 \
  --policy cache_aware \
  --host 0.0.0.0 --port 30000
```

```
                        TP8 × 1      TP2 × 4      Change
Output tok/s              3284.90      4418.72      +34.5%
Median TTFT (ms)           511.44       362.18      -29.2%
P99 TTFT (ms)             1247.02       938.55      -24.7%
Median ITL (ms)             12.63        17.94      +42.0%
P99 ITL (ms)                34.18        45.60      +33.4%
cache_hit_rate               0.34         0.31       -0.03
```

#### Interpreting Changes

Throughput and TTFT improved, while ITL worsened. The reasons for these diverging trends are different.

-   **Throughput Increase:** TP8 communicates across 8 GPUs per layer, while TP2 only communicates across 2, reducing communication overhead. Additionally, 4 instances independently fill their batches, increasing GPU utilization.
-   **TTFT Decrease:** With 4 instances, the queue is split into 4. While one instance processes a long prefill, other requests can go to different instances.
-   **ITL Increase:** The number of GPUs dedicated to a single request decreased from 8 to 2, meaning computation for generating individual tokens is less distributed, slowing down individual token generation.
-   **Slight Decrease in Cache Hit Rate:** The cache is split across 4 instances. Although the `cache_aware` policy sends requests with the same prefix to the same worker, it doesn't fully compensate.

The median ITL of 17.94ms is well within the SLO of 50ms, so the TP2 x 4 configuration is adopted.

If the ITL SLO were tighter, like 15ms, the opposite conclusion would have been necessary.

### Final Configuration and Cumulative Effects

```
Round            Output tok/s   Median TTFT   P99 TTFT   Median ITL
0 Baseline                 1554          612        4381        28.4
1 Chunked Prefill           1499          648        1621        27.9
2 FP8                      1971          482        1185        21.1
3 Speculative Decoding          3285          511        1247        12.6
4 TP2 × 4 + Router      4419          362         939        17.9

Cumulative                  2.84×        -41%        -79%        -37%
```

SLO Assessment
-   TTFT p99 939ms (p95 approx. 780ms) < 1000ms ✅
-   ITL median 17ms < 50ms ✅

### Summary

Chunked prefill should always be enabled if there's a latency SLO. It trades 3-4% throughput for over 60% tail latency reduction.

**Larger TP size is not always better.** TP reduces individual request latency (due to more memory space) but increases communication costs and reduces opportunities for parallel batching. If there's ample ITL SLO, reducing TP and increasing replicas is advantageous for both throughput and TTFT.

This trade-off varies by case, so it must be verified with actual measurements.

Speculative decoding's effectiveness reverses depending on concurrency. The break-even point should be measured in advance, and an operational rule should be established to automatically disable it if traffic exceeds that point.

<br>

## Document QA / RAG Service

| Item | Value |
|---|---|
| Service | Contract and regulation document Q&A |
| Number of Documents | 1,000 types (80% of traffic concentrated on 50 popular types) |
| Document Length | Average 20,000 tokens |
| Request Rate | Average 8 req/s, Peak 22 req/s |
| Hardware | 4×H100 SXM (80GB, 3.35TB/s) |
| SLO | TTFT p95 < 1,500ms |

### Traffic Profile

```
입력 중앙값: 20530
입력 p95: 24800
출력 중앙값: 300
출력 p95: 620
접두사 공유율: 0.81
```

This case's characteristics are the opposite of the previous one.

The input-output ratio is 68:1, with prefill being overwhelmingly long and decode short.

Speculative decoding is a technique that accelerates the decode phase, so its benefit here is minimal.

Optimization resources should be concentrated on prefill.

**A sharing rate of 0.81 is in the range where SGLang provides maximum benefit.** This means that for questions about the same document, 20,500 out of 20,500 tokens overlap, and only the 30-token question differs.

```
Request A: [System 500][Contract 20,000][Question1 30]
Request B: [System 500][Contract 20,000][Question2 30]
                    └─ 20,500 shared ─┘  └ 30 only different
```

| Model | FP8 Weights | Native Context | Long Document Understanding |
|---|---:|---:|---|
| Qwen3.6-27B | 27 GB | 262K | Excellent |
| Qwen3.6-9B | 9 GB | 262K | Average |
| GLM-5.2 | 372 GB | 1M | Best |

**Qwen3.6-27B** is selected.

GLM5.2 has the best long-document performance, but even with FP8, it's 372GB, so it won't fit into 4xH100 (320GB). The 9B model's accuracy in finding specific clauses within a 20,000-token document noticeably dropped.

### Capacity Calculation

```
Qwen3.6-27B FP8, KV per Token = 96KB

4xH100 = 320GB
Weights: 27GB
Activations, buffer headroom 35GB
KV Cache Available 258GB

Number of cacheable documents = 258GB / (20,000 x 96KB)
                    = 258 GB ÷ 1.92 GB
                    ≈ 134 types
```

Only 134 out of 1000 document types can reside on the GPU.

While the 50 popular types account for 80% of traffic, theoretically allowing most to be cached, the continuous influx of the remaining 950 types causes evictions. This becomes the central tuning problem for this case.

### Single Instance Baseline - Round 0

```bash
python -m sglang.launch_server \
  --model-path Qwen/Qwen3.6-27B \
  --tp-size 4 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --chunked-prefill-size 8192 \
  --mem-fraction-static 0.88 \
  --host 0.0.0.0 --port 30000 \
  --enable-metrics --enable-cache-report
```

`--enable-cache-report` includes the number of cache-hit tokens in the response. This is an essential flag for this case.

Benchmark simulating actual document distribution:

```py
#!/usr/bin/env python3
"""gen_rag_trace.py — Generate trace reflecting document reuse distribution"""
import json, random

SYSTEM = "당신은 계약서 분석 어시스턴트입니다. " * 40          # Approx 500 tokens
DOCS = {i: f"[문서{i}] " + "계약 조항 본문 " * 3300 for i in range(1000)}
QUESTIONS = ["해지 조항의 통지 기간은?", "위약금 산정 기준은?",
             "관할 법원은 어디인가?", "자동 갱신 조건은?"]

def pick_doc():
    # 80% for 50 popular types, 20% for the remaining 950 types
    return random.randint(0, 49) if random.random() < 0.8 else random.randint(50, 999)

with open("rag_trace.jsonl", "w") as f:
    for _ in range(2000):
        d = pick_doc()
        f.write(json.dumps({
            "prompt": SYSTEM + DOCS[d] + random.choice(QUESTIONS),
            "output_len": random.randint(200, 600),
            "doc_id": d,
        }) + "\n")
```

```
============ Serving Benchmark Result ============
Max request concurrency:                 22
Successful requests:                     2000
Benchmark duration (s):                  412.66
Request throughput (req/s):              4.85
Input token throughput (tok/s):          99512.30
Output token throughput (tok/s):         1455.11
Concurrency:                             21.74
---------------Time to First Token----------------
Mean TTFT (ms):                          1962.44
Median TTFT (ms):                        1843.07
P99 TTFT (ms):                           3721.85
---------------Inter-Token Latency----------------
Median ITL (ms):                         14.22
==================================================
```
```bash
curl -s http://127.0.0.1:30000/metrics | grep cache_hit_rate
# sglang_cache_hit_rate{model="Qwen/Qwen3.6-27B"} 0.62
```

The request rate of 4.85 req/s falls short of the peak 22 req/s, and the median TTFT of 1,843ms exceeds the SLO of 1,500ms.

The cache hit rate of 0.62 is below the sharing rate of 0.81, with the 0.19 difference being lost to evictions.

With a capacity of 134 types, 1,000 types are constantly entering and leaving, causing even popular documents to be evicted.

### Round 1: Replica Splitting and Cache-Aware Routing

Split one TP4 into two TP2s and place a router in front. Compare two routing policies.

```bash
for i in 0 1; do
  CUDA_VISIBLE_DEVICES=$((i*2)),$((i*2+1)) \
  python -m sglang.launch_server \
    --model-path Qwen/Qwen3.6-27B --tp-size 2 \
    --quantization fp8 --kv-cache-dtype fp8_e5m2 \
    --chunked-prefill-size 8192 --mem-fraction-static 0.88 \
    --port $((30001+i)) --enable-metrics --enable-cache-report &
done

# Comparison A: Round Robin
python -m sglang_router.launch_router \
  --worker-urls http://127.0.0.1:30001 http://127.0.0.1:30002 \
  --policy round_robin --port 30000

# Comparison B: Cache-Aware
python -m sglang_router.launch_router \
  --worker-urls http://127.0.0.1:30001 http://127.0.0.1:30002 \
  --policy cache_aware --port 30000
```

```
                        TP4×1     RR TP2×2   cache_aware TP2×2
cache_hit_rate           0.62       0.44          0.79
Median TTFT (ms)      1843.07    2510.33        892.41
P99 TTFT (ms)         3721.85    4980.12       1684.30
Output tok/s          1455.11    1288.40       2740.66
req/s                    4.85       4.29          9.14
```

With round-robin, performance worsened. Questions about the same document were scattered across two workers, causing both to prefill the same document, effectively halving the cache capacity. The drop in hit rate from 0.62 to 0.44 illustrates this loss.

`cache_aware` routing sends requests with the same prefix hash to the same worker.

Since each worker handles a subset of documents, they only need to manage 500 types each. Cache contention is halved, and the hit rate increased to 0.79.

**With the same hardware, a single routing policy can make a 2.8x difference in TTFT.** For workloads with high prefix sharing, round-robin is a configuration that nullifies the cache.

### Round 2 - Prompt Redesign

Let's assume a review of the system prompt revealed a variable value at the beginning.

```python
# Problematic form
SYSTEM = f"""현재 시각: {datetime.now()}
요청 ID: {request_id}
사용자: {user_name}
당신은 계약서 분석 어시스턴트입니다. ..."""
```

The `현재시각` (current time) changes with each request, causing the prefix to differ from the first token. This makes the subsequent 20,500 tokens uncacheable even if they are identical.

Move the variable values to the end.

```py
# Modified form
SYSTEM = """당신은 계약서 분석 어시스턴트입니다. ...
(고정 지침 전체)"""

PROMPT = SYSTEM + DOCUMENT + f"""
---
현재 시각: {datetime.now()}
요청 ID: {request_id}
질문: {question}"""
```

```
Round 1    Round 2      Change
cache_hit_rate          0.79       0.94       +0.15
Median TTFT (ms)      892.41     412.88      -53.7%
P99 TTFT (ms)        1684.30     961.22      -42.9%
Output tok/s         2740.66    3612.19      +31.8%
req/s                   9.14      12.04       +31.7%
```

Moving three lines of code halved the TTFT, with no change in hardware settings.

If we look at the number of tokens that need to be prefetched:

```
Before modification: Prefill all 20,530 tokens for each request (cache invalidated)
After modification: Prefill only 30-50 tokens on cache hit

20,530 -> 40
```

Prefix caching only works for exact prefix matches.

If there are variable values in the prompt template, everything after them is invalidated. Therefore, the prompt structure must be reviewed before serving optimization.

### Round 3 - Expanding Capacity with HiCache

The remaining 0.06 of the 0.94 hit rate is due to unpopular 950 documents, limited by the GPU's capacity for 134 types. This calls for extending to CPU memory with hierarchical caching.

### What Hierarchical Cache Does

RadixAttention keeps KV cache only in GPU memory.

When capacity is full, older entries are evicted using LRU. If an evicted document is requested again, 20,000 tokens are prefetched from scratch.

Hierarchical Cache (HiCache) instead **moves them to a lower tier.**

```
Capacity        Bandwidth        Role in this case
┌──────────────┐
│  GPU HBM     │      258 GB      3,350 GB/s    134 popular documents reside
├──────────────┤
│  CPU DRAM    │      516 GB         64 GB/s    Stores evicted documents
├──────────────┤
│  NVMe / Remote│      Several TB      7 GB/s    Long-term storage (optional)
└──────────────┘
        ↑ Larger and slower as you go down
```

KV evicted from the GPU is moved to CPU memory. If that document is requested again, it's restored by **transfer instead of re-computation.** Therefore, if transfer time is slower than re-computation time, it should not be used.

> It's faster to keep 134 frequently used books on your desk (GPU) and put the rest in a drawer under your feet or on a distant bookshelf than to go back to the library (re-computation) to borrow them again. However, taking them out of the drawer is not free.

The names also differ depending on the transfer direction:

```
Eviction: GPU >> CPU is called offloading and is performed in the background.
Hit: CPU >> GPU is called prefetching and is performed just before prefill.
```

Since fetching directly impacts latency, it's crucial to start it before the scheduler processes the request.

`hicache-storage-prefetch-policy` controls this behavior.

**Policies**
-   `best_effort`: If transfer is slow, it doesn't wait and recomputes that portion. Suitable when latency SLOs are strict.
-   `wait_complete`: Waits until transfer is complete. Suitable when recomputation is very expensive.
-   `timeout`: Waits only for a certain period, then gives up. Suitable for a compromise.

**Before adoption, a break-even analysis is necessary.**

```
KV cache for one document = 20,000 × 96 KB = 1.92 GB

Recomputation cost (prefill)
  20,000 tokens × 2 × 27e9 FLOP = 1.08e15 FLOP
  H100 FP8 1,979 TFLOPS × 4 GPUs × 60% effective = 4,750 TFLOPS
  → 1.08e15 / 4.75e15 ≈ 227 ms

Transfer cost (CPU DRAM → GPU)
  1.92 GB ÷ 64 GB/s (PCIe 5.0) ≈ 30 ms

30 ms < 227 ms  →  Offloading is 7.6 times more advantageous
```

If this ratio is less than 1, HiCache is detrimental.

For workloads with long outputs and short inputs, the opposite result may occur, so always calculate with your own numbers.

```diff
python -m sglang.launch_server \
    --model-path Qwen/Qwen3.6-27B --tp-size 2 \
    --quantization fp8 --kv-cache-dtype fp8_e5m2 \
+   --page-size 64 \
+   --enable-hierarchical-cache \
+   --hicache-ratio 2 \
+   --hicache-write-policy write_through \
+   --hicache-io-backend kernel \
    --chunked-prefill-size 8192 --mem-fraction-static 0.88 \
    --port 30001 --enable-metrics --enable-cache-report
```

Meaning of each flag:

-   `--enable-hierarchical-cache`: Enables hierarchical cache. If not enabled, other hicache flags are ignored.
-   `--page-size 64`: Sets the unit size for KV cache pages in tokens. One page = 64 tokens.
-   `--hicache-ratio 2`: Determines the size of the CPU-side KV pool. Using 2 times the GPU pool means a total of 3 times the capacity (GPU + 2x CPU).
-   `--hicache-write-policy write_through`: Determines when to copy to CPU. `write_through` copies immediately upon writing to GPU, used when hit rate is prioritized.
-   `--hicache-io-backend kernel`: Determines the path for GPU <-> CPU transfer. `kernel` uses the kernel path, while `direct` is advantageous for large transfers.

In this case, `kernel` is used because the transfer targets are scattered.

Evicted pages from the Radix tree are not physically contiguous. A 20,000-token document is split into 313 pages.

For such scattered transfers, bundling them via the kernel is advantageous.

`direct` would be considered when the chunks being moved are large and contiguous, for example, if page size was increased to 128 or more, or if prefetching was done at the document level, organizing transfers into a few large blocks.

> The actual difference between the two methods depends on PCIe generation, page size, and number of concurrent transfers. It's safer to start with the default `kernel` and only try `direct` if transfer is identified as a bottleneck. If there's no difference after switching, it means transfer wasn't the bottleneck.

`--page-size` determines how many tokens are managed per KV cache page. The default value of 1 means one token is one page, and hierarchical cache does not operate in this state.

This is because transferring token by token between GPU and CPU incurs overhead larger than the actual data for each request.

64 is used as a compromise between transfer efficiency and cache waste.

Cache waste occurs because prefix sharing is only recognized at the page level.

If even one token within a page differs, the entire page must be recomputed.

```
Page size 64, prefixes of two requests are identical up to 20,010 tokens, then diverge.

Pages:  [0~63][64~127] ... [19,904~19,967][19,968~20,031][20,032~ ...
                                    ↑ Identical up to here      ↑ Diverges here
                                      = Reusable ✅              = Recompute ❌

Within the last page 19,968~20,031:
  Up to token 20,010 is the same   ← 43 tokens could have been reused
  From token 20,011 onwards differs ← This invalidates the entire page

→ 43 tokens that could have been reused are wasted. This is boundary waste.
```

The amount of waste is **(maximum page size - 1) tokens**. For a page size of 64, it's a maximum of 63 tokens, varying from 0 to 63 depending on where the prefix diverges within the page.

```
Larger page size
  Larger data per transfer, better PCIe efficiency   ← Benefit
  Higher upper bound for boundary waste              ← Detriment

For a 20,000-token document, page size 64:
  Transfer unit = 64 × 96 KB = 6.1 MB   → PCIe round-trip overhead is amortized
  Boundary waste = max 63 tokens            → 0.3% compared to 20,000
```

This workload involves long documents (20,000 tokens), so the fractional loss is negligible. Conversely, for services with prompts of only a few hundred tokens, 64 would incur significant waste, so 16 or 32 should be considered.

`--hicache-ratio 2` results in the following capacity:

```
GPU KV Pool    258 GB  →  134 documents     (Base for ratio. Always 1x)
Host Pool      516 GB  →  268 documents     (258 × ratio 2)
                        ────────────
Total Cacheable          402 documents          → 3 times when using GPU only (1 + 2)

40% of 1,000 documents reside in cache. 50 popular documents always remain on GPU.
```

The host pool does not replace the GPU pool but is layered on top, so the total capacity is always (1 + ratio) times. A ratio of 4 would mean 5 times.

`--hicache-size 200` can also be used to specify an absolute size in GB.

If specified, it overrides the ratio, and the host pool occupies that much CPU memory, so first check system RAM availability.

To allocate 516GB, there must be more physical memory than that.

```
Round 2    Round 3      Change
cache_hit_rate          0.94       0.97       +0.03
Median TTFT (ms)      412.88     356.20      -13.7%
P99 TTFT (ms)         961.22     594.71      -38.1%
Output tok/s         3612.19    3844.02       +6.4%
req/s                  12.04      12.81       +6.4%
```

Median improvement is 13.7%, not large, but P99 improved by 38%.

Popular documents already in cache (dominating the median) remain unchanged. The **restoration of unpopular documents (requests causing tail latency) to CPU memory** shortened the tail.

Compared `write_through` and `write_back`.

```
write_through   write_back
cache_hit_rate            0.97         0.93
Median TTFT (ms)        356.20       341.55
Output tok/s           3844.02      3901.30
```

`write_back` delays host writes, reducing GPU overhead, but there might be unwritten blocks at eviction time, leading to a lower hit rate. **For workloads where hit rate is dominant, `write_through` is appropriate.**

### Final Configuration and Cumulative Effects

```
Round                      hit_rate   Median TTFT   P99 TTFT   req/s
0 TP4 Single                    0.62        1843        3722      4.85
1 TP2×2 + cache_aware         0.79         892        1684      9.14
2 Prompt Redesign               0.94         413         961     12.04
3 HiCache                     0.97         356         595     12.81

Cumulative                        +0.35        -81%        -84%     2.64×
```

Peak 22 req/s is not met by 12.81 req/s, so either adding another node or pre-warming popular documents is needed.

Pre-warming involves prefilling and caching the 50 popular documents at service startup.

```bash
# Pre-warm popular documents
for doc in $(head -50 popular_docs.txt); do
  curl -s http://127.0.0.1:30000/generate \
    -d "{\"text\": \"$(cat docs/$doc.txt)\", \"sampling_params\": {\"max_new_tokens\": 1}}" \
    > /dev/null
done
```

### Lessons Learned

1.  **Routing policy becomes part of the caching strategy.** For workloads with high prefix sharing, round-robin invalidates the cache. Before increasing workers, check routing.
2.  **Prompt structure can have a greater effect than infrastructure settings.** Simply moving variable values from the front to the back halved TTFT. If serving and application teams are separate, this issue might go unnoticed.
3.  **HiCache improves tail latency more than median latency:** Requests already cached don't change, but requests that would have missed are restored from CPU memory, making it especially valuable for services with p99 SLOs.
4.  **Before adopting hierarchical caching, a break-even analysis is necessary:** If the computation speed to transfer ratio is not greater than 1, it's a net loss. Of course, cases with insufficient computational resources might be different.

<br>

## Nightly Batch Document Processing

| Item | Value |
|---|---|
| Task | Classify and summarize 2.4 million customer inquiries |
| Input | Average 1,200 tokens |
| Output | Average 150 tokens (JSON label + 3-sentence summary) |
| Deadline | Complete within 12 hours overnight |
| Hardware | 2×H100 SXM |
| SLO | **None.** Individual latency irrelevant, only total completion time. |

The absence of a latency SLO differentiates this case from others.

The optimizations done in Case 1 are entirely unnecessary and even counterproductive here.

### Required Throughput Calculation

```
Total Input = 2,400,000 x 1,200 = 2.88e9 tokens
Total Output = 2,400,000 x 150 = 3.60e8 tokens

12 hours = 43,200 seconds
Required Output Throughput =  3.60e8 / 43,200 ≈ 8,333 tok/s
Required Input Throughput = 2.88e9 / 43,200 ≈ 66,667 tok/s
```

### Round 0 - Assuming Case 1's Settings are Adopted

```bash
python -m sglang.launch_server \
  --model-path Qwen/Qwen3.6-27B --tp-size 2 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --chunked-prefill-size 4096 \
  --speculative-algorithm EAGLE3 \
  --speculative-draft-model-path Qwen/Qwen3.6-27B-eagle3 \
  --speculative-num-steps 5 --speculative-eagle-topk 8 \
  --mem-fraction-static 0.90 \
  --port 30000 --enable-metrics
```

```
Max request concurrency:                 256
Output token throughput (tok/s):         2148.33
Input token throughput (tok/s):          17186.64
Concurrency:                            248.91
Median ITL (ms):                         112.40
```

```
Required output throughput 8,333 tok/s vs 2,148 tok/s
→ Estimated completion time = 3.60e8 / 2148 / 3600 ≈ 46.5 hours
```

The deadline is 12 hours, but it's estimated to take 46.5 hours, nearly four times longer.

This is natural. The optimization itself was done with a different objective.

### Round 1 - Model Downsizing

Throughput in batch jobs is strongly inversely proportional to model size.

Finding the minimum model suitable for the task difficulty is paramount.

Let's start with quality verification. Blindly evaluate candidates using 500 human-labeled validation sets.

```bash
python eval_classification.py \
  --models Qwen3.6-27B Qwen3.6-9B Qwen3.6-4B Qwen3.6-2B \
  --testset labeled_500.jsonl \
  --metrics accuracy,rouge-l
```

```
Model              Classification Accuracy   Summary ROUGE-L   FP8 Weights
Qwen3.6-27B         0.941         0.412         27 GB
Qwen3.6-9B          0.933         0.398          9 GB
Qwen3.6-4B          0.897         0.361          4 GB
Qwen3.6-2B          0.812         0.294          2 GB
```

Assume that the 9B model's accuracy drop of 0.8%p and ROUGE-L drop of 0.014 compared to 27B are within the acceptable error margin for this task (which they likely are, being small).

> ROUGE-L is an evaluation score that measures how similar an AI-generated summary or translation is to a human-generated reference. Unlike other methods that only look at consecutive words, it finds the **Longest Common Subsequence (LCS)** in sentences to determine the degree of structural similarity.

The 4B model is excluded because its accuracy drops by 4.4%p, which would increase subsequent manual review costs.

```bash
python -m sglang.launch_server \
  --model-path Qwen/Qwen3.6-9B --tp-size 2 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --chunked-prefill-size 4096 \
  --speculative-algorithm EAGLE3 \
  --speculative-draft-model-path Qwen/Qwen3.6-9B-eagle3 \
  --speculative-num-steps 5 --speculative-eagle-topk 8 \
  --mem-fraction-static 0.90 \
  --port 30000 --enable-metrics
```

```
                       27B        9B        Change
Output tok/s         2148.33   5912.77     +175%
Input tok/s         17186.64  47302.16     +175%
Median ITL (ms)      112.40     41.83      -62.8%
Estimated Completion (hours)        46.5      16.9      -63.7%
```

By simply changing the model without altering settings, efficiency increased by **2.75 times**.

It still exceeds the 12-hour deadline at 16.9 hours, but the gap has significantly narrowed.

### Round 2 - Removing Speculative Decoding

A concurrency of 256 is close to the H100 threshold batch size of 295.

According to calculations in the GPU hardware notes, there's almost no computational resource headroom in this range, and running the draft model can take away resources from computation.

```diff
- --speculative-algorithm EAGLE3 \
- --speculative-draft-model-path Qwen/Qwen3.6-9B-eagle3 \
- --speculative-num-steps 5 --speculative-eagle-topk 8 \
```
```
                    Speculative ON    Speculative OFF     Change
Output tok/s        5912.77    7104.20     +20.2%
Median ITL (ms)       41.83      35.98     -14.0%
Estimated Completion (hours)        16.9       14.1     -16.6%
```

The reason is clear from the server logs' acceptance rate:

```
[Speculative ON, Concurrency 256]
  accept_length_mean: 2.14 / 6
  accept_rate: 0.357
  draft_overhead_ratio: 0.284
```

The acceptance rate, which was 0.637 in Case 1, is 0.357 in this case, and draft overhead increased from 11.8% to 28.4%.

Proposing 6 tokens and only accepting 2.14 while spending 28% of the cost means it's better to turn it off.

The technique that yielded a 66% gain in Case 1 results in a 20% loss here. This is a prime example of how the sign of the same technique's effect can reverse depending on the workload.

In this case, optimization for input rather than output is needed, so work for decode optimization is unnecessary, and speculative decoding is removed.

> Speculative decoding is a technique that speeds up generation by having a lightweight draft model quickly predict multiple tokens, which are then validated all at once by a heavier target model. Removing the draft model means the target model and GPU scheduler can focus solely on their inherent large-scale computations without the draft model's prefill task. This frees up all GPU memory (SRAM/HBM) and tensor cores previously occupied by the draft model, allowing the target model to process long prompts more efficiently.

### Round 3 - Maximizing Batch Parameters

Since there is no latency SLO, maximize the batch size.

```diff
  python -m sglang.launch_server \
    --model-path Qwen/Qwen3.6-9B --tp-size 2 \
    --quantization fp8 --kv-cache-dtype fp8_e5m2 \
-   --chunked-prefill-size 4096 \
+   --chunked-prefill-size 32768 \
+   --max-running-requests 512 \
+   --max-prefill-tokens 65536 \
-   --mem-fraction-static 0.90 \
+   --mem-fraction-static 0.94 \
+   --schedule-conservativeness 0.3 \
    --port 30000 --enable-metrics
```

-   `--chunked-prefill-size 32768`: Latency is irrelevant, so there's no reason to split. Larger matrices are beneficial for GPU efficiency.
-   `--max-running-requests 512`: Expand the upper limit for concurrent sequences.
-   `--max-prefill-tokens 65536`: Expand prefill tokens to accommodate a larger batch.
-   `--mem-fraction-static 0.94`: Expand KV pool. Safe to reduce headroom for a dedicated instance.
-   `--schedule-conservativeness 0.3`: Make the scheduler more aggressive. Fill batches at the risk of preemption.

```
                  Round 2    Round 3     Change
Output tok/s       7104.20    9438.51    +32.9%
Input tok/s       56833.60   75508.08    +32.9%
Median ITL (ms)      35.98      68.22    +89.6%
Concurrency         248.91     487.33
Estimated Completion (hours)       14.1       10.6    -24.8%
token_usage           0.71       0.93
```

**ITL worsened by 90%, but this is not an issue in this case, as there is no latency SLO.**

`token_usage` 0.93 means the KV cache is almost saturated. If we increase `mem-fraction-static` further to 0.96:

```
--mem-fraction-static 0.96
[2026-08-07 03:14:52] Warning: cache pool exhausted, preempting 14 requests
[2026-08-07 03:16:08] Warning: cache pool exhausted, preempting 22 requests
Output tok/s: 8712.44     (-7.7%)
```

Preemption occurs, actually slowing it down. Preempted requests are recomputed from scratch, wasting work. 0.94 is the upper limit for this configuration.

### Round 4 - Adjusting Client-side Parallelism

Sometimes the server is ready, but the client cannot push enough requests.

```py
#!/usr/bin/env python3
"""batch_runner.py — Batch job injector"""
import asyncio, aiohttp, json, time

CONCURRENCY = 512          # Matches server's max-running-requests
ENDPOINT = "http://127.0.0.1:30000/generate"

async def worker(session, sem, item, results):
    async with sem:
        payload = {
            "text": item["prompt"],
            "sampling_params": {"max_new_tokens": 200, "temperature": 0.0},
        }
        async with session.post(ENDPOINT, json=payload) as r:
            results.append(await r.json())

async def main(path):
    items = [json.loads(l) for l in open(path)]
    sem = asyncio.Semaphore(CONCURRENCY)
    results = []
    t0 = time.time()
    conn = aiohttp.TCPConnector(limit=CONCURRENCY + 64)
    async with aiohttp.ClientSession(connector=conn) as s:
        await asyncio.gather(*[worker(s, sem, i, results) for i in items])
    dt = time.time() - t0
    print(f"{len(results)}건 / {dt:.1f}초 / {len(results)/dt:.1f} req/s")

asyncio.run(main("batch_input.jsonl"))
```

```
Client Concurrency    Server Concurrency   Output tok/s
      128                 127.4            5920.11
      256                 254.8            7802.33
      512                 487.3            9438.51
     1024                 496.1            9401.77   ← Saturation
```

Client concurrency around 512 is best; beyond that is the saturation point. Further increases only lengthen the queue without increasing throughput. It's important to match the server's `--max-running-requests` with client concurrency.

### Final Configuration and Cumulative Effects

```
Round                    Output tok/s   Estimated Completion (h)   Median ITL
0 27B + Case 1 Settings          2148          46.5         112.4
1 Replaced with 9B                   5913          16.9          41.8
2 Removed Speculative Decoding            7104          14.1          36.0
3 Maximized Batch Parameters          9439          10.6          68.2
4 Client Concurrency Alignment        9439          10.6          68.2

Cumulative                        4.39×         -77%
```

Compared to the 12-hour deadline, 10.6 hours leaves 1.4 hours of buffer.

### Lessons Learned

1.  **Model selection yields greater gains than the sum of all configuration tuning.** Switching from 27B to 9B alone provided a 2.75x benefit. All three subsequent tuning rounds combined only yielded 1.6x. Before tuning, verify with a validation set if the chosen model size is truly necessary for the task.
2.  **Downsizing without quality verification is risky:** A 0.8%p accuracy drop might be acceptable, but a larger drop like 4.4%p could lead to increased manual review costs. **Acceptable error should be judged not only by accuracy metrics but also by the trade-off between increased subsequent costs and reduced GPU costs.**
3.  Speculative decoding, which provided a +66% efficiency in Case 1, resulted in -20% here, and chunked prefill size increased eightfold from 4,096 to 32,768. The difference was the presence or absence of a latency SLO. This is why settings from other projects should not be adopted blindly.
4.  In this case, simply maximizing memory usage was not always beneficial. Setting `mem-fraction-static` to 0.96 caused preemption, reducing throughput by 7.7%. Preempted requests are recomputed, wasting work. Monitor `preempting` warnings and `token_usage` above 0.95.

<br>

## Structured Output API

| Item | Value |
|---|---|
| Service | Tool call backend for customer CRM agents |
| Request Rate | Average 25 req/s, Peak 40 req/s |
| Input | 1,800 tokens (Tool definition 1,200 + Conversation 600) |
| Output | 120 tokens (JSON function call) |
| Hardware | 2×H100 SXM |
| SLO | TTFT p95 < 800ms, **JSON parsing failure rate 0%** |

The JSON parsing failure rate SLO is a distinguishing feature of this case.

If an agent's task involves, say, 20 chained tool calls, and each step can independently fail:

```
If step-by-step success rate is p, overall success rate for 20 steps = p²⁰

p = 0.999  →  0.999²⁰ = 0.980   →  98.0%
p = 0.99   →  0.99²⁰  = 0.818   →  81.8%
```

Even a 1% failure rate at one step means that if 20 steps are chained, the entire task will fail one out of five times.

The same multiplication does not apply to latency. If TTFT increases from 800ms to 900ms, the user just waits a little longer. But a parsing failure renders all subsequent steps meaningless. Therefore, **failure rate is a constraint that takes precedence over latency.**

### Model Selection

| Model | FP8 Weights | Tool Call Accuracy | Batch 40 Throughput |
|---|---:|---:|---|
| Qwen3.6-27B | 27 GB | 0.947 | Low |
| Qwen3.6-9B | 9 GB | 0.921 | Medium |
| Qwen3.6-35B-A3B | 35 GB | 0.938 | **High** |

**Qwen3.6-35B-A3B** is selected.

The output is short (120 tokens), and the request rate is high, maintaining concurrency around 40.

The active parameter of MoE (3B) means only 3GB (FP8) of weights are read per decode, making it faster than dense 9B.

Although 35GB total weights must be loaded into memory, there's ample headroom on 2xH100 = 160GB.

This is the opposite conclusion from Case 1, where dense was chosen. **MoE is advantageous when output is short and concurrency is high, while dense is advantageous when output is long and concurrency is low.**

### Round 0 - JSON Request via Prompt Only

```bash
python -m sglang.launch_server \
  --model-path Qwen/Qwen3.6-35B-A3B --tp-size 2 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --chunked-prefill-size 8192 --mem-fraction-static 0.90 \
  --port 30000 --enable-metrics
```

Requests instruct the format via the prompt.

```json
{
  "text": "...tool list...\n\nRespond strictly in the following JSON format:\n{\"tool\": \"...\", \"args\": {...}}\n\nUser: Show me Kim Cheol-su's last 3 orders",
  "sampling_params": {"max_new_tokens": 200, "temperature": 0.0}
}
```

```
Max request concurrency:                 40
Successful requests:                     5000
Median TTFT (ms):                        412.33
P99 TTFT (ms):                           788.21
Output token throughput (tok/s):         4128.60
req/s                                    34.41
```

Parsing results are aggregated separately.

```py
#!/usr/bin/env python3
"""check_schema.py — Aggregate schema compliance rate of output"""
import json, jsonschema, collections

SCHEMA = json.load(open("tool_call.schema.json"))

def audit(path):
    stats = collections.Counter()
    for line in open(path):
        out = json.loads(line)["generated_text"]
        try:
            obj = json.loads(out)
        except json.JSONDecodeError:
            stats["JSON parsing failed"] += 1; continue
        try:
            jsonschema.validate(obj, SCHEMA)
            stats["Normal"] += 1
        except jsonschema.ValidationError as e:
            stats[f"Schema violation: {e.validator}"] += 1
    return stats

for k, v in audit("outputs.jsonl").most_common():
    print(f"{k:28s} {v:5d}  ({v/50:.2f}%)")
```
```
Normal                          4813  (96.26%)
JSON parsing failed               87  ( 1.74%)
Schema violation: enum            61  ( 1.22%)
Schema violation: required        28  ( 0.56%)
Schema violation: type            11  ( 0.22%)
```

The failure rate is 3.74%. While TTFT meets the SLO, the failure rate SLO is significantly exceeded.

As seen earlier, even a 1% failure rate can drop agent task success to 82%. The current figure could drop to 46.7%, meaning **more than half would fail midway.**

Examining failure cases reveals different types.

```
[JSON parsing failed example]
"Of course! I will look it up as follows.\n{\"tool\": \"get_orders\"...}"
  → Prepends an explanatory sentence

"{\"tool\": \"get_orders\", \"args\": {\"customer\": \"김철수\", \"limit\": 3}"
  → Missing closing curly brace

[enum violation example]
{"tool": "get_customer_orders", ...}
  → Invents a tool name not in the tool list
```

These are probabilistic failures that remain no matter how much the prompt is refined.

### Round 1 - Grammar-Constrained Decoding

#### What Constrained Decoding Does

Asking for JSON via a prompt is probabilistic. While the model is likely to pick `{` as the next token, `물` (water) is not 0, and 187 out of 5,000 times, that might be chosen.

Constrained decoding, instead of asking, selectively and physically blocks certain tokens.

```
Generated so far: {"tool": "get_
Next tokens allowed by grammar: orders  profile   ← Only those in enum
                             ↓
Model's original probability distribution        [orders 0.6][profile 0.3][customer 0.08][물 0.02]
After masking                    [orders 0.67][profile 0.33][  0  ][  0  ]
                                                        ↑ Made -inf, so cannot be sampled
```

It prevents the model from inventing non-existent tool names by setting their probabilities to 0.

#### XGrammer

This engine performs masking at each token. It's the default grammar backend for SGLang, and can be changed to `llguidance` or `outlines` using `--grammar-backend`.

The problem is that checking all 150,000 vocabulary items at each token becomes a bottleneck itself. XGrammer avoids this by dividing tokens into two categories:

```
Context-independent tokens   Validity is determined regardless of current parsing state.
                             Example: ordinary characters within a string are always allowed,
                             regardless of nesting depth. → Pre-calculated.

Context-dependent tokens   Validity requires checking the parsing stack.
                           Example: `}` is valid only if an object is open.
                           → Checked at runtime, but performed concurrently with GPU computation.
```

Most of the vocabulary is context-independent. Only a few dozen tokens (`{ } [ ] , : "`) form the structure, while the rest are content. Therefore, only a minority require real-time judgment. Even these are processed in parallel by the CPU while the GPU computes the next token.

Let's enable the XGrammer backend and pass the JSON schema with each request.

```diff
  python -m sglang.launch_server \
    --model-path Qwen/Qwen3.6-35B-A3B --tp-size 2 \
    --quantization fp8 --kv-cache-dtype fp8_e5m2 \
+   --grammar-backend xgrammar \
    --chunked-prefill-size 8192 --mem-fraction-static 0.90 \
    --port 30000 --enable-metrics
```

```json
{
  "text": "...",
  "sampling_params": {
    "max_new_tokens": 200,
    "temperature": 0.0,
    "json_schema": "{\"type\":\"object\",\"properties\":{\"tool\":{\"type\":\"string\",\"enum\":[\"get_orders\",\"get_profile\",\"create_ticket\",\"search_kb\"]},\"args\":{\"type\":\"object\"}},\"required\":[\"tool\",\"args\"]}"
  }
}
```

```
Normal                          5000 (100.00%)
JSON parsing failed                0
Schema violation                   0
```

```
                  Round 0    Round 1     Change
Failure Rate              3.74%      0.00%     Resolved
Median TTFT (ms)   412.33     468.90    +13.7%
P99 TTFT (ms)      788.21    1342.55    +70.3%
Output tok/s      4128.60    4402.18     +6.6%`
```

The failure rate became 0 percent.

Because it sets the probability of grammatically impossible tokens to 0, probabilistic failures are fundamentally eliminated.

The disappearance of `enum` violations is particularly important, as it's now impossible to invent non-existent tool names.

#### Jump-Forward Decoding

The output token throughput increased by 6.6%.

This is an additional optimization by SGLang. While constrained decoding **prevents incorrect answers**, jump-forward skips predetermined answers.

```
When generating {"tool": "

Normal constrained decoding
  {  →  "  →  tool  →  "  →  :  →  (space)  →  "     7 model calls
  ↑ Even if only one token is valid at each position, the model is queried every time.

Jump-forward
  Emits {"tool": " all at once                        0 model calls
  ↑ Grammar already knows the answer, so it just writes it.
```

Calling the model when the next token's entropy is 0 is wasteful.

Therefore, **outputs with a high structural component can actually be faster when constrained.**

### Results

-   **Failure rate resolved**: By setting the probability of grammatically impossible tokens to 0, probabilistic failures are fundamentally blocked. This eliminated `enum` violations, which were the most common in Round 0, meaning the model no longer invents non-existent tool names.
-   **Throughput increased by 6.6%**: Due to the jump-forward decoding effect, tool call JSONs have nearly half their tokens as structural tokens, reducing model calls proportionally.
-   **P99 TTFT worsened by 70.3%**: For grammar compilation cost, when a new schema arrives, XGrammer creates a parsing automaton and precomputes context-independent masks. The first request for that schema bears the entire cost. Subsequent requests use the cache.

The median increased by only 13.7%, while P99 increased by 70.3%. This indicates that most requests hit the cache, and the few that incurred compilation costs drove up P99.

Looking at the server logs:

```
[2026-08-07 09:12:04] Grammar cache miss, compiling schema (hash=a3f21b8c) ... 384ms
[2026-08-07 09:12:07] Grammar cache miss, compiling schema (hash=7d90e412) ... 411ms
[2026-08-07 09:12:11] Grammar cache hit (hash=a3f21b8c)
```

### Round 2 - Grammar Cache Warming

The schemas for this service are determined by tool combinations and are finite.

Since 17 types of schemas appeared in actual traffic, let's pre-compile all of them at service startup to fill the compilation cache.

```bash
#!/usr/bin/env bash
# warm_grammar.sh — Grammar cache pre-compilation

for schema in schemas/*.json; do
  curl -s http://127.0.0.1:30000/generate \
    -H "Content-Type: application/json" \
    -d "$(jq -n --arg s "$(cat $schema)" \
         '{text: "warmup", sampling_params: {max_new_tokens: 1, json_schema: $s}}')" \
    > /dev/null
  echo "Compilation complete: $schema"
done
```

```
                  Round 1    Round 2     Change
Median TTFT (ms)   468.90     421.07    -10.2%
P99 TTFT (ms)     1342.55     612.33    -54.4%
Output tok/s      4402.18    4488.90     +2.0%
Failure Rate               0.00%      0.00%
```

P99 dropped by more than half. The compilation cost didn't disappear; it was **shifted to service startup**, so user requests no longer bear that cost.

There are cases where schemas are dynamically generated and cannot be pre-compiled. If users can register custom tools, the number of schema types is infinite. In such cases, the characteristics of each backend must be checked.

```
Backend          Dynamic Schema    Compilation Time (Complex Schema)    Overhead per Token
xgrammar          Supported          40~400 ms                < 40 µs
llguidance        Supported          Almost none                 ~50 µs
outlines          Limited        40 s ~ 10 min              1 lookup
```

`outlines` fully precomputes the FSM, so for static schemas, its per-token cost is the lowest. However, for complex schemas, compilation can take several minutes, and it cannot handle recursive structures.

**If dynamic schemas are present**, `xgrammer` or `llguidance` are the choices, and must be verified with actual schemas.

### Round 3 - Placing Tool Definitions as Prefixes

Out of 1,800 input tokens, 1,200 tokens are tool definitions, and they are identical across all requests. However, the cache hit rate was low.

```bash
curl -s http://127.0.0.1:30000/metrics | grep cache_hit_rate
# sglang_cache_hit_rate{model="Qwen/Qwen3.6-35B-A3B"} 0.18
```

Checking the prompt composition, the conversation history was placed before the tool definitions.

```
Before modification: [Conversation History 600][Tool Definition 1,200][User Question]
                └ Varies each time ┘ → Invalidates cache for subsequent 1,200 tokens

After modification: [Tool Definition 1,200][Conversation History 600][User Question]
         └ Always identical ┘ → 1,200 tokens cache hit
```

```
                  Round 2    Round 3     Change
cache_hit_rate       0.18       0.67      +0.49
Median TTFT (ms)   421.07     198.44    -52.9%
P99 TTFT (ms)      612.33     341.20    -44.3%
Output tok/s      4488.90    5920.33    +31.9%
req/s               37.41      49.34    +31.9%
```

This is a familiar pattern. The principle of placing fixed parts at the beginning and variable parts at the end applies to all workloads.

### Final Configuration

```bash
python -m sglang.launch_server \
  --model-path Qwen/Qwen3.6-35B-A3B \
  --tp-size 2 \
  --quantization fp8 \
  --kv-cache-dtype fp8_e5m2 \
  --grammar-backend xgrammar \
  --chunked-prefill-size 8192 \
  --mem-fraction-static 0.90 \
  --max-running-requests 64 \
  --host 0.0.0.0 --port 30000 \
  --enable-metrics --enable-cache-report

# After startup
./warm_grammar.sh
```

```
Round                    Failure Rate   Median TTFT   P99 TTFT   req/s
0 Prompt Instruction Only           3.74%       412         788     34.41
1 XGrammar                0.00%       469        1343     36.68
2 Grammar Cache Warming            0.00%       421         612     37.41
3 Tool Definition Forward          0.00%       198         341     49.34

Cumulative                      Resolved       -52%        -57%     1.43×
```

The peak was 40 req/s, but it can now handle 49.34 req/s, satisfying all SLOs.

### Results Summary

**Constrained decoding fundamentally eliminates probabilistic failures.** Prompt instructions, no matter how refined, always leave a few percent chance of failure, which is critical in cumulative structures like agents.

**Structured output can actually increase throughput.** Jump-forward decoding emits determined tokens without model calls, so JSON output with a high structural component is faster than unconstrained generation. If constraints are only seen as a cost, this benefit can be missed.

**Grammar compilation costs can be shifted.** If the number of schema types is finite, warming can remove the cost from user requests. If infinite, the backend must be chosen carefully.

**The advantage of MoE vs. dense depends on the workload.** In Case 1 (400-token output, 25 concurrency), dense was advantageous. Here (120-token output, 40 concurrency), MoE was advantageous. The criteria are how close the batch is to the threshold batch size and the output length.

<br>

## Large-scale MoE Serving

| Item | Value |
|---|---|
| Service | Public API. Mixed general conversation and coding |
| Request Rate | Average 80 req/s, Peak 140 req/s |
| Input | Average 3,000 tokens |
| Output | Average 800 tokens |
| Hardware | 16 nodes × 8×H200 = 128 GPUs, NVLink + InfiniBand NDR |
| SLO | TTFT p95 < 3,000ms, ITL Median < 60ms |
| Top Priority Metric | **Cost per 1 million output tokens** |

### Model Selection

| Model | Total/Active | FP8 Weights | Coding | Features |
|---|---|---:|---|---|
| DeepSeek-V4-Flash | 284B / 13B | 284 GB | Strong | MLA + Sparse Attention |
| GLM-5.2 | 744B / 40B | 744 GB | Best | MLA + DSA + IndexShare |
| Qwen3.5-397B-A17B | 397B / 17B | 397 GB | Strong | Hybrid Linear Attention |

While the choice depends, I will select **DeepSeek-V4-Flash**.

Its active parameters (13B) are the smallest among the candidates. The weights read per decode are 13GB (FP8), about one-third of GLM-5.2's 40GB. MLA's compression benefit for KV cache also helps with concurrent requests.

GLM-5.2 has higher coding performance, but its active 40B parameters would make the cost per token about 3 times higher. While a coding-specific service might lead to a different conclusion, for mixed traffic including general conversation, DeepSeek has a greater cost advantage.

### Required Throughput and Target Cost

```
Peak 140 req/s
  Input Throughput = 140 × 3,000 =  420,000 tok/s
  Output Throughput = 140 ×   800 =  112,000 tok/s

Target Cost
  H200 node approx. $32/hour (based on 8 GPUs, assuming cloud on-demand)
  16 nodes = $512/hour
  At peak sustained output 112,000 tok/s × 3,600 = 4.03e8 tok/hour
  → $512 / 403 (million tokens) ≈ $1.27 / 1M output tokens
```

The goal is to reduce tuning to this value.

### Round 0 - TP8 x 16 Replicas

Let's start with the simplest configuration: launch independent instances on each node and group them with a router.

```bash
# On each node
python -m sglang.launch_server \
  --model-path deepseek-ai/DeepSeek-V4-Flash \
  --tp-size 8 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --chunked-prefill-size 8192 \
  --mem-fraction-static 0.88 \ 
  --host 0.0.0.0 --port 30000 --enable-metrics

# Router
python -m sglang_router.launch_router \
  --worker-urls http://node{01..16}:30000 \
  --policy cache_aware --host 0.0.0.0 --port 8000
```

```
============ Serving Benchmark Result ============
Max request concurrency:                 1400
Successful requests:                     20000
Request throughput (req/s):              71.28
Input token throughput (tok/s):          213840.00
Output token throughput (tok/s):         57024.00
Concurrency:                            1288.44
---------------Time to First Token----------------
Median TTFT (ms):                        4218.90
P99 TTFT (ms):                          11840.22
---------------Inter-Token Latency----------------
Median ITL (ms):                          88.41
P99 ITL (ms):                            212.66
==================================================
```

```
Cost = $512 / (57,024 × 3600 / 1e6) ≈ $2.49 / 1M output tokens
```

Compared to the peak of 140 req/s, 71.28 req/s is half, which is insufficient. Both TTFT and ITL exceed their SLOs.

Looking at the metrics for a single node:

```
sglang_num_running_reqs  80.5
sglang_num_queue_reqs    7.4
sglang_token_usage       0.79
```

`token_usage` of 0.79 means there's headroom in the KV cache, but the queue is building up.

The bottleneck is **computation and communication**, not memory. When a 284B MoE is split with TP8, expert weights are fragmented across 8 GPUs. Regardless of which expert is selected, all 8 GPUs must participate in communication.

### Round 1 - Expert Parallelism and DP Attention

For MoE, assigning experts entirely to GPUs is better for communication.

For attention, data is parallelized to leverage MLA's compression benefits.

```bash
# Group 4 nodes into one instance (32 GPUs), total 4 instances
python -m sglang.launch_server \
  --model-path deepseek-ai/DeepSeek-V4-Flash \
  --tp-size 32 \
  --dist-init-addr node01:5000 \
  --nnodes 4 --node-rank $NODE_RANK \
  --enable-ep-moe \
  --ep-size 32 \
  --enable-dp-attention --dp-size 8 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --chunked-prefill-size 8192 \
  --mem-fraction-static 0.88 \
  --host 0.0.0.0 --port 30000 --enable-metrics
```

```
First, determine how to split 128 GPUs.

Total        16 nodes × 8 GPUs = 128 GPUs        (5.1 hardware)
Instance     4 nodes × 8 GPUs =  32 GPUs        (Unit serving one model)
Number of Instances  128 ÷ 32      =   4
```

4 nodes are grouped because the model weights are 284GB with FP8. One H200 has 141GB, so at least 3 GPUs are needed for weights, leaving almost no KV cache headroom. To distribute 256 experts, enough GPUs are needed. 32 GPUs allow for an even distribution of 8 experts per GPU.

Why not group larger? If instances are larger, the all-to-all communication scope widens, and more InfiniBand traffic occurs between nodes. With 32 GPUs across 4 nodes, a significant portion of communication is handled by NVLink within the nodes.

-   `--nnodes 4`: Specifies how many nodes are grouped into one instance. 4 nodes = 32 GPUs serving a single model.
-   `--node-rank $NODE_RANK`: Specifies the rank of this node among them (0, 1, 2, 3).
-   `--dist-init-addr node01:5000`: Rendezvous point for nodes to find each other. Node 0 acts as the rendezvous point.
-   `--tp-size 32`: The size of the entire parallel world, referring to all 32 GPUs. The two arguments below divide this.
-   `--enable-ep-moe`: Processes MoE layers with expert parallelism. If not enabled, a single weight matrix is fragmented into 32 pieces across 32 GPUs, so all 32 GPUs are involved in computation regardless of which expert is chosen. If enabled, experts are kept whole on one GPU, so only the GPU holding that expert works.
-   `--ep-size 32`: How many branches to assign experts to. 32 GPUs divide 256 experts, handling 8 each (256 / 32).
-   `--enable-dp-attention`: Processes only attention layers with data parallelism. Prevents replication of MLA-compressed KV.
-   `--dp-size`: How many branches to divide experts into. 8 groups each handle a different set of requests, with 4 GPUs per group (32 / 8).

The structure where `--tp-size` is 32, and EP and DP are contained within it, can be confusing at first. It means that the 32 GPUs are used differently depending on the layer type.

```
Attention Layer (DP=8)
  GPU 0~3   : Fully processes attention for request group A (holds its own KV)
  GPU 4~7   : Request group B
  ...        Each group is independent. No communication between groups.

MoE Layer (EP=32)
  GPU 0     : Handles experts 0~7
  GPU 1     : Handles experts 8~15
  ...
  GPU 31    : Handles experts 248~255
             Sends tokens to the responsible GPU (all-to-all) and receives results back.
```

-   **Why separate attention:** DeepSeek models store KV cache compressed with MLA. If attention is split with TP, each GPU redundantly stores compressed KV, negating the memory savings from compression. With DP, each GPU only needs to hold the KV for the requests it handles.
-   **How communication changes with EP for MoE:** TP approach had high overhead due to all-to-all communication across all 32 GPUs at each layer, as expert fragments were scattered. A token uses only 8 experts, but all 32 GPUs participate. EP only performs all-to-all to send tokens to the responsible GPU and receive them back. The communication involves tokens, not weights, and the scope of participation is narrower.

```
                     Round 0    Round 1     Change
req/s                  71.28     118.44     +66.2%
Output tok/s          57024      94752      +66.2%
Median TTFT (ms)      4218.90    2814.33    -33.3%
P99 TTFT (ms)        11840.22    6120.88    -48.3%
Median ITL (ms)         88.41      61.20    -30.8%
Cost ($/1M Output)         2.49       1.50     -39.8%
```

#### Throughput increased by 66.2%

With expert parallelism, GPUs now handle only 8 experts each, reducing weight memory per GPU. This increased available KV cache space, leading to more concurrent requests. Communication also shifted from full all-reduce at each layer to all-to-all for sending tokens to selected experts, reducing total communication.

#### Latency TTFT improved by 33.3%, ITL by 30.8%

Thanks to DP attention, KV replication was eliminated, increasing the number of requests a GPU can process and reducing queue wait times, thus improving TTFT. ITL improvement is a direct result of reduced total communication.

#### Remaining Problem - Expert Load Imbalance

```bash
curl -s http://node01:30000/metrics | grep expert_load | sort -t= -k2 -rn | head -5
```
```
sglang_expert_load{expert="47"}  0.0281
sglang_expert_load{expert="192"} 0.0264
sglang_expert_load{expert="8"}   0.0247
...
sglang_expert_load{expert="231"} 0.0009
```
If balanced, each of the 256 experts should have a load of 0.00039. The maximum is 0.0281, indicating a 7x imbalance. The GPU handling the popular expert is the bottleneck.

Simply put, too many requests are directed to certain experts, burdening the GPU holding those expert weights.

### Round 2 - Expert Load Balancing

#### EPLB

EPLB stands for Expert Parallelism Load Balancer, a mechanism that recalculates how experts are assigned to GPUs.

In Round 1, experts were simply assigned sequentially.

```
GPU 0 : Experts 0~7      GPU 1 : Experts 8~15      ...      GPU 31 : Experts 248~255
```

This assumes all experts are chosen with similar frequency, which was not the case. Expert 47 was chosen 7.2 times more often than average. GPU 5, handling expert 47, was constantly busy, while GPU 28, handling expert 231, was idle.

**Since the MoE layer cannot proceed to the next layer until all GPUs are finished, the slowest GPU determines the overall speed.**

EPLB observes actual routing statistics and determines two things:

```
1. Replicate popular experts.
   Place expert 47 on both GPU 5 and GPU 19.
   → Tokens destined for expert 47 are split between two GPUs.

2. Recalculate assignments.
   Rearrange experts to prevent popular experts from being clustered on the same GPU.
```

Thus, over time, EPLB recalculates and modifies the expert assignment configuration for routing. Initially, it starts with a fixed configuration and gradually changes.

> Imagine bank tellers assigned numbers sequentially. It turns out 7 times more customers come for service #3 than other services. Teller #3 has a long line, while others are idle. EPLB would then increase the number of windows for service #3 to two and reallocate other popular services to prevent them from clustering.

Enable EPLB to replicate popular experts across multiple GPUs.

```diff
    --enable-ep-moe \
    --ep-size 32 \
+   --enable-eplb \
+   --eplb-algorithm deepseek \
+   --eplb-rebalance-num-iterations 500 \
+   --ep-num-redundant-experts 32 \
```

-   `--enable-eplb`: Enables the load balancer. If disabled, it reverts to a fixed assignment like Round 1.
-   `--eplb-algorithm deepseek`: The algorithm for recalculating assignments, based on DeepSeek's published method. It prioritizes intra-node communication by considering hierarchical structure.
-   `--eplb-rebalance-num-iterations 500`: How often to recalculate assignments. For 500, it collects routing statistics for 500 steps before making a decision. Too short, and statistics fluctuate; too long, and it's slow to adapt to traffic changes.
-   `--ep-num-redundant-expert 32`: The number of replicas to create. Adding 32 means the expert slots increase from 256 to 288, and 32 popular experts will exist in two locations.

```
                     Round 1    Round 2     Change
req/s                 118.44     139.86     +18.1%
Output tok/s           94752     111888     +18.1%
Median TTFT (ms)      2814.33    2402.11    -14.6%
Median ITL (ms)         61.20      52.44    -14.3%
Cost ($/1M Output)         1.50       1.27     -15.3%
```

```
Expert Load Imbalance (Max/Avg)
  Round 1: 7.2x
  Round 2: 1.9x
```

Adding 32 redundant experts increased weight memory by 12.5% (32 / 256), but resolving load imbalance boosted throughput by 18%.

Peak 140 req/s was almost reached at 139.86 req/s.

### Round 3 - PD Disaggregation

Prefill and decode have opposite optimal settings. Prefill is compute-bound, so smaller TP with multiple instances is advantageous. Decode is memory-bound, so larger EP and larger batches are advantageous. Combining them into one instance means compromising both.

Estimate prefill and decode load based on the input/output ratio of 3,000:800, then allocate nodes.

```
Prefill Load = 420,000 tok/s (peak)
Decode Load = 112,000 tok/s (peak)

Prefill is compute-heavy per token, decode is memory-access-heavy per token.
Initial allocation based on measurements: 5 nodes for prefill / 11 nodes for decode
```

```bash
# Prefill nodes (5 nodes, each TP8)
python -m sglang.launch_server \
  --model-path deepseek-ai/DeepSeek-V4-Flash \
  --tp-size 8 \
  --disaggregation-mode prefill \
  --disaggregation-transfer-backend mooncake \
  --disaggregation-bootstrap-port 8998 \
  --disaggregation-ib-device mlx5_0 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --chunked-prefill-size 16384 \
  --mem-fraction-static 0.85 \
  --host 0.0.0.0 --port 30000 --enable-metrics

# Decode nodes (11 nodes as a single EP domain, 88 GPUs)
python -m sglang.launch_server \
  --model-path deepseek-ai/DeepSeek-V4-Flash \
  --tp-size 88 --nnodes 11 --node-rank $NODE_RANK \
  --dist-init-addr node06:5000 \
  --disaggregation-mode decode \
  --disaggregation-transfer-backend mooncake \
  --enable-ep-moe --ep-size 88 \
  --enable-dp-attention --dp-size 22 \
  --enable-eplb --eplb-algorithm deepseek \
  --ep-num-redundant-experts 88 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --max-running-requests 2048 \
  --mem-fraction-static 0.90 \
  --host 0.0.0.0 --port 30000 --enable-metrics

# Router
python -m sglang_router.launch_router \
  --pd-disaggregation \
  --prefill http://node01:30000 8998 \
  --prefill http://node02:30000 8998 \
  --prefill http://node03:30000 8998 \
  --prefill http://node04:30000 8998 \
  --prefill http://node05:30000 8998 \
  --decode http://node06:30000 \
  --prefill-policy cache_aware \
  --decode-policy power_of_two \
  --host 0.0.0.0 --port 8000
```

#### PD Disaggregation Specific Arguments

These are common to both node types.

-   `--disaggregation-mode prefill / decode`: Declares the role of this instance. Prefill nodes create and pass KV cache, then finish. Decode nodes receive KV and only generate.
-   `--disaggregation-transfer-backend mooncake`: Specifies the KV transfer mechanism between nodes. Mooncake Transfer Engine uses RDMA to directly transfer between GPU memories, bypassing the CPU. There's also `nixl`.
-   `--disaggregation-bootstrap-port 8998`: The port where decode nodes connect to prefill nodes. Specified only on the prefill side and included in the router settings like `--prefill http:/node01:30000 8998` after the serving port.
-   `--disaggregation-ib-device mlx5_0`: Specifies which InfiniBand device to use for transfer. Use the name found with `ibv_devinfo`. If not specified, it falls back to Ethernet, becoming a transfer bottleneck.

#### Prefill Node

-   `--tp-size 8`: Each node is an independent instance. Prefill involves thousands of tokens, already saturating the GPU, so widening parallelism offers no gain and only increases communication costs. Instead, 5 nodes handle different requests to increase throughput.
-   `--chunked-prefill-size 16384`: Increased from 8192 in Round 2. This node does not decode, so there's no concern about long prefill blocking other token generation. Larger chunks improve matrix operation efficiency.
-   `--mem-fraction-static 0.85`: Set lower than decode nodes. KV on prefill nodes is created and passed, so it doesn't need to be stored long. Instead, activations during prefill can be large, so headroom is reserved.
-   **No EP DP arguments:** Prefill has small batches, so the benefits of large-scale EP are not realized, and it only increases all-to-all communication costs, so it's not included.

#### Decode Node

-   `--tp-size 88 --nnodes 11`: Groups 11 nodes into one. Decode is memory-bound, so batch size must be maximized. To maximize batch size, more KV cache space is needed. Grouping more nodes reduces weight burden per GPU, increasing space for KV.
-   `--ep-size 88`: Divides 256 experts among 88 GPUs. This means about 3 experts per GPU, further reducing weight burden compared to Round 2 (8 experts).
-   `--dp-size 22`: 88 / 22, 4 GPUs per group, same ratio as Round 2.
-   `--ep-num-redundant-experts 88`: Increased replicas to match EP size. The impact of imbalance grows with rank count.
-   `--max-running-requests 2048`: Upper limit for concurrent sequences. Since it's decode-only, there's no contention with prefill, so it's set high.
-   `--mem-fraction-static 0.90`: Maximizes KV cache. The performance of this node directly relates to the number of requests it can hold.

The key is assigning different routing policies to prefill and decode. Prefill benefits from prefix reuse, so `cache_aware` is used. Decode prioritizes load balancing over caching, so `power_of_two` is used.

`--prefill-policy` and `--decode-policy` are rules for deciding which node to send a request to. Four options are available:

-   `round_robin`: Assigns nodes in a round-robin fashion.
-   `random`: Selects one randomly.
-   `cache_aware`: Hashes the request prefix and always sends the same prefix to the same node.
-   `power_of_two`: Picks two random nodes and selects the less busy one.

If only `--policy A` is used, the same value applies to both prefill and decode. If separated, they can be set differently.

**Prefill** uses `cache_aware` because prefill nodes create KV. If the same prefix's KV is already created, it can be reused. Hashing prefixes and sending them to the same node increases the probability of a cache hit. If scattered by round-robin, the same system prompt would be redundantly created on 5 nodes, effectively reducing cache capacity to one-fifth.

**Decode** uses `power_of_two` because decode nodes don't create KV but receive it from prefill. Whether prefixes are the same or different, there's no cache to reuse, making `cache_aware` meaningless. Instead, which node is less busy is important. While checking the load of all 11 nodes every time would burden the router, comparing just two random nodes provides sufficient balance.

> Why is checking just two enough? If you pick one randomly, you might unluckily hit a busy node. Picking two and choosing the less busy one reduces that probability quadratically. Increasing to three or four nodes yields rapidly diminishing returns.

#### Comparison of Two Group Settings

The same model, but with diametrically opposite settings.

```
                      Prefill              Decode
Bottleneck                  Compute                Memory Bandwidth
TP Size               8 (per node)        88 (entire group)
EP                    Not Used           88 ranks
Chunk Size             16,384 (large)        N/A
mem-fraction          0.85                0.90
Optimization Goal           TTFT                Throughput
```

```
                     Round 2    Round 3     Change
req/s                 139.86     198.20     +41.7%
Output tok/s          111888     158560     +41.7%
Median TTFT (ms)      2402.11    1884.66    -21.5%
P99 TTFT (ms)         5218.40    3402.18    -34.8%
Median ITL (ms)         52.44      44.90    -14.4%
Cost ($/1M Output)         1.27       0.90     -29.1%
```

### Round 4 - Node Allocation Readjustment

Check the utilization of prefill and decode nodes separately.

```bash
# Prefill node
curl -s http://node01:30000/metrics | grep -E 'num_queue_reqs|token_usage'
# sglang_num_queue_reqs 18.2      ← Queue is building up
# sglang_token_usage 0.42

# Decode node
curl -s http://node06:30000/metrics | grep -E 'num_queue_reqs|token_usage'
# sglang_num_queue_reqs 0.4
# sglang_token_usage 0.61         ← Has headroom
```

The queue is building up on the prefill side, while the decode side is idle. Shift allocation towards prefill.

```
Allocation              req/s    Median TTFT   Median ITL   Cost($/1M)
Prefill 5 / Decode 11   198.20      1884         44.9        0.90
Prefill 6 / Decode 10   214.44      1502         48.2        0.83
Prefill 7 / Decode 9    218.90      1344         56.1        0.81
Prefill 8 / Decode 8    206.12      1288         71.4        0.86   ← ITL SLO Exceeded
```

Prefill 7, Decode 9 is optimal. 8/8 actually reduces throughput, and median ITL of 71.4ms exceeds the SLO of 60ms.

This table shows the core advantage of PD disaggregation: the balance between TTFT and ITL can be adjusted with a single node allocation ratio. In a unified configuration, this adjustment knob is absent.

### Final Configuration and Cumulative Effects

```
Round                        req/s   Median TTFT   Median ITL   Cost($/1M)
0 TP8 × 16 Replicas               71.28       4219         88.4        2.49
1 EP + DP Attention             118.44       2814         61.2        1.50
2 EPLB Load Balancing             139.86       2402         52.4        1.27
3 PD Disaggregation (5/11)             198.20       1884         44.9        0.90
4 Allocation Readjustment (7/9)           218.90       1344         56.1        0.81

Cumulative                        3.07×        -68%        -37%       -67%
```

SLO Results
1.  TTFT p95 < 3,000ms was achieved at approximately 2,410ms.
2.  ITL median < 60ms was also achieved at 56.1ms.
3.  req/s, which was 140, was comfortably achieved at 218.9 req/s based on peak.
4.  Cost also decreased by 67% from $2.49 to $0.81, with the hardware remaining at 128 GPUs.

### Lessons Learned

1.  **Parallelization strategy dominates cost in large MoE models**: The difference between a TP-only configuration and an EP + PD disaggregation configuration on the same hardware is 3x.
2.  **Expert load imbalance must be measured**: Ignoring a 7x imbalance means the GPU handling the popular expert determines the overall speed. `expert_load` metric should be a constant monitoring item.
3.  **The true value of PD disaggregation is the adjustment knob.** While throughput improvement is significant, the ability to adjust the balance between TTFT and ITL post-deployment by changing node allocation ratios is more useful in operations. If traffic patterns change, only the ratio needs adjustment.
4.  **PD disaggregation is conditional on scale.** This case involved 128 GPUs, InfiniBand NDR, and Mooncake transfer layer. If a configuration like 8 GPUs were used, both pools would be inefficient, making it slower.

<br>

## Five Cases Comparison

### The same techniques are used, but optimization points can improve or worsen depending on the characteristics.

| Technique | Case 1 (Coding) | Case 2 (RAG) | Case 3 (Batch) | Case 4 (Structured) | Case 5 (Large-scale) |
|---|---|---|---|---|---|
| Speculative Decoding | **+66%** | Minimal benefit | **-20%** | Not applied | Not applied |
| Chunked Prefill Size | 4,096 | 8,192 | **32,768** | 8,192 | 16,384 |
| Model Type | **dense 27B** | dense 27B | dense 9B | **MoE 35B-A3B** | MoE 284B-A13B |
| TP Size | 2 (4 replicas) | 2 (2 replicas) | 2 | 2 | 8 / 88 |
| `mem-fraction-static` | 0.90 | 0.88 | **0.94** | 0.90 | 0.85 / 0.90 |
| HiCache | Unnecessary | **Essential** | Unnecessary | Unnecessary | Under review |
| PD Disaggregation | Detrimental | Detrimental | Detrimental | Detrimental | **+42%** |

Optimization differs for each workload's input/output cost, concurrency, sharing rate, scale, etc.

```
Input/Output Ratio
  Input ≫ Output (Case 2: 68:1)
    → Focus on prefill optimization. Cache/routing dominates.
    → Minimal benefit from speculative decoding.
  Input ≈ Output (Case 1: 5:1)
    → Balanced. Chunked prefill to prevent interference.
  Output dominant
    → Decode optimization. Quantization/speculative decoding.

Concurrency vs. Threshold Batch Size
  Concurrency ≪ Threshold Batch (Case 1: 25 vs 206)
    → Memory-bound. Computational resources available.
    → Speculative decoding effective, dense advantageous.
  Concurrency ≈ Threshold Batch (Case 3: 256 vs 295)
    → Transition zone. Speculative decoding detrimental.
    → MoE advantageous, batch maximization.

Prefix Sharing Rate
  < 0.2  → No RadixAttention benefit.
  > 0.6  → Routing policy dominates performance. Prompt structure review essential.

GPU Scale
  < 16   → Single node tuning. PD disaggregation detrimental.
  > 64   → EP + PD disaggregation review zone.
```

### Metric Change Patterns

| Observed Change | Meaning | Next Action |
|---|---|---|
| Throughput ↑, Latency ↑ | Effect of increasing batch size | Maintain if SLO has headroom |
| Throughput ↑, Latency ↓ | Bottleneck itself removed | Correct direction. Push further. |
| Throughput ↓, Tail Latency ↓↓ | Trade-off between stability and efficiency | Maintain if latency SLO exists |
| Median unchanged, P99 only ↓ | Only tail requests improved | Typical pattern for HiCache/warming |
| Throughput ↓, Latency ↑ | Wrong direction | Rollback immediately |
| Nothing changed | Bottleneck is elsewhere | Re-examine bottleneck with metrics |

### Summary of Final Settings by Case

```bash
# Case 1 — In-house Coding Assistant (4 workers)
--model-path Qwen/Qwen3.6-27B --tp-size 2
--quantization fp8 --kv-cache-dtype fp8_e5m2
--chunked-prefill-size 4096 --mem-fraction-static 0.90
--speculative-algorithm EAGLE3 --speculative-num-steps 5 --speculative-eagle-topk 8
# Router: --policy cache_aware

# Case 2 — Document QA / RAG (2 workers)
--model-path Qwen/Qwen3.6-27B --tp-size 2
--quantization fp8 --kv-cache-dtype fp8_e5m2
--chunked-prefill-size 8192 --mem-fraction-static 0.88
--page-size 64 --enable-hierarchical-cache --hicache-ratio 2
--hicache-write-policy write_through --hicache-io-backend kernel
--enable-cache-report
# Router: --policy cache_aware

# Case 3 — Nightly Batch
--model-path Qwen/Qwen3.6-9B --tp-size 2
--quantization fp8 --kv-cache-dtype fp8_e5m2
--chunked-prefill-size 32768 --mem-fraction-static 0.94
--max-running-requests 512 --max-prefill-tokens 65536
--schedule-conservativeness 0.3
# Speculative decoding not used

# Case 4 — Structured Output API
--model-path Qwen/Qwen3.6-35B-A3B --tp-size 2
--quantization fp8 --kv-cache-dtype fp8_e5m2
--grammar-backend xgrammar
--chunked-prefill-size 8192 --mem-fraction-static 0.90
--max-running-requests 64 --enable-cache-report
# Grammar cache warming essential after startup

# Case 5 — Large-scale MoE (7 prefill nodes / 9 decode nodes)
# Prefill
--tp-size 8 --disaggregation-mode prefill
--disaggregation-transfer-backend mooncake --disaggregation-ib-device mlx5_0
--chunked-prefill-size 16384 --mem-fraction-static 0.85
# Decode
--tp-size 72 --nnodes 9 --disaggregation-mode decode
--enable-ep-moe --ep-size 72 --enable-dp-attention --dp-size 18
--enable-eplb --eplb-algorithm deepseek --ep-num-redundant-experts 72
--max-running-requests 2048 --mem-fraction-static 0.90
# Router: --pd-disaggregation --prefill-policy cache_aware --decode-policy power_of_two
```
