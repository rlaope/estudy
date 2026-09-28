# Adaptive Retrieval for Agents

### Terms to Know First

- **RAG (Retrieval-Augmented Generation):** A method where the model retrieves relevant documents from an external repository and appends them to the input before generating an answer, similar to consulting a reference book during an exam.
- **Adaptive Retrieval:** Refers to the overall techniques for deciding when to retrieve information only when necessary, rather than consulting a reference every time.
- **Knowledge Boundary:** The line between what the model already knows from training and what it doesn't; the goal of retrieval judgment is to search only for questions that cross this boundary.
- **Under Retrieval:** Occurs when retrieval was needed but not performed, leading to hallucinations where the model invents unknown information.
- **Over-retrieval:** Occurs when retrieval is performed unnecessarily, increasing latency and cost, and irrelevant documents can interfere, causing the model to provide incorrect answers (retrieval distraction).

<br>

## Tool Description-Based Judgment

### What is the idea behind this method?

Tool description-based judgment involves writing **what is in this repository and for what types of questions it should be used** in the tool description, and entrusting the judgment to the agent's model's reasoning ability.

It's like telling a new employee, "This drawer contains contracts; if you have questions about contract terms, look here."

Since the model cannot see inside the repository, the description serves as the sole guide.

Anthropic's "Writing effective tools for agents" (2025) formalized this approach, and Claude Code's use of grep and file exploration tools directly by the agent when needed for codebase exploration, instead of vector indexes, falls into the same category.

### Pros and Cons

- **Pros:** Can be applied immediately without training or a separate model, allows for rapid iterative experiments by simply modifying the description, and enables judgment that considers the conversation context (e.g., whether documents have already been provided).
- **Cons:** The quality of judgment heavily depends on model performance and the wording of the description; smaller models may not follow descriptions well, and changing the model can alter behavior even with the same description. It's also difficult to trace why a tool was called, as the reasoning is internal to the model.

### When is it suitable?

Suitable for cases where powerful models like Claude or GPT are used via API, when the number of tools is limited, or when judgment data is scarce in the early stages of a project. It's a good starting point for almost any system.

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

An especially effective way to reduce over-retrieval is to include **cases where retrieval is not needed** in the description.

When returning search results, clearly indicating empty results as **no relevant documents** and providing relevance scores helps the agent make better subsequent decisions.

<br>

## Pre-classification Router Adaptive-RAG

When a question comes in, a small classifier determines "what strategy should be used to answer this question" before the main model. (JEV is trending these days and is strong in this area).

Similar to a hospital reception desk listening to symptoms and assigning a department. Adaptive-RAG (Jeong et al., NAACL 2024) categorizes questions into three levels: no retrieval / single retrieval / multi-hop retrieval.

The key is how labels are created. Instead of human judgment, the same question is attempted with all three strategies, and **the cheapest strategy that yields the correct answer** is used as the label for that question.

### Pros and Cons

Advantages include fast and inexpensive judgment, explicit labels for easy logging and debugging. The router can be reused even if the main model changes, and data accumulation costs are low because labels are automatically generated.

Disadvantages include the inability to capture new information needs that arise during a conversation, as judgment is made only once at the time of the question. If questions outside the training data distribution appear, classification can become unstable, and it adds an extra step to the pipeline.

### When is it suitable?

Suitable for single-turn Q&A services (e.g., internal Q&A bots, customer support) where one question leads to one answer, scenarios with high traffic where search cost reduction is significant, and domains where question types are somewhat repetitive.

#### Simple Construction Method

The sequence is: automatic label generation -> embedding -> lightweight classifier.

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

`is_correct` is implemented using exact match with the gold answer, F1 score, or an LLM as a judge.

Initially, a router can be built that works with just a few hundred questions.

<br>

## Self-Knowledge Guided Judgment (SRK, Popularity Rules)

This method involves the model recording "what it answered correctly on its own and what it only answered correctly with retrieval," and then judging new questions based on their similarity to these past cases.

It's like a student reviewing their incorrect answers and deciding to check the textbook for types of questions they frequently get wrong.

**SKR (Self-Knowledge guided Retrieval, Wang et al., 2023)** uses this structure.

In a simpler version, Mallen et al. (2023) showed in experiments on the PopQA dataset that models perform well on famous entities but rapidly make mistakes on less known (long-tail) entities. Therefore, they improved both accuracy and cost by simply using a rule to retrieve only when the popularity of the entity in the question (e.g., Wikipedia page views) is low.

### Pros and Cons

- **Pros:** The advantage is that judgment is tailored to the model's actual weaknesses. While a router looks at question complexity, this method directly assesses whether the model knows the topic. The kNN approach updates judgment simply by adding examples without retraining.
- **Cons:** If the model changes, the entire record must be rebuilt, because a new model might know things the old one didn't. Popularity rules require extracting entities from questions, making them difficult to use for questions where the entity is not clear.

### Suitable Situations

Suitable for domains with many fact-checking questions (e.g., people, products, places), environments where the model is fixed for a period, and services where hallucinations due to under-retrieval are a particular problem.

#### How to Build Simply

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

Questions that are both incorrect are excluded from the record as they don't provide evidence. Continuously adding operational logs to memory allows the judgment to adapt to the service over time.

<br>

## Uncertainty Detection During Generation (FLARE, DRAGIN)

Instead of making a judgment once at the time of the question, this method involves detecting moments when the model is uncertain while generating an answer and performing a search then. It's similar to consulting notes when you get stuck during a presentation.

**FLARE (Jiang et al., 2023)** pre-generates the next sentence once, and if any token in that sentence has a logprob below a threshold, it uses that sentence as a query to find documents and then regenerates the sentence. DRAGIN (Su et al., ACL 2024) additionally considers the token's entropy (how spread out the probability distribution is), its attention influence on subsequent generation, and whether the token is a meaningful word.

### Pros and Cons

- **Pros:** The advantage is that it can capture new information needs that arise in the middle of a long answer. In generations where multiple facts are connected, such as reports or explanations, it retrieves as much as needed at the sentence level. It uses the model's own confidence as a signal without separate training.
- **Cons:** The disadvantage is that it requires access to logprob and attention values, which is easy with self-hosted open models like vLLM but limited with API models. Furthermore, it cannot catch cases where the model confidently states incorrect information (overconfidence), and pre-generating each sentence increases latency.

### Suitable Situations

Suitable for environments where open models are self-hosted, for long-form generation (reports, document creation, detailed explanations) where factual accuracy is critical, and when operating with a single generation pipeline without an agent loop.

#### Simple Construction

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

On vLLM or OpenAI-compatible servers, you can get token-level values by enabling the `logprobs=True` option. Since the threshold varies by model, you need to find a point in the evaluation set where the balance between under-retrieval and over-retrieval is appropriate.

<br>

## Learning Judgment Itself (Self-RAG, Toolformer, Search-R1)

This method involves embedding retrieval judgment directly into the model's weights, similar to letting someone practice on real roads to develop a sense for driving, rather than just memorizing rules. There are three representative examples.

Self-RAG (Asai et al., ICLR 2024) trains the model to output special tokens (reflection tokens) like `[Retrieve]` during generation. It uses tokens to indicate whether retrieval is needed, the relevance of retrieved documents, and whether the answer is supported by documents. Training labels were created using GPT-4 as a critic and then distilled.

Toolformer (Schick et al., 2023) fine-tunes by inserting API calls throughout the text and keeping them only where the loss for predicting subsequent text is reduced when the call results are included. This is a self-supervised method that obtains signals that tool calls were actually helpful without human labels.

Recent research like Search-R1 (Jin et al., 2025), R1-Searcher, and ReSearch uses reinforcement learning (RL). The model is allowed to call retrieval with a `<search>` tag during inference, and only the correctness of the final answer is given as a reward. This allows the model to learn through trial and error in which situations retrieval is necessary to ultimately get the correct answer.

### Pros and Cons

- **Pros:** The advantage is that judgment can be the most natural and sophisticated. It learns even chained judgments, such as determining the next search query based on the results of the first search in multi-hop questions. It operates with a single model without descriptions or external classifiers.
- **Cons:** The disadvantage is the highest cost and difficulty, requiring open models, GPUs, training pipelines, and thousands of training data points. In RL, if the reward design is flawed, it can easily converge to over-retrieval or no retrieval at all (reward hacking), and learned judgments are difficult to modify as they are embedded in internal weights.

### Suitable Situations

Suitable for environments where open models can be fine-tuned directly, when sufficient judgment logs have accumulated from operating with methods 1-4, and when building dedicated agents where retrieval is a core function (e.g., research agents or domain-specific search bots).

#### Simple Construction

The core of the Search-R1 method lies in the rollout format and the reward function.

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

During rollout, if the model outputs `</search>`, generation stops, actual search results are appended as `<result>`, and then generation resumes.

GRPO (an RL algorithm that compares multiple answers and gives relative rewards) is mainly used for training, and Search-R1 examples are available in open-source RL frameworks like verl. Search result tokens are not generated by the model, so they should be excluded from loss calculation.

> Rollout: Refers to the process and resulting data where an agent interacts with an environment following its current policy, generating a sequence of data such as states, actions, and rewards (trajectory).

<br>

## What is Needed in Common for Any Method

### Evaluation Set to Measure Judgment

All methods required a ground truth for "was retrieval needed for this question?" to measure improvement.

Labels are automatically generated using the same method as in point 3 (solving without retrieval vs. solving with retrieval) and compared against the agent's actual calls.

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

If there are many under-retrievals, broaden "cases where retrieval is needed" in the description. If there are many over-retrievals, specify "cases where retrieval is not needed." This evaluation should be rerun every time the model is changed.

#### Reducing Search Costs

The faster the search and the shorter the results, the smaller the cost of a judgment error. This is because a strategy of lightly probing when uncertain (probe search) becomes a reasonable choice. Making the search itself lightweight is as effective a direction for improvement as refining the judgment logic.
