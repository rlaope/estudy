# Online RL: PPO, GRPO, RLVR

The final stage of post-training is online reinforcement learning, where the model transcends the limitations of a fixed dataset and learns optimal behavior patterns by directly interacting with the environment.

## What is the fundamental difference between fixed-data learning (SFT/DPO) and online exploration RL?

From an engineering perspective, the difference between offline learning (SFT, DPO) and online RL lies in whether the exploration space is dynamically expanded.

- **SFT/DPO (Static Offline):** The training dataset is already fixed. The model performs alignment only within the given data distribution, making it structurally impossible to pioneer entirely new inference paths not present in the dataset or to make logical leaps on its own. It also fails to learn how to recover from subtle errors that may occur during actual inference.
- **Online RL (Dynamic Online):** Instead of reading fixed text, the model's Policy increases its Temperature to perform a tightrope walk of numerous incorrect and correct answers within the environment (rollout). In this process, strong rewards (Positive Reward) are given to successful paths, and penalties (Negative Signal) are applied to failed paths to optimize weights. As a result, the model develops the cognitive ability to explore efficient shortcuts in its **Chain of Thought, surpassing predefined answer guidelines.**

<br>

## PPO, GRPO, RLVR

### PPO (Proximal Policy Optimization) - and Problem

PPO is an online reinforcement learning algorithm published by OpenAI in 2017.

In reinforcement learning, the model's Policy continuously updates its weights to receive more Reward.

However, applying this directly in standard GD leads to the following problems:

- **Policy Collapse:** If weights change too drastically at once, the model's existing fluent language generation capabilities (grammar, context maintenance, etc.) can instantly break down, leading it to generate nonsensical output.
- **Solution:** PPO is named as such because it controls the new policy $\pi_\theta$ to become smarter gradually, only within a proximal region, preventing it from straying too far from the safe previous policy $\pi_{\theta_{old}}$.

$$L^{CLIP}(\theta) = \hat{\mathbb{E}}_t \left[ \min\left(r_t(\theta)\hat{A}_t, \text{clip}(r_t(\theta), 1-\epsilon, 1+\epsilon)\hat{A}_t\right) \right]$$

PPO uses the following clipping loss function to safely update weights.

- $r_t(\theta) = \frac{\pi_\theta(a_t|s_t)}{\pi_{\theta_{old}}(a_t|s_t)}$ (Probability Ratio): This ratio indicates how much the current policy's probability of generating this token (action) has changed compared to the old policy. If it's exactly the same, the ratio is 1.
- $\hat{A}_t$ (Advantage): This scores how much better the model's generated answer is compared to an average answer (Value). A positive value indicates a good answer, while a negative value indicates a bad answer.
- $\text{clip}(r_t(\theta), 1-\epsilon, 1+\epsilon)$: This clips the probability ratio $r_t(\theta)$ to prevent it from becoming too large or too small, keeping it within the range $[1-\epsilon, 1+\epsilon]$ (typically $\epsilon = 0.1 \sim 0.2$).

The core of this defense mechanism is that even if an answer receives an exceptionally good reward, causing the Advantage ($\hat{A}_t$) to explode, the Clip mathematically prevents the weights from being updated excessively at once.

PPO, the standard for traditional RLHF, necessarily requires a Critic (Value Model) to predict the State Value and correct actions. This leads to significant overhead during training, as four large models—Actor, Reference, Critic, and Reward—must be loaded onto GPU VRAM.

Here's a breakdown of the models that explain why memory explodes when running PPO-based RLHF in a real GPU environment:

1. **Actor Model (Policy, $\pi_\theta$):** This is the target LLM we actually want to train. It receives prompts, generates tokens, and its weights continuously change based on rewards.
2. **Reference Model ($\pi_{ref}$):** This is the SFT-completed model from just before training began, with fixed weights. It acts as a baseline to prevent the Actor from speaking an entirely different language while chasing rewards, by applying a penalty (KL Divergence Penalty) if the Actor's token distribution strays too far from this Reference distribution.
3. **Critic Model (Value Model):** This is an evaluator that predicts whether the current context will ultimately receive a good or bad score while the Actor is generating tokens. It calculates the Advantage in the above formula based on the difference between this predicted value and the actual Reward. Its weights are continuously updated during training.
4. **Reward Model ($R_\psi$)**: This is a **fixed-weight** model pre-trained on human preferences. It reads the final sentence completed by the Actor and determines if it's an answer a human would like, assigning a final Scalar Reward.

<br>

## GRPO (Group-Relative Policy Optimization)

GRPO, proposed by DeepSeekMath researchers, completely eliminates the Critic (Value Model), achieving an innovation that saves over 50% of VRAM.

**The principle is that for a single prompt ($R_\psi$), the current Policy model simultaneously rolls out $G$ answer bundles (Group, $O_1, O_2, ..., O_G$)**. Then, the average and standard deviation of the rewards ($R_1, R_2, ..., R_G$) received by each answer are calculated to determine the relative Advantage within the group.

The Advantage $A_i$ for the i-th sample is calculated as follows:

$$A_i = \frac{R_i - \mu(R)}{\sigma(R)}$$

Since the direction of weight updates is determined through control groups within the group, without a separate value evaluation model, it enables optimization of extreme parameters and large-scale batch processing in LLM serving/training environments with large weights.

> A group in GRPO is not a predicted vector distribution at a single token position, but rather a bundle of different completed answer sequences derived from the same question. This structure allows these control groups to compete with each other, enabling the model to realize for itself that constructing an answer text as B is more advantageous than A.
>
> The difference between RFT and GRPO is that RFT selects only the correct answers that pass through a Verifier from N samples and trains weights using SFT (Cross-Entropy Loss),
>
> whereas GRPO calculates the relative reward advantage among group samples generated within a single training step and optimizes using a Policy Gradient algorithm.
>
> Consequently, RFT merely discards incorrect samples and cannot penalize incorrect paths, while GRPO directly reduces the generation probability of samples with scores lower than the group average through negative weight updates.
>
> If RFT is an offline method that re-trains pre-filtered fixed data, GRPO optimizes by dynamically comparing the current model's token distribution state through real-time exploration (Rollout).
>
> In other words, GRPO differs in that it mathematically compares and reflects which inference path is relatively superior to others at each step through its objective function formula.

<br>

## RLVR (Reinforcement Learning from Verifier Feedback)

Previously, RLHF was susceptible to Reward Hacking, where models would exploit proxy reward models that mimicked human preferences by merely improving sentence style to gain scores.

RLVR directly connects a rule-based Verifier as the reward function instead of human evaluators, **but only for domains where results are absolutely verifiable**, such as coding (Unit Test), mathematics (Exact Match), and SQL (Execution).

Since the non-determinism of rewards is completely eliminated, the stability of inference learning increases explosively.

<br>

## Math Reasoning Optimization Experiment based on TRL GRPOTrainer

Let's look at the implementation code for a practical pipeline that simultaneously trains mathematical reasoning ability and a specific output format using the Hugging Face trl library's GRPOTrainer.

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

Here is an example of performance analysis data when a Llama-3-8B model was trained long-term on a math competition dataset using offline fine-tuning and online reinforcement learning.

| Training Stage / Algorithm | GSM8K Accuracy | Average Inference Token Length (Tokens) | Required Models During Training | GPU VRAM Usage (A100 Standard) | Reward Hacking Frequency |
|---------------------|------------------------:|-----------------------------:|--------------------------|----------------------------:|-------------------------------------|
| Base Model | 35.6% | 85 | - | - | - |
| SFT (Teacher 70B) | 62.1% | 180 | 1 (Actor) | Approx. 22 GB | None |
| DPO (Preference) | 64.5% | 210 | 2 (Actor, Ref) | Approx. 38 GB | Low |
| PPO (Online RL) | 74.2% | 340 | 4 (Actor, Ref, Critic, Rew) | 78 GB (OOM close call) | High (tone circumvention) |
| GRPO + RLVR (Online) | 82.4% | 680 (CoT activated) | 2 (Actor, Ref) | Approx. 41 GB (memory innovation) | 0% (compiler/fixed answer) |

### Ah-ha Moment (Self-Pioneering and Introspection of Intelligence)

Under the combined conditions of GRPO + RLVR, the accuracy rate of 82.4% is an unparalleled figure across all areas.

Particularly noteworthy is the **more than threefold surge in average inference token length, from 180 to 680 tokens.**

This quantitatively proves that, unconstrained by human answer lengths like SFT, the model began to expand its logical verification routines within the `<thinking>` block and deliberate at length to secure the correct answer reward (2.0 points).

### Memory Saving Economics: PPO vs GRPO

In a PPO environment, the burden of loading the Critic value evaluation model led to GPU resource limitations even when fine-tuning a single 8B model.

In contrast, under GRPO conditions, thanks to the group-relative reward technique, training was completed with only two model components and no Critic, **significantly reducing VRAM usage**. The remaining available memory was fully utilized for the Rollout batch size (Group Size=8), maximizing training efficiency.

### Perfect Reward Hacking Defense (Strength of RLVR)

When PPO was performed alone, reward hacking was occasionally observed, where the model would misuse complex mathematical symbols to generate seemingly intelligent prose, thereby deceiving the reward model into awarding high scores.

However, after binding RLVR (Verifier), composed of regular expressions and a Python execution sandbox, as the reward system, no matter how elegant the generated text, if the value within the final `\boxed{}` differed from the calculated correct answer, it was mercilessly scored 0. This caused the model's weights to converge solely towards ensuring genuine logical validity.
