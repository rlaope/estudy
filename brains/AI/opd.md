# Off-policy Distillation과 Teacher-Student 학습

오픈소스 모델 생태계가 폭발적으로 성장한 배경에는 이 기법이 있다.

값비싼 사람이 직접 라벨링을 하는 대신 GPT-4 or Llama-3-70B 같은 거대한 Teacher 모델이 만들어낸 수십만개의 정답을 작은 student 모델이 모방하게 만드는 방법론이다.

## Teacher가 만든 좋은 답변을 그대로 학습하면 Student도 같은 Reasoning 능력을 얻는가?

"GPT-4의 답변 100만개를 8B모델에 학습시키면 8B 모델도 GPT-4처럼 똑똑해질까?" AI 엔지니어가 가장 많이하는 기대이자 착각이다.

결론부터 말하면 **Student 모델은 Teacher의 추론능력 Reasoning 자체를 얻는것이 아니라 Teacher의 표면적인 말투와 형식 (Styled and Format) 만을 모방하는데 그치는 경우가 많다.** 이를 학계에서는 **False Promise of Imitation Learning** 또는 **Imitation Gap** 즉 모방 격차라고 한다.

- **Style Imitation**: Student는 Teacher가 자주 쓰는 단어 (예: 첫째, 따라서, 결론적으로) 같은 논리적 전개 형식을 매우 빠르고 와녑ㄱ하게 배운다.
- **Reasoning Bottleneck (추론의 한계)**: 하지만 복잡한 수학 문제나 다단계 논리 퍼즐이 주어지면, 형식은 완벽한 단계별 풀이를 내놓지만, 중간 연산은 완전하게 틀려버리는 환각이 있고, Student의 제한된 파라미터 Capacity로는 Teacher의 깊은 내부 공간을 온전히 담아낼 수 없기 때문이다.

> SFT는 사람이 만든 정답 데이터로 평가하는 Off-Policy이고 Off-Policy Distilation은 Teacher 모델이 평가하는 Off-Policy이다. 이 차이를 기억해두자

<br>

## Off-policy의 개념적 이해

강화학습 및 행동복제 (Behavioral Cloning) 맥락에서 Off-Policy라는 단어는 데이터의 출처를 의미한다.

- **On-Policy:** 현재 훈련중인 모델(student)이 직접 생성한 궤적이나 답변을 평가하고 그 피드백을 학습하는 방식론이다. (RLHF, PPO)
- **Off-Policy:** 학습 데이터가 현재 모델의 확률 분포 $\pi_{\theta}$가 아닌, 외부의 고정된 분포 (Teacher 모델의 정책 $\pi_{teacher}$)로부터 생성된 경우를 말한다.

Off Policy Distillation은 Teacher 모델에게는 매우 쉽고 자연스러운 답변 분포지만, 작은 Student 모델의 파라미터 구조에는 도달하기 극도로 어려운 토큰 분포일수도 있다.

이로 인해 모델이 억지로 정답을 외우려다 Distribution Shift 분포의 붕괴를 겪기도 한다.

<br>

## Synthetic Dataset Generation Pipeline 구축

Distillation의 첫 단계는 vLLM과 같은 고속 추론 엔진을 사용하여 무수히 많은 프롬프트에 대한 Teacher 모델의 응답 Synthetic Data를 대량으로 찍어내는 Generation 것이다.

아래는 py, vLLM 활용을 해 고정된 JSONL 형식의 데이터셋을 구축하는 파이프라인 예시다.

```py
import json
from vllm import LLM, SamplingParams

# 1. 막강한 지능을 가진 Teacher Model 로드 (vLLM 엔진 사용)
# 실제 환경에서는 다중 GPU(TP) 설정이 필요합니다.
teacher_model_id = "meta-llama/Meta-Llama-3-70B-Instruct"
llm = LLM(model=teacher_model_id, tensor_parallel_size=4)

# 2. 다양한 지시문(Seed Prompts) 목록
# 보통 수만~수백만 개의 instruction을 준비합니다. (예: Alpaca, Evol-Instruct 방식)
prompts = [
    "파이썬에서 리스트 컴프리헨션이 메모리 효율적인 이유를 설명해 줘.",
    "양자 얽힘을 초등학생에게 설명하듯 비유를 들어 설명해 봐.",
    # ... 수만 개의 프롬프트 ...
]

# 3. Sampling Parameters 설정 (다양성과 품질 확보)
# 온도를 낮춰 논리적이고 정제된 답변을 유도합니다.
sampling_params = SamplingParams(
    temperature=0.3,
    top_p=0.9,
    max_tokens=1024
)

# 4. vLLM을 통한 대규모 병렬 추론 실행 (Generation)
print("Teacher Model이 답변을 생성하고 있습니다...")
outputs = llm.generate(prompts, sampling_params)

# 5. 결과를 JSONL(또는 Parquet) 형태로 디스크에 저장 (Student 모델의 SFT 데이터셋용)
output_file = "synthetic_distillation_dataset.jsonl"
with open(output_file, 'w', encoding='utf-8') as f:
    for output in outputs:
        prompt = output.prompt
        generated_text = output.outputs[0].text.strip()
        
        # SFT 학습 규격에 맞게 저장
        data_point = {
            "instruction": prompt,
            "output": generated_text
        }
        f.write(json.dumps(data_point, ensure_ascii=False) + '\n')

print(f"Distillation 데이터셋 생성이 완료되었습니다: {output_file}")
```

이렇게 만들어진 `synthetic_distillation_dataset.jsonl` 파일은 앞선 장의 SFTTraniner에 그대로 주입되어 Student 모델 Llama-3-8B를 학습시키는 데 사용한다.

<br>

## 산출물 Distillation 결과 및 Imitation Gap 분석 리포트

10만개의 Teacher(70B) 생성 데이터를 Student(8B)에 SFT 방식으로 Distillation 한 후, 추론 Reasoning이 필요한 새로운 논리 문제로 두 모델을 테스트한 정성적 비교다.

### 테스트 프롬프트 - 수리 논리력 테스트

User: "농장에 닭과 돼지가 합쳐져 35마리가 있다. 다리의 수를 모두 세어보니 94개였고 닭과 돼지는 각각 몇 마리인지 단계별로 논리적으로 계산하라"

```
이 문제는 연립방정식을 통해 해결할 수 있습니다.
1단계: 변수 설정
- 닭의 수를 x, 돼지의 수를 y라고 합니다.
2단계: 방정식 세우기
- 총 마리 수: x + y = 35
- 총 다리 수: 2x + 4y = 94 (닭은 다리가 2개, 돼지는 4개)
3단계: 방정식 풀이
- 첫 번째 식에서 x = 35 - y를 두 번째 식에 대입합니다.
- 2(35 - y) + 4y = 94
- 70 - 2y + 4y = 94
- 2y = 24
- y = 12 (돼지 12마리)
- x = 35 - 12 = 23 (닭 23마리)
결론: 닭은 23마리, 돼지는 12마리입니다.
```

완벽한 논리 전개와 정확한 산술 연산을 보여준다.

### Student Model (8B 0 Distilled) 출력 - 학습 결과


```
이 문제는 연립방정식을 통해 해결할 수 있습니다.
1단계: 변수 설정
- 닭의 수를 x, 돼지의 수를 y라고 합니다.
2단계: 방정식 세우기
- 총 마리 수: x + y = 35
- 총 다리 수: 2x + 4y = 94
3단계: 방정식 풀이
- 첫 번째 식에 2를 곱합니다: 2x + 2y = 70
- 두 식을 뺍니다: (2x + 4y) - (2x + 2y) = 94 - 70
- 2y = 14  <-- [에러 발생: 94-70은 24이나, 14로 계산함]
- y = 7 (돼지 7마리)
- x = 35 - 7 = 28 (닭 28마리)
결론: 닭은 28마리, 돼지는 7마리입니다.
```

- **평가 Imitation Gap 발현:** Teacher의 단계별 포맷, 변수 설정, 심지어 연립 방정식을 푸는 전개 방식까지 복제했다. 하지만 작은 모델의 한계로 중간 산술 94 - 70 = 14 (24인데) 라는 환각이 발생해서 최종 논리가 붕괴되었다.

결론적으로 Off-Policy Distillation은 사용할 때 단순한 SFT 만으로는 Teacher의 지능을 복사할 수 없음을 인지해야한다. 모델의 기초 체력 Base Model의 규모 및 사전 학습 데이터 품질이 추론 능력의 상한성을 결정하며 Distillation은 단지 그 상한선 내에 모델이 말을 잘 듣게 만드는 촉매제 역할로 이해하고 설계해야한다.

<br>

## 어떤 방식으로 학습하길래 문제가 있는가

SFT, Distillation은 본질적으로 **다음 단어 맞추기 Next Token Prediction** 게임이다.

정답지 (Teacher의 완벽한 답변)를 펼쳐놓고, 모델에게 이 문맥 다음에는 무조건 이 단어가 와야해!라고 주입한다.

이는 머신러닝 용어로 Teacher Forcing이라고 한다.

### 그래서 어떤 문제가 있는가.

이 주입식 교육은 치명적인 두 가지 문제가 있다.

- **온실 속 화초 현상 Exposure Bias**: 학습할 때 완벽하게 작성된 정답의 길만 걷는다. 하지만 실제 서비스 추론 단계에서는 모델이 자기가 직접 단어를 하나씩 뱉어야한다. 만약 중간에서 숫자 하나를 실수로 잘못 뱉었다면 모델은 자기가 한 실수를 수습하려고 다시 정답으로 돌아오는 방법을 단 한번도 배운적이 없기 때문에, 그 순간부터 논리가 도미노처럼 와르르 무너져버린다.
- **생각의 생략 (표면적 모방)**: 큰 모델은 수학 문제를 풀 때 내부적으로 엄청나게 깊고 복잡한 연산 Reasoning을 거친 뒤에 최종 텍스트를 반환하지만, 작은 모델은 그 생각의 깊이는 보지 못하고 겉으로 들어난 텍스트만 보기 때문에 수학 공식을 왜 그렇게 전개해야하는지 이해하지 못한다 "이런 문제에는 대충 이런 순서로 숫자를 나열하면 칭찬받는구나"하고 풀이 과정의 껍데기만 외운다

### 해결 방식

그렇다면 정답의 껍데기만 외우는 주입식 교육의 한계를 파훼해아한다.

정답은 텍스트를 강제로 떠먹여주기보단 "스스로 수만번 부딪히고 돌려가며 최종적으로 정답을 맞혔을때 큰 보상을 주는 방식"으로 훈련 패러다임을 뒤집는 것이다.

마치 자전거를 배울때 몸으로 넘어지면서 중심을 잡는 법을 스스로 깨우치는 것

단순한 '단어 예측기'를 넘어, 스스로 논리를 전개하고 Thinking하는 법을 깨우치게 만든 AI 진화의 마스터키

인간의 피드백과 보상을 통해 모델의 뇌 구조를 근본적으로 뜯어고치는 강화학습 RLHF, DPO 그리고 최근 세상을 놀라게한 추론 중심 강화학습 OpenAI o1, DeepSeek-R1의 GRPO가 존재한다.

다음시간에 보겠다.