# 에이전트의 Adaptive Retrival 

### 먼저 알아둘 용어

- **RAG(Retrieval-Augmented Generation):** 모델이 답을 만들기 전에 외부 저장소에서 관련 문서를 찾아 입력에 붙여주는 방식이고 시험을 볼 때 참고서를 펼쳐 보는 것과 비슷하다
- **Adaptive Retrieval(적응형 검색):** 매번 참고서를 펴지 않고, 필요할 때만 펴도록 판단하는 기법 전반을 맡는다.
- **Knowledge Boundary(지식 경계):** 모델이 학습으로 이미 알고 있는 것과 모르는 것 사이의 선으로 검색 판단의 목표는 이 선을 넘는 질문에서만 검색하는 것이다
- **Under Retrieval (검색 누락):** 검색이 필요했는데 안 한 경우로, 모델이 모르는 내용을 지어내는 할루시네이션으로 이어진다. 
- **과다 검색 (over-retrival)**은 필요 없는데 검색한 경우로 지연시간과 비용이 늘고 관련 없는 문서가 끼어들어 원래 답힐 답을 틀리게 만들기도 한다(retrival distraction)

<br>

## 도구 설명문 기반 판단

### 이 방식은 어떤 아이디어 인가

검색 도구의 설명문 기반 판단은 tool description에 **이 저장소에는 무엇이 들어있고 어떤 질문일때 써라** 를 적어두고 판단은 에이전트의 모델의 추론능력에 맡기는 방식이다.

새로온 직원에게 "이 서랍에는 계약서가 있고 계약 조건을 물으면 여기를 봐라" 라고 알려주는 것과 같다.

모델은 저장소 안을 볼 수 없으므로 설명문이 유일한 지도 역할을 한다.

Anthropic의 "Writing effective tools for agents" (2025) 에서 이 방식을 정리했고, Claude Code가 코드베이스 탐색에서 벡터 인덱스 대신 grep과 파일 탐색 도구를 에이전트가 필요할 때 직접 쓰게 한 것도 같은 계열이다.

### 장단점

- **장점:** 학습이나 별도 모델 필요 없이 오늘 바로 적용할 수 있으며, 설명문만 고치면 되므로 반복 실험이 빠르고 대화 맥락(이미 붙여 넣은 문서가 있는지 등)까지 함께 고려한 판단이 가능하다
- **단점:** 판단 품질이 모델 성능과 설명문 문장에 크게 좌우된다는 점이고 작은 모델은 설명문을 잘 따르지 못하고, 모델을 교체하면 같은 설명문에서도 행동이 달라질 수 있다. 판단 근거가 모델 내부에 있어서 왜 호출했는지 추적하기도 어렵다.

### 어떤 상황에 적합할까?

Claude, GPT 같은 강한 모델을 api로 쓰는 경우 도구 수가 몇 개 수준인 경우, 프로젝트 초기라 판단 데이터가 아직 없는 경우에 적합하다 거의 모든 시스템의 출발점으로 삼기 좋다

```py
search_tool = {
    "name": "search_internal_docs",
    "description": (
        "사내 기술 문서 검색. 포함 범위: 2023년 이후 사내 API 명세, "
        "배포 런북, 장애 회고, 팀별 코딩 컨벤션.\n"
        "호출해야 하는 경우: 사내 서비스명/약어가 등장할 때, "
        "'우리 팀' 같은 조직 고유 맥락을 물을 때, 정확한 설정값이나 절차를 인용해야 할 때.\n"
        "호출하지 않아도 되는 경우: 일반 프로그래밍 개념, 공개 라이브러리 사용법, "
        "대화에 이미 붙여 넣은 문서에 대한 질문, 요약/번역/글 다듬기.\n"
        "결과가 비면 검색어를 바꿔 한 번 더 시도하고, 그래도 없으면 사용자에게 알립니다."
    ),
    "input_schema": {
        "type": "object",
        "properties": {"query": {"type": "string", "description": "핵심 키워드 3~6개"}},
        "required": ["query"],
    },
}
```

과다 검색을 줄이는데 특히 효과적인 방법은, 설명문에 **호출하지 않아도 되는 경우**를 함께 적는 것이다.

검색 겨로가를 돌려줄때도 빈 결과를 **관련 문서 없음**처럼 명확히 알리고 관련도 점수를 함께 주면, 에이전트가 다음 행동을 더 잘 고른다.

<br>

## 사전 분류 라우터 Adaptive-RAG

질문이 들어오면 메인 모델보다 먼저 작은 분류기가 "이 질문은 어떤 전략으로 풀어야하는가"를 정하는 방식이다. (요즘 JEV가 뜨는데 여기에 강함)

병원 접수처에서 증상을 듣고 진료과를 배정하는 것과 비슷하다. Adaptive-RAG(Jeong et al., NAACL 2024)는 질문을 검색 없음 / 한 번 검색 / 여러 번 반복 검색(멀티홉) 세 등급으로 나눴다

핵심은 라벨을 만드는 방법이다. 사람이 판단하는 대신 같은 질문을 세 전략으로 모두 풀게 해보고 **정답을 맞힌 전략중 가장 비용이 싼 것**을 그 질문의 라벨로 쓴다.

### 장단점

장점은 판단이 빠르고 싸며, 결과가 명시적인 라벨로 남아 로깅과 디버깅이 쉽다는 점이다. 메인 모델을 바꿔도 라우터는 그대로 쓸 수 있고, 라벨은 자동 생성 하므로 데이터 쌓는 비용이 낮다.

단점은 대화 도중 새로 생긴 정보 필요를 잡지 못한다는 점이다. 질문 시점에 한 번 판단하고 끝내기 때문이다. 학습 데이터 분포와 다른 질문이 오면 분류가 흔들리고, 파이프라인 단계가 하나 늘어난다

### 어떤 상황에 적합할까?

단발성 질의응답 서비스(사내 Q&A 봇, 고객지원) 처럼 질문 하나에 답 하나가 오가는 구조, 트래픽이 많아 검색 비용 절감 효과가 큰 경우, 질문 유형이 어느정도 반복되는 도메인에 적합하다.

#### 간단하게 구축하는 방법

라벨 자동 생성 -> 임베딩 -> 가벼운 분류기 순서다

```py
from sklearn.linear_model import LogisticRegression

STRATEGIES = ["no_retrieval", "single", "multi_hop"]  # 비용 오름차순

def auto_label(question, gold_answer):
    for strategy in STRATEGIES:                      # 싼 전략부터 시도
        if is_correct(run(strategy, question), gold_answer):
            return strategy
    return "multi_hop"                               # 다 틀리면 가장 강한 전략

X = [embed(q) for q, _ in train_set]
y = [auto_label(q, a) for q, a in train_set]
router = LogisticRegression(max_iter=1000).fit(X, y)

def answer(question):
    strategy = router.predict([embed(question)])[0]
    return run(strategy, question)
```

`is_correct`는 정답과 완전 일치, F1 점수 또는 LLM 채점기 (LLM as judge)로 구현한다.

처음에는 수백개 질문으로도 동작하는 라우터를 만들 수 있다.

<br>

## 자기 지식 기반 판단 (SRK, 인기도 규칙)

모델이 과거에 "무엇을 혼자 맞혔고 무엇을 검색해야만 맞혔는지"를 기록해두고 새 질문이 어느족과 닮았는지로 판단하는 방식이다

학생이 오답노트를 보고 이 유형은 내가 자주 틀렸으니 교과서를 확인하자고 정하는 것과 같다.

**SKR(Self-Knowledge guided Retrieval, Wang et al..,2023)**이 이 구조다

더 단순한 버전으로, Mallen et al.(2023)은 PopQA 데이터셋 실험에서 모델이 유명한 대상은 잘 알지만 덜 알려진 대상 (long-tail)에서 급격히 틀린다는 것을 보였다, 그래서 질문 속 대상의 인지도 (예: 위키피디아 조회수)가 낮을때만 검색하는 규칙만으로도 정확도와 비용을 함께 개선했다.

### 장단점

- **장점**: 모델의 실제 약점에 맞춘 판단이라는 점이다 라우터가 질문의 복잡도를 본다면 이 방식은 이 모델이 주제를 아는가를 직접 본다 kNN 방식은 학습 없이 예시를 추가하는 것만으로 판단이 갱신된다.
- **단점**: 모델이 바뀌면 기록 전체를 다시 만들어야한다, 왜냐면 새 모델은 예전 모델이 몰랐던 것을 알수 있기 때문에 인기도 규칙은 질문에서 대상(엔티티)를 뽑아야하므로 대상이 명확하지 않은 질문에는 쓰기 어렵다.

### 적합한 상황

사실 확인형 질문이 많은 도메인(인물, 제품, 지명 등), 모델을 한동안 고정해서 쓰는 환경, 검색 누락으로 인한 하룰시네이션이 특히나 문제인 서비스에 적합하다.

#### 간단하게 어떻게 구축하는지

```py
import numpy as np

memory = []  # (임베딩, 검색이 필요했는지)

def build_memory(train_set):
    for q, gold in train_set:
        closed = is_correct(llm(q), gold)                 # 검색 없이
        opened = is_correct(llm(q, docs=search(q)), gold)  # 검색 붙여서
        if closed or opened:
            memory.append((embed(q), not closed and opened))

def needs_retrieval(question, k=10):
    qv = embed(question)
    sims = [(np.dot(qv, v), label) for v, label in memory]
    top = sorted(sims, key=lambda s: -s[0])[:k]
    return sum(label for _, label in top) > k / 2          # 이웃 다수결
```

둘 다 틀린 질문은 근거가 되지 않으므로 기록에서 뺀다, 운영중 로그를 계속 memory에 넣으면 판단이 점점 서비스에 맞춰진다. 

<br>

## 생성 중 불확실성 감지 (FLARE, DRAGIN)

질문 시점에 한 번 판단하지 말고, 답을 쓰는 도중에 모델이 자신 없어 하는 순간을 포착해서 그때 검색하는 방식이다. 발표하다 말이 막히는 부분에서 메모를 들춰보는 것과 비슷하다

**FLARE(Jiang et al., 2023)**는 다음 문장을 미리 한 번 생성해보고, 그 문장이 logprob이 기준보다 낮은 토큰에 있으면 그 문장을 검색어로 문서를 찾은 뒤 문장을 다시 생성한다. DRAGIN(Su et al., ACL 2024)은 여기에서 토큰의 엔트로피(확률 분포가 얼마나 퍼져있는지), 그 토큰이 이후 생성에 미치는 어텐션 영향력, 그 토큰이 의미가 있는 단어인지를 함께 본다.

### 장단점

장점은 긴 답변 중간에 새로 생기는 정보 필요를 잡을 수 있다는 점이다. 보고서나 설명문처럼 여러 사실이 이어지는 생성에서 문장 단위로 필요한 만큼 검색한다. 별도 학습 없이 모델 자체의 확신도를 신호로 쓴다

단점은 logprob, attention 값에 접근해야한다는 점이라 자체 호스팅 오픈모델 vLLM등에서는 쉽지만 api model에서는 적용이 제한된다 또한 모델이 틀린 내용을 확신하며 말하는 경우(과신)는 잡지 못하고 문장마다 미리 생성해보므로 지연시간이 는다.

### 적합한 상황

오픈 모델을 직접 서빙하는 환경, 긴 형식의 생성(리포트나 문서 생성, 긴설명)에서 사실 정확도가 매우 중요한 경우 에이전트 루프 없이 단일 생성 파이프라인으로 운영하는 경우 적합하다.

#### 간단하게 구축 

```py
THRESHOLD = -2.5  # logprob 기준값, 평가 세트로 조정

def flare_generate(question, max_sentences=10):
    answer = ""
    for _ in range(max_sentences):
        draft, logprobs = llm_next_sentence(question, answer)  # logprob 포함 생성
        if min(logprobs) < THRESHOLD:                           # 자신 없는 토큰 존재
            docs = search(question + " " + draft)
            draft, _ = llm_next_sentence(question, answer, docs=docs)
        answer += draft
        if draft.strip().endswith(END_MARK):
            break
    return answer
```

vLLM이나 OpenAI 호환 서버에서 logprobs=True 옵션을 켜면 토큰별 값을 받을 수 있는데 기준값은 모델마다 다르므로 평가 세트에서 검색 누락과 과다 검색의 균형이 맞는 지점을 찾아야한다.

<br>

## 학습으로 판단 자체를 익히기 Self-RAG, Toolformer, Search-R1

검색 판단을 모델 가중치 안에 직접 새기는 방식으로 운전 규칙을 외우게하는 대신 실제 도로에서 연습시켜 감각을 몸에 익히게하는 것과 비슷하다. 세가지 대표 사례가 있다.

Self-RAG(Asai et al, ICLR 2024)는 모델이 생성중에 `[Retrieve]` 같은 특수 토큰 (반성 토큰, reflection token)을 스스로 출력하도록 학습시킨다. 검색 필요 여부, 가져온 문서의 관령성, 답이 문서로 뒷받침되는지 각각 토큰으로 표시한다. 학습 라벨은 gpt-4를 비평가 critic로 써서 만든뒤 증류했다.

Toolformer(Schick et al., 2023)는 텍스트 곳곳이 api 호출을 넣어보고 호출 결과를 넣었을 때 뒤따르는 텍스트를 예측하는 손실 loss이 줄어든 위치에만 남겨 파인튜닝한다 이 도구 호출이 실제로 도움이 됐다. 는 신호를 사람 라벨 없이 얻는 자기지도 방식이다.

Search-R1(Jin et al., 2025)과 R1-Searcher, ReSearch 같은 최근 연구는 강화학습(RL)을 쓴다, 모델이 추론중 `<search>` tag로 검색을 호출할 수 있게 두고, 최종 답이 맞았는지만 보상으로 준다. 그러면 모델이 어떤 상황에서 검색해야 결국 맞히는지 시행착오로 익힌다.

### 장단점

- **장점**은 판단이 가장 자연스럽고 정교해줄수 있따는 점, 멀티홉 질문에서 첫 검색 결과를 보고 다음 검색어를 정하는 연쇄판단까지 학습된다 설명문이나 외부 분류기 없이 모델 하나로 동작한다
- **단점**은 비용과 난이도가 가장 높다는 점으로 오픈 모델, gpu, 학습 파이프라인 수천개이상의 학습데이터가 필요하다 RL은 보상설계가 어긋나면검색을 남발하거나 아예 안하는 쪽으로 수렴하기 쉽고 보상해킹, 학습된 판단은 내부 가중치에 있어서 수정이 어렵다.

### 적합한 상황

오픈 모델을 직접 파인튜닝 할 수 있는 환경, 앞의 1~4번 방식으로 운영하며 판단 로그가 충분히 쌓인 경우, 검색이 핵심기능인 전용 에이전트 (리서치 에이전트나 도메인 특화 검색 봇)을 만드는 경우에 적합하다.

#### 간단 구축

Search-R1 방식의 핵심은 로아웃 형식과 보상함수다.

```py
SYSTEM = (
    "생각은 <think></think>, 검색은 <search>검색어</search>로 합니다. "
    "검색 결과는 <result></result>로 주어집니다. 최종 답은 <answer></answer>로 냅니다."
)

def reward(trajectory, gold_answer):
    answer = extract_tag(trajectory, "answer")
    if answer is None:
        return 0.0                                    # 형식 위반
    score = 1.0 if is_correct(answer, gold_answer) else 0.0
    n_search = trajectory.count("<search>")
    return score - 0.05 * max(0, n_search - 1)        # 과다 검색에 작은 패널티
```

rollout중 모델이 `</serach>`를 출력하면 생성을 멈추고 실제 검색 결과를 `<result>`로 붙인뒤 생성시킨다.

학습에는 GRPO(여러 답안을 뽑아 서로 비교해 상대적 보상을 주는 RL 알고리즘)를 주로 쓰며, verl같은 오픈소스 RL 프레임워크에 Search-R1 예제가 공개되어있다. 검색 결과 토큰은 모델이 생성한 것이 아니므로 손실 계산에서 제외해야한다.

> Rollout: 에이전트가 현재 정책(Policy)을 따라 환경과 상호작용하며 상태(State), 행동(Action), 보상(Reward) 등의 데이터 시퀀스(궤적, Trajectory)를 생성하는 과정 및 그 결과 데이터를 뜻한다.

<br>

## 어떤 방식이든 공통으로 필요한 것

### 판단을 측정하는 평가 세트

모든 방식은 "검색이 필요했던 질문인가"라는 정답이 있어야 개선 여부를 잴 수 있었다.

3번에서 쓴 것과 같은 방법 (검색 없이 풀기 vs 검색 붙여 풀기)으로 라벨을 자동 생성하고, 에이전트의 실제 호출과 비교한다.

```py
from collections import Counter

def evaluate(agent, labeled_set):
    c = Counter()
    for q, needed in labeled_set:
        called = agent.run(q).used_tool("search_internal_docs")
        c[(needed, called)] += 1
    return {
        "검색 누락": c[(True, False)],   # 필요했는데 안 부름 → 할루시네이션 위험
        "과다 검색": c[(False, True)],   # 불필요한데 부름 → 비용, 혼란
        "정확도": (c[(True, True)] + c[(False, False)]) / sum(c.values()),
    }
```

검색 누락이 많으면 설명문에 "호출 해야하는 경우"를 넓히고 과다 검새깅 많으면 "호출하지 않아도 되는 경우"를 구체화한다. 모델을 교체할때마다 이 평가를 다시 돌려야한다.

#### 검색 비용 줄이기

검색이 빠르고 결과가 짧을수록 판단 실수의 대가가 작아진다. 애매할 때 한 번 가볍게 찔러보는 전략 (probe search)이 합리적인 선택이 되기 때문이다. 판단 로직을 정교하게 만드는 것 만큼, 검색 자체를 가볍게 만드는 것도 효과적인 개선 방향이다.

