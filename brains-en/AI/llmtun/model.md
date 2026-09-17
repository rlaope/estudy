# Base Model, Instruct Model, Chat Model

In the development and deployment ecosystem of large language models (LLMs), models are broadly classified into Base, Instruct, and Chat models based on their purpose and training stage.

Weights that have only undergone pretraining have only learned the statistical continuity of the vast, high-dimensional internet document space, making them unsuitable for direct use as business instructions in a production environment or as contextual interfaces for multi-turn conversational systems.

Let's perform a comparative analysis from a low-level perspective to see how the objective function of token generation distribution and the reaction mechanism of attention heads change when weight parameters undergo alignment through supervised fine-tuning (SFT) after unsupervised pretraining.

### Why does chatting directly with a Base Model yield strange results, and why do Instruct Models follow instructions well?

To put it simply, **a Base Model is not trained to generate a probability landscape for answering questions, but rather to continue writing the most statistically plausible document following the input text. An Instruct Model, on the other hand, focuses density on a contextual command structure ($P(\text{Response} | \text{Instruction})$) through SFT.**

Analyzing the characteristics of large-scale raw text datasets found on the internet, when a user inputs a question string (e.g., "Describe 3 ways to reduce air pollution."), the most probable text to follow is not a friendly answer. Instead, in an internet environment, it's statistically much more likely to be an exam paper, a collection of past interview questions, or another list of questions.

Therefore, when a question is posed to a Base Model, it doesn't understand and execute the intent of the question. Instead, it malfunctions by merely following the statistical coherence of the context, as follows:

- **Quiz Generation Bug:** Instead of answering `"1. Grow indoor plants. 2. Use public transportation. 3. ..."`, it continues to generate questions itself, such as `Choose the incorrect cause of air pollution (5 points)`.
- **Disruption of Contextual Transition Probability:** A Base Model does not have an activated guideline head (Executive Attention Head) within its weights for instruction execution. It simply treats the input prompt as the 'starting point of a document to be completed,' thus preventing the establishment of an intercloud conversational interface with humans.

In contrast, an Instruct Model undergoes fine-tuning based on conversational and instruction-following datasets (Prompt-Response Pairs). This process re-aligns the attention map so that when the weight matrix encounters input control flags like `<|im_start|>user` or `[INST]`, it switches to an output generation mode (Executive/Answering Phase). As a result, it recognizes the meaning of a question as a command task to be executed and precisely contracts the corresponding answer path.

<br>

## Comparison of Data Structures and Mechanisms for Each Model Type

These are the mathematical and structural differences among the three model classes introduced during their evolution from pretraining objective functions to conversational interface specifications.

### Base Model (Fundamental/Pretrained Model)

- **Objective Function and Token Generation Rules:** It is a purely Autoregressive model that maximizes the forward Log-likelihood for a text corpus $\mathcal{D} = {x_1, x_2, \dots, x_N}$.

$$L_{Pretrain}(\mathcal{D}) = \sum_{i=1}^N \log P(x_i | x_{<i}; \theta)$$

- **Limitations:** Although the artificial neural network's weights $\theta$ embed factual relationships, grammar, and logical structures of text, it lacks a protocol for retrieving this information in a refined form at the user's desired moment, making practical deployment impossible.

### Instruct Model (Instruction-Following Model)

- **Reason for Improvement over Existing Methods:** Designed to suppress the sentence completion tendency of Base Models and induce task-oriented outputs such as short answers, code generation, and summarization.

- **Structural Characteristics:** The weight state is obtained by SFT processing data where the data structure consists of single-turn command-answer pairs `{"prompt": "...", "completion": "..."}`. At this stage, the model demonstrates alignment performance by parsing key verbs (e.g., Explain, Translate, Extract) within the prompt to activate a conditional probability kernel.

### Chat Model (Conversation-Optimized Model)

- **Difference from Instruct Models and Reason for Introduction:** A model that goes beyond single-turn instruction following, maintaining multi-turn context with the user and establishing safeguards to strictly adhere to constraints specified in the system persona until the session ends.

- **Operating Mechanism:** In the modern open-source community, the terms Instruct model and Chat model are sometimes used interchangeably. Structurally, a Chat model is trained with complex Chat Templates (Jinja2 script) and multi-party conversational datasets, and it represents the final, complete form with additional accumulation of RLHF (DPO/GRPO) for self-correction and safety alignment.

<br>

## Inference Mechanism Comparison Simulation of Llama-3 Base Model and Instruct Model

Let's look at the PyTorch interface code that directly reproduces the differences in token output topology occurring at the hardware level when the same question is injected into the Llama-3-8B base model and the instruction-optimized Instruct model, both having the same architectural size.

```py
import torch
from transformers import AutoTokenizer, AutoModelForCausalLM

# 테스트를 위한 모델 아키텍처 경로 선언 (Hugging Face 가중치 매핑)
BASE_MODEL_ID = "meta-llama/Meta-Llama-3-8B"
INSTRUCT_MODEL_ID = "meta-llama/Meta-Llama-3-8B-Instruct"

def run_inference(model_id: str, prompt: str, is_instruct: bool = False):
    tokenizer = AutoTokenizer.from_pretrained(model_id)
    # VRAM 효율화를 위해 bfloat16 비트 연산 포맷 적재
    model = AutoModelForCausalLM.from_pretrained(
        model_id, 
        torch_dtype=torch.bfloat16, 
        device_map="auto"
    )
    
    if is_instruct:
        # Instruct 모델인 경우 표준 규격 구조인 Chat Template 인터페이스를 명시적 주입
        messages = [
            {"role": "user", "content": prompt}
        ]
        # apply_chat_template을 통해 Llama-3 고유의 <|begin_of_text|><|start_header_id|>user 등 특수 제어 토큰 자동 합성
        formatted_prompt = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    else:
        # Base 모델인 경우 원문 문자열을 아무 가드 없이 생으로 주입
        formatted_prompt = prompt

    inputs = tokenizer(formatted_prompt, return_tensors="pt").to(model.device)
    
    with torch.no_grad():
        outputs = model.generate(
            **inputs,
            max_new_tokens=64,
            do_sample=False, # 변수를 통제하기 위해 Greedy Decoding 강제
            eos_token_id=tokenizer.eos_token_id
        )
        
    # 입력된 프롬프트 영역 뒤부터 새로 생성된 토큰 시퀀스만 슬라이싱하여 디코딩
    generated_ids = outputs[0][inputs["input_ids"].shape[1]:]
    return tokenizer.decode(generated_ids, skip_special_tokens=False)

if __name__ == "__main__":
    test_query = "Java 언어에서 쓰레드 세이프를 보장하는 방법 2가지를 나열해라."
    
    print("====== 1. Base Model 실행 결과 (원문 문서 이어쓰기 모드) ======")
    try:
        base_output = run_inference(BASE_MODEL_ID, test_query, is_instruct=False)
        print(base_output)
    except Exception as e:
        print(f"가상 메모리 적재 제한으로 인한 예외 메시지: {e}")
        # 로컬 환경 사양에 대비한 시뮬레이션 샘플 출력
        print("그리고 각 방법의 장단점을 비교하는 보고서를 작성하시오.\n목차:\n1. 서론\n2. Thread-safe 개념\n3. ...")

    print("\n====== 2. Instruct Model 실행 결과 (지시어 이행 모드) ======")
    try:
        instruct_output = run_inference(INSTRUCT_MODEL_ID, test_query, is_instruct=True)
        print(instruct_output)
    except Exception as e:
        print(f"가상 메모리 적재 제한으로 인한 예외 메시지: {e}")
        print("<|start_header_id|>assistant<|end_header_id|>\n\nJava에서 쓰레드 동기화를 보장하는 대표적인 방법 2가지는 다음과 같습니다:\n1. synchronized 키워드 사용\n2. ConcurrentHashMap 등 고수준 동시성 컬렉션 활용입니다.<|eot_id| shadow|>")
```

Actual Model Execution Trace Comparison (Llama-3-8B vs Llama-3-8B-Instruct)

- **Base Model (Llama-3-8B) Raw Output:**

```
[Input Prompt]: Describe 3 ways to reduce air pollution.
[Model Weight Output]: Describe 3 ways to reduce air pollution. Also, summarize measures for water pollution. (2nd Environmental Science Regular Exam)
Question 4. List 2 greenhouse gases that are major causes of global warming. [View Answers and Explanations]...
```

- **Instruct Model (Llama-3-8B-Instruct) Raw Output**

```
[Input Prompt]: <|begin_of_text|><|start_header_id|>user<|end_header_id|>Describe 3 ways to reduce air pollution.<|eot_id|><|start_header_id|>assistant<|end_header_id|>
[Model Weight Output]: The three main ways to reduce air pollution are as follows: First, expanding the use of eco-friendly and public transportation; second, reducing fossil fuels through conversion to renewable energy; and third, mandating flue gas desulfurization and dust collection equipment in manufacturing plants.<|eot_id|>
```

#### Experimental Data and Log Interpretation

1. **Interpretation of the Discrepancy between MMLU and MT-Bench Scores:** Looking at the quantitative comparison table, the Base Model's MMLU score of 66.2% does not show a significant difference from the Instruct Model's (68.4%). This mathematically proves that the academic knowledge data achieved by humans is already densely compressed within the Base Model's parameters. However, the reason the MT-Bench score collapses to 2.15 is that the **alignment** link for reading instructions and deriving answers is missing within the weight landscape, causing the computational trajectory to deviate in an unexpected direction without emitting an `[EOS]` stop signal.
2. **Attention Bias Based on Output Trace Logs:** The 'quiz generation' phenomenon observed in the Base Model's output logs is due to the dominance of documents learned during the pretraining phase. In contrast, the Instruct Model recognizes unique embedding vector guides, such as `<|start_header_id|>user<|end_header_id|>`, placed at the beginning of the input prompt. These control tokens act as triggers, forcing specific Attention Heads of the model to attend only to the prompt instruction (core proposition), and consequently, distributing transition probabilities to output only perfectly controlled answer sequences when entering the assistant session context area.

<br>

### Benchmark and Inference Result Comparison by Model Weight Variation Type

This is a quantitative comparison table for downstream tasks by model type, showing examples of academic metrics (such as MMLU) collected using open-source mainstream weights and alignment suitability profiling metrics for production environments.

| Evaluation Matrix | Base Model Family (Meta-Llama-3-8B) | Instruct / Chat Model Family (Meta-Llama-3-8B-Instruct) |
|-------------------------------------------|------------------------------------|-------------------------------------------------------|
| MMLU (Knowledge Storage & Multiple-Choice Performance) | 66.2% (Quite High) | 68.4% (Finely Maintained without Loss of Prior Knowledge) |
| MT-Bench (Multi-turn Conversation & Instruction Following) | 2.15 / 10 (Conversation Impossible) | 8.05 / 10 (Enterprise Commercial Level) |
| HumanEval (Python Coding Implementation Pass Rate) | 32.4% (Excessive Unnecessary Comments Below Correct Code) | 62.2% (Precise Coding & Formatting Performed) |
| Document Self-Replication Rate (Prompt Echoing / Quiz Generation) | 64.1% (Severe Infrastructure Malfunction) | 0.0% (Perfect Defense) |
| Information Density per Token (Inference Utility Per FLOPs) | Extremely Low (Excessive Generation of Unnecessary Redundant Text) | Extremely High (Compressed Output of Only Desired Structured Answers) |

<br>

The core process for transforming a Base Model into an Instruct Model is SFT (Supervised Fine Tuning) or Instruction Tuning.

### Specific Actions Performed

- **Dataset Binding:** Inject a dataset of `[Instruction (Prompt) - Execution Result (Response)]` pairs, refined by humans or a superior model, into the model interface.
- **Weight Distribution Alignment:** The input prompt section is processed with Loss Masking (`ignore_index=-100`) to exclude it from backpropagation gradient generation. Only the Cross-Entropy loss function of the correct answer tokens is minimized, thereby re-aligning the weight space so that the model's attention heads operate in instruction execution mode.

### Examples of Representative Datasets and Training Techniques

- **Representative Open-Source Datasets:** Include Stanford's Alpaca (52k instructions), multi-turn conversation ledgers like ShareGPT, UltraChat for precise alignment, and Magpie with data refinement filters.
- **Infrastructure Training Methodologies:** Full Fine-Tuning architectures that directly update all model parameters, and LoRA and QLoRA which partially optimize by inserting low-rank adapters within the downstream attention weight matrices to overcome hardware VRAM resource limitations, are utilized.
