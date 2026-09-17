# LLM/Agent 정량적 Evaluation 시스템

"새로운 프롬프트를 적용하거나 검색 인덱스를 교체했을 때 Agent의 답변 품질이 좋아졌는지 나빠졌는지를 어떻게 증명할 수 있을까?"

기존 소프트웨어처럼 `assert expected == actual` 형태의 단순한 단위 테스트로는 매번 단어가 조금씩 바뀌는 LLM의 non deterministic인 텍스트 출력을 검증할 수 없다.

사람이 일일이 수천 개의 답변을 읽고 점수를 매길 수 없다면 품질 변화를 어떻게 자동화된 수치로 측정해야할까?

위 문제를 해결하기 위해 플랫폼 팀이 구축하는 것이 **LLM Evaluation 파이프라인이다.** 모델의 답변을 다각도로 채점하는 자동화된 시스템이다.

- **LLM as a Judge**: 강력한 추론 능력을 가진 모델을 채점관으로 사용하여 타겟 Agent가 생성한 텍스트가 사전에 정의된 채점 기준을 얼마나 충족했는지 점수를 매기는 기법이다.
- **Faithfulness (신실성/사실부합성)**: 생성된 답변이 RAG에서 검색해온 문서의 내용만을 바탕으로 작성되었는지 할루시네이션은 없는지 측정하는 지표다.
- **Answer Relevance (답변 관련성)**: 생성된 답변이 사용자의 원본 질문 의도에 정확히 부합하는지 측정하는 지표다.
- **Context Precision (문서 검색 정밀도)**: 검색 모듈이 질문에 답하는데 곡 필요한 정답 문서를 상위 랭킹으로 잘 가져왔는지 측정하는 지표다.

<br>

## 문제 정의

**LLM의 응답이 매번 달라지는 특성 때문에 전통적인 테스트 자동화가 불가능한 문제가 존재했고**

**프롬프트나 RAG 파이프라인을 수정할 때마다 사람의 주관적이고 수동적인 평가에 의존하여 배포 속도가 지연되는 한계가 있었다.**

예를들면 이런 문제인데, B팀이 검색 품질을 높이겠다고 Vector DB의 임베딩 모델을 교체한 후, 실제로 고객 응답의 정확도가 얼마나 개선되었는지 혹은 엉뚱한 문서를 참고하는 환각이 얼마나 늘어났는지 수치화된 지표를 제시하지 못해 프로덕션 배포를 결정내리지 못하는 문제다.

### Solution

- **RAGAS 프레임워크 기반의 평가 파이프라인 도입**: 답변의 품질을 단순한 좋음/나쁨이 아니라 검색 품질 Context Precision Recall과 생성 품질 Faithfulness, Answer Relevance로 명확히 분리하여, 어느 구간(Retriever or Generator)에서 성능 하락이 발생했는지 추적한다.
- **CI/CD 통합 평가**: 플랫폼 팀은 사내 팀들이 평가용 데이터셋(질문, 정답 문맥, 이상적인 답변)만 제공하면 젠킨스나 깃헙 액션에서 파이프라인이 빌드될 때 자동으로 수천건의 평가를 돌려 변경된 agent의 품질 점수를 대시보드에 리포팅하는 인프라를 제공한다.

<br>

## 상세 동작 원리 및 구조화

플랫폼에서 타 팀의 Agent를 자동으로 평가하고 지표를 산출하는 로직 흐름이다.

```mermaid
graph TD
    Dataset[(Evaluation Dataset\n질문, 이상적 정답)] --> Tester[Platform Evaluation Engine]
    
    subgraph "Target RAG Agent"
        Tester -->|1. Test Query 입력| Agent[사내 서비스 Agent]
        Agent -->|2. 검색된 문서 (Contexts)| Tester
        Agent -->|3. 생성된 답변 (Answer)| Tester
    end
    
    subgraph "LLM-as-a-Judge (Evaluator)"
        Tester -->|4. 평가 프롬프트 조합| Judge[GPT-4 Evaluator]
        Judge -->|5. 채점 (Faithfulness 계산)| Metric1[신실성: 0.95]
        Judge -->|5. 채점 (Relevance 계산)| Metric2[관련성: 0.88]
    end
    
    Metric1 --> Dashboard[Platform Dashboard\n품질 리포트]
    Metric2 --> Dashboard
```

1. **테스트 데이터 주입**: 평가 엔진이 사전에 준비된 query를 타겟 agent에게 보낸다
2. **응답 및 컨텍스트 주입**: Agent가 답변을 생성하기 위해 참고한 원본 문서와 최종 생성된 텍스트를 모두 수집한다.
3. **LLM 평가자 호출**: 엔진이 질문, 검색된 문서, 생성된 답변 세 가지를 조합하여 평가용 LLM에게 전달하고 각 지표별 점수를 요구한다.
4. **점수 집계**: 평가용 LLM이 반환한 숫자 0.0 ~ 1.0를 평균을 내어 최종 품질 지표로 사용한다.

### Example

별도의 라이브러리 없이 원리 이해를 좀 해보자면 LLM이 답변의 환각 여부를 1점(실패) 또는 5점(성공)으로 평가하게 만드는 원리다.

```py
import json
import openai

def evaluate_faithfulness(question: str, context: str, answer: str) -> dict:
    """LLM을 평가자(Judge)로 사용하여 답변이 주어진 컨텍스트에만 기반했는지 채점합니다."""
    
    evaluator_prompt = f"""
    당신은 깐깐한 평가자입니다. 주어진 [답변]이 [참고 문서]의 내용만을 바탕으로 작성되었는지 평가하세요.
    문서에 없는 내용을 지어냈다면(Hallucination) 1점, 문서의 내용만으로 정확히 답했다면 5점입니다.
    
    결과는 반드시 JSON 형식으로만 반환하세요. 예: {{"score": 5, "reason": "문서의 내용과 일치함"}}
    
    [질문]: {question}
    [참고 문서]: {context}
    [답변]: {answer}
    """
    
    client = openai.OpenAI(api_key="sk-...")
    response = client.chat.completions.create(
        model="gpt-4",
        messages=[{"role": "user", "content": evaluator_prompt}],
        response_format={ "type": "json_object" } # JSON 응답 강제
    )
    
    return json.loads(response.choices[0].message.content)

# 원리 테스트
# print(evaluate_faithfulness("휴가 일수는?", "정규직 휴가는 15일입니다.", "15일입니다.")) -> score: 5
# print(evaluate_faithfulness("휴가 일수는?", "정규직 휴가는 15일입니다.", "20일입니다.")) -> score: 1
```

Evaulation API (RAGAS 활용)을 보면

플랫폼 팀이 사내 개발자들에게 제공하는 평가 파이프라인 스크립트다.

타 팀은 구체적으로 구축한 데이터셋(HuggingFace Dataset 포맷 등)을 이 함수에 통과시키기만 하면 검증된 RAGAS 알고리즘을 통해 수치화된 품질 리포트를 얻을 수 있다.

```py
from datasets import Dataset
from ragas import evaluate
from ragas.metrics import (
    faithfulness,
    answer_relevancy,
    context_precision
)
from langchain_openai import ChatOpenAI

def run_platform_evaluation(test_data_dict: dict):
    """
    사내 제품 팀이 제공한 테스트 데이터를 바탕으로 RAGAS 기반의 정량적 평가를 수행합니다.
    """
    
    # 1. 입력 데이터를 RAGAS가 요구하는 HuggingFace Dataset 포맷으로 변환
    # 필수 키: question, answer, contexts(List), ground_truth(이상적 정답)
    dataset = Dataset.from_dict(test_data_dict)
    
    # 2. 평가를 수행할 고성능 LLM 설정 (플랫폼 내부 Gateway 활용 권장)
    # 빠르고 저렴한 모델로 서비스하더라도, 평가는 무조건 가장 똑똑한 모델(GPT-4 등)로 수행해야 정확함
    evaluator_llm = ChatOpenAI(model_name="gpt-4", temperature=0.0)
    
    # 3. RAGAS 평가 실행 (비동기 병렬 처리 지원)
    print("플랫폼 평가 엔진 구동 중... 지표(Metrics) 계산을 시작합니다.")
    result = evaluate(
        dataset=dataset,
        metrics=[
            faithfulness,      # 답변이 컨텍스트에 기반하는가? (환각 측정)
            answer_relevancy,  # 질문과 답변이 관련 있는가?
            context_precision  # 검색된 문서가 유용한가? (검색 엔진 성능)
        ],
        llm=evaluator_llm
    )
    
    # 4. 결과 출력 및 대시보드 저장 (예: pandas DataFrame으로 변환)
    print("\n[평가 완료] 최종 품질 지표 리포트:")
    print(result)
    
    return result.to_pandas()

# --- 사내 팀 사용 예시 ---
# data = {
#     "question": ["2026년 복지 포인트는 얼마야?"],
#     "answer": ["100만 원입니다."],
#     "contexts": [["2026년 전 직원 복지 포인트는 100만 원으로 상향되었습니다."]],
#     "ground_truth": ["100만 원"]
# }
# report_df = run_platform_evaluation(data)
# save_to_dashboard(report_df)
```