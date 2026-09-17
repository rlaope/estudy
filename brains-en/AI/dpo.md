# Preference Optimization: DPO Family

Elegantly dismantling complex reinforcement learning pipelines with a single mathematical proof,

let's explore DPO (Direct Preference Optimization), which has become the standard in the open-source community.

## Can a model move towards human preferences without a Reward Model?

Yes, it can.

Traditional RLHF (PPO) required training a separate scoring AI, a Reward Model, to learn human preferences.

However, DPO researchers proved, by reverse-engineering the mathematical equations of reinforcement learning, that the optimal reward function can be perfectly substituted by the probability distribution of the Policy itself (the LLM we intend to train).

<br>

## What problems led to the emergence of DPO? Limitations of PPO

To build an RLHF pipeline using the traditional PPO (Proximal Policy Optimization) method, one had to contend with the following technical debt (we'll explore PPO-based GRPO, RLVR, etc., next time):

- **Memory Explosion OOM:** Running PPO requires loading no less than four models (1. Actor Model, 2. Reference Model, 3. Reward Model, 4. Value Model) onto GPU memory simultaneously. For a 70B model, this entails unimaginable infrastructure costs.
- **Instability:** Due to the nature of reinforcement learning, Reward Hacking (where the model exploits loopholes to get high scores) frequently occurs, and training collapse is common due to extreme sensitivity to hyperparameters like learning rate and KL Penalty.

<br>

## The Principle of DPO - A Mathematical Bypass

To solve the above problems, DPO transformed the training pipeline into a **simple classification problem.**

1.  **Data Preparation (Preference Pair):** For a single prompt, prepare a pair of a good response (chosen) and a bad response (rejected).
2.  **Implicit Reward:** DPO instructs the model as follows: "Compared to your existing understanding in the Reference, increase the Log Probability of generating chosen tokens and decrease the probability of generating rejected tokens."
3.  **Resource Saving:** You can eliminate the Reward and Value models from PPO's four models, and only load two models (the Policy Model to be trained and the Reference Model to serve as a baseline) into memory (using LoRA makes it effectively equivalent to loading only one model).

<br>

## DPO Pipeline using TRL

Looking at the code, you can intuitively understand how similar DPO is to SFT and why engineers are so enthusiastic about it.

```py
import torch
from datasets import load_dataset
from transformers import AutoModelForCausalLM, AutoTokenizer
from trl import DPOTrainer, DPOConfig
from peft import LoraConfig

# 1. Load Model (Policy and Reference start from the same Base model)
model_id = "my-sft-8B-model" # Model for which SFT has been completed
tokenizer = AutoTokenizer.from_pretrained(model_id)

# DPO internally compares the original weights (Reference) with the weights to be updated (Policy).
model = AutoModelForCausalLM.from_pretrained(model_id, torch_dtype=torch.bfloat16)

# 2. Load Preference Dataset
# Structure: {"prompt": "...", "chosen": "good response", "rejected": "bad response"}
dataset = load_dataset("json", data_files="preference_data.jsonl", split="train")

# 3. LoRA Configuration (Memory optimization via PEFT)
peft_config = LoraConfig(
    r=16,
    target_modules=["q_proj", "v_proj", "k_proj", "o_proj"],
    task_type="CAUSAL_LM"
)

# 4. DPO Hyperparameter Configuration
dpo_config = DPOConfig(
    output_dir="./dpo_results",
    beta=0.1, # KL Penalty (Force to prevent drifting too far from the Reference model)
    learning_rate=5e-6, # Use a much smaller LR than SFT
    per_device_train_batch_size=2,
    gradient_accumulation_steps=8,
)

# 5. Run DPOTrainer (Direct tuning without a Reward Model!)
trainer = DPOTrainer(
    model,
    args=dpo_config,
    train_dataset=dataset,
    tokenizer=tokenizer,
    peft_config=peft_config
)

trainer.train()
```

### DPO Training Results and Chosen / Rejected Win-rate Analysis

The key indicator for determining if DPO training is progressing correctly is not the typical Loss value, but rather the difference in log probabilities.

-   **Reward Margin (Evaluation Metric):** As training progresses, the implicit reward (probability) the model assigns to chosen responses should trend upwards, and the probability assigned to rejected responses should trend downwards.
-   **Win-Rate (Final Output):** When comparing and evaluating the SFT model before DPO application and the DPO model after application using LLM-as-a-Judge, the following results are obtained.

| Evaluation Item (1,000 Test Set) | SFT Model Win Rate | DPO Model Win Rate | Draw |
| :------------------------------- | -----------------: | -----------------: | ---: |
| Instruction Following            | 12%                | 85%                | 3%   |
| Rejected Responses (Safety)      | 4%                 | 96%                | 0%   |
| Response Formatting and Readability | 22%                | 70%                | 8%   |

In conclusion, without the massive resources required to build a separate Reward Model, we succeeded in perfectly aligning the model's behavior with human preferences using only high-quality chosen and rejected data.

## PEFT (Parameter-Efficient Fine-Tuning)

PEFT is a collective term for fine-tuning methodologies that **efficiently select and train only a small subset of parameters** without training all of them.

Traditional Full-Fine Tuning: If you train a Llama-3-8B model, you must update all 8 billion weights (Parameters).

At this time, not only the weights but also the optimizer states (Optimizer States) and gradients (Gradients) must be loaded into memory, requiring massive VRAM, several times the size of the model.

**PEFT methods** freeze the weights of the existing model and add only tiny adapter weights, typically 0.1-1% of the total parameters, training only those parts. As a result, GPU memory requirements are dramatically reduced, making it possible to train large models even on a single GPU.

### LoRA (Low-Rank Adapation)

LoRA is the most widely used specific algorithm among PEFT methodologies, utilizing the low-rank decomposition property of matrices.

-   **How it works**: The original model's massive weight matrix $W_0$ (e.g., $4096 \times 4096$ dimensions) is kept frozen. Instead, two very thin weight matrices, A and B, are attached next to it.
-   Instead of training the massive matrix, the change in weights ($\Delta W$) is represented through the product of two small matrices, $B \times A$.
-   **Merge**: Once training is complete, you simply mathematically add $\Delta W$ to the original weights $W_0$. $W = W_0 + \Delta W$. The advantage is that no additional computational delay occurs during inference.

<br>

## Can we also teach models how to think?

DPO demonstrates overwhelming performance in style and value alignment, such as generating polite language and avoiding harmful responses.

But what about domains where clear mathematical truths exist, like 1+1=2?

The concept of 'preference' is difficult to apply to solving mathematical problems. It's not about choosing a preferred solution between A and B; there are simply wrong solutions and correct solutions.

In other words, **there is a fundamental limitation in improving reasoning capabilities with DPO.**

While Llama-3 remained stagnant, how did OpenAI's o1 model and DeepSeek-R1 break through this limitation?

The answer lies not in comparative pairwise methods like DPO, but in reasoning-specific reinforcement learning approaches such as GRPO & RL for Reasoning, where the model itself develops thousands of chains of thought and explores rewards based on rule-based rewards.

Let's explore them next time.
