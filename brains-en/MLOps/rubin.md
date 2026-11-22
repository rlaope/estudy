# Vera Rubin Analysis

This article aims to summarize NVIDIA's next-generation AI data center platform, Vera Rubin.

A single Rubin GPU reads 288GB of HBM4 memory at 22TB per second and performs 50 petaflops per second with 4-bit operations.

The Vera Rubin NVL72 combines 72 of these GPUs into one cabinet. Compared to its predecessor, Blackwell, inference performance is 5 times higher, memory speed is 2.8 times faster, and GPU-to-GPU interconnect speed is 2 times faster.

The goal of these notes is to understand what these numbers mean and why it was designed this way.

## How to Read GPU Specs in Numbers

### FLOPS

GPU computation speed is measured in FLOPS, which stands for how many floating-point operations (addition or multiplication) it can perform per second.

Units increase by a factor of a thousand: 10^12 is Tera (TFLOPS), 10^15 is Peta (PFLOPS), and 10^18 is Exa (EFLOPS). So, 1 EFLOPS is 1,000 PFLOPS. A single Rubin GPU is 50 PFLOPS, and a rack of 72 such GPUs totals 3,600 PFLOPS, or 3.6 EFLOPS.

### Data Speed and Size: Bandwidth and Capacity

No matter how fast the computation, if the data to be processed doesn't arrive on time, the GPU will sit idle.

That's why memory bandwidth, the amount of data that can be transferred per second, is as important as FLOPS.

Think of it as the width of a road. Separately, capacity is the amount of data that can be stored at once, analogous to the size of a warehouse.

There's a common point of confusion in unit notation. Uppercase B stands for Byte, lowercase b for bit, and 1 Byte equals 8 bits. Connections between memory and GPU are usually expressed in TB/s, while network cards are expressed in Tb/s.

For example, 1.6 Tb/s is only 0.2 TB/s.

### How Many Bits Are Used to Store Numbers: Precision

Computers store numbers using a fixed number of bits. More bits mean greater accuracy, but they consume more memory and slow down calculations.

The principle is similar to reducing the resolution of a photo to make the file smaller.

Scientific simulations require precise numbers, such as 64-bit (FP64) or 32-bit (FP32).

On the other hand, AI models don't suffer significantly in results even with less precise calculations. Therefore, AI has moved from 16-bit (FP16, BF16) to 8-bit (FP8), and recently even down to 4-bit (FP4).

With 4 bits, only 16 different numbers can be represented. To compensate for this, NVIDIA created a format called NVFP4. This method divides numbers into small blocks and applies a separate scale factor to each block to improve accuracy. The Rubin's flagship performance figure of 50 PFLOPS is based on this NVFP4 standard.

Also, be aware of values labeled "dense" in spec sheets. Dense refers to performance when all values are actually computed. Its opposite, "sparse," refers to inflated performance when features for skipping matrices with many zeros are enabled. Dense is the more honest number.

### Training and Inference, and the Two Stages of Inference

Training is the process of adjusting model weights by observing data.

Inference is the process of generating answers using a trained model. Most service costs come from inference.

Inference is further divided into two stages:

1.  **Prefill**: This stage involves reading the entire input prompt at once. It multiplies thousands to hundreds of thousands of tokens into a matrix simultaneously, requiring significant computation. Therefore, prefill is bottlenecked by computation speed.
2.  **Decode**: This stage generates the answer one token at a time. Each time a token is generated, the entire model's weights must be re-read from memory. Compared to the amount read, the computation is very small. Therefore, decode is bottlenecked by memory bandwidth.

This distinction is key to understanding Rubin's design, so remember it: Prefill is bottlenecked by computation, and decode by bandwidth.

### KV Cache

The model stores the computed key and value results of already-read tokens and reuses them to generate the next token. This is called the KV cache.

It's like a notepad that prevents the model from re-reading everything from scratch each time. The problem is that as the input gets longer, this notepad also grows tremendously.

In a context of a million tokens, a single KV cache could potentially occupy the entire GPU memory.

### MoE and Active Parameters

Most large models today use an MoE (Mixture of Experts) architecture, where the model contains multiple experts, and only a subset of them is selected for each token.

Thus, even if the total parameters number in the trillions, the active parameters actually used to process a single token amount to only tens of billions.

Conversely, models that use all parameters every time are called dense models. This is different from the "dense" referring to precision.

<br>

## What is Vera Rubin

Vera Rubin is named after Vera Florence Cooper Rubin, an American astronomer who observed the rotation speeds of galaxies and supported the existence of dark matter.

Within the platform, the CPU is named Vera, and the GPU is named Rubin. It succeeds the previous generation's Grace (CPU) and Hopper Blackwell (GPU).

### Timeline So Far

Nvidia first detailed Vera Rubin on its roadmap at GTC in March 2025.

In September 2025, it announced Rubin CPX, a chip dedicated to long context processing, and in December, it signed a technology licensing and talent acquisition deal worth approximately $20 billion with inference chip startup Groq.

At CES in January 2026, Vera Rubin NVL72 was officially announced, and at GTC 2026 in March, it was revealed that all seven chips had entered mass production. At this time, Rubin CPX quietly disappeared from the roadmap, and its place was filled by the Groq 3 LPU, built with Groq technology. In August, racks began shipping to the first customer, Microsoft, and now, in Q4 2026, is the period when shipments are significantly increasing.

One thing to note is that articles published in 2025 introduced Rubin CPX as a core component and referred to the rack name as NVL144 because, at the time, names were based on the number of GPU dies. The current official designation is NVL72, based on the number of GPU packages, and CPX no longer exists.

### Chips, System

The most important perspective when understanding Vera Rubin is that it's not just a single GPU chip, but a data system designed with seven types of chips working together. The seven chips are as follows:

1.  **Rubin GPU:** The chip responsible for computation
2.  **Vera CPU:** The management chip that divides and coordinates tasks
3.  **Groq 3 LPU:** A decode-only chip that rapidly generates responses
4.  **NVLink 6 Switch:** An interconnect that links GPUs within a rack
5.  **ConnectX-9 SuperNIC:** An auxiliary chip that handles network storage security processing
6.  **Spectrum-6:** An Ethernet switch that connects the entire data center

<br>

## A Look Inside the Rubin GPU

### Physical Structure of the Chip

Chips are made by etching circuits onto a silicon wafer. A single piece cut from the wafer is called a **die**, and this is what we commonly refer to as a chip.

Lithography equipment used to etch circuits has a limit to the size it can imprint at once, and this is called the **reticle limit**. Chips larger than this cannot be manufactured.

Therefore, recent high-performance chips connect multiple dies together. The Rubin GPU packages two compute dies, each close to the reticle limit in size, and an I/O die responsible for external connections, into a single package.

The technology that connects dies and memory very closely with a large number of interconnections is TSMC's CoWoS-L packaging. Although there are two dies, the system recognizes them as a single GPU.

Manufacturing is done using TSMC's 3nm process, and it contains 336 billion transistors. This is 1.6 times more than Blackwell's 208 billion.

### Compute Units

The inside of a GPU is divided into workgroup units called **SMs (Streaming Multiprocessors)**. The Rubin has 224 SMs.

Each SM contains Tensor Cores specialized in matrix multiplication, and the total number of Tensor Cores is 896. Since most AI operations involve matrix multiplication, the AI performance of a GPU is virtually determined by its Tensor Cores.

The **Transformer Engine** is attached here. It's a combination of hardware and software that automatically selects whether to compute with FP8 or FP4 for each layer. The Rubin is equipped with the 3rd generation Transformer Engine and a new feature called 'adaptive compression' has been added. Additionally, the **TMA (Tensor Memory Accelerator)**, a dedicated unit for transferring data from memory to Tensor Cores, has also been improved.

An interesting point is that the number of SMs and Tensor Cores only increased by 17% compared to Blackwell (192, 768). Despite this, 4-bit inference performance increased by 5 times, meaning that most of the performance improvement came not from the number of cores, but from low-precision computation paths and memory connection speeds.

### Official Specifications

The official figures for a single GPU, a Superchip combining two GPUs and one CPU, and a rack of 72 units are as follows. You only need to know the rule that a Superchip is 2 times the GPU value, and a rack is 72 times.

| Item | 1 GPU | Superchip | NVL72 Rack |
| --- | --- | --- | --- |
| NVFP4 Inference | 50 PF | 100 PF | 3,600 PF |
| NVFP4 Training (dense) | 35 PF | 70 PF | 2,520 PF |
| FP8 Training (dense) | 17.5 PF | 35 PF | 1,260 PF |
| BF16 (dense) | 4 PF | 8 PF | 288 PF |
| HBM4 Capacity | 288 GB | 576 GB | 20.7 TB |
| HBM4 Bandwidth | 22 TB/s | 44 TB/s | 1,580 TB/s |
| NVLink Bandwidth | 3.6 TB/s | 7.2 TB/s | 260 TB/s |

The reason for the difference between 50 PF for inference and 35 PF for training is that the inference figure includes the adaptive compression effect of the Transformer Engine. The 35 PF for training is the value for uncompressed dense operations.

FP64 for scientific computing is 33 TFLOPS per GPU, which is very small compared to AI performance. However, using an emulation technique that mimics FP64 calculations with Tensor Cores can achieve TFLOPS.

<br>

## Memory Hierarchy

### Four Types of Memory

The Vera Rubin system has four layers of memory with different characteristics. The faster it is, the more expensive and smaller it is; the slower it is, the cheaper and larger it is. It's easy to understand if you compare it to a desk, bookshelf, warehouse, or external logistics center.

1.  **SRAM (Desk):** This is ultra-high-speed memory embedded within the chip. The Groq 3 LPU has 512MB of SRAM per chip, with speeds reaching 150TB per second, but its capacity is very small.
2.  **HBM4 (Bookshelf):** This is high-bandwidth memory where a stack of multiple memory chips is attached right next to the GPU. The Rubin GPU has 8 HBM4 stacks, totaling 288GB, and provides 22TB/s of bandwidth. Its predecessor, Blackwell, used the previous generation HBM3e and had about 8TB/s, making this 2.8 times faster.
3.  **LPDDR5X (Warehouse):** This is low-power memory also used in smartphones. It's slower than HBM but cheaper and can be installed in larger capacities. Each Vera CPU can have up to 1.5TB, and an entire rack can have 54TB. It's installed as replaceable modules, and this standard is called SOCAMM.
4.  **BlueField-4 STX Storage Rack (Logistics Center):** This is an external storage device outside the rack that stores KV caches not currently in use and loads them back when needed.

#### Why Hierarchy is Necessary

In AI services that handle long contexts, the KV cache cannot all fit into HBM.

Therefore, currently used KVs are temporarily stored in HBM, while KVs for inactive users are offloaded to CPU memory or STX and reloaded when needed.

The core technology for long-context serving ultimately lies in how well KV caches are moved between these layers.

<br>

## Connectivity: A Rack as a Single GPU

### Scale-Up and Scale-Out

There are two ways to increase the number of GPUs. Tightly coupling GPUs within a single unit is called **scale-up**, and connecting multiple units via a network is called **scale-out**.

Scale-up connections are very fast, while scale-out connections are relatively slow.

### NVLink 6

NVLink is a dedicated high-speed pathway that directly connects NVIDIA GPUs. Rubin uses 6th-generation NVLink, providing 3.6 TB/s per GPU, which is double that of Blackwell. Inside the rack, there are 9 trays with NVLink switches, allowing all 72 GPUs to communicate with each other at the same speed. The total connection bandwidth for the entire rack is 260 TB/s.

NVLink switches also have a feature called 'in-network computing'. During training, an all-reduce communication, where multiple GPUs collect their computed values, combine them, and then redistribute them, occurs repeatedly. This aggregation is handled within the switch rather than by the GPUs, reducing the burden on the GPUs.

Thanks to these fast connections, even if MoE model experts are distributed across 72 GPUs, the time it takes for tokens to travel to and from experts is short. NVIDIA claims that only one-fourth the number of GPUs compared to Blackwell is needed to train MoE models.

### From Chip to Data Center

The Rubin system is built in the following stages:

```
Rubin GPU 1개 (50 PF)
   × 2 + Vera CPU 1개  →  슈퍼칩 (100 PF)
   × 2                 →  컴퓨트 트레이 (GPU 4개, 200 PF)
   × 18 + 스위치 트레이 9개  →  NVL72 랙 (GPU 72개, 3.6 EF)
   × 16 + 지원용 랙 24개    →  POD (40랙, GPU 1,152개, 약 60 EF)
```

A **tray** is a board that slides into a rack like a drawer, and a rack is a refrigerator-sized cabinet that holds trays vertically.

Since a single rack consumes hundreds of kW of power, air cooling is insufficient, so 100% liquid cooling is used. A POD is a cluster unit that bundles dozens of racks, comprising 16 NVL72 racks along with LPX racks for decoding, CPU racks, storage racks, and network racks.

### Inter-Rack Network

**ConnectX-9 SuperNICs** handle communication between racks. Each GPU has a speed of 1.6 Tb/s, or 0.2 TB/s, which is about 1/18th of NVLink.

These network cards support **RDMA** technology, which allows direct reading and writing to another server's memory without going through the CPU, and connect to Ethernet Spectrum-X or InfiniBand (Quantum-X800) for high-performance computing networks.

The Spectrum-6 switch, which connects the entire data center, uses NVIDIA's first Co-Packaged Optics (CPO) approach, integrating optical communication components directly next to the switch chip. This consumes less power and is less prone to failure than separate pluggable optical transceivers.

The **BlueField-4 DPU** is an auxiliary chip that packages a 64-core Grace CPU and ConnectX-9 together. It handles miscellaneous tasks like networking, storage, and encryption, allowing the CPU and GPU to focus on their primary workloads.

The speed difference between NVLink and the network dictates parallelization strategies. **Tensor parallelism**, where multiple GPUs split and compute a matrix of a single layer, or **expert parallelism**, where experts are distributed, require very frequent communication and must be performed within a rack. In contrast, data parallelism, which involves having multiple copies of the same model and distributing data for training, has less communication and can be performed across racks.

<br>

## Vera CPU: The Role of the Manager

Vera is NVIDIA's first data center CPU designed from the ground up.

It is compatible with Arm (Armv9.2), a low-power design specification originating from smartphone chips, and is equipped with 88 NVIDIA-designed Olympus cores. Its predecessor, Grace, had 72 Arm-designed Neoverse V2 cores.

It runs on 176 threads using Spatial Multithreading, NVIDIA's variation of multithreading technology where a single core processes two workflows concurrently.

It reads up to 1.5TB of LPDDR5X memory at 1.2TB/s and connects to GPUs via an inter-chip link called **NVLink-C2C**. This connection is 1.8TB/s, double that of its predecessor, allowing GPUs to quickly borrow CPU memory.

Vera's role is more about **coordination** than computation: it determines which GPU receives which request (orchestration), manages where KV caches move, and actually performs tasks when AI agents call tools like code execution or search. For reinforcement learning, which learns through trial and error, a large number of CPUs are needed to run execution environments, so there's a dedicated rack containing only 256 Vera CPUs.

<br>

# Split Inference

Prefill is compute-bound, while decode is bandwidth-bound. If two tasks with opposite characteristics are processed on the same GPU, resources will be idle on one side. This led to the emergence of split inference, where these two stages are handled by different hardware.

### Rubin CPX (Withdrawn, First Attempt)

NVIDIA planned to offload **prefill** in 2025.

Rubin CPX was a prefill-specific chip equipped with 128GB of GDDR7, memory typically used for gaming graphics cards, instead of expensive HBM, and capable of 30 PF with 4-bit operations.

The idea was to reduce costs by using cheaper memory since prefill is less bandwidth-critical. However, this plan was removed from the GTC roadmap in March 2026.

There were three main reasons for this: architectural limitations, memory bottlenecks, and changes in inference trends.

1.  **KV Cache Transfer Bottleneck:** If prefill and decode are physically separated onto different chips, the vast KV cache computed by the prefill chip must be transferred in real-time to the decode chip (HBM-equipped Rubin). As context length increases, KV cache size grows exponentially. Even with NVLink or high-speed interconnects, the latency and power consumption incurred by inter-chip data movement would erode the chip cost savings, making the overhead greater than the benefit.
2.  **Early Arrival of GDDR7 Bandwidth and Capacity Limits**: While the idea originated from the premise that prefill is compute-intensive and decode is memory-intensive, with the advent of modern large models featuring MoE architectures, multimodal capabilities, and ultra-long contexts, severe memory bandwidth and capacity bottlenecks emerged even in the prefill stage with 128GB. GDDR7 has lower power efficiency and bandwidth density compared to HBM, making it less attractive in terms of overall system heat generation and TCO (Total Cost of Ownership) when scaled up to data center racks.
3.  **Inference Paradigm Shift (Test Time, Compute, and Caching)**: As reasoning models, which involve a decode-centric thought process of generating tens of thousands of tokens after a single prefill, became standard, it became difficult to maintain high utilization of prefill-specific chips. Furthermore, with the widespread adoption of prompt caching, the proportion of repeated prefill computations being read directly from memory instead of re-calculated increased dramatically, eliminating the need to produce separate prefill-specific ASIC derivative chips.

Focusing on silicon packaging and supply chain efficiency for HBM4 was also a reason.

From NVIDIA's perspective, it was deemed far more profitable and scalable to unify around the next-generation HBM4-based standard Rubin and Rubin Ultra architectures and flexibly schedule prefill and decode with software, rather than fragmenting TSMC's CoWoS packaging capacity and design resources to create niche GDDR7-based derivative chips.

> TSMC CoWoS (Chip on Wafer on Substrate): This is TSMC's advanced packaging technology that vertically and horizontally integrates compute AI chips and high-bandwidth memory (HBM) at ultra-high density on a single substrate (interposer). It is an essential process for manufacturing high-performance AI accelerators like NVIDIA H100, B200, and Rubin because it drastically reduces the distance between chips and memory, thereby increasing data transfer speeds and reducing power consumption.

### Current Approach: Groq 3 LPX

In 2026, NVIDIA reversed course and **offloaded decode**. The Groq 3 LPU is a decode-specific chip built with Groq's LPU (Language Processing Unit) technology. It generates tokens very quickly because it loads the model into on-chip SRAM and reads it at 150TB/s. A rack containing 256 of these chips is called Groq 3 LPX.

Consequently, a single request is processed in the following sequence.

```
User Request
  → Received and assigned by Vera CPU
  → Rubin GPU performs prefill (reads input, generates KV cache)
  → KV cache transferred to Groq 3 LPX (idle KVs stored in STX)
  → Groq 3 LPX performs decode (generates response)
  → If a tool call occurs, Vera executes it and re-inserts the result
```

This entire flow is managed by NVIDIA's inference serving software, **Dynamo**. NVIDIA claims that using NVL72 and LPX together can achieve up to 35 times the throughput per megawatt compared to Blackwell.

<br>

## Comparing with Blackwell

We can see what has changed and by how much compared to the previous generation.

| Item | Blackwell | Rubin | Multiplier |
| --- | --- | --- | --- |
| NVFP4 Inference | — | 50 PF | 5x |
| NVFP4 Training | — | 35 PF | 3.5x |
| Memory Bandwidth | Approx. 8 TB/s | 22 TB/s | 2.8x |
| NVLink (per GPU) | 1.8 TB/s | 3.6 TB/s | 2x |
| Transistors | 208 billion | 336 billion | 1.6x |
| Number of SMs | 192 | 224 | 1.17x |
 

While the chip size itself only increased by about 1.2 to 1.6 times, AI performance jumped by 3.5 to 4 times.

This is because low-precision computation, memory, and connectivity were improved together.

<br>

## Cultivating a Sense of Numbers

Now, let's perform some calculations based on the concepts we've learned. If you can do just three types of calculations, you'll be able to roughly estimate how much a GPU's specifications can be utilized for various tasks.

### Ridge Point: The Boundary Between Computation and Bandwidth

If you divide the GPU's computation speed by its memory speed, you get the number of computations the GPU must perform for every byte of data read to keep it busy.

This value is called the **Ridge Point**, and the method of comparing these two bottlenecks is called **Roofline Analysis**.

> 릿지 포인트 = 최대 FLOPS / 메모리 대역폭 = 35 x 10^15 / 22 x 10^12 ≈ 1,590 FLOP/byte

Decoding performs only a few computations when reading 1 byte of weights. Since this is far below 1590, decoding is almost always bandwidth-bound. To increase the batch size for processing multiple user requests, where multiple users share weights read once, this boundary is gradually approached.

### Decode Speed Upper Limit

Since active weights must be read once per decode step, there is an upper limit to the number of steps that can be performed per second.

> 초당 스텝수 <= 메모리 대역폭 / (활성 파라미터 수 x 파라미터당 바이트)

If a 70 billion (70B) parameter dense model is stored in NVFP4, including scaling information, it's approximately 0.5625 bytes per parameter, so the weights are about 39.4 GB. On Rubin (22 TB/s), this allows for 559 steps per second, while on Blackwell (8 TB/s), it's about 203 steps per second. The 2.8x increase in memory bandwidth directly translates to an equivalent increase in decode speed.

### KV Cache Size

> 토큰당 kv 크기 = 2 x 레이어수 x kv헤드수 x 헤드차원 x 바이트

Assuming 80 layers, a KV head dimension of 128, and storage in 8-bit (1 byte), one token would be 2 x 80 x 8 x 128 x 1 = 163,840, or approximately 160 KB.

For a 1 million token context, the KV cache alone would be about 164 GB. If we subtract the 39 GB for the 70B model weights from Rubin's 288 GB, 249 GB remains. This means only about 1.5 users with 1 million tokens can be accommodated. This numerically confirms why 4 memory tiers and 7 STX storages are needed.

<br>

## Virtual Use Cases

Let's build a hypothetical company. The input values are official specifications, and the calculations are rough estimates to get a sense of where bottlenecks might occur. Communication costs, attention calculation costs, etc., have been simplified.

### Example A. CodeBridge - A Coding Agent that Reads Long Contexts

CodeBridge provides a coding agent service using an MoE model with a total of 1 trillion parameters and 32 billion active parameters. When a user connects a repository, the agent code reads 500,000 tokens and suggests modifications.

First, let's consider the weights. Stored in NVFP4, they take up about 562.5GB, which is only 2.7% of the 0.7TB HBM2 in a single NVL72 rack. The model itself fits comfortably within the rack.

Next is prefill. The calculation is approximately 2 x active parameters x number of tokens, so 2 x 32 billion x 500,000 = 3.2 x 10^16 FLOPs. Even if a single Rubin operates at half its maximum performance, it would finish in 1.3 seconds. (In reality, attention calculations for long contexts add significant overhead.)

The real problem is the KV cache. If 160KB is used per token, approximately 82GB is needed per session, and a single rack can only accommodate about 245 concurrent sessions.

Therefore, CodeBridge is designed as follows: RubinGPU handles prefill, and Groq 3 LPX handles decoding. While the user is thinking, the KV cache is offloaded to BlueField-4 STX and then reloaded. Tool calls like test execution or linting are performed by VeraCPU.

<br>

### Example B. Pretraining a 200 Billion Parameter Model

We plan to train a 200 billion parameter dense model with 20 trillion tokens. The computational cost required for training is estimated using the empirical formula: 6 x number of parameters x number of tokens.

> 6 x (2 x 10^11) x (2 x 10^13) = 2.4 x 10^25 FLOP

In large-scale training, the theoretical performance cannot be fully utilized; typically, only about 30-50% is achieved. This ratio is called MFU utilization, and we assume 40% here.

If trained with FP8, a single NVL72 rack (1,260 PF) would take approximately 551 days. Using 16 racks (1,152 GPUs, POD scale) reduces this to about 34 days. If trained with NVFP4, performance doubles, resulting in approximately 276 days and 17 days, respectively.

Training is purely a computational problem, so it's clear that FLOPS and the number of racks directly reduce the training time. However, 4-bit training can only be used if there's a proven method for stable model convergence. Additionally, since inter-rack connections are 1/18th of NVLink, placing tensor parallelism within a rack and data parallelism between racks is the basic setup.

<br>

### Example C. In-house Chatbot with a Single GPU

We run an in-house document search chatbot by loading a 70B model in NVFP4 onto a single Rubin GPU.

The context per user is about 4,000 tokens, and the KV cache is approximately 0.67GB per user.

First, looking at capacity, if we subtract the 39.4GB for weights and divide the remaining 249GB by 0.67GB, we can accommodate approximately 370 users concurrently.

Next is speed. If 370 users connect simultaneously, approximately 288GB (weights + KV cache) must be read at each step. Reading at 22TB/s takes about 13 milliseconds, resulting in approximately 76 tokens per second per user, or about 28,000 tokens per second overall.

An interesting fact emerges here: if the target is 50 tokens per second per user, the bandwidth could handle approximately 600 users. However, in reality, the 288GB capacity runs out at 370 users. Therefore, in actual service, techniques like reducing the KV cache to 8-bit or 4-bit, or offloading to CPU memory, are as crucial as GPU performance.

<br>

## Next-Generation Rubin Ultra

According to NVIDIA's roadmap, the next-generation Rubin Ultra aims for approximately 100 PF of 4-bit performance per GPU and 1TB of next-generation HBM4E memory.

Kyber, the rack designed to house it, features a new vertical tray insertion design, with each rack consuming approximately 600 kW of power. At the OCP event in October 2026, the Vera Rubin Ultra NVL576 was introduced, which uses optical interconnects to link multiple racks, treating 576 GPUs as a single unit.

However, in July 2026, reports emerged that Kyberfordl was delayed by over a year to 2028, and there are rumors that the number of dies has been reduced. NVIDIA only responded that the roadmap remains unchanged. When studying, consider Vera Rubin NVL72 as confirmed for now, and Rubin Ultra as a plan that could change.
