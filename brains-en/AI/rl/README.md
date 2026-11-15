# RL Learning Curriculum

This curriculum is designed for just-in-time learning of Reinforcement Learning (RL) used in LLM post-training. Classical RL theory will only be covered as much as needed to understand current algorithms like PPO and GRPO. The goal is to develop the ability to read, configure, and debug RL pipelines used for training reasoning models and agents. Since understanding theory alone makes it difficult to interpret real-world issues like reward hacking or training instability, completion criteria are set for each stage.

### 1. RL Fundamentals and Policy Gradient from an LLM Perspective

This section provides the minimum foundation for reading LLM RL papers and code. We define LLM response generation as an MDP, derive the policy gradient—the principle for training models with non-differentiable rewards—and then directly implement REINFORCE on a small LM. PPO, GRPO, and DPO are all variations of the equations from this stage. Therefore, without a precise understanding of advantage, baseline, and importance sampling, it will be difficult to implement losses and interpret training logs in subsequent stages.

- Completion criteria: Able to write a policy gradient loss using token-level log-probabilities and explain the meaning of advantage
- Content to skim: Tabular methods (dynamic programming, Q-learning), DQN, convergence proofs
- Estimated duration: 1 week

#### 1.1 Mathematical Notation and Reading

The notation used in this stage is shown in the table below. Most papers in subsequent stages also use the same notation.

| Symbol | Reading | Meaning |
|---|---|---|
| $s_t$ | s t | state at time $t$ |
| $a_t$ | a t | action chosen at time $t$ |
| $r_t$ | r t | reward received immediately after choosing $a_t$ |
| $\theta$ | theta | all model parameters |
| $\pi_\theta(a \mid s)$ | pi theta, a given s | probability of policy with parameter $\theta$ choosing action $a$ in state $s$ |
| $\tau$ | tau | trajectory, a sequence $(s_0, a_0, r_0, s_1, a_1, r_1, \dots)$ |
| $\tau \sim \pi_\theta$ | tau follows pi theta | sampling a trajectory using the current policy |
| $\gamma$ | gamma | discount factor, between 0 and 1 inclusive |
| $G_t$ | g t | return from time $t$ to the end |
| $R(\tau)$ | r of tau | return of the entire trajectory |
| $V^\pi(s)$ | v pi of s | state value function |
| $Q^\pi(s, a)$ | q pi of s a | action value function |
| $A^\pi(s, a)$ | a pi of s a | advantage |
| $J(\theta)$ | j of theta | optimization objective, expected return |
| $\mathbb{E}_{x \sim p}[f(x)]$ | expected value of f of x when x follows p | average of $f(x)$ calculated with $x$ sampled from $p$ |
| $\nabla_\theta f$ | nabla theta f, gradient of f with respect to theta | vector of partial derivatives of $f$ with respect to each element of $\theta$ |
| $\log$ | log | natural logarithm |
| $\sum_{t=0}^{T-1}$ | sigma, t from 0 to T-1 | sum by varying $t$ |
| $\prod_{t=1}^{T}$ | product, t from 1 to T | product by varying $t$ |
| $y_{<t}$ | y less than t | all tokens generated before the $t$-th token |

When reading equations for the first time, check three things:

- Subscript of $\mathbb{E}$: From which distribution is data sampled to compute the average?
- Subscript of $\nabla$: With respect to which parameter is the differentiation performed?
- Index of $\sum$: Is the summation based on tokens, responses, or batches?

The subscript $t$ denotes time, and the superscript $i$ denotes the sample number. $a_t^i$ means "the $t$-th action of the $i$-th sample" and is not an exponent. In contrast, superscripts attached to numbers, like $\gamma^{k-t}$, are exponents.

#### 1.2 Mapping MDP to LLM Generation

A **MDP** (Markov Decision Process) is a standard formalism for sequential decision-making problems where an agent observes a state, chooses an action, and the environment returns the next state and a reward. It is defined by five elements $(\mathcal{S}, \mathcal{A}, P, r, \gamma)$.

- $\mathcal{S}$, $\mathcal{A}$: set of states and set of actions
- $P(s' \mid s, a)$: probability that the next state is $s'$ when action $a$ is taken in state $s$ (transition)
- $r(s, a)$: reward function
- $\gamma$: discount factor determining how much future rewards are discounted

The premise of an MDP is the Markov property.

$$
P(s_{t+1} \mid s_t, a_t, s_{t-1}, a_{t-1}, \dots, s_0) = P(s_{t+1} \mid s_t, a_t)
$$

Reading: The probability of the next state $s_{t+1}$ is determined only by the current state $s_t$ and action $a_t$, not by the entire past history.

LLM response generation is mapped to an MDP as follows. In the LLM context, token indices are counted from 1.

| MDP Element | Meaning in LLM Generation | Notes |
|---|---|---|
| state $s_t$ | prompt $x$ and tokens generated so far $y_{<t}$ | Markov property holds by definition as the entire prefix is the state |
| action $a_t$ | next token $y_t$ | action space is the entire vocabulary (tens to hundreds of thousands) |
| transition $P$ | $s_{t+1} = (x, y_{<t}, y_t)$, concatenation | deterministic, no probabilistic element |
| reward $r_t$ | response score only at the last token, 0 for other tokens | sparse reward |
| episode termination | EOS token generation or maximum length reached | |
| policy $\pi_\theta$ | LM's next-token distribution (softmax of logits) | |
| $\gamma$ | usually 1 | no discounting as response length is finite |

The probability of the entire response $y = (y_1, \dots, y_T)$ is the product of token probabilities.

$$
\pi_\theta(y \mid x) = \prod_{t=1}^{T} \pi_\theta(y_t \mid x, y_{<t})
$$

$$
\log \pi_\theta(y \mid x) = \sum_{t=1}^{T} \log \pi_\theta(y_t \mid x, y_{<t})
$$

Reading: The probability of response $y$ given prompt $x$ is the product of probabilities of choosing $y_t$ at each time step $t$, given preceding tokens, from start to end. Taking the log of both sides converts the product into a sum.

The log form is used for the following reasons:

- Multiplying probabilities less than 1 hundreds of times can cause underflow, but the sum of logs is stable.
- Differentiation of sums is simpler than differentiation of products.
- Can be directly obtained by applying `log_softmax` to model output.

As an example, if the prompt is "Name a fruit." and the response is "Banana.", giving a reward of 1 if the response is a fruit name, it can be written as an MDP: (Tokenization is an example for explanation.)

| $t$ | state $s_t$ | action $a_t = y_t$ | reward $r_t$ |
|---|---|---|---|
| 1 | $x$ | "Ban" | 0 |
| 2 | $x$ + "Ban" | "ana" | 0 |
| 3 | $x$ + "Banana" | "." | 0 |
| 4 | $x$ + "Banana." | EOS | 1 |

Two perspectives are used together in LLM RL:

| Perspective | Action Unit | Advantage Unit | Representative Algorithm |
|---|---|---|---|
| Token-level MDP | single token | can differ per token | PPO (critic estimates token-wise value) |
| Response-level contextual bandit | entire response | one per response, same value applied to all tokens | REINFORCE, RLOO, GRPO |

A contextual bandit is a problem where an action (response) is chosen once based on a context (prompt), a reward is received, and the episode ends. Since $\log \pi_\theta(y \mid x)$ is the sum of token log-probabilities, the two perspectives are connected by the same gradient equation.

#### 1.3 Return, Value Function, Advantage

**Return** is the discounted sum of rewards received from a specific time step until the end of the episode.

$$
G_t = \sum_{k=t}^{T-1} \gamma^{k-t} r_k = r_t + \gamma r_{t+1} + \gamma^2 r_{t+2} + \cdots
$$

Reading: $G_t$ is the sum of rewards received from time $t$ to the end, where the $k$-th reward is multiplied by $\gamma$ for $k-t$ times, making rewards further in the future contribute less. The total trajectory return is $R(\tau) = G_0$.

In LLMs, rewards are only at the last token and $\gamma = 1$, so the return at all time steps is equal to the response score.

$$
G_t = R(x, y) \quad (t = 1, \dots, T)
$$

This means all tokens within a response receive the same return. The problem of not being able to distinguish which token contributed to the outcome is called the credit assignment problem, which PPO's critic, GAE, and process reward address.

**Value function** is the expected return when following policy $\pi$.

$$
V^\pi(s) = \mathbb{E}_{\tau \sim \pi}\left[G_t \mid s_t = s\right]
$$

$$
Q^\pi(s, a) = \mathbb{E}_{\tau \sim \pi}\left[G_t \mid s_t = s,\ a_t = a\right]
$$

Reading: $V^\pi(s)$ is the average return starting from state $s$ and proceeding according to policy $\pi$ until the end. $Q^\pi(s, a)$ is the average return when first choosing action $a$ in state $s$, and then proceeding according to $\pi$.

**Advantage** is a value indicating how much better a specific action is compared to the average in that state.

$$
A^\pi(s, a) = Q^\pi(s, a) - V^\pi(s)
$$

If $A > 0$, it's a better-than-average action, so its probability is increased. If $A < 0$, its probability is decreased. The expected value of advantage when following the policy is always 0 ($\mathbb{E}_{a \sim \pi}[A^\pi(s, a)] = 0$).

As an example, if the current model's accuracy for a math prompt is 30% and the reward is 1 for correct, 0 for incorrect, then $V(x) = 0.3$.

| Response | Reward $R$ | Advantage $R - V(x)$ | Learning Direction |
|---|---|---|---|
| Correct | 1 | +0.7 | Increase probability |
| Incorrect | 0 | -0.3 | Decrease probability |

The expected advantage is $0.3 \times 0.7 + 0.7 \times (-0.3) = 0$. The approach of sampling multiple responses for the same prompt and estimating $V(x)$ with the average reward is central to GRPO and RLOO.

The **Bellman equation** recursively expresses the value of the current state in terms of the immediate reward and the value of the next state.

$$
V^\pi(s) = \mathbb{E}_{a \sim \pi(\cdot \mid s),\ s' \sim P(\cdot \mid s, a)}\left[r(s, a) + \gamma V^\pi(s')\right]
$$

Reading: The value of the current state is the average of the sum of the immediate reward received when an action is chosen by the policy and the environment determines the next state, plus the discounted value of the next state.

From this equation, the TD error is derived.

$$
\delta_t = r_t + \gamma V(s_{t+1}) - V(s_t)
$$

$\delta_t$ indicates "how much better than expected it was after one step" and is a one-step estimate of advantage. It is used when training the critic and calculating token-wise advantage with GAE. In LLM generation, since the reward for intermediate tokens is 0 and transitions are deterministic, $\delta_t$ at intermediate steps is $V(s_{t+1}) - V(s_t)$, i.e., the change in value due to a single token.

#### 1.4 Policy Gradient Theorem and Log-Derivative Trick

**Policy gradient** is the gradient of the expected return $J(\theta)$ with respect to the policy parameter $\theta$, and parameters are updated in this direction to improve the policy.

$$
J(\theta) = \mathbb{E}_{\tau \sim \pi_\theta}\left[R(\tau)\right]
$$

In LLMs, it is written as:

$$
J(\theta) = \mathbb{E}_{x \sim \mathcal{D},\ y \sim \pi_\theta(\cdot \mid x)}\left[R(x, y)\right]
$$

Reading: This is the average reward received when a prompt $x$ is drawn from dataset $\mathcal{D}$ and a response $y$ is sampled using the current model.

This equation cannot be differentiated directly:

- $\theta$ is inside the sampling distribution $\pi_\theta$, not the reward function.
- Rewards are often non-differentiable functions, such as string comparisons or unit test results.
- Token sampling is discrete, so backpropagation cannot pass through.

The **log-derivative trick** is a technique that uses the differentiation rule of the log function to transform the gradient of a probability into "probability × gradient of log probability".

$$
\nabla_\theta \log p_\theta(x) = \frac{\nabla_\theta p_\theta(x)}{p_\theta(x)}
\quad \Longrightarrow \quad
\nabla_\theta p_\theta(x) = p_\theta(x)\, \nabla_\theta \log p_\theta(x)
$$

Reading: The derivative of $\log u$ is $1/u$, so the left equation comes from the chain rule. Multiplying both sides by $p_\theta(x)$ gives the right equation.

Applying this trick to $J(\theta)$:

$$
\begin{aligned}
\nabla_\theta J(\theta)
&= \nabla_\theta \sum_{\tau} P_\theta(\tau)\, R(\tau) \\
&= \sum_{\tau} \nabla_\theta P_\theta(\tau)\, R(\tau) \\
&= \sum_{\tau} P_\theta(\tau)\, \nabla_\theta \log P_\theta(\tau)\, R(\tau) \\
&= \mathbb{E}_{\tau \sim \pi_\theta}\left[\nabla_\theta \log P_\theta(\tau)\, R(\tau)\right]
\end{aligned}
$$

The meaning of each line is as follows:

1.  The expectation is expanded into a probability-weighted sum over all possible trajectories.
2.  $R(\tau)$ is independent of $\theta$, so the gradient only applies to $P_\theta(\tau)$.
3.  The log-derivative trick transforms $\nabla_\theta P_\theta$ into $P_\theta \nabla_\theta \log P_\theta$.
4.  The sum weighted by $P_\theta(\tau)$ is again an expectation, which can be estimated by averaging over sampled trajectories.

The last line means that even if all trajectories cannot be enumerated, the gradient can be estimated by sampling from the current policy and taking the average.

Trajectory probability is the product of the environment part and the policy part.

$$
P_\theta(\tau) = \rho_0(s_0) \prod_{t=0}^{T-1} P(s_{t+1} \mid s_t, a_t)\, \pi_\theta(a_t \mid s_t)
$$

Taking the log and differentiating with respect to $\theta$, the initial state distribution $\rho_0$ and transition $P$ disappear as they are independent of $\theta$.

$$
\nabla_\theta \log P_\theta(\tau) = \sum_{t=0}^{T-1} \nabla_\theta \log \pi_\theta(a_t \mid s_t)
$$

The gradient can be calculated even without knowing the environment's transition, which is why policy gradient is a model-free method. Combining the two equations yields the following result:

$$
\nabla_\theta J(\theta) = \mathbb{E}_{\tau \sim \pi_\theta}\left[\sum_{t=0}^{T-1} \nabla_\theta \log \pi_\theta(a_t \mid s_t)\, R(\tau)\right]
$$

Reading: The gradient of $J$ with respect to $\theta$ is the expected value of the sum of the log probability gradient of each time step's action multiplied by the return, when trajectories are sampled from the current policy.

Comparing with SFT clarifies its meaning:

| Category | Gradient | Data Source | Weight |
|---|---|---|---|
| SFT | $\nabla_\theta \log \pi_\theta(y^* \mid x)$ | Ground truth label $y^*$ | All 1 |
| Policy gradient | $\nabla_\theta \log \pi_\theta(y \mid x)\, R(x, y)$ | $y$ sampled directly by the model | Reward |

Policy gradient is "maximum likelihood weighted by reward for responses generated by the model itself." Responses with positive rewards increase in probability, those with negative rewards decrease, and those with zero reward have no effect.

Sutton & Barto's policy gradient theorem is a general form expressing the same content with state visitation distribution and $Q$.

$$
\nabla_\theta J(\theta) \propto \sum_{s} \mu(s) \sum_{a} Q^\pi(s, a)\, \nabla_\theta \pi_\theta(a \mid s)
$$

Reading: $\mu(s)$ is the proportion of time spent in state $s$ when following the policy, and $\propto$ is read as "is proportional to". The gradient is in the direction of increasing the probability of actions with large $Q$ in frequently visited states.

In actual implementations, a value $\Psi_t$ (psi t) with smaller variance is used in place of $R(\tau)$.

$$
\nabla_\theta J(\theta) = \mathbb{E}_{\tau \sim \pi_\theta}\left[\sum_{t=0}^{T-1} \nabla_\theta \log \pi_\theta(a_t \mid s_t)\, \Psi_t\right]
$$

| $\Psi_t$ | Name | Characteristics |
|---|---|---|
| $R(\tau)$ | Total return | Simplest, highest variance |
| $G_t$ | Reward-to-go | Action cannot affect past rewards, so only future rewards are used, variance reduced |
| $G_t - b(s_t)$ | Baseline applied | REINFORCE with baseline, covered in 1.6 |
| $A^\pi(s_t, a_t)$ | Advantage | Smallest variance but requires estimation, Actor-Critic and GAE |

For LLMs, if the reward is only at the end, $R(\tau)$ and $G_t$ are the same, so there is no gain from reward-to-go.

#### 1.5 REINFORCE and Surrogate Loss

**REINFORCE** is the most basic algorithm that updates parameters by estimating the expected value of the policy gradient using Monte Carlo sample averages.

$$
\hat{g} = \frac{1}{N} \sum_{i=1}^{N} \sum_{t=0}^{T_i - 1} \nabla_\theta \log \pi_\theta(a_t^i \mid s_t^i)\, G_t^i
$$

Reading: $\hat{g}$ (g hat) is the estimated gradient. $N$ trajectories are sampled, and for each time step of the $i$-th trajectory, the log probability gradient is multiplied by the return and summed, then averaged by dividing by $N$. $T_i$ is the length of the $i$-th trajectory.

The operational sequence is as follows:

1.  Sample $N$ trajectories using the current policy $\pi_\theta$.
2.  Calculate rewards and returns for each trajectory.
3.  Calculate $\hat{g}$.
4.  Update with gradient ascent: $\theta \leftarrow \theta + \alpha \hat{g}$ ($\alpha$ is the learning rate).
5.  Discard used samples and return to step 1.

Since PyTorch optimizers minimize loss, a **surrogate loss** is created such that its gradient is $-\hat{g}$, and `backward()` is called. A surrogate loss has no meaning in its value itself; it is a function constructed so that its gradient matches the desired value.

$$
\mathcal{L}(\theta) = -\frac{1}{N} \sum_{i=1}^{N} \sum_{t=0}^{T_i - 1} \log \pi_\theta(a_t^i \mid s_t^i)\ \mathrm{sg}\!\left[G_t^i\right]
$$

Reading: $\mathrm{sg}[\cdot]$ means stop-gradient, which treats the value inside the parenthesis as a constant and prevents gradients from flowing through it. The preceding minus sign is to convert a maximization problem into a minimization problem.

In LLMs, the response-level advantage $A_i$ is multiplied by all tokens.

$$
\mathcal{L}(\theta) = -\frac{1}{N} \sum_{i=1}^{N} A_i \sum_{t=1}^{T_i} \log \pi_\theta(y_t^i \mid x, y_{<t}^i)
$$

If there is no baseline, $A_i = R_i$. If there is a baseline, $A_i = R_i - b$. The core code is two lines:

```python
# seq_logp: log pi(y|x) per response = sum of token log-probs, [N]
# adv: advantage per response, [N]
loss = -(seq_logp * adv.detach()).mean()
loss.backward()  # gradient is -g_hat, optimizer moves in opposite direction to increase J
```

Points to note when dealing with surrogate loss:

- Loss value is not a performance metric: Since data is newly sampled at each step, performance can improve even if loss increases. Performance is judged by reward.
- Apply `detach()` to advantage: If gradients flow towards reward or baseline, it becomes a different objective.
- Aggregation method: Token log-probabilities must be summed per response and then averaged over the batch to match the above equation. Dividing by the total number of tokens changes the weighting based on response length, leading to the topic of loss aggregation in PPO and GRPO families.

#### 1.6 Baseline and Variance Reduction

A **baseline** is a value $b(s_t)$ that is subtracted from the return in the policy gradient equation and does not depend on the action. It reduces the variance of the estimate without changing the expected gradient.

$$
\nabla_\theta J(\theta) = \mathbb{E}_{\tau \sim \pi_\theta}\left[\sum_{t=0}^{T-1} \nabla_\theta \log \pi_\theta(a_t \mid s_t)\, \big(G_t - b(s_t)\big)\right]
$$

The reason a baseline does not introduce bias is that the expected value of the log probability gradient is 0.

$$
\mathbb{E}_{a \sim \pi_\theta(\cdot \mid s)}\left[\nabla_\theta \log \pi_\theta(a \mid s)\right]
= \sum_{a} \pi_\theta(a \mid s)\, \frac{\nabla_\theta \pi_\theta(a \mid s)}{\pi_\theta(a \mid s)}
= \nabla_\theta \sum_{a} \pi_\theta(a \mid s)
= \nabla_\theta 1
= 0
$$

Reading: The sum of probabilities for all actions is always 1, and the gradient of the constant 1 is 0. Therefore, the expected value of a term where this gradient is multiplied by an action-independent value like $b(s)$ is also 0.

Spinning Up calls this property the EGLP lemma (Expected Grad-Log-Prob lemma). If $b$ depends on the action of the current sample, this proof does not hold.

The reason variance is reduced is clear when looking at numbers. If we add a constant 10 to a reward of 1 for correct and 0 for incorrect, changing them to 11 for correct and 10 for incorrect, the expected gradient remains the same, but the following problems arise:

- All responses receive large positive weights, mixing in a direction that tries to uniformly increase the probability of sampled responses.
- This direction cancels out in expectation, but if the number of samples is finite, it doesn't cancel and remains as noise.
- The informative difference (1, the difference between 11 and 10) gets buried in noise.

Subtracting the average reward as a baseline centers the weights around 0, making correct responses positive and incorrect ones negative, leaving only the necessary signal. The baseline that reduces variance the most is close to $V^\pi(s_t)$, in which case $G_t - V^\pi(s_t)$ becomes an estimate of advantage.

Baselines used in LLM RL are as follows:

| Baseline | Advantage Calculation | Pros | Cons | Subsequent Algorithms |
|---|---|---|---|---|
| None | $A_i = R_i$ | Simple to implement | High variance, sensitive to reward offset | REINFORCE |
| Batch mean | $A_i = R_i - \frac{1}{N}\sum_{j} R_j$ | No additional cost | Prompt difficulty mixed, includes own reward | |
| Leave-one-out | $A_i = R_i - \frac{1}{N-1}\sum_{j \ne i} R_j$ | Strictly unbiased as it excludes its own sample | Requires multiple samples per prompt | RLOO |
| Prompt-wise group mean | Subtract mean of responses for same prompt, divide by std dev | Corrects for prompt difficulty | Cost of generating $G$ samples per prompt | GRPO |
| Trained critic | Calculates token-wise advantage with $V_\phi(s_t)$ | Token-level advantage possible | Requires an additional model, training instability | PPO |

The batch mean baseline includes its own reward, so it is strictly dependent on the action. However, if the equation is rearranged, it takes the form of leave-one-out multiplied by a constant, so the gradient direction is the same, and only the magnitude changes by a factor of $\frac{N-1}{N}$.

$$
R_i - \frac{1}{N}\sum_{j} R_j = \frac{N-1}{N}\left(R_i - \frac{1}{N-1}\sum_{j \ne i} R_j\right)
$$

Using a baseline means that if all rewards in a batch are the same (all 0 or all 1), the advantage becomes 0 for all, and the learning signal disappears. This phenomenon, where prompts that are too easy or too difficult do not contribute to learning, is why RLVR performs difficulty filtering.

#### 1.7 On-policy, Off-policy, Importance Sampling

-   **On-policy**: Data used for training is sampled from the policy $\pi_\theta$ that is currently being updated.
-   **Off-policy**: The behavior policy from which data was sampled is different from the policy being trained.

The policy gradient equation assumes $\tau \sim \pi_\theta$, so REINFORCE is on-policy. After one update, $\theta$ changes, and existing samples become data from the previous policy $\pi_{\theta_\text{old}}$. Reusing them as is introduces bias into the gradient.

**Importance sampling** is a technique to estimate the expected value for distribution $p$ using samples drawn from distribution $q$.

$$
\mathbb{E}_{x \sim p}\left[f(x)\right]
= \sum_{x} p(x)\, f(x)
= \sum_{x} q(x)\, \frac{p(x)}{q(x)}\, f(x)
= \mathbb{E}_{x \sim q}\left[\frac{p(x)}{q(x)}\, f(x)\right]
$$

Reading: Averaging samples drawn from $q$ after multiplying them by a weight $p(x)/q(x)$ yields the same expected value as if they were drawn from $p$. This holds if $q(x) > 0$ wherever $p(x) > 0$.

In LLM RL, token-level probability ratios are used.

$$
\rho_t(\theta) = \frac{\pi_\theta(a_t \mid s_t)}{\pi_{\theta_\text{old}}(a_t \mid s_t)} = \exp\Big(\log \pi_\theta(a_t \mid s_t) - \log \pi_{\theta_\text{old}}(a_t \mid s_t)\Big)
$$

$$
\mathcal{L}^{\text{IS}}(\theta) = -\,\mathbb{E}_{\tau \sim \pi_{\theta_\text{old}}}\left[\sum_{t} \rho_t(\theta)\, A_t\right]
$$

Reading: $\rho_t$ (rho t) indicates how many times more likely the current policy is to choose that token compared to when it was sampled. This ratio is multiplied by data generated by the previous policy to approximate the current policy's objective. In implementation, it's calculated by taking `exp` of the log-prob difference.

At the point where $\theta = \theta_\text{old}$, the gradient of this loss is exactly the same as REINFORCE.

$$
\nabla_\theta \rho_t = \rho_t\, \nabla_\theta \log \pi_\theta(a_t \mid s_t), \qquad \rho_t\big|_{\theta = \theta_\text{old}} = 1
$$

REINFORCE is a special case where $\rho_t = 1$, and the loss for PPO and GRPO is this form with clipping and KL constraints added.

This equation is an approximation that ignores the effect of changing state distribution and only uses token ratios. The exact ratio for an entire response is the product of token ratios, which can become extremely large or small for long responses.

| Ratio per token | Product for 500 tokens |
|---|---|
| 1.01 | approx 145 |
| 0.99 | approx 0.0066 |

Therefore, actual algorithms use token-level ratios and restrict $\pi_\theta$ from deviating too far from $\pi_{\theta_\text{old}}$ using clipping and KL.

Off-policy situations in LLM RL occur in the following cases:

| Situation | Cause | Countermeasure |
|---|---|---|
| Multiple mini-batch or epoch updates with one rollout batch | $\pi_\theta \ne \pi_{\theta_\text{old}}$ after first update | Ratio and clipping (PPO) |
| Async RL | Trainer updates weights during rollout generation, making data stale | Staleness limits, importance sampling correction (Stage 6) |
| Train-inference mismatch | Log-probs differ due to kernel/precision differences between inference engine (e.g., vLLM) and trainer | Recalculate log-probs in trainer, importance sampling correction (Stage 6) |
| Sampling settings | top-p, top-k, temperature not equal to 1 make sample distribution different from $\pi_\theta$ | Rollout with temperature 1, top-p 1, top-k disabled |

#### 1.8 Practice: Implementing REINFORCE on a Small LM

Assume the following virtual environment:

-   Apple Silicon Mac (MPS) or single NVIDIA GPU, Python 3.11
-   Model: `HuggingFaceTB/SmolLM2-135M-Instruct` (approx. 135M parameters, runs slowly even on CPU)
-   Task: Reward 1 if the response to prompt "Write one short sentence about a fruit." contains "banana", else 0.

Environment setup commands:

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

File structure:

```text
rl-stage1/
├── common.py    # reward, rollout, log-prob, loss
├── train.py     # REINFORCE training loop
└── gradvar.py   # gradient variance measurement per baseline (1.9)
```

`common.py` contains rollout, mask, log-prob, advantage, and loss.

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
    model.eval()  # only turns off dropout, gradient calculation still works
    return tok, model

def reward_fn(text: str) -> float:
    # 1 if keyword is in response, else 0
    return 1.0 if KEYWORD in text.lower() else 0.0

@torch.no_grad()
def rollout(model, tok, n, max_new_tokens=48):
    # prompt length is same as same prompt is replicated n times, no padding needed
    msgs = [{"role": "user", "content": PROMPT}]
    text = tok.apply_chat_template(msgs, tokenize=False, add_generation_prompt=True)
    enc = tok([text] * n, return_tensors="pt", add_special_tokens=False).to(model.device)
    seqs = model.generate(
        **enc,
        do_sample=True,
        temperature=1.0, top_k=0, top_p=1.0,  # match sample distribution with pi_theta
        max_new_tokens=max_new_tokens,
        pad_token_id=tok.pad_token_id,
    )
    return seqs, enc["input_ids"].shape[1]

def response_mask(seqs, prompt_len, eos_id):
    # 1 only up to first EOS in response section, 0 for subsequent padding
    resp = seqs[:, prompt_len:]
    is_eos = (resp == eos_id).long()
    after_eos = (is_eos.cumsum(dim=1) - is_eos) > 0
    return (~after_eos).float()  # [N, T_resp]

def response_logprobs(model, seqs, prompt_len):
    logits = model(seqs).logits[:, :-1, :]  # logits at position t predict t+1-th token
    targets = seqs[:, 1:]
    logp = torch.log_softmax(logits.float(), dim=-1)
    token_logp = logp.gather(-1, targets.unsqueeze(-1)).squeeze(-1)
    return token_logp[:, prompt_len - 1:]  # keep only response token positions, [N, T_resp]

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
    seq_logp = (token_logp * mask).sum(dim=1)  # log pi(y|x) per response
    return -(seq_logp * adv.detach()).mean()
```

Points to check in `response_logprobs`:

-   One-step shift: For causal LMs, output at position $t$ is the distribution for the $t+1$-th token, so `logits[:, :-1]` and `seqs[:, 1:]` are aligned.
-   Reason for slicing from `prompt_len - 1`: After shifting, index $j$ refers to the $j+1$-th token of the original sequence. So, the first response token (original index `prompt_len`) is at index `prompt_len - 1` after shifting.
-   `logits.float()`: Calculating `log_softmax` with bf16 can lead to significant precision loss.
-   Re-performing forward pass: `generate` does not leave a gradient graph, so the forward pass must be performed again with the same sequence.

`train.py` implements the operational sequence from 1.5 directly.

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
    p.add_argument("--n", type=int, default=16)  # number of responses per step
    p.add_argument("--lr", type=float, default=1e-5)
    p.add_argument("--seed", type=int, default=0)
    args = p.parse_args()

    torch.manual_seed(args.seed)
    tok, model = load()
    opt = torch.optim.AdamW(model.parameters(), lr=args.lr)

    for step in range(1, args.steps + 1):
        # 1. Sample with current policy
        seqs, prompt_len = rollout(model, tok, args.n)
        texts = tok.batch_decode(seqs[:, prompt_len:], skip_special_tokens=True)
        # 2. Calculate reward and advantage
        raw = torch.tensor([reward_fn(t) for t in texts], device=seqs.device)
        adv = compute_advantages(raw + args.reward_offset, args.baseline)
        # 3. Re-perform forward pass to calculate log-prob and loss
        mask = response_mask(seqs, prompt_len, tok.eos_token_id)
        token_logp = response_logprobs(model, seqs, prompt_len)
        loss = reinforce_loss(token_logp, mask, adv)
        # 4. Update and discard samples (on-policy)
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

Run for each baseline and reward offset setting.

```bash
mkdir -p logs
python train.py --baseline none --steps 100 | tee logs/none.log
python train.py --baseline mean --steps 100 | tee logs/mean.log
python train.py --baseline loo  --steps 100 | tee logs/loo.log
python train.py --baseline none --reward-offset 5 --steps 100 | tee logs/none_off5.log
python train.py --baseline mean --reward-offset 5 --steps 100 | tee logs/mean_off5.log
```

The output below is a hypothetical example. Assume pre-training reward is about 0.12.

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

Criteria for reading logs:

-   reward: Main indicator for judging if learning signal is being delivered.
-   loss: Normal for sign to fluctuate, irrelevant to performance as it's a surrogate loss.
-   grad_norm: Magnitude before clipping. If it suddenly increases, suspect advantage calculation or mask error.
-   len: If length moves in one direction regardless of reward, it might be due to loss aggregation or reward design.
-   sample: Don't just look at numbers, check actual output.

Assumed results for each setting (hypothetical values):

| Setting | Step 100 Reward | Interpretation |
|---|---|---|
| none, offset 0 | 0.56 | Incorrect responses have 0 weight, only correct responses are boosted. |
| mean, offset 0 | 0.81 | Correct responses are boosted, incorrect ones are suppressed. |
| loo, offset 0 | 0.81 | Same direction as mean, slightly different magnitude. |
| none, offset 5 | 0.21 | Learning is slow due to large noise trying to boost all responses. |
| mean, offset 5 | 0.81 | Offset disappears when mean is subtracted. Almost same trajectory as mean, offset 0 with same seed. |

AdamW normalizes gradient magnitudes and grad clipping also limits magnitude, so the reward curve partially masks variance differences. Variance itself is measured with raw gradients in 1.9.

Running training for a long time reveals flaws in toy rewards (hypothetical example):

```text
$ python train.py --baseline mean --steps 300
...
step  300 | reward 1.000 | loss +0.0000 | grad_norm 0.00 | len 6.0
  sample: 'Banana banana banana.'
```

Two phenomena appear together:

-   Reward hacking: Reward function is satisfied, but not the intended "natural fruit sentence." This is addressed by KL penalty with a reference model and reward design.
-   Loss of learning signal: All responses have reward 1, so advantage is all 0, and grad_norm becomes 0 (1.6).

#### 1.9 Practice: Comparing Gradient Variance by Baseline

We fix parameters, sample batches $K$ times to get gradient estimates $\hat{g}_1, \dots, \hat{g}_K$, and measure their dispersion.

$$
\mathrm{tr}\,\mathrm{Cov}[\hat{g}] = \mathbb{E}\left[\lVert \hat{g} \rVert^2\right] - \lVert \mathbb{E}[\hat{g}] \rVert^2
$$

Reading: $\mathrm{tr}\,\mathrm{Cov}$ (trace cov) is the sum of variances of each element of the gradient vector. It is calculated by subtracting the squared norm of the mean gradient from the mean of the squared gradient norm.

All settings' gradients must be calculated with the same rollout to prevent sample differences from mixing into the comparison.

```python
# gradvar.py
import torch
from common import (load, reward_fn, rollout, response_mask, response_logprobs,
                    compute_advantages, reinforce_loss)

CONFIGS = [("none", 0.0), ("none", 5.0), ("mean", 0.0), ("mean", 5.0), ("loo", 0.0)]

def main(k=16, n=16):
    torch.manual_seed(0)
    tok, model = load()
    params = list(model.model.layers[-1].parameters())  # measure only last layer to save memory
    g_sum = {c: 0.0 for c in CONFIGS}
    sq_sum = {c: 0.0 for c in CONFIGS}

    for _ in range(k):
        # parameters fixed, calculate gradients for all settings with same rollout
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

The output below is a hypothetical example.

```text
$ python gradvar.py
none offset 0 | total_var 2.41e-03 | x1.00
none offset 5 | total_var 5.87e-02 | x24.36
mean offset 0 | total_var 1.62e-03 | x0.67
mean offset 5 | total_var 1.62e-03 | x0.67
 loo offset 0 | total_var 1.84e-03 | x0.76
```

Results are interpreted as follows:

-   `none offset 5` is tens of times larger than `offset 0`: Adding a constant keeps the expected gradient the same but greatly increases variance (1.6).
-   `mean offset 0` and `5` are exactly the same: Subtracting the mean precisely removes the constant, which can be used as a sanity check for implementation.
-   `loo` is about 1.14 times `mean`: The advantage of `mean` is $\frac{15}{16}$ times that of `loo`, so variance differs by exactly $(\frac{16}{15})^2 \approx 1.14$ times, and the direction is the same.
-   Values vary depending on accuracy and response length; judgment should be based on ratios and relative magnitudes.

The content of 1.7 can also be verified with the same batch. At $\theta = \theta_\text{old}$, the gradients of importance sampling loss and REINFORCE loss should be identical.

```python
# Run inside the loop of gradvar.py
ratio = torch.exp(token_logp - token_logp.detach())  # values are all 1, gradient is log pi's gradient
adv = compute_advantages(raw, "mean")
loss_is = -((ratio * mask).sum(dim=1) * adv).mean()
loss_pg = reinforce_loss(token_logp, mask, adv)
g_is = torch.autograd.grad(loss_is, params, retain_graph=True)
g_pg = torch.autograd.grad(loss_pg, params, retain_graph=True)
print(max((a - b).abs().max().item() for a, b in zip(g_is, g_pg)))  # 0 or floating point error level
```

The values of the two losses are different, but their gradients are the same. This practice confirms that surrogate loss is designed by its gradient, not its value.

#### 1.10 Common Mistakes

| Symptom | Cause | How to Check |
|---|---|---|
| Responses break immediately after training starts | Missing shift between logits and targets | Check if average log-prob of response tokens before training is roughly -0.5 to -3. If near -10 (uniform distribution level for vocab 49152), it's looking at probabilities of wrong tokens. |
| Training unstable, large grad_norm | Prompt tokens included in loss | Check if `mask.sum(1)` equals actual number of response tokens. |
| Response length changes abnormally | Padding after EOS included in loss | Check if mask is 0 for positions after EOS. |
| `element 0 of tensors does not require grad` error | Calculating loss directly from `generate` output | Re-perform forward pass with the same sequence. |
| Incorrectly judging training failure | Viewing surrogate loss as performance metric | Judge by reward curve and sample output. |
| Symptoms not obvious, but gradient bias occurs | Default top-k, top-p in generation config applied | Check with `print(model.generation_config)` and explicitly set rollout configurations. |
| Reward doesn't move at all | All batch rewards are same, advantage is 0 | Record reward standard deviation along with reward at each step. |
| Loss is NaN | Applying `log_softmax` directly to bf16 logits | Calculate after `logits.float()`. |
| Reward is 1.0 but response is strange | Flaw in reward function (reward hacking) | Periodically check sample output. |

#### 1.11 Completion Checklist and Resources

You have completed this stage if you can explain or write the following items without external resources:

-   Define state, action, transition, and reward for LLM response generation, and explain the meaning of deterministic transition (1.2).
-   Explain why all tokens have the same return in sparse reward and $\gamma = 1$ (1.3).
-   Calculate the advantage for correct and incorrect responses for a prompt with 30% accuracy (1.3).
-   Write the 4-line derivation to transform $\nabla_\theta J$ into expectation form using the log-derivative trick (1.4).
-   Explain why policy gradient can be calculated without knowing environment transitions (1.4).
-   Explain why the surrogate loss value is not a performance metric (1.5).
-   Explain why a baseline does not introduce bias using the EGLP lemma (1.6).
-   Explain what changes when a constant is added to the reward, depending on the presence of a baseline (1.6, 1.9).
-   Explain why the gradient of importance sampling loss is the same as REINFORCE when $\theta = \theta_\text{old}$ (1.7).
-   Explain why top-p and top-k are turned off in rollout (1.7).
-   Explain why shift and `prompt_len - 1` slicing are needed in token log-prob calculation (1.8).

| Resource | Sections to Read | What to Gain from This Stage |
|---|---|---|
| OpenAI Spinning Up "Key Concepts in RL" | Entire | Terminology and notation, value function and Bellman equation |
| OpenAI Spinning Up "Intro to Policy Optimization" | Entire, especially EGLP lemma and reward-to-go | Derivation process for 1.4-1.6 |
| Sutton & Barto Chapter 13 | Read 13.1-13.4 thoroughly, skim 13.5 onwards | General form of policy gradient theorem, REINFORCE with baseline |

Sutton & Barto uses notation like $\pi(a \mid s, \boldsymbol{\theta})$, which is slightly different from Spinning Up, so read it by comparing with the notation table in 1.1.

#### 1.12 Summary

-   LLM generation is an MDP where state is prompt and prefix, action is next token, and reward is sparse reward at the end of the response.
-   Response log-prob is the sum of token log-probs, calculated only for the response segment using shift and mask.
-   Advantage indicates how much better an action is compared to the average; positive means increased probability, negative means decreased.
-   Policy gradient is "maximum likelihood weighted by reward" obtained via the log-derivative trick.
-   REINFORCE loss is `-(seq_logp * adv.detach()).mean()`, and the loss value is not a performance metric.
-   Baseline reduces variance without changing the expected gradient; group mean leads to GRPO, leave-one-out to RLOO, and critic to PPO.
-   Importance ratio is a correction for reusing data from a previous policy; REINFORCE is a special case where the ratio is 1.

### 2. Policy Gradient and Actor-Critic

All current LLM RL algorithms are policy gradient-based, making this stage the starting point for the entire curriculum.

-   Key topics
    -   Policy gradient theorem, log-derivative trick
    -   REINFORCE
    -   Baseline and variance reduction
    -   Actor-Critic architecture
    -   GAE (Generalized Advantage Estimation)
    -   Entropy bonus
-   Content to skim: DQN-family value-based methods (rarely used in LLM RL)
-   Completion criteria: Able to implement REINFORCE on a simple environment like CartPole and verify variance differences with/without baseline.
-   Resources: Spinning Up "Intro to Policy Optimization", Sutton & Barto Chapter 13, GAE paper

### 3. PPO and KL Constraints

This is the standard algorithm for RLHF, and subsequent GRPO-family objectives inherit PPO's structure.

-   Key topics
    -   Importance sampling and policy ratio
    -   Trust region idea (TRPO only conceptually)
    -   PPO clipped objective, value loss
    -   Reference model and KL penalty
    -   Key hyperparameters: clip range, number of mini-batches, number of update epochs
-   Completion criteria: Able to explain the role of each term in PPO loss and identify anomalies from training logs (KL, clip fraction, entropy).
-   Resources: PPO paper (Schulman 2017), "The 37 Implementation Details of Proximal Policy Optimization"

### 4. RLHF and Preference Optimization

This involves aligning models by converting human preferences into rewards. Understanding the difference between online RL and offline methods (DPO) is key.

-   Key topics
    -   SFT → reward model → PPO pipeline (InstructGPT)
    -   Bradley-Terry based reward model training
    -   Reward hacking and reward overoptimization
    -   DPO derivation and limitations
    -   Comparison of variations: IPO, KTO, SimPO, ORPO
    -   Trade-offs of online vs. offline preference optimization
-   Completion criteria: Able to explain the flow from RLHF objective to DPO loss derivation and run DPO training on a small model with TRL.
-   Resources: InstructGPT paper, DPO paper, Nathan Lambert "RLHF Book"

### 5. RLVR and GRPO-Family Algorithms

This focuses on training reasoning abilities with verifiable rewards (RLVR, RL with Verifiable Rewards), which is currently central to reasoning model training.

-   Key topics
    -   Verifiable reward design: math answer verification, unit tests, format reward
    -   GRPO: group-wise relative advantage, critic removal
    -   Critic-free alternatives: RLOO, REINFORCE++
    -   GRPO improvements: DAPO, Dr. GRPO, GSPO
    -   Common issues: length bias, entropy collapse, loss of learning signal for easy/hard prompts
    -   Discussion on whether RL creates new capabilities or sharpens existing distributions (pass@1 vs. pass@k)
-   Completion criteria: Able to run GRPO training on a small model with GSM8K-level math data and interpret reward curves and response length changes.
-   Resources: DeepSeekMath (proposing GRPO), DeepSeek-R1, DAPO, "Understanding R1-Zero-Like Training" (Dr. GRPO)

### 6. Agentic RL and Training Infrastructure

This stage covers training tool use and multi-turn interactions with RL, along with understanding the infrastructure for actual training.

-   Key topics
    -   Multi-turn rollout, trajectory construction including tool call results
    -   Environment design: sandbox, code execution, external APIs
    -   Trajectory-level reward and credit assignment
    -   LLM-as-a-judge, rubric-based rewards and their risks
    -   Separation of rollout engine (vLLM, SGLang) and trainer
    -   Async RL and off-policy correction
    -   Framework structure comparison: TRL, verl, OpenRLHF
-   Completion criteria: Able to define one tool-calling environment and construct a training loop including multi-turn rollout.
-   Resources: verl, TRL, OpenRLHF official documentation and example code

### Summary

-   Stages 1-3: Minimum foundation from classical RL needed for LLM RL.
-   Stages 4-5: Core of LLM post-training, currently the largest portion in practice.
-   Stage 6: Competence in configuring and debugging actual training environments.
-   If time is short, compress stages 1-2 and focus on stages 3 and 5.
-   Move to the next stage after passing the completion criteria for the current stage.
