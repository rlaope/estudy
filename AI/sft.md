# SFT와 Policy Learning

Post-Traning의 SFT(Supervised Fine-Tuning), **Policy Leearning**에 대해서 알아보자.

아무리 똑똑하게 방대한 지식을 학습한 Pre-trained 모델이라도, 사용자의 의도에 맞춰 대화할 줄 모른다면 단순한 텍스트 자동 완성기에 불가하다.

이를 우리가 아는 AI 비서로 탈바꿈 시키는 과정이 SFT이다.

## SFT는 모델에게 새로운 추론 능력을 주는가, 아니면 기존 분포를 정렬하는가

LLM 엔지니어링을 시작할 때 가장 많이 하는 오해는 파인튜닝 SFT를 통해 모델에게 새로운 지식이나 강력한 추론 능력을 주입할 수 있다고 믿는다.

결론부터 말하면 **SFT는 모델에게 새로운 능력을 주는것이 아니라 이미 내재된 지식과 추론 능력을 인간의 지시를 따르는 형식으로 표면적 정렬 Superficial Alignment를 수행하는 과정이다.**

- **Pre-Training**: 세상의 모든 텍스트를 읽으며 지식, 문법, 논리적 추론 능력을 형성한다. 도서관의 모든 책을 읽은 학자같은 느낌의 비유
- **SFT (Supervised Fine-Tuining)**: 질문이 들어오면 이어서 글을 쓰는 대신에 답변을 하는 방식을 가르친다. 인간이 작성한 고품질의 Q&A 데이터셋을 통해 행동을 모방하게 만든다. (비유는 학자에게 인터뷰하는 방법과 예의를 가르친다)

유명한 Paper LIMA(Less Is More for Alignment)에서도 증명되었듯, 단 1,000개의 매우 고품질적인 데이터만으로도 SFT는 성공적으로 수행할 수 있다. 이는 SFT가 지식의 학습이 아닌 형식의 학습 Formatting 임을 시사한다.

<br>

## SFT 훈련의 4대 핵심 구조

SFT를 성공적으로 수행하기 위해 데이터와 모델 내부에서 처리되는 구조를 이해해야한다.

### Prompt and Response (데이터 구조)

데이터셋은 인간이 입력한 프롬프트(Instruction/Input)와 그에 대한 이상적인 응답 Response의 쌍으로 구성된다. 이 텍스트 쌍을 모델이 이해할 수 있도록 하나로 이어붙여야한다.

### Chat Template (대화 템플릿)

Base Model은 사용자와 비서의 개념이 없고 따라서 특수한 토큰 Special Tokens를 활용해 역할을 명시적으로 구분해주는 템플릿을 씌운다. Hugging face에서는 이를 `apply_chat_template` 함수로 관리한다.

```
<|im_start|>system
당신은 도움이 되는 AI 어시스턴트입니다.<|im_end|>
<|im_start|>user
대한민국의 수도는 어디인가요?<|im_end|>
<|im_start|>assistant
대한민국의 수도는 서울입니다.<|im_end|>
```

### Label Masking (정답 마스킹)

단순히 위 텍스트를 통째로 모델에 넣고 학습시키면 치명적인 문제가 발생한다.

모델이 사용자의 질문 부분까지 자기가 예측하도록 학습해버리기 때문이다.

우리는 모델이 어시스턴트 답변을 생성할 때만 평가해야한다. 이를 위해 PyTorch `CrossEntropyLoss`가 무시하도록 프롬프트 부분의 정답 라벨을 -100 (IGNORE_INDEX)로 덮어씌운다.

이를 Label Masking or Data Collator for Completion Only라고 부른다.

### Token-Level Loss 구조

SFT의 손실함수 Loss Fucntion은 다음 토큰을 예측하는 Cross Entropy이다. 

Label Masking이 적용된 손실 함수 수식은 다음과 같다.

$$L = -\frac{1}{M} \sum_{i \in \text{Assistant\_Tokens}} \log P(x_i | x_{<i}, \theta)$$

- $x_{<i}$: 이전까지의 모든 토큰 (Prompt 포함)
- $x_i$: 현재 예측해야 할 정답 토큰
- $M$: 전체 시퀀스 길이 $N$이 아닌, 마스킹되지 않은 Assistant 토큰만의 개수
- 즉, 프롬프트 부분에서는 Loss를 0으로 처리하여 가중치를 업데이트하지 않는다.

<br>

## Hugging Face TRL을 활용한 SFT 및 LoRA

대형모델 (8B 파라미터 이상)을 전체 파인튜닝 (Full Fine-Tuning)하려면 막대한 GPU 메모리가 필요하다.

현업에서는 PEFT(Parameter-Efficient Fine-Tuning) 기법인 LoRA (Low-Rank Adaptation)를 적용하며 모델 가중치의 극히 일부만 학습시켜 비용을 획기적으로 낮춘다.

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

작은 크기의 LLM을 1만개의 Instruction 데이터셋으로 SFT한후 동일한 프롬프트에 대한 결괏값 차이를 분석한 리포트다.

Test Prompt

```
양자 컴퓨터와 기존 컴퓨터의 가장 큰 차이점 3가지를 설명해줘
```

### Base Model (Pre-train only)의 출력 예시

```
양자 컴퓨터와 기존 컴퓨터의 가장 큰 차이점 3가지를 설명해 줘.
양자 컴퓨터 관련 기술의 발전은 최근 눈부시게 이루어지고 있다.
IBM은 최근 433 큐비트 양자 프로세서를 발표했다.
기존 컴퓨터는 0과 1의 비트를 사용하지만 양자 컴퓨터는...
(사용자의 질문에 답하지 않고, 이어서 블로그 글이나 위키백과 문서처럼 자연스럽게 텍스트를 이어 나가려고 시도함. 중간에 말이 끊기거나 반복됨.)
```

Base Model은 다음 단어를 확률적으로 예측할 뿐, 대화의 턴이나 질문에 대한 답변 형식은 알지 못한다.

입력된 질문을 문서의 첫 문장으로 인식하고 그 뒤에 올 법한 텍스트를 생성한다.

### SFT Model (Instruction-Tuned)의 출력 예시

```
양자 컴퓨터와 기존 컴퓨터의 가장 큰 차이점 3가지는 다음과 같습니다:

1. 데이터 처리 단위: 기존 컴퓨터는 0 또는 1의 상태만 가지는 '비트(Bit)'를 사용하지만, 양자 컴퓨터는 0과 1의 상태를 동시에 가질 수 있는 '큐비트(Qubit)'를 사용합니다.
2. 연산 방식: 기존 컴퓨터는 순차적으로 연산을 수행하지만, 양자 컴퓨터는 양자 중첩과 얽힘 현상을 이용해 방대한 양의 연산을 동시에 병렬적으로 처리합니다.
3. 해결 가능한 문제 영역: 양자 컴퓨터는 복잡한 분자 구조 시뮬레이션이나 암호 해독 등 기존 슈퍼컴퓨터로 수백 년이 걸릴 연산을 몇 초 만에 해결할 잠재력을 지닙니다.
```

SFT를 거친 모델은 Chat Template (`User`, `Assistant`)을 인지하며 질문에 대해 구조화된 리스트형태 답변을 제공한다.

이는 모델 내부에 있던 양자 역학 지식이 SFT라는 과정을 통해 사용자가 읽기 편한 포맷으로 성공적으로 정렬되었음을 보여준다.

