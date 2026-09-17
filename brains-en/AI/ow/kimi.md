# Kimi

Scale is the challenger on the front lines.

Founded in Beijing in March 2023, its founder Yang Zhilin is an NLP researcher from Tsinghua University.

The co-founders are also Tsinghua alumni, and it received investment from Alibaba and HongShan.

**From the beginning, they bet on two things: long-context agent capabilities, and their first version in 2023 garnered attention for its then-unprecedented 128,000 token context, a path they continue to pursue.**

| Period | Model | Features |
|------|------|------|
| 2025.1 | K1.5 | Math, coding, multimodal reasoning |
| 2025.7 | K2 (1T/32B) | 8 active out of 384 experts. 0 loss spikes over 15.5T tokens with MuonClip. Agent data synthesis pipeline, RLVR + self-criticism rubric |
| 2025 | Kimi Linear | First implementation of KDA (48B scale) |
| 2026.1 | K2.5 | Native multimodal (vision+text 15T tokens). Agent Swarm (up to 100 agents in parallel) |
| 2026.4.20 | K2.6 (1T/32B) | Architecture identical to K2.5, enhanced post-training. Video input, INT4 QAT, 300 sub-agents / 4,000 steps / 12+ hours autonomous execution |
| 2026.7.16 | K3 (2.78T/104B) | World's first open 3T-class model. KDA + AttnRes + Stable LatentMoE, 16 active out of 896 experts, 1M context, NoPE, MXFP4 |

> K2.5 service ended on May 25, 2026. The live rotation is incredibly fast.

The design philosophy is "continuously pushing the upper limits of scale. And making models work as long as humans do."

According to Moonshot's self-description, for 9 out of the past 12 months, **Kimi has defined the upper limit of open model size. Building large models is its very identity.**

Simultaneously, there is a second axis. For models that work for a long time, K2.6 championed continuous autonomous execution for over 12 hours, and K3 was released with its thinking mode always on.

### Implementation Techniques

- **MuonClip / Per-Head Muon**
- **KDA(Kimi Delta Attention):** Refines Gated DeltaNet by gating it channel-wise and uses Gated MLA across the entire attention layer, accelerating decoding by up to 6.3x at 1 million tokens.
- **AttnRes(Attention Residuals)**: **Selectively retrieves** residuals in the depth direction, improving learning efficiency by approximately 25% with less than 2% additional cost.
- **Stable LatentMoE + Quantile Balancing**
  - Extreme sparsity of 1.8 percent, with only 16 out of 896 experts active
  - **LatentMoE:** Compresses large linear layers into lower dimensions (down-projection) like MLA. It compresses the expert parameters themselves.
  - **Quantile Balancing:** Directly derives expert allocation from the quantiles of router scores, eliminating heuristic updates and sensitive balance hyperparameters.

Other K3 features include:
- **NoPE:** Removes all RoPE, proceeding without positional embeddings.
- Uses the **SiTU** activation function
- Ships 4-bit weights with **MXFP4** (based on quantization-aware training)
- Overall, it claims a 2.5x increase in scaling efficiency compared to K2, extracting 2.5x more intelligence with the same compute.

### AgentSwarm - A Unique Asset

This is Kimi's differentiator.

```
General Model:  [Task] → Step 1 → Step 2 → Step 3 → ... → [Complete]   (Sequential)

Agent Swarm: [Task]
                ├─→ Sub-agent 1 ─┐
                ├─→ Sub-agent 2 ─┤
                ├─→ Sub-agent 3 ─┼→ [Coordination·Integration] → [Complete]
                ├─→ ...            │
                └─→ Sub-agent N ─┘   (Parallel, N up to 300)
```

The concept was introduced in K2.5 (up to 100 agents) and expanded in K2.6 to 300 sub-agents / 4,000 steps of coordination. It was reported that parallel processing shortens execution time by 4.5 times.

<br>

### Strengths

**Strength 1 - World leader in frontend UI coding.** K3 ranked 1st in the Frontend Code Arena with 1,679 points, surpassing top commercial models, and its predecessor K2.6 was 18th on the same board, marking a dramatic leap.

> Why is that? Frontend has no single correct answer and falls into the realm of **aesthetic judgment and user experience**. Kimi's self-criticism rubric reward allows the model to learn by evaluating its own quality on tasks without a definitive answer + it mass-learns a loop of creating -> observing -> correcting through an agent data synthesis pipeline + native vision -> it can actually see and judge the UI it created = it has strengths in areas that are difficult to verify.

**Strength 2 - Long-term autonomous execution.** Over 12 hours and 4,000 steps of coordination has no equivalent in other models.
Learning stability secured by Muonclip -> completes very long training without incidents + agent synthesis pipeline -> artificially mass-produces long-trajectory data + AttnRes -> information is transmitted without loss in deep models + AgentSwarm -> distributes failure points through parallel division instead of a single long chain.

**Strength 3 - Multimodal** Trained with an integrated text and image architecture since K2.5 (vision + text 15 trillion tokens), with video added in K2.6d. Among the four families in this note, it is truly multimodal along with Qwen.

**Strength 4 - Tool use and web browsing** Strong in BrowseComp and has excelled in tool-calling benchmarks like Tau2-Bench and ACEBench since the K2 series.

**Strength 5 - Cache efficiency**. Moonshot reports over 90% cache hit rate in coding workloads, significantly reducing effective input costs.

### Weaknesses

- **Expensive** K3 costs $15 per 1 million tokens, making it the most expensive among the four models. While $1 can buy approximately 1.15 million output tokens from DeepSeek V4 Pro and 227,000 from GLM 5.2, it only buys about 67,000 from K3. The common perception that open-weight models are cheap does not apply to K3.
- **Slow** In independent measurements, it generates approximately 62 tokens, which is less than half of GLM-5.2 (approx. 168), requiring separate tuning.
- **Self-hosting is virtually impossible** The 2.8T model is designed on the premise of a 64-accelerator-class supernode infrastructure, and even INT4 weights are around 594GB.
- **License is not pure MIT** It's a Modified MIT license, requiring Kimi K2 branding on the UI if monthly active users exceed 100 million or monthly revenue exceeds $20 million. While irrelevant for most organizations, it needs verification for consumer products.
- **Less verified:** K3 was just released, and a complete technical report is currently pending. Figures like KDA 6.3 and AttnRes 25% are all vendor announcements and have not been independently reproduced. Agent Swarm's claims of autonomous execution also require direct verification before production deployment.
- The lineup rotates quickly, and there is a risk of retirement. K2.5 was discontinued in 5 months.

### When to Choose It

O: Advantageous for code generation requiring frontend UI design sensibility
Autonomous execution workflows spanning several hours, parallel multi-agent orchestration, or agents handling images and videos together
Advantageous as a web browsing research agent
X: Not recommended for cost-sensitive batch processing, low-latency interactive responses, when self-hosting is essential, or when verified stability is the top priority.
