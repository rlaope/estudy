# Preference Optimization: DPO 계열

복잡한 강화학습 파이프라인을 수학적 증명 하나로 우아하게 박살내고

오픈소스 진영의 표준으로 자리잡은 DPO (Direct Preference Optimization)을 알아보자

## Reward Model 없이도 모델은 인간이 선호하는 방향으로 이동이 가능한가

가능하다.

기존의 RLHF(PPO)는 인간의 선호도를 학습시켜 반드시 Reward Model 이라는 별도의 채점관 AI를 먼저 학습시켜야 했다.

하지만 DPO 연구진은 강화학습의 수학적 수식을 역산하며 최적의 보상 함수는 이미 Policy(우리가 학습시킬 LLM) 자체의 확률 분포로 완벽하제 치환할 수 있다는 것을 증명했다.

<br>

## 어떤 문제가 있었길래 DPO가 등장했는가. PPO의 한계

기존 PPO (Proximal Policy Optimization) 방식의 RLHF 파이프라인을 구축하려면 다음의 기술적 부채를 감당해야했다 (PPO 방식 GRPO, RLVR등은 다음 시간에 알아볼거임)

- **메모리 폭발 OOM:** PPO를 돌리려면 GPU 메모리 위에 무려 4개의 모델 (1. Actor Model, 2. Reference Model, 3. Reward Model, 4. Value Model)을 동시에 띄워야한다. 70B 모델이라고 치면 상상 초월 인프라 비용이 필요하다.
- **불안정성:** 강화 학습 특성상 Reward Hacking (모델이 꼼수로 점수만 높게 받는 현상)이 자주 발생하고 학습률이나 KL Penalty 등의 하이퍼파라미터에 극도로 민감하여 학습 붕괴가 빈번하다.

<br>

## DPO의 원리 - 수학적 우회로

DPO는 위 문제를 해결하기 위해 학습 파이프라인을 **단순한 분류 문제로 바꿨다.**

1. **데이터 준비 (Prefernece Pair):** 하나의 질문 prompt에 대해 좋은 답변 chosen과 나쁜 답변 rejected 쌍을 준비한다.
2. **Implict Reward (내재적 보상):** DPO는 모델에게 이렇게 지시한다 "네가 기존 Reference 에 가지고 있던 생각과 비교했을때 chosen 토큰을 뱉을 확률 Log Probability는 높이고 Rejected 토큰들은 뱉을 확률을 낮추어라'
3. **자원 절약:** PPO의 4개 모델중 Reward, Value 모델을 날려버리고 딱 2개의 모델 (학습할 Policy Model, 기준점 역할을 할 Reference Model)만 메모리에 올려버리면 된다 (LoRA를 쓰면 사실상 1개의 모델만 올리는것과 같아짐)

<br>

## TRL을 활용한 DPO 파이프라인

코드를보면 DPO가 SFT와 얼마나 비슷한지, 왜 엔지니어들이 열광하는지 직관적으로 알 수 있다.

```py
import torch
from datasets import load_dataset
from transformers import AutoModelForCausalLM, AutoTokenizer
from trl import DPOTrainer, DPOConfig
from peft import LoraConfig

# 1. 모델 로드 (Policy와 Reference를 동일한 Base 모델에서 시작)
model_id = "my-sft-8B-model" # 앞서 SFT가 완료된 모델
tokenizer = AutoTokenizer.from_pretrained(model_id)

# DPO는 내부적으로 원본 가중치(Reference)와 업데이트될 가중치(Policy)를 비교합니다.
model = AutoModelForCausalLM.from_pretrained(model_id, torch_dtype=torch.bfloat16)

# 2. Preference 데이터셋 로드
# 구조: {"prompt": "...", "chosen": "좋은 답변", "rejected": "나쁜 답변"}
dataset = load_dataset("json", data_files="preference_data.jsonl", split="train")

# 3. LoRA 설정 (PEFT를 통해 메모리 최적화)
peft_config = LoraConfig(
    r=16,
    target_modules=["q_proj", "v_proj", "k_proj", "o_proj"],
    task_type="CAUSAL_LM"
)

# 4. DPO 하이퍼파라미터 설정
dpo_config = DPOConfig(
    output_dir="./dpo_results",
    beta=0.1, # KL Penalty (Reference 모델에서 너무 멀어지지 않게 잡아주는 힘)
    learning_rate=5e-6, # SFT보다 훨씬 작은 LR 사용
    per_device_train_batch_size=2,
    gradient_accumulation_steps=8,
)

# 5. DPOTrainer 실행 (Reward Model 없이 직접 튜닝!)
trainer = DPOTrainer(
    model,
    args=dpo_config,
    train_dataset=dataset,
    tokenizer=tokenizer,
    peft_config=peft_config
)

trainer.train()
```

### DPO 학습 결과 및 Chosen / Rejected Win-rate 분석

DPO 학습지 제대로 진행되고 있는지 판단하는 핵심 지표는 일반적인 Loss 값이 아니라 로그 확률의 격차다.

- **Reward Margin (평가 지표):** 학습이 진행될 수록 모델이 chosen 응답에 부여하는 확률 implict reward는 우상향하고 rejected 응답 여부하는 확률에는 우하향 해야한다.
- **Win-Rate (최종 산출물):** DPO 적용 전 SFT 모델과 적용 후 DPO 모델을 비교 평가 LLM-as-a-Judge 했을때 다음과 같은 결과가 도출된다.

| 평가 항목 (1,000개 Test 셋) | SFT Model 승리 비율 | DPO Model 승리 비율 | 무승부 |
|---------------------------|-------------------:|-------------------:|-------:|
| 지시 사항 준수 (Instruction Following) | 12% | 85% | 3% |
| 거절 응답 (안전성 / Safety) | 4% | 96% | 0% |
| 답변의 포맷팅 및 가독성 | 22% | 70% | 8% |

결론은 별도의 Reward Model을 구축하는 막대한 리소스 없이 오직 고품질 chosen, reject 데이터만으로 모델의 행동을 인간의 선호에 맞게 완벽하게 정렬 alignment하는데 성공했다.

## PEFT (Parameter-Efficient Fine-Tuning)

PEFT는 모든 파라미터를 다 학습시키지 않고도 **극히 일부만 효율저긍로 골라내 학습시키는 파인튜닝 방법론의 총칭이다.**

기존 방식 Full-Fine Tuning: Llama-3-8B 모델을 학습시킨다면 80억개의 가중치 Parameters를 전부 업데이트 해야한다.

이때 가중치 뿐만 아니라 옵티마이저 상태 Optimizer States, Gradients까지 메모리에 올려야 하므로 모델 크기의 수 배에 달하는 막대한 VRAM이 필요하다.

**PEFT 방식**은 기존 모델의 가중치는 얼려두고 전체 파라미터 0.1~1% 수준의 아주 작은 어댑터 가중치만 추가하여 그 부분만 학습시켜 결과적으로 GPU 메모리 요구량이 극적으로 줄어들어 단일 GPU에서도 대형 모델 학습이 가능해진다.

### LoRA (Low-Rank Adapation)

LoRA는 PEFT 방법론중 가장 널리 쓰이는 구체적 알고리즘이고 행렬의 Low-Rank(낮은 순서/차원) 분해 특성을 이용한다.

- **동작 원리**: 원래 모델의 거대한 가중치 행렬 $W_0$ (예: $4096 \times 4096$ 차원)는 그대로 열러둔다 대신 그 옆에 아주 날씬한 두 개의 가중치 행렬 A B를 붙인다.
- 거대한 행렬을 학습시키는 대신  $B \times A$라는 작은 두 행렬의 곱을 통해 가중치의 변화량($\Delta W$)을 표현하는 방식이다.
- **Merge**: 학습이 끝나면 기존 가중치 $W_0$에 $\Delta W$를 수학적으로 더해버리면 끝난다. $W = W_0 + \Delta W$ 추론 시점에 추가적인 연산 지연이 전혀 발생하지 않는다는 것이 장점이다.

<br>

## 생각하는 법도 가르칠 수 있을까

DPO가 말 예쁘게 하기, 해로운 답변 피하기 같은 스타일과 가치관 교정 (Alignment)에는 압도적인 성능을 발휘한다.

하지만 1+1=2 같은 명확한 수학적 진리가 존재하는 영역에서는?

수학 문제 풀이에는 선호 preference 라는 개념이 적용되기 어렵다. A 풀이와 B 풀이중 더 선호하는 것을 고르는게 아니라 그냥 틀린 풀이와 맞는 풀이가 있을 뿐이다.

즉 **추론 (Reasoning) 능력은 DPO로 끌어 올리는데 근본적인 한계가 존재한다.**

Llama-3에서 정체되어 있을때 OpenAI의 o1모델과 DeepSeek-R1는 이 한계를 어떻게 박살냈을까

정답은 DPO같은 비교 Pairwise 방식이 아니라 모델 스스로 수천번의 chain of thought를 전개하여 rule-based reward에 따라 보상을 탐색하는 추론 특화 강화 학습 GRPO & RL for Reasoning등이 있다.

담에 알아보자