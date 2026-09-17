# SGLang 서빙 실전 케이스북

> 기준 버전: SGLang 0.5.13 / 2026. 8  
> 이 노트의 벤치마크 수치와 로그는 시뮬레이팅한 값으로 실제 측정지와 편차가 있을 수 있다.  
> 공개된 벤치마크의 배수관계와 하드웨어 스펙을 역산한 값이며 절대값이 아니다. **튜닝 전후의 변화와 방향 크기를 익히는데만 사용하자**

### 다루는 케이스

1. **사내 코딩 어시스턴트**, 8xH200, TTFT p95 제약을 핵심기법 청크 프리필, 투기적 디코딩, 복제분할
2. **문서 QA / RAG**, 4xH200, 접두사 재사용 제약과 핵심기법 캐시 인지 라우팅, HiCache
3. **야간 배치 문서 처리**, 2xH100, 처리량 지배제약에서 모델 다운사이징, 배치 극대화
4. **구조화 출력 api**, 2xH100, 스키마 준수율 지배제약, XGrammer, 점프-포워드
5. **대형 MoE 대규모 서빙**, 16 Node x H200, 원가가 지배제약, EP, DP Attention, PD 분리

케이스마다 같은 순서로 진행한다.

```
요구사항과 SLO → 트래픽 프로파일 측정 → 모델 선정 → 용량 계산
    → 1차 기동 → 벤치마크 → 진단 → 튜닝 라운드 반복 → 최종 구성
```

## 공통 도구

### 워크로드 프로파일 - 튜닝 전에 확보할 수치

- **입력 길이 분포 (중앙값, p95):** 프리필 부담, 청크 크기를 결정
- **출력 길이 분포:** 디코드 부담, KV 캐시 체류 시간을 결정
- **접두사 공유율:** RadixAttention 이득의 크기를 결정
- **피크 동시성:** 투기적 디코딩 손익, 배치 파라미터를 결정

프로덕션 로그가 있으면 다음 스크립트로 뽑아볼 수 있음

```py
#!/usr/bin/env python3
"""profile_workload.py — 로그에서 워크로드 프로파일 추출"""
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

    # 접두사 공유율: 정렬 후 인접 쌍의 공통 접두사 길이 합 / 전체 토큰
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

접두사 공유율 기준
```
0 ~ 0.20   RadixAttention 이득 거의 없음. vLLM과 비교 우위 없음
0.20 ~ 0.60  둘 다 벤치마크해 볼 가치 있음
0.60 이상    SGLang 명확한 우위. 캐시 적중률 75~95% 기대
```

### 벤치마크 실행

```bash
# 실제 트래픽 분포를 모사한 온라인 벤치마크
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

- `--max-concurrency`: 동시 요청 상한을 피크에 맞추는 역할로 이를 설정안하면 무한 동시성으로 측정되어 지연 수치가 의미를 잃는것을 방지하기 위해서다
- `--random-range-ratio`: 길이를 균일 분포로 흩뜨리는 역할로 요청이 고정 길이로 측정되어 현실과 괴리감이 있는걸 해소해준다
- `--warmup-requests`: CUDA 그래프 캡처나 JIT 제외하기위해서 미리 첫 요청의 이상치와 평균을 오염시킨다.
- `--dataset-name sharegpt`: 실제 대화 분포를 사용하고 사용하지 않으면 접두사 공유 특성이 반영되지 않는다.

콜드 스타트 성능을 재려면 `--disable-radix-cache` 로 띄우거나 매 실행전에 캐시를 비운다.

```bash
curl -X POST http://127.0.0.1:30000/flush_cache
```

#### 출력

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

읽는 순서를 정해 두면 진단을 빠르게 할 수 있다.

```
1. Concurrency vs Max request concurrency (실제 처리중인 동시 요청 수 vs 허용된 최대치)
  -> Concurrency가 Max request concurrency보다 높다는 것은 서버가 요청을 다 못 받고 있다는 뜻 (큐 또는 메모리 문제)
  -> Concurrency값이 max에 비해 낮다는 것은 시스템 자원이 여유롭게 부하가 적은 상태라는 뜼이다.

2. Median TTFT vs p99 TTFT
  -> 배수가 3배 이상이면 긴 프롬프트의 블로킹, 청크 프리필을 의심한다.

3. Median ITL vs P99 ITL / Max ITL
  -> Max ITL이 수백 ms면 스케줄러가 특정 스텝에서 멈춰진것 선점(preemption) 또는 프리필 삽입이 원인이다.

4. Output token throughput
  -> 원가 지표, 튜닝 라운드 간 비교의 기준선
```

### 서버 메트릭

```bash
# Prometheus 엔드포인트 (--enable-metrics 필요)
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

- `num_queue_reqs`가 0에 가까운게 정상범위고 벗어나면 큐에 쌓이는 뜻으로 용량 부족 또는 스케줄링 상한이라는 뜻이다.
- `token_usage` 정상범위는 0.5~0.85로 1.0 근접시 kv 캐시 포화 -> 선점 발생 의미
- `cache_hit_rate`: 정상범이는 워크로드에 의존되고 공유율 대비 낮으면 프롬프트 설계문제 (캐싱이 잘 안됨, 변하는값이 앞에 있거나)

`token_usage`가 1.0에 붙어있는데 `num_queue_reqs`도 높다면 KV 캐시 부족이 병목이다. gpu활용률이 낮은데 큐가 길면스케줄링 문제고 이 둘을 구분하는것이 진단의 출발점이다.

### 튜닝 순서

효과 크기 순으로 순서를 지켜보자.

```
1. 모델 선정          작업 난이도에 맞는 최소 모델          최대 수 배
2. 병렬화 배치        TP 크기와 복제 수의 분할              2~3배
3. 청크 프리필        p99 TTFT 안정화                       지연 안정
4. 캐시 전략          RadixAttention, 라우팅, HiCache        공유율 의존
5. 양자화             FP8 가중치 + FP8 KV                    1.3~1.5배
6. 배치 파라미터      SLO 곡선 위 지점 선택                  10~30%
7. 투기적 디코딩      저동시성일 때만                        1.5~2배
8. PD 분리 / 대규모 EP  수십 GPU 이상에서만                   최대 5배
```

<br>

## 사내 코딩 어시스턴트

### 요구사항

- **사용자:** 엔지니어 200명
- **피크 동시 요청:** 25
- **인터페이스:** IDE Plugin, Streaming
- **하드웨어:** 8xH200 SXM(141GB, 4.8TB/s) 단일 노드
- **SLO:** **TTFT p95 1,000ms**, **ITL 중앙값 < 50ms**

### Profiling

현재 `profile_workload.py` 실행 결과

```
입력 중앙값  : 2100
입력 p95     : 28000
출력 중앙값  : 400
출력 p95     : 1800
접두사 공유율 : 0.35
```

입력 p95가 중앙값의 13배 병목이 존재함, 파일 전체를 붙여 넣는 사용패턴 때문이며, 청크 프리필 없이는 ㅉ랍은 요청이 긴 요청 뒤에서 멈춘다.

공유율 0.35는 경계선이고 시스템 프롬프트와 사내 코딩 규약 3,000토큰이 전원 공유이며 나머지는 제각각이다. 에이전트형 도구 호출을 도입하면 이 값이 올라가므로 SGLang을 택할 근거가 된다.

### 모델 선정

후보 셋을 비교해보자

**후보1. Qwen3.6-27B, dense형태, FP8가중치 27GB, 코딩성능 SWE-bench Verfied 77.2, 배치 1 디코드 빠름**:

**후보2. Qwen3.6-35B-A3B: MoE (활성 3B), FP8 가중치 35GB, 27B보다 코딩성능 낮음. 배치1 디코드는 매우 빠름**

**후보3. GLM-4.5-Ari, MoE 106B/12B, FP8 가중치 106GB, 코딩성능은 27B보다 높음, 배치 1디코드 중간**

#### Qwen3.6-27B로 결정

동시성 25는 H200 임계 배치값 (206) 보다 훨 작다. 이 구간은 메모리 바운드 구간이므로 성능이 **읽어야할 가중치 바이트 수**로 결정된다.

MoE의 활성 파라미터 이점은 대배치에서 나오는데 여기서 배치가 작아 이점이 작고, 대신 총 파라미터를 전부 메모리에 올려야 하는 부담만 남는다.

GLM-4-5-Air는 코딩 성능이 더 높지만 FP8로도 106GB라 KV 캐시 여유가 줄고, 27B 대비 디코드가 느려 ITL SLO를 맞추기가 어렵다.

### 용량 계산

Qwen3.6-27B의 어텐션 구성 (층 48, KV헤드 8, 헤드 차원 128)으로 KV 캐시를 계산한다.

```
토큰당 KV (BF16) = 2 × 48 × 8 × 128 × 2바이트 = 196,608 바이트 ≈ 192 KB
토큰당 KV (FP8)  = 96 KB

평균 컨텍스트 3,000토큰 × 동시 25건
  BF16: 3,000 × 192 KB × 25 ≈ 14.4 GB
  FP8 :                       ≈  7.2 GB

p95 시나리오 (28,000토큰 × 25건)
  BF16: 28,000 × 192 KB × 25 ≈ 134 GB
  FP8 :                       ≈  67 GB
```

8xH200 = 1,128GB에서 가중치 27GB를 빼면 여유가 크므로 메모리 제약은 아니다.

이 케이스의 제약은 지연이지 용량이 아니고 그래서 튜닝도 지연쪽으로 가야한다.

### 라운드 0 기본 기동

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

- **TTFT p95:** p99가 4381로 추정시 약 3,200쯤 예측되고 값은 < 1000에 비하면 터무니없이 초과되어 있다 판정     ❌
- **ITL 중앙값:** 28.41로 50보다 낮으므로 통과 판정 ✅

P99 TTFT가 중앙값의 7배로 Max ITL 982ms도 진단되는데 디코드중이던 요청이 한 스텝에서 1초 가까이 멈췄다는 뜻이고 원인은 긴 프롬프트의 프리필이 통째로 들어와 배치를 점유한 것이다.


### 라운드 1 - 청크 프리필

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

변화 해석을 해보면 TTFT가 4,381 -> 1,621ms로 떨어졌고 Max ITL은 982 -> 118ms가 되엇다.

28,000 토큰 프리필이 4,096 토큰 조각 7개로 나뉘어 디코드 배치 사이사이에 삽입되면서 한 요청이 gpu를 독점하던 구간이 사라졌다.

대가로 처리량 3.6%가 떨어지고 중앙값 TTFT가 소폭 올랐다. 프리필을 쪼개면 각 조각의 행렬이 작아져 gpu 연산을 더 해 효율이 조금 낮아지기 때문이다.

**꼬리 지연을 63%줄이고 중간 처리량 3.6%를 낸 거래이며** 지연이 SLO인 이 케이스에서는 유리한 결정이다.

**+청크 크기를 2,048로 더 줄여도 보았다.**

```
--chunked-prefill-size 2048
P99 TTFT (ms):                           1584.22     (-2.3% vs 4096)
Output token throughput (tok/s):         1402.11     (-6.4% vs 4096)
```

꼬리개선은 2% 남짓인데, 처리량은 6% 손해다. 4096에서 수확 체감이 시작되므로 여기서 멈추자.

### 라운드 2 - FP8 양자화

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

모든 지표가 동시에 개선되었다. 이 구간이 메모리 바운드이기 때문이다.

가중치 54GB(BF16)에서 27GB(FP8)로 줄어 디코드 스텝마다 읽는 바이트가 절반이 되었고 디코드 시간이 그대로 절반 가까이 줄었다.

**품질 검증은 반드시 함께 해야한다** 양자화 손실은 일반 대화에서 안보이다가 코드에서 드러나는 경우가 있기에

```bash
# 사내 코드 케이스 50건으로 BF16 대비 비교
python eval_codegen.py --baseline http://bf16-server:30000 \
                       --candidate http://fp8-server:30000 \
                       --cases internal_50.jsonl --blind
```

```
케이스 50건 블라인드 평가
  BF16 우세  : 6건
  FP8 우세   : 5건
  동등       : 39건
  컴파일 실패: BF16 1건 / FP8 1건
→ 유의한 차이 없음. FP8 채택
```

### 라운드 3 - 투기적 디코딩

동시성 25는 임계배치 206보다 한참 아래이므로 계산 자원이 남아있다.

투기적 디코딩이 그 여유를 쓰게한다.

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

서버 로그에서 수용률을 확인해보자

```
[2026-08-07 14:22:11] Speculative decoding stats:
  accept_length_mean: 3.82 / 6
  accept_rate: 0.637
  draft_overhead_ratio: 0.118
```

변화 해석을 해보면 토큰 하나를 생성할 때 읽는 가중치는 6개를 검증할때와 같다.

드래프트가 제안한 6개중 평균 3.82개가 수락되면서 스텝당 유효토큰이 3.8배가 되었고 드래프트 모델 실행 비용 11.8%를 빼고도 ITL이 40%가 떨어졌다.

TTFT가 5~6%로 올랐다 이는 드래프트 모델도 프리필을 수행하기 때문이다.

SLO 여유 안에 있어 수용이 가능하다.

**동시성을 올려 손익 분기를 측정해보자**

```
동시성   투기 OFF (tok/s)   투기 ON (tok/s)   배수
  8          712               1584          2.22×
 25         1971               3285          1.67×
 64         3844               4912          1.28×
128         5901               5734          0.97×   ← 역전
```

동시성 128부터 투기적 디코딩이 손해로 바뀐다.

이는 계산바운드에 가까워지면서 드래프트 실행이 정직한 계산의 자리를 뺏기 때문이다.

피크 동시성이 늘어나면 이 설정을 재검토해야한다는 뜻이며 운영 지표로 걸어둘 항목이다.

### 라운드 4 - TP8 단일 인스턴스 vs TP2 4복제

여기까지 8장을 하나의 인스턴스로 묶었다. 이 구성은 요청 하나의 계산을 8장에 분산하므로 개별 지연에 유리하지만, 층마다 all-reduce 통신이 발생하고 배치가 작을 때 GPU가 논다.

8장을 TP2 인스턴스 4개로 쪼개고 라우터로 묶어 비교해보자.

```bash
# 워커 4개 (GPU 0-1, 2-3, 4-5, 6-7)
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

# 캐시 인지 라우터
python -m sglang_router.launch_router \
  --worker-urls http://127.0.0.1:30001 http://127.0.0.1:30002 \
                http://127.0.0.1:30003 http://127.0.0.1:30004 \
  --policy cache_aware \
  --host 0.0.0.0 --port 30000
```

```
TP8 × 1      TP2 × 4      변화
Output tok/s              3284.90      4418.72      +34.5%
Median TTFT (ms)           511.44       362.18      -29.2%
P99 TTFT (ms)             1247.02       938.55      -24.7%
Median ITL (ms)             12.63        17.94      +42.0%
P99 ITL (ms)                34.18        45.60      +33.4%
cache_hit_rate               0.34         0.31       -0.03
```

#### 변화 해석

처리량과 TTFT는 개선되고 ITL은 나빠졌다. 두 방향이 갈리는 이유가 다르다.

- **처리량 상승:** TP8은 층마다 8장이 통신하는데 TP2는 2장만 통신하여 통신 오버헤드가 줄고 4개 인스턴스가 각각 독립적으로 배치를 채워 GPU활용률이 올라가기 때문이다.
- **TTFT 하락:** 인스턴스 4개라 큐가 4개로 나뉘어 한 인스턴스가 긴 프리필을 처리하는 동안 다른 요청은 다른 인스턴스로 가게된다
- **ITL은 상승:** 요청 하나에 투입되는 GPU가 8장에서 2장으로 줄어 한 토큰을 만드는데 계산이 덜 분산되므로 개별 토큰 생성이 느려진다.
- **캐시 적중률 소폭 하락:** 캐시가 4개 인스턴스로 쪼개졌다. `cache_aware` 정책이 접두사를 보고 같은 워커로 보내지만 완전히 상쇄하지는 못한다.

ITL 중앙값 17.94ms는 SLO 50ms에 여유가 있으므로 TP2 x 4 구성을 채택한다.

ITL SLO가 15ms처럼 빡빡했다면 반대 결론을 선택해야겠지만.

### 최종 구성과 누적 효과

```
라운드            Output tok/s   Median TTFT   P99 TTFT   Median ITL
0 기본                 1554          612        4381        28.4
1 청크 프리필           1499          648        1621        27.9
2 FP8                  1971          482        1185        21.1
3 투기적 디코딩          3285          511        1247        12.6
4 TP2 × 4 + 라우터      4419          362         939        17.9

누적                  2.84×        -41%        -79%        -37%
```

SLO판정
- TTFT p99 939ms p95 약 780ms < 1000ms    ✅
- ITL 중앙값 17ms < 50ms     ✅

### 총정리

청크프리필은 지연 SLO가 있으면 예외 없이 킨다. 처리량 3~4%를 내고 꼬리지연 60%이상을 산다.

**TP크기는 크다고 좋지 않다.** TP는 개별의 요청 지연을 낮추지만(메모리 자리가 많아서), 통신 비용을 늘리고 병렬 배치 기회를 줄인다. ITL SLO에 여유가 있다면 TP를 줄이고 복제를 늘리는쪽이 처리량과 TTFT 양쪽에서 유리하다.

이 트레이드오프는 케이스마다 결론이 달라지므로 반드시 실측으로 확인해야한다.

투기적 디코딩은 동시성에 따라 효과가 뒤집히고 손익분기를  미리 측정해두고 트래픽이 그 지점을 넘으면 자동으로 끄는 운영 규칙을 만들어야한다.


<br>

## 문서 QA / RAG 서비스

| 항목 | 값 |
|---|---|
| 서비스 | 계약서·규정 문서 질의응답 |
| 문서 수 | 1,000종 (인기 50종에 트래픽 80% 집중) |
| 문서 길이 | 평균 20,000토큰 |
| 요청률 | 평균 8 req/s, 피크 22 req/s |
| 하드웨어 | 4×H100 SXM (80GB, 3.35TB/s) |
| SLO | TTFT p95 < 1,500ms |

### 트래픽 프로파일

```
입력 중앙값: 20530
입력 p95: 24800
출력 중앙값: 300
출력 p95: 620
접두사 공유율: 0.81
```

전 케이스와 성격이 반대다

입출력 비율이 68:1로 프리필이 압도적으로 길고 디코드가 짧다.

투기적 디코딩은 디코드 구간을 가속하는 기법이므로 여기서는 이득이 적다.

최적화 자원을 프리필에 집중시켜야한다.

**공유율 0.81은 SGLang의 최대효과를 내는 구간으로** 같은 문서에 대한 질문이 20,500 토큰중에 20,500개가 겹치고 질문 30토큰만 다르다.

```
요청 A: [시스템 500][계약서 20,000][질문1 30]
요청 B: [시스템 500][계약서 20,000][질문2 30]
                    └─ 20,500 공유 ─┘  └ 30만 다름
```

| 모델 | FP8 가중치 | 네이티브 컨텍스트 | 장문 이해 |
|---|---:|---:|---|
| Qwen3.6-27B | 27 GB | 262K | 우수 |
| Qwen3.6-9B | 9 GB | 262K | 보통 |
| GLM-5.2 | 372 GB | 1M | 최상 |

**Qwen3.6-27B**를 선택한다.

GLM5.2은 장문 성능이 가장 좋지만 FP8로도 372GB이라서

4xH100(320GB)에는 들어가지 않는다. 9B는 20,000토큰 문서에서 특정 조항을 정확히 찾는 작업의 정확도가 눈에 띄게 떨어졌다.

### 용량 계산

```
Qwen3.6-27B FP8, Token당 KV = 96KB

4xH100 = 320GB
가중치: 27GB
활성값, 버퍼 여유 35GB
KV 캐시 가용 258GB

캐시 가능한 문서 수 = 258GB / (20,000 x 96KB) 
                    = 258 GB ÷ 1.92 GB
                    ≈ 134종
```

1000종중 134종만 GPU에 상주할수 있다.

인기 50종 트래픽이 80퍼센트를 차지하므로 이론상 대부분을 담을 수 있지만,

나머지 950종이 계속 들어오면 축출을 유발하고 이 점이 이 케이스 튜닝의 중심 문제가 된다.

### 단일 인스턴스 기본 - 라운드 0

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

`--enable-cache-report`는 응답에 캐시 적중 토큰 수를 포함시킨다 이 케이스에서는 필수 플래그다

실제 문서 분포를 모사한 벤치마크

```py
#!/usr/bin/env python3
"""gen_rag_trace.py — 문서 재사용 분포를 반영한 트레이스 생성"""
import json, random

SYSTEM = "당신은 계약서 분석 어시스턴트입니다. " * 40          # 약 500토큰
DOCS = {i: f"[문서{i}] " + "계약 조항 본문 " * 3300 for i in range(1000)}
QUESTIONS = ["해지 조항의 통지 기간은?", "위약금 산정 기준은?",
             "관할 법원은 어디인가?", "자동 갱신 조건은?"]

def pick_doc():
    # 인기 50종에 80%, 나머지 950종에 20%
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

요청률 4.85 req/s 피크는 22 req/s에 못 미치고, TTFT 중앙값 1,843ms는 SLO 1,500ms를 넘는다.

캐시 적중률 0.62는 공유율 0.81에 못미치고 차이 0.19가 축출로 날아간 몫이다.

134종 용량에 1,000종이 드나들면서 인기 문서까지 밀려나고 있다.

### 라운드1: 복제 분할과 캐시 인지 라우팅

TP4 하나를 TP2 둘로 쪼개고 라우터를 앞에 둔다. 라우팅 정책을 두 가지로 비교한다.

```bash
for i in 0 1; do
  CUDA_VISIBLE_DEVICES=$((i*2)),$((i*2+1)) \
  python -m sglang.launch_server \
    --model-path Qwen/Qwen3.6-27B --tp-size 2 \
    --quantization fp8 --kv-cache-dtype fp8_e5m2 \
    --chunked-prefill-size 8192 --mem-fraction-static 0.88 \
    --port $((30001+i)) --enable-metrics --enable-cache-report &
done

# 비교 A: 라운드로빈
python -m sglang_router.launch_router \
  --worker-urls http://127.0.0.1:30001 http://127.0.0.1:30002 \
  --policy round_robin --port 30000

# 비교 B: 캐시 인지
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

라운드로빈에서는 오히려 나빠졌다 같은 문서에 대한 질문이 두 워커로 흩어지면서 양쪽 모드 같은 문서를 프리필하게 되었고 캐시 용량이 실질적으로 절반이 되었다. 적중률 0.62에서 0.44로 덜어진 것이 이 손실을 보여준다.

`cache_aware`는 요청 접두사 해시를 보고 같은 문서를 같은 워커로 보낸다.

워커마다 담당 문서가 나뉘므로 각자 500종만 다루면 되고 캐시 경쟁이 절반으로 줄어 적중률이 0.79로 올랐다.

**같은 하드웨어에서 라우팅 정책 하나로 TTFT 2.8배 갈린다.** 접두사 공유가 큰 워크로드에서 라운드로빈은 캐시를 무력화하는 설정이다.

### 라운드2 - 프롬프트 재설계

시스템 프롬프트를 점검했더니 앞부분에 변하는 값이 있었다고 치자

```python
# 문제가 있던 형태
SYSTEM = f"""현재 시각: {datetime.now()}
요청 ID: {request_id}
사용자: {user_name}
당신은 계약서 분석 어시스턴트입니다. ..."""
```

`현재시각`이 매 요청마다 달라 첫 토큰부터 접두사가 달라져 뒤에 20,500 토큰이 동일해도 캐시로 쓸 수 없게 된다.

변하는 값을 뒤로 옮긴다.

```py
# 수정된 형태
SYSTEM = """당신은 계약서 분석 어시스턴트입니다. ...
(고정 지침 전체)"""

PROMPT = SYSTEM + DOCUMENT + f"""
---
현재 시각: {datetime.now()}
요청 ID: {request_id}
질문: {question}"""
```

```
라운드 1    라운드 2      변화
cache_hit_rate          0.79       0.94       +0.15
Median TTFT (ms)      892.41     412.88      -53.7%
P99 TTFT (ms)        1684.30     961.22      -42.9%
Output tok/s         2740.66    3612.19      +31.8%
req/s                   9.14      12.04       +31.7%
```

코드 세 줄을 옮겨 TTFT가 절반이 되었다 하드웨어 설정도 그대로다.

프리필을 해야할 토큰 수로 확인하면 이렇다.

```
수정 전: 매 요청 20,530 토큰 전부 프리필 (캐시 무효)
수정 후: 캐시 적중시 30~50토큰만 프리필

20,530 -> 40
```

접두사 캐싱은 완전히 일치하는 접두사에만 작동한다.

프롬프트 템플릿에서 변하는 값이 있으면 그 뒤 전부가 무효가 되므로 서빙 최적화 이전에 프롬프트 구조부터 점검해야한다.

### 라운드3 - HiCache로 용량 확장


적중률 0.94에서 남은 0.06은 인기없는 950종 문서로 GPU 용량 134종의 한계이므로 계층 캐시로 CPU메모리 까지 확장한다.

### 계층 캐시가 하는일

RadixAttetnion은 KV 캐시를 GPU 메모리에만 둔다.

용량이 차면 LRU로 오래된 것부터 버리고, 버려진 문서가 다시 오면 20,000토큰을 처음부터 프리필한다.

계층 캐시(HiCache)는 버리는 대신 **아래층으로 내린다.**

```
용량        대역폭        이 케이스의 역할
┌──────────────┐
│  GPU HBM     │      258 GB      3,350 GB/s    인기 문서 134종 상주
├──────────────┤
│  CPU DRAM    │      516 GB         64 GB/s    밀려난 문서 보관
├──────────────┤
│  NVMe / 원격  │      수 TB           7 GB/s    장기 보관 (선택)
└──────────────┘
        ↑ 아래로 갈수록 크고 느림
```

GPU에서 축출될 KV를 CPU메모리에 옮겨두고 그 문서가 다시 요청되면

**재계산 대신 전송으로 복원한다.** 그래서 전송 시간이 재계산시간보다 느리면 쓰면 안된다.

> 책상(GPU)에 자주 보는 책 134권을 두고 나머지는 버리는 대신 발밑 서랍. 혹은 멀리있는 책장에 두는것이 도서관에 다시가서 빌려오는(재계산)것보다 빠르다 다만 서랍에서 꺼내는것도 공짜는 아니다.

전송 방향에 따라 이름도 다른데

```
축출시 GPU >> CPU 내보내기 오프로드라고 하고 백그라운드로 수행한다
적중시에 CPU >> GPU로 가져오는 프리패치라고 하고 프리필 직전에 수행한다.
```

가져오기가 지연에 직결되므로 스케줄러가 요청을 처리하기 전에 미리 시작하는 것이 중요하다.

`hicache-storage-prefetch-poliy`가 이 동작을 제어한다.

**정책**
- `best_effort`: 전송이 늦으면 기다리지 않고 그만큼 재계산, 지연 SLO가 빡빡할때 적합
- `wait_complete`: 전송이 끝날때까지 대기, 재계산이 매우 비쌀때 적합
- `timeout`: 일정 시간까지만 대기 후 포기, 절충할때 적합함

**도입 전에 손익 분기를 계산해야한다.**

```
문서 하나의 KV 캐시 = 20,000 × 96 KB = 1.92 GB

재계산 비용 (프리필)
  20,000토큰 × 2 × 27e9 FLOP = 1.08e15 FLOP
  H100 FP8 1,979 TFLOPS × 4장 × 실효 60% = 4,750 TFLOPS
  → 1.08e15 / 4.75e15 ≈ 227 ms

전송 비용 (CPU DRAM → GPU)
  1.92 GB ÷ 64 GB/s (PCIe 5.0) ≈ 30 ms

30 ms < 227 ms  →  오프로딩이 7.6배 유리
```

이 비율이 1보다 작은 HiCache는 손해다.

출력이 길고 입력이 짧은 워크로드에서는 반대결과가 나오므로 반드시

자기 숫자로 계산해야한다.

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

플래그별 의미

- `--enable-hierarchical-cache`: 계층 캐시를 연다는 뜻이고 켜지 않으면 나머지 hicache 플래그가 무시된다.
- `--page-size 64`: KV 캐시를 몇 토큰 단위로 자를지 정하는 값으로 페이지 하나 = 64토큰이라는 뜻이다.
- `--hicache-ratio 2`: CPU쪽 KV 풀을 얼마나 크게 잡을지를 정한다 GPU풀의 2배를 쓰는거니까 총 gpu까지해서 3배의 힘을 내는것
- `--hicache-write-policy write_through`: 언제 cpu에 복사할지 정하고 write_through는 gpu에 쓰자마자 즉시, 적중률 우선일때 쓰자
- `--hicache-io-backend kernel`: gpu <-> cpu 전송을 어떤 경로로 할지 정한다. 이는 커널 경유이며, direct는 대용량시에 유리하다.

이 케이스에서 `kernel`을 쓰는 이유는 전송 대상이 흩어져있기 때문이다.

라딕스 트리에서 축출되는 페이지는 물리적으로 연속이 아니고 한 문서에 20,000토큰도 313개의 페이지로 나뉘어져있다.

이런 산발적 전송은 커널 하나로 묶는편이 유리하다

`direct`를 검토할 상황은 한 번에 옮기는 덩어리가 크고 연속적일때 페이지 크기를 128이상으로 키웟거나 프리페치를 문서 단위로 크게 걸어 전송이 몇 개의 큰 블록으로 정리되는 구성이 해당한다.

> 두 방식의 실측 차이는 PCIe 세대, 페이지 크기, 동시 전송 수에 따라 갈린다. 기본값 kernel로 시작해서 전송이 병목으로 확인될때만 direct를 재보는 순서가 안전하다. 바꿨는데 차이가 없으면 병목이 전송이 아니라는 뜻이다.


`--page-size`는 kv캐시를 몇 토큰 단위로 잘라 관리할지 정하며 기본값은 1 토큰 하나가 곧 한페이지라는 뜻이고 이 상태에서 계층 캐시가 동작하지 않는다.

gpu, cpu사이를 오갈때 토큰 단위로 전송하면 요청 하나당 오버헤드가 실제 데이터보다 커지기 때문이다.

64를 쓰는 이유는 전송 효율과 캐시 낭비의 절충이다.

캐시 낭비가 생기는 이유는 접두사 공유가 페이지 단위로만 인정되기 때문이다.

페이지 안에 토큰 하나라도 다르면 그 페이지 전체를 새로 계산해야한다.

```
페이지 크기 64, 두 요청의 접두사가 20,010토큰까지 같고 그 뒤가 갈릴 때

페이지:  [0~63][64~127] ... [19,904~19,967][19,968~20,031][20,032~ ...
                                    ↑ 여기까지 완전 일치      ↑ 여기서 갈림
                                      = 재사용 ✅              = 재계산 ❌

마지막 페이지 19,968~20,031 안에서
  20,010번까지는 같은데   ← 43토큰은 재사용 가능했음
  20,011번부터 다름       ← 이 때문에 페이지 전체가 무효

→ 재사용 가능했던 43토큰이 버려짐. 이게 경계 낭비.
```

낭비되는 양은 **(최대 페이지 크기 - 1)토큰**으로 페이지 64면 최대 63토큰이고 접두사가 갈리는 지점이 페이지 어디에 걸리느냐에 따라 0~63

```
페이지 크기가 클수록
  전송 1회당 데이터가 커져 PCIe 효율이 좋아짐   ← 이득
  경계 낭비의 상한이 커짐                      ← 손해

20,000토큰 문서, 페이지 64 기준
  전송 단위 = 64 × 96 KB = 6.1 MB   → PCIe 왕복 오버헤드가 묻힘
  경계 낭비 = 최대 63토큰            → 20,000 대비 0.3%
```

이 워크로드는 문서가 20,000 토큰으로 길어 자투리 손실이 무시할 수준으로 반대로 프롬프트가 수백토큰밖에 안되는 서비스라면, 64는 낭비가 커지니 16 32를 검토하자.

`--hicache-ratio 2`는 이런 용량이 된다.

```
GPU KV 풀    258 GB  →  문서 134종     (ratio의 기준. 항상 1배)
호스트 풀    516 GB  →  문서 268종     (258 × ratio 2)
                        ────────────
총 캐시 가능             402종          → GPU만 쓸 때의 3배 (1 + 2)

1,000종 중 40%가 캐시에 상주. 인기 50종은 항상 GPU에 남습니다.
```

호스트 풀이 gpu 풀을 대체하는 것이 아니라 그 위에 얹히는 구조라서 총 용량은 항상 1 + ratio 배가 된다. ratio 4면 5배이다.

`--hicache-size 200` 으로 절대 크기 GB를 지정할 수도 있다.

지정하면 ratio를 덮어쓰고 호스트 풀은 cpu 메모리를 그만큼 점유하므로 시스템 RAM여유를 먼저 확인하자.

516GB를 잡으려면 그 이상의 물리 메모리가 있어야한다.

```
라운드 2    라운드 3      변화
cache_hit_rate          0.94       0.97       +0.03
Median TTFT (ms)      412.88     356.20      -13.7%
P99 TTFT (ms)         961.22     594.71      -38.1%
Output tok/s         3612.19    3844.02       +6.4%
req/s                  12.04      12.81       +6.4%
```

중앙값 개선은 13.7%으로 크지 않은데 P99가 38% 개선되었다.

이미 캐시에 있던 인기문서 (중앙값을 지배중인)는 달라질 게 없고 **캐시에 없던 비인기 문서(꼬리를 만들던 요청)가 cpu 메모리에 복원**되면서 꼬리가 짧아진 것이다.

`write_through`, `write_back`을 비교했다.

```
write_through   write_back
cache_hit_rate            0.97         0.93
Median TTFT (ms)        356.20       341.55
Output tok/s           3844.02      3901.30
```

`write_back`은 호스트 기록을 지연시켜 gpu쪽 오버헤드가 작지만 축출 시점에 아직 기록되지 않은 블록이 있어 적중률이 낮다. **적중률이 지배적인 워크로드에서는 write_through가 맞다**

### 최종 구성과 누적 효과

```
라운드                      hit_rate   Median TTFT   P99 TTFT   req/s
0 TP4 단일                    0.62        1843        3722      4.85
1 TP2×2 + cache_aware         0.79         892        1684      9.14
2 프롬프트 재설계               0.94         413         961     12.04
3 HiCache                     0.97         356         595     12.81

누적                        +0.35        -81%        -84%     2.64×
```

피크 22 req/s는 12.81 req/s로 부족하므로 노드를 하나 더 붙이거나 인기문서를 사정 워밍업하는 방안이 필요하다.

사전 워밍은 서비스 시작시 인고 50종을 미리 프리필해 캐시해 올려두는 방식이다.

```bash
# 인기 문서 사전 워밍
for doc in $(head -50 popular_docs.txt); do
  curl -s http://127.0.0.1:30000/generate \
    -d "{\"text\": \"$(cat docs/$doc.txt)\", \"sampling_params\": {\"max_new_tokens\": 1}}" \
    > /dev/null
done
```

### 교훈

1. **라우팅 정책이 캐시 전략에 일부가 된다.** 접두사 공유가 큰 워크로드에서 라운드로빈은 캐시를 무력화 한다. 워커를 늘리기전에 라우팅부터 확인해야한다.
2. **프롬프트 구조가 인프라 설정보다 큰 효과를 내는 경우가 있다.** 변하는 값을 앞에서 뒤로 옮긴것만으로 TTFT를 절반으로 만들수있었다. 서빙팀과 애플리케이션 팀이 분리되어있으면 이 문제를 아무도 발견하지 못할 것이다.
3. **HiCache는 중앙값보다 꼬리값을 개선한다:** 이미 캐시된 요청은 달라지지않고 미스가 날대 요청이 복원되기 때문에 p99 SLO가 있는 서비스에서 특히 값어지가 있다.
4. **계층 캐시는 도입전에 손익 분기를 계산해야한다:** 계산속도와 전송 비율이 1을넘지 않으면 순손해다. 물론 계산 자원이 부족한 경우는 별개겠지만

<br>

## 케이스3 야간 배치 문서 처리

| 항목 | 값 |
|---|---|
| 작업 | 고객 문의 240만 건 분류 + 요약 |
| 입력 | 평균 1,200토큰 |
| 출력 | 평균 150토큰 (JSON 라벨 + 3문장 요약) |
| 마감 | 야간 12시간 내 완료 |
| 하드웨어 | 2×H100 SXM |
| SLO | **없음.** 개별 지연 무관, 총 완료 시간만 |

지연 SLO가 없다는 점이 다른 케이스와 차별점이겠다.

케이스 1에서 했던 최적화는 전혀 필요없는 반대되는 케이스다.

### 필요 처리량 계산

```
총 입력 = 2,400,000 x 1,200 = 2.88e9 tokens
총 출력 = 2,400,000 x 150 = 3.60e8 tokens

12시간 = 43,200초
필요 출력 처리량 =  3.60e8 / 43,200 ≈ 8,333 tok/s
필요 입력 처리량 = 2.88e9 / 43,200 ≈ 66,667 tok/s
```

### 라운드 0 - 케이스 1의 설정을 그대로 가져온다치면

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
필요 출력 처리량 8,333 tok/s 대비 2,148 tok/s
→ 완료 예상 시간 = 3.60e8 / 2148 / 3600 ≈ 46.5 시간
```

마감 12시간안에 처리해야되는게 46.5시간이 걸려 네 배 가까이 부족하다.

당연하다. 최적화 자체를 다른 방향성으로 했으니까

### 라운드 1 - 모델 다운사이징

배치 작업의 처리량은 모델 크기에 강하게 반비례한다.

작업 난이도에 맞는 최소 모델을 찾는 것이 가장 중요하다.

품질 검증부터 하자. 사람이라벨링한 검증셋 500건으로 후보들을 블라인드 평가하자.

```bash
python eval_classification.py \
  --models Qwen3.6-27B Qwen3.6-9B Qwen3.6-4B Qwen3.6-2B \
  --testset labeled_500.jsonl \
  --metrics accuracy,rouge-l
```

```
모델              분류 정확도   요약 ROUGE-L   FP8 가중치
Qwen3.6-27B         0.941         0.412         27 GB
Qwen3.6-9B          0.933         0.398          9 GB
Qwen3.6-4B          0.897         0.361          4 GB
Qwen3.6-2B          0.812         0.294          2 GB
```

**9B는 27B대비 정확도가 0.8%p, ROUGE-L 0.014하락**으로 이 작업의 허용오차 안에 들어온다고 가정하자, (실제로 들어올거임 적어서)

> ROUGE-L은 인공지능이 만든 요약이나 번역 문장이 사람이 만든 정답 문장과 얼마나 비슷한지 측정하는 평가 점수다. 연속된 단어만 보는 다른 방식과 달리 문장에서 **가장 긴 공통 부분(LCS, 최장 공통 부분 수열)**을 찾아내어 문장 구조이 닮은 정도를 찾아낸다.

4B는 정확도가 4.4&p나 떨어져 후속 수작업 검수 비용이 늘어나므로 제외한다.

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
                       27B        9B        변화
Output tok/s         2148.33   5912.77     +175%
Input tok/s         17186.64  47302.16     +175%
Median ITL (ms)      112.40     41.83      -62.8%
완료 예상 (시간)        46.5      16.9      -63.7%
```

설정 변경없이 모델만 바꿔서 **2.75배**의 효율을 낸다.

여전히 16.9시간으로 마감을 넘기긴 하지만 격차가 크게 줄긴했다.

### 라운드2 - 투기적 디코딩 제거

동시성 256정도는 H100 임계 배치 295에 근접한다.

gpuㅎ ㅏ드웨어 노트의 계산에 따르면 이 구간에서 계산 자원 여유가 거의 없고 드래프트 모델 실행이 계산의 자리를 뺏기도한다.

```diff
- --speculative-algorithm EAGLE3 \
- --speculative-draft-model-path Qwen/Qwen3.6-9B-eagle3 \
- --speculative-num-steps 5 --speculative-eagle-topk 8 \
```
```
                    투기 ON    투기 OFF     변화
Output tok/s        5912.77    7104.20     +20.2%
Median ITL (ms)       41.83      35.98     -14.0%
완료 예상 (시간)        16.9       14.1     -16.6%
```

서버 로그의 수용률을 보면 이유가 명확한데

```
[투기 ON, 동시성 256]
  accept_length_mean: 2.14 / 6
  accept_rate: 0.357
  draft_overhead_ratio: 0.284
```

수용률이 케이스 1에서는 0.637에서 0.357로 이번케이스에 나타나있으며 드래프트 오버헤드는 11.8퍼센트에서 28.4퍼센트로 올랐다.

6개를 제안해 2.14개만 통과시키면서 비용의28%를쓰는 상태로 끄는편이 낫다.

케이스1에서 66% 이득이던 기법이 여기서는 20% 손해라는 소리다. 같은 기법의 부호가 워크로드에 따라 뒤집히는 대표적인 예다.

이번 케이스에서는 출력보다는 입력에 대한 최적화가 필요하므로 디코드 최적화를 위한 작업이 불필요해 투기적 디코딩을 뺀다

> 투기적 디코딩은 가벼운 초안 모델이 먼저 여러 개 토큰을 추측해 만들고 무거운 타겟 모델이 한꺼번에 검증하여 속도를 높이는 기법으로 초안 모델을 빼게 되면 초안모델의 프리필 작업 없이 타겟모델과 gpu스케줄러가 본연의 대규모 연산에만 전념하게 된다.  초안 모델이 차지하던 GPU 메모리(SRAM/HBM)와 텐서 코어를 전부 회수하여, 타겟 모델이 긴 프롬프트를 처리하는 데 밀어주게 되는것

### 라운드3 - 배치 파라미터 극대화

지연 SLO가 없으므로 배치를 최대한 키운다.

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

- `--chunked-prefill-size 32768`: 지연이 무관하므로 쪼갤 이유가 없음 큰 행렬이 gpu 효율에 율함.
- `--max-running-requests 512`: 동시 시퀀스 상한 확대
- `--max-prefill-tokens 65536`: 한 배치를 담을 프리필 토큰 확대
- `--mem-fraction-static 0.94`: kv 풀 확대, 전용 인스턴스라 여유를 줄여도 안전
- `--schedule-conservativeness 0.3`: 스케줄러를 공격적으로. 선점 위험을 감수하고 배치를 채움

```
                  라운드 2    라운드 3     변화
Output tok/s       7104.20    9438.51    +32.9%
Input tok/s       56833.60   75508.08    +32.9%
Median ITL (ms)      35.98      68.22    +89.6%
Concurrency         248.91     487.33
완료 예상 (시간)       14.1       10.6    -24.8%
token_usage           0.71       0.93
```

**ITL이 90% 나빠지긴 했는데 이케이스에서는 문제가 아니다. 지연slo가 없기 때문에**

`token_usage` 0.93은 kv캐시가 거의 포화상태라는 뜻으로 `mem-fraction-static`을 0.96으로 더 올려보면

```
--mem-fraction-static 0.96
[2026-08-07 03:14:52] Warning: cache pool exhausted, preempting 14 requests
[2026-08-07 03:16:08] Warning: cache pool exhausted, preempting 22 requests
Output tok/s: 8712.44     (-7.7%)
```

선점이 발생해 오히려 느려진다. 선점된 요청은 처음부터 다시 계산되므로 그 작업이 낭비가 된다. 0.94가 구성의 상한값이라 이렇게 둔것.

### 라운드 4 - 클라이언트측 병렬도 조정

서버는 준비되었는데 클라이언트가 요청을 충분히 밀어넣지 못하는 경우가 있다.

```py
#!/usr/bin/env python3
"""batch_runner.py — 배치 작업 투입기"""
import asyncio, aiohttp, json, time

CONCURRENCY = 512          # 서버의 max-running-requests와 맞춤
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
클라이언트 동시성    서버 Concurrency   Output tok/s
      128                 127.4            5920.11
      256                 254.8            7802.33
      512                 487.3            9438.51
     1024                 496.1            9401.77   ← 포화
```

클라이언트 동시성 512쯤이 베스트고 그 이상은 포화 지점이다. 그 이상은 큐만 길어지고 처리량은 늘지 않는다. 서버 `--max-running-requests`와 클라이언트 동시성을 맞춰두는 것이 중요하다.

### 최종 구성과 누적 효과

```
라운드                    Output tok/s   완료 예상(h)   ITL 중앙값
0 27B + 케이스1 설정          2148          46.5         112.4
1 9B로 교체                   5913          16.9          41.8
2 투기적 디코딩 제거            7104          14.1          36.0
3 배치 파라미터 극대화          9439          10.6          68.2
4 클라이언트 동시성 정렬        9439          10.6          68.2

누적                        4.39×         -77%
```

마감 12시간 대비 10.6시간으로 여유시간 1.4시간을 확보했다.

### 교훈

1. **모델 선정이 모든 설정 튜닝의 합보다 얻는게 크다** 27B -> 9B로 한번 바꿨을뿐인데 2.75배 이득을 봤고 이후 세 라운드의 설정 튜닝을 다 합쳐봐야 1.6배 즉 튜닝시작전 맞는 모델, 이 작업을 하는데 이 크기가 정말 필요한가를 검증셋으로 확인해야한다.
2. **품질 검증 없는 다운사이징은 위험**: 정확도 0.8%p 하락은 수용가능하지만 4.4%p 같이 많이 하락되는 경우는 수작업 검수 시간에 손해를 가져올 수 있다. **허용 오차는 정확도 수치 뿐만 아니라 늘어난 후속 비용과 절감된 gpu 비용의 대소로 판단해야한다**
3. 투기적 디코딩은 케이스 1에서 +66%의 효율을 가져다 줬지만 여기서는 -20%였고 청크 프리필 크기는 4,096에서 32,768로 여덟배가 되었다. 차이를 만든것은 지연 SLO의 유무. 다른 프로젝트의 설정을 그대로 가져오면 안 되는 이유가 여기였다.
4. 이번케이스에서 메모리 사용률을 무조건 올릴수록 좋지 않았었다 `mem-fraction-static`을 0.96으로 두자 선점이 발생해 처리량이 7.7%나 떨어졌기에 선점된 요청은 다시 계산되므로 그때까지 작업이 버려진다 `preempting`경고, `token_usage` 0.95 이상을 감시 항목으로 두자

<br>

## 구조화 출력 API

| 항목 | 값 |
|---|---|
| 서비스 | 고객 CRM 에이전트의 도구 호출 백엔드 |
| 요청률 | 평균 25 req/s, 피크 40 req/s |
| 입력 | 1,800토큰 (도구 정의 1,200 + 대화 600) |
| 출력 | 120토큰 (JSON 함수 호출) |
| 하드웨어 | 2×H100 SXM |
| SLO | TTFT p95 < 800ms, **JSON 파싱 실패율 0%** |

파싱 실패율 SLO가 들어가있는 점이 이 케이스에 차별점인 것 같다

에이전트는 한 번의 요청으로 끝나지 않고 도구 호출을 20번쯤 연쇄한다 치면.

각 단계가 독립적으로 실패할 수 있으므로 전체 성공률은 단계별 성공률의 곱이 된다.

```
단계별 성공률이 p일 때, 20단계 전체 성공률 = p²⁰

p = 0.999  →  0.999²⁰ = 0.980   →  98.0%
p = 0.99   →  0.99²⁰  = 0.818   →  81.8%
```

한 단계에서 1%만 실패하더라도 20단계를 붙이게 되면 다섯 번 중 한 번은 작업이 전체가 무너지게 된다.

같은 곱셈이 지연에서는 이렇게 작동하지 않는다 TTFT가 800ms에서 900ms로 늘면 사용자가 조금 더 기다릴 뿐이지만 파싱실패는 그 단계부터 뒤가 전부 무의미해 지는거라 **실패율은 지연보다 우선하는 제약**이다.

### 모델 선정

| 모델 | FP8 가중치 | 도구 호출 정확도 | 배치 40 처리량 |
|---|---:|---:|---|
| Qwen3.6-27B | 27 GB | 0.947 | 낮음 |
| Qwen3.6-9B | 9 GB | 0.921 | 중간 |
| Qwen3.6-35B-A3B | 35 GB | 0.938 | **높음** |

**Qwen3.6-35B-A3B**를 선택

출력이 120토큰으로 짧고 요청률이 높아 동시성 40 근처에서 유지된다.

MoE의 활성 파라미터 3B는 디코드마다 읽는 가중치 3GB(FP8)에 불과해 dense 9B보다 빠르다.

총 35GB를 메모리에 올려야하지만 2xH100 = 160GB에서 여유가 있다.

케이스 1에서 dense를 고른 것과 반대 결론이다. **출력이 짧고 동시성이 높으면 MoE가 유리하고 출력이 길고 동시성이 낮으면 dense가 유리하다.**

### 라운드 0 - 프롬프트로만 JSON 요청

```bash
python -m sglang.launch_server \
  --model-path Qwen/Qwen3.6-35B-A3B --tp-size 2 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --chunked-prefill-size 8192 --mem-fraction-static 0.90 \
  --port 30000 --enable-metrics
```

요청은 프롬프트에 형식을 지시하는 방식이다.

```json
{
  "text": "...도구 목록...\n\n반드시 다음 JSON 형식으로만 답하세요:\n{\"tool\": \"...\", \"args\": {...}}\n\n사용자: 김철수 고객의 최근 주문 3건 조회해줘",
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

파싱 결과를 별도로 집계한다.

```py
#!/usr/bin/env python3
"""check_schema.py — 출력의 스키마 준수율 집계"""
import json, jsonschema, collections

SCHEMA = json.load(open("tool_call.schema.json"))

def audit(path):
    stats = collections.Counter()
    for line in open(path):
        out = json.loads(line)["generated_text"]
        try:
            obj = json.loads(out)
        except json.JSONDecodeError:
            stats["JSON 파싱 실패"] += 1; continue
        try:
            jsonschema.validate(obj, SCHEMA)
            stats["정상"] += 1
        except jsonschema.ValidationError as e:
            stats[f"스키마 위반: {e.validator}"] += 1
    return stats

for k, v in audit("outputs.jsonl").most_common():
    print(f"{k:28s} {v:5d}  ({v/50:.2f}%)")
```
```
정상                          4813  (96.26%)
JSON 파싱 실패                  87  ( 1.74%)
스키마 위반: enum               61  ( 1.22%)
스키마 위반: required           28  ( 0.56%)
스키마 위반: type               11  ( 0.22%)
```

실패율은 3.74%로 TTFT는 SLO를 만족하지만 실패율 SLO는 크게 벗어난다.

아까 위에서 봤을때 1퍼만 떨어져도 에이전트 작업의 성공률이 82%로 떨어지는데 지금 수치는 46.7%까지 떨어질 수 있어 **절반 이상이 중간에 무너진다.**

실패 사례를 살펴보면 유형이 나뉜다.

```
[JSON 파싱 실패 예시]
"물론입니다! 다음과 같이 조회하겠습니다.\n{\"tool\": \"get_orders\"...}"
  → 앞에 설명 문장을 붙임

"{\"tool\": \"get_orders\", \"args\": {\"customer\": \"김철수\", \"limit\": 3}"
  → 닫는 중괄호 누락

[enum 위반 예시]
{"tool": "get_customer_orders", ...}
  → 도구 목록에 없는 이름을 지어냄
```

프롬프트를 아무리 다듬어도 확률적으로 남는 실패다.

### 라운드 1 - 문법 제약 디코딩

#### 제약 디코딩이 하는일

프롬프트로 json을 부탁하는 것은 확률적이라 모델이 다음 토큰을 고를때 `{` 가 나올 확률이 높긴 하지만 `물`도 0이 아니고 5,000번중 187번은 그 쪽이 뽑힌다.

제약 디코딩은 부탁하는 대신 선택적으로 물리적으로 막는다.

```
현재까지 생성: {"tool": "get_
문법이 허용하는 다음 토큰: orders  profile   ← enum에 있는 것만
                             ↓
모델의 원래 확률 분포        [orders 0.6][profile 0.3][customer 0.08][물 0.02]
마스킹 후                    [orders 0.67][profile 0.33][  0  ][  0  ]
                                                        ↑ -inf로 만들어 샘플링 불가
```

존재하지 않는 도구 이름을 지어내는 것 자체를 못하게 만들고 확률을 0으로 만들어버린다.

#### XGrammer

이 마스킹을 매 토큰마다 수행하는 엔진으로 SGLang의 기본 문법백엔드고 `--grammar-backend`로 `llguidance` `outlines`로 바꿀 수 있다.

문제는 매 토큰마다 어휘 15만개를 전부 검사하면 그 자체가 병목이라는 점이다. XGrammer는 토큰을 두 부류로 나눠 이를 피한다.

```
문맥 독립 토큰   현재 파싱 상태와 무관하게 유효성이 정해짐
                 예: 문자열 안의 일반 문자는 중첩 깊이와 상관없이 항상 허용
                 → 미리 계산해 둠

문맥 의존 토큰   파싱 스택을 봐야 판단 가능
                 예: `}`는 열린 객체가 있어야만 유효
                 → 실행 시 검사하되 GPU 계산과 겹쳐서 수행
```

어휘의 대부분은 문맥 독립이고 구조를 만드는 토큰 `{ } [ ] , : "`는 수십개 뿐이라 나머지 전부 내용물들이기에 실시간 판단이 필요한것은 소수다 그마저 gpu가 다음 토큰을 계산하는동안 cpu가 병렬로 처리해버린다.

XGrammer 백엔드를 켜고 요청마다 JSON 스키마를 전달해보자

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
정상                          5000 (100.00%)
JSON 파싱 실패                    0
스키마 위반                       0
```

```
                  라운드 0    라운드 1     변화
실패율              3.74%      0.00%     해소
Median TTFT (ms)   412.33     468.90    +13.7%
P99 TTFT (ms)      788.21    1342.55    +70.3%
Output tok/s      4128.60    4402.18     +6.6%`
```

실패율은 0퍼센트가 되었다.

문법상 불가능한 토큰의 확률을 0으로 만들기 때문에 확률적 실패가 원리적으로 사라진다

`enum` 위반이 사라진 것이 특히 중요한데, 존재하지 않는 도구 이름을 지어내는 것 자체가 불가능해졌다.

#### 점프-포워드 디코딩

위에서 Output tok/s의 처리량이 6.6퍼가 는걸 확인할 수 있다.

이는 SGLang이 추가로 얹는 최적화로 제약 디코딩이 **틀린 답을 막는다면** 점프 포워드는 정해진 답을 건너뛴다.

```
{"tool": "  를 생성할 때

일반 제약 디코딩
  {  →  "  →  tool  →  "  →  :  →  (공백)  →  "     모델 호출 7회
  ↑ 각 위치에서 유효한 토큰이 하나뿐인데도 매번 모델에게 물어봄

점프-포워드
  {"tool": "  를 한 번에 방출                        모델 호출 0회
  ↑ 문법이 이미 답을 알고 있으므로 그냥 씀
```

다음 토큰의 엔트로피가 0인데 모델을 호출하는 것은 낭비다.

그래서 **구조 비중이 큰 출력은 제약을 걸었을 때 오히려 빨라진다.**

### 결과

- **실패율 해소**: 문법상 불가능한 토큰의 확률을 0으로 만들기 때문에, 확률적 실패가 원리적으로 차단되어 라운드 0에서 가장 많았던 enum위반, 즉 존재하지 않는 도구 이름을 지어내는 사례가 없어졌다
- **처리량 6.6퍼상승**: 점프-포워드 디코딩효과로 도구 호출 json은 120토큰중 구조 토큰이 절반가까이 되므로 그만큼 모델 호출이 준거다
- **P99 TTFT는 70.3퍼 악화**: 문법 컴파일 비용으로 새 스키마가 들어오면 XGrammer가 파싱 오토마톤을 만들고 문맥 독립 마스크를 사전 계산해야하는데 그 스키마의 첫 요청을 비용이 전부 부담한다. 이후 요청은 캐시를 쓴다.

중앙값이 13.7%만 오르고 P99가 70.3% 오른것이 대부분의 요청은 캐시 적중이고 컴파일에 걸린 소수가 p99를 높여버린거다

서버로그를 보면

```
[2026-08-07 09:12:04] Grammar cache miss, compiling schema (hash=a3f21b8c) ... 384ms
[2026-08-07 09:12:07] Grammar cache miss, compiling schema (hash=7d90e412) ... 411ms
[2026-08-07 09:12:11] Grammar cache hit (hash=a3f21b8c)
```

### 라운드 2 - 문법 캐시 워밍

이 서비스의 스키마는 도구 조합에 따라 정해지고 종류가 유한하기에

실제 트래픽에 등장한 스키마는 17종이니까 서비스 시작시 전부 한번씩 태워 컴파일 캐시를 채워버리자

```bash
#!/usr/bin/env bash
# warm_grammar.sh — 문법 캐시 사전 컴파일

for schema in schemas/*.json; do
  curl -s http://127.0.0.1:30000/generate \
    -H "Content-Type: application/json" \
    -d "$(jq -n --arg s "$(cat $schema)" \
         '{text: "warmup", sampling_params: {max_new_tokens: 1, json_schema: $s}}')" \
    > /dev/null
  echo "컴파일 완료: $schema"
done
```

```
                  라운드 1    라운드 2     변화
Median TTFT (ms)   468.90     421.07    -10.2%
P99 TTFT (ms)     1342.55     612.33    -54.4%
Output tok/s      4402.18    4488.90     +2.0%
실패율               0.00%      0.00%
```

P99가 절반 이하로 떨어졌다 컴파일 비용은 사라지지 않고 **서비스 시작 시점으로 옮겨졌을 뿐**이며 사용자 요청이 그 비용을 부담하지 않게 되었다.

스키마가 동적으로 생성되어 사전 컴파일이 불가능한 경우도 있다. 사용자가 커스텀 도구를 등록할 수 있는 구조라면 스키마 종류가 무한하다. 이 경우 백엔드별 특성을 확인해야한다.

```
백엔드          동적 스키마    컴파일 시간(복잡 스키마)    토큰당 오버헤드
xgrammar          지원          40~400 ms                < 40 µs
llguidance        지원          거의 없음                 ~50 µs
outlines          제한적        40 s ~ 10 분              조회 1회
```

`outlines`는 FSM의 완전히 사전 계산하므로 정적 스키마에서는 토큰당 비용이 가장낮지만, 복잡한 스키마에서 컴파일이 수 분 걸리고 재귀 구조를 다루지 못한다.

**동적 스키마가 있으면** `xgrammer` 또는 `llguidance`가 선택지이며 반드시 실제 스키마로 검증해야한다.

### 라운드 3 - 도구 정의를 접두사로 배치

입력 1,800 토큰 중 1,200 토큰이 도구 정의이고 전 요청이 동일하다 그런데 캐시 적중률이 낮았다.

```bash
curl -s http://127.0.0.1:30000/metrics | grep cache_hit_rate
# sglang_cache_hit_rate{model="Qwen/Qwen3.6-35B-A3B"} 0.18
```

프롬프트 구성을 확인하니 대화 이력이 도구 정의보다 앞에 있었다.

```
수정 전: [대화 이력 600][도구 정의 1,200][사용자 질문]
                └ 매번 다름 ┘ → 뒤의 1,200토큰까지 캐시 무효

수정 후: [도구 정의 1,200][대화 이력 600][사용자 질문]
         └ 항상 동일 ┘ → 1,200토큰 캐시 적중
```

```
                  라운드 2    라운드 3     변화
cache_hit_rate       0.18       0.67      +0.49
Median TTFT (ms)   421.07     198.44    -52.9%
P99 TTFT (ms)      612.33     341.20    -44.3%
Output tok/s      4488.90    5920.33    +31.9%
req/s               37.41      49.34    +31.9%
```

자주 보던 패턴이지요. 고정 부분을 앞에, 변하는 부분을 뒤에 배치하는 원칙은 워크로드를 가리지 않는다.


### 최종 구성

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

# 기동 후
./warm_grammar.sh
```

```
라운드                    실패율   Median TTFT   P99 TTFT   req/s
0 프롬프트 지시만           3.74%       412         788     34.41
1 XGrammar                0.00%       469        1343     36.68
2 문법 캐시 워밍            0.00%       421         612     37.41
3 도구 정의 앞으로          0.00%       198         341     49.34

누적                      해소       -52%        -57%     1.43×
```

피크 40 req/s였는데 49.34 req/s로 처리할 수 있게 만들어 SLO를 모두 만족한다.

### 결과 총총

**제약 디코딩은 확률적 실패를 원리적으로 제거한다.** 프롬프트 지시는 아무리 다듬어도 몇 %가 남고, 에이전트처럼 단계가 누적되는 구조에서는 그 몇%가 치명적이다.

**구조화 출력이 오히려 처리량을 올릴 수 있다.** 점프-포워드 디코딩이 결정된 토큰을 모델 호출 없이 방출하므로, 구조 비중이 큰 JSON 출력은 제약 없는 생성보다 빠르다. 제약을 비용으로만 생각하면 이 이득을 놓칠 수 있다.

**문법 컴파일은 비용을 옮길 수 있다.** 스키마 종류가 유한하면 워밍으로 사용자 요청에서 걷어낼 수 있기에 무한하면 백엔드를 잘 선택해야한다.

**MoE, dense의 유불리가 워크로드로 갈린다.** 케이스 1(출력 400토큰 동시성 25 상황)에서는 dense가 여기 출력(120토큰 동시성 40)에서는 MoE가 유리했다. 판단 기준은 배치가 임계 배치에 얼마나 가까운지 출력 길이다.

<br>

## 대형 MoE 대규모 서빙

| 항목 | 값 |
|---|---|
| 서비스 | 공개 API. 일반 대화·코딩 혼합 |
| 요청률 | 평균 80 req/s, 피크 140 req/s |
| 입력 | 평균 3,000토큰 |
| 출력 | 평균 800토큰 |
| 하드웨어 | 16노드 × 8×H200 = 128 GPU, NVLink + InfiniBand NDR |
| SLO | TTFT p95 < 3,000ms, ITL 중앙값 < 60ms |
| 최우선 지표 | **출력 100만 토큰당 원가** |

### 모델 선택

| 모델 | 총/활성 | FP8 가중치 | 코딩 | 특징 |
|---|---|---:|---|---|
| DeepSeek-V4-Flash | 284B / 13B | 284 GB | 강 | MLA + 희소 어텐션 |
| GLM-5.2 | 744B / 40B | 744 GB | 최상 | MLA + DSA + IndexShare |
| Qwen3.5-397B-A17B | 397B / 17B | 397 GB | 강 | 하이브리드 선형 어텐션 |

선택하기 나름이겠지먄 **DeepSeek-V4-Flash**를 선택하겠다.

활성파라미터 13B로 후보군중에 가장 작다. 디코드마다 읽는 가중치가 13GB(FP8) 로 GLM-5.2 40GB대비 3분의 1이다. MLA로 KV캐시가 압축되어 동시요청수에도 유리하다.

GLM-5.2는 코딩 성능이 더 높지만 활성이 40B라서 토큰당 원가가 약 3배가 된다 코딩 전용 서비스라면 다른 결론이 나오지만 일반 대화가 섞인 혼합 트래픽에서는 원가 우위가 딥싴이 더 크다

### 필요 처리량과 목표 원가

```
피크 140 req/s
  입력 처리량 = 140 × 3,000 =  420,000 tok/s
  출력 처리량 = 140 ×   800 =  112,000 tok/s

목표 원가
  H200 노드 시간당 약 $32 (8장 기준, 클라우드 온디맨드 가정)
  16노드 = $512/시간
  피크 지속 시 출력 112,000 tok/s × 3,600 = 4.03e8 tok/시간
  → $512 / 403 (백만 토큰 단위) ≈ $1.27 / 1M 출력 토큰
```

이 값으로 튜닝을 낮추는것이 목표다.

### 라운드 0 - TP8 x 16 복제

가장 단순한 구성부터 시작하자 노드마다 독립 인스턴스를 띄우고 라우터로 묶는다.

```bash
# 각 노드에서
python -m sglang.launch_server \
  --model-path deepseek-ai/DeepSeek-V4-Flash \
  --tp-size 8 \
  --quantization fp8 --kv-cache-dtype fp8_e5m2 \
  --chunked-prefill-size 8192 \
  --mem-fraction-static 0.88 \ 
  --host 0.0.0.0 --port 30000 --enable-metrics

# 라우터
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
원가 = $512 / (57,024 × 3600 / 1e6) ≈ $2.49 / 1M 출력 토큰
```

피크 140 req/s대비 71.28req/s로 절반이나 부족하다 TTFT, ITL 모두 SLO를 초과한다.

노드 하나의 지표를 보면

```
sglang_num_running_reqs  80.5
sglang_num_queue_reqs    7.4
sglang_token_usage       0.79
```

`token_usage` 0.79는 KV 캐시에 여유가 있다는 뜻인데 큐가 쌓이고 있다.

메모리가 아니라 **연산과 통신이 병목** 284B MoE를 TP8로 나누면 전문가 가중치가 8장에 조각조각 흩어져 어떤 전문가가 선택되든 8장 전체가 통신에 참여해야한다.

### 라운드 1- 전문가 병렬과 DP 어텐션

MoE는 전문가를 통째로 GPU에 배정하는 편이 통신에 유리하다.

어텐션은 MLA의 압축 이점을 살리기위해 데이터를 병렬로 돌린다.

```bash
# 노드 4개를 하나의 인스턴스로 묶음 (32 GPU), 총 4 인스턴스
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
128 GPU를 어떻게 쪼갤지부터 정한다.

전체        16노드 × 8 GPU = 128 GPU        (5.1 하드웨어)
인스턴스     4노드 × 8 GPU =  32 GPU        (하나의 모델을 서빙하는 단위)
인스턴스 수  128 ÷ 32      =   4개
```

노드 4개로 묶는 이유는 모델 가중치가 FP8로 284GB이다. H200 한장이 141GB이므로 최소 3장이면 가중치는 들어가지만 kv 캐시 여유가 거의 남지 않는다. 여기에 전문가 256개를 나눠 담으려면 gpu수가 충분해야하고 32 gpu면 전문가를 gpu당 8개씩 균등 배분할 수 있다.

왜 더 크게 묶지 않는가? 답을 하면 인스턴스가 커질수록 all to all 통신 범위가 넓어지고 노드간 InfiniBand를 더타게된다 32gpu는 노드 4대이므로 통신의 상당 부분이 노드 내부 NVLink에서 처리된다.

- `--nnodes 4`: 몇 대의 노드를 한 인스턴스로 묶을지를 지정하는 값이도 노드 4 대 = 32 GPU가 하나의 모델을 서빙한다.
- `--node-rank $NODE_RANK`: 이 노드가 그 중 몇번째인지 노드마다 0 1 2 3 을넣어 실행한다.
- `--dist-init-addr node01:5000`: 노드들이 서로를 찾는 접선 장소. 0번 노드가 랑데부 지점(만나는 장소) 역할을 한다.
- `--tp-size 32`: 전체 병렬 세계의 크기로 32gpu전체를 가리키고 아래 두 인자가 이 안을 나눈다.
- `--enable-ep-node`: MoE층을 전문가 병렬로 처리, 켜지 않으면 하나의가중치 행렬이 32조각으로 잘려 32개 gpu에 흩어지므로 어떤 전문가가 뽑히든 32gpu전부 계산에 동원된다 켜면 전문가를 통째로 한 gpu에 두어 전문가가 가진 gpu만 일을 하게 된다.
- `--ep-size 32`: 전문가를 몇 갈래로 배정할건지 32gpu가 전문가 256개를 8개씩 (256 / 32) 나눠 담당함.
- `--enable-dp-attention`: 어텐션 층만 데이터 병렬로 처리. MLA 압축 KV가 복제되지 않게한다.
- `--dp-size`: 전문가를 몇 갈래로 나눌지 8개 그룹이 각각 다른 요청 묶음을 담당하며 그룹당 gpu 4장 (32 / 8)이 배정된다.

`--tp-size`가 32인데 EP, DP가 그 안에 들어있는 구조가 처음에는 헷갈린다, 층 종류에 따라 32 GPU를 다르게 쓴다는 뜻이다.

```
어텐션 층 (DP=8)
  GPU 0~3   : 요청 그룹 A의 어텐션을 온전히 처리 (자기 KV만 보유)
  GPU 4~7   : 요청 그룹 B
  ...        각 그룹이 독립. 그룹 간 통신 없음

MoE 층 (EP=32)
  GPU 0     : 전문가 0~7 담당
  GPU 1     : 전문가 8~15 담당
  ...
  GPU 31    : 전문가 248~255 담당
             토큰을 담당 GPU로 보내고(all-to-all) 결과를 되받음
```

- **어텐션만 따로 때는 이유:** 딥시크 계열은 MLA로 KV캐시를 압축해 저장한다 어텐션을 TP로 나누면 각 gpu가 압축된 kv를 중복 보관하게 되어 압축해서 아낀 메모리를 복제로 다시 날린다. DP로 처리하면 GPU마다 자기가 맡은 요청의 KV만 들고있으면 된다.
- **MoE를 EP로 두면 통신량이 어떻게 달라지는가:** TP방식은 전문가 조각이 흩어져있어 층마다 32gpu를 전체가 결과를 맞추게 되는 all to all 통신으로 오버헤드가 컸다. 토큰 하나가 실제로 쓰는 전문가는 8개뿐인데 32 gpu 전부가 참여하는 셈이다. EP는 토큰을 담당 GPU로 보내고 되받는 all to all만 하므로, 오가는 것이 가중치가 아니라 토큰이고 참여 범위도 좁다.

```
                     라운드 0    라운드 1     변화
req/s                  71.28     118.44     +66.2%
Output tok/s          57024      94752      +66.2%
Median TTFT (ms)      4218.90    2814.33    -33.3%
P99 TTFT (ms)        11840.22    6120.88    -48.3%
Median ITL (ms)         88.41      61.20    -30.8%
원가 ($/1M 출력)         2.49       1.50     -39.8%
```

#### 처리량 66.2% 상승

전문가 병렬로 gpu가 전문가 8개만 담당하게 되면서 gpu당 가중치 메모리가 줄었고 그만큼 kv캐시에 쓸 공간이 늘어 동시 요청도 증가했다 통신도 층마다 전체 all-reduce를 하던것에서 선택된 전문가로 토큰을 보내는 all to all로 바뀌어 총량이 줄었다.

#### 지연 TTFT 33.3%, ITL 30.8% 개선

DP 어텐션 덕에 KV 복제가 사라지면서 gpu당 처리 가능한 요청수가 늘었고 큐 대기가 줄어 TTFT가 개선되었다 ITL은 통신 총량 감소가 직접 반영된 결과다.

#### 남은 문제 - 전문가 부하 편중

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
이게 균등하면 전문가 256개에 각 0.00039여야한다 최대 0.0281로 7배 편중되어있다 인기 전문가를 담당한 gpu가 병목이다.

쉽게 말해 너무 한 전문가들에게만 쏠리기 때문에 그 전문가 가중치를 들고있는 gpu가 부담이 된다.

### 라운드2 - 전문가 부하 분산

#### EPLB

EPLB는 Expert Paralleism Load Balancer의 약자로 전문가를 gpu에 어떻게 배치할지 다시 계산해주는 장치다.

라운드 1에서 배치는 단순히 순서대로 나눈것이였다.

```
GPU 0 : 전문가 0~7      GPU 1 : 전문가 8~15      ...      GPU 31 : 전문가 248~255
```

이 방식은 모든 전문가가 비슷하게 선택된다고 가정한다 실제로는 그렇지 않았고 47번 전문가가 평균의 7.2배로 뽑혔고 47번을 담당한 gpu 5는 계속 바쁘고 231번을 담당한 gpu 28은 놀게된다.

**MoE층은 모든 gpu가 끝나야 다음 층으로 넘어가므로 가장 느린 gpu가 전체 속도를 결정한다.**

EPLB는 실제 라우팅 통계를 보고 두가지를 정한다.

```
1. 인기 전문가를 복제한다
   전문가 47을 GPU 5와 GPU 19 두 곳에 둠
   → 47번으로 갈 토큰이 두 GPU로 나뉨

2. 배치를 재계산한다
   인기 전문가끼리 같은 GPU에 몰리지 않도록 자리를 바꿈
```

즉 시간이 지날수록 elpb는 배치구성을 재계산해 라우팅하도록 변하게해주는 메커니즘으로 초기에는 고정으로 시작해서 점차 바뀌는걸 확인할 수 있을 것이다.

> 은행 창구에 번호를 순서대로 배정했는데 알고보니 3번 업무를 보러 오는 손님이 다른 창구의 7배였다 3번 창구만 줄이길고 나머지는 한산하다 EPLB는 그래서 3번 업무 창구를 두개로 늘리고 다른 인기 업무 창구가 몰리지않게 자리를 재배치한다.

EPLB를 켜서 인기 전문가를 여러 GPU에 복제한다.

```diff
    --enable-ep-moe \
    --ep-size 32 \
+   --enable-eplb \
+   --eplb-algorithm deepseek \
+   --eplb-rebalance-num-iterations 500 \
+   --ep-num-redundant-experts 32 \
```

- `--enable-eplb`: 부하 분산기를 키고 끄면 라운드1처럼 고정 배치로 간다
- `--eplb-algorithm deepseek`: 재배치를 계산하는 알고리즘으로 DeepSeek이 공개한 방식이다 계층 구조를 고려해 노드 내부 통신을 우선한다.
- `--eplb-rebalance-num-iteration 500`: 몇 번의 포워드마다 배치를 다시 계산할지 500이면 500스텝치 라우팅 통계를 모아 판단한다. 너무 짧으면 통계가 흔들리고 너무 길면 트래픽 변화를 늦게 따라간다.
- `--ep-num-redundant-expert 32`: 복제본을 몇개까지 만들지 정하는 숫자로 32개를 추가하므로 전문가 슬롯이 256에서 288개가 되고 인기전문가 32개가 두 곳에 존재하게 된다.

```
                     라운드 1    라운드 2     변화
req/s                 118.44     139.86     +18.1%
Output tok/s           94752     111888     +18.1%
Median TTFT (ms)      2814.33    2402.11    -14.6%
Median ITL (ms)         61.20      52.44    -14.3%
원가 ($/1M 출력)         1.50       1.27     -15.3%
```

```
전문가 부하 편중도 (최대/평균)
  라운드 1: 7.2배
  라운드 2: 1.9배
```

중복 전문가 32개를 추가하는 대가로 가중치 메모리가 12.5% (32 / 256) 늘었지만 부하 편중이 해소되면서 처리량이 18퍼센트 올랐다.

피크 140 req/s에서 139.86 req/s까지 거의 도달했다.

### 라운드3 - PD 분리

프리필과 디코드는 최적 설정이 반대다 프리필은 계산 바운드라 작은 TP로 여러 인스턴스가 유리하고 디코드는 메모리 바운드라 큰 EP와 큰 배치가 유리하다 하나의 인스턴스에 묶으면 양쪽 다 타협하게 된다.

입출력 비율 3,000:800에서 프리필과 디코드의 부하를 추정하고 노드를 배분한다.

```
프리필 부하 = 420,000 tok/s (피크)
디코드 부하 = 112,000 tok/s (피크)

프리필은 토큰당 계산이 무겁고 디코드는 토큰당 메모리 접근이 무거움
실측 기반 초기 배분: 프리필 5노드 / 디코드 11노드
```

```bash
# 프리필 노드 (5노드, 각 TP8)
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

# 디코드 노드 (11노드를 하나의 EP 도메인으로, 88 GPU)
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

# 라우터
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

#### PD 분리 전용 인자

양쪽 노드에 공통으로 들어가는 것들이다

- `--disaggregation-mode prefill / decode` 이 인스턴스가 어느 역할인지 선언한다 프리필 노드는 KV 캐시를 만들어 넘기고 거기서 끝내고 디코드 노드는 KV를 받아서 생성만한다.
- `--disaggregation-transfer-backend mooncake`: 노드간 kv 전송을 무엇으로 할지 정하는 인자인데 Mooncake Transfer Engine은 RDMA로 GPU 메모리 끼리 직접 주고받아 cpu를 거치지 않는다. `nixl`이라는것도 있다
- `--disaggregation-bootstrap-port 8998`: 디코드 노드가 프리필 노드에 접선하는 포트로 프리필 쪽에만 지정하고 라우터 설정에서 `--prefill http:/node01:30000 8998` 처럼 서빙 포트 뒤에 같이 적는다.
- `--disaggregation-ib-device mlx5_0`: 어느 InfiniBand 장치로 전송할지 정한다 `ibv_devinfo`로 확인한 이름을 넣는다 지정하지 않으면 이더넷으로 떨어져 전송 병목이 된다.

#### 프리필 노드

- `--tp-size 8`: 노드 하나가 독립된 인스턴스로 프리필은 토큰이 수천 개씩 들어와 gpu가 이미 꽉 차므로 병렬을 넓혀도 얻을 게 없고 통신 비용만 늘어난다 대신 노드 5개가 각자 다른 요청을 처리해 처리량을 낸다
- `--chunked-prefill-size 16384`: 라운드 2의 8192보다 높게 키웠다. 이 노드는 디코드를 하지 않으므로 긴 프리필이 남의 토큰 생성을 막을 걱정이 없다 조각을 크게 잡아 행렬 연산의효율을 올린다.
- `--mem-fraction-static 0.85`: 디코드 노드보다 낮게 잡혀있다 프리필 노드의 kv는 만들어서 넘기면 끝이라 오래 보관할 필요 없고 대신 프리필중 활성값이 크게잡히므로 여유를 둔다/
- **EP DP 인자 없음:** 프리필은 배치가 작아 대규모 ep의 이점이 나오지 않고 all to all 통신 비용만 커지기 때문에 넣지 않았다.

#### 디코드 노드

- `--tp-size 88 --nnode 11`: 11노드를 하나로 묶고 디코드는 메모리 바운드라 배치를 최대한 키워야하고 배치를 키우려면 kv캐시 공간이 많아야한다 노드를 많이 묶을수록 gpu당 가중치 부담이 줄어 kv에쓸 공간이 늘어난다.
- `--ep-size 88`: 전문가 256개를 88gpu에 나눈다. gpu당 약 3개골이라 라운드2(8개)보다 가중치 부담이 더 줄어든다.
- `--dp-size 22`: 88 / 22 그룹당 gpu 4장, 라운드 2와 같은 비율이다.
- `--ep-num-redundant-experts 88`: ep 크기에 맞춰 복제본도 늘렸다 랭크 수가 커질수록 편중의 영향이 커지기 때문이다.
- `--max-running-requests 2048`: 동시 시퀀스 상한. 디코드 전용이므로 프리필과 자리를 다툴일이 없어 크게 잡는다.
- `--mem-fraction-static 0.90`: kv캐시를 최대한 확보한다 이노드의 성능은 곧담을수있는 요청수다.

라우팅 정책을 프리필과 디코드에 다르게 준 것이 핵심이다 프리필은 접두사 재사용이 의미 있으므로 `cache_aware`, 디코드는 캐시보다 부하 균형이 중요하므로 `power_of_two`를 쓴다.

`--prefill-policy`와 `--decode-policy`는 요청이 들어왔을 때 어느 노드로 보낼지 정하는 규칙이다. 네 가지 중에서 고른다.

- `round_robin`: 노드를 차례로 돌아가며 배정
- `random`: 무작위하나 선택
- `cache_aware`: 요청이 접두사를 해싱해 같은 접두사는 항상 같은 노드로
- `power_of_two`: 무작위 두곳을 뽑아 그중 덜 바쁜쪽 선택

`--policy A`하나만 쓰면 프리필 디코드에 같은값이 적용되고 나눠쓰면 각각 다르게 줄 수 있다

**프리필**에 cache_aware을 쓴 이유는 프리필 노드는 kv를 만드는 곳이고같은 접두사의 kv는 이미 만들어둔것을 재사용할 수 있다 접두사를 해싱해 같은 노드로 보내면 그 노드의 캐시에 이미 있을 확률이 높아진다. 라운드로빈으로 흩뿌리면 같은 시스템 프롬프트가 5개 노드에 중복 생성되어 캐시 용량이 실질적으로5분의 1이 된다.

**디코드**에 power_of_two를 쓰는 이유는 디코드 노드는 kv를 만들지 않고 프리필에서 받아쓰는데 접두사가 같든 다르든 재사용할 캐시가 없으므로 cache_aware가 무의미다 대신 어느 노드가 덜 바쁜지 중요하긴한데 매번 11개노드 전부의 부하를 확인하긴 라우터의 부담이라 무작위 두곳만 비교해도 균형이 충분히 잡힌다

> 왜 두곳만 봐도 되냐면 무작위 하나를 고르면 운 나쁘게 바쁜 노드에 걸릴 수 있다 두 곳응ㄹ 뽑아 바쁜쪽을 고르면 그 확률이 제곱으로 줄어든다 세 곳 네곳으로 늘려도 추가 이득은 급격히 작아진다.

#### 두 그룹 설정 대비

같은 모델인데 설정이 정 반대로 갈린다.

```
                      프리필              디코드
병목                  계산                메모리 대역폭
TP 크기               8 (노드 단위)        88 (전체 묶음)
EP                    사용 안 함           88랭크
청크 크기             16,384 (크게)        해당 없음
mem-fraction          0.85                0.90
최적화 목표           TTFT                처리량
```


```
                     라운드 2    라운드 3     변화
req/s                 139.86     198.20     +41.7%
Output tok/s          111888     158560     +41.7%
Median TTFT (ms)      2402.11    1884.66    -21.5%
P99 TTFT (ms)         5218.40    3402.18    -34.8%
Median ITL (ms)         52.44      44.90    -14.4%
원가 ($/1M 출력)         1.27       0.90     -29.1%
```

### 라운드 4 - 노드 배분 재조정

프리필과 디코드 노드의 활용률을 각각 확인한다.

```bash
# 프리필 노드
curl -s http://node01:30000/metrics | grep -E 'num_queue_reqs|token_usage'
# sglang_num_queue_reqs 18.2      ← 큐가 쌓임
# sglang_token_usage 0.42

# 디코드 노드
curl -s http://node06:30000/metrics | grep -E 'num_queue_reqs|token_usage'
# sglang_num_queue_reqs 0.4
# sglang_token_usage 0.61         ← 여유 있음
```

프리필쪽에 큐가 쌓이고 있고 디코드는 놀고있다 배분을 프리필쪽으로 옮긴다.

```
배분              req/s    Median TTFT   Median ITL   원가($/1M)
프리필 5 / 디코드 11   198.20      1884         44.9        0.90
프리필 6 / 디코드 10   214.44      1502         48.2        0.83
프리필 7 / 디코드 9    218.90      1344         56.1        0.81
프리필 8 / 디코드 8    206.12      1288         71.4        0.86   ← ITL SLO 초과
```


프리필 7 디코드 9에서 최적이다. 8 8은 오히려 처리량이 떨어지고 ITL 중앙값 71.4ms가 SLO 60ms를 넘는다.

이 표가 pd 분리의 핵심 이점을 보여주는데 노드 배분 비율 하나로 TTFT, ITL 균형을 조절할 수 있다.  통합 구성에서는 이 조절 손잡이가 없다.

### 최종 구성과 누적 효과

```
라운드                        req/s   Median TTFT   Median ITL   원가($/1M)
0 TP8 × 16 복제               71.28       4219         88.4        2.49
1 EP + DP 어텐션             118.44       2814         61.2        1.50
2 EPLB 부하 분산             139.86       2402         52.4        1.27
3 PD 분리 (5/11)             198.20       1884         44.9        0.90
4 배분 재조정 (7/9)           218.90       1344         56.1        0.81

누적                        3.07×        -68%        -37%       -67%
```

SLO 결과
1. TTFT p95 < 3,000ms는 최종 약 2,410ms정도로 달성되엇고
2. ITL 중앙값은 < 60ms도 56.1ms로 달성되었다.
3. req/s 도 140 이였는데 피크기준 218.9req/s로 여유있게 달성되었다.
4. 원가도 $2.49에서 $0.81로 67퍼 내려갔고 하드웨어는 끝까지 128GPU로 동일하다.

### 교훈

1. **대형 MoE에서 병렬화 전략이 원가를 지배한다**: 같은 하드웨어에서 TP만 쓴 구성과 EP + PD분리 구성의 차이가 3배나 난다.
2. **전문가 부하 편중은 반드시 측정해야한다.**: 7배 편중을 방치하면 인기 전문가를 담당한 gpu가 전체속도를 결정한다 `expert_load` 지푤르 상시 감시 항목에 넣어야한다/
3. **PD 분리의 진짜 가치는 조절 손잡이다.** 처리량 개선도 크지만 노드 배분 비율로 TTFT, ITL의 균형을 사후에 조절할 수 있다는 점이 운영에서 더 유용하다 트래픽 패턴이 바뀌면 비율만 조정하면 된다.
4. **PD 분리는 규모가 조건이다** 이 케이스는 128gpu고 InfiniBand NDR, Mooncake 전송계층이 있는데 만약 8 GPU같은 구성을 하면 양쪽 풀이 모두 비효율이라 오히려 느려진다.

<br>

## 다섯켕시ㅡ 비교

### 같은 기법들이 쓰이지만 최적화 포인트가 성향에 따라 좋아질수도있고 나빠질수있다.

| 기법 | 케이스 1 (코딩) | 케이스 2 (RAG) | 케이스 3 (배치) | 케이스 4 (구조화) | 케이스 5 (대규모) |
|---|---|---|---|---|---|
| 투기적 디코딩 | **+66%** | 이득 미미 | **-20%** | 미적용 | 미적용 |
| 청크 프리필 크기 | 4,096 | 8,192 | **32,768** | 8,192 | 16,384 |
| 모델 형태 | **dense 27B** | dense 27B | dense 9B | **MoE 35B-A3B** | MoE 284B-A13B |
| TP 크기 | 2 (복제 4) | 2 (복제 2) | 2 | 2 | 8 / 88 |
| `mem-fraction-static` | 0.90 | 0.88 | **0.94** | 0.90 | 0.85 / 0.90 |
| HiCache | 불필요 | **필수** | 불필요 | 불필요 | 검토 대상 |
| PD 분리 | 손해 | 손해 | 손해 | 손해 | **+42%** |

각각의 워크로드의 입출력 비용 동시성 공유율 규모등에서 최적화가 다르기때문에

```
입출력 비율
  입력 ≫ 출력 (케이스 2: 68:1)
    → 프리필 최적화 집중. 캐시·라우팅이 지배
    → 투기적 디코딩 이득 작음
  입력 ≈ 출력 (케이스 1: 5:1)
    → 균형. 청크 프리필로 간섭 차단
  출력 비중 큼
    → 디코드 최적화. 양자화·투기적 디코딩

동시성 대비 임계 배치
  동시성 ≪ 임계 배치 (케이스 1: 25 vs 206)
    → 메모리 바운드. 계산 자원 남음
    → 투기적 디코딩 유효, dense 유리
  동시성 ≈ 임계 배치 (케이스 3: 256 vs 295)
    → 전환 구간. 투기적 디코딩 손해
    → MoE 유리, 배치 극대화

접두사 공유율
  < 0.2  → RadixAttention 이득 없음
  0.6 이상 → 라우팅 정책이 성능을 지배. 프롬프트 구조 점검 필수

GPU 규모
  < 16   → 단일 노드 튜닝. PD 분리 손해
  > 64   → EP + PD 분리 검토 구간
```

### 지표 변화 패턴

| 관찰된 변화 | 의미 | 다음 행동 |
|---|---|---|
| 처리량 ↑, 지연 ↑ | 배치를 키운 효과 | SLO 여유가 있으면 유지 |
| 처리량 ↑, 지연 ↓ | 병목 자체를 제거 | 방향이 맞음. 더 밀어붙임 |
| 처리량 ↓, 꼬리 지연 ↓↓ | 안정성과 효율의 거래 | 지연 SLO가 있으면 유지 |
| 중앙값 그대로, P99만 ↓ | 꼬리 요청만 개선 | HiCache·워밍의 전형적 패턴 |
| 처리량 ↓, 지연 ↑ | 잘못된 방향 | 즉시 롤백 |
| 아무것도 안 변함 | 병목이 다른 곳 | 지표로 병목 재탐색 |


### 케이스별 최종 설정 요약

```bash
# 케이스 1 — 사내 코딩 어시스턴트 (워커 4개)
--model-path Qwen/Qwen3.6-27B --tp-size 2
--quantization fp8 --kv-cache-dtype fp8_e5m2
--chunked-prefill-size 4096 --mem-fraction-static 0.90
--speculative-algorithm EAGLE3 --speculative-num-steps 5 --speculative-eagle-topk 8
# 라우터: --policy cache_aware

# 케이스 2 — 문서 QA / RAG (워커 2개)
--model-path Qwen/Qwen3.6-27B --tp-size 2
--quantization fp8 --kv-cache-dtype fp8_e5m2
--chunked-prefill-size 8192 --mem-fraction-static 0.88
--page-size 64 --enable-hierarchical-cache --hicache-ratio 2
--hicache-write-policy write_through --hicache-io-backend kernel
--enable-cache-report
# 라우터: --policy cache_aware

# 케이스 3 — 야간 배치
--model-path Qwen/Qwen3.6-9B --tp-size 2
--quantization fp8 --kv-cache-dtype fp8_e5m2
--chunked-prefill-size 32768 --mem-fraction-static 0.94
--max-running-requests 512 --max-prefill-tokens 65536
--schedule-conservativeness 0.3
# 투기적 디코딩 사용 안 함

# 케이스 4 — 구조화 출력 API
--model-path Qwen/Qwen3.6-35B-A3B --tp-size 2
--quantization fp8 --kv-cache-dtype fp8_e5m2
--grammar-backend xgrammar
--chunked-prefill-size 8192 --mem-fraction-static 0.90
--max-running-requests 64 --enable-cache-report
# 기동 후 문법 캐시 워밍 필수

# 케이스 5 — 대형 MoE (프리필 7노드 / 디코드 9노드)
# 프리필
--tp-size 8 --disaggregation-mode prefill
--disaggregation-transfer-backend mooncake --disaggregation-ib-device mlx5_0
--chunked-prefill-size 16384 --mem-fraction-static 0.85
# 디코드
--tp-size 72 --nnodes 9 --disaggregation-mode decode
--enable-ep-moe --ep-size 72 --enable-dp-attention --dp-size 18
--enable-eplb --eplb-algorithm deepseek --ep-num-redundant-experts 72
--max-running-requests 2048 --mem-fraction-static 0.90
# 라우터: --pd-disaggregation --prefill-policy cache_aware --decode-policy power_of_two
```
