# CUDA, Tensor Core

Let's start with a question: CPUs focus on minimizing single-thread latency through branch prediction and large L1/L2 caches.
GPUs, on the other hand, are throughput-optimized machines that hide memory latency by switching between thousands of thread contexts. So, what structural differences exist at the physical transistor level between CUDA Cores, which handle traditional scalar operations within a GPU core, and Tensor Cores, which are central to AI computations?
To understand GPU architecture, one must first recognize the differences in design philosophy aimed at overcoming the limitations of the Von Neumann architecture.

- **CUDA Core (Streaming Processor)**: Essentially, it's a combination of a simple Arithmetic Logic Unit (ALU) and a Floating Point Unit (FPU) that performs one scalar operation (e.g., Fused Multiply-Add, FMA) per clock cycle. While it operates similarly to CPU's SIMD (AVX) instructions, GPUs adopt a SIMT (Single Instruction, Multiple Threads) architecture, breaking these down into hardware thread units and executing thousands of them concurrently.
- **Tensor Core (Matrix Processing Unit)**: This is a special-purpose hardware accelerator (systolic array structure) designed to perform matrix multiplication and accumulation (Matrix Multiply-Accumulate, MMA) at a single hardware instruction level. It transforms the repetition of one-dimensional scalar operations into two-dimensional spatial operations, processing tens to hundreds of floating-point operations per clock cycle at once.

**Let's define physical/structural bottlenecks**

The physical bottlenecks that arise when performing massive neural network matrix multiplications using only traditional CUDA Cores are not simply due to a lack of ALUs, but rather to **instruction processing overhead** and **register bandwidth exhaustion**.

- **Instruction Fetch/Decode Power Waste (Instruction Overhead)**: To process a 4x4 matrix multiplication with CUDA Cores (scalar), it requires 64 multiplications and 48 additions, meaning at least over 100 FMA instructions. CPUs and GPUs consume enormous energy during the process of fetching and decoding instructions from the L1 I-Cache, leading to an overhead bottleneck where the power used to interpret instructions is far greater than the power used for actual mathematical operations.
-  **Register File Bandwidth Bottleneck**: CUDA Cores read operands from registers and write results back to registers for every operation. Executing a 4x4 matrix operation by breaking it down into scalar operations results in hundreds of register read/write (RW) operations. While register files, composed of SRAM, are very fast, their large area and high power consumption cause data bus bottlenecks.

<br>

## Solution

To overcome the bottlenecks defined above, NVIDIA abandoned scalar-centric CUDA operations and introduced Tensor Cores (utilizing a Systolic Array structure) that leverage hardware spatial arrays.

- **Introduction of Mixed Precision and HMMA Instructions**: Tensor Cores use a single hardware instruction called `HMMA`, which stands for Half-Precision Matrix Multiply Accumulate. It performs the operation $D = A \times B + C$ in just one clock cycle (or a fraction of a pipeline clock). Since only one instruction needs to be fetched and decoded, front-end overhead is dramatically reduced.
- **Minimizing Register Access through Data Reuse**: Inside a Tensor Core, MAC (Multiply-Accumulate) units are physically wired in a grid-like fashion. Partial sums calculated by one MAC do not return to the register file but immediately flow as input to the adjacent next MAC. This is called Systolic Flow, and through it, the number of register RW operations is reduced to about 1/10, maximizing the chip's performance per watt (Perf/W).

### How it Works

When a General Matrix Multiply (GEMM) workload is executed, we trace the detailed path data takes from physical memory to reach the Tensor Cores.

1. **Movement from HBM to L2 Cache:** When the GPU's thousands of cores (SMs, Streaming Multiprocessors) are assigned tasks, weight and input matrix (activation) data stored in HBM (High Bandwidth Memory) are loaded into the large, shared L2 cache located at the center of the GPU die via an ultra-wide bus (e.g., 5,120-bit memory bus).
2. **Loading from L2 Cache to L1/Shared Memory:** The Warp Scheduler within each SM (Streaming Multiprocessor) issues memory load commands. Data in tile units, which are finely divided matrix segments, physically moves from L2 to the L1 cache/Shared Memory within the SM. Unlike a CPU's L1 data cache, this shared memory is a high-speed SRAM space where programmers can manually control data placement.
3. **ldmatrix Transfer from Shared Memory to Registers:** Just before the Tensor Core begins computation, an `ldmatrix` instruction is executed (Load Matrix). Unlike a typical scalar load, this instruction collectively inserts 2D data blocks from shared memory into the register files within a warp (a group of 32 threads).
4. **Data Injection from Registers to Tensor Cores:** When the `HMMA` instruction is decoded, the FP16 or BF16 format A and B operand matrix data stored in the register files are routed via a crossbar switch to the input stage of the Tensor Core's systolic array.
5. **Spatial Computation within Tensor Cores (Systolic Flow)**: With each clock cycle, data flows like a waterfall through the MAC units inside the array. A matrix data flows horizontally, and B matrix data flows vertically, multiplying at their intersection with FP16 precision. The product is then passed to the adder below and accumulated with FP32 precision. During computation, the register file is not accessed.
6. **Result Write-back:** Finally, the computed 4x4 or 8x4 FP32 partial sum matrix D is written back to the warp register file at once. Subsequently, this result is stored back into HBM via shared memory.

### Quantitative Performance Metrics and Mathematical Modeling

Let's quantify the difference in hardware computational capabilities between CUDA Cores and Tensor Cores by mathematically modeling them (based on NVIDIA Ampere A100).

- **CUDA Core FP32 FLOPS Calculation**: $\text{FLOPS}_{\text{CUDA}} = (\text{Number of SMs}) \times (\text{CUDA Cores per SM}) \times (\text{Clock Speed}) \times 2 \text{ (FMA Operations)}$
For A100: $108 \text{ SMs} \times 64 \text{ Cores/SM} \times 1.41 \text{ GHz} \times 2 \approx 19.5 \text{ TFLOPS}$
- **Tensor Core FP16/FP32 FLOPS Calculation**: A Tensor Core performs $4 \times 4 \times 4 = 64$ FMA operations (128 operations) per clock cycle. The A100's 3rd generation Tensor Cores have 4 per SM, and their pipeline structure is expanded, resulting in higher throughput per clock.
$\text{FLOPS}_{\text{Tensor}} = (\text{Number of SMs}) \times (\text{Tensor Cores per SM}) \times (\text{Clock Speed}) \times (\text{Operations per Clock})$
For A100: $108 \text{ SMs} \times 4 \text{ Cores/SM} \times 1.41 \text{ GHz} \times 512 \approx 312 \text{ TFLOPS}$

In conclusion, the moment Tensor Cores are activated, matrix operations dramatically increase by approximately 16 times (19 vs 312) due to structural innovations that eliminate instruction overhead and register access, even without increasing transistor clock speeds.

<br>

## Layer Profiling Setup

Let's explore how this massive hardware is controlled and its state observed at the software (PyTorch, CUDA) layer.

There is a way to **force-enable Tensor Cores**.

In the past, FP16 casting was done manually, but now PyTorch uses AMP (Automatic Mixed Precision) to guide the hardware data path to Tensor Cores.

```py
import torch
# 1. Ampere 이상 아키텍처에서 FP32 입력을 TF32(Tensor Float 32) 형식으로 
# Tensor 코어에 태우도록 허용 (성능 3~4배 증가, 정밀도 손실 미미)
torch.backends.cuda.matmul.allow_tf32 = True 

# 2. BFloat16을 사용하여 Tensor 코어 데이터 패스 명시적 활성화
with torch.autocast(device_type="cuda", dtype=torch.bfloat16):
    output = model(input_tensor) # 내부 GEMM 연산이 HMMA 명령어로 컴파일됨
```

**Hardware Pipeline Profiling with Nsight Compute**

To check the actual utilization of CUDA Cores and Tensor Cores, use the NVIDIA Nsight Compute `ncu` command to extract GPU pipeline metrics.

- `sm__inst_executed_pipe_fma.avg.pct_of_peak`: Utilization rate of traditional CUDA Cores (FMA pipeline). If this value is high and the value below is low, it indicates that the AI model is performing scalar operations inefficiently.
- `sm__inst_executed_pipe_tensor.avg.pct_of_peak`: Utilization rate of the Tensor Core pipeline. For optimized Large Language Model (LLM) training, this value should be maintained at 60-80% or higher to ensure no bottlenecks.

**Regarding memory alignment constraints**, Tensor Cores' hardware buses ingest data in chunks such as 8x8 or 16x16. Therefore, if a PyTorch model's Linear layer `in_features` or `out_features`, or `batch_size`, is not a multiple of 8 or 16, the compiler is forced to abandon Tensor Cores and fall back to slower CUDA Cores, or incur overhead by padding with zeros. Just as CPU engineers pay attention to cache line alignment, AI engineers must absolutely consider 8/16 multiple alignment for matrix dimensions.
