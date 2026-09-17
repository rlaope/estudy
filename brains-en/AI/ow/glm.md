# GLM

A sniper specialized in agentic coding.

Developed by Zhipu AI, a spin-off from Tsinghua University's KEG Lab (Knowledge Engineering Group) in 2019.

The brand changed to Z.ai around 2025, and GLM, short for General Language Model, has a long lineage dating back to 2019.

In January 2026, it raised 43.5 billion Hong Kong dollars (approximately 560 million USD) through a Hong Kong IPO, with the funds directly allocated to GLM-5 development. The fact that it is **on the U.S. Entity List** is a point to consider when reviewing its adoption.

| Period | Model | Features |
|---|---|---|
| 2022 | GLM-130B | One of the early large open models in China |
| 2023~24 | GLM-4 series | Expansion of various open and commercial models like GLM-4, GLM-4-9B |
| 2025.7 | GLM-4.5 (355B/32B), Air (106B/12B) | Deep-over-wide, slime RL infrastructure released, MIT |
| 2026.2.11 | GLM-5 (744B/40B) | 28.5T tokens, DSA adopted, 200K context |
| 2026.4 | GLM-5.1 | Improved inference, coding, agent performance, GLM-5 series update |
| 2026.6.13 | GLM-5.2 (744B/40B) | 1M context with IndexShare, MTP improvement, High/Max effort |

ARC - Agentic, Reasoning, Coding is the philosophy of achieving these three in a single model.

GLM does not aim to be a general-purpose assistant; instead, it explicitly targets **agents that self-correct code through multiple steps** and pours all resources into that goal.

Z.ai describes this as a **transition from "vibe coding" to "true agentic engineering."**

### Implementation Techniques: GLM's Three Unique Choices

**Deep over Wide** is a choice to go deep rather than wide.

This is GLM's signature approach; while models like DeepSeek-V3 and Kimi K2 increase width (hidden dimension, number of experts), GLM went the opposite way.

By reducing the number of experts and hidden dimensions, and instead increasing the number of layers.

The rationale comes from experimental observations: for the same computational budget, deeper models perform better at reasoning.

By analogy, a wide model is like someone who considers many things at once, while a deep model is like someone who repeatedly re-evaluates thoughts through multiple stages. The latter is more advantageous for multi-step logic.

**Unusually Many Attention Heads**

GLM-4.5 has 5,120 hidden dimensions and 96 attention heads, which is 2.5 times more than usual.

An interesting point here is that increasing heads **does not improve training loss.** However, reasoning benchmarks like MMLU and BBH consistently show improvement.

This is a textbook example of the optimization vs. generalization trade-off. It demonstrates that lower loss does not always equate to a better model. This improvement would have been missed if the design had focused solely on training objective metrics.

GLM-5.2 configuration: 78 layers, 6,144 hidden dimensions. The front blocks use dense FFN, while the rest use 8 out of 256 routing experts + 1 shared expert. Attention combines MLA + DSA.

- **slime - Leveraging RL infrastructure**: Asynchronous disaggregated mode is key. It rapidly streams rollouts with FP8 to fill buffers and performs updates stably with BF16.
- **IndexShare (GLM-5.2)**: Four layers share one indexer, reducing FLOPs per token by 2.9x in a 1M context. By enabling IndexShare from the intermediate training stage with 128K sequences, **computation was reduced, but long-context performance improved.**
- **Data Strategy**: Based on GLM-4.5, the total scale is 23 trillion tokens, of which **7 trillion tokens are dedicated to code and reasoning.** During intermediate training, sequences are extended to 128K using repository-level code and synthetic reasoning data. Data curation is precise, including SemDeDup (semantic-based deduplication) and upsampling by quality grade. GLM-5 expanded to 28.5 trillion tokens.

> Disaggregated mode in LLM RL is a method to maximize productivity and GPU utilization by completely separating learning and data generation (rollout) physically and temporally, executing them asynchronously. Disaggregation prevents bottlenecks by using dedicated nodes for training, dedicated nodes for inference, and separating PD (Prefill, Decode).

<br>

### Strengths

**Strength 1** is long-term agentic coding. It scores 81.0 (vendor-reported) on Terminal-Bench 2.1, just 4 points behind the top model at 85.0. With 62.1 on SWE-bench Pro, it is **among the top open-weight models.**

> Causal chain: deep-over-wide -> structure favorable for multi-step reasoning + high number of heads -> improved reasoning on benchmarks (benefits not visible in loss) -> concentrated investment of 7 trillion tokens in code/reasoning -> domain density + slime asynchronous RL -> makes slow, long-term tasks actually learnable + expert training -> integrated training -> maximizing and combining signals from each domain = an agent that doesn't falter even after 300 steps.

**Strength 2** is cost-effectiveness. It ranks first in the open-weight category of the Artificial Analysis intelligence index, with a price about a quarter of frontier models. The vendor claims it outperforms commercial models in certain long-context coding benchmarks at one-sixth the cost.

**Strength 3** is speed. Independent measurements show GLM-5.2 processing approximately 168 tokens per second, significantly outperforming DeepSeek V4 Pro and Kimi K3 (both at 62). This is advantageous for latency-sensitive applications like interactive coding tools.

**Strength 4** is practical 1M Context. Thanks to IndexShare + MTP optimization, 1M context is not just for advertising but genuinely usable.

**Strength 5** is clean MIT + tool integration. It can be plugged into Claude Code by simply changing the model name to LM-5.2[1m], and it also provides its own desktop agent called ZCode.

### Weaknesses

- **Weak in multimodal capabilities.** Although the model metadata includes a multimodal class, there is no verified vision input.
- **High reliance on vendor-reported benchmarks.** "Beat GPT!"-type figures are mostly Z.ai's own measurements, so rankings might differ in independent comprehensive metrics.
- **Service stability history.** After the GLM-5 launch, traffic surged tenfold, leading to severe limitations. Even paid plans experienced "Too much concurrency" errors, limiting concurrent requests to one during peak hours. This was followed by a public apology, compensation, and a phased rollout, with the stock price dropping by about 23%. This is a **direct blow for enterprises running multiple parallel agents.**
- **Geopolitical regulations:** The hosting API routes through Chinese infrastructure, and the company is on the U.S. Entity List. Regulated industries and public procurement may require self-hosting or Western hosting providers.
- Self-hosting 744B has a high barrier. General inference speed has disadvantages in certain areas compared to hybrid models due to architectural characteristics.

### When to Choose It

When you need a multi-file, multi-step coding agent, want to put an entire repository into context, require a responsive interactive coding tool, or aim to optimize performance-to-cost ratio.

X: Do not use if image/audio is required, if you need to run large-scale parallel agents based on a hosting API, or if you cannot send sensitive data.
