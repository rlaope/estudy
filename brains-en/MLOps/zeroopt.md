# ZeRO Optimizer (DeepSpeed, FSDP)

Training large language models (LLMs) with tens of billions of parameters requires terabytes of memory.

However, even top-tier GPUs like the A100 and H100 only have 80GB of VRAM.

How do distributed system engineers overcome the physical limitation where even a single-GPU memory model cannot fully fit?

The core of the **ZeRO (Zero Redundancy Optimizer)** architecture is to physically eliminate memory redundancy that occurs when deploying GPU distributed training models across multiple devices.

- **Model States**: These are the three core data types that permanently occupy GPU HBM space during training.
  - **Parameters**: The model's weight data $W$
  - **Gradients**: The derivative values calculated during backward pass ($\nabla W$)
  - **Optimizer States**: Momentum and variance data maintained by optimizers like Adam
- **Traditional Data Parallel (DP)**: In this method, all GPUs identically hold 100% perfect copies of the model states in HBM, and only the data batches for training are split for computation.
- **ZeRO**: An architecture devised by the Microsoft DeepSpeed team, where instead of all GPUs redundantly holding copies of the model, the entire model state is physically sharded into N pieces (equal to the number of GPUs) and distributed across each GPU's HBM. PyTorch's FSDP (Fully Sharded Data Parallel) is also a native implementation of this ZeRO-3 concept.

<br>

## Physical/Structural Phenomenon Definition

This refers to critical memory redundancy and OOM bottlenecks that occur in HBM when using the traditional Data Parallel (DP) method.

**Optimizer State Memory Hogging:** Let's assume training a 70B LLaMA model with 16-bit (FP16, 2 bytes) parameters.

- Parameters: $70 \times 10^9 \times 2 \text{ Bytes} \approx 140 \text{ GB}$
- Gradients: Same size as parameters $\approx 140 \text{ GB}$
- Adam Optimizer (FP32 standard): Requires storing both momentum and variance, so 4 times the parameter size $\approx 280 \text{ GB}$. A total of 560GB of HBM is needed. A single 80GB GPU is nowhere near enough, and even if 8 GPUs are combined using traditional DP, each GPU would need to hold the entire 560GB, leading to an immediate CUDA OOM error and kernel crash upon hardware launch.

Model scalability limitations due to memory barriers: The tensor cores are idle, but there's no space to load data into VRAM, leading to a hardware deadlock where training cannot even begin.

### Solution

ZeRO sacrifices communication bandwidth (NVLink, RDMA) to eliminate HBM space redundancy, thereby infinitely expanding the size of models that can be computed. It is divided into 3 stages based on the level of partitioning.

- **ZeRO-Stage 1 (Optimizer State Partitioning):** Only the optimizer states, which consume the most memory, are sharded and distributed among the GPUs. This significantly reduces memory usage.
- **ZeRO-Stage 2 (Gradient Partitioning Added):** In addition to optimizer states, gradient data (after backpropagation) is also divided into N equal parts, with each GPU owning only its assigned portion.
- **ZeRO-Stage 3 / FSDP (Full Parameter Partitioning Applied)**: Even parameter weights are completely sharded and distributed. A single GPU only holds 1/N fragments of the entire model, completely breaking the physical limits of HBM capacity. As more GPUs are added to the cluster, the model size that can be loaded scales linearly.

### RDMA

When sending data from server A to server B using typical TCP/IP socket communication, it goes through `user space buffer -> OS TCP/IP stack -> NIC`, and on the receiving server, it goes up through the kernel in reverse. This process involves CPU intervention and data copying, which slows it down.

RDMA (Remote Direct Memory Access) is a networking technology that completely bypasses the OS kernel and CPU.

Server A's NIC directly fetches data from Server A's memory and directly writes it to a specific address in Server B's memory (RAM or GPU VRAM) via Server B's NIC. Since the CPU is not involved at all, there is no context switching overhead, and ping times are extremely low. In AI clusters, where GPUs across multiple servers exchange data, this technology is essential; without it, network bottlenecks would make the system unusable.

### Parameters and Gradients

- **Parameter (Weight, $W$)**: These are the numerous variable weights an AI model possesses.
- **Gradient ($\nabla W$)**: This is an update instruction value that indicates how much and in which direction each parameter should be adjusted to reduce the error.

For example, if there are 100 million parameters in matrix form, during training, the update instructions for each of these 100 million parameters must be precisely paired 1:1, such as increasing parameter 1 by +0.01 and decreasing parameter 2 by -0.05.

That is, the operation $\text{new parameter} = \text{old parameter} - (\text{learning rate} \times \text{gradient})$ must be performed. This requires the shape and count of the existing parameter matrix and the gradient matrix to be perfectly identical for mathematical validity. Therefore, if parameters are 140GB, the gradients will also be 140GB.

<br>

## Hardware Data Path and Memory Hierarchy Operation Principle

Let's trace how a GPU, holding only 1/N fragments in the most extreme **ZeRO-3 (FSDP)** environment, continuously performs full network forward and backward computations. (Assuming 8 GPUs)

1.  **Waiting Before Forward Pass**: GPU0 holds only 1/8 of the parameters for the model's first layer in HBM and needs the remaining 7/8 to perform computations.
2.  **All-Gather Communication Trigger (JIT Fetch)**: Based on the compute cores (SMs), just before the first layer begins computation, it requests parameter fragments from the other 7 GPUs via the hardware communication network (NVLink or RoCEv2). Parameters flow into the empty HBM space of GPU 0 via the Network Interface Card (NIC) and PCIe/NVLINK switches.
3.  **Tensor Core Computation:** In the brief moment when the first layer's parameters are 100% fully assembled, the Tensor Cores perform matrix multiplication operations like a storm via `HMMA` instructions.
4.  **Memory Eviction/Free**: As soon as the first layer's computation is complete, GPU 0 physically deletes the 7/8 parameter fragments borrowed from others from HBM to free up memory.
5.  **Backward Pass and Reduce-Scatter**: During backpropagation, parameters are temporarily gathered to calculate gradients. The computed full gradients are then distributed back to their respective GPUs, and the GPU immediately deletes the gradient fragments belonging to others.
6.  Consequently, in exchange for HBM savings, there is constant parameter packet movement between nodes. This means the GPU memory capacity bottleneck is replaced by a network bandwidth bottleneck.

<br>

### Low-Level Metrics and Profiling Log Analysis

When running training with DeepSpeed, this is a terminal-level initialization log that can diagnose the physical state of node memory.

```
[INFO] [logging.py:96:log_dist] [Rank 0] DeepSpeed info: version=0.10.0, git-hash=unknown, git-branch=unknown
[INFO] [engine.py:3213:_configure_zero_optimizer] ZeRO Stage 3 is enabled.
[INFO] [engine.py:3214:_configure_zero_optimizer] Partitioning Parameters, Gradients, and Optimizer States.

# Memory allocation status report for a single GPU after parameter partitioning
[INFO] [memory_stats.py:45] OOM Catch: False
[INFO] [memory_stats.py:46] --------------------------------------------------
[INFO] [memory_stats.py:47] Allocated Memory (after ZeRO-3 init): 
[INFO] [memory_stats.py:48]     Total Model Parameters: 70.00 B
[INFO] [memory_stats.py:49]     Total Allocated per GPU: 18.25 GB
[INFO] [memory_stats.py:50]     Remaining HBM Capacity: 61.75 GB
[INFO] [memory_stats.py:51] --------------------------------------------------
```

A 70B model, which was impossible to load on a single GPU due to exceeding 140GB, settled in, occupying only 18.25GB of HBM (`Allocated per GPU`) immediately after ZeRO Stage 3 initialization.

The remaining 61GB of free space will be used at runtime to temporarily fetch fragments from other GPUs via All-Gather or as a buffer to hold activation data.

This proves that the distributed system is operating normally without OOM.

<br>

### Practical Application Layer and Profiling Settings

When running distributed training scripts in an AI infrastructure environment, the hardware's behavior is directly controlled via the JSON configuration file (`ds_config.json`) injected by the DeepSpeed engine.

```json
{
  "train_batch_size": 256,
  "gradient_accumulation_steps": 4,
  "zero_optimization": {
    "stage": 3,
    "contiguous_gradients": true,
    "overlap_comm": true,
    "reduce_scatter": true,
    "reduce_bucket_size": 500000000,
    "allgather_bucket_size": 500000000,
    "offload_optimizer": {
      "device": "cpu",
      "pin_memory": true
    },
    "offload_param": {
      "device": "nvme",
      "nvme_path": "/mnt/nvme_storage",
      "buffer_count": 5,
      "buffer_size": 100000000
    }
  },
  "fp16": {
    "enabled": true
  }
}
```

- `stage: 3`: Partitions all parameters, gradients, and optimizer states to unlock HBM limitations.
- `overlap_comm: true`: This is a core hardware optimization. While the Tensor Cores perform computations for the L-th layer, a background network thread pre-fetches the L-1-th layer parameters via All-Gather communication. This overlaps computation and communication, masking network bandwidth bottlenecks.
- `offload_optimizer`, `offload_param` (ZeRO-Infinity): If HBM is full even with partitioning or insufficient GPUs, unused fragments are forcibly offloaded to the host CPU's system RAM (`device: "cpu"`) or even to an NVMe SSD (`device: "nvme"`) plugged into the motherboard. While training speed slows down due to traversing the PCIe bus, this is an engineering survival technique that allows ultra-large models to be forcibly run, overcoming physical VRAM limitations.
