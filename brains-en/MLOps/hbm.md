# HBM, Roofline Model

> Is it compute-bound or memory-bound?

Let's start with a question.

Modern GPU Tensor cores have advanced to the point where they can perform hundreds of trillions of floating-point operations per second (TFLOPS).

If, when inferring LLMs, introducing the latest GPUs to further increase computation speed sometimes yields negligible performance improvements, what could be the reason?

Why does the system always **wait** for memory to fetch data?

To optimize the performance of AI systems, we need to quantify the physical correlation between the amount of computation processed by software and the amount of memory data transfer.

- **Arithmetic Intensity (AI)**: Arithmetic Intensity is the ratio indicating how many floating-point operations (FLOPs) are performed on 1 byte of data after it is brought from physical memory to the compute core. (FLOPs/Byte)
- **Roofline Model**: The Roofline Model is an evaluation model that visually and mathematically diagnoses how well a currently running workload (algorithm) performs against hardware limits, based on the hardware's maximum computational performance (Compute Roof) and maximum memory bandwidth (Memory Roof), and whether the bottleneck is due to computation or memory.
- **HBM (High Bandwidth Memory)**: To overcome the bandwidth limitations of traditional GDDR memory, HBM is a next-generation memory architecture that vertically stacks DRAM dies and places them on a silicon interposer very close to the GPU die, drastically widening the data bus.

<br>

## Problem Definition

In the evolution of hardware, the rate of advancement in compute units has far outpaced that of memory bandwidth, leading to a critical structural bottleneck.

- **Limitations of GDDR Pins and Traces**: GDDR memory connects to the GPU via a PCB (mainboard). Due to the physical limits of the chip's perimeter, it's very difficult to increase data pins beyond 384 (384-bit). To increase bandwidth, the clock frequency must be raised, which exponentially increases power consumption and heat generation.
- **Memory Wall Phenomenon**: Among deep learning workloads, the inference stage of Transformer models, in particular, has very low arithmetic intensity. This means it repeatedly reads massive weight data from memory, performs a single multiplication, and then discards it. Regardless of how fast the hardware compute units are, the GPU often idles due to a lack of data supply, solidifying a Memory-Bound state as the primary system bottleneck.

### Solution

To overcome this memory bottleneck (the slanted line limit of the Roofline model), the physical chip packaging architecture was completely redesigned.

- **2.5D Packaging and TSV (Through Silicon Via):** Instead of arranging DRAM chips linearly on a PCB, they are stacked vertically. Thousands of vertical connections are made by drilling holes directly into the chips using TSVs (Through Silicon Vias), which are microscopic copper pillars thinner than a human hair.
- **Silicon Interposer:** GPU chips and HBM chips are placed side-by-side (2.5D) on a silicon interposer, which is a micro-fabricated silicon chip, rather than a conventional mainboard. This compresses the physical distance between the GPU and memory to millimeters.
- **Ultra-Wide Bus:** Instead of a 384-bit bus, HBM uses a massive 4096-bit or 5120-bit bus interface. By lowering the clock speed compared to GDDR, it manages power consumption and heat, while increasing the number of data lanes by tens of times, achieving terabyte-per-second memory bandwidth improvements.

<br>

## How it Works

Let's trace how data physically enters the GPU from the perspective of the packaging layers in an HBM structure.

1.  **DRAM Cell Array I/O**: Data read requests are processed in the DRAM cells on the topmost layer of the HBM stack.
2.  **TSV Penetration**: Instead of exiting through the external pins of the PCB, data rapidly descends vertically through TSV (Through Silicon Via) micro-channels drilled directly into the chip, reaching the base logic die at the bottom of the HBM stack.
3.  **Microbump Passage**: On the underside of the base die, there are thousands of microbumps, much denser than typical chip pins. Data passes through these bumps into the silicon interposer.
4.  **Silicon Interposer Routing**: Data travels along thousands of ultra-fine traces etched within the silicon interposer, directly to the GPU die's memory controller PHY, located just a few millimeters away.
5.  **GPU L2 Cache and Crossbar Entry**: Data that has passed through the GPU memory controller immediately settles into the large L2 cache via a large crossbar switch traversing the center of the GPU die, and is then distributed to the SMs.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2Fbel0bR%2FbtsI8knD7TP%2FAAAAAAAAAAAAAAAAAAAAAPYSTskg_LzjUNsuf4e43WTrnBhjvhiVmmEw52K6IRoU%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1780239599%26allow_ip%3D%26allow_referer%3D%26signature%3DwnMXvial9w8%252F6DrDzpyj%252FxO341k%253D)

### profiling

This is a Roofline analysis log using NVIDIA Nsight Compute `ncu` to identify whether a specific kernel is Compute-Bound or Memory-Bound.

```bash
root@ai-node01:~# ncu --metrics sm__throughput.avg.pct_of_peak_sustained_elapsed,dram__throughput.avg.pct_of_peak_sustained_elapsed --section SpeedOfLight_RooflineChart python inference.py

==PROF== Connected to process 15024
==PROF== Profiling "attention_kernel" - 1 of 1

Section: GPU Speed Of Light Roofline Chart
---------------------------------------------------------------------- --------------- ------------------------------
Metric Name                                                                Metric Unit                    Metric Value
---------------------------------------------------------------------- --------------- ------------------------------
sm__throughput.avg.pct_of_peak_sustained_elapsed                             %                              12.45
dram__throughput.avg.pct_of_peak_sustained_elapsed                           %                              94.82
---------------------------------------------------------------------- --------------- ------------------------------

[Warning] This kernel is Memory Bound. 
The memory bandwidth utilization (94.82%) is significantly higher than the compute utilization (12.45%). 
Arithmetic Intensity is 0.82 FLOPs/Byte, which falls under the Memory Roof slanted line.
Consider using operator fusion or shared memory to increase data reuse.
```

-   `dram__throughput 94.82%`: This indicates that approximately 95% of the HBM's physical maximum bandwidth limit (e.g., 1.5TB/s for A100) is being used, and the memory controller is in a starvation state.
-   `sm__throughput 12.45%`: This means that the compute cores (Tensor/CUDA) are operating at only 12.45% of their maximum performance, implying they spend a lot of time idle, waiting for data to arrive.

In conclusion, this kernel is perfectly Memory Bound. In this situation, overclocking the GPU's clock or replacing it with a GPU that has more Tensor cores will not improve performance.

The only solutions are to switch to hardware with wider memory bandwidth, such as HBM3 or HBM3e, or to optimize software to increase Arithmetic Intensity.

Additionally, system engineers or AI engineers can try the following optimization techniques to avoid memory bottlenecks and artificially increase arithmetic intensity, pushing the workload into the compute-bound region on the right side of the roofline.

#### Operator Fusion

If multiple individual operations like `MatMul` -> `Scale` -> `Mask` -> `Softmax` are executed separately, each step incurs HBM read/write I/O, leading to severe Memory-Bound issues. By using FlashAttention or Triton kernels to combine these steps into one large kernel, intermediate results can be kept in the ultra-fast SRAM within the SM without being written to HBM, thus completing the computation. This drastically reduces the denominator in FLOPs/Byte, resolving the bottleneck.

This reduces the 'Byte' part of FLOPs/Byte.

#### Utilizing the Pytorch JIT Compiler (torch.compile)

Using the `torch.compile` mechanism in PyTorch 2.0 and above internally leverages OpenAI Triton as a backend to automatically perform kernel fusion.

```py
import torch

def memory_bound_logic(x, y):
    # In Python, this inefficient code results in 3 HBM R/W operations
    a = torch.sin(x)
    b = torch.cos(y)
    return a + b

# Compilation merges the 3 operations into a single CUDA kernel (Fusion)
# Minimizing I/O to 1 HBM load, internal SRAM computation, then 1 HBM store
optimized_logic = torch.compile(memory_bound_logic)

x = torch.randn(10000, 10000, device="cuda")
y = torch.randn(10000, 10000, device="cuda")
output = optimized_logic(x, y)
```

#### Data Bandwidth Optimization through Quantization

Quantizing weights and activation function data from FP16 (2 Bytes) to INT8 (1 Byte) or FP8 (1 Byte) halves the amount of data that needs to be read from HBM.

This has the same effect as doubling the physical memory bandwidth, shifting the workload's position on the Roofline model into the Compute Bound region.

<br>

## GDDR

GDDR (Graphics Double Data Rate), mentioned above, is the video memory (VRAM) commonly found in graphics cards.

The RAM in your home RTX 3080, 4090, or gaming consoles like PlayStation, is all GDDR.

The "green stick" RAM you typically plug into a motherboard when building a computer is called DDR (DDR4, DDR5). While the names are similar, their design purposes are different.

-   **DDR (RAM for CPU): All-in on Responsiveness**: Optimized for quickly transferring small amounts of data with minimal latency, as the CPU needs to respond rapidly when jumping between code sections.
-   **GDDR (RAM for GPU): All-in on Data Throughput**: The GPU needs to paint millions of pixels on the monitor screen simultaneously, so it's optimized for pouring out massive amounts of data at once. Its response speed is slightly slower than DDR, but the sheer volume of data it carries is overwhelmingly greater.

What's its relationship with HBM? While the latest GDDR models are fast, AI models with billions of parameters have started to overwhelm GDDR. GDDR had a structure where memory chips were laid out flat on the graphics card's PCB, making it physically difficult to increase the Bus beyond 384-bit.

HBM emerged to overcome this planar limitation by stacking memory chips vertically, creating over 4000 lanes, and placing them next to the GPU core.

You can think of GDDR as GPU memory for gaming and general tasks, and HBM as specialized memory for AI computations.
