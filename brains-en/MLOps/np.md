# Parallelization Strategies — TP, PP, EP, DP

Let's assume we're deploying a model on a single GPU. We'll soon encounter the following problems:

1.  **Weights don't fit**: For instance, DeepSeek-V4-Flash is 284GB in FP8, but a single H200 has 141GB. The load itself fails.
2.  **Even if it fits, no KV cache:** Qwen3.6-27B is 27GB, so it fits into 141GB, but how many users can the remaining 114GB accommodate? If the KV cache for one user with a 32K context is 6GB, it can accommodate 19 users. This is insufficient for an in-house service used by 200 people.
3.  **Slow speed per user:** To generate a single token, the entire 27GB must be read, which takes 5.6ms with an H200 bandwidth of 4.8TB/s.

For problem #3, if we use four GPUs, each only needs to read 6.75GB, reducing the time to about 1.4ms.

However, splitting isn't free; the divided pieces must synchronize their computation results, and communication takes time.

```
스텝당 통신량 41 MB일 때
  같은 노드 안 (NVLink 1,800 GB/s)   → 0.023 ms
  노드를 넘어 (InfiniBand 50 GB/s)   → 0.82 ms
```

If a single decode step takes several milliseconds, and 0.82ms is spent just on communication, the benefits gained from splitting can disappear.

In other words, **the communication volume and timing vary depending on how you split, leading to four different methods.**

```
자르는 대상        약어    통신 시점
─────────────────────────────────────
행렬 조각내기      TP     층마다
층을 깊이로 자르기  PP     구간 경계
전문가 통째로 배정  EP     MoE 층마다
안 자르고 요청 분배 DP     없음
```

Today, let's explore these one by one.

<br>

## TP Tensor Parallelism

This method involves splitting a matrix, having multiple GPUs compute parts of it, and then combining the results.

```
Original:   Y = X @ W          W has size [5120 × 20480]

TP=4:   W is vertically divided into 4 parts → W1, W2, W3, W4  (each [5120 × 5120])
        GPU0 computes X @ W1
        GPU1 computes X @ W2
        GPU2 computes X @ W3
        GPU3 computes X @ W4
        → Combine the four results to complete Y          ← Communication happens here
```

Attention is split by head. Four heads are distributed across four cards, 16 per card. Attention originally uses multiple heads to view sentences from different perspectives, and the heads are independent of each other. If there are 64 heads, they can be divided among four cards, 16 per card.

> Note - The number of heads must be divisible by the TP size. If you give TP=16 to a model with 8 heads, execution will fail.

### Advantages

This way, the computation for generating a single token is distributed across multiple GPUs, **making the perceived speed faster for individual users.**

The interval between tokens is called **ITL (Inter-Token Latency)**, and this value is directly reduced.

### Trade-off

Since communication occurs at each layer, if a model has 80 layers, a single token involves 80 communications.

```
When communication per step is approx. 41 MB
  NVLink (1,800 GB/s)     →  0.023 ms
  InfiniBand (50 GB/s)    →  0.82 ms
```

It takes several milliseconds to generate a single token, but if 0.82ms is spent just on communication, the benefits gained from parallelization are lost due to communication latency.

How should `--tp-size` be set?

Assuming there are 8 GPUs, there are two options.

```
Option A: One TP8 instance
  8 cards form one team to handle all requests
  When a request comes in, 8 cards together generate that token

Option B: Four TP2 instances
  Paired in twos, four independent teams
  When a request comes in, the router assigns it to one team
  Each team handles only its own requests
```

Let's say we actually measure it and switch from A to B.

```
                  TP8 × 1     TP2 × 4      Change
Total Throughput        3,285       4,419      +34.5%
Time to First Token     511 ms      362 ms     -29.2%
Inter-Token Latency        12.6 ms     17.9 ms    +42.0%
```

**Throughput and time to first token improve, while inter-token latency worsens.**

#### Why throughput increases

1. **Fewer communication partners:** TP8 needs to synchronize results from 8 cards at each layer, but TP2 only needs to synchronize 2 cards, meaning fewer partners to exchange data with, thus finishing faster.
2. **Easier to fill GPUs:** GPUs are efficient when processing multiple requests in batches. With TP8, there's only one queue, so the batch size only grows when requests pile up. With four TP2 instances, there are four queues, and each fills its batch independently, allowing other teams to continue working while one team handles a heavy request.

#### **Why the first token is faster**
**Because the queue is divided into four.** With TP8, if someone is processing a long request by pasting an entire file, shorter questions behind it also have to wait. But with four TP2 instances, that long request only ties up one team, while the other three teams can receive other requests.

#### Why inter-token latency worsens

**Reason for worse ITL:** The number of GPUs dedicated to a single token decreases from 8 to 2. As calculated earlier, if weights are divided into 8 parts, each GPU reads 3.4GB. If divided into 2 parts, each reads 13.5GB. Since the amount to read is four times greater, the time to generate a single token increases.

### Criteria for selection

Let's first define the response time required by the service.

```
Human reading speed is about 10-15 tokens per second
-> If inter-token latency is below 50ms (20 tokens/second), it feels faster than reading.

In the measurements above, TP2 x 4 is 17ms
-> Since there's ample margin against the 50ms criterion, choose TP2 and prioritize throughput.

If the criterion is strict, like 15ms
-> Map 17.9ms to TP2 and choose TP4, TP8.
```

- **If response time criteria are strict** -> Increase TP, allocate more GPUs to a single request.
- **If there's flexibility in response time** -> Decrease TP and increase instances to serve more customers with the same hardware.
- **Unsure** -> Launch both and measure;

`--tp-size 2` (Launch four instances on different GPUs and group them with a router.)

<br>

## PP - Pipeline Parallelism

Transformer models stack layers of identical structure vertically.

```
입력 토큰
   ↓
[레이어 1]   ← 어텐션 + FFN. 각각 큰 가중치 행렬을 가짐
   ↓
[레이어 2]   ← 같은 구조, 다른 값
   ↓
  ...
   ↓
[레이어 80]
   ↓
출력 토큰
```

Tokens pass through layers sequentially, and the input for Layer 5 cannot skip the output of Layer 4.

```
        레이어 1  ─────────────────────────
        레이어 2  ─────────────────────────
        레이어 3  ─────────────────────────    ← PP: 여기를 가로로 자름
        레이어 4  ─────────────────────────
          ...
        레이어 80 ─────────────────────────
                  │    │    │    │
                  └────┴────┴────┘
                  TP: 각 레이어를 세로로 자름
```

TP slices within layers, while PP slices between layers.

PP slices horizontally as shown below.

```
GPU0            GPU1            GPU2            GPU3
[레이어 1~20]  → [레이어 21~40] → [레이어 41~60] → [레이어 61~80]
                ↑               ↑               ↑
              통신 1회         통신 1회         통신 1회

각 GPU가 자기 레이어만 온전히 가짐
토큰 하나 만드는 데 통신 3번
```

This reduces the communication count to about 3 times instead of 80. The amount of data exchanged also differs.

TP needs to aggregate calculation results for each layer, but PP only needs to pass a single intermediate tensor after passing through 20 layers.

### Why it's not commonly used in inference

Even though communication is much less, there's a reason PP isn't the primary method.

#### PP GPUs work sequentially.

```
토큰 하나를 만드는 과정

시간 →
GPU0: [L1~20 계산]
GPU1:              [L21~40 계산]
GPU2:                            [L41~60 계산]
GPU3:                                          [L61~80 계산]

GPU0이 일할 때 GPU1,2,3은 놀고 있음
```

While TP's 4 GPUs work simultaneously, PP's 4 GPUs work sequentially, so only one GPU is busy at any given time.

### How to mitigate this

If multiple requests are streamed in an overlapping manner, the idle slots are filled.

```
시간 →
GPU0: [요청1][요청2][요청3][요청4][요청5]
GPU1:       [요청1][요청2][요청3][요청4]
GPU2:             [요청1][요청2][요청3]
GPU3:                   [요청1][요청2]
      └버블┘                        └버블┘
       처음                          끝
```

In the middle section, all 4 GPUs are busy, and the idle sections at the beginning and end are called **pipeline bubbles**.

> Analogy: In a factory assembly line, if only one product is made, the subsequent processes remain idle until the preceding one finishes. All processes must run simultaneously by continuously feeding products.

### Why it works in training but is difficult in inference

In training, all data to be processed is available beforehand, so if batches are finely split and continuously pushed into the pipeline, bubbles can be reduced to a few percent of the total. However, in **inference**, requests can arrive at any unpredictable time.

If a single user connects at 3 AM, there are no other requests to push into the pipeline, and only 1 out of 4 GPUs works. If it were TP, that single request would have been processed by 4 GPUs, making it 4 times faster.

Also, the output length varies for each request. While one request generates 500 tokens, another might finish at 50 tokens, constantly disrupting the pipeline flow.

Therefore, the situations where PP is chosen are as follows.

```
모델이 기계 한 대(GPU 8장)에 안 들어간다
        ↓
기계를 넘어가야 하는데 TP는 통신 때문에 안 됨
        ↓
선택지 두 개
  A. PP로 레이어를 나눠 기계 간 확장
  B. MoE 모델이라면 EP로 전문가를 나눔  ← 대개 이쪽이 나음
```

**PP is effective when an ultra-large dense model (not MoE) needs to be deployed across multiple machines, or during training.**

Since all data to be processed for training is **available beforehand**, a single batch can be finely split into micro-batches and continuously pushed into the pipeline.

```
배치 1024개를 마이크로배치 64개로 분할, P=4

버블 = 3 / (64 + 3) = 4.5%
```

All 4 GPUs work for 95.5% of the time.

There's another advantage unique to training. **Backpropagation fills the pipeline once more.** While forward propagation flows from GPU0 to GPU3, backpropagation flows backward from GPU3 to GPU0, so the two flows fill each other's gaps.

```
GPU3: [순전파 4][역전파 4][순전파 8][역전파 8] ...
GPU0: [순전파 1][순전파 5][역전파 1][순전파 9] ...
       ↑ 순전파와 역전파가 교차하며 빈자리를 채움
```

Inference does not have backpropagation, so this advantage is absent.

#### Prefill

Prefill is the stage where the entire prompt entered by the user is read at once, so a 3,000-token request processes all 3,000 tokens simultaneously.

A large number of tokens means there's more to split. If 3,000 tokens are divided into 6 chunks of 512 tokens each, with P=4, then:

```
버블 = 3 / (6 + 3) = 33%
```

33% is much higher than the 4.5% bubble rate in training, due to the smaller number of chunks. However, in a real service, multiple requests come in, so M increases.

```
동시 요청 20개 × 요청당 6조각 = M 120

버블 = 3 / (120 + 3) = 2.4%
```

This means PP can be useful in prefill if there's sufficient traffic. The problem is that performance rapidly deteriorates with low traffic.

#### Decode

Decode is the stage where tokens are generated one by one, revealing a fundamental limitation of PP.

**Consecutive tokens of the same request cannot overlap.**

```
요청 A의 100번째 토큰을 만드는 중

GPU0: [L1~20] → GPU1: [L21~40] → GPU2: [L41~60] → GPU3: [L61~80]
                                                          ↓
                                                    100번째 토큰 완성
                                                          ↓
                                        이제야 101번째를 시작할 수 있음
```

Since the input for the 101st token is the 100th token, the 100th token must fully pass through GPU3 before the 101st can be fed into GPU0. To fill the pipeline, other requests are needed.

This leads to a divergence in two metrics.

```
동시 요청 40개일 때

처리량   : 40개 요청이 각 단계를 채움 → 4장이 다 바쁨. 괜찮음
토큰 간 간격 : 요청 하나의 토큰 하나가 GPU0→1→2→3을 다 거쳐야 함
              = 전체 계산 시간 + 전송 3회
              → 1장이 모델 전체를 갖고 있을 때보다 오히려 느림
```

**PP does not improve the speed of a single request at all**, because calculations are done sequentially, not in parallel.

TP uses 4 GPUs to calculate simultaneously, generating one token 4 times faster. PP, however, processes 0 layers sequentially across 4 GPUs, ultimately taking the full time equivalent to 80 layers.

Here, transmission latency is added 3 more times.

For MoE models, EP is better than PP. EP has no bubbles because GPUs work simultaneously even when experts are divided.

<br>

## EP - Expert Parallelism

### MoE

Standard models use all weights for every token, but MoE employs hundreds of FFN modules, called experts, and selects only a few for each token.

```
MoE 레이어 하나의 내부

        [토큰]
           ↓
       [라우터]  ← 점수 계산 → 상위 8개 선택
           ↓
   ┌──┬──┬──┬──┬─ ... ─┬──┐   전문가 256개
   │  │██│  │██│       │  │   (██ = 이번에 켜진 것)
   └──┴──┴──┴──┴───────┴──┘
           ↓
       [결과 합산]
```

DeepSeek-V4-Flash uses 8 out of 256 experts, with only 13B parameters participating in computation out of a total of 284B.

### What EP Does

Assigns experts entirely to GPUs.

```
EP=32
  GPU0:  전문가 0~7     GPU1: 전문가 8~15   ...   GPU31: 전문가 248~255

토큰이 전문가 17, 93, 201... 8개 선택
  ↓ all-to-all — 각 토큰을 담당 GPU로 전송
  ↓ 각 GPU가 자기 전문가로 계산
  ↓ all-to-all — 결과를 원래 자리로 회수
```

Compared to TP, if the same MoE layer is divided using TP, it looks like this:

```
TP=32
  전문가 0의 가중치를 32조각으로 잘라 32장에 분산
  전문가 1의 가중치를 32조각으로 잘라 32장에 분산
  ... 256개 전부 그렇게

토큰이 전문가 8개를 선택
  → 그 8개의 조각이 32장에 흩어져 있음
  → 32장 전부가 계산에 동원됨
  → 레이어마다 32장 전체 all-reduce
```

While tokens actually use 8 out of 256 experts, TP involves all 32 GPUs.

MoE is designed to activate only a subset of experts, but TP effectively nullifies this advantage.

```
              TP=32                    EP=32
전문가 배치    조각내서 전 GPU에         통째로 한 GPU에
동원되는 GPU   항상 32장                 선택된 8개를 가진 곳만
오가는 것      계산 결과 (all-reduce)    토큰 (all-to-all)
GPU당 가중치   256개 전부의 1/32         8개 전체
```

The last line is crucial: the total amount is similar, but the nature of the weights differs.

### Advantages

#### KV Cache Space

```
GPU 한 장의 141GB를 어떻게 쓰는가

TP=32:  가중치 조각 + 활성값 + KV 캐시
EP=32:  가중치 조각 + 활성값 + KV 캐시
        ↑ 총량은 비슷하지만
```

The real difference comes from communication. With TP, all 32 GPUs must synchronize results after each layer, and GPUs cannot compute during this communication.

EP has a narrower scope of participation and smaller data transfer, which reduces this waiting time.

In actual measurements, throughput increased by 66% compared to a TP-only configuration, and public benchmarks report up to a 5x improvement.

### Trade-off: All-to-All

Every GPU communicates with every other GPU because it's impossible to know in advance which expert a token will be routed to.

```
32장이 서로에게 → 32 × 31 = 992개의 통신 경로
```

This is the biggest bottleneck in MoE serving and why dedicated communication libraries like DeepEP have emerged.

#### Load Imbalance

Theoretically, all 256 experts should be selected uniformly, but in practice, this is not the case.

```
실측 예시
  전문가 47번  : 평균의 7.2배
  전문가 231번 : 평균의 0.23배
```

The GPU assigned to expert 47 remains constantly busy, while the GPU assigned to expert 231 remains idle.

Furthermore, an MoE layer cannot proceed to the next until all GPUs have finished, meaning the slowest GPU determines the overall speed.

```
GPU5  (47번 담당): ████████████████  ← 이 시간만큼 전원 대기
GPU28 (231번 담당): ██
                     └ 놀고 있음 ┘
```

### Prefill and Decode

```
프리필: 토큰이 수천 개 들어와 GPU가 이미 꽉 참
        → EP의 이점(가중치 부담 분산)이 크지 않음
        → all-to-all 비용만 붙어서 손해인 경우가 많음
        → 프리필 노드는 EP를 안 쓰는 구성이 흔함

디코드: 배치를 키워야 하고, 키우려면 KV 공간이 필요
        → EP로 가중치 부담을 흩뿌리는 것이 직접적인 이득
        → 대규모 EP는 디코드 쪽에 배치
```

### TP vs EP

#### TP

```
        GPU0      GPU1      GPU2      GPU3
전문가0  [W0의 1/4][W0의 1/4][W0의 1/4][W0의 1/4]
전문가1  [W1의 1/4][W1의 1/4][W1의 1/4][W1의 1/4]
전문가2  [W2의 1/4][W2의 1/4][W2의 1/4][W2의 1/4]
전문가3  [W3의 1/4][W3의 1/4][W3의 1/4][W3의 1/4]

모든 GPU가 모든 전문가의 조각을 조금씩 가짐
```

#### EP

```
        GPU0      GPU1      GPU2      GPU3
전문가0  [  W0   ]
전문가1            [  W1   ]
전문가2                      [  W2   ]
전문가3                                [  W3   ]

각 GPU가 자기 전문가의 가중치를 온전히 가짐
```

Let's assume a single token arrives and only expert #1 is selected.

With TP:

```
필요한 것: W1 전체
W1이 있는 곳: GPU0, GPU1, GPU2, GPU3에 1/4씩

  GPU0: W1의 1/4로 부분 계산  ┐
  GPU1: W1의 1/4로 부분 계산  ├→ all-reduce로 합산 → 결과
  GPU2: W1의 1/4로 부분 계산  │
  GPU3: W1의 1/4로 부분 계산  ┘

4장 전부 동원. 통신은 all-reduce 1회.
```

With EP:

```
필요한 것: W1 전체
W1이 있는 곳: GPU1

  GPU0: 놀음 (또는 다른 토큰 처리)
  GPU1: W1으로 전체 계산 → 결과
  GPU2: 놀음
  GPU3: 놀음

1장만 동원. 통신은 토큰을 GPU1에 보내고 결과를 받는 것.
```

It's the difference between performing the same computation across 4 GPUs versus entirely on 1 GPU.

```
TP: 계산 결과가 오간다
    각 GPU가 만든 부분합을 서로 더해야 함
    → 히든 차원 크기의 벡터가 오감

EP: 토큰이 오간다
    "이 토큰을 처리해 줘"라고 보내고 결과를 받음
    → 토큰 임베딩이 오감
```

While the sizes are similar, the patterns differ. TP's all-reduce is an operation where all GPUs end up with the same value, whereas EP's all-to-all is an operation where each GPU sends data to different destinations.

#### The Difference Between MoE's Sparsity Surviving or Dying

MoE is designed to save computation by activating only 8 out of 256 experts.

```
TP로 나누면
  토큰이 8개 전문가를 골랐다
  → 그 8개가 32장에 조각조각 흩어져 있음
  → 32장 전부가 깨어나서 계산
  → 3%만 쓰기로 한 설계가 무의미해짐

EP로 나누면
  토큰이 8개 전문가를 골랐다
  → 그 8개를 가진 GPU들만 일함
  → 나머지는 다른 토큰을 처리
  → 희소성이 유지됨
```

Consequently, the difference is:

```
all-reduce (TP)
  각 GPU가 부분합을 갖고 있음 → 전부 더해서 → 모두가 같은 값을 갖게 됨

  GPU0: [a0]  ┐
  GPU1: [a1]  ├→ 합산 → 모두가 [a0+a1+a2+a3] 보유
  GPU2: [a2]  │
  GPU3: [a3]  ┘

all-to-all (EP)
  각 GPU가 각 GPU에게 서로 다른 데이터를 보냄

  GPU0: [→G0][→G1][→G2][→G3]
  GPU1: [→G0][→G1][→G2][→G3]      각자 다른 목적지, 다른 내용
  GPU2: [→G0][→G1][→G2][→G3]
  GPU3: [→G0][→G1][→G2][→G3]
```

Comparing the amount of data:

```
all-reduce  : GPU당 약 2S(N-1)/N 바이트 이동  → N이 크면 약 2S
all-to-all  : GPU당 약 S(N-1)/N 바이트 이동   → N이 크면 약 S
```

<br>

## DP - Data Parallelism

### Different Concept from the Previous Three

TP, PP, and EP all involve splitting the model, whereas DP does not split the model.

```
TP: 4 model chunks × all requests
PP: 4 layer segments × all requests
DP: entire model × 1/4 of requests each
```

The entire model is copied to each GPU, and different requests are assigned to them. The biggest advantage is the complete absence of communication, but it requires the entire model to fit on a single GPU.

In other words, it feels like a scale-out approach where memory is abundant, and requests are processed in parallel.

### DP Referring to Attention in LLM Serving

Pure DP, which replicates the entire model, cannot be used for large models. Therefore, in practice, when `--dp-size` is mentioned, it usually refers to running only the attention layers with DP.

```
Process layers differently by type

Attention Layer (DP=8)
  GPU 0~3: Fully process attention for request group A ← Holds only its own KV
  GPU 4~7: Request group B
  ...      No communication between groups

MoE Layer (EP=32)
  The same 32 GPUs now divide and handle experts
```

The reason for isolating only the attention is that some recent models store KV cache in a compressed format, reducing it to a fraction of its original size.

However, splitting attention with TP creates a problem.

```
When splitting attention with TP
  Attention heads are distributed among GPUs
  → Each GPU needs KV to compute with its own heads
  → Compressed KV does not split cleanly by head
  → Multiple GPUs end up storing the same KV redundantly

Memory saved by compression is lost again through duplication
```

When processed with DP, each GPU holds the KV for its assigned requests, so there is no duplication.

However, when requests are low:

```
For 4 concurrent requests, with DP=8
  One request assigned to each of groups 0~3
  Groups 4~7 have nothing to do → Half of GPUs are idle

In the same situation, if TP=32
  4 requests are processed by 32 GPUs → All work
```

DP is only effective when there are a very large number of requests; during periods of low traffic, TP is actually better.

<br>

## Summary

| Item | TP | PP | EP | DP (Attention) |
|---|---|---|---|---|
| Where to cut | **Within** a layer | **Between** layers | By expert unit | No cutting (request distribution) |
| What each GPU holds | Pieces of all layers | Entirety of some layers | Entirety of some experts | KV of its own request |
| Communication timing | Per layer | Section boundaries | Per MoE layer | None |
| Communication pattern | all-reduce | Sequential transfer | all-to-all | — |
| GPU operation | Simultaneously | Sequentially | Simultaneously | Independently |
| When there is 1 request | All work | Only 1 card | Only some | Only 1 group |
| Scope of application | Within the same machine | Between machines | MoE required | Large batch |
