# Online RL: PPO, GRPO, RLVR

Post-training의 최종 단계는 모델이 고정된 데이터셋의 한계를 벗어나, 환경과 직접 상호작용하며 최적의 행동 양식을 스스로 터득하는 온라인 강화학습이다.

## 고정 데이터 학습 SFT/DPO와 온라인 탐색 RL의 본질적인 차이는?

엔지니어링 관점에서 오프라인 학습 SFT, DPO와 온라인 RL의 차이는 탐색 공간 Exploration Space의 동적 확장 여부에 있다.

- **SFT/DPO (Static Offline):** 학습 데이터셋이 이미 고정되어 있다. 모델은 주어진 데이터 분포 범위 내에서만 정렬을 수행하므로 데이터셋에 없는 완전히 새로운 추론 경로를 개척하거나 스스로 논리적 도약을 이루어내기가 구조적으로 불가능하고 실제 추론 시 발생할 수 있는 미세한 오차를 수습하는 법을 배우지 못한다.
- **Online RL (Dynamic Online):** 모델 Policy는 고정된 텍스트를 읽는 대신, Tempertature를 높여 환경 속에서 수많은 오답과 정답의 줄타기를 수행한다 (rollout) 이 과정에서 성공한 경로에는 강한 보상 Positive Reward을, 실패한 경로에는 패널티 Negative Signal을 찔러넣어 가중치를 최적화한다. 결과적으로 모델은 사전에 정의된 정답 가이드라인을 뛰어넘어 **스스로 생각하는 과정 Chain of Thought의 효율적인 지름길을 탐색하는 인지 능력을 발달시키게 된다.**

<br>

## PPO, GRPO, RLVR

### PPO (Proximal Policy Optimzation) - and Problem

PPO는 2017년 OpenAI가 발표한 온라인 강화학습 알고리즘이다.

강화학습에서 모델 Policy는 Reward를 많이 받기 위해 자신의 가까운 가중치를 계속 업데이트한다.

하지만 일반적인 GD에서는 그대로 적용하면 아래와 같은 문제가 발생한다

- **정책 붕괴 Policy Collapse:** 가중치가 한 번에 너무 크게 바뀌면 모델이 기존에 가지고 있던 유창한 언어 생성 능력 (문법, 문맥 유지등)이 한순간에 망가지고 헛소리를 뱉는 상태가 된다.
- **해결책:** PPO는 새로운 정책 $\pi_\theta$이 안전했던 이전 정책 $\pi_{\theta_{old}}$)에 너무 멀어지지 않도록 근접한 Proxmimal 영역 내에서만 조금씩 똑똑해지도록 제어하기 때문에 붙여진 이름이다.

$$L^{CLIP}(\theta) = \hat{\mathbb{E}}_t \left[ \min\left(r_t(\theta)\hat{A}_t, \text{clip}(r_t(\theta), 1-\epsilon, 1+\epsilon)\hat{A}_t\right) \right]$$

PPO는 가중치를 안전하게 업데이트 하기위해 다음과 같은 클리핑 손실 함수를 사용한다.

- $r_t(\theta) = \frac{\pi_\theta(a_t|s_t)}{\pi_{\theta_{old}}(a_t|s_t)}$ (Probability Ratio): 과거 정책 대비 현재 정책이 이 토큰(행동)을 생성할 확률이 얼마나 변했는지를 나타내는 비율이다. 완전히 똑같다면 1
- $\hat{A}_t$ (Advantage, 우위): 모델이 뱉은 답변이 평균적인 답변(Value) 보다 얼마나 더 나은지를 점수화한것이다. 값이 양수면 좋은 답변 음수면 나쁜 답변이다.
- $\text{clip}(r_t(\theta), 1-\epsilon, 1+\epsilon)$: 확률 비율 $r_t(\theta)$가 너무 커지거나 작아지지 않도록 $[1-\epsilon, 1+\epsilon]$ 범위(보통 $\epsilon = 0.1 \sim 0.2$)로 잘라 버린다.

방어 메커니즘의 핵심은 어떤 답변이 엄청나게 좋은 보상을 받아서 Advantage($\hat{A}_t$)가 폭발하더라도 Clip 덕분에 가중치가 한번에 과도하게 업데이트 되는것을 수학적으로 차단한다.

전통적인 RLHF의 표준인 PPO 모델의 상태 가치 State Value를 예측하여 행동을 보정하는 Critic (Value Model)을 필수적으로 요구한다 이로 인해 학습중 GPU VRAM 위에 Actor, Refernece, Critic, Reward 총 4개의 대형 모델을 적재해야하므로 오버헤드가 심하다.

실제 GPU 환경에서 PPO 기반의 RLHF를 가동할때 메모리가 폭발하는 이유에 대한 모델들을 정리해보면

1. **Actor Model (Policy, $\pi_\theta$):** 우리가 실제로 학습시키고자 하는 target LLM이고 프롬프트를 입력받아 토큰을 생성하고, 보상에 따라 가중치가 계속 변한다.
2. **Reference Model ($\pi_{ref}$):** 학습 시작 직전의 SFT 완료된 모델로 가중치가 고정되어 있고 Actor가 보상을 쫓다가 아예 다른 언어를 구사하는 것을 막기 위해 Actor의 토큰 분포가 이 Refernece 분포와 너무 멀어지지 않도록 패널티 (KL Divergence Penalty)를 부여하는 기준점 역할을 한다.
3. **Critic Model (Value Model):** Actor가 토큰을 생성하는 도중 현재 문맥이 최종적으로 좋은 점수를 받을지 나쁜 점수를 받을지 예측하는 평가관이고 이 예측값과 실제 Reward으 차이를 위 수식의 Advatnage를 계산해낸다 학습 과정에서 가중치는 계속 업데이트 된다.
4. **Reward Model ($R_\psi$)**: 인간의 선호도를 미리 학습해둔 **가중치 고정** 모델이다. Actor가 완성한 최종 문장을 읽고 인간이 좋아할 만한 답변인가를 판별해 최종 점수 Scalar Reward를 부여한다.

<br>

## GRPO (Group-Relative Policy Optimization)

DeepSeekMath 연구진이 제안한 GRPO는 Critic(Value Model)을 완전히 제거하여, VRAM을 50퍼이상 아끼는 혁신을 이루어냈다.

**원리는 하나의 프롬프트 ($R_\psi$)에 대해 현재 Policy모델이  $G$개의 답변 묶음 (Group, $O_1, O_2, ..., O_G$)를 동시에 Rollout**한다. 그 후 각 답변이 받은 보상 ($R_1, R_2, ..., R_G$)들의 평균과 표준편차를 구해 그룹 내부의 상대적 우위 Advantage를 계산한다.

i번째 샘플의 Advantage $A_i$는 다음과 같이 계산된다.

$$A_i = \frac{R_i - \mu(R)}{\sigma(R)}$$

별도의 가치 평가 모델 없이 그룹 내 대조군들을 통해 가중치 업데이트 방향을 결정하므로 가중치 크기가 큰 LLM 서빙/학습 환경에서는 극단적인 파라미터가 최적화와 대규모 배치 처리가 가능해진다.

> GRPO 내의 그룹은 단일 토큰 위치에서 예측 벡터 분포가 아니라 동일한 질문에서 파생된 서로 다른 완성된 답변 시퀀스 묶음이다 이 대조군들끼리 서로 경쟁을 붙여 A볻다 B로 답변 텍스트를 구성하는것이 더 이득이구나를 모델 스스로 깨우치게 만드는 구조다.
>
> RFT와 GRPO와의 다른점은 RFT는 N개의 샘플중 검증기 Verfier를 통과한 정답만 추려내어 SFT(Cross-EntropyLoos)로 가중치를 학습하고
>
> 반면 GRPO는 한 훈련스텝 내에서 생성된 그룹 샘플간의 상대적 보상 우위를 계산해 정책 경사 Policy Gradient 알고리즘으로 최적화한다.
>
> 이로인해 RFT는 오답 샘플을 버리기만할뿐 오답 경로에 패널티를 주지 못하지만, GRPO는 그룹 평균보다 점수가 낮은 샘플의 생성확률을 직접 끌어내리는 음의 가중치 업데이트가 일어난다.
>
> RFT는 사전에 필터링된 고정 데이터를 재학습하는 오프라인 방식이라면 GRPO는 실시간 탐색 Rollout을 통해 현재 모델의 토큰 분포 상태를 동적으로 비교하여 최적화한다.
>
> 즉, GRPO는 목적 함수 수식을 통해서 어떤 추론 경로가 다른 경로보다 상대적으로 우수한지를 매 스텝마다 수학적으로 비교하고 반영한다는 차이가 있다.


<br>

## RLVR (Reinforcement Learning from Verfier Feedback)

기존에는 RLHF에서 인간의 선호도를 흉내낸 Proxy Reward Model을 흉내내씩에 모델이 꼼수를 부려 문장 스타일만 좋게 만들어 점수를 따내는 Reward Hacking에 취약했다.

RLVR은 코딩(Unit Test), 수학 (Exact Match), SQL(Execution)처럼 **결과물이 절대적으로 검증가능한 도메인에 한해 인간 평가자 대신 규칙 기반 검증기 Verifer를 보상 함수로 직접 연결하는 구조이다.**

보상의 비결정성이 완전히 제거되므로 추론학습의 안정성이 폭발적으로 상승한다.

<br>

## TRL GRPOTraniner 기반 수학 추론 최적화 실험

Huggin Face trl 라이브러리 GRPOTrainer 써서 수학적 추론 능력과 특정 출력 포맷을 동시에 학습시키는 실전 파이프라인 구현 코드를 보자

```py
import re
import torch
from datasets import load_dataset
from transformers import AutoTokenizer, AutoModelForCausalLM
from trl import GRPOTrainer, GRPOConfig

# 1. Base Model 및 토크나이저 로드 (vLLM 호환 가속 설정)
model_id = "meta-llama/Meta-Llama-3-8B-Instruct"
tokenizer = AutoTokenizer.from_pretrained(model_id)
tokenizer.pad_token = tokenizer.eos_token

model = AutoModelForCausalLM.from_pretrained(
    model_id,
    torch_dtype=torch.bfloat16,
    device_map="auto"
)

# 2. 데이터셋 로드 (수학 문제와 엄격한 정답 매핑 구조)
# 예시 딕셔너리 구조: {"prompt": "15 * 4 / 2 = ?", "answer": "30"}
dataset = load_dataset("json", data_files="math_reasoning_data.json", split="train")

# 3. Reward Functions (보상 함수 정의 - 핵심 축)

def correctness_reward_func(prompts, completions, answer, **kwargs):
    """정답 검증 보상 (RLVR 방식): 정답 유무를 완벽히 대조"""
    rewards = []
    for completion, ans in zip(completions, answer):
        # 모델의 출력 중 최종 정답 블록 \boxed{결과} 추출
        match = re.search(r'\\boxed\{(.+?)\}', completion)
        if match and match.group(1).strip() == ans.strip():
            rewards.append(2.0) # 정답 시 높은 가중 보상
        else:
            rewards.append(0.0)
    return rewards

def format_reward_func(prompts, completions, **kwargs):
    """포맷 제어 보상: DeepSeek-R1 스타일의 <thinking> 구조 강제"""
    rewards = []
    for completion in completions:
        # 논리적 사고 과정을 거치고 최종 답안 포맷을 맞췄는지 정규식 검사
        pattern = r"<thinking>.*?</thinking>\s*.*\\boxed\{.*?\}"
        if re.search(pattern, completion, re.DOTALL):
            rewards.append(1.0) # 포맷 만족 시 보상 부여
        else:
            rewards.append(0.0)
    return rewards

# 4. GRPO 학습 하이퍼파라미터 세팅
training_args = GRPOConfig(
    output_dir="./grpo_math_results",
    learning_rate=1e-6,
    per_device_train_batch_size=2,
    gradient_accumulation_steps=4,
    num_train_epochs=1,
    bf16=True,
    
    # GRPO 핵심 매개변수 설정
    num_generations=8,     # 하나의 프롬프트당 생성할 그룹 크기 (G=8)
    max_completion_length=512,
    beta=0.04,             # KL divergence penalty 가중치
)

# 5. GRPOTrainer 초기화 및 가동
trainer = GRPOTrainer(
    model=model,
    processing_class=tokenizer,
    reward_funcs=[correctness_reward_func, format_reward_func],
    args=training_args,
    train_dataset=dataset,
)

# 별도의 Reward Model 가동 없이 Rule 기반 Verifier와 Group 생성만으로 파라미터 깎기 시작
trainer.train()
```

Llama-3-8B 기준 오프라인 파인튜닝과 온라인 강화학습으로 수학경시대회 셋을 장기 훈련했을때 지표 분석자료 예시다.

| 훈련 단계 / 알고리즘 | GSM8K 정답률 (Accuracy) | 평균 추론 토큰 길이 (Tokens) | 학습 중 필수 소요 모델 수 | GPU VRAM 소요량 (A100 기준) | 보상 해킹 발생 빈도 (Reward Hacking) |
|---------------------|------------------------:|-----------------------------:|--------------------------|----------------------------:|-------------------------------------|
| Base Model | 35.6% | 85 | - | - | - |
| SFT (Teacher 70B) | 62.1% | 180 | 1 (Actor) | 약 22 GB | 없음 |
| DPO (Preference) | 64.5% | 210 | 2 (Actor, Ref) | 약 38 GB | 낮음 |
| PPO (Online RL) | 74.2% | 340 | 4 (Actor, Ref, Critic, Rew) | 78 GB (OOM 아슬아슬) | 높음 (어투 우회) |
| GRPO + RLVR (오안) | 82.4% | 680 (CoT 활성화) | 2 (Actor, Ref) | 약 41 GB (메모리 혁신) | 0% (컴파일러/정답 고정) |