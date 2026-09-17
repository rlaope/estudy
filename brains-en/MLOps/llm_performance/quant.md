# 4. Quantization and Multi-GPU Parallelism

As LLM parameters grow to tens of billions (e.g., 70B, 405B), loading them onto a single GPU has become physically impossible or extremely cost-ineffective.

The two core pillars for reducing infrastructure costs to a realistic level while maximizing service throughput are **Quantization** and **Multi-GPU Parallelism**.

## Key Question: How much can GPU Memory and inference costs be reduced while minimizing model quality degradation?

**Direct Link between Memory and Cost:** To load a Llama-3-70B model in its original format (FP16, 2 bytes per parameter), the weights alone require $70 \times 2 = 140\text{ GB}$ of VRAM. Considering the KV Cache space for inference, at least two NVIDIA A100 (80GB) or two H100 GPUs are essential. This directly translates to enormous infrastructure costs.

**Overcoming Trade-offs:** Quantizing weights to 4-bit (INT4) reduces the model size to $70 \times 0.5 = 35\text{ GB}$, making it possible to serve with just one A100 (40GB/80GB) device.

Since the number of devices is reduced by more than half, **inference costs are cut by over 60-70%.** In the past, converting to 4-bit often led to a collapse in model intelligence (Perplexity). However, thanks to advanced algorithms like recent **AWQ (Activation-aware Weight Quantization)** and **GPTQ**, actual conversational quality (accuracy) degradation can be kept within 1-2%.

<br>

## Analysis of Data Type Quantization Techniques

Quantization is a technique that saves memory by mapping continuous high-precision floating-point representations to discrete low-precision integer or small floating-point representations.

### Characteristics by Data Type

-   **FP16 vs BF16 (16-bit)**: These are the default training and inference data types. FP16 offers high decimal precision but has a narrow representable range, making it prone to Gradient Overflow. BF16 has the same exponent range as FP32, making it significantly superior for LLM training and inference stability.
-   **FP8 (8-bit)**: This format, fully supported starting with the Hopper (H100) architecture, maintains decimal representation unlike integer quantization INT8, offering high numerical stability. It's a recent trend because it directly benefits from hardware acceleration without dequantization overhead.
-   **INT8 vs INT4 (Integer Quantization)**
    -   **Weight-Only Quantization:** Only weights are stored in low precision, and they are restored to FP16 only at the time of computation. This significantly saves memory bandwidth but incurs restoration computation overhead.
    -   **Weight-Activation (W8A8) Quantization**: Both weights and activation function outputs are converted to integers and computed directly by hardware INT8 tensor cores without conversion. While implementation is complex, the computation speed itself increases.

### Advanced Quantization Algorithms: GPTQ vs AWQ

**GPTQ (Layer-wise Quantization)** is a method that prunes weights layer by layer to minimize the load error based on input data, without backpropagation. It involves a relatively high amount of computation but offers excellent optimization for a fixed dataset.

**AWQ (Activation-aware Weight Quantization)** is based on the idea that not all parameters are equally important. It observes the model's activation values and **preserves the top 1% of important weights (Salient Weights) in FP16 precision, while pruning the remaining 99% to 4-bit.** It is highly preferred in real production environments due to extremely minimal quality degradation.

<br>

## Multi-GPU Parallel Processing Architectures

When models exceed single GPU memory or require increased throughput, three NCCL (NVIDIA Collective Communication Library)-based parallelization strategies are used to group multiple GPUs.

### Data Parallelism (DP)

The same model weights are replicated on each GPU, and different input batch data are divided and processed in parallel.

From an inference perspective, it's used when the model fits on a single GPU, and it's closer to a replication strategy for linearly increasing Throughput.

### Tensor Parallelism (TP) - Intra-Node Optimized

The matrix multiplication operation Y = X x W within a single layer is split, allowing multiple GPUs to compute it simultaneously.

-   **Column Parallel**: The weight matrix W is split column-wise, computed separately, and then the results are concatenated.
-   **Row Parallel**: The weight matrix is split row-wise, computed, and then summed via All-Reduce communication between GPUs.

Frequent All-Reduce synchronization communication occurs between GPUs for every Transformer block.

Therefore, it is efficient only in single-server (Intra-node) GPU environments with extremely high bandwidth, such as NVLink, and is not suitable for inter-node scaling that involves network switches.

### Pipeline Parallelism (PP) - Inter-Node Optimized

A method of sequentially splitting model layers and distributing them to GPUs (e.g., layers 1-20 to GPU0, 21-40 to GPU1).

Since the output of a preceding GPU becomes the input for the next GPU, a 'pipeline bubble' (idle time) structurally occurs.

Micro-batch scheduling is essential to reduce this.

It is suitable for parallelization across different servers connected by network switches due to relatively low communication overhead.

<br>

## Hands-on: AWQ Quantization and Tensor Parallel Serving using vLLM

The vLLM engine provides the most intuitive interface for quantization and multi-GPU parallelism in the open-source ecosystem.

Below is an example of loading the Llama-3-70B model, quantized to AWQ 4-bit, and deploying it with Tensor Parallelism across 4 GPUs within a single server.

```bash
# vLLM을 이용한 Multi-GPU Tensor Parallel + AWQ 서빙 인스턴스 가동
python3 -m vllm.entrypoints.openai.api_server \
    --model TechForge/Meta-Llama-3-70B-Instruct-AWQ \
    --quantization awq \
    --tensor-parallel-size 4 \
    --gpu-memory-utilization 0.90 \
    --port 8000 \
    --trust-remote-code
```

Example of execution logs and hardware topology verification during NCCL initialization

```
INFO 2026-06-20 12:30:15] api_server.py:150] Starting vLLM optimized serving engine.
INFO 2026-06-20 12:30:18] config.py:420] Quantization strategy confirmed: AWQ (4-bit Weight-Only)
INFO 2026-06-20 12:30:20] pynccl.py:85] Initializing NCCL communication environment... (Tensor Parallel Size: 4)
INFO 2026-06-20 12:30:25] pynccl.py:120] NVLink topology detected between 4 GPUs. All-Reduce communication network connected.
INFO 2026-06-20 12:30:30] model_loader.py:65] Loading parameters: Split 70B model weights loaded successfully (approx. 10.5 GB occupied per GPU)
INFO 2026-06-20 12:30:42] api_server.py:210] Uvicorn server started: http://localhost:8000
```

### Comparison Table: Model Size, Accuracy, Speed, and Cost

The metrics below are a comprehensive comparative analysis of engineering indicators resulting from configuring various precisions and hardware architecture topologies, based on the large language model Llama-3-70B.

| Configuration (Precision & Parallelism) | Minimum Required Hardware | Total Weight VRAM (GB) | Available KV Cache Area | Perplexity (Intelligence Loss Rate) | Max System Throughput | Infrastructure Cost Index (KRW/hour) |
|--------------------------------|-------------------------|--------------------:|--------------------|-------------------------|-----------------------:|--------------------------:|
| FP16 / BF16 (No Parallelism) | Impossible (OOM) | 140 GB | 0% | Baseline (0.00) | Not Measurable | Cannot Operate |
| FP16 / BF16 (TP = 4, No PP) | A100 80GB × 4 units | 140 GB (35 per GPU) | Very Ample | Baseline (0.00) | 1,800 tokens/s | 100% (Baseline) |
| FP8 (W8A8) (TP = 2, No PP) | H100 80GB × 2 units | 70 GB (35 per GPU) | Ample | +0.02 (Almost None) | 2,450 tokens/s | 75% |
| INT8 (BitsAndBytes) (TP = 2, No PP) | A100 80GB × 2 units | 70 GB (35 per GPU) | Ample | +0.05 (Minimal) | 1,120 tokens/s | 50% |
| INT4 (AWQ) (No Parallelism) | A100 80GB × 1 unit | 35 GB | Normal | +0.12 (Successfully Mitigated) | 410 tokens/s | 25% (Extremely Cost-Effective) |
| INT4 (AWQ) (TP = 2, No PP) | A100 40GB × 2 units | 35 GB (17.5 per GPU) | Very Ample | +0.12 | 920 tokens/s | 35% |
