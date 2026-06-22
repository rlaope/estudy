# On-policy SFT와 Rejection Sampling Fine-Tuning

Off-Policy Distilliation에서 언급한 문제 남의 정답을 억지로 외우는 방식

즉 외우기만해서 정해진 길 외에 문제가 발생했을때 고치지 못하는 그런 주입식 교육의 한계 방식을 

극복하기 위해 등작한 과도기적, 하지만 실무에서는 강력한 방법론인 On-Policy SFT (Rejection Sampling Fine-Tuining RFT)를 먼저 알아보자.

## RL 없이도 현재 모델이 만든 좋은 샘플만 골라 다시 SFT 하면 성능이 좋아지는가.

**네 좋아진다.** 엄청나게다. 특히 수학 문제를 풀이나 코딩처럼 명확한 채점 기준 Verifier이 존재하는 도메인에서는 기적같은 향상을 보여준다.

앞선장에서 Teacher 모델의 정답을 외우면 모방격차 Imitation Gap이 생성된다고 했다.

Teacher의 뇌 구조(파라미터 공간)과 Student의 뇌 구조가 다르기 때문이다.

하지만 RFT (Rejection Sampling Fine-Tuning)은 Teacher를 버리고 Student 스스로 여러번 생각하게 만든다.

한 문제에 대해 Student의 온도 (Temperature)를 높여 마루허게나 100가지 방식으로 풀어보라고 시킨다.

95개는 헛소리일거고 우연히 자기 자신의 논리 on-policy로 정답에 도달하는 5개의 풀이가 나온다.

이 5개는 student의 뇌 구조에는 완벽하게 들어맞는 소위 입에 착착 감기는 논리 전개 방식이다.

이걸 골라내서 다시 자기 자신을 sft 시키면 억지로 외우는 것이 아니라 **자기 자신이 어렴풋이 알던 깨달음으로 바뀌는 과정이 된다.**

<br>

### Generate N -> Score -> Filter -> SFT Pipeline

이 파이프라인을 어떻게 구축하는지 스택별로 쪼개보자

### Step 1: Generate N (대량 생성)

- stack: vLLM

모델에게 동일한 질문을 주고 단 한번만 묻는것이 아니라 `n=56`, `temperature=0.7`을 주어 하나의 질문에 대해 50개의 다양한 추론 경로 Reasoning Path를 병렬로 쏟아내게 한다.

### Step 2: Score & Filtering (채점 및 필터링)

- stack: Python Verfiler, Regex Parser, Unit Test, LLM-as-a-Judge

가장 중요한 단계다 50개의 답변 중 쓰레기를 걸러내고 진짜 정답만 남긴다

- **수학:** 파이썬 정규식 regex를 돌려 `\boxed{정답}` 부분만 추출해 실제 정답과 일치하는지 확인한다 (Rule-Based Grader)
- **코딩:** 생성된 코드를 샌드박스 위에 실행해 Unit Test를 돌려본다.
- **일반 대화:** 정답이 없는 경우 더 똑똑한 모델 GPT-5-5 or 판사 LLM as a Judge로 써서 이 답변이 지시사항을 잘 따랐나 하고 점수를 매긴다


### Step 3: SFT (다시 학습)

- stack: `TRL SFTTrainer`

필터링을 통과한 고품질의 On-Policy 데이터들만 모아 앞서 SFT 내용에서 봤던 코드 SFTTrainer로 학습을 한바퀴 더 돌린다.

```py
import re
from vllm import LLM, SamplingParams

# 1. 현재 Student 모델 로드 (On-policy)
model = LLM(model="my-student-8B-model")

prompt = "문제: 12와 18의 최소공배수를 구하고 최종 답은 \boxed{} 안에 넣어줘."

# 2. Generate N: 동일 문제에 대해 20개의 다른 풀이 생성
sampling_params = SamplingParams(n=20, temperature=0.8, max_tokens=512)
outputs = model.generate([prompt], sampling_params)

# 3. Score & Filter: 파이썬 정규식 Verifier
good_samples = []
ground_truth = "36"

for output in outputs[0].outputs:
    text = output.text
    # \boxed{숫자} 형태를 찾는 정규식 파서
    match = re.search(r'\\boxed\{(.+?)\}', text)
    
    if match:
        extracted_answer = match.group(1).strip()
        if extracted_answer == ground_truth:
            # 정답을 맞춘 '풀이 과정(Reasoning Path)'만 수집
            good_samples.append({
                "instruction": prompt,
                "output": text # 자기 자신이 만들어낸 성공적인 풀이 과정
            })

print(f"20개 중 정답을 도출한 훌륭한 샘플 {len(good_samples)}개 확보 완료. SFT로 넘어갑니다.")
```

<br>

## 왜 이 좋은걸 두고 복잡한 RL을 할까?

RFT는 구현이 쉽고 성능도 좋지만 한계가 명확하다

1. **부분 점수가 없다 (Binary Reward):** SFT는 정답이 아니면 오답이다. 풀이 과정 90%까지는 천재적이었는데 마지막 덧셈 실수 하나를 했다면 -? RFT 에서는 오답으로 간주되어 통째로 버려진다. 그 아까운 천재적 추론 능력이 모델이 배우지 못한다.
2. **비용 낭비 (Loss of Negative Signal):** 모델이 어떤식으로 생각하면 틀리는지 오답노트에 대한 정보도 배워야하는데, 맞춘 데이터만으로 SFT를 하니까 이렇게는 하지마! 같은 제약을 걸 수 없다.

맞춘건 알겠는데 더 좋은 풀이와 덜 좋은 풀이를 미세하게 구분해서 모델의 가중치를 정밀하게 깎아낼 순 없을까? 오답을 생성했을때, 패널티를 주어 다시 그런 바보같은 길로 빠지게 할 순 없을까

다음시간에 알아볼 이 문제를 해결하기 위한게 수학적 공식을 통해 모델에게 정교한 보상을 넣는 RLHF(PPO, DPO) 그리고 추론 중심 강화학습 GRPO이다.

모델이 스스로 생각의 깊이를 키워가는 포스트 트레이닝의 일종이다.