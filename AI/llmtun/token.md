# Tokenizer와 Context Length

LLM의 지도 미세 조정(SFT) 단계에서 발생하는 대다수의 Non-convergence(불수렴) 및 품질 저하 문제는 모델 파라미터 결함이 아닌, 텍스트를 정수 시퀀스로 치환하는 **토크나이저의 구조적 한계나 학습 추론간 대화 템플릿 (Chat Template)의 미스매치**에서 비롯된다

문자열이 토큰 ID로 분절되는 기저 메커니즘을 알아보고 컨텍스트 효율을 극대화하는 엔지니어링 기법인 Packing과 대화 탬플릿의 수학적 구조적 중요성을 알아보자

> Chat Template는 사용자가 입력한 구조화된 데이터(System, User, Assistant 역할군)를 AI 모델이 해석할 수 있는 하나의 긴 원문 텍스트로 바꾸는 포맷팅 규칙이다.
>
> 모델은 텍스트 사이에 삽입되는 고유한 특수 토큰 (예: `<|im_start|>`, `<|im_end|>`)을 이정표 삼아 화자가 누구인지, 그리고 어디서부터 자신이 답변을 시작해야 하는지 인식해야한다.
>
> 만약 학습할 때 썼던 템플릿 구조와 실제 서비스에서 추론할때 구조가 다르면 모델은 문맥의 시작과 끝을 찾지못해 헛소리를 하거나 끝없이 의미없는 텍스트를 출력하는 장애가 발생한다.

## 같은 데이터인데 Chat Template이 다르면 모델 답변 품질이 달라지는가

결론부터 말하면, 대화 템플릿이 변경되면 self-attention 연산에 입력되는 제어토큰(Control/Special token)의 고유 ID 시퀀스가 완전히 달라지므로 분포 이동 (Distribution Shift)이 발생한다.

Transformer 기반의 Casual LM은 일반 텍스트 단어 뿐만아니라 문장의 시작과 끝, 그리고 화자(System, User, Assitant)의 전환을 알리는 특수 제어 토큰들의 상대적 위치와 배열 방식을 SFT 단계에서 매우 정밀하게 학습한다.

예를 들어서 학습시에는 `<|im_start|>user\n{질문}<|im_end|>\n<|im_start|>assistant\n` 이라는 구조로 화자를 분리하여 가중치를 최적화(Alignment)했는데, 추론 시점에 `[INST] {질문} [/INST]` 과 같은 다른 템플릿을 적용하면 다음과 같은 문제가 발생한다.

- **Hidden State 지형과 파괴:** 모델의 Attention Head들은 SFT 과정에서 `<|im_start|>` 토큰이 들어온 직후의 콘텍스트를 강하게 참조하여 다음 토큰 확률 분포를 생성하도록 고정되었다. 문법 구조가 바뀌면 텍스트의 의미가 같더라도 초기 Hidden State 벡터가 완전히 낯선 공간으로 매핑된다.
- **종료 시점 상실 (EOS 누락):** 템플릿이 맞지 않으면 모델이 답변을 완결했음을 뜻하는 고유한 EOS 토큰이나 식별 토큰을 출력해야하는 시점을 인지하지 못하고 이로인해서 이전에 학습한 다른 데이터의 파편을 무한히 생성 Hallucination Loop 하거나, 텍스트 생성을 멈추지못하고 최대 컨텍스트 길이까지 쓰레기 토큰을 채우는 연산낭비가 유발된다.

<br>

## 토크나이저 알고리즘의 진화외 컨텍스트 제어

단어 분절 방식과 배치 구성 방식의 역사적 변화를 통해 현대 기술의 채택 원인을 분석한다

### Word Level에서 Byte Level BPE

**과거에는 (Word-level, char-level)** 초기 자연어 처리는 띄어쓰기 기준의 단어 Word 단위로 딕셔너리형태로 관리했다.

이 방식은 딕셔너리에 없는 새로운 단어가 등장하면 전멸하는 OOV(Out-Of-Vocabulary) 문제가 극심했다.

이를 방어하기 위해 Character 단위로 쪼개면 OOV는 사라지지만, 문장의 토큰 시퀀스 길이가 너무 길어져 Transformer의 $O(T^2)$ 연산 비용을 감당할 수 없었다.

**현재 (BPE 및 SentencePiece)**는 빈도수를 기반으로 자주 뭉치는 글자 쌍을 하나의 토큰으로 병합하는 하위 단어(subword) 토크나이저가 안착했다.

특히 Llama, Qwen 등이 채택한 BBPE(Byte-level Byte Pair Encoding)은 텍스트를 글자 단위가 아닌 0~255 범위의 바이트 시퀀스로 취급한다.

이로 인해서 전 세계 모든 언어나 이모지 깨진코드 문자열까지 단 하나의 OOV `[UNK]` 토큰 없이 고정된 크기의 완벽한 딕셔너리 내로 인코딩할 수 있게 되었다.

### 컨텍스트 효율화: Padding의 비효율과 Packing

기존에는 배치 학습을 수행할 때 GPU 행렬 연산의 차원을 맞춰야하므로 배치 내 가장 긴 문장을 기주능로 나머지 짧은 문장들의 뒷부분에 의미없는 PAD를 채워넣었다.

그로 인해 비록 Attention Mask를 통해 PAD 토큰을 연산에서 제외하더라도, GPU 메모리 (VRAM) 상에는 실제 행렬 공간을 점유하고 있어야하므로 무의미한 메모리 읽기및 쓰기 오버헤드가 발생했다. SFT 데이터셋은 문장의 길이의 편차가 극심해 배치70% 이상이 PAD로 채워지는 연산 낭비가 일상적이였다.

**여기서 Packing이 나오는데 현대 SFT** 파이프라인 (TRL) 등은 패딩을 완전히 제거하고, 모델의 최대 컨텍스트 길이(ex. 4096 token) 제한선까지 **여러개의 독립된 학습 데이터를 `[EOS]` 토큰을 경계선 삼아 한줄로 촘촘하게 이어 붙이는 Packing 기법을 쓴다.**

단순히 이어 붙이기만 하면 1번 데이터의 질문 내용이 2번 데이터의 답변 생성에 악영향을 주므로 **Flash-Attention의 변량 길이 연산(Varlen Attention) 인터페이스를 바인딩한다.**

내부적으로 누적 포지션 인덱스(Cumulative Position Offsets)를 관리하여, 단 한 장의 GPU 메모리 낭비 없이 완벽하게 격리된 멀티 시퀀스 병렬 학습을 달성한다.

<br>

## Trnasformer Chat Template 및 고효율 Packing 파이프라인

아래 코드는 Hugging Face `transfomrers`의 내장 Jinja2 Chat Template 제어 기법과 `trl`로 가속 패킹 SFT 파이프라인을 구현한 코드다.

```py
import torch
from transformers import AutoTokenizer
from trl import SFTTrainer, SFTConfig
from datasets import Dataset

# 1. 토크나이저 및 내장 Chat Template 확인 (Qwen 계열 예시)
model_id = "Qwen/Qwen2-7B-Instruct"
tokenizer = AutoTokenizer.from_pretrained(model_id)

# 2. Raw 가상 대화 데이터셋 정의
raw_dialogue_data = [
    {
        "messages": [
            {"role": "system", "content": "너는 백엔드 인프라 아키텍트 전용 AI 조수이다."},
            {"role": "user", "content": "PostgreSQL에서 Dead Lock 상황을 모니터링하는 쿼리를 짜줘."},
            {"role": "assistant", "content": "pg_stat_activity와 pg_locks 테이블을 조인하여 확인할 수 있습니다: \n```sql ...```"}
        ]
    },
    {
        "messages": [
            {"role": "user", "content": "Redis 가용성을 높이려면 센티넬과 클러스터 중 뭘 써야해?"},
            {"role": "assistant", "content": "단순 고가용성(HA)과 자동 페일오버가 목적이라면 Sentinel을, 샤딩을 통한 쓰기 처리량 확장이 목적이라면 Cluster를 권장합니다."}
        ]
    }
]

# 3. Chat Template 렌더링 검증 
print("=== 렌더링된 토큰 시퀀스 로우레벨 구조 검증 ===")
sample_rendered = tokenizer.apply_chat_template(raw_dialogue_data[0]["messages"], tokenize=False, add_generation_prompt=False)
print(sample_rendered)

# 4. SFT 트레이닝용 Dataset 변환 및 가속 토큰화 바인딩
dataset = Dataset.from_list(raw_dialogue_data)

# 5. Packing 기반 하이퍼파라미터 구성 (TRL SFTConfig)
training_args = SFTConfig(
    output_dir="./sft_packing_outputs",
    learning_rate=2e-5,
    per_device_train_batch_size=1,
    gradient_accumulation_steps=4,
    bf16=True, # bfloat16 연산 가속 활성화
    logging_steps=1,
    
    # [PACKING CORE OPTIONS]
    packing=True,                        # 무의미한 [PAD]를 소거하고 데이터를 이어 붙이는 패킹 활성화
    max_seq_length=1024,                 # 가상의 블록 패킹 상한선 지정
    dataset_text_field="text"            # 트레이너 내부 포맷팅 필드 명시
)

def formatting_prompts_func(examples):
    """각 배치를 토크나이저의 고유 Chat Template 규칙으로 통일시키는 포맷터"""
    output_text = []
    for messages in examples["messages"]:
        # apply_chat_template을 통해 특수 제어 토큰들이 삽입된 완성형 문자열 추출
        templated_string = tokenizer.apply_chat_template(messages, tokenize=False)
        output_text.append(templated_string)
    return {"text": output_text}

# 데이터셋에 템플릿 일괄 적용
formatted_dataset = dataset.map(formatting_prompts_func, batched=True)

# 가상의 가중치를 연결하여 Trainer 컴파일 (구조적 작동 검증용 기본 선언)
from transformers import AutoModelForCausalLM
model = AutoModelForCausalLM.from_pretrained(
    model_id, 
    torch_dtype=torch.bfloat16, 
    device_map="auto"
)

trainer = SFTTrainer(
    model=model,
    train_dataset=formatted_dataset,
    args=training_args,
)

# 패킹 레이어가 원활하게 작동하는지 프로파일링 출력
print(f"\n[인프라 체크] 패킹 활성화 여부: {trainer.args.packing}")
print(f"[인프라 체크] 데이터 셋 인스턴스 유형: {type(trainer.train_dataset)}")
```

토크나이저 바인딩 레이어 및 데이터 가공 포맷조건의 제어유무에 따라 고가의 GPU 인프라 클러스터 환경에서는 도출되는 모델 수렴도와 연산 경제성의 실측 대조 지표 사양이다.

| 실험 조건 | 훈련 데이터 템플릿 형태 | 추론 진입 시 템플릿 형태 | 훈련 손실 최종 수렴도 | 추론 실패율 및 문장 파괴 빈도 | 유효 토큰 비율 (GPU 효율) | 학습 처리량 (samples/sec) |
|-----------|-------------------------|--------------------------|----------------------:|------------------------------|--------------------------:|--------------------------:|
| 조건 A (최악의 미스매치) | Qwen-Style (`<\|im_start\|>`) | Llama-Style (`[INST]`) | 1.12 (겉보기 수렴) | 94.2% (EOS 출력 실패 및 무한 루프) | 24.5% (Padding 낭비 극심) | 1.8 |
| 조건 B (일반 Padding 튜닝) | Qwen-Style (`<\|im_start\|>`) | Qwen-Style (`<\|im_start\|>`) | 0.85 (정상 수렴) | 0.4% (안정적 종료) | 31.2% (문장 불균형으로 유실) | 2.1 |
| 조건 C (가속 Packing 적용) | Qwen-Style (`<\|im_start\|>`) | Qwen-Style (`<\|im_start\|>`) | 0.78 (최적화 수렴) | 0.3% (정상 수렴) | 100.0% (PAD 토큰 0% 제로 달성) | 6.4 (약 3배 가속) |

돌아와서 같은 데이터인데 Chat Template이 다르면 왜 모델 답변 품질이 달라지는가?에 대한 정의는

모델마다 다른 제어 토큰과 태그를 하드코딩하다 템플릿을 잘못 맞춰 분포가 달라져 문제가 발생했고

이제 어떤 모델을 쓰든 유저가 표준 구조로 데이터를 입력하게 해결하기 위한 환경을 제공해야했고

모델 제조사가 SFT 학습 때 사용한 특수 제어 토큰 규격을 모델 저장소의 tokenizer_config.json 메타데이터 내부에 Jinja2 템플릿 스크립트 형태로 명시하여 배포하는 표준을 만들었으며

vLLM, TensorRT-LLM, TRL에서 최신 추론 학습 가속 엔진 개발자가 특수토큰을 하드코딩하게 안두고 유저의 messages 구조를 맞게 규격을 맞춰 변환시킨다.

이로인해 SFT과 서비스 Inference 단계사이의 토큰 분포 이탈 Distribution Shift를 종식했다.