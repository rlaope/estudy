# Quantitative Evaluation Metrics for Search Systems (IR Metrics & Evaluation)

Quantitative evaluation metrics for search systems provide a mathematical framework to measure how accurate the results returned by the system are for a user's query (image/text) (precision), whether it missed any correct answers (recall), and whether it displayed the best correct answers at the top (ranking).

They serve as a crucial compass for A/B testing before tuning search engine parameters or deploying new AI models.

Today, we will explore the following key evaluation metrics (terms).

- **IR (Information Retrieval)**: A field of technology that finds information desired by users within vast amounts of data.
- **Recall**: The ratio indicating how many correct answers the system found without missing any from the total set of correct answers.
- **Precision**: The purity ratio indicating how many of the results retrieved by the system are truly correct answers.
- **mAP (Mean Average Precision)**: A metric that evaluates not only how many correct answers were found but also how well the best correct answers were displayed at the top (1st to 3rd positions).
- **NDCG (Normalized Discounted Cumulative Gain)**: A metric that measures the ranking quality of the current system between 0 and 1, by comparing it to an ideal ranking where correct answers are perfectly ordered.

### Origin of Names

First, let's examine the terms related to quantitative system evaluation metrics and their origins.

**IR (Information Retrieval)** refers to the field of computer science that involves retrieving data matching a user's information need from vast amounts of unstructured data (documents, images).

**mAP** directly reflects its mathematical operation in its name: Mean Average Precision, which calculates the average of precisions computed each time a correct answer is found, and then averages these across all queries.

NDCG is an acronym for the calculation itself: it discounts the value (Gain) as correct answers are pushed further down the ranking using a logarithmic function, and then normalizes it by comparing it to the score of a perfect ranking.

## Problem Addressed

There was a dilemma with **ANN (Approximate Nearest Neighbor)**, where vector search engines like LSH or FAISS intentionally sacrifice accuracy for speed. Increasing hash tables improves the probability of finding correct answers, but memory usage and search latency increase explosively.

**Limitations of heuristic-based tuning**: If engineers decide hash bits (e.g., 16 or 32) based on intuition, it can lead to server OOM crashes during traffic spikes or failures where customers cannot find desired products.

**Error of ignoring order**: Simply knowing that 3 out of 10 search results are correct is insufficient. A system where those 3 correct answers are ranked 1st, 2nd, and 3rd is vastly different in customer experience (click-through rate) from one where they are ranked 8th, 9th, and 10th, yet traditional simple hit measurement methods failed to distinguish this.

### Solution Approach

- **Establish an offline benchmark pipeline**: Automatically evaluate the system using tens of thousands of test queries and ground truth data before applying it to live servers.
- **Rank-Aware Metrics**: Introduce mAP and NDCG instead of simple recall/precision to mathematically score whether the most relevant products are positioned at the top.
- **Pareto Frontier**: Adopt an engineering approach by plotting two graphs with QPS (search speed) on the x-axis and Recall@K (recall rate) on the y-axis, to find the maximum recall point achievable within the target response time, thereby determining parameters (e.g., number of LSH tables).

### Recall & Precision

Let's understand this with an intuitive example. I'll use an analogy of a clothing store part-timer.

Imagine a very large warehouse with 10 red dresses. Two part-timers (search engines A and B) are asked to find red dresses (Query).

Here, **Recall** indicates how many correct answers were not missed. If part-timer A brought a pile of clothes from the warehouse, and upon inspection, 8 of them were red dresses, then 8 out of 10 correct answers were found, making the recall 80%.

Here, **Precision** indicates how much "trash" (incorrect answers) is among the retrieved items. If the part-timer brought 20 clothes, and 8 were correct answers while 12 were pants, yellow shirts, etc., then 8 out of 20 retrieved items were useful, meaning more than half were trash. This makes the precision 40%.

While they might seem similar, the denominator for recall is the total number of correct answers, whereas the denominator for precision is not the number of correct answers, but rather the number of attempts (items retrieved).

### AP, mAP

Next, **AP (Average Precision)** measures the quality of the order (ranking). Part-timers A and B both brought 20 clothes, and both had 8 correct answers mixed in. However, part-timer A handed over red dresses consecutively from the 1st to the 8th item, and then gave irrelevant clothes. This is "top rank dominance".

On the other hand, if part-timer B only gave irrelevant blue pants and then started giving red dresses from the 13th item, it means the correct answers were pushed to lower ranks.

From a home shopping customer's perspective, part-timer A, who showed the correct answers at the top of the first page, is an overwhelmingly superior search engine. Even if both found 8 items, AP (Average Precision) is the mathematical metric that assigns different weights based on ranking, giving part-timer A a much higher score.

Here, AP is a score for the results brought by a single search, while mAP is the average of AP scores across hundreds or thousands of different queries, used to finally evaluate the system's performance.

<br>

## NDCG

NDCG is a metric for evaluating the ranking of search results. Instead of simply classifying answers as correct or incorrect, it assigns relevance scores (e.g., 3 points, 2 points, 1 point) to evaluate the quality of the ranking on a scale from 0 to 1.

- **CG (Cumulative Gain)**: The sum of relevance scores of retrieved results, regardless of their rank.
- **DCG (Discounted Cumulative Gain)**: As rank decreases, the value to the user diminishes, so scores are added after being discounted by a logarithmic function.
- **NDCG (Normalized ~)**: Since the perfect score varies per query, the current ranking score (DCG) is divided by the ideal perfect score (IDCG) when results are perfectly sorted (Normalized), creating an absolute evaluation score between 0 and 1.

The AP/mAP we learned earlier can only be calculated when an item is either correct (1) or incorrect (0). However, in actual home shopping searches, when searching for "red dress," there might be partial scores, such as a red dress being 3 points, a red skirt 2 points, and a pink dress 1 point. AP cannot reflect these subtle quality differences in its calculation.

### Solution Approach

Introduce Relevance Scores, assigning a weight of $rel_i$ to the results.

**Another penalty is applied (logarithmic discount)**: For example, a 3-point correct answer on page 10 of the search results is less valuable than a 3-point correct answer on the first page. Therefore, the score is divided by $\log_2(rank + 1)$, which increases the denominator as the rank increases.

Let's use the clothing store part-timer example again. Part-timer A was asked to find an autumn trench coat. If a perfect match is 3 points, only matching color is 1 point, and trash is 0 points:

1. First item: trash, 0 points
2. Second item: perfect trench coat, 3 points
3. Third item: similar beige jacket, 1 point
4. **CG (Simple Sum)**: 0 + 3 + 1 = 4 points. Ignoring the order hides the mistake of presenting trash first.
5. **DCG (Discounted Sum)**: A penalty is applied because the 3-point item was given second and the 1-point item third (e.g., 0 + 1.8 + 0.5 = 2.3 points).
6. **IDCG (Perfect Score Standard)**: If the part-timer had perfectly handed them over in the order 3, 1, 0, the ideal score is calculated (3 + 0.6 + 0 = 3.6 points). Penalties are still applied.
7. **NDCG**: Part-timer A's final score is concluded as 2.3 / 3.6 (based on the example above) = 0.63 points, which is my score divided by the perfect score.

Mathematically, DCG and IDCG are defined as follows. p denotes the top K ranks to be evaluated.

$$DCG_p = \sum_{i=1}^{p} \frac{rel_i}{\log_2(i+1)}$$

- $rel_i$: Relevance score of the document at the $i$-th rank (e.g., 0, 1, 2, 3)
- $\log_2(i+1)$: Role of increasing the denominator to discount the score as the rank goes down.

IDCG (Ideal DCG) is the value of the most ideal ranking DCG, obtained by sorting the currently retrieved results in descending order of their relevance scores $rel_i$, and then applying the same DCG formula.

$$NDCG_p = \frac{DCG_p}{IDCG_p}$$

```py
import numpy as np

def calculate_ndcg(relevance_scores):
    """
    관련성 점수 배열을 받아 NDCG를 계산합니다.
    예: relevance_scores = [0, 3, 1] (1등 0점, 2등 3점, 3등 1점)
    """
    # 1. DCG 계산
    dcg = 0.0
    for i, rel in enumerate(relevance_scores):
        rank = i + 1
        # 공식: rel / log2(rank + 1)
        dcg += rel / np.log2(rank + 1)
        
    # 2. IDCG 계산을 위한 이상적인 정렬 (내림차순)
    # [0, 3, 1] -> [3, 1, 0] 으로 정렬
    ideal_scores = sorted(relevance_scores, reverse=True)
    
    idcg = 0.0
    for i, rel in enumerate(ideal_scores):
        rank = i + 1
        idcg += rel / np.log2(rank + 1)
        
    # 예외 처리: 가져온 결과가 전부 0점이라 IDCG가 0인 경우
    if idcg == 0:
        return 0.0
        
    # 3. NDCG 계산 (현재 DCG / 이상적인 만점 DCG)
    ndcg = dcg / idcg
    return ndcg

# ==========================================
# 실행 시뮬레이션
# ==========================================
# 알바생 A: 엉뚱한 걸(0점) 1등으로 가져오고, 진짜 정답(3점)을 2등으로 가져옴
scores_A = [0, 3, 1, 0, 0] 
ndcg_A = calculate_ndcg(scores_A)

# 알바생 B: 가장 완벽한 정답(3점)을 1등으로 가져오고, 그 다음 정답(1점)을 2등으로 가져옴
scores_B = [3, 1, 0, 0, 0]
ndcg_B = calculate_ndcg(scores_B)

print(f"알바생 A의 NDCG: {ndcg_A:.3f}") # 0.695
print(f"알바생 B의 NDCG: {ndcg_B:.3f}") # 1.000 (완벽한 순서이므로 만점)
```

<br>

## Search Engine Evaluation Pipeline Operation Principle

An actual search engine evaluation pipeline consists of a 4-stage architecture.

#### 1. Ground Truth Collection Layer

- **Explicit:** Humans directly label product A and product B as similar.
- **Implicit:** Analyze user behavior logs (if a user who clicked product A also clicked and purchased product B, the two vectors are considered similar).

#### 2. Parameter Grid Search Layer

The evaluation script automatically generates dozens of temporary indices by varying Elasticsearch's LSH parameters (num_tables, hash_length) or HNSW parameters m, ef_construction.

#### 3. Scoring Engine Layer

After submitting a query and receiving K results, the rank-aware metric Average Precision is calculated.

AP is calculated by averaging the accumulated Precision each time a correct answer is found.

#### 4. Trade-off Analysis Layer

The calculated mAP scores and query processing time (Latency) are mapped to generate a report, and the optimal system parameters are determined.

### Example

This is the logic for calculating Average Precision AP@K, a rank-aware metric widely used to evaluate search quality beyond simple intersection calculations.

This code assigns higher scores as correct answers are ranked higher.

```py
def calculate_average_precision_at_k(retrieved_ids, ground_truth_ids, k=10):
    """
    순위를 고려하여 검색 품질을 평가하는 Average Precision @ K 를 계산합니다.
    
    :param retrieved_ids: 검색 엔진이 점수 순(랭킹)으로 정렬하여 반환한 예측 ID 리스트
    :param ground_truth_ids: 실제 정답 상품 ID 리스트
    :param k: 평가할 상위 K 개의 기준
    :return: AP@K 점수 (0.0 ~ 1.0)
    """
    
    # 1. 상위 K개의 결과만 추출
    top_k_retrieved = retrieved_ids[:k]
    gt_set = set(ground_truth_ids)
    
    # 예외 처리: 정답이 아예 없는 쿼리면 0 반환
    if not gt_set:
        return 0.0

    hit_count = 0        # 찾은 정답의 누적 개수
    sum_precisions = 0.0 # 정답을 찾을 때마다의 정밀도 합계
    
    # 2. 순위(Rank)를 1등부터 K등까지 순회하며 검사
    for i, item_id in enumerate(top_k_retrieved):
        rank = i + 1 # 랭킹은 1부터 시작 (인덱스 + 1)
        
        # 3. 만약 현재 순위의 상품이 정답지에 존재한다면 (Hit!)
        if item_id in gt_set:
            hit_count += 1
            
            # 4. 정답을 발견한 "바로 그 시점(Rank)"까지의 Precision을 계산하여 누적
            # 공식: (지금까지 찾은 정답 수) / (지금까지 탐색한 검색 결과 수)
            current_precision = hit_count / rank
            sum_precisions += current_precision
            
            # [디버깅용 출력]
            # print(f"  -> Rank {rank}에서 정답 발견! 현재까지의 Precision: {current_precision:.2f}")

    # 5. Average Precision 연산
    # (누적된 Precision의 합) / (실제 전체 정답 수와 K 중 작은 값)
    # k개 안에 들어갈 수 있는 최대 정답 수로 나누어 정규화(Normalize) 합니다.
    total_possible_hits = min(len(gt_set), k)
    
    if total_possible_hits == 0:
        return 0.0
        
    average_precision = sum_precisions / total_possible_hits
    return average_precision

# ==========================================
# 실행 시뮬레이션: 왜 "순위"가 중요한가?
# ==========================================
ground_truths = ["item_A", "item_B", "item_C"]

# Case 1: 훌륭한 검색 엔진 (정답을 1등, 2등에 올려놓음)
result_good = ["item_A", "item_B", "item_X", "item_Y", "item_Z"]
# 연산: 
# Rank 1 (item_A) -> Precision: 1/1 = 1.0
# Rank 2 (item_B) -> Precision: 2/2 = 1.0
# AP = (1.0 + 1.0) / 3(총정답수) = 0.666
ap_good = calculate_average_precision_at_k(result_good, ground_truths, k=5)

# Case 2: 아쉬운 검색 엔진 (정답을 찾긴 했지만 4등, 5등으로 밀려남)
result_bad = ["item_X", "item_Y", "item_Z", "item_A", "item_B"]
# 연산:
# Rank 4 (item_A) -> Precision: 1/4 = 0.25
# Rank 5 (item_B) -> Precision: 2/5 = 0.40
# AP = (0.25 + 0.40) / 3(총정답수) = 0.216
ap_bad = calculate_average_precision_at_k(result_bad, ground_truths, k=5)

print(f"훌륭한 검색 엔진의 AP@5 : {ap_good:.3f}")
print(f"아쉬운 검색 엔진의 AP@5 : {ap_bad:.3f}")
# 결론: 두 엔진 모두 정답을 2개 찾았지만(단순 Recall은 동일), 
# AP 지표를 도입하면 정답을 상단에 노출시킨 훌륭한 엔진이 3배 더 높은 점수를 받습니다.
```

### @ K

"@ K" is a rank cut-off threshold in search system evaluation, meaning only the top K results will be considered.

In reality, users do not browse through hundreds of pages of search results; they typically check only the top 5-10 items and then leave.

Therefore, evaluating all data is meaningless. The number K is appended to focus evaluation on how well the system performs in the top-most section that customers actually see.

- **Recall@5**: How many correct answers were retrieved within the top 1-5 ranks presented by the engine, regardless of the total number of correct answers.
- **Precision@10**: How many of the results ranked 1-10 by the engine are true correct answers, not false ones.
- **AP@K**: Similarly, it calculates ranking quality only up to the K-th rank. Correct answers pushed beyond the K-th rank are mercilessly excluded from the score, assuming users will not click them.

<br>

## Application: LSH Parameter Tuning and Pareto Optimization

Now that we understand these metrics, we need to answer what can be done with them and how they can be utilized.

We previously learned that the following can be done:

- **Establish an offline benchmark pipeline**: Automatically evaluate the system using tens of thousands of test queries and ground truth data before applying it to live servers.
- **Rank-Aware Metrics**: Introduce mAP and NDCG instead of simple recall/precision to mathematically score whether the most relevant products are positioned at the top.
- **Pareto Frontier**: Adopt an engineering approach by plotting two graphs with QPS (search speed) on the x-axis and Recall@K (recall rate) on the y-axis, to find the maximum recall point achievable within the target response time, thereby determining parameters (e.g., number of LSH tables).

Let's delve deeper.

This topic is a methodology for quantitatively analyzing the trade-off relationship between search quality (Recall, mAP) and system costs (latency/memory), to determine the optimal system parameters (e.g., number of LSH hash tables) that meet service requirements.

- **Grid Search**: An automated technique for collecting performance metrics by individually testing all possible combinations of configurable parameters, for example, 5, 10, or 15 tables.
- **Trade-off**: A relationship where improving one metric inevitably worsens another. For example, increasing hash tables improves the hit rate but slows down search speed.
- **Pareto Frontier**: A curve representing a set of optimal balance points where, given resource constraints, one side (e.g., speed) cannot be improved further without sacrificing the other side (e.g., hit rate).
- **SLA (Service Level Agreement)**: The minimum performance standard that a service must guarantee to customers.

### Problem to Address

In LSH (Locality Sensitive Hashing), the `num_tables` parameter (number of hash tables) creates a critical trade-off.

- Increasing the number of tables means the query image searches through multiple tables, increasing the recall probability. However, it also increases the memory area to be queried and overlaps CPU operations, leading to longer response times.
- Decreasing the number of tables speeds up response time, but leads to more frequent False Negatives (missing correct answers), thus lowering the recall metric.

Engineers must numerically prove what the most cost-effective number of tables is, balancing a reasonably high hit rate with fast speed, and refer to these metrics.

### Solution Approach

Build an evaluation pipeline to simultaneously measure recall scores and average response times by varying `num_tables` from 1 to 50.

Based on the measured data, plot a 2D scatter plot with latency on the x-axis and recall on the y-axis.

Draw an x-axis line at the SLA (e.g., search response must not exceed 50ms) set by management or the planning team, and set the `num_tables` corresponding to the highest recall among the test results within 50ms as the final operational parameter.

1. **Define Parameter Space:** Set the array of `num_tables` to test (e.g., [5, 10, 15, 20, 25, 30]).
2. **Batch Indexing**: For each parameter setting, create a temporary Elasticsearch index and index 1 million dummy product data items.
3. **Load Testing**: Fire 10,000 queries, calculate recall@k as seen above, and simultaneously record Average Latency (ms).
4. **Derive Pareto Curve**: Map the collected Latency and Recall pairs onto a graph to observe the Pareto frontier in an upward-curving parabolic shape. The increase in recall gradually slows down as the number of tables increases - logarithmic function shape.
5. **Optimization under SLA Constraints**: Adopt parameters that achieve max(recall) under the condition that the inequality `Latency <= Target_SLA` is satisfied.

The following is Python-based parameter grid search simulation code written to automate the above process.

```py
def tune_lsh_parameters(test_queries, ground_truths, sla_max_latency_ms):
    """
    다양한 해시 테이블 개수를 테스트하여 SLA(최대 허용 지연시간)를 만족하면서
    가장 높은 Recall을 기록하는 최적의 num_tables를 판별합니다.
    """
    
    # 테스트할 후보군 (해시 테이블 개수)
    candidate_num_tables = [5, 10, 15, 20, 25, 30, 40, 50]
    
    best_tables = 0
    best_recall = 0.0
    best_latency = 0.0
    
    print(f"--- 튜닝 시작 (목표 SLA: {sla_max_latency_ms}ms 이하) ---")
    
    for tables in candidate_num_tables:
        # 1. 시스템 설정 변경 (가상 함수 호출)
        # deploy_temp_index(num_tables=tables)
        
        # 2. 부하 테스트 실행 (가상 함수: 쿼리 실행 후 평균 Recall과 Latency 반환)
        # 실제로는 앞서 배운 calculate_recall_at_k 함수를 수만 번 호출하여 통계를 냅니다.
        avg_recall, avg_latency = simulate_load_test(tables)
        
        print(f"테스트 [테이블 {tables}개] -> 지연시간: {avg_latency}ms | Recall: {avg_recall:.3f}")
        
        # 3. 파레토 최적점 판별 로직
        # 현재 테이블 설정이 목표 SLA(지연 시간)를 초과하지 '않았을' 때만 고려
        if avg_latency <= sla_max_latency_ms:
            # 그 중에서 가장 높은 Recall을 기록한 것을 챔피언으로 갱신
            if avg_recall > best_recall:
                best_recall = avg_recall
                best_latency = avg_latency
                best_tables = tables
                
    print("-" * 40)
    if best_tables == 0:
        print("경고: 주어진 SLA를 만족하는 파라미터가 없습니다. 서버 스케일업이 필요합니다.")
    else:
        print(f"✅ 최종 판별 결과: 최적 해시 테이블 수는 '{best_tables}개' 입니다.")
        print(f"   (예상 성능 -> Recall: {best_recall:.3f}, 지연시간: {best_latency}ms)")
        
    return best_tables

# ---------------------------------------------------------
# 가상의 부하 테스트 결과 모의 함수 (이해를 돕기 위한 Mock)
# ---------------------------------------------------------
def simulate_load_test(num_tables):
    # 테이블이 늘어날수록 지연시간은 선형적으로 증가 (테이블 1개당 약 5ms 소모 가정)
    latency = num_tables * 5 
    # Recall은 로그 함수처럼 초반에 확 오르고 뒤로 갈수록 둔화됨 (임의의 수식 적용)
    import math
    recall = min(0.98, math.log10(num_tables + 1) * 0.5) 
    return recall, latency

# ==========================================
# 실행 시뮬레이션
# 기획팀 요구사항: "무슨 일이 있어도 100ms 안에는 검색 결과가 떠야 합니다."
# ==========================================
optimal_parameter = tune_lsh_parameters(test_queries=[], ground_truths=[], sla_max_latency_ms=100)
```
