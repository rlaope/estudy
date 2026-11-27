# Speculative Decoding

When an LLM generates a single token, it must read the entire model weights from memory.

During this process, most of the GPU's computational power remains idle.

Speculative decoding is a method that utilizes this idle computational power.

**A small, fast 'drafter' speculatively generates several next tokens, and the larger model then scores them all at once, accepting only up to the first incorrect token.**

Since multiple tokens can be obtained by running the large model just once, it speeds up the process. If the scoring rules are well-designed, the resulting distribution is mathematically identical to what the large model would generate on its own.

| Term | Meaning |
|---|---|
| target model | The large model originally intended for use. The "scorer". |
| draft (drafter) | The component that speculatively predicts the next tokens. It can be a small model or a small head attached to the target. |
| k (num_speculative_tokens) | The number of tokens predicted at once. |
| accept / reject | Whether the target accepts the draft tokens. |
| acceptance rate | The probability that a single draft token is accepted. |
| mean accepted length (MAL) | The average number of tokens obtained in one verification round. |
| lossless | Means the output distribution is exactly the same as that of the target alone. |

## Why is Decoding Slow?

> Decoding is bound by memory reads, not computation, leaving most of the GPU's computational power idle.

#### Analogy: Someone Flipping Through an Entire Encyclopedia Every Time

Imagine someone writing a text, and every time they write a single character, they flip through an entire encyclopedia from beginning to end.

The actual time to write a character might be 1 second, but flipping through the dictionary takes 10 minutes. This person's speed is determined not by their writing speed, but by the speed of flipping through the dictionary.

But what if they could **check 5 characters simultaneously** while flipping through the dictionary once? The flipping time remains the same, but 5 times more work gets done. This is the point speculative decoding aims for.

Let's assume a 70B model running in bf16 on a single H100 (though it doesn't actually fit on one, this is for calculation intuition).

```
Weight size     : 70B × 2 byte = 140 GB
HBM bandwidth      : approx. 3.35 TB/s
→ One weight read : 140 / 3,350 ≈ 0.042 seconds = approx. 42 ms

Computation per token : approx. 2 × 70B = 140 GFLOPs
bf16 computation performance   : approx. 989 TFLOPS
→ Computation time      : 140 / 989,000 ≈ 0.00014 seconds = approx. 0.14 ms
```

**With memory reads at 42ms and computation at 0.14ms**, if the batch size is 1, the GPU's computational unit spends over 99% of its time waiting for weights to arrive.

This state is called memory-bound.

### The Opportunity This Creates

Once the weights are read, computing 5 tokens instead of 1 **barely increases the time**, because the computational unit was idle anyway. The problem is how to compute 5 tokens at once when we don't know what the next 5 tokens are, as the next token depends on the previous one.

Therefore, we first make a guess and then verify them all at once.

## Basic Operating Principle

> Draft predicts k tokens -> Target scores k tokens at once -> Accepts up to the point just before the first incorrect token + Target adds its own token at that position.

### Following One Round

Let's say the prompt is `"The weather today is"` and k = 4.

```
① Draft predicts 4 tokens:      sunny  and  warm  outside

② Target scores at once:  Inputs these 4 tokens and performs 1 forward pass.
                          Simultaneously calculates "what the target would have chosen here" at each position.

③ Compares from left:
   sunny   → Target also chose sunny  ✓ Accepted
   and     → Target also chose and    ✓ Accepted
   warm    → Target chose clear  ✗ Rejected → Stops here
   outside → (No need to check, discarded as previous was incorrect)

④ Result: "sunny and" accepted + target's chosen "clear" added
        → 3 tokens confirmed in this round.

⑤ The next round starts from "The weather today is sunny and clear".
```

Even though the target was run only once, 3 tokens were obtained. If generated normally, it would have taken 3 runs.

### Why Can the Target Score All at Once?

Because the predicted tokens are already fixed. If "sunny and warm outside" is input as a whole, all positions can be computed in parallel, similar to prefill when processing a prompt.

By looking at the target's next token predictions at each position, we can simultaneously check if the target would have chosen 'and' next if the previous was 'sunny'.

### Are There No Disadvantages?

- **Even if all are incorrect**, the target inserts its own token at the first position, so at least 1 token is obtained, just like normal generation.
- **If all are correct**, k + 1 bonus token (the target has already computed the token after the last position) = k + 1 tokens are obtained.

Thus, the tokens obtained in one round range from 1 to k + 1. The only costs are the time spent running the draft and the computational cost of discarded tokens. We will explore situations where this cost becomes significant below.

## When the Result is Identical to the Original Model

> For greedy, accept if identical; for sampling, accept based on probability ratio, and if rejected, resample from a corrected distribution. Thanks to these rules, the output distribution is exactly the same as that of the target.

### greedy (temperature = 0)

If the target is configured to always choose the highest probability token, it's simple: **if the draft token is the same as the target's top token, accept; otherwise, reject and use the target's top token.**

In any case, the final token is the target's top token, so the result does not differ by a single character.

### sampling (temperature > 0)

For probabilistic sampling, the 'accept if identical' rule should not be used, as it would lead to an excessive number of tokens frequently chosen by the draft. Therefore, these rules are used:

```
When draft chooses token x:

p(x) = probability target assigned to x
q(x) = probability draft assigned to x

1) Accept with probability min(1, p(x) / q(x))
2) If rejected, resample from a distribution normalized such that max(0, p - q) sums to 1.
```

1. Tokens chosen by the draft with less confidence than the target (q <= p) are always accepted.
2. Tokens chosen by the draft with excessive confidence compared to the target (q > p) are accepted with a reduced probability proportional to the excess.
3. If rejected, the deficit is filled by resampling from tokens where the draft chose less (p - q is positive).

### Verification with Numbers

Let's assume there are only three tokens: A, B, C.

```
target p = [0.4, 0.4, 0.2]
draft  q = [0.6, 0.3, 0.1]
```

The draft excessively favors A and less favors B and C.

#### Acceptance Probability

```
A: min(1, 0.4 / 0.6) = 0.667   (reduced due to excess)
B: min(1, 0.4 / 0.3) = 1       (always accepted)
C: min(1, 0.2 / 0.1) = 1       (always accepted)
```

**Rejection Probability** (Only when A is chosen and rejected)

```
0.6 × (1 - 0.667) = 0.6 × 0.333 = 0.2
```

Distribution for resampling after rejection

```
max(0, p - q) = [max(0, -0.2), max(0, 0.1), max(0, 0.1)] = [0, 0.1, 0.1]
Normalized        = [0, 0.5, 0.5]
```

The final probability of each token appearing is

```
A: draft chooses A and accepted               = 0.6 × 0.667        = 0.4
B: draft chooses B and accepted + B after rejection    = 0.3 × 1 + 0.2 × 0.5 = 0.4
C: draft chooses C and accepted + C after rejection    = 0.1 × 1 + 0.2 × 0.5 = 0.2

Final = [0.4, 0.4, 0.2]  =  Exactly same as target p ✓
```

No matter how poor the draft is, the final distribution is identical to the target's. The quality of the draft only affects **how much faster it becomes, not the correctness of the result.**

This is why speculative decoding is called lossless.

### Caution: May Not Be Lossless Depending on Engine Settings

Some serving engines offer options to loosen acceptance criteria for speed.

For example, vLLM has a `rejection_sample_method` option, and its default value is `strict`.

If non-default settings are used, the results might differ from the target alone, so check the settings in evaluations where accuracy is critical.

## Calculating the Speedup

> Speedup = Tokens obtained per round / Time taken per round. Higher acceptance rate and cheaper draft are better, and a larger k is not always better.

### Tokens Obtained Per Round

Simplifying by assuming the acceptance probability 'a' is the same for each token, the average number of tokens obtained in one round when k tokens are predicted is as follows:

```
Average tokens = (1 - a^(k+1)) / (1 - a)
```

`a^(k+1)` means 'a' multiplied by itself k+1 times; refer to the table below.

| Acceptance Rate a | k=1 | k=2 | k=4 | k=8 |
|---|---|---|---|---|
| 0.6 | 1.60 | 1.96 | 2.31 | 2.47 |
| 0.8 | 1.80 | 2.44 | 3.36 | 4.33 |
| 0.9 | 1.90 | 2.71 | 4.10 | 6.13 |

To read this: if the acceptance rate 'a' is 0.8, then with k=4, an average of 3.36 tokens are obtained.

The reason the effect diminishes as k increases is that if one token is incorrect, all subsequent tokens are discarded. If the acceptance rate is 0.8, the probability of surviving up to the 5th token is about 0.33 (0.8 multiplied by itself 5 times).

### Time Taken Per Round

```
Round time ≈ target 1 run + draft k runs
```

If the cost of one draft run is 'c' times the cost of one target run, then the round time is `1 + k x c`. This assumes that for small batches, the cost for the target to verify k+1 tokens is almost the same as generating 1 token.

### Speedup

```
Speedup ≈ Average tokens / (1 + k × c)
```

| Condition | k=1 | k=2 | k=4 | k=8 |
|---|---|---|---|---|
| a=0.8, c=0.05 (draft very cheap) | 1.71× | 2.22× | 2.80× | 3.09× |
| a=0.8, c=0.2 (draft somewhat expensive) | 1.50× | 1.74× | 1.87× | 1.66× |
| a=0.6, c=0.2 | 1.33× | 1.40× | 1.28× | 0.95× |

What can be learned from these results:

- If the draft is cheap, k can be set higher.
- If the draft is expensive, increasing k might eventually slow it down. For k=8 in the bottom row, it's less than 1, meaning it's worse than not using it at all.
- Therefore, making the draft as cheap as possible and achieving a high acceptance rate can be considered the entire focus of research in this area. We will look at these attempts below.
- The optimal k value varies by task. In actual measurements, increasing k did not always improve performance, and the optimal value depended on the model and workload.

### Acceptance Rate Varies by Task

Even with the same draft, the acceptance rate can vary significantly depending on the task.

- **Well-suited tasks:** Code, math problems, structured formats (JSON), summarization/editing that heavily copies input. The next token is often predictable.
- **Poorly-suited tasks:** Creative writing, poetry, free-form conversation, etc., where the next token choices are broad, so it's not as effective.
- **Higher temperature** leads to a lower acceptance rate. Since the target itself samples randomly, it's harder to predict correctly.

In experiments attaching DSpark to SGLang, the verification length was automatically adjusted according to difficulty. For GSM8K (math), an average of about 5.24 tokens were verified; for arena-hard (general conversation), about 3.78 tokens; and for poetry writing, about 2.91 tokens. The difference between tasks is this significant.

## Methods for Creating Drafts

> Separate small model -> String matching without a model -> Attaching small heads to the target (EAGLE, MTP) -> Predicting multiple tokens at once (DFlash, DSpark) in order of development.

| Method | Draft Identity | Additional Training | Characteristics |
|---|---|---|---|
| Separate draft model | Small model of the same family (e.g., 1B for 70B) | Usually unnecessary | Simplest. Must use the same tokenizer. Draft is relatively expensive. |
| n-gram / prompt lookup | No model. Finds and appends matching strings from input. | Unnecessary | Almost free. Effective only for tasks that copy input. |
| suffix decoding | No model. Pattern tree of previous outputs. | Unnecessary | Strong for repetitive agent tasks. |
| Medusa | Multiple heads on top of the target (predicting +1st, +2nd, +3rd tokens respectively) | Only heads trained | Pre-EAGLE generation |
| EAGLE family (EAGLE-3) | Small layer receiving target's hidden state | Only draft trained | Most commonly used for general purposes |
| MTP | Prediction module built into the model from pre-training | Already included | Easiest choice if the model supports it |
| DFlash / DSpark | Draft that predicts multiple tokens at once in block units | Draft training | Latest in 2026. Significantly reduces draft cost. |

### Separate Draft Model

This was the first method, from a 2023 Google DeepMind paper. It uses a small model with the same tokenizer as the draft.

- **Pros:** Uses an existing small model.
- **Cons:** The draft is also an independent model, so it uses its own KV cache and memory. Also, a small model doesn't know the large model's 'thoughts' and predicts on its own, limiting the acceptance rate.

### Model-less Methods: n-gram, Suffix Decoding

**n-gram (prompt lookup)** is a method that finds strings in the prompt identical to the last few generated tokens and uses the subsequent tokens from the prompt directly as predictions.

```
If the prompt contains "def calculate_total(items):" and
the model has just written "def calculate_"
→ It finds "total(items):" in the prompt and submits it as a prediction.
```

It is highly effective for **tasks that heavily copy input**, such as code modification, document editing, or RAG responses. The draft cost is almost zero, so it imposes little burden even during high traffic.

Suffix decoding builds a Suffix Tree to quickly search past output history and dynamically adjusts the prediction length based on the matching length.

vLLM documentation also classifies n-gram and suffix decoding as methods that offer small speed improvements but do not increase overhead during peak traffic.

### EAGLE Family

The idea is to let the draft predict not on its own, but by looking at the target's internal information (hidden state) that has already been computed.

When the target generates a token, its last layer produces a vector containing rich information about what comes next.

EAGLE receives this vector and uses a small single layer to predict the next token. By inheriting the target's 'thoughts', even a small draft achieves a high acceptance rate.

### Changes in EAGLE-3 (NeurIPS 2025)

- Previous EAGLE was trained to "predict the next hidden state," which became a constraint, preventing performance from scaling well even with more training data. EAGLE-3 abandons this constraint and **directly predicts tokens.**
- Instead of using only the last layer, it **mixes hidden states from early, middle, and late layers.**
- **training-time test:** This technique involves exposing the draft to situations during training where it uses its own output as input again, mimicking real inference conditions to align training and inference.

According to the paper, it was up to 6.5 times faster and about 1.4 times faster than EAGLE-2, and throughput increased by 1.38 times with batch size 64 in SGLang.

### MTP (Multi-Token Prediction)

The idea is not to attach the draft later, but to build the model from initial training to predict the next-next tokens as well.

It became widely known with DeepSeek V3, and now many latest models like DeepSeek V4, Qwen3-Next, and Qwen3.5 series come with MTP modules by default.

During the pre-training phase, it serves to augment the training signal, and during inference, it is used directly as the draft.

Its operation is similar to EAGLE, combining the target'
