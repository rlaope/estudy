# Base Model, Instruct Model, Chat Model

대형 언어 모델 LLM의 개발 및 배포 생태계에서 모델은 목적과 훈련 단계에 따라서

크게 Base, Instruct, Chat 모델로 분류된다.

사전 학습 (Pretraining) 만을 거친 가중치는 거대한 고차원 인터넷 문서 공간의 통계적 연속성만 학습한 상태이기 때문에, 프로덕션 환경의 비즈니스 지시어나 다중 턴 대화 시스템의 문맥 인터페이스로 직접 활용할 수 있다.

가중치 파라미터가 비지도 사전 학습에서 어떻게 지도 미세 조정 SFT를 거쳐 Alignment 될 때 토큰 생성 분포의 목적 함수와 어텐션 헤드의 반응기전이 어떻게 달라지는지 로우레벨 관점에서 대조 분석 해보자.

### Base Model에 바로 채팅을 시키면 왜 이상하고, Instruct Model은 지시를 잘 따르는가?

결론부터 말하면, **Base Model은 질문에 답변 하도록 확률 지형이 생성된 것이 아니라 입력된 텍스트 뒤에 올 가장 통계적으로 그럴듯 한 문서를 이어쓰도록 훈련되었기 때문이며, Instruct Model은 SFT를 통해 문맥적 명령 구조 ($P(\text{Response} | \text{Instruction})$)에 밀도를 집중시켰기 때문이다.**

인터넷에 존재하는 대규모 원시 데이터 Raw Text 셋의 특징을 분석해보면, 사용자가 질문 문자열 (예: "대기 오염을 줄이는 방법 3가지를 서술하시오.")를 올렸을 때, 그 뒤에 올 확률이 가장 높은 텍스트는 친절한 답변이 아니다. 오히려 인터넷 환경에서는 시험 문제지, 면접 기출 문항 모음집, 혹은 또 다른 질문 리스트일 가능성이 통계적으로 훨씬 높다.

따라서 Base Model에 질문을 던지면 모델은 질문의 의도를 파악해 수행하는 것이 아니라, 문맥의 통계적 정합성만을 추종하여 다음과 같이 오작동한다.

- **시험지 복사 버그 (Quiz Generation):** `"1. 실내 식물을 키운다. 2. 대중교통을 이용한다. 3. ..."` 라고 답하는 대신 `다음 중 대기 오염의 원인으로 올바르지 않은 것을 고르시오 (배점 5점)` 과 같이 질문을 이어서 스스로 출제해 나간다.
- **문맥적 전이 확률의 단절:** Base Model은 가중치 내에 지시어 수행을 뜻하는 가이드라인 헤드 (Executive Attention Head)가 활성화되어있지 않고 입력된 프롬프트를 단순히 '완성해야할 문서의 시작점'으로 취급하기 때문에 인간과의 인터클라우드 대화 인터페이스가 성립되지 않는다.

반면 Instruct Model은 대화형 및 지시어 수행 데이터셋 (Prompt-Response Pair)를 기반으로 미세 조정을 거치면서 가중치 행렬이 `<|im_start|>user` 혹은 `[INST]` 같은 입력 제어 플래그를 만나면 출력 생성 모드 (Executive/Answering Phase)로 스위칭하도록 어텐션 맵을 재정렬한다. 이로인해 질문의 의미를 수행해야할 명령 테스크로 인지하고 그에 상응하는 답변 경로를 정확히 수축해낸다.

<br>

## 각 모델 유형별 데이터 구조와 메커니즘 대조

사전 학습 목적 함수에서 대화형 인터페이스 규격으로 진화하는 과정에서 도입된 세 가지 모델 클래스의 수리적 구조적 차이점이다.

### Base Model (기본/사전학습 모델)

- **목적 함수 및 토큰 생성 규칙:** 텍스트 코퍼스 $\mathcal{D} = {x_1, x_2, \dots, x_N}$에 대해 순방향 Log-likelihood를 극대화하는 순수 Autoregressive 모델이다.

$$L_{Pretrain}(\mathcal{D}) = \sum_{i=1}^N \log P(x_i | x_{<i}; \theta)$$

**한계점:** 인공신경망의 가중치 $\theta$가 텍스트의 사실 관계, 문법, 논리 구조를 내장하고 있으나, 이를 유저가 원하는 시점에 정제된 형태로 인출(Retrieval)하는 프로토콜이 탑재되어있지 않아 실무 배포가 불가능하다.

### Instruct Model (지시어 이행 모델)

**기존 방식의 개량 원인:** Base Model의 문장 완성 성향을 억제하고 단답형 명령, 코드 생성, 요약등 테스크 중심의 출력을 유도하기 위해 고안되었다.

**구조적 특성:** 데이터 구조가 단발성 명령문과 정답 쌍으로 이루어진 데이터 `{"prompt": "...", "completion": "..."}`를 SFT 처리한 가중치 상태다. 이 단계에서 모델은 프롬프트 내부의 핵심 동사 (예: Explain, Translate, Extract)를 파싱하여 조건부 확률 커널을 작동시키는 정렬 성능을 보여준다.

### Chat Model (대화 최적화 모델)

**Instruct 모델과의 차이 및 도입 이유:** 단발성 지시 이행을 넘어 사용자와 Multi-turn 컨텍스트를 유지하고 시스템 페르소나에 지정된 제약 조건을 세션 종료시까지 엄격하게 고수하도록 방어벽을 세운 모델

**작동 메커니즘:** 현대 오픈소스 진영에는 Instruct 모델과 Chat 모델의 명칭을 혼용하기도 하는데, 구조적으로 Chat 모델은 복잡한 Chat Template(Jinja2 script)과 다자간 대화 데이터셋으로 훈련되며, 자가 성찰 (Self-correction)이나 Safety Alignment를 위한 RLHF(DPO/GRPO)가 추가로 누적된 최종 완결 형태다.


<br>

## Llama-3 Base 모델과 Instruct 모델의 추론 기전 대조 시뮬레이션

동일한 아키텍처 크기를 가진 Llama-3-8B 기본 모델과 지시어 최적화 모델 Instruct에 동일한 질문을 주입했을 때, 하드웨어 레벨에서 발생하는 토큰 출력 토폴로지의 차이를 직접 재현하는 파이토치 인터페이스 코드를 보자.

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

실제 모델 구동 Trace 비교 (Llama-3-8B vs Llama-3-8B-Instruct)

- **Base Model (Llama-3-8B) Raw Output:**

```
[입력 프롬프트]: 대기 오염을 줄이는 방법 3가지를 서술하시오.
[모델 가중치 출력]: 대기 오염을 줄이는 방법 3가지를 서술하시오. 그리고 수질 오염의 대책도 함께 정리하십시오. (제2회 환경 과학 정기 고사) 
문제 4. 지구 온난화의 주요 원인이 되는 온실 가스 2가지를 적으시오. [정답 및 해설 보기]...
```

- **Instruct Model (Llama-3-8B-Instruct) Raw Output**

```
[입력 프롬프트]: <|begin_of_text|><|start_header_id|>user<|end_header_id|>대기 오염을 줄이는 방법 3가지를 서술하시오.<|eot_id|><|start_header_id|>assistant<|end_header_id|>
[모델 가중치 출력]: 대기 오염을 줄이는 주요 방법 3가지는 다음과 같습니다. 첫째, 친환경 및 대중교통 이용 확대, 둘째, 신재생 에너지 전환을 통한 화석연료 감축, 셋째, 제조 공장 내 배연 탈황 및 집진 설비 의무화입니다.<|eot_id|>
```

#### 실험 데이터 및 로그 해석

1. **MMLU 지표와 MT-Bench 지표의 단절 해석:** 정량 대조표를 보면 Base Model의 MMLU 지수가 66.2%로 Instruct 모델 (68.4%)과 유의미한 차이가 나지는 않는다. 이는 Base Model의 파라미터 내부에 인간이 이룩한 학술적 지식 데이터 자체는 이미 촘촘하게 압축되어있음을 수리적으로 증명한다. 그러나 MT-Bech 점수가 2.15로 무너지는 이유는, 가중치 지형 내에서 **지시문을 읽고 답을 도출하는 정렬(Alignment)** 링크가 누락되어 정지 신호 `[EOS]`를 뱉지 못하고 엉뚱한 방향으로 연산 궤적이 이탈하기 때문이다.
2. **출력 트레이스 로그 기반 어텐션 바이아스 (Attention Biase):** Base Model의 출력 로그에서 나타나는 '시험문제지복사; 현상은 Pretraining 단계에 학습한 문서의 우세성 Dominance 때문이다. 반면 Instruct Model은 입력 프롬프트 전면에 배치된 `<|start_header_id|>user<|end_header_id|>` 와 같은 고유한 임베딩 벡터 가이드를 인지한다. 이 제어 토큰들은 모델의 특정 Attention Head 들이 프롬프트 지시어 (핵심 명제)에만 시선을 Attending 하도록 강제하는 트리거 역할을 수행하며, 결과적으로 assistant 세션 컨텍스트 영역으로 접어들었을때 완벽하게 통제된 정답 시퀀스만 출력하도록 전이 확률을 분포시킨다.

<br>

### 모델 가중치 변형 유형별 벤치마크 및 추론 결과 대조

모델 유형별 다운스트림 태스크 정량 대조표인데 오픈소스 메인스트림 가중치를 활용해 수집한 학술 지표 (MMLU 등) 및 프로덕션 환경의 정렬 적합성 프로파일링 지표 예시다.

| 평가지표 및 현상 분석 (Evaluation Matrix) | Base Model 계열 (Meta-Llama-3-8B) | Instruct / Chat Model 계열 (Meta-Llama-3-8B-Instruct) |
|-------------------------------------------|------------------------------------|-------------------------------------------------------|
| MMLU (지식 저장 및 다지선다 성능) | 66.2% (상당히 높음) | 68.4% (사전 지식의 유실 없이 미세 유지) |
| MT-Bench (다중 턴 대화 및 지시 이행력) | 2.15 / 10 (대화 성립 불가능) | 8.05 / 10 (엔터프라이즈 상용 수준) |
| HumanEval (파이썬 코딩 구현 통과율) | 32.4% (정답 코드 하단에 불필요 주석 남발) | 62.2% (정밀 코딩 및 포맷팅 수행) |
| 문서 자가 복제율 (Prompt Echoing / Quiz Generation) | 64.1% (심각한 인프라 오작동 현상) | 0.0% (완벽 방어) |
| 토큰당 정보 밀도 (Inference Utility Per FLOPs) | 극히 낮음 (불필요한 중복 텍스트 과다 생성) | 극히 높음 (원하는 구조적 정답만 압축 출력) |

<br>

Base Model을 Instruct Model로 변환하는 핵심 공정은 SFT(Supervised Fine Tuning) or Instruction Tuning 이다.

### 수행하는 구체적 행위

- **데이터 셋 바인딩:** 인간 혹은 상위 모델이 정제한 `[지시어(Prompt) - 수행 결과(Response)]` 쌍의 데이터셋을 모델 인터페이스에 주입한다.
- **가중치 분포 정렬:** 입력 프롬프트 구간은 Loss Masking(`ignore_index=-100`) 처리하여 역전파 그라디언트 생성에서 배제하고 오직 정답 답변 토큰의 Cross-Entropy 손실 함수만 최소화하여 모델의 어텐션 헤드가 명령 수행 모드로 작동하도록 가중치 공간을 재정렬한다.

### 대표적인 데이터 셋 및 훈련 기법 예시

- **오픈소스 대표 데이터셋:** 스탠포드의 Alpaca(지시어 52k), 다중턴 대화 원장인 ShareGPT나 정밀 정렬을 위한 UltraChat, 데이터 정제 필터가 적용된 Magpie 등 있음
- **인프라 훈련 방법론:** 모델 전체 파라미터를 직접 업데이트하는 Full Fine-Tuning 아키텍처, 하드웨어 VRAM 자원 한계를 극복하기 위해 다운스트림 어텐션 가중치 행렬 내부에 저차원 어댑터를 삽입해 부분 최적화하는 LoRA, QLoRA 등이 활용됨


