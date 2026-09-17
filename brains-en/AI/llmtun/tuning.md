# Differences Between Continual Pretraining and Fine-tuning

When optimizing LLMs for specific domains (e.g., medical, legal, financial) or transforming them into proprietary enterprise solutions, architectural engineering that determines at which stage and for what mathematical purpose to update the model's layer weights is a critical juncture that dictates infrastructure resource efficiency.

The post-training chain is divided into Continual Pretraining (CPT) for embedding unknown corpora, Supervised Fine-Tuning (SFT) for correcting instruction grammar and conversational style, and Preference Alignment for designing the final reward landscape. Each technique has fundamental differences in the update region of the activated weight space and the mechanism for controlling the predictive entropy of tokens.

## Is it a Lack of Knowledge, a Style Issue, or a Preference Alignment Problem?

The essence of this question is, when a model's performance defect occurs, is it **a phenomenon caused by the absence of internal parameter information (Parameter Knowledge) within the weights, an encoding/style issue that failed to trigger the desired output interface specification, or a reward distortion problem that cannot push away abnormal outputs disliked by humans?**

In other words, it's about establishing a baseline for the infrastructure pipeline to diagnose which of CPT, SFT, or PA is the bottleneck.

Confusing these can lead to a surge in hallucinations where a model without information fabricates lies if only simple SFT style tuning is applied. Conversely, if CPT is re-executed with tens of gigabytes of data on a model that only needs a change in tone, catastrophic forgetting, which collapses its existing general computational capabilities, can occur. Therefore, precise diagnosis to geometrically identify the root cause of the defect is essential.

## Three Major Alignment Processes Based on Weight Update Objectives

### Continual Pretraining (CPT)

**Basic Mechanism:** This process involves injecting a large-scale raw text corpus from a specific domain while maintaining the unsupervised next-token prediction objective function of the pretraining phase, thereby compressing and accumulating a new knowledge base (Parameter Storage) within the model parameters themselves.

**Reason for Introduction:** This is to physically embed the unique domain-specific terminology structure, source code grammar, and statistical transition probability density landscape between special domain nouns into the model weights, which cannot be overcome by SFT data binding or RAG frameworks alone. It is the only post-training technique that increases the sheer volume of knowledge.

If we compare it to a human, it's like having them read books or dictionaries, adding to their foundational knowledge.

### Instruction Fine-Tuning (SFT)

**Basic Mechanism:** This process combines a structured conversational dataset in the `[Prompt - Response]` format with prompt masking guards, applied to a weight state that has already acquired the necessary knowledge, to forcibly align the model's attention heads to instruction execution and conversational format specifications (Behavioral/Formatting Alignment).

**Reason for Introduction:** A model that has completed CPT still retains its tendency to continue raw documents. Therefore, SFT is used to align it into an API-like form that can control this tendency, triggering tone, adherence to constraints, and task completeness. It is a style correction technique, not for injecting knowledge.

It's like teaching a new employee knowledge through CPT, and then using SFT to train them on engineering equipment usage guides and operational instructions to ensure they can perform tasks.

### Preference Alignment (DPO/GRPO)

**Basic Mechanism:** This is a policy gradient variant process that designs a subtle reward landscape within the generation distribution of an SFT-completed model, increasing the density of paths preferred by humans (Chosen) and pushing away the density of disliked paths (Rejected).

**Reason for Introduction:** Since SFT only increases the log-likelihood of correct answers, it cannot actively generate repulsive forces to push away incorrect answers or security vulnerabilities that need to be bypassed. It is introduced to establish safety guardrails and align with human preferences.

It's like a new employee following work instructions, but also being taught "pro tips" to avoid unnecessary repetition, or being corrected and informed when they make a mistake during task execution.

## Contrast Between Block-Packed Datasets for CPT (Continual Pretraining) and SFT Data Processing Pipelines

The code below is an engineering source code that separately implements a CPT data collator (which splits the entire text into block sizes and trains without masking) for knowledge injection and an SFT data collator (which erases the prompt area) for style alignment, both within the same raw data environment, from a hardware pipeline perspective.

```py 
import torch
from datasets import Dataset
from transformers import AutoTokenizer, DataCollatorForLanguageModeling

# 1. 환경 및 인프라 토크나이저 초기화 (Qwen 아키텍처 기준)
MODEL_ID = "Qwen/Qwen2-7B"
tokenizer = AutoTokenizer.from_pretrained(MODEL_ID)
if tokenizer.pad_token is None:
    tokenizer.pad_token = tokenizer.eos_token

# 2. 로우 레벨 원시 도메인 데이터 세트 예시 (금융 로그 및 지시어 데이터 복합)
raw_cpt_corpus = [
    {"text": "KOSPI200 지수 옵션 거래 대금 정산 매커니즘은 일일 정산 방식을 채택하며 마진 콜 발동 시 자청산 절차로 이행된다."},
    {"text": "바젤III 규제 가이드라인에 따른 위험가중자산(RWA) 측정 방식은 내부등급법(IRB)과 표준방법으로 이원화된다."}
]
raw_sft_corpus = [
    {"prompt": "바젤III에서 RWA가 뭐야?", "response": "위험가중자산(Risk-Weighted Assets)을 뜻하며 자산의 위험도에 따라 가중치를 부여한 수치입니다."}
]

# ==========================================
# [PIPE 1] Continual Pretraining (CPT) 파이프라인
# ==========================================
def preprocess_cpt(examples):
    # CPT는 문맥의 구분이 없으므로 전체 텍스트를 통째로 인코딩한 뒤 고정된 Block Size로 슬라이싱 및 패킹합니다.
    return tokenizer(examples["text"], truncation=False, add_special_tokens=True)

cpt_dataset = Dataset.from_list(raw_cpt_corpus).map(preprocess_cpt, batched=True, remove_columns=["text"])

# CPT용 콜레이터: mlm=False 지정을 통해 전형적인 Causal Next-token Prediction labels 자동 생성 (마스킹 없음)
cpt_collator = DataCollatorForLanguageModeling(tokenizer=tokenizer, mlm=False)

# ==========================================
# [PIPE 2] Instruction Fine-Tuning (SFT) 파이프라인
# ==========================================
def preprocess_sft(example):
    # SFT는 프롬프트와 답변이 명확히 분리되며, 프롬프트 영역은 -100으로 손실 계산에서 소거되어야 합니다.
    prompt_ids = tokenizer.encode(f"User: {example['prompt']}\nAssistant: ", add_special_tokens=False)
    response_ids = tokenizer.encode(example["response"], add_special_tokens=False) + [tokenizer.eos_token_id]
    
    input_ids = prompt_ids + response_ids
    # Prompt 구간 길이만큼 -100을 채워 그라디언트 전이 차단 가드 설정
    labels = [-100] * len(prompt_ids) + response_ids
    attention_mask = [1] * len(input_ids)
    
    return {"input_ids": input_ids, "labels": labels, "attention_mask": attention_mask}

sft_dataset = Dataset.from_list(raw_sft_corpus).map(preprocess_sft, remove_columns=["prompt", "response"])

# 하드웨어 배치 적재 파워 검증 출력
if __name__ == "__main__":
    cpt_batch = cpt_collator([cpt_dataset[0], cpt_dataset[1]])
    
    print("=== [인프라 프로파일러] CPT 텐서 레이아웃 ===")
    print(f"input_ids 구조: {cpt_batch['input_ids'].shape}")
    print(f"labels 구조   : {cpt_batch['labels'].shape}")
    print(f"CPT 첫 번째 토큰의 Label 일치 여부: {cpt_batch['input_ids'][0][0] == cpt_batch['labels'][0][0]}") # True (전체 학습)

    print("\n=== [인프라 프로파일러] SFT 텐서 레이아웃 ===")
    sft_sample = sft_dataset[0]
    print(f"SFT 전이 input_ids 길이: {len(sft_sample['input_ids'])}")
    print(f"SFT 전이 labels 내부의 마스킹 개수 (-100): {sft_sample['labels'].count(-100)}")
    print(f"SFT 실제 Loss 연산 대상 토큰 개수: {len([l for l in sft_sample['labels'] if l != -100])}")
```

### Comparison of Actual Model Operation Trace Logs

- **State 1: When injecting a specialized financial domain question into a general-purpose Base model**

```
[User]: 바젤III 규제 환경에서 RWA를 산출할 때 내부등급법(IRB)과 표준방법의 핵심 차이를 설명하라.
[Base Model]: 바젤III 규제 환경에서 RWA를 산출할 때 내부등급법(IRB)과 표준방법의 핵심 차이를 설명하라. 그리고 그것이 은행 자본 확충에 미치는 영향에 대한 논문을 다운로드 하려면 아래 링크를 클릭... (문서 이어쓰기 폭주)
```

- **State 2: When only CPT (Continual Pretraining) is performed on the State 1 model, without instruction alignment**

```
[User]: 바젤III 규제 환경에서 RWA를 산출할 때 내부등급법(IRB)과 표준방법의 핵심 차이를 설명하라.
[CPT Model]: 내부등급법(IRB)은 은행이 자체 개발한 신용평가 모형(PD, LGD, EAD)을 활용하여 위험가중자산을 산출하는 반면 표준방법은 금융감독당국이 지정한 고정 위험가중치를 일괄 적용한다. 한편, 2026년도 자산 건전성 분류 기준에 따르면... (금융 지식은 정확하나 끊기지 않고 무한 책 서술 모드 가동)
```

- **State 3: When SFT (conversational alignment) and preference optimization (DPO) are completed on the State 2 model**

```
[User]: 바젤III 규제 환경에서 RWA를 산출할 때 내부등급법(IRB)과 표준방법의 핵심 차이를 설명하라.
[Aligned Model]: 바젤III 기준 RWA(위험가중자산) 산출 방식의 핵심 차이는 '자체 추정치 사용 여부'에 있습니다.
1. 내부등급법(IRB): 은행이 자체 보유한 내부 데이터로 부도확률(PD), 부도시손실률(LGD)을 직접 추정하여 유연하게 반영합니다.
2. 표준방법: 감독기관이 규정한 고정 위험가중치를 일괄 적용하므로 보수적입니다.
요청하신 차이점 정리를 마칩니다.<|im_end|>
```

#### Results by State

By interpreting the logs, we can find **mathematical proof of knowledge storage mechanisms and style separation**:

State 1 Base exhibits low transfer probability density for financial domain knowledge, leading to nonsensical responses or copying the question.

State 2 CPT accurately retrieves financial knowledge, indicating that knowledge assets have been embedded within the parameters through large-scale raw corpus training.

However, a limitation is observed where it fails to emit the stop token `<\im_end\>` and continues to write related financial whitepapers.

Finally, upon reaching State 3, the SFT Behavioral guard activates, normalizing structural summarization and session return.

#### Infrastructure Resource Consumption and Bottleneck Control

CPT is a **compute-bound** process that requires pushing large-scale raw data in millions of window sizes, necessitating the intervention of large-scale distributed environment acceleration frameworks (**Megatron-LM, DeepSpeed ZeRO-3**) beyond single-node VRAM.

In contrast, SFT or Preference Alignment typically involve data volumes in megabytes and focus solely on format optimization, making them memory and I/O bound processes that can be handled even in large clusters.

### Architectural Comparison Metrics by Post-Training Methodology

A comparative table of quantitative technical indicator attributes for allocating infrastructure budgets differently based on objectives in the modern LLM refinement chain.

| Comparative Architectural Metric | Stage 1: Continual Pretraining (CPT) | Stage 2: Instruction Fine-Tuning (SFT) | Stage 3: Preference Alignment (DPO/GRPO) |
|----------------------------------|---------------------------------------|-----------------------------------------|------------------------------------------|
| Main Mechanism & Purpose         | Injecting and physically imprinting unknown domain knowledge (Parametric Knowledge) within weights. | Correcting interface behavior to parse input commands and respond in desired formats and styles. | Reinforcing human-preferred paths among multiple output candidates and suppressing jailbreaking and incorrect paths. |
| Dataset Specification & Processing Form | Pure Text sequence without format (`{"text": "..."}`).<br>Concatenated tightly up to maximum context without `[PAD]`. | Structured instruction pairs (`{"prompt": "...", "response": "..."}`).<br>Prompt area must apply `-100` Loss masking. | Preference contrast pairs (`{"prompt": "...", "chosen": "...", "rejected": "..."}`).<br>Or real-time group reward scoring. |
| Loss Function (Objective)        | Cross-Entropy (Vanilla Next-token Prediction) | Cross-Entropy (Completion-only Masked Likelihood) | Implicit Reward Log-odds (DPO) or Policy Gradient (GRPO) |
| VRAM & Acceleration Infra Stack  | Extremely High (Megatron-LM pipeline parallelism, DeepSpeed ZeRO-3, multi-node tensor slicing essential) | Moderate (TRL SFTTrainer, single-node operation possible, LoRA/QLoRA combined for hardware efficiency) | Moderate~High (For Online RL, real-time vLLM distributed engine combination needed for large-scale rollout generation) |
| Crash Phenomena if Misdesigned   | Catastrophic Forgetting: Acquires domain knowledge but loses the language model's inherent general conversational and reasoning abilities. | Superficial Alignment: Hallucinations are maximized as the model learns a plausible tone without internal knowledge. | Reward Hacking: Infinitely outputs tokens that are superficially flashy but lack substance or are bizarre, to trick the reward score. |
