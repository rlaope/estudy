# DeepSeek

If we were to categorize them, they are masters of efficiency.

DeepSeek is a research organization that originated from a hedge fund (Huanfang Quant) in Hangzhou, China.

The organization is small, operates research-paper-centric, and has proven through its work that **constraints breed invention**.

In an environment where they couldn't acquire and use top-tier GPUs as much as they wanted, they closed the gap through architecture and system optimization.

**Most distinctive modus operandi:** Before releasing a model, they first **publish core technical papers.** In late 2025 to early 2026, they successively released three papers on mHC, Engram, and DSA, which allowed people to combine them and predict the design of V4.

The same pattern was observed with V3.

| Period | Model | Introduced |
|------|------|-----------|
| 2024 | V2 (236B/21B) | MLA, DeepSeekMoE (fine-grained + shared experts) |
| 2024.12 | V3 (671B/37B) | Load balancing without auxiliary loss, MTP, first large-scale demonstration of FP8 training, DualPipe. 14.8T tokens, 2.78 million H800 hours |
| 2025.1 | R1 | Pure RL inference based on GRPO. Ushered in the era of "reasoning models" |
| Late 2025 | V3.2 (685B/37B) | DSA + Lightning Indexer. 128K context linearization |
| 2026.4.24 | V4 | Pro 1.6T/49B, Flash 284B/13B. mHC + Engram + Hybrid Sparse Attention + Muon. 1M context, MIT |

**Key integration in V4:** Previously, users had to choose between general-purpose V-series and specialized R-series.

V4 combines these two, allowing it to **adjust its inference depth based on task complexity.** Note that the legacy aliases deepseek-chat and deepseek-reasoner will be fully retired on July 24, 2026.

The design philosophy is to **find the cheapest path to achieve performance.**

- **Reason for creating MLA:** Because KV cache dominates service costs.
- **Reason for eliminating auxiliary loss:** Because it's a shame to sacrifice performance for balance.
- **Reason for using FP8:** Because the same results can be achieved with half the bits.
- **Reason for creating Engram:** Because reconstructing static facts through computation is wasteful.

Engram, in particular, is an extreme embodiment of this philosophy. While other companies ask how to become smarter, DeepSeek ponders what we are currently wasting.

```
DeepSeek-V4 Architecture (Conceptual Diagram)

  [Input]
     ↓
  ┌──────────────────────────────────┐
  │  Engram Conditional Memory (some layers)     │ ← O(1) hash lookup, can offload to DRAM
  ├──────────────────────────────────┤
  │  Hybrid Attention                 │
  │   · Compressed Sparse Attention                │ ← Select top k with Lightning Indexer
  │   · Highly Compressed Attention                   │
  │   · MLA-based (KV cache compression)         │
  ├──────────────────────────────────┤
  │  DeepSeekMoE (fine-grained + shared experts)  │
  ├──────────────────────────────────┤
  │  mHC Residual (Berkhoff Polytope Constraint)      │ ← Suppress signal amplification to less than 2x
  └──────────────────────────────────┘
     ↓
  [MTP Module] ← For speculative decoding
     ↓
  [Output]

Optimizer: Muon (transitioned from AdamW)
Precision: FP8 training, FP8 KV cache
```

### Strengths

**Overwhelming cost efficiency.** V4-Flash costs $0.14 per million input tokens and $0.28 per million output tokens. This is 80-95% cheaper than frontier commercial models at the output level.

1.  **Strength 1:** Minimized active parameters (13B) -> reduced computation per token + KV compression with MLA -> increased concurrent requests per GPU + linearized attention cost in long contexts with DSA -> 10% KV cache compared to V3.2 at 1M context + FP8 training and inference -> memory bandwidth savings + custom kernels and communication optimization -> increased hardware utilization = **handling several times more traffic with the same GPU** -> still profitable even with lower prices.
2.  **Strength 2:** Long context economics, KV cache is 1/10th compared to V3.2 at 1M tokens. In scenarios with massive system prompts, long conversation histories, and document-based generation, **costs increase moderately even as length grows.**
3.  **Strength 3:** Balance of reasoning and coding. The reasoning capabilities of the R-series are integrated into V4, adjusting depth based on difficulty without needing a separate model. V4-Pro achieved 80.6% on SWE-bench Verified according to vendor announcements, placing it among the top-tier downloadable weights.
4.  **Strength 4:** License and compatibility. Being MIT licensed, its API supports both OpenAI ChatCompletions and Anthropic formats, allowing it to be directly integrated into tools like Claude Code without a proxy.

### Weaknesses

-   **Still a slight gap with the top frontier models:** V4-Pro-Max's 80.6% on SWE-bench Verified is a few points behind top commercial models in the 90% range. For critical tasks, those few points mean a **cheaper model might repeatedly retry, spending time instead of money.** (But it's overwhelmingly cheap, though)
-   **Primary documentation is thin:** Details like compression ratios or indexer internals are sometimes only found in third-party analyses.
-   **Self-hosting Pro requires significant infrastructure:** Even quantized, 1.6T needs to be run on multiple nodes, making it realistically difficult for an individual to host alone. Flash (284B) is a more practical alternative.
-   **Text-only:** If multimodal capabilities are needed, other options should be considered.
-   Serverless hosting providers often quantize active values to FP8 for serving. This can lead to subtle differences between the public weights and the output.

### When to use DeepSeek?

When unit cost is critical for high-volume traffic, or

When routinely dealing with very long contexts (RAG, repo analysis, log analysis), or

When MIT license is required for commercial distribution, fine-tuning, etc., or for text-based reasoning and coding.

However, it is not recommended if image/video understanding is required, or if the last few percentage points of high-level accuracy are more important than cost.
