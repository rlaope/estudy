# RL 학습 커리큘럼

LLM post-training에 쓰이는 강화학습(RL)을 적시학습 방식으로 익히기 위한 커리큘럼입니다. 고전 RL 이론은 PPO, GRPO 같은 현재 알고리즘을 이해하는 데 필요한 만큼만 다룹니다. 목표는 reasoning model과 agent 학습에 쓰이는 RL 파이프라인을 읽고, 구성하고, 디버깅하는 역량입니다. 이론만 익히면 reward hacking이나 학습 불안정 같은 실제 문제를 해석하기 어렵기 때문에, 단계마다 완료 기준을 둡니다.

### 1. LLM 관점의 RL 기초와 policy gradient

LLM RL 논문과 코드를 읽기 위한 최소 기반입니다. LLM의 응답 생성을 MDP로 정의하고, 미분할 수 없는 reward로 모델을 학습시키는 원리인 policy gradient를 유도한 뒤, 작은 LM에 REINFORCE를 직접 구현합니다. PPO, GRPO, DPO는 모두 이 단계의 식을 변형한 것입니다. 따라서 advantage, baseline, importance sampling을 정확히 이해하지 못하면 이후 단계에서 loss 구현과 학습 로그를 해석하기 어렵습니다.

- 완료 기준: token 단위 log-prob로 policy gradient loss를 작성하고 advantage의 의미를 설명할 수 있음
- 가볍게 넘길 내용: tabular 방법(dynamic programming, Q-learning), DQN, 수렴 증명
- 예상 기간: 1주

#### 1.1 수식 기호와 읽는 법

이 단계에서 쓰는 기호는 아래 표와 같습니다. 이후 단계의 논문도 대부분 같은 표기를 씁니다.

| 기호 | 읽는 법 | 의미 |
|---|---|---|
| $s_t$ | 에스 티 | 시점 $t$의 state |
| $a_t$ | 에이 티 | 시점 $t$에 고른 action |
| $r_t$ | 알 티 | $a_t$를 고른 직후 받은 reward |
| $\theta$ | 세타 | 모델 parameter 전체 |
| $\pi_\theta(a \mid s)$ | 파이 세타, s가 주어졌을 때 a | parameter가 $\theta$인 policy가 state $s$에서 action $a$를 고를 확률 |
| $\tau$ | 타우 | trajectory, $(s_0, a_0, r_0, s_1, a_1, r_1, \dots)$ 한 줄 |
| $\tau \sim \pi_\theta$ | 타우는 파이 세타를 따름 | 현재 policy로 trajectory를 샘플링함 |
| $\gamma$ | 감마 | discount factor, 0 이상 1 이하 |
| $G_t$ | 지 티 | 시점 $t$부터 끝까지의 return |
| $R(\tau)$ | 알 오브 타우 | trajectory 전체의 return |
| $V^\pi(s)$ | 브이 파이 오브 에스 | state value function |
| $Q^\pi(s, a)$ | 큐 파이 오브 에스 에이 | action value function |
| $A^\pi(s, a)$ | 에이 파이 오브 에스 에이 | advantage |
| $J(\theta)$ | 제이 오브 세타 | 최적화 목표, 기대 return |
| $\mathbb{E}_{x \sim p}[f(x)]$ | x가 p를 따를 때 f of x의 기댓값 | $p$에서 샘플링한 $x$로 계산한 $f(x)$의 평균 |
| $\nabla_\theta f$ | 나블라 세타 f, f의 세타에 대한 gradient | $f$를 $\theta$의 각 원소로 편미분해 모은 vector |
| $\log$ | 로그 | 자연로그 |
| $\sum_{t=0}^{T-1}$ | 시그마, t는 0부터 T-1까지 | $t$를 바꿔 가며 더함 |
| $\prod_{t=1}^{T}$ | 프로덕트, t는 1부터 T까지 | $t$를 바꿔 가며 곱함 |
| $y_{<t}$ | 와이 t 미만 | $t$번째 이전에 생성된 token 전체 |

수식을 처음 읽을 때는 세 가지를 먼저 확인합니다.

- $\mathbb{E}$의 아래첨자: 어떤 분포에서 샘플링한 데이터로 평균을 내는가
- $\nabla$의 아래첨자: 어떤 parameter로 미분하는가
- $\sum$의 index: token, 응답, batch 중 무엇을 기준으로 더하는가

아래첨자 $t$는 시점이고, 위첨자 $i$는 샘플 번호입니다. $a_t^i$는 "$i$번째 샘플의 $t$번째 action"이며 거듭제곱이 아닙니다. 반면 $\gamma^{k-t}$처럼 숫자에 붙은 위첨자는 거듭제곱입니다.

#### 1.2 MDP와 LLM 생성의 매핑

**MDP**(Markov Decision Process)는 agent가 state를 보고 action을 고르면 environment가 다음 state와 reward를 돌려주는 순차적 의사결정 문제의 표준 형식입니다. 다섯 요소 $(\mathcal{S}, \mathcal{A}, P, r, \gamma)$로 정의합니다.

- $\mathcal{S}$, $\mathcal{A}$: state 집합과 action 집합
- $P(s' \mid s, a)$: state $s$에서 action $a$를 했을 때 다음 state가 $s'$일 확률(transition)
- $r(s, a)$: reward 함수
- $\gamma$: 미래 reward를 얼마나 할인할지 정하는 discount factor

MDP의 전제는 Markov property입니다.

$$
P(s_{t+1} \mid s_t, a_t, s_{t-1}, a_{t-1}, \dots, s_0) = P(s_{t+1} \mid s_t, a_t)
$$

> 읽는 법: 다음 state $s_{t+1}$의 확률은 과거 이력 전체가 아니라 현재 state $s_t$와 action $a_t$만으로 결정됩니다.

LLM의 응답 생성은 다음과 같이 MDP로 옮깁니다. LLM 문맥에서는 token index를 1부터 셉니다.

| MDP 요소 | LLM 생성에서의 의미 | 비고 |
|---|---|---|
| state $s_t$ | prompt $x$와 지금까지 생성한 token $y_{<t}$ | prefix 전체가 state이므로 Markov property가 정의상 성립 |
| action $a_t$ | 다음 token $y_t$ | action space는 vocabulary 전체(수만~수십만 개) |
| transition $P$ | $s_{t+1} = (x, y_{<t}, y_t)$, 이어 붙이기 | 결정적(deterministic)이며 확률 요소 없음 |
| reward $r_t$ | 마지막 token에서만 응답 점수, 나머지 token은 0 | sparse reward |
| episode 종료 | EOS token 생성 또는 최대 길이 도달 | |
| policy $\pi_\theta$ | LM의 next-token 분포(logits의 softmax) | |
| $\gamma$ | 보통 1 | 응답 길이가 유한하므로 할인하지 않음 |

응답 $y = (y_1, \dots, y_T)$ 전체의 확률은 token 확률의 곱입니다.

$$
\pi_\theta(y \mid x) = \prod_{t=1}^{T} \pi_\theta(y_t \mid x, y_{<t})
$$

$$
\log \pi_\theta(y \mid x) = \sum_{t=1}^{T} \log \pi_\theta(y_t \mid x, y_{<t})
$$

> 읽는 법: prompt $x$가 주어졌을 때 응답 $y$가 나올 확률은, 각 시점 $t$에서 앞선 token들을 보고 $y_t$를 고를 확률을 처음부터 끝까지 곱한 값입니다. 양변에 log를 취하면 곱이 합으로 바뀝니다.

log 형태를 쓰는 이유는 다음과 같습니다.

- 1보다 작은 확률을 수백 번 곱하면 underflow가 생기지만 log의 합은 안정적
- 곱의 미분보다 합의 미분이 단순
- 모델 출력에 `log_softmax`를 적용하면 바로 얻을 수 있음

예시로 prompt "Name a fruit."에 "Banana."라고 답하고, 응답이 과일 이름이면 reward 1을 주는 경우를 MDP로 쓰면 다음과 같습니다. token 분할은 설명을 위한 예시입니다.

| $t$ | state $s_t$ | action $a_t = y_t$ | reward $r_t$ |
|---|---|---|---|
| 1 | $x$ | "Ban" | 0 |
| 2 | $x$ + "Ban" | "ana" | 0 |
| 3 | $x$ + "Banana" | "." | 0 |
| 4 | $x$ + "Banana." | EOS | 1 |

LLM RL에는 두 가지 관점이 함께 쓰입니다.

| 관점 | action 단위 | advantage 단위 | 대표 알고리즘 |
|---|---|---|---|
| token 단위 MDP | token 하나 | token마다 다를 수 있음 | PPO(critic이 token별 value를 추정) |
| 응답 단위 contextual bandit | 응답 전체 | 응답 하나에 하나, 모든 token에 같은 값 적용 | REINFORCE, RLOO, GRPO |

contextual bandit은 context(prompt)를 보고 action(응답)을 한 번 고른 뒤 reward를 받고 끝나는 문제입니다. $\log \pi_\theta(y \mid x)$가 token log-prob의 합이므로 두 관점은 같은 gradient 식으로 연결됩니다.

#### 1.3 Return, value function, advantage

**return**은 특정 시점부터 episode 끝까지 받은 reward의 할인 합입니다.

$$
G_t = \sum_{k=t}^{T-1} \gamma^{k-t} r_k = r_t + \gamma r_{t+1} + \gamma^2 r_{t+2} + \cdots
$$

> 읽는 법: $G_t$는 시점 $t$부터 끝까지 받은 reward를 더하되, $k$번째 reward에는 $\gamma$를 $k-t$번 곱해 먼 미래일수록 작게 반영한 값입니다. trajectory 전체 return은 $R(\tau) = G_0$입니다.

LLM에서는 reward가 마지막 token에만 있고 $\gamma = 1$이므로 모든 시점의 return이 응답 점수와 같습니다.

$$
G_t = R(x, y) \quad (t = 1, \dots, T)
$$

응답 안의 모든 token이 같은 return을 받는다는 뜻입니다. 어떤 token이 결과에 기여했는지 구분하지 못하는 문제를 credit assignment 문제라고 하며, PPO의 critic과 GAE, process reward가 이 문제를 다룹니다.

**value function**은 policy $\pi$를 따를 때 기대되는 return입니다.

$$
V^\pi(s) = \mathbb{E}_{\tau \sim \pi}\left[G_t \mid s_t = s\right]
$$

$$
Q^\pi(s, a) = \mathbb{E}_{\tau \sim \pi}\left[G_t \mid s_t = s,\ a_t = a\right]
$$

> 읽는 법: $V^\pi(s)$는 state $s$에서 출발해 policy $\pi$대로 끝까지 진행했을 때의 평균 return입니다. $Q^\pi(s, a)$는 state $s$에서 먼저 action $a$를 고르고, 그 뒤로는 $\pi$대로 진행했을 때의 평균 return입니다.

**advantage**는 특정 action이 그 state에서의 평균보다 얼마나 나은지를 나타내는 값입니다.

$$
A^\pi(s, a) = Q^\pi(s, a) - V^\pi(s)
$$

$A > 0$이면 평균보다 나은 action이므로 확률을 올리고, $A < 0$이면 확률을 내립니다. policy를 따를 때 advantage의 기댓값은 항상 0입니다($\mathbb{E}_{a \sim \pi}[A^\pi(s, a)] = 0$).

예시로 어떤 수학 prompt에 대한 현재 모델의 정답률이 30%이고 reward가 정답 1, 오답 0이면 $V(x) = 0.3$입니다.

| 응답 | reward $R$ | advantage $R - V(x)$ | 학습 방향 |
|---|---|---|---|
| 정답 | 1 | +0.7 | 확률 증가 |
| 오답 | 0 | -0.3 | 확률 감소 |

advantage의 기댓값은 $0.3 \times 0.7 + 0.7 \times (-0.3) = 0$입니다. 같은 prompt에 응답 여러 개를 샘플링해 평균 reward로 $V(x)$를 추정하는 방식이 GRPO와 RLOO의 핵심입니다.

**Bellman equation**은 현재 state의 value를 즉시 reward와 다음 state의 value로 재귀적으로 표현한 식입니다.

$$
V^\pi(s) = \mathbb{E}_{a \sim \pi(\cdot \mid s),\ s' \sim P(\cdot \mid s, a)}\left[r(s, a) + \gamma V^\pi(s')\right]
$$

> 읽는 법: 지금 state의 value는, policy로 action을 하나 고르고 environment가 다음 state를 정했을 때 받는 즉시 reward와 다음 state value의 할인 값을 더한 것의 평균입니다.

이 식에서 TD error가 나옵니다.

$$
\delta_t = r_t + \gamma V(s_{t+1}) - V(s_t)
$$

$\delta_t$는 "한 step 진행해 보니 예상보다 얼마나 좋았는가"를 나타내며 advantage의 한 step 추정값입니다. critic을 학습하고 GAE로 token별 advantage를 계산할 때 쓰입니다. LLM 생성에서는 중간 token의 reward가 0이고 transition이 결정적이므로, 중간 시점의 $\delta_t$는 $V(s_{t+1}) - V(s_t)$, 즉 token 하나로 인한 value 변화량입니다.

#### 1.4 Policy gradient theorem과 log-derivative trick

**policy gradient**는 기대 return $J(\theta)$를 policy parameter $\theta$로 미분한 gradient이며, 이 방향으로 parameter를 갱신해 policy를 개선합니다.

$$
J(\theta) = \mathbb{E}_{\tau \sim \pi_\theta}\left[R(\tau)\right]
$$

LLM에서는 다음과 같이 씁니다.

$$
J(\theta) = \mathbb{E}_{x \sim \mathcal{D},\ y \sim \pi_\theta(\cdot \mid x)}\left[R(x, y)\right]
$$

> 읽는 법: 데이터셋 $\mathcal{D}$에서 prompt $x$를 뽑고, 현재 모델로 응답 $y$를 샘플링했을 때 받는 reward의 평균입니다.

이 식은 그대로 미분할 수 없습니다.

- $\theta$가 reward 함수 안이 아니라 샘플링 분포 $\pi_\theta$에 들어 있음
- reward는 문자열 비교, unit test 결과처럼 미분할 수 없는 함수인 경우가 대부분
- token 샘플링은 이산적이라 backpropagation이 통과하지 못함

**log-derivative trick**은 log 함수의 미분 규칙을 이용해 확률의 gradient를 "확률 × log 확률의 gradient"로 바꾸는 기법입니다.

$$
\nabla_\theta \log p_\theta(x) = \frac{\nabla_\theta p_\theta(x)}{p_\theta(x)}
\quad \Longrightarrow \quad
\nabla_\theta p_\theta(x) = p_\theta(x)\, \nabla_\theta \log p_\theta(x)
$$

> 읽는 법: $\log u$의 미분은 $1/u$이므로 chain rule로 왼쪽 식이 나오고, 양변에 $p_\theta(x)$를 곱하면 오른쪽 식이 됩니다.

이 trick을 $J(\theta)$에 적용합니다.

$$
\begin{aligned}
\nabla_\theta J(\theta)
&= \nabla_\theta \sum_{\tau} P_\theta(\tau)\, R(\tau) \\
&= \sum_{\tau} \nabla_\theta P_\theta(\tau)\, R(\tau) \\
&= \sum_{\tau} P_\theta(\tau)\, \nabla_\theta \log P_\theta(\tau)\, R(\tau) \\
&= \mathbb{E}_{\tau \sim \pi_\theta}\left[\nabla_\theta \log P_\theta(\tau)\, R(\tau)\right]
\end{aligned}
$$

각 줄의 의미는 다음과 같습니다.

1. 기댓값을 가능한 모든 trajectory에 대한 확률 가중합으로 풀어 씁니다.
2. $R(\tau)$는 $\theta$와 무관하므로 gradient는 $P_\theta(\tau)$에만 걸립니다.
3. log-derivative trick으로 $\nabla_\theta P_\theta$를 $P_\theta \nabla_\theta \log P_\theta$로 바꿉니다.
4. $P_\theta(\tau)$로 가중한 합은 다시 기댓값이므로, 샘플링한 trajectory의 평균으로 추정할 수 있습니다.

모든 trajectory를 나열할 수 없어도, 현재 policy로 샘플을 뽑아 평균을 내면 gradient를 추정할 수 있다는 것이 마지막 줄의 의미입니다.

trajectory 확률은 environment 부분과 policy 부분의 곱입니다.

$$
P_\theta(\tau) = \rho_0(s_0) \prod_{t=0}^{T-1} P(s_{t+1} \mid s_t, a_t)\, \pi_\theta(a_t \mid s_t)
$$

log를 취하고 $\theta$로 미분하면 초기 state 분포 $\rho_0$와 transition $P$는 $\theta$와 무관하므로 사라집니다.

$$
\nabla_\theta \log P_\theta(\tau) = \sum_{t=0}^{T-1} \nabla_\theta \log \pi_\theta(a_t \mid s_t)
$$

environment의 transition을 몰라도 gradient를 계산할 수 있으며, 이 성질 때문에 policy gradient는 model-free 방법입니다. 두 식을 합치면 다음 결과를 얻습니다.

$$
\nabla_\theta J(\theta) = \mathbb{E}_{\tau \sim \pi_\theta}\left[\sum_{t=0}^{T-1} \nabla_\theta \log \pi_\theta(a_t \mid s_t)\, R(\tau)\right]
$$

> 읽는 법: $J$의 $\theta$에 대한 gradient는, 현재 policy로 trajectory를 샘플링했을 때 각 시점 action의 log 확률 gradient에 return을 곱해 모두 더한 값의 기댓값입니다.

SFT와 비교하면 의미가 분명해집니다.

| 구분 | gradient | 데이터 출처 | 가중치 |
|---|---|---|---|
| SFT | $\nabla_\theta \log \pi_\theta(y^* \mid x)$ | 정답 label $y^*$ | 모두 1 |
| policy gradient | $\nabla_\theta \log \pi_\theta(y \mid x)\, R(x, y)$ | 모델이 직접 샘플링한 $y$ | reward |

policy gradient는 "모델이 직접 생성한 응답에 reward로 가중치를 준 maximum likelihood"입니다. reward가 양수인 응답은 확률이 올라가고, 음수인 응답은 내려가고, 0인 응답은 영향이 없습니다.

Sutton & Barto의 policy gradient theorem은 같은 내용을 state 방문 분포와 $Q$로 표현한 일반형입니다.

$$
\nabla_\theta J(\theta) \propto \sum_{s} \mu(s) \sum_{a} Q^\pi(s, a)\, \nabla_\theta \pi_\theta(a \mid s)
$$

> 읽는 법: $\mu(s)$는 policy를 따를 때 state $s$에 머무는 비율이고, $\propto$는 "비례한다"로 읽습니다. 자주 방문하는 state에서 $Q$가 큰 action의 확률을 올리는 방향이 gradient입니다.

실제 구현에서는 $R(\tau)$ 자리에 분산이 더 작은 값 $\Psi_t$(프사이 티)를 넣습니다.

$$
\nabla_\theta J(\theta) = \mathbb{E}_{\tau \sim \pi_\theta}\left[\sum_{t=0}^{T-1} \nabla_\theta \log \pi_\theta(a_t \mid s_t)\, \Psi_t\right]
$$

| $\Psi_t$ | 이름 | 특징 |
|---|---|---|
| $R(\tau)$ | 전체 return | 가장 단순, 분산 가장 큼 |
| $G_t$ | reward-to-go | action은 과거 reward에 영향을 줄 수 없으므로 이후 reward만 사용, 분산 감소 |
| $G_t - b(s_t)$ | baseline 적용 | REINFORCE with baseline, 1.6에서 다룸 |
| $A^\pi(s_t, a_t)$ | advantage | 분산이 가장 작지만 추정이 필요, Actor-Critic과 GAE |

LLM처럼 reward가 마지막에만 있으면 $R(\tau)$와 $G_t$가 같으므로 reward-to-go로 얻는 이득은 없습니다.

#### 1.5 REINFORCE와 surrogate loss

**REINFORCE**는 policy gradient의 기댓값을 Monte Carlo 샘플 평균으로 추정해 parameter를 갱신하는 가장 기본적인 알고리즘입니다.

$$
\hat{g} = \frac{1}{N} \sum_{i=1}^{N} \sum_{t=0}^{T_i - 1} \nabla_\theta \log \pi_\theta(a_t^i \mid s_t^i)\, G_t^i
$$

> 읽는 법: $\hat{g}$(지 햇)은 gradient의 추정값입니다. trajectory $N$개를 샘플링하고, $i$번째 trajectory의 각 시점마다 log 확률 gradient에 return을 곱해 더한 뒤 $N$으로 나눠 평균을 냅니다. $T_i$는 $i$번째 trajectory의 길이입니다.

동작 순서는 다음과 같습니다.

1. 현재 policy $\pi_\theta$로 trajectory $N$개를 샘플링
2. 각 trajectory의 reward와 return 계산
3. $\hat{g}$ 계산
4. gradient ascent로 갱신: $\theta \leftarrow \theta + \alpha \hat{g}$ ($\alpha$는 learning rate)
5. 사용한 샘플은 버리고 1번으로 돌아감

PyTorch optimizer는 loss를 최소화하므로, gradient가 $-\hat{g}$가 되는 **surrogate loss**(대리 loss)를 만들어 `backward()`를 호출합니다. surrogate loss는 값 자체에는 의미가 없고 gradient만 원하는 값과 일치하도록 만든 함수입니다.

$$
\mathcal{L}(\theta) = -\frac{1}{N} \sum_{i=1}^{N} \sum_{t=0}^{T_i - 1} \log \pi_\theta(a_t^i \mid s_t^i)\ \mathrm{sg}\!\left[G_t^i\right]
$$

> 읽는 법: $\mathrm{sg}[\cdot]$는 stop-gradient로, 괄호 안의 값을 상수로 취급해 gradient를 흘리지 않는다는 뜻입니다. 앞의 마이너스는 최대화 문제를 최소화 문제로 바꾸기 위한 부호입니다.

LLM에서는 응답 단위 advantage $A_i$를 모든 token에 곱합니다.

$$
\mathcal{L}(\theta) = -\frac{1}{N} \sum_{i=1}^{N} A_i \sum_{t=1}^{T_i} \log \pi_\theta(y_t^i \mid x, y_{<t}^i)
$$

baseline이 없으면 $A_i = R_i$이고, baseline이 있으면 $A_i = R_i - b$입니다. 핵심 코드는 두 줄입니다.

```python
# seq_logp: 응답별 log pi(y|x) = token log-prob의 합, [N]
# adv: 응답별 advantage, [N]
loss = -(seq_logp * adv.detach()).mean()
loss.backward()  # gradient는 -g_hat, optimizer가 반대 방향으로 이동해 J가 증가
```

surrogate loss를 다룰 때 주의할 점은 다음과 같습니다.

- loss 값은 성능 지표가 아님: 매 step 데이터가 새로 샘플링되므로 loss가 올라가도 성능은 좋아질 수 있음, 성능은 reward로 판단
- advantage에는 `detach()` 적용: reward나 baseline 쪽으로 gradient가 흐르면 다른 objective가 됨
- aggregation 방식: token log-prob를 응답별로 합한 뒤 batch 평균을 내야 위 식과 일치함, 전체 token 수로 나누면 응답 길이에 따른 가중치가 달라지며 PPO와 GRPO 계열의 loss aggregation 주제로 이어짐

#### 1.6 Baseline과 variance reduction

**baseline**은 policy gradient 식에서 return에서 빼 주는, action에 의존하지 않는 값 $b(s_t)$입니다. 기대 gradient를 바꾸지 않으면서 추정값의 분산을 줄입니다.

$$
\nabla_\theta J(\theta) = \mathbb{E}_{\tau \sim \pi_\theta}\left[\sum_{t=0}^{T-1} \nabla_\theta \log \pi_\theta(a_t \mid s_t)\, \big(G_t - b(s_t)\big)\right]
$$

baseline이 bias를 만들지 않는 이유는 log 확률 gradient의 기댓값이 0이기 때문입니다.

$$
\mathbb{E}_{a \sim \pi_\theta(\cdot \mid s)}\left[\nabla_\theta \log \pi_\theta(a \mid s)\right]
= \sum_{a} \pi_\theta(a \mid s)\, \frac{\nabla_\theta \pi_\theta(a \mid s)}{\pi_\theta(a \mid s)}
= \nabla_\theta \sum_{a} \pi_\theta(a \mid s)
= \nabla_\theta 1
= 0
$$

> 읽는 법: 모든 action의 확률을 더하면 항상 1이고, 상수 1의 gradient는 0입니다. 따라서 $b(s)$처럼 action과 무관한 값에 이 gradient를 곱한 항의 기댓값도 0이 됩니다.

Spinning Up에서는 이 성질을 EGLP lemma(Expected Grad-Log-Prob lemma)라고 부릅니다. $b$가 현재 샘플의 action에 의존하면 이 증명은 성립하지 않습니다.

분산이 줄어드는 이유는 숫자로 보면 분명합니다. 정답 1, 오답 0인 reward에 상수 10을 더해 정답 11, 오답 10으로 바꾸면 기대 gradient는 그대로지만 다음 문제가 생깁니다.

- 모든 응답이 큰 양수 가중치를 받아, 샘플된 응답의 확률을 일제히 올리려는 방향이 섞임
- 이 방향은 기댓값으로는 상쇄되지만 샘플 수가 유한하면 상쇄되지 않고 noise로 남음
- 정보가 담긴 차이(11과 10의 차이 1)가 noise에 묻힘

평균 reward를 baseline으로 빼면 가중치가 0 근처로 모여 정답은 양수, 오답은 음수가 되고 필요한 신호만 남습니다. 분산을 가장 많이 줄이는 baseline은 $V^\pi(s_t)$에 가까우며, 이때 $G_t - V^\pi(s_t)$는 advantage의 추정값이 됩니다.

LLM RL에서 쓰는 baseline은 다음과 같습니다.

| baseline | advantage 계산 | 장점 | 단점 | 이어지는 알고리즘 |
|---|---|---|---|---|
| 없음 | $A_i = R_i$ | 구현 단순 | 분산 큼, reward offset에 민감 | REINFORCE |
| batch 평균 | $A_i = R_i - \frac{1}{N}\sum_{j} R_j$ | 추가 비용 없음 | prompt 난이도가 섞임, 자기 reward를 포함 | |
| leave-one-out | $A_i = R_i - \frac{1}{N-1}\sum_{j \ne i} R_j$ | 자기 샘플을 제외하므로 엄밀히 unbiased | prompt당 여러 샘플 필요 | RLOO |
| prompt별 group 평균 | 같은 prompt 응답들의 평균을 빼고 표준편차로 나눔 | prompt 난이도 보정 | prompt당 $G$개 샘플 생성 비용 | GRPO |
| 학습한 critic | $V_\phi(s_t)$로 token별 advantage 계산 | token 단위 advantage 가능 | 모델이 하나 더 필요, 학습 불안정 | PPO |

batch 평균 baseline은 자기 reward를 포함하므로 엄밀히는 action에 의존합니다. 하지만 식을 정리하면 leave-one-out에 상수를 곱한 형태라 gradient 방향은 같고 크기만 $\frac{N-1}{N}$배가 됩니다.

$$
R_i - \frac{1}{N}\sum_{j} R_j = \frac{N-1}{N}\left(R_i - \frac{1}{N-1}\sum_{j \ne i} R_j\right)
$$

baseline을 쓰면 batch 안의 reward가 모두 같을 때(전부 0이거나 전부 1) advantage가 모두 0이 되어 학습 신호가 사라집니다. 너무 쉽거나 너무 어려운 prompt가 학습에 기여하지 못하는 현상이며, RLVR에서 난이도 필터링을 하는 이유입니다.

#### 1.7 On-policy, off-policy, importance sampling

- **on-policy**: 학습에 쓰는 데이터가 지금 업데이트하려는 policy $\pi_\theta$에서 샘플링된 경우
- **off-policy**: 데이터를 샘플링한 behavior policy와 학습하려는 policy가 다른 경우

policy gradient 식은 $\tau \sim \pi_\theta$를 전제로 하므로 REINFORCE는 on-policy입니다. 한 번 업데이트하면 $\theta$가 바뀌어 기존 샘플은 이전 policy $\pi_{\theta_\text{old}}$의 데이터가 되고, 이를 그대로 다시 쓰면 gradient에 bias가 생깁니다.

**importance sampling**은 분포 $q$에서 뽑은 샘플로 분포 $p$에 대한 기댓값을 추정하는 기법입니다.

$$
\mathbb{E}_{x \sim p}\left[f(x)\right]
= \sum_{x} p(x)\, f(x)
= \sum_{x} q(x)\, \frac{p(x)}{q(x)}\, f(x)
= \mathbb{E}_{x \sim q}\left[\frac{p(x)}{q(x)}\, f(x)\right]
$$

> 읽는 법: $q$에서 뽑은 샘플에 $p(x)/q(x)$라는 가중치를 곱해 평균을 내면 $p$에서 뽑은 것과 같은 기댓값이 됩니다. $p(x) > 0$인 곳에서는 $q(x) > 0$이어야 성립합니다.

LLM RL에서는 token 단위 확률 비율을 씁니다.

$$
\rho_t(\theta) = \frac{\pi_\theta(a_t \mid s_t)}{\pi_{\theta_\text{old}}(a_t \mid s_t)} = \exp\Big(\log \pi_\theta(a_t \mid s_t) - \log \pi_{\theta_\text{old}}(a_t \mid s_t)\Big)
$$

$$
\mathcal{L}^{\text{IS}}(\theta) = -\,\mathbb{E}_{\tau \sim \pi_{\theta_\text{old}}}\left[\sum_{t} \rho_t(\theta)\, A_t\right]
$$

> 읽는 법: $\rho_t$(로 티)는 현재 policy가 그 token을 고를 확률이 샘플링 당시의 몇 배인지 나타냅니다. 이전 policy로 만든 데이터에 이 비율을 곱해 현재 policy의 objective를 근사합니다. 구현에서는 log-prob 차이에 exp를 취해 계산합니다.

$\theta = \theta_\text{old}$인 지점에서는 이 loss의 gradient가 REINFORCE와 정확히 같습니다.

$$
\nabla_\theta \rho_t = \rho_t\, \nabla_\theta \log \pi_\theta(a_t \mid s_t), \qquad \rho_t\big|_{\theta = \theta_\text{old}} = 1
$$

REINFORCE는 $\rho_t = 1$인 특수한 경우이고, PPO와 GRPO의 loss는 여기에 clipping과 KL 제약을 더한 형태입니다.

이 식은 state 분포가 바뀌는 효과를 무시하고 token 비율만 쓰는 근사입니다. 응답 전체의 정확한 비율은 token 비율의 곱인데, 응답이 길면 극단적으로 커지거나 작아집니다.

| token당 비율 | 500 token의 곱 |
|---|---|
| 1.01 | 약 145 |
| 0.99 | 약 0.0066 |

그래서 실제 알고리즘은 token 단위 비율을 쓰고, $\pi_\theta$가 $\pi_{\theta_\text{old}}$에서 멀어지지 않도록 clipping과 KL로 제한합니다.

LLM RL에서 off-policy 상황이 생기는 경우는 다음과 같습니다.

| 상황 | 원인 | 대응 |
|---|---|---|
| rollout 한 batch로 여러 mini-batch 또는 epoch 업데이트 | 첫 업데이트 이후 $\pi_\theta \ne \pi_{\theta_\text{old}}$ | ratio와 clipping(PPO) |
| async RL | rollout 생성 중 trainer가 weight를 갱신해 데이터가 stale해짐 | staleness 제한, importance sampling 보정(6단계) |
| train-inference mismatch | vLLM 같은 inference engine과 trainer의 kernel, precision 차이로 log-prob가 다름 | trainer에서 log-prob 재계산, importance sampling 보정(6단계) |
| sampling 설정 | top-p, top-k, 1이 아닌 temperature는 샘플 분포를 $\pi_\theta$와 다르게 만듦 | rollout은 temperature 1, top-p 1, top-k 해제 |

#### 1.8 실습: 작은 LM에 REINFORCE 구현

가상 환경은 다음과 같다고 가정합니다.

- Apple Silicon Mac(MPS) 또는 단일 NVIDIA GPU, Python 3.11
- 모델: `HuggingFaceTB/SmolLM2-135M-Instruct`(약 135M parameter, CPU에서도 느리게 동작)
- 과제: prompt "Write one short sentence about a fruit."의 응답에 "banana"가 있으면 reward 1, 없으면 0

환경 구성 명령은 다음과 같습니다.

```bash
mkdir rl-stage1 && cd rl-stage1
python3 -m venv .venv
source .venv/bin/activate
pip install torch transformers
python -c "import torch; print(torch.__version__, torch.backends.mps.is_available())"
```

```text
2.9.0 True
```

파일 구성은 다음과 같습니다.

```text
rl-stage1/
├── common.py    # reward, rollout, log-prob, loss
├── train.py     # REINFORCE 학습 루프
└── gradvar.py   # baseline별 gradient 분산 측정(1.9)
```

`common.py`에는 rollout, mask, log-prob, advantage, loss를 둡니다.

```python
# common.py
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer

MODEL = "HuggingFaceTB/SmolLM2-135M-Instruct"
PROMPT = "Write one short sentence about a fruit."
KEYWORD = "banana"

def get_device():
    if torch.cuda.is_available():
        return "cuda"
    if torch.backends.mps.is_available():
        return "mps"
    return "cpu"

def load():
    tok = AutoTokenizer.from_pretrained(MODEL)
    if tok.pad_token is None:
        tok.pad_token = tok.eos_token
    model = AutoModelForCausalLM.from_pretrained(MODEL).to(get_device(), dtype=torch.float32)
    model.eval()  # dropout만 끄고 gradient 계산은 그대로 동작
    return tok, model

def reward_fn(text: str) -> float:
    # 응답에 keyword가 있으면 1, 없으면 0
    return 1.0 if KEYWORD in text.lower() else 0.0

@torch.no_grad()
def rollout(model, tok, n, max_new_tokens=48):
    # 같은 prompt를 n번 복제하므로 prompt 길이가 같아 padding이 필요 없음
    msgs = [{"role": "user", "content": PROMPT}]
    text = tok.apply_chat_template(msgs, tokenize=False, add_generation_prompt=True)
    enc = tok([text] * n, return_tensors="pt", add_special_tokens=False).to(model.device)
    seqs = model.generate(
        **enc,
        do_sample=True,
        temperature=1.0, top_k=0, top_p=1.0,  # 샘플 분포를 pi_theta와 일치시킴
        max_new_tokens=max_new_tokens,
        pad_token_id=tok.pad_token_id,
    )
    return seqs, enc["input_ids"].shape[1]

def response_mask(seqs, prompt_len, eos_id):
    # 응답 구간에서 첫 EOS까지만 1, 그 뒤 padding은 0
    resp = seqs[:, prompt_len:]
    is_eos = (resp == eos_id).long()
    after_eos = (is_eos.cumsum(dim=1) - is_eos) > 0
    return (~after_eos).float()  # [N, T_resp]

def response_logprobs(model, seqs, prompt_len):
    logits = model(seqs).logits[:, :-1, :]  # 위치 t의 logits는 t+1번째 token을 예측
    targets = seqs[:, 1:]
    logp = torch.log_softmax(logits.float(), dim=-1)
    token_logp = logp.gather(-1, targets.unsqueeze(-1)).squeeze(-1)
    return token_logp[:, prompt_len - 1:]  # 응답 token 위치만 남김, [N, T_resp]

def compute_advantages(rewards, baseline):
    if baseline == "none":
        return rewards
    if baseline == "mean":
        return rewards - rewards.mean()
    if baseline == "loo":  # leave-one-out
        n = rewards.numel()
        return rewards - (rewards.sum() - rewards) / (n - 1)
    raise ValueError(baseline)

def reinforce_loss(token_logp, mask, adv):
    seq_logp = (token_logp * mask).sum(dim=1)  # 응답별 log pi(y|x)
    return -(seq_logp * adv.detach()).mean()
```

`response_logprobs`에서 확인할 부분은 다음과 같습니다.

- 한 칸 shift: causal LM에서 위치 $t$의 출력은 $t+1$번째 token의 분포이므로 `logits[:, :-1]`과 `seqs[:, 1:]`을 맞춤
- `prompt_len - 1`부터 자르는 이유: shift 후 index $j$는 원래 sequence의 $j+1$번째 token을 가리키므로, 응답 첫 token(원래 index `prompt_len`)은 shift 후 index `prompt_len - 1`
- `logits.float()`: bf16에서 `log_softmax`를 계산하면 정밀도 손실이 큼
- forward 재수행: `generate`는 gradient graph를 남기지 않으므로 같은 sequence로 forward를 다시 수행

`train.py`는 1.5의 동작 순서를 그대로 구현합니다.

```python
# train.py
import argparse
import torch
from common import (load, reward_fn, rollout, response_mask, response_logprobs,
                    compute_advantages, reinforce_loss)

def main():
    p = argparse.ArgumentParser()
    p.add_argument("--baseline", choices=["none", "mean", "loo"], default="mean")
    p.add_argument("--reward-offset", type=float, default=0.0)
    p.add_argument("--steps", type=int, default=100)
    p.add_argument("--n", type=int, default=16)  # step당 응답 수
    p.add_argument("--lr", type=float, default=1e-5)
    p.add_argument("--seed", type=int, default=0)
    args = p.parse_args()

    torch.manual_seed(args.seed)
    tok, model = load()
    opt = torch.optim.AdamW(model.parameters(), lr=args.lr)

    for step in range(1, args.steps + 1):
        # 1. 현재 policy로 샘플링
        seqs, prompt_len = rollout(model, tok, args.n)
        texts = tok.batch_decode(seqs[:, prompt_len:], skip_special_tokens=True)
        # 2. reward와 advantage 계산
        raw = torch.tensor([reward_fn(t) for t in texts], device=seqs.device)
        adv = compute_advantages(raw + args.reward_offset, args.baseline)
        # 3. forward를 다시 수행해 log-prob와 loss 계산
        mask = response_mask(seqs, prompt_len, tok.eos_token_id)
        token_logp = response_logprobs(model, seqs, prompt_len)
        loss = reinforce_loss(token_logp, mask, adv)
        # 4. 업데이트 후 샘플은 버림(on-policy)
        opt.zero_grad()
        loss.backward()
        grad_norm = torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        opt.step()

        if step % 10 == 0:
            print(f"step {step:4d} | reward {raw.mean().item():.3f} | loss {loss.item():+.4f} "
                  f"| grad_norm {grad_norm.item():.2f} | len {mask.sum(1).mean().item():.1f}")
            print(f"  sample: {texts[0][:80]!r}")

if __name__ == "__main__":
    main()
```

baseline과 reward offset 설정별로 실행합니다.

```bash
mkdir -p logs
python train.py --baseline none --steps 100 | tee logs/none.log
python train.py --baseline mean --steps 100 | tee logs/mean.log
python train.py --baseline loo  --steps 100 | tee logs/loo.log
python train.py --baseline none --reward-offset 5 --steps 100 | tee logs/none_off5.log
python train.py --baseline mean --reward-offset 5 --steps 100 | tee logs/mean_off5.log
```

아래 출력은 가상의 예시입니다. 학습 전 reward는 약 0.12라고 가정합니다.

```text
$ python train.py --baseline mean --steps 100
step   10 | reward 0.188 | loss +1.8342 | grad_norm 21.37 | len 19.4
  sample: 'Apples are crisp and sweet, perfect for a healthy snack.'
step   20 | reward 0.250 | loss -0.6120 | grad_norm 17.85 | len 18.1
  sample: 'An orange is full of vitamin C.'
...
step   50 | reward 0.438 | loss +0.9715 | grad_norm 14.02 | len 15.7
  sample: 'Bananas are a great source of potassium.'
...
step  100 | reward 0.812 | loss -0.2481 | grad_norm 9.66 | len 13.2
  sample: 'Bananas are sweet and easy to peel.'
```

로그를 읽는 기준은 다음과 같습니다.

- reward: 학습 신호가 전달되는지 판단하는 주 지표
- loss: 부호가 오르내려도 정상, surrogate loss이므로 성능과 무관
- grad_norm: clipping 전 크기, 갑자기 커지면 advantage 계산이나 mask 오류를 의심
- len: reward와 무관하게 길이가 한쪽으로 움직이면 loss aggregation이나 reward 설계의 영향일 수 있음
- sample: 숫자만 보지 않고 실제 출력을 확인

설정별 결과는 다음과 같다고 가정합니다(가상의 값).

| 설정 | step 100 reward | 해석 |
|---|---|---|
| none, offset 0 | 0.56 | 오답 응답의 가중치가 0이라 정답 응답만 밀어 올림 |
| mean, offset 0 | 0.81 | 정답은 올리고 오답은 내림 |
| loo, offset 0 | 0.81 | mean과 방향이 같고 크기만 약간 다름 |
| none, offset 5 | 0.21 | 모든 응답을 올리려는 noise가 커서 학습이 느림 |
| mean, offset 5 | 0.81 | 평균을 빼면 offset이 사라짐, 같은 seed면 mean, offset 0과 거의 같은 궤적 |

AdamW는 gradient 크기를 정규화하고 grad clipping도 크기를 제한하므로, reward 곡선만으로는 분산 차이가 일부 가려집니다. 분산 자체는 1.9에서 raw gradient로 측정합니다.

학습을 길게 돌리면 toy reward의 허점이 드러납니다(가상의 예시).

```text
$ python train.py --baseline mean --steps 300
...
step  300 | reward 1.000 | loss +0.0000 | grad_norm 0.00 | len 6.0
  sample: 'Banana banana banana.'
```

두 가지 현상이 함께 나타납니다.

- reward hacking: reward 함수는 만족하지만 의도한 "자연스러운 과일 문장"이 아님, reference model과의 KL penalty와 reward 설계로 다루는 문제
- 학습 신호 소실: 모든 응답의 reward가 1이라 advantage가 전부 0, grad_norm이 0이 됨(1.6)

#### 1.9 실습: baseline에 따른 gradient 분산 비교

parameter를 고정한 채 batch를 $K$번 샘플링해 gradient 추정값 $\hat{g}_1, \dots, \hat{g}_K$를 구하고, 그 흩어진 정도를 측정합니다.

$$
\mathrm{tr}\,\mathrm{Cov}[\hat{g}] = \mathbb{E}\left[\lVert \hat{g} \rVert^2\right] - \lVert \mathbb{E}[\hat{g}] \rVert^2
$$

> 읽는 법: $\mathrm{tr}\,\mathrm{Cov}$(트레이스 코브)는 gradient vector 각 원소의 분산을 모두 더한 값입니다. gradient 제곱 norm의 평균에서 평균 gradient의 제곱 norm을 빼서 계산합니다.

같은 rollout으로 모든 설정의 gradient를 계산해야 샘플 차이가 비교에 섞이지 않습니다.

```python
# gradvar.py
import torch
from common import (load, reward_fn, rollout, response_mask, response_logprobs,
                    compute_advantages, reinforce_loss)

CONFIGS = [("none", 0.0), ("none", 5.0), ("mean", 0.0), ("mean", 5.0), ("loo", 0.0)]

def main(k=16, n=16):
    torch.manual_seed(0)
    tok, model = load()
    params = list(model.model.layers[-1].parameters())  # 마지막 layer만 측정해 메모리 절약
    g_sum = {c: 0.0 for c in CONFIGS}
    sq_sum = {c: 0.0 for c in CONFIGS}

    for _ in range(k):
        # parameter는 고정, 같은 rollout으로 모든 설정의 gradient를 계산
        seqs, prompt_len = rollout(model, tok, n)
        texts = tok.batch_decode(seqs[:, prompt_len:], skip_special_tokens=True)
        raw = torch.tensor([reward_fn(t) for t in texts], device=seqs.device)
        mask = response_mask(seqs, prompt_len, tok.eos_token_id)
        token_logp = response_logprobs(model, seqs, prompt_len)

        for c in CONFIGS:
            baseline, offset = c
            loss = reinforce_loss(token_logp, mask, compute_advantages(raw + offset, baseline))
            grads = torch.autograd.grad(loss, params, retain_graph=True)
            g = torch.cat([x.flatten() for x in grads])
            g_sum[c] = g_sum[c] + g
            sq_sum[c] += g.pow(2).sum().item()

    ref = None
    for c in CONFIGS:
        mean = g_sum[c] / k
        var = sq_sum[c] / k - mean.pow(2).sum().item()  # E||g||^2 - ||E[g]||^2
        ref = var if ref is None else ref
        print(f"{c[0]:>4} offset {c[1]:.0f} | total_var {var:.2e} | x{var / ref:.2f}")

if __name__ == "__main__":
    main()
```

아래 출력은 가상의 예시입니다.

```text
$ python gradvar.py
none offset 0 | total_var 2.41e-03 | x1.00
none offset 5 | total_var 5.87e-02 | x24.36
mean offset 0 | total_var 1.62e-03 | x0.67
mean offset 5 | total_var 1.62e-03 | x0.67
 loo offset 0 | total_var 1.84e-03 | x0.76
```

결과는 다음과 같이 해석합니다.

- none offset 5가 offset 0보다 수십 배 큼: 상수를 더해도 기대 gradient는 같지만 분산은 크게 늘어남(1.6)
- mean offset 0과 5가 완전히 같음: 평균을 빼면 상수가 정확히 사라지므로 구현 검증용 sanity check로 쓸 수 있음
- loo가 mean보다 약 1.14배: mean의 advantage는 loo의 $\frac{15}{16}$배이므로 분산은 정확히 $(\frac{16}{15})^2 \approx 1.14$배 차이가 나고 방향은 같음
- 수치는 정답률과 응답 길이에 따라 달라지며, 비율과 대소 관계를 기준으로 판단

1.7의 내용도 같은 batch로 확인할 수 있습니다. $\theta = \theta_\text{old}$에서는 importance sampling loss와 REINFORCE loss의 gradient가 같아야 합니다.

```python
# gradvar.py의 루프 안에서 실행
ratio = torch.exp(token_logp - token_logp.detach())  # 값은 모두 1, gradient는 log pi의 gradient
adv = compute_advantages(raw, "mean")
loss_is = -((ratio * mask).sum(dim=1) * adv).mean()
loss_pg = reinforce_loss(token_logp, mask, adv)
g_is = torch.autograd.grad(loss_is, params, retain_graph=True)
g_pg = torch.autograd.grad(loss_pg, params, retain_graph=True)
print(max((a - b).abs().max().item() for a, b in zip(g_is, g_pg)))  # 0 또는 부동소수점 오차 수준
```

두 loss의 값은 다르지만 gradient는 같습니다. surrogate loss는 값이 아니라 gradient로 설계한다는 점을 확인하는 실습입니다.

#### 1.10 자주 하는 실수

| 증상 | 원인 | 확인 방법 |
|---|---|---|
| 학습 시작 직후 응답이 깨짐 | logits와 target의 shift 누락 | 학습 전 응답 token의 평균 log-prob가 대략 -0.5 ~ -3인지 확인, -10 근처(vocab 49152의 균등 분포 수준)면 엉뚱한 token의 확률을 보고 있음 |
| 학습이 불안정하고 grad_norm이 큼 | prompt token이 loss에 포함됨 | `mask.sum(1)`이 실제 응답 token 수와 같은지 확인 |
| 응답 길이가 비정상적으로 변함 | EOS 이후 padding이 loss에 포함됨 | EOS 뒤 위치의 mask가 0인지 확인 |
| `element 0 of tensors does not require grad` 오류 | `generate` 결과로 바로 loss를 계산함 | 같은 sequence로 forward를 다시 수행 |
| 학습이 안 된다고 잘못 판단 | surrogate loss를 성능 지표로 봄 | reward 곡선과 샘플 출력으로 판단 |
| 증상은 잘 보이지 않지만 gradient에 bias가 생김 | generation config의 top-k, top-p 기본값이 적용됨 | `print(model.generation_config)`로 확인하고 rollout 설정을 명시 |
| reward가 전혀 움직이지 않음 | batch reward가 모두 같아 advantage가 0 | step마다 reward 표준편차를 함께 기록 |
| loss가 NaN | bf16 logits에 바로 `log_softmax` 적용 | `logits.float()` 후 계산 |
| reward는 1.0인데 응답이 이상함 | reward 함수의 허점(reward hacking) | 샘플 출력을 주기적으로 확인 |

#### 1.11 완료 점검과 자료

다음 항목을 자료 없이 설명하거나 작성할 수 있으면 이 단계를 마친 것입니다.

- LLM 응답 생성의 state, action, transition, reward를 정의하고 transition이 결정적이라는 의미를 설명할 수 있음 (1.2)
- sparse reward와 $\gamma = 1$에서 모든 token의 return이 같아지는 이유를 설명할 수 있음 (1.3)
- 정답률 30%인 prompt에서 정답 응답과 오답 응답의 advantage를 계산할 수 있음 (1.3)
- log-derivative trick으로 $\nabla_\theta J$를 기댓값 형태로 바꾸는 4줄 유도를 쓸 수 있음 (1.4)
- environment transition을 몰라도 policy gradient를 계산할 수 있는 이유를 설명할 수 있음 (1.4)
- surrogate loss 값이 성능 지표가 아닌 이유를 설명할 수 있음 (1.5)
- baseline이 bias를 만들지 않는 이유를 EGLP lemma로 설명할 수 있음 (1.6)
- reward에 상수를 더했을 때 baseline 유무에 따라 무엇이 달라지는지 설명할 수 있음 (1.6, 1.9)
- $\theta = \theta_\text{old}$에서 importance sampling loss의 gradient가 REINFORCE와 같아지는 이유를 설명할 수 있음 (1.7)
- rollout에서 top-p, top-k를 끄는 이유를 설명할 수 있음 (1.7)
- token log-prob 계산에서 shift와 `prompt_len - 1` slicing이 필요한 이유를 설명할 수 있음 (1.8)

| 자료 | 읽을 부분 | 이 단계에서 얻을 것 |
|---|---|---|
| OpenAI Spinning Up "Key Concepts in RL" | 전체 | 용어와 표기법, value function과 Bellman equation |
| OpenAI Spinning Up "Intro to Policy Optimization" | 전체, 특히 EGLP lemma와 reward-to-go | 1.4~1.6의 유도 과정 |
| Sutton & Barto 13장 | 13.1~13.4 정독, 13.5 이후는 훑기 | policy gradient theorem 일반형, REINFORCE with baseline |

Sutton & Barto는 policy를 $\pi(a \mid s, \boldsymbol{\theta})$로 쓰는 등 표기가 Spinning Up과 조금 다르므로, 1.1의 기호표와 대조하며 읽습니다.

#### 1.12 정리

- LLM 생성은 state가 prompt와 prefix, action이 다음 token, reward가 응답 끝의 sparse reward인 MDP
- 응답 log-prob는 token log-prob의 합이며, shift와 mask로 응답 구간만 계산
- advantage는 평균 대비 얼마나 나은지를 나타내며, 양수면 확률 증가, 음수면 확률 감소
- policy gradient는 log-derivative trick으로 얻은 "reward로 가중한 maximum likelihood"
- REINFORCE loss는 `-(seq_logp * adv.detach()).mean()`이며 loss 값은 성능 지표가 아님
- baseline은 기대 gradient를 바꾸지 않고 분산을 줄임, group 평균은 GRPO, leave-one-out은 RLOO, critic은 PPO로 이어짐
- importance ratio는 이전 policy의 데이터를 재사용하기 위한 보정이며, REINFORCE는 ratio가 1인 특수한 경우

### 2. Policy gradient와 Actor-Critic

현재 LLM RL 알고리즘은 모두 policy gradient 계열이므로 이 단계가 전체의 출발점입니다.

- 핵심 주제
  - policy gradient theorem, log-derivative trick
  - REINFORCE
  - baseline과 variance reduction
  - Actor-Critic 구조
  - GAE(Generalized Advantage Estimation)
  - entropy bonus
- 가볍게 넘길 내용: DQN 계열 value-based 방법(LLM RL에서는 거의 쓰지 않음)
- 완료 기준: CartPole 같은 간단한 환경에 REINFORCE를 직접 구현하고, baseline 유무에 따른 분산 차이를 확인할 수 있음
- 자료: Spinning Up "Intro to Policy Optimization", Sutton & Barto 13장, GAE 논문

### 3. PPO와 KL 제약

RLHF의 표준 알고리즘이며, 이후 GRPO 계열 objective도 PPO의 구조를 그대로 물려받습니다.

- 핵심 주제
  - importance sampling과 policy ratio
  - trust region 아이디어(TRPO는 개념만)
  - PPO clipped objective, value loss
  - reference model과 KL penalty
  - 주요 hyperparameter: clip range, mini-batch 수, update epoch 수
- 완료 기준: PPO loss 각 항의 역할을 설명하고, 학습 로그(KL, clip fraction, entropy)로 이상 징후를 판단할 수 있음
- 자료: PPO 논문(Schulman 2017), "The 37 Implementation Details of Proximal Policy Optimization"

### 4. RLHF와 preference optimization

사람의 선호를 reward로 바꾸어 모델을 정렬하는 방법입니다. online RL과 offline 방식(DPO)의 차이를 이해하는 것이 핵심입니다.

- 핵심 주제
  - SFT → reward model → PPO 파이프라인(InstructGPT)
  - Bradley-Terry 기반 reward model 학습
  - reward hacking과 reward overoptimization
  - DPO 유도 과정과 한계
  - 변형 비교: IPO, KTO, SimPO, ORPO
  - online과 offline preference optimization의 트레이드오프
- 완료 기준: RLHF objective에서 DPO loss가 유도되는 흐름을 설명하고, TRL로 작은 모델에 DPO 학습을 돌릴 수 있음
- 자료: InstructGPT 논문, DPO 논문, Nathan Lambert "RLHF Book"

### 5. RLVR과 GRPO 계열 알고리즘

정답 검증이 가능한 reward(RLVR, RL with Verifiable Rewards)로 reasoning 능력을 학습하는 방식으로, 현재 reasoning model 학습의 중심입니다.

- 핵심 주제
  - verifiable reward 설계: 수학 정답 검증, unit test, format reward
  - GRPO: group 단위 상대 advantage, critic 제거
  - critic 없는 대안: RLOO, REINFORCE++
  - GRPO 개선 변형: DAPO, Dr. GRPO, GSPO
  - 자주 겪는 문제: length bias, entropy collapse, 쉬운/어려운 prompt의 학습 신호 소실
  - RL이 능력을 새로 만드는지, 기존 분포를 날카롭게 하는지에 대한 논의(pass@1과 pass@k)
- 완료 기준: 작은 모델에 GSM8K 수준의 수학 데이터로 GRPO 학습을 돌리고, reward 곡선과 응답 길이 변화를 해석할 수 있음
- 자료: DeepSeekMath(GRPO 제안), DeepSeek-R1, DAPO, "Understanding R1-Zero-Like Training"(Dr. GRPO)

### 6. Agentic RL과 학습 인프라

tool use와 multi-turn 상호작용을 RL로 학습하는 단계이며, 실제 학습을 돌리는 인프라 이해를 함께 다룹니다.

- 핵심 주제
  - multi-turn rollout, tool 호출 결과를 포함한 trajectory 구성
  - environment 설계: sandbox, code execution, 외부 API
  - trajectory 단위 reward와 credit assignment
  - LLM-as-a-judge, rubric 기반 reward와 그 위험
  - rollout engine(vLLM, SGLang)과 trainer의 분리
  - async RL과 off-policy 보정
  - 프레임워크 구조 비교: TRL, verl, OpenRLHF
- 완료 기준: tool 호출 environment 하나를 정의하고 multi-turn rollout을 포함한 학습 루프를 구성할 수 있음
- 자료: verl, TRL, OpenRLHF 공식 문서와 예제 코드

### 정리

- 1~3단계: 고전 RL 중 LLM RL에 필요한 최소 기반
- 4~5단계: LLM post-training의 핵심이며 현재 실무 비중이 가장 큰 구간
- 6단계: 실제 학습 환경을 구성하고 디버깅하는 역량
- 시간이 부족하면 1~2단계를 압축하고 3단계와 5단계에 집중
- 각 단계의 완료 기준을 통과한 뒤 다음 단계로 이동
