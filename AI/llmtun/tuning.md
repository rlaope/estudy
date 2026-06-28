# Continual Pretraining과 Fine-tuning 차이

LLM을 특정 도메인 (의료, 법률, 금융 등)에 최적화하거나 기업용 독자 솔루션으로 변환할 때, 모델의 레이어 가중치를 어느 단계에서 어떤 수리적 목적으로 업데이트할지 결정하는 아키텍처 엔지니어링은 인프라 자원 효율성을 가르는 핵심 분기점이다.

포스트 트레이닝 사슬은 미지의 코퍼스를 내장시키는 Continual Pretraining(CPT), 지시어 문법과 대화 스타일로 교정하는 Supervised Fine-Tuning(SFT) 최종적인 보상 지형을 설계하는 Preference Alignment로 나뉜다. 각 기법은 활성화되는 가중치 공간의 업데이트 영역과 토큰의 예측 앤프로피를 통제하는 메커니즘에서 근본적인 차리를 가진다.

## 지식이 부족한 문제인가, 말투가 문제인가, 선호 정렬이 문제인가

이 질문의 본질은 모델의 성능 결함이 발생했을 때, 이를 **가중치 내부 파라미터 정보 (Parameter Knowledge) 부재로 인한 현상인건지 원하는 규격의 출력 인터펲이스를 트리거하지 못한 인코딩/스타일의 문제인건지 혹은 인간이 선호하지 않는 비정상적 출력을 밀어내지 못하는 보상왜곡의 문제인지**

즉 CPT, SFT, PA중에 어떤게 문제인가를 병목을 진단하는 인프라 파이프라인의 기존선 정립이다.

이를 혼동하여 정보가 없는 모델에 단순 SFT 스타일 튜닝만 가하면 모델이 아는 척 거짓말을 지어내는 환각이 폭증하고, 반대로 말투만 바꾸면 되는 모델에 수십 기가바이트의 데이터로 CPT를 재수행하면 기존에 가진 범용 연산 능력이 무너지는 치명적 망각 (Catastrophic Forgetting)이 유발되므로 결함의 근본 원인을 기하학적으로 식별하는 정밀 진단이 필수적이다.

<br>

## 가중치 업데이트 목적에 따른 3대 정렬 공정

### Continual Pretraining (CPT, 지속적 사전 학습)

**기본 메커니즘:** 사전 학습 Pretraining 단계의 비지도 학습 (Unsupervised Next-token Prediction) 목적 함수를 그대로 유지한 채, 특정 도메인의 대규모 원시 텍스트 코퍼스 (Raw Text Corpus)를 주입하여 모델 파라미터 자체에 새로운 Knowledge base를 압축 축적 (Parameter Storage) 시키는 공정이다.

**도입 원인:** SFT 데이터 바인딩이나 RAG 프레임워크만으로는 극복할 수 없는, 고유 도메인의 특수 용어 구조, 소스 코드 문법, 특수 도메인 명사간의 통계적 전이 확률 밀도 지형을 모델 가중치에 물리적으로 내장하기 위함이다. 지식의 양 자체를 늘리는 유일한 사후 학습 기법

사람이라 치면 책을 읽게하는것, 사전을 읽게하는것 처럼 기초적으로 갖고있는 지식을 더해주는 작업

### Instruction Fine-Tuning (SFT, 지도 미세 조정)

**기본 메커니즘:** 이미 필요한 지식을 확보한 가중치 상태에 `[Prompt - Response]` 형태의 구조화된 대화 데이터셋과 프롬프트 마스킹 가드를 결합해, 모델의 attention head를 명령어 수행 및 대화 포맷 규격에 강제 정렬(Behavioral/Formatting Alignment) 하는 과정이다.

**도입 원인:** CPT가 완료된 모델은 여전히 원시 문서 이어쓰기 성향을 버리지 못하므로, 이를 제어할 수 있는 API 형태로 정렬하여 말투, 제약 조건 준수율, 태스크 완결성을 트리거하기 위함이다. 지식을 주입하는 용도가 아닌 스타일 교정 기법.

새 직원에게 CPT로 지식을 알려주고 SFT는 엔지니어링 장비 사용 가이드, 그리고 수행 지침등을 알려주고 이행하는 일을 시키기 위해 교육하는 작업.

### Preference Alignment (선호도 정렬 - DPO/GRPO)

**기본 메커니즘:** SFT까지 완료된 모델의 생성 분포상에서 미세한 보상 지형 (Reward Landscape)를 설계하여 인간이 선호하는 경로(Chosen)의 밀도는 올리고 기피하는 경로 (Rejected) 밀도는 밀어내는 정책 그라디언트 (Policy Gradient) 변형 공정이다.

**도입 원인:** SFT는 정답 데이터의 로그 우도(Log-=likelihood)만 높기 때문에, 틀린 답변이나 우회해야하는 보안 취약점을 적극적으로 밀어내는 척력을 발생시키지 못하낟. 안정성 가드레일 확립 및 인간 선호도 일치를 위해 도입된다.

새 직원이 일처리 지침은 지키지만, 쓸대없는 반복 방지를 위해 꿀팁 같은걸 전수해주거나 일을 수행하다 틀리게 처리했을때 이를 보정해주고 알려주는 작업

<br>

## CPT(지속 사전 학습)을 위한 블록 패킹 데이터셋과 SFT 데이터 가공 파이프라인 대조

아래 코드는 동일한 원시 데이터 환경에서 지식 주입을 위한 CPT 데이터 콜레이터(텍스트 전체를 블록 크기로 쪼개어 마스킹 없이 학습)과 스타일 정렬을 위한 SFT용 데이터 콜레이터(프롬프트 영역 소거)를 하드웨어 파이프라인 관점에서 분리 구현한 엔지니어링 소스 코드다.

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

### 실제 모델 구동 Trace로그 비교

- **상태1: 범용 Base 모델에 특수 금융 도메인 질문 주입시**

```
[User]: 바젤III 규제 환경에서 RWA를 산출할 때 내부등급법(IRB)과 표준방법의 핵심 차이를 설명하라.
[Base Model]: 바젤III 규제 환경에서 RWA를 산출할 때 내부등급법(IRB)과 표준방법의 핵심 차이를 설명하라. 그리고 그것이 은행 자본 확충에 미치는 영향에 대한 논문을 다운로드 하려면 아래 링크를 클릭... (문서 이어쓰기 폭주)
```

- **상태 2: 상태 1 모델에 추가로 CPT(지속 사전학습)만 수행하고 지시어 정렬은 안 했을 시**

```
[User]: 바젤III 규제 환경에서 RWA를 산출할 때 내부등급법(IRB)과 표준방법의 핵심 차이를 설명하라.
[CPT Model]: 내부등급법(IRB)은 은행이 자체 개발한 신용평가 모형(PD, LGD, EAD)을 활용하여 위험가중자산을 산출하는 반면 표준방법은 금융감독당국이 지정한 고정 위험가중치를 일괄 적용한다. 한편, 2026년도 자산 건전성 분류 기준에 따르면... (금융 지식은 정확하나 끊기지 않고 무한 책 서술 모드 가동)
```

- **상태3:** **상태 2 모델에 SFT(대화형 정렬) 및 선호도 최적화(DPO)까지 완료했을 시**

```
[User]: 바젤III 규제 환경에서 RWA를 산출할 때 내부등급법(IRB)과 표준방법의 핵심 차이를 설명하라.
[Aligned Model]: 바젤III 기준 RWA(위험가중자산) 산출 방식의 핵심 차이는 '자체 추정치 사용 여부'에 있습니다.
1. 내부등급법(IRB): 은행이 자체 보유한 내부 데이터로 부도확률(PD), 부도시손실률(LGD)을 직접 추정하여 유연하게 반영합니다.
2. 표준방법: 감독기관이 규정한 고정 위험가중치를 일괄 적용하므로 보수적입니다.
요청하신 차이점 정리를 마칩니다.<|im_end|>
```

#### 상태별 결과

로그 해석을 해보면 **지식 저장 메커니즘과 스타일 분리의 수리적 증명**을 할 수 있는데

상태1 Base는 금융 도메인 지식의 전이 확률 밀도가 낮아서 헛소리를 하거나 질문을 복사하고

상태 2 CPT는 금융 지식을 정확하게 인출해내 이는 대규모 raw corpus 학습을 통해 파라미터 내부에 지식 자산이 안착했음을 뜻한다.

그러나 정지 토큰인 `<\im_end\>` 를 내뱉지 못하고 계속 관련 금융 백서를 이어쓰는 한계가 관찰된다.

최종적으로 상태 3에 이르러셔야 SFT Behavioral 가드가 작동해 구조적 요약 및 세션 반환이 정상화된다.

#### 인프라 자원 소모 및 병목 제어

CPT는 대규모 원시 데이터를 수백만 윈도우 크기로 밀어 넣어야하므로 단일 노드 VRAM을 넘어 대규모 분산 환경 가속 프레임워크 (**Megatron-LM, DeepSpeed ZeRO-3**)가 필수적으로 개입해야하는 **Compute-bound(연산 중심 병목)** 공정이다.

반면 SFT나 Prefrenece Alignment는 상대적으로 데이터 용량이 메가바이트 단위로 작고, 오직 포맷 최적화에만 집중되므로 대규모 클러스터에서도 처리가능한 메모리, IO 바운드 성격을 띤다.

<br>

### 사후 학습 방법론별 아키텍처 비교 메트릭스

현대 LLM 고도화 체인에서 목적에 따라 인프라 예산을 다르게 분배하는 정량적 기술 지표 속성 대조표

| 비교 아키텍처 지표 | 1단계: Continual Pretraining (CPT) | 2단계: Instruction Fine-Tuning (SFT) | 3단계: Preference Alignment (DPO/GRPO) |
|--------------------|------------------------------------|---------------------------------------|-----------------------------------------|
| 인프라 주 기전 및 목적 | 가중치 내부에 미지의 도메인 지식(Parametric Knowledge) 주입 및 물리적 각인. | 입력된 명령어를 파싱하여 원하는 포맷과 스타일로 응답하는 인터페이스 행동 교정. | 다중 출력 후보군 중 인간이 선호하는 경로를 강화하고 탈옥 및 오답 경로를 억제. |
| 데이터셋 규격 및 가공 형태 | 포맷 없는 순수 Text 시퀀스 (`{"text": "..."}`).<br>`[PAD]` 없이 최대 문맥까지 촘촘히 이어 붙임. | 구조화된 지시어 쌍 (`{"prompt": "...", "response": "..."}`).<br>Prompt 영역은 `-100` Loss 마스킹 필수 적용. | 선호도 대조 쌍 (`{"prompt": "...", "chosen": "...", "rejected": "..."}`).<br>또는 실시간 그룹 보상 채점. |
| 손실 함수 (Objective) | Cross-Entropy (Vanilla Next-token Prediction) | Cross-Entropy (Completion-only Masked Likelihood) | Implicit Reward Log-odds (DPO) 또는 Policy Gradient (GRPO) |
| VRAM 및 가속화 인프라 스택 | 극도로 높음 (Megatron-LM 파이프라인 병렬화, DeepSpeed ZeRO-3, 다중 노드 텐서 슬라이싱 필수) | 보통 (TRL SFTTrainer, 단일 노드 가동 가능, 하드웨어 효율화를 위해 LoRA/QLoRA 결합) | 보통~높음 (Online RL의 경우 대규모 Rollout 생성을 위한 실시간 vLLM 분산 엔진 결합 필요) |
| 잘못 설계 시 발생 크래시 현상 | 치명적 망각(Catastrophic Forgetting): 도메인 지식은 얻으나 언어 모델 고유의 범용 대화 및 추론 능력이 소멸함. | 표면적 모방(Superficial Alignment): 내부에 지식이 없는 상태에서 말투만 그럴듯하게 배워 환각(Hallucination)이 극대화됨. | 보상 해킹(Reward Hacking): 보상 점수를 속이기 위해 겉보기에만 화려하고 알맹이가 없거나 기괴한 토큰을 무한 출력함. |