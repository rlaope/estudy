# 검색 시스템 정량적 평가 지표 (IR Metrics & Evaluation)

검색 시스템의 정량적 평가 지표는 사용자의 쿼리 (이미지/텍스트)에 대해 시스템이 반환한 결과물이 얼마나 정확한가? (정밀도), 정답을 빼먹지 않았는가? (재현율) 그리고 가장 좋은 정답을 상단에 노출시켰는가? (순위)를 수학적으로 측정하는 프레임워크다.

검색엔진의 파라미터 튜닝을 하거나 새로운 ai 모델을 배포하기 이전에 a/b 테스트의 기준이 되는 핵심 나침반 역할을 한다.

오늘은 아래의 핵심 평가 지표 (용어)들을 알아볼 것이다.

- **IR(Information Retrieval)**: 사용자가 원하는 정보를 방대한 데이터 속에서 찾아내는 기술 분야
- **Recall (재현율)**: 시스템이 전체 정답중에 몇 개를 빼먹지 않고 찾아냈는가를 나타내는 비율
- **Precision (정밀도)**: 시스템이 가져온 결과물 중 진짜 정답이 몇 개인가를 나타내는 순도 비율
- **mAP (Mean Average Precision)**: 정답을 얼마나 많이 찾았는지 뿐만 아니라 가장 좋은 정답을 얼마나 상단 1~3등에 노출시켰는가 까지 평가하는 지표
- **NDCG (Normalized Discounted Cumulative Gain)**: 완벽하게 정답이 정렬된 이상적인 순위와 비교하여, 현재 시스템의 랭킹 품질을 0 ~ 1 사이로 측정한 지표

### 명칭의 유래

일단 시스템 정량적 평가 지표에 관한 단어들을 살펴보며 명칭의 유래를 알아보겠다.

**IR(Information Retrieval - 정보 검색)**은 방대한 비정형 데이터(문서, 이미지) 속에서 사용자의 의도 information need에 맞는 데이터를 찾아내는 Retrieve 컴퓨터 과학 분야를 뜻한다.

**mAP**은 매번 정답이 발견될 때마다 계산한 정밀도들의 평균을 구하고 이를 모든 쿼리에 대해 다시 평균을 내었다는 수학적 연산 과정을 그대로 이름에 담은것이다 Mean Average Precision

NDCG 정답이 뒤로 밀릴수록 가치 Gain를 로그 함수에서 깎아내려(Discount), 완벽한 순위의 점수와 비교해 정규화(Normalize) 했다는 게산 식 자체를 약어로 만든것이다.

## 해결하고자 한 문제

**ANN(근사 최근접 이웃)**의 딜레마가 있었는데 LSH나 FAISS 같은 벡터 검색 엔진은 속도를 위해 정확도를 일부러 희생한다. 해시 테이블을 늘리면 정답을 찾을 확률은 올라가지만, 메모리 사용량과 검색 지연 시간이 폭발적으로 증가한다.

**휴리스틱(감)에 의존한 튜닝 한계**: 해시 비트를 16으로 할지 32로 할지 엔지니어의 감으로 결정하면 트래픽이 몰렸을 대 서버가 oom이 나 뻗거나 고객이 원하는 상품이 검색되지 않는 장애가 발생한다.

**순서 무시의 오류**: 10개의 검색 결과중에 3개가 정답이다 라는 사실만으로는 부족하다. 그 3개의 정답이 1 2 3등에 있는 시스템과 8 9 10등에 있는 시스템은 고객 경험(클릭률)에서 천지 차이임에도 기존의 단순 hit 측정 방식은 이를 구분하지 못했다.

### 문제 해결 방식

- **오프라인 벤치마크 파이프라인 구축**: 라이브 서버에 적용하기 이전에, 수만개의 테스트 쿼리와 정답지 Ground Truth를 이용해 시스템을 자동 평가한다
- **순위 민감형 지표(Rank-Aware Metrics)**: 단순 recall/precision 대신에 mAP, NDCG를 도입해 가장 관련성이 높은 상품이 최상단에 위치하는가를 수학적으로 채점한다
- **파레토 프론티어 (Pareto Frontier)**: x축을 검색속도 QPS, Y축을 재현율 Recall@K로 두 그래프를 그려 목표 응답시간 내에서 달성할 수 있는 최대 recall 지점을 찾아 파라미터(lsh 테이블 수)를 결정하는 공학적 접근을 취한다. 

### Recall & Precision

직관적인 예시로 이해해보자. 옷가게 알바생에 비유해보겠다.

매우 거대한 창고에 빨간색 원피스가 10벌 있다고 가정해볼때 두 명의 알바생(검색엔진 A, B)에게 빨간 원핀스를 찾아오라고 심부름(Query)를 시킨다.

여기서 **Recall(재현율)**이 정답을 얼마나 안빼먹었냐, 즉 알바생 A가 창고에서 옷을 한 무더기 가져왔는데 뒤져보니 그 빨간 원피스가 8개가 있으면 정답 10개중에 8개를 찾아온것 recall은 80%

여기서 **가져온 것 중 쓰레기(오답)이 얼마나 있는가 Precision 정밀도**는 알바생이 가져온 옷 무더기를 세어보니 20벌이고 8벌이 정답이였지만 12벌은 바지, 노란셔츠 이런것들 이였다면 8/20으로 20개 가져온것중 쓸모있는건 8개라 절반이상이 쓰레기다. 40% 인것.

얼추 비슷할 수 있지만 recall의 분모는 정답의 총 개수인거고, precision의 분모는 정답의 개수가 아닌. 가져온 시도의 개수라고 볼 수 있다.

### AP, mAP

그 다음 **AP(Average Precision)**는 순서(랭킹)의 퀄리티가 어떤가다. 알바생 A, B가 똑같이 20벌을 가져왔고 똑같이 8벌의 정답이 섞여있었다. 그런데 알바생 A는 손에 옷을 건네줄때 1번째부터 8번째까지 빨간 원피스만 먼저 연속으로 건네주고 그 뒤에 쓸데없는 옷을 주었다. 이는 상위 랭크 장악.

반면 알바생 B는 쓸모없는 파란바지만 주다가 13번째부터 빨간 원피스를 줬다면 하위 랭크로 밀린것이다.

홈쇼핑 고객 입장에서는 1페이지 상단에 정답을 보여준 알바생 A가 압도적으로 우수한 검색엔진이고 똑같이 8개를 찾았더라도 랭킹에 따라 가중치를 다르게 주어 알바생 A에게 훨씬 높은 점수를 주는 수학적 지표가 바로 AP Average Precision이다.

여기서 AP가 한 검색이 가져온 결과에 대한 수치이고, mAP가 시스템의 성능을 최종평가 하기위해 수백 수천개의 다양한 질문들에 대한 ap 점수들을 모두 더해 평균을 낸 값이다.

<br>

## NDCG

NDCG는 검색 결과의 순위를 평가하는 지표로 정답을 단순히 맞다 틀리다의 이분법으로 나누지 않고 얼마나 관련성이 높은가 3점, 2점, 1점등.. 라는 등급으로 매겨 랭킹의 품질을 0 ~ 1사이의 값으로 평가한다.

- **CG(Cumulative Gain - 누적 이득)**: 검색된 결과들의 관련성 점수를 순위에 상관없이 그냥 다 더한 값이다.
- **DCG(Discounted Cumulative Gain - 할인된 누적 이득):** 순위가 뒤로 밀릴수록 사용자에게 주는 가치가 떨어지므로, 점수를 로그 함수로 깎아내서 더한다.
- **NDCG(Normalized - 정규화된 ~ )**: 쿼리마다 만점 기준이 다르기 때문에, 현재 랭킹점수 DCG를 완벽하게 정렬됐을때의 이상적인 만점(IDCG)으로 나누어(Normalize) 0 ~ 1 사이의 절대 평가 점수로 만든다.

앞서 배운 AP/mAP는 맞다 1 혹은 틀리다 0 둘중 하나일때만 계산할 수 있다. 하지만 실제 홈쇼핑 검색에서 빨간 원피스를 검색했을 때 빨간 원피스는 3점 빨간 치마는 2점, 분홍 원피스는 1점처럼 부분 점수가 존재하고 AP는 이러한 미묘한 퀄리티 차이를 계산식에 반영할 수 없다.

### 해결 방식

관련성 등급 Relaevance Score를 도입해 결과물에  $rel_i$ 라는 가중치를 둔다.

**패널티를 또 부여하는데 (로그 할인)** 검색결과 하단에 예: 10페이지에 있는 3점짜리 정답은 첫 페이지에 있는 3점짜리 정답보다 가치가 낮다. 따라서 순위가 커질수록 분모가 커지는 $\log_2(rank + 1)$ 로 점수를 나눈다.

옷가게 알바생 예시를 또 들어보자면 가을용 트렌치 코트를 찾아오라고 알바생 A에게 시켰다. 완벽하게 일치하면 3점 색상만 일치하면 1점 쓰레기는 0점이라 햇을때

1. 첫번째 옷 쓰레기 0점
2. 두번째 옷 완벽한 트렌치코트 3점
3. 3번째 옷 비슷한 베이지색 자켓 1점
4. **CG(단순 합산)**: 0 + 3 + 1 = 4점으로 순서를 무시하니 쓰레기를 먼저 준 잘못이 가려진다.
5. **DCG(할인 합산)**: 3점짜리를 2번째에 줬고 1점짜리를 3번째에 줬으니 패널티를 준다 (예: 0 + 1. 8 + 0.5 = 2.3점)
6. **IDCG(만점 기준)**: 만약 알바생이 완벽하게 3 1 0 순으로 건네주었다면 이상적인 점수를 구한다 (3 + 0.6 + 0 = 3.6점) 패널티도 있는거임
7. **NDCG:** 알바생 A의 최종점수는 내 점수를 만점으로 나눈 즉 2.3 / 3.6 같은 (위 예시 기준) = 0.63점으로 결론이 지어진다.

수학적으로 DCG와 IDCG는 다음과 같이 정의된다. p는 평가할 상위 K개의 랭킹을 의미한다.

$$DCG_p = \sum_{i=1}^{p} \frac{rel_i}{\log_2(i+1)}$$

- $rel_i$: $i$번째 순위에 있는 문서의 관련성 점수 (예: 0, 1, 2, 3)
- $\log_2(i+1)$: 순위가 뒤로 갈 수록 분모를 키워 점수를 깎아내는 역할

IDCG(Ideal DCG)는 현재 가져온 결과물들을 관련성 점수 $rel_i$가 높은 순서대로 내림차순 정렬한 뒤, 똑같은 DCG 공식을 적용하여 구한 가장 이상적인 랭킹 DCG의 값이다.

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

## 검색 엔진 평가 파이프라인 동작 원리

실제 검색 엔진 평가 파이프라인은 4단계 아키텍처로 구성된다

#### 1. Ground Truth 정답지 수집층

- **Explicit:** 사람이 직접 A상품과 B상품은 유사하다고 라벨링한다.
- **Implict:** 사용자의 행동 로그를 분석 (A상품을 클릭한 유저가 B상품도 클릭하고 구매했다면 두 벡터는 유사한 것으로 간주)

#### 2. Parameter Grid Search 층

평가 스크립트가 Elasticsearch의 LSH 파라미터(num_tables, hash_length)나 HNSW 파라미터 m, ef_construction을 바꿔가며 수십 개의 임시 인덱스를 자동 생성한다.

#### 3. 평가 엔진층 Scoring

쿼리를 던져 K개의 결과를 받은 후, 순위를 고려한 지표 Average Precision을 연산한다.

AP는 매 정답이 발견될 때마다 누적된 Precision의 평균을 구하는 방식이다.

#### 4. Trade-off 분석층

산출된 mAP 점수와 쿼리 처리 시간 Latency를 매핑하여 리포트를 생성하고, 최적의 시스템 파라미터를 확정한다.

### Example

단순한 교집합 계산을 넘어 검색 품질을 평가할 때 가장 널리 쓰이는 순위 민감형 지표인 Average Precision AP@K를 계산하는 로직이다.

이 코드는 정답이 상단에 랭크될 수록 더 높은 점수를 부여한다.

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

at K는 검색 시스템 평가에서 상위 K개 까지만 결과를 보겠다 라는 순위 절단 cut-off 기준선이다.

현실의 사용자는 검색 결과의 수백 페이지를 전부 뒤져보지 않고 보통 상위 5~10개만 확인하고 이탈한다.

따라서 전체 데이터를 다 평가하는 것은 무의미하며 고객이 실제로 눈으로 확인하는 상위 최상단 구간에서 얼마나 잘했는가를 집중적으료 평가하기 위해 숫자 K를 붙이는 것이다.

- **Recall@5**: 전체 정답이 몇개든 상관없이 엔진이 내놓은 상위 1~5등 안에 정답을 몇개 건졌는가
- **Precision@10**: 엔진이 내놓은 1~10등 결과물중에 가짜가 아닌 진짜 정답은 몇개인가
- **AP@K**: 마찬가지로 상위 K등까지만 랭킹의 품질을 계산하며, K등 밖으로 밀려난 정답은 사용자가 클릭핮 않을 것이라 간주하여 가차없이 점수에 반영하지 않는다.

<br>

## 응용. LSH 파라미터 튜닝과 파레토 최적화

이제 이런 지표들은 알았는데, 이 지표로 어떤것을 할 수 있으며 어떻게 활용하는가에 대한 질문에 답을 해야한다.

아까 위에서 아래를 할수있다고 알아봤는데

- **오프라인 벤치마크 파이프라인 구축**: 라이브 서버에 적용하기 이전에, 수만개의 테스트 쿼리와 정답지 Ground Truth를 이용해 시스템을 자동 평가한다
- **순위 민감형 지표(Rank-Aware Metrics)**: 단순 recall/precision 대신에 mAP, NDCG를 도입해 가장 관련성이 높은 상품이 최상단에 위치하는가를 수학적으로 채점한다
- **파레토 프론티어 (Pareto Frontier)**: x축을 검색속도 QPS, Y축을 재현율 Recall@K로 두 그래프를 그려 목표 응답시간 내에서 달성할 수 있는 최대 recall 지점을 찾아 파라미터(lsh 테이블 수)를 결정하는 공학적 접근을 취한다. 

더 자세히 알아보겠다.

이 주제는 검색 품질 Recall, mAP와 시스템 비용 latency/memory 사이의 상충 관게를 정량적으로 분석하여, 서비스 요구사항에 맞는 최적의 시스템 파라미터(예: LSH 해시 테이블 개수)를 결정하는 방법론이다.

- **Grid Search(그리드 탐색)**: 설정 가능한 모든 파라미터의 조합 예를들어 테이블 5개, 10개, 15개를 일일이 테스트하여 각각의 성능 지표를 수집하는 자동화 기법
- **Trade-off(상충 관계)**: 하나의 지표가 개선되면 필연적으로 다른 지표가 악화되는 관계. 예를들어 해시 테이블을 늘리면 정답률은 오르지만 검색 속도는 느려진다.
- **Pareto Frontier(파레토 프론티어)**: 주어진 자원 환경에서 어느 한쪽(예: 속도)의 손실 없이는 다른 한쪽(예: 정답률)을 더이상 개선할 수 없는 최적의 균형점들이 모인 곡선
- **SLA(Service Level Agreement)**: 서비스가 고객에게 보장해야하는 최소한의 성능 기준

### 해결하고자 하는 문제

LSH(Locality Sensitive Hashing)에서 `num_tables` 해시 테이블수 파라미터는 치명적인 trade-off를 발생시킨다.

- 테이블을 늘릴 경우 쿼리 이미지가 여러 테이블을 뒤지게 되므로 찾을 확률 recall 이 높아지지만, 동시에 조회해야할 메모리 영역이 늘어나고 cpu 연산이 겹치면서 응답시간이 길어진다.
- 테이블을 줄일 경우 응답속도는 빨라지지만, 정답을 놓치는 경우 False Negative가 잦아져 recall 지표가 낮아진다.

엔지니어는 이 정답률도 적당히 높고 속도도 빠른 가장 가성비 좋은 테이블 개수는 몇개인지인가를 수치로 증명해야하고 이 지표들을 참고한다.

### 문제 해결 방식

평가 파이프라인을 구축해 num_tables를 1개부터 50개까지 바꿔가며 recall 점수와 평균 응답 속도를 동시에 측정한다.

측정한 데이터를 바탕으로 x축은 지연시간, y축은 recall로 둔 2차원 산점도 scatter plot을 그린다

경영진이나 기획팀이 설정한 SLA(검색 응답은 최대 50ms를 넘기지 말것) 기준선에 x축을 긋고 50ms 안쪾에 위치한 테스트 결과중 recall이 가장 높은 점에 해당하는 num_tables를 최종 운영 파라미터로 설정한다.

1. **Parameter Space 정의:** 테스트할 num_tables 배열을 설정한다 (ex [5, 10, 15, 20, 25, 30])
2. **Batch Indexing**: 각 파라미터 설정값마다 임시 Elasticsearch 인덱스를 생성하고 100만건의 더미 상품 데이터를 색인
3. **Load Testing**: 1만개의 쿼리를 쏘면서 위에서 본 recall@k를 계산하고 동시에 Average Latency(ms)를 기록한다.
4. **Pareto 곡선 도출**: 수집된 Latency, Recall 쌍을 그래프에 매핑하여 우상향하는 포물선 형태로 파레토 프론티어를 확인한다. 테이블 수가 늘어날수록 recall 상승폭은 점차 둔화된다 - 로그 함수 형태
5. **SLA 제약 하의 최적화**: `Latency <= Targer_SLA` 부등식을 만족하는 조건하에 max(recall)을 달성하는 파라미터를 채택한다.

위 과정을 자동화하기 위해 작성하는 파이썬 기ㅏ반에 paramter grid search 시뮬레이션 코드다.

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
