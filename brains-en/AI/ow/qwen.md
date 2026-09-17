# Qwen

It is a model from Alibaba Cloud. Tongyi Qianwen from China. It has **the broadest lineup among the four models**.

It ranks **first in download count and number of fine-tuned derivative models** in the open-weight ecosystem.

| Date | Model | Features |
|---|---|---|
| 2025.5 | Qwen3 | dense 0.6B~32B + MoE 30B-A3B, 235B-A22B. thinking/non-thinking integration + thought budget. 36T tokens, 119 languages, all Apache 2.0 |
| 2025.9 | Qwen3-Next-80B-A3B | Introduced Gated DeltaNet 3:1 hybrid. Equivalent or better performance at **9.3%** of Qwen3-32B training cost |
| 2026.2.16 | Qwen3.5-397B-A17B | Hybrid across the entire generation. Early-fusion native multimodal, 512 experts, 201 languages, 1M context. 19x decoding throughput compared to Qwen3-Max |
| 2026.2~3 | 3.5 Small Wave | 122B-A10B, 35B-A3B, 27B, 9B, 4B, 2B, 0.8B |
| 2026.4 | Qwen3.6 | 35B-A3B(4/16), 27B dense(4/22) |
| 2026.5~8 | 3.7-Max, 3.8-Max | Transitioned to closed. 3.8 open release is announced |

It has a design philosophy of enabling everyone to use it in all sizes and all languages.

- **Size Spectrum:** It offers models from 0.8B (mobile phone) to 397B (data center) within the same generation. While other models focus on a single flagship, Qwen covers the entire range.
- **Open License:** Most open models are Apache 2.0, which is the most permissive.
- **Multilingual:** Expanded from 119 languages in Qwen3 to 201 languages and dialects in Qwen3.5. It aims to cover regional cultures and nuances.

### Implementation Techniques

**Qwen3 Generation (Traditional Transformer Approach)**
- GQA, SwiGLU, RoPE, RMSNorm + Pre-Norm
- Removed QKV bias from Qwen2 and introduced QK-Norm (for training stability)
- MoEs with 128 experts, 8 of which are activated, no shared experts, and specialization is induced by global batch load balancing loss.
- **thinking / non-thinking integrated framework + thought budget**

**Qwen3.5/3.6 Generation (Hybrid)**
- **Gated DeltaNet3: Gated Attention 1** batch, 75% of layers are linear attention.
- Example: Qwen3.6-35B-A3B is 40 layers = (GDN x 3 + Gated Attention x 1) x 10
- Gated Attention: 16 query heads, 2 KV heads, 256 head dimension, 64 RoPE dimension
- MoE: 8 routing out of 256 + 1 shared
- MTP multi-stage training -> speculative decoding
- Early-fusion multimodal: Instead of attaching a separate VL adapter, it **learns text, image, and video tokens together from the beginning**.
- Native 262K context, extended to approximately 1M with YaRN
- RL at the scale of a million-agent environment

### Strengths

1.  **Strength 1:** Good for local execution. Qwen3.6-27B is a dense model and is approximately **17GB** when 4-bit quantized. It fits on a single RTX 4090 or 3090. However, its performance surpasses the **Qwen3.5 397B flagship** in coding benchmarks (SWE-bench Verified 77.2 vs 76.2, Terminal-Bench 2.0 59.3 vs 52.5).

    This is because, unlike MoE, all parameters in dense27B are active every time. In batch 1 (personal use environment), the advantages of MoE are minimal, and it only adds the burden of loading all layer parameters into memory + **strong-to-weak distillation** from superior models allows smaller models to inherit the capabilities of larger models + hybrid attention enables even small models to handle 256K context = creating its position as the strongest coder on a single GPU.

2.  **Strength 2:** Multilingual, especially in CJK, Korean, Japanese, and Chinese processing, it has long maintained a clear advantage over Western models. The expansion to 201 languages has extended its reach to low-resource languages, which is of significant practical importance to Korean users.

3.  **Strength 3:** Ecosystem depth. A high number of downloads means that when issues arise, someone else has likely encountered them first. GGUF conversions, fine-tuning recipes, Ollama/LM Studio support, and quantization presets are among the first and most frequently released. In practice, this seemingly mundane advantage often outweighs a 2-point benchmark difference.

4.  **Training Efficiency:** Qwen3-Next achieved equivalent or better performance at 9.3% of Qwen3-32B's compute cost. This means $100,000 becomes $9,300, demonstrating the power of the hybrid architecture.

5.  **Strength 5:** Native multimodal. Being early-fusion, it directly understands images and videos without a separate VL model.

### Weaknesses and Caveats

-   **Top-tier models went closed:** Qwen3.7-Max and Qwen3.8-Max are API-only, so don't expect top-tier performance with the perception of them being open. As of August 2026, the currently downloadable general-purpose model is 3.6-27B, although a 3.8 open release has been announced.
-   **Limitations of Hybrid Search:** The 75% proportion of linear attention presents a theoretical weakness in very precise long-range search. While compensated by full attention layers, it is not entirely complete.
-   **Rapid versioning leads to high cognitive load**: With 3, 3.5, 3.6, 3.7, and 3.8 all released within a year and a half, it's difficult to determine which generation to consider standard.
-   Flagship models have higher computational costs per request than pure text models due to multimodal overhead.

### When to Use

O:
When needing to run locally on-premise, it's a top priority, or for multilingual tasks including Korean, or when fine-tuning to create an in-house dedicated model.

When a very small model is needed for edge mobile devices, or when image and video understanding are required together.

And when a permissive Apache 2.0 license is needed.

X: When desiring absolute top performance with open weights for ultra-long context agent workloads (e.g., 1 million tokens), GLM/Kimi are more suitable.
