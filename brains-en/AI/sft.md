# SFT and Policy Learning

Let's learn about SFT (Supervised Fine-Tuning) and **Policy Learning** in Post-Training.

No matter how intelligently a pre-trained model has learned vast knowledge, if it cannot converse according to the user's intent, it is merely a simple text auto-completer.

The process of transforming it into the AI assistant we know is SFT.

## Does SFT give models new reasoning abilities, or does it align existing distributions?

A common misconception when starting LLM engineering is believing that fine-tuning SFT can inject new knowledge or powerful reasoning abilities into the model.

To put it simply, **SFT does not give the model new abilities; rather, it performs Superficial Alignment, aligning the already inherent knowledge and reasoning abilities into a format that follows human instructions.**

- **Pre-Training**: Forms knowledge, grammar, and logical reasoning abilities by reading all the text in the world. An analogy would be a scholar who has read every book in a library.
- **SFT (Supervised Fine-Tuning)**: Teaches the model to answer questions instead of continuing to write text when a question is posed. It makes the model imitate behavior through high-quality Q&A datasets written by humans. (The analogy is teaching a scholar how to interview and proper etiquette.)

As proven in the famous paper LIMA (Less Is More for Alignment), SFT can be successfully performed with just 1,000 pieces of very high-quality data. This suggests that SFT is about learning formatting, not learning knowledge.

<br>

## Four Core Structures of SFT Training

To successfully perform SFT, one must understand the data and the internal processing structures of the model.

### Prompt and Response (Data Structure)

The dataset consists of pairs of human-entered prompts (Instruction/Input) and their ideal responses. These text pairs must be concatenated into a single sequence for the model to understand.

### Chat Template

Base Models do not have the concept of a user and an assistant, so a template is applied that explicitly distinguishes roles using special tokens. Hugging Face manages this with the `apply_chat_template` function.

```
<|im_start|>system
당신은 도움이 되는 AI 어시스턴트입니다.<|im_end|>
<|im_start|>user
대한민국의 수도는 어디인가요?<|im_end|>
<|im_start|>assistant
대한민국의 수도는 서울입니다.<|im_end|>
```

### Label Masking

Simply feeding the entire text above into the model for training leads to a critical problem.

This is because the model learns to predict even the user's question part.

We only need to evaluate the model when it generates the assistant's response. To achieve this, we overwrite the ground truth labels for the prompt portion with -100 (IGNORE_INDEX) so that PyTorch's `CrossEntropyLoss` ignores them.

This is called Label Masking or Data Collator for Completion Only.

### Token-Level Loss Structure

The loss function for SFT is Cross Entropy, which predicts the next token.

The loss function formula with Label Masking applied is as follows:

$$L = -\frac{1}{M} \sum_{i \in \text{Assistant\_Tokens}} \log P(x_i | x_{<i}, \theta)$$

- $x_{<i}$: All previous tokens (including Prompt)
- $x_i$: The ground truth token to be predicted currently
- $M$: The number of unmasked Assistant tokens, not the total sequence length $N$
- In other words, the loss for the prompt portion is treated as 0, and weights are not updated.

<br>

## SFT and LoRA using Hugging Face TRL

Full Fine-Tuning of large models (8B parameters or more) requires enormous GPU memory.

In practice, PEFT (Parameter-Efficient Fine-Tuning) techniques like LoRA (Low-Rank Adaptation) are applied to train only a tiny fraction of the model's weights, significantly reducing costs.

```py
import torch
from datasets import load_dataset
from transformers import AutoModelForCausalLM, AutoTokenizer, TrainingArguments
from peft import LoraConfig, get_peft_model
from trl import SFTTrainer, DataCollatorForCompletionOnlyLM

# 1. Base Model 및 Tokenizer 로드
model_id = "meta-llama/Meta-Llama-3-8B"
tokenizer = AutoTokenizer.from_pretrained(model_id)
tokenizer.pad_token = tokenizer.eos_token # 패딩 토큰 설정

model = AutoModelForCausalLM.from_pretrained(
    model_id,
    torch_dtype=torch.bfloat16,
    device_map="auto"
)

# 2. LoRA 설정 (PEFT)
# 모델의 가중치를 고정하고, 훈련 가능한 작은 행렬(Rank=16)을 Attention 레이어에 삽입
lora_config = LoraConfig(
    r=16,
    lora_alpha=32,
    target_modules=["q_proj", "v_proj", "k_proj", "o_proj"],
    lora_dropout=0.05,
    bias="none",
    task_type="CAUSAL_LM"
)
model = get_peft_model(model, lora_config)

# 3. 데이터셋 로드 및 Chat Template 포맷팅
dataset = load_dataset("json", data_files="instruction_dataset.json", split="train")

def formatting_prompts_func(example):
    # System, User, Assistant 구조로 텍스트 결합
    text = f"<|system|>\nYou are a helpful assistant.<|end_of_text|>\n<|user|>\n{example['instruction']}<|end_of_text|>\n<|assistant|>\n{example['output']}<|end_of_text|>"
    return text

# 4. Label Masking 설정 (Data Collator)
# <|assistant|> 이후의 텍스트에 대해서만 Loss를 계산하도록 설정
response_template = "<|assistant|>\n"
collator = DataCollatorForCompletionOnlyLM(response_template=response_template, tokenizer=tokenizer)

# 5. SFTTrainer 설정 및 학습 시작
training_args = TrainingArguments(
    output_dir="./sft_results",
    per_device_train_batch_size=4,
    gradient_accumulation_steps=4,
    learning_rate=2e-4,
    num_train_epochs=3,
    logging_steps=10,
    bf16=True,
    optim="paged_adamw_8bit" # 메모리 최적화 옵티마이저
)

trainer = SFTTrainer(
    model=model,
    train_dataset=dataset,
    args=training_args,
    formatting_func=formatting_prompts_func,
    data_collator=collator,
    max_seq_length=1024,
)

trainer.train()
```

This is a report analyzing the difference in output values for the same prompt after SFT on a small-sized LLM with a 10,000-instruction dataset.

Test Prompt

```
양자 컴퓨터와 기존 컴퓨터의 가장 큰 차이점 3가지를 설명해줘
```

### Example Output of Base Model (Pre-train only)

```
양자 컴퓨터와 기존 컴퓨터의 가장 큰 차이점 3가지를 설명해 줘.
양자 컴퓨터 관련 기술의 발전은 최근 눈부시게 이루어지고 있다.
IBM은 최근 433 큐비트 양자 프로세서를 발표했다.
기존 컴퓨터는 0과 1의 비트를 사용하지만 양자 컴퓨터는...
(사용자의 질문에 답하지 않고, 이어서 블로그 글이나 위키백과 문서처럼 자연스럽게 텍스트를 이어 나가려고 시도함. 중간에 말이 끊기거나 반복됨.)
```

The Base Model only probabilistically predicts the next word; it does not understand conversational turns or the format for answering questions.

It perceives the input question as the first sentence of a document and generates text that would likely follow.

### Example Output of SFT Model (Instruction-Tuned)

```
양자 컴퓨터와 기존 컴퓨터의 가장 큰 차이점 3가지는 다음과 같습니다:

1. 데이터 처리 단위: 기존 컴퓨터는 0 또는 1의 상태만 가지는 '비트(Bit)'를 사용하지만, 양자 컴퓨터는 0과 1의 상태를 동시에 가질 수 있는 '큐비트(Qubit)'를 사용합니다.
2. 연산 방식: 기존 컴퓨터는 순차적으로 연산을 수행하지만, 양자 컴퓨터는 양자 중첩과 얽힘 현상을 이용해 방대한 양의 연산을 동시에 병렬적으로 처리합니다.
3. 해결 가능한 문제 영역: 양자 컴퓨터는 복잡한 분자 구조 시뮬레이션이나 암호 해독 등 기존 슈퍼컴퓨터로 수백 년이 걸릴 연산을 몇 초 만에 해결할 잠재력을 지닙니다.
```

The SFT-trained model recognizes the Chat Template (`User`, `Assistant`) and provides structured, list-formatted answers to questions.

This demonstrates that the quantum mechanics knowledge inherent in the model has been successfully aligned into a user-friendly format through the SFT process.
