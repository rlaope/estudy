# Self-Distillation과 Iterative Self-Improvement

Post Traning의 패러다임은 고정된 정적 데이터셋(Static Dataset)을 정제하는 단계를 넘어서 모델이 스스로 데이터를 생산하고 검증하며 아키텍처를 점진적으로 진화시키는 **반복적 자가 개선(Iterative Self-Improvement) 구조로 수렴하고 있다.**

자가 데이터 학습시 발생하는 기술적 위험인 모델 붕괴 Model Collapse의 원인을 진단하고, 이를 방어하는 **On-Policy Distillation 및 반복 최적화 루프의 로우레벨 메커니즘을 분석한다.**

## 자가 데이터 학습은 성능을 개선하는가, 오류를 강화하는가?

### 기본적 위험: 에러 증폭(Error Amplification)과 모델 붕괴

외부의 엄격한 통제 없이 모델이 생성한 데이터셋 (($\mathcal{D}_{syn} \sim \pi_\theta$)만을 가지고 자가 학습(SFT/DPO)를 반복하면, 모델 내부의 미세한 환각과 편향이 매 세대마다 누적된다.

데이터의 엔트로피가 감소하면서 특정 단어만 반복하거나 논리가 단순화되는 **Model Collapse**가 발생한다.

즉 단순한 자기 오류를 더 강하게 학습하게 되는 꼴이 되는것이다.

### 해결책: 엄격한 필터링(Sifting)과 On-Policy Distillation

스스로 성능 개선 (Self-Improvement) 하기 위해서는 반드시 두 가지 엔지니어링 조건이 충족되어야한다.

1. **결정론적 검증기(Programmatic Verifier) or 강력한 상위 모델 Teacher Judge**: 생성된 데이터 중 참/거짓이 확실히 판별된 데이터만 남기는 가차없는 필터링 체계가 결합되어야 한다.
2. **트레이젝토리 미스매치(Trajectory Mismatch) 해소**: Off-Policy Distillation은 Teacher 모델의 분포 ($\pi_{teacher}$)에서 샘플링된 최적의 경로만을 학습하므로, Student 모델이 실제 추론(Inference) 단계에서 경로를 이탈했을 때 복구하지 못하는 문제가 있었다. 반면, 최신 On-Policy Distillation 기법은 Student 모델이 직접 실패와 성공의 궤적 (Trajectory, $\tau \sim \pi_\theta$)을 생성하게 한 뒤, 그 오답노트 위에 Teacher 모델의 피드백(Supervision)을 읽는 방식으로 학습 분위기를 정렬하여 추론 분포 오차를 원천 차단한다.

> 여기서 나오는 OPD(On-Policy Distillation)은 전장에서 알아본 On-Policy SFT랑은 다르다. OPD(SD)는 온폴리시 증류로 모델 스스로가 답변을 생성 후 토큰 확률 분포 Logit을 가져와 KL 발산 (Kullback-Leibler Divergence)를 계산해 업데이트 한다. 즉 단어별 확률 정보라는 지식기반이고 OnSFT는 결과 자체를 보는 것이다.


<br>

## Iterative Self-Improvement 루프 아키텍처

반복 개선 시스템은 한 번의 학습으로 끝나지 않고 모델 $\pi_{\theta_t}$가 다음 세대 $\pi_{\theta_{t+1}}$의 데이터 공급원이 되는 선순환 플로우를 구성한다.

1. **Self-Generate(온라인 탐색):** 현재 정책 $\pi_{\theta_t}$를 사용하여 대량의 프롬프트 세트에 대해 멀티 샘플링(Rollout)을 수행한다.
2. **Judge / Verifier (다면적 평가):** 생성된 텍스트 결과물을 Execution, Regex 또는 LLM-as-a-Judge를 통해 정밀 스코어링 한다/
3. **Filter (오답 노트 추출 및 Preference Pair 구성) *SFT 대상:** 오답 경로를 버리고 우연히 성공한 고품질 경로만 추출한다. (STaR 알고리즘 계열)
   1. **DPO 대상:** 동일 프롬프트에서 도출된 성공 응답과 실패응답을 매치해 Prefrence 데이터셋을 빌드한다.
4. **Train & Evolve (가중치 업데이트):** 최적화 엔진 TRL을 통해 모델 업데이트 후 $\pi_{\theta_{t+1}}$ 를 도출하고 W&B 수렴도를 모니터링 뒤에 Iteration의 가속 엔진으로 재투입한다.


Self-Distillation은 과거 딥러닝처럼 단어별 분포 Logits를 지식으로 넘겨주는 것이 아니라 모델이 직접 생성한 완전한 Sequence를 데이터셋으로 재활용하는 것을 의미한다.

이때 자기가 생성한 문장들을 채점하여 chosen, rejected 쌍으로 만들고 이 쌍을 활용해 가중치 분포를 가장 효율적으로 업뎃하는  학습 도구가 바로 DPO라서 파이프라인에 포함된다.

정답만 골라 학습하는 SFT 방식은 자기가 뱉은 오답 경로를 제어하지 못해 자기 오류를 강화하는 문제를 낳지만 DPO를 결합하면 스스로 저지른 오답 확률을 직접 낮출 수 있어 오류 증폭을 방어한다.

요약하면 **Self-Distllation은 스스로 텍스트 데이터를 생산하는 공급 방식에 DPO는 그 생성된 데이터의 선호도를 비교하여 가중치를 깎아내는 수학적 알고리즘이다.**

<br>

## Iterative Self-Improvement 의사코드

매 세대마다 자가 데이터를 수집, 필터링하여 DPO 데이터셋을 동적으로 구하고 점진적으로 갱신하는 기획 코드를 보자


```py
import json
from vllm import LLM, SamplingParams
from transformers import AutoTokenizer
from trl import DPOConfig, DPOTrainer
from datasets import Dataset

def run_iterative_loop(model_path, prompts, iteration_id):
    print(f"=== START ITERATION {iteration_id} ===")
    
    # 1. Self-Generate: 현재 세대 모델로 Rollout 샘플 추출
    # 다양성 확보를 위해 temperature를 높여 프롬프트당 4개씩 생성
    llm = LLM(model=model_path, tensor_parallel_size=2)
    sampling_params = SamplingParams(n=4, temperature=0.7, max_tokens=256)
    raw_outputs = llm.generate(prompts, sampling_params)
    
    # vLLM 인스턴스 해제 (VRAM 확보 후 SFT/DPO 트레이닝에 자원 반환)
    import gc; import torch
    del llm; gc.collect(); torch.cuda.empty_cache()
    
    # 2 & 3. Judge & Filter: Preference Pair 구축
    preference_data = []
    for output in raw_outputs:
        prompt_text = output.prompt
        completions = [out.text.strip() for out in output.outputs]
        
        # 외부 Verifier 로직 작동 (여기서는 코드 구문 컴파일 및 길이 기반 가상 검증)
        scored_completions = []
        for c in completions:
            score = 1.0 if ("def " in c and len(c) > 50) else 0.0 # 간단한 룰셋 예시
            scored_completions.append((c, score))
            
        # 정답(Chosen)과 오답(Rejected) 분리 매칭
        chosen = [c for c, s in scored_completions if s == 1.0]
        rejected = [c for c, s in scored_completions if s == 0.0]
        
        if chosen and rejected:
            preference_data.append({
                "prompt": prompt_text,
                "chosen": chosen[0],
                "rejected": rejected[0]
            })
            
    if len(preference_data) < 10:
        print("유효 데이터 부족으로 루프 조기 종료.")
        return model_path

    # 4. Train: TRL 엔진을 활용한 DPO 갱신
    train_dataset = Dataset.from_list(preference_data)
    tokenizer = AutoTokenizer.from_pretrained(model_path)
    model = AutoModelForCausalLM.from_pretrained(model_path, torch_dtype=torch.bfloat16, device_map="auto")
    
    output_dir = f"./checkpoint_iter_{iteration_id}"
    dpo_config = DPOConfig(
        output_dir=output_dir,
        learning_rate=2e-6,
        per_device_train_batch_size=2,
        num_train_epochs=1,
        bf16=True,
        report_to="wandb" # Weights & Biases 실험 추적 연동
    )
    
    trainer = DPOTrainer(model, args=dpo_config, train_dataset=train_dataset, tokenizer=tokenizer)
    trainer.train()
    
    # 모델 저장 후 자원 해제 및 다음 세대 경로 반환
    trainer.save_model(output_dir)
    del model, trainer; gc.collect(); torch.cuda.empty_cache()
    
    return output_dir
```

| 세대 (Generation) | 소스 데이터 출처 (Data Source) | 코딩 벤치마크 (HumanEval Pass@1) | 데이터 단편화/중복도 (Token Diversity) | 오답 이탈률 (Exposure Bias) | W&B 보상 수렴도 (Implicit Reward Margin) |
|------------------|--------------------------------|---------------------------------:|----------------------------------------:|----------------------------|------------------------------------------:|
| Iteration 0 | Human SFT Baseline | 45.2% | 88.5% | High | 0.00 (기준점) |
| Iteration 1 | πθ₀ Rollout + Verify | 52.4% | 86.1% | Medium | +0.22 |
| Iteration 2 | πθ₁ Rollout + Verify | 58.9% | 84.0% | Low | +0.45 |
| Iteration 3 | πθ₂ Rollout + Verify | 64.1% | 82.5% | Minimal (안정) | +0.68 (최적 수렴) |
| Iteration 4 | πθ₃ Rollout + Verify | 63.8% | 71.2% | Minimal | +0.72 (정체 발생) |
| Iteration 5 | πθ₄ Rollout + Verify | 61.2% (붕괴 조짐) | 52.4% (위험) | Minimal | +0.95 (Overfitting) |

> $\pi$는 모델의 토큰 생성 확률 분포(정책)를 뜻하며, 하첨자 $\theta_0$는 0세대(초기 Baseline) 모델의 가중치(파라미터) 상태를 의미, . 즉, $\pi_{\theta_0}$는 '0세대 모델 그 자체'를 학술적으로 표현한 기호로 여기서 'Rollout'은 이 0세대 모델이 프롬프트를 주어 여러개의 답변 문장을 시제로 출력(추론)해 내는 과정을 말하고 여기서 Vefiy는 그렇게 모델이 직접 뱉은 답변들을 컴파일러나 정규식 파서같은 검증기로 채점하여 참/거짓을 가려내는 단계다 결론적으로 $\pi_{\theta_0}$ Rollout + Verify$는 0세대 모델이 스스로 생성한 답변들을 검증기로 필터링하여 통과시킨후 자가데이터셋을 다음 세대의 학습 소스로 활용한것

### 지능 부트스트래핑 구간 Iteration 1 -> 3

초기 3세대까지는 코딩 정확도 Pass@1가 45.2퍼에서 64.1퍼로 선형적으로 성장한다.

그 이유는 Student 모델의 현재 연산 능력 범위 안에서 도출된 다양한 실패 경로들이 상위 Verifier에 의해 필터링되고 성공 경로 위주로 로그 확률이 매핑되면서 스스로 생성한 오답을 수정하고 복구하는 확률공간이 압축되었기 때문이다. 오답 이탈률 Exposure Bias가 극단적으로 완화된다.

### 자가 포화(Self-Saturation) 및 모델 붕괴 한계점 (Iteration 4->5)

4세대를 넘어서면서 정확도 상승이 멈추고 5세대에 이르러서는 61.2퍼로 성능이 역행한다. 이때 Token Diversity 지표는 52.4퍼로 급락한다

그 이유는 새로운 외부 지식 주입없이 자가 데이터를 풀 내에서만 학습이 순환되다 보니 모델이 안전하게 정답을 맞출수있는 특정 코딩 양식과 단어 집합만을 과도하게 재사용 (Mode Collapse) 한 결과다. W&B Reward Margin은 계속 벌어지지만 (수학적 최적화는 진행됨) 언어의 다양성이 훼손되어 벤치마크 성능은 파괴되는 전형적인 오버핏 양상이다.


### 최적화 전략

상용 레벨에서 Iterative Self-Improvement를 설계할때는 무한 루프를 돌려서는 안되며

**토큰의 다양성 지표가 80% 이하로 감소하는 시점을 (본 지표에서는 Iteration 3~4 사이)을 훈련 종료 Early Stopping 임계점으로 설정해야한다.** 

혹은 주기적으로 새로운 인간 지시문이나 외부 데이터소스를 가공하여 주입하는 아키텍처 방어선이 결합되어야 인프라 낭비없는 강건한 에이전트 브레인 확보가 가능하다.