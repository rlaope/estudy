# Policy Distribution과 On-policy / Off-policy 이론

대형 언어 모델의 포스트 트레이닝을 단순히 데이터셋으로 바꾸는 과정으로 이해하면 

아키텍처의 한계와 병목을 제어할 수 없다. 포스트 트레이닝의 본질은 고차원 토큰 시퀀스 공간상에 존재하는 정책 분포 (Policy Distribution)를 우리가 목표로 하는 타겟 분포로 이동시키는 연속적인 정렬 과정이다.

이 문서에는 SFT, Off-policy Distillation, On-policy SFT, DPO, GRPO를 기하학적 분포 이동 관점에서 제 해석하고 데이터의 생성 주체가 모델 내부 손실함수의 역전파 신호에 미치는 수리적 차이를 분석해볼거다.

## 같은 정답이라도 Teacher가 만든 답변과 현재 모델이 만든 답변은 왜 다른 학습 신호인가?

결론부터 말하면, 두 데이터는 **모델의 현재 확률 분포 Support of Current Policy** 내부에 존재하는가 외부에 존재하는가의 물리적 차이를 가진다.

이로 인해 모델 가중치가 업데이트 되는 수학적 방향과 안정성이 완전히 달라진다.

- **Teacher가 만든 정답 Off-policy Signal:** Teacher모델의 분포 $\pi_{teacher}(y|x)$에서 샘플링 된 데이터는 현재 Student 모델의 분포 $\pi_\theta(y|x)$  관점에서 볼때 확률이 0에 가까운 초고차원 희소 영역 Out-of-support일 가능성이 높다. 이 정답을 강제로 모방하게 하면 (Teacher Forcing), 모델은 자신의 내부 전이 확률을 무시하고 해당 궤적(Trajectory)을 강제로 끌어올려야하므로 가중치의 왜곡이 발생해서 추론시경로를 조금만 이탈해도 수습하지 못하는 Exposure Bias에 직면한다
- **현재 모델이 직접 만든 정답 On-policy Signal:** 현재 모델의 분포 $\pi_\theta(y|x)$ 내에서 스스로 탐색 Exploration하여 우연히 맞춘 정답은, 현재 가중치 상태  $\theta$의 기울기(Gradient) 공간 위에 밀접하게 매핑되어 있다. 즉, 모델이 이미 높은 확률분포의 지지 집합(Support)으로 가지고 있던 전이 경로중에서 최적의 경로를 Anchoring 하는 신호다. 이는 새로운 공간을 억지로 외우는것이 아니라 기존 내부 밀도 분포를 압축하는 연산으로 분포의 급격한 변형 없이 안전하고 강건하게 추론 성공률을 고착화 시킨다.

<br>

## 포스트 트레이닝의 7대 핵심 개념

LLM 포스트 트레이닝 사슬을 관통하는 7가지 핵심 이론적 지표와 정의를 알아보자.

### Policy Distribution ($\pi_\theta(y|x)$)

프롬프트 시퀀스 x가 주어졌을때, 자율적으로 생성할 토큰 시퀀스 y에 대해 조건부 확률 분포다. Autoregression 모델 특성상 다음과 같이 정형화 된다.

$$\pi_\theta(y|x) = \prod_{t=1}^T \pi_\theta(y_t | x, y_{<t})$$

### Behavior Policy ($\pi_b$)

학습 단계에서 손실 함수를 계산하기 위해 실제 토큰 궤적 Trajectory를 생성(rollout)해낸 소스 분포를 의미한다.

사람이 만든 데이터셋 이면 $\pi_{human}$, GPT-4가 만든 데이터셋이면 $\pi_{gpt4}$, 모델 자신이 실시간으로 생성하면 $\pi_\theta$ 가 된다.

### Target Policy ($\pi_\theta$)

우리가 손실 함수를 통해 가중치 $\theta$를 최종적으로 업데이트하여 도달하고자 하는 최적화 대상 정책이다.

### Reference Policy ($\pi_{ref}$)

Prefrerence Optimization (DPO등) 또는 PPO 학습시, 모델이 보상을 과도하게 추종하다

언어 능력 자체가 붕괴하는 Reward Hacking을 막기 위하여 잡아두는 기준 분포다.

주로 SFT가 완료된 시점의 고정된 가중치를 활용하여, KL Divergence penalty y($D_{KL}(\pi_\theta || \pi_{ref})$)의 토엦선 역할을 수행한다.

### Distributiopn Shift & Covariate Shift

학습시 모델이 경험한 입력 context 분포 $\pi_b(y_{<t}|x)$와 실제 서비스 추론 환경에서

모델스스로 마주하게 되는 입력 context 분포 $\pi_\theta(y_{<t}|x)$가 불일치하여, 레이어가 거듭될수록 오차가 기하 급수적으로 누적되는 현상이다.

### Exposure Bias

오프라인 학습 SFT/Distillation 환경에서는 항상 이전 타겟 정답 토큰을 입력으로 받는 환경에서만 전이 확률을 학습한다.

그러나 실제 추론 단계에서는 자기가 이전에 잘못 생성한 오답토큰 $\hat{y}_{<t}$를 입력받아 다음 토큰을 생성해야하므로 한 번의 실수가 시퀀스 전체의 붕괴를 초래하는 메커니즘적 부채다.

<br>

## Policy 분포 이동 메커니즘 해석

5대 알고리즘이 있는데 다섯가지 정렬 방법론 "무슨 데이터를 썼는가"를 넘어 "어떤 분포 법칙에 의해 가중치 공간이 이동하는가"를 재해석 해보자

### SFT (Supervised Fine-Tuning)

**기하학적인 메커니즘으로** $\pi_{human}$ 또는 $\pi_{expert}$분포의 고정된 샘플 경로 위로 타겟 정첵  $\pi_\theta$의 밀도를 강제로 정렬시킨다.

**분포 특성**으로 완전한 오프라인 학습이다. 모델 내부의 실제 확률 공간 지형을 고려하지 않고 정답 궤적의 Log-likeihood만을 극대화하므로, 학습 도중 Covariate Shift, Exposure Bias 리스크가 상존한다.

### Off-policy Distillation

**기하학적 메커니즘:** 거대한 상위 모델의 고정된 생성 정책 $\pi_{teacher}$의 출력 분포를 하위 모델  $\pi_\theta$에 이식

**분포 특성은** Behavior Policy가 $\pi_{teacher}$로 고정된 완벽한 Off-policy 구조

Student 모델의 파라미터 capacity 한계로 인해 Teacher 분포 경계면 (Support boundary)를 온전히 모방하지 못해 껍데기만 닮는 Imitation Gap 발생

### On-Policy SFT (Rejection Sampling / RFT)

기하학적인 메커니즘으로 현재 가중치 상태인 $\pi_{\theta_{old}}$에서 온도(Temperature) 커널을 통해 주변 공간을 다중 탐색한 뒤, 검증 필터를 성공 궤적 집합 $\mathcal{D}_{pass} \sim \pi_{\theta_{old}}$ 위로 $\pi_\theta$를 수축

분포 특성은 데이터의 생성 기원이 현재 모델 자가 분포이므로 데이터 획득 관점에서는 on-policy인데 다만 가중치 최적화 수학식 자체는 cross-entropy 오프라인 손실 함수를 사용하므로 과도한 루프 반복시 자가 분포가 특정 국소 기저로 고착되는 mode collapse 위험이 있다.

### DPO (Direct Preference Optimization)

Reference Policy $\pi_{ref}$의 확률 지형을 기준으로, Chosen 경로의 상대적 밀도는 인력(Pull)을 가해 끌어올리고, Rejected 경로의 밀도는 척력 Push를 가해 밀어낸다.

일반적으로 Preference Dataset(오프라인 고정 분포)을 사용하므로 수리적으로는 Off-policy 최적화에 가깝고 이를 보완하기 위해 학습중인 $\pi_\theta$가 직접 생성한 토큰을 기반으로 실시간으로 Chosen/Rejected를 샘플링 하여 DPO를 수행하는 Online DPO 아키텍처로 진화하고 있다.

> RFT와 차이는 RFT는 좋은 답변만 기억하고 나쁜 답변을 버리지만, DPO는 chosen의 확률을 올리고 rejected의 확률을 떨구는 식이라 좋은 답변과 나쁜 답변의 대비를 보는점이 있다.


| 구분    | RFT                           | DPO                      |
| ----- | ----------------------------- | ------------------------ |
| 좋은 답변 | 사용함                           | 사용함                      |
| 나쁜 답변 | 보통 버림                         | 직접 사용함                   |
| 학습 방식 | Good answer를 SFT              | Chosen vs Rejected 비교 학습 |
| 핵심 신호 | pass/fail 또는 reward threshold | pairwise preference      |

### GRPO (Group-Relative Policy Optimazation)

현재 활성화된 온라인 분포  $\pi_\theta$로부터 추출된 $G$개 샘플 간의 상대적 보상 높낮이 지형(Advantage Landscape)을 수학적 경사면으로 치환하여 분포 전체를 동적으로 변형시킨다.

완벽한 순수 온라인 on policy 구조로 현재 가중치가 발동하는 실제 확률과 직접 부딪히며 미세한 가중치 경사면을 깎아 나가기 때문에 Exposure Bias를 그본적으으로 파괴하고 모델 스스로 오류 복구 경로를 획득하게 만드는 원동력이 된다.

<br>

## 분포 관점에서 알고리즘 분류 개념

각 포스트 트레이닝 아키텍처가 VRAM 메모리 위에서 어떤 분포 행렬들을 바인딩하고 가중치 전이를 유도하는지 한눈에 스캔어블하게 정리한 명세다.

```markdown
| 세대 (Generation) | 소스 데이터 출처 (Data Source) | 코딩 벤치마크 (HumanEval Pass@1) | 데이터 단편화/중복도 (Token Diversity) | 오답 이탈률 (Exposure Bias) | W&B 보상 수렴도 (Implicit Reward Margin) |
|------------------|--------------------------------|---------------------------------:|----------------------------------------:|----------------------------|------------------------------------------:|
| Iteration 0 | Human SFT Baseline | 45.2% | 88.5% | High | 0.00 (기준점) |
| Iteration 1 | πθ₀ Rollout + Verify | 52.4% | 86.1% | Medium | +0.22 |
| Iteration 2 | πθ₁ Rollout + Verify | 58.9% | 84.0% | Low | +0.45 |
| Iteration 3 | πθ₂ Rollout + Verify | 64.1% | 82.5% | Minimal (안정) | +0.68 (최적 수렴) |
| Iteration 4 | πθ₃ Rollout + Verify | 63.8% | 71.2% | Minimal | +0.72 (정체 발생) |
| Iteration 5 | πθ₄ Rollout + Verify | 61.2% (붕괴 조짐) | 52.4% (위험) | Minimal | +0.95 (Overfitting) |
```

렌더링 결과:

| 세대 (Generation) | 소스 데이터 출처 (Data Source) | 코딩 벤치마크 (HumanEval Pass@1) | 데이터 단편화/중복도 (Token Diversity) | 오답 이탈률 (Exposure Bias) | W&B 보상 수렴도 (Implicit Reward Margin) |
| --------------- | ----------------------- | -------------------------: | ----------------------------: | ---------------------- | ----------------------------------: |
| Iteration 0     | Human SFT Baseline      |                      45.2% |                         88.5% | High                   |                          0.00 (기준점) |
| Iteration 1     | πθ₀ Rollout + Verify    |                      52.4% |                         86.1% | Medium                 |                               +0.22 |
| Iteration 2     | πθ₁ Rollout + Verify    |                      58.9% |                         84.0% | Low                    |                               +0.45 |
| Iteration 3     | πθ₂ Rollout + Verify    |                      64.1% |                         82.5% | Minimal (안정)           |                       +0.68 (최적 수렴) |
| Iteration 4     | πθ₃ Rollout + Verify    |                      63.8% |                         71.2% | Minimal                |                       +0.72 (정체 발생) |
| Iteration 5     | πθ₄ Rollout + Verify    |              61.2% (붕괴 조짐) |                    52.4% (위험) | Minimal                |                 +0.95 (Overfitting) |


대규모 포스트 트레이닝 클러스터를 설계할때 단순히 좋은 데이터를 수집하는 관점에서 벗어나

하드웨어 예산과 타겟 도메인의 특성에 맞춰 어떤 정책 분포의 인터페이스 통신망을 구축할 것인가

선제적으로 정의해야만 배포 단계에서 성능 분괴와 예산 비효율을 통제가 가능하다.