# GPU Hardware and Performance Models

When studying inference serving, I keep hitting the same wall.

- They say decoding is memory-bound, but how exactly is that determination made?
- Why does FlashAttention have to be **rewritten from scratch** for every hardware generation?
- When a serving engine chooses FlashAttention on Hopper and a different kernel on Blackwell, what criteria guide that decision?
- If I switch to FP4, where exactly is time saved?
- They say throughput increases with larger batches, but at what point do the benefits stop?

The answers to all these questions lie within GPU hardware. Serving engines are tools that leverage hardware characteristics, and model architectures are reverse-engineered results tailored to those characteristics.

Without understanding the hardware layer, the two layers above remain a collection of rules to be memorized. Once understood, most conclusions can be derived.

So, this article has two goals:

1.  **To enable judgment:** When a new GPU is released, one should be able to look at the spec sheet and calculate what a given workload means for it. The Roofline model is a tool for this.
2.  **To prepare for reading kernels:** Without understanding warps, shared memory, bank conflicts, or the asynchronous execution model of Tensor Cores, FlashAttention source code is an enigma. Once understood, it becomes clear why things were done a certain way.

<br>

## Why GPUs Are Designed This Way

CPUs and GPUs don't solve the same problem differently; they are **fundamentally designed to solve different problems based on different questions.**

The CPU asks: How quickly can a single task be completed? The GPU asks: How many tasks can be completed per unit of time?

The former is **latency-optimized**, the latter **throughput-optimized**.

> To use a parcel delivery analogy, a CPU is like a motorcycle courier, delivering one package quickly. It predicts traffic (branch prediction), memorizes frequent routes (large cache), and reorders tasks if blocked (out-of-order execution). However, it handles only one or two items at a time. A GPU is a cargo truck. If it only sends one item, it's slower than a motorcycle, but if you're sending ten thousand containers, there's no comparison.

This difference stems from the allocation of silicon area. CPUs dedicate most of their die area to **control logic and cache**. They feature branch predictors, out-of-order execution schedulers, and tens of MBs of L3 cache. GPUs, on the other hand, pack the same area with **compute units**.

```
H100 (132 SMs)
  Register File: 64K × 32bit per SM → Total approx. 33.8 MB
  L2 Cache:                                     50 MB
  → Register file is approx. 68% of L2

Typical Server CPU
  Architectural Registers: hundreds of bytes to a few KB per core
  L3 Cache:          tens of MB
  → Register file is thousands of times smaller than cache
```

CPU registers are negligibly small compared to cache, while GPU registers are comparable to cache.

This design choice is because it's difficult to physically hold the registers of all resident threads simultaneously. Thus, a CPU context switch, which involves saving and restoring registers to memory, incurs a heavy cost of hundreds to thousands of cycles.

GPUs, however, switch contexts without saving and restoring; they merely change which register bank to read from, resulting in virtually zero overhead.

However, even a large register file is finite. If a single thread uses many registers, the number of threads that can reside simultaneously decreases. This is the physical basis for occupancy, which we'll discuss later.

Also, the path to reduce latency with cache is blocked. If H100's 50MB L2 cache is divided by the number of resident threads (132 SMs x 2048 threads ≈ 270,000), each thread gets about 185 bytes, which is incomparable to a CPU core monopolizing MBs. GPUs cannot cope with memory latency by increasing cache hit rates; they must use other methods.

### How GPUs Hide Latency

GPUs also need to fetch data from memory, which takes hundreds of cycles.

If GPUs don't reduce this latency like CPUs do with cache and prediction, how do they endure memory loading times?

The answer is to hide latency, not reduce it.

```
CPU Strategy — Reduce Latency
  Thread 1: [Compute][Memory Wait 300 cycles.......][Compute]
            ↑ Increase cache hit rate to shorten this wait

GPU Strategy — Hide Latency
  Warp 1:  [Compute][────── Memory Wait ──────][Compute]
  Warp 2:        [Compute][────── Wait ──────][Compute]
  Warp 3:              [Compute][───── Wait ─────][Compute]
  Warp 4:                    [Compute][──── Wait ────]
           ↑ Execute other warps while one warp waits → Compute units stay busy
```

This is the entirety of GPU performance. If there are many warps computing, memory latency is hidden; if there aren't enough, it becomes apparent.

The concept of occupancy, which will be discussed later, is precisely the metric that measures this.

This design is the root of the constraints that will appear later. Since all registers must be held, the register file must be huge. For H100, this means 64K 32-bit registers per SM. But it's still finite, so if one thread uses many registers, the number of warps that can be active simultaneously decreases.

### GPU Performance

When evaluating GPU performance, three things should always be considered:
-   **Compute Capability:** Floating-point operations per second (FLOPs). If insufficient, it's compute-bound.
-   **Memory Bandwidth:** Bytes transferred per second (GB/s). If insufficient, it's memory-bound.
-   **Parallelism:** Number of concurrently active warps. If insufficient, latency is exposed (neither compute nor memory is fully utilized).

The third point is often forgotten. Even if compute capability and bandwidth are sufficient, **if parallelism is lacking, neither can be fully utilized.** This is the fundamental reason why a GPU idles at batch size 1.

<br>

## GPU

### SM - The True Unit of a GPU

Consider the phrase "**10,000 GPU cores**." A spec sheet might list a number like 16,896 CUDA cores.

This number might suggest that over ten thousand independent processors are each doing different tasks, but it shouldn't be understood that way.

**Actual Hierarchy**: The true unit of a GPU is not the core, but the **SM (Streaming Multiprocessor).**

```
GPU (e.g., H100 SXM)
├── SM 0
│   ├── Partition 0 ─ [FP32 Units ×32][INT Unit][Tensor Cores][Warp Scheduler][16K Registers]
│   ├── Partition 1 ─ ...
│   ├── Partition 2 ─ ...
│   ├── Partition 3 ─ ...
│   ├── Shared Memory / L1 Cache (Max 228 KB)
│   └── TMA Unit (Hopper and later)
├── SM 1
├── ...
└── SM 131                      ← H100 has 132 SMs

Common
├── L2 Cache (50 MB class)
└── HBM (80~288 GB)
```

Each of the four partitions within an SM has its own warp scheduler.

Every cycle, each scheduler picks one ready warp and issues instructions.

This means one SM can advance up to four different warps per cycle.

SMs are the unit because three types of resources are allocated per SM:
1.  **Register File:** 64K (32-bit) per SM. All threads launched on this SM share these.
2.  **Shared Memory:** Max 228KB per SM (H100). Blocks launched on this SM share this.
3.  **Warp Slots:** Max 64 warps per SM.

Kernel performance is determined not by the number of cores, but by how well these three resources are shared.

Therefore, understanding the internal structure of an SM is crucial.

SM configuration by generation, e.g.:
-   A100 (Ampere): 108 SMs, max 164KB shared memory per SM, 64K x 32bit registers/SM
-   H100 (Hopper): 132 SMs, max 228KB shared memory per SM, 64K x 32bit registers/SM
-   B200 (Blackwell): 148 SMs x 2 dies, max 228KB shared memory per SM, 64K x 32bit registers/SM

> The B200 has two physically attached dies. These two dies are connected by a **NV-HBI** link at 10TB/s, making them appear as a single GPU to software. This design aims to prevent programmers from needing to worry about die boundaries, but sometimes invisible boundaries can cause performance issues.

### Threads, Warps, Blocks - Execution Hierarchy

**How to manage tens of thousands of threads?**

If tens of thousands of threads were scheduled independently, the control logic couldn't handle it. The GPU's goal of saving area by reducing control logic would be lost.

A **Warp** refers to a group of 32 threads. This is the true execution unit of the GPU. The 32 threads within a warp **execute the same instruction simultaneously.** Only the data differs.

```
Hierarchy

Thread      ─ Unit for programmers to write code
   ↓ 32
Warp          ─ Hardware execution unit ★
   ↓ Multiple
Block (CTA)    ─ Unit sharing shared memory. Assigned to a single SM
   ↓ Multiple
Cluster        ─ New in Hopper. Can access each other's shared memory
   ↓ Multiple
Grid        ─ Entire kernel execution
```

> A warp is like a platoon of 32 people standing in a line, moving to the drill sergeant's command. When commanded "Forward march!", all 32 move simultaneously. They start from different positions (different data), but perform the same action.

The reason for grouping in 32s is that applying one instruction to 32 data items reduces the instruction decoding cost by a factor of 32. This method is called SIMT (Single Instruction Multiple Threads). NVIDIA chose 32 as a balance between control logic reduction and flexibility, and it hasn't changed across generations. (AMD uses 64 in CDNA, and this difference is one reason why porting kernels between the two platforms is difficult.)

### Warp Divergence - The Cost of Design

Executing the same instruction together comes with a clear cost: **when threads within a warp take different branches.**

```c
if (threadIdx.x % 2 == 0) {
    A();          // 16 even threads
} else {
    B();          // 16 odd threads
}
```

Hardware processes this as follows:

```
Step 1: Execute A() — Only even threads active, 16 odd threads idle
Step 2: Execute B() — Only odd threads active, 16 even threads idle
       → Total execution time = Time for A + Time for B (Not parallel!)
       → Effective throughput 50%
```

This is called warp divergence. In the worst case, where branches diverge into 32 paths, throughput becomes 1/32.

**In LLM Kernels, this is a real problem point.** It's attention causal masking.

Since masking depends on the sequence position, a naive implementation would lead to divergence. Therefore, FlashAttention-like kernels skip blocks where the mask is fully applied and only perform masking calculations for blocks that straddle the boundary, pushing the branches to tile boundaries. If branches cannot be eliminated, the key is to **align branches with warp boundaries.**

> Attention Causal Masking: A technique in AI to prevent the model from seeing future tokens that haven't been generated yet when creating or understanding sentences.
>
> GPGPU Tile: A Tile Boundary is the area where a tile meets an adjacent tile when large-scale data is broken down into small blocks (tiles) for parallel computation. It's a technique to divide the entire data into small rectangular tiles to reduce slow global memory access and increase shared memory utilization. In other words, these are memory chunks that threads efficiently divide among themselves.

#### Thread Block Cluster - Hopper

Hopper introduced a layer called a cluster between blocks and grids. Up to 16 blocks can form a cluster, and they can **directly access each other's shared memory.**

This is called Distributed Shared Memory (DSMEM).

**Why was it needed:** Shared memory was confined within an SM, so data exchange between blocks had to go through global memory, incurring high round-trip costs.

**How much better is it:** Microbenchmark measurements show that inter-SM access latency within a cluster is 33-213 cycles, significantly lower than L2 cache or global memory access.

**Where is it used in LLMs?** >> When multiple blocks need the same weight tile, one block reads it and broadcasts it to the entire cluster (TMA multicast). This reduces L2 traffic, and is relevant in GEMM where one block of matrix A is reused across multiple N tiles.

### Occupancy - How Much Latency Can Be Hidden

Let's discuss the metrics you'll look at first when diagnosing kernel performance.

**What happens when there aren't enough warps:** In Chapter 0, we discussed how GPUs hide latency. To hide it, there must be other warps to execute when one is waiting. If not, it simply stalls.

```
When there are enough warps
Compute Units: ████████████████████████  100% utilization

When there are not enough warps
Compute Units: ███░░░░░░░░███░░░░░░░░███  20% utilization
                ↑ Memory wait, no warps to fill
```

#### **Definition of Occupancy**

Occupancy = `Number of warps actually resident on an SM / Maximum number of warps an SM can accommodate`

For Hopper, with a maximum of 64 warps per SM, if 32 warps are resident, occupancy is 50%.

Occupancy is determined by the lowest of three upper bounds, and **the lowest value is the actual occupancy.**

#### Register Pressure

An SM has 64K 32-bit registers. If one thread uses R registers:

```
Max resident threads = 65,536 / R
Max resident warps   = 65,536 / (R × 32)

When R = 32 → 64 warps (100%)
When R = 64 → 32 warps (50%)
When R = 128 → 16 warps (25%)
When R = 255 →  8 warps (12.5%)   ← Max per thread
```

**More complex kernels use more registers, which reduces occupancy.**

This becomes important in the context of Tensor Cores. If accumulators are held in registers, registers become scarce. Blackwell addressed this by moving accumulators to separate memory, which we'll discuss further below.

#### Shared Memory Pressure

If one block uses S bytes of shared memory, only `SM's total shared memory / S` blocks can reside on the SM.

```
H100: Max 228 KB per SM
Using 64 KB per block → Only 3 blocks resident
Using 32 KB per block → 7 blocks resident
```

FlashAttention places tiles in shared memory for processing, so **tile size directly impacts occupancy.** Increasing tile size improves data reuse, which is good, but reduces occupancy. This tug-of-war is a fundamental part of kernel tuning.

### Block Size

If the number of threads per block is awkward, resources are wasted. This is why multiples of 32 (the warp size), like 128 or 256, are used.

And **higher occupancy doesn't always mean faster performance.** This is a common pitfall: **aiming for 100% occupancy** is not always the best approach.

Occupancy only indicates the ability to hide latency, not performance itself. In fact, some high-performance kernels intentionally aim for lower occupancy.

This is because if registers and shared memory are used generously to maximize data reuse, memory access itself is reduced, and there's less latency to hide. Therefore, fewer warps are needed, meaning occupancy isn't the core issue.

```
Strategy A (High Occupancy): Small tiles, fewer registers → Hide latency with 64 warps
Strategy B (Low Occupancy): Large tiles, many registers  → Less memory access, less to hide

FlashAttention-like kernels are closer to Strategy B.
```

If occupancy is 50% or higher, it's generally sufficient. Beyond that, check for other bottlenecks first.

If it's below 25%, it's worth investigating register or shared memory usage.

> Why intentionally aim for lower occupancy?
>
> Strategy A is to deploy many workers because data fetching takes a long time anyway. However, FlashAttention's Strategy B is to avoid slow memory in the first place. GPUs have very fast but extremely limited-capacity registers and shared memory. If one thread block monopolizes a lot of this precious memory, it doesn't need to go to slow VRAM, allowing it to reuse data continuously and finish calculations at ultra-high speed. But this precious memory has a finite total capacity, so if one block uses many registers, the number of blocks that can be launched simultaneously on the GPU decreases. This is the strategy of intentionally lowering hardware occupancy to maximize memory utilization.

High occupancy = many workers doing a good job of patching things up; a decent optimization.

Intentional low occupancy = giving a few workers top-tier equipment (registers) to finish tasks at ultra-high speed without patching (FlashAttention).

#### How Much is Needed to Hide Latency?

Returning to occupancy, how much is needed can be estimated using **Little's Law**.

```
Required concurrent bytes = Memory Bandwidth × Memory Latency

H100 Example:
  Bandwidth 3,350 GB/s, HBM Latency approx. 600ns
  → 3,350 × 10⁹ × 600 × 10⁻⁹ ≈ 2 MB

This means to saturate bandwidth, approximately 2MB of memory requests must always be "in flight."
```

This number directly influences kernel design. If a single warp requests a small amount at a time, many warps are needed. If one request is large (e.g., a large tile at once via TMA), fewer warps are needed. **This calculation is behind Hopper's introduction of TMA.**

### Summary

-   SMs are the unit of GPU resource allocation, where registers, shared memory, and warp slots are divided.
-   Warps are execution units of 32 threads. If branches diverge within a warp, throughput is lost.
-   Warp divergence, due to branching within a warp, leads to serialization and affects attention masking design.
-   Clusters and DSMEM enable shared memory access between blocks, based on tile reuse and TMA multicast.
-   Occupancy is resident warps / max warps, indicating latency hiding capability. Higher isn't always faster.
-   Register pressure is when per-thread registers limit occupancy, a motivation for Tensor Core generation evolution.

> TMA (Tensor Memory Accelerator) is a hardware engine and instruction set introduced from NVIDIA Hopper, SM 90+ architecture, that asynchronously transfers large, multi-dimensional data between global memory and shared memory.

<br>

## Memory Hierarchy

We've seen how GPUs hide latency. This chapter discusses where that latency comes from and how to prevent it from occurring in the first place. Most kernel optimization work involves the content of this chapter.

### Layer

Faster memory is more expensive. More precisely, fast memory must be physically close to the compute units, but that space is finite. Therefore, all computers use a **hierarchy from small and fast to large and slow.**

**Actual figures for H100**

```
Capacity (per SM/total)   Latency          Bandwidth       Notes
┌──────────────┐
│  Registers    │   256 KB / SM     ~1 cycle      ~tens of TB/s   Thread-private
├──────────────┤
│ Shared Mem/L1 │   228 KB / SM     ~30 cycles     ~few TB/s     Block-local sharing, manual management
├──────────────┤
│  L2 Cache    │   50 MB (total)   ~200 cycles    ~few TB/s     GPU-wide sharing, automatic
├──────────────┤
│  HBM (VRAM)  │   80 GB           ~600 cycles    3.35 TB/s    ★ This is the bottleneck
├──────────────┤
│ CPU DRAM     │   TB-class        ~few μs        ~64 GB/s     Via PCIe 5.0
├──────────────┤
│  NVMe SSD    │   tens of TB       ~tens of μs     ~7 GB/s      
└──────────────┘
```

**The gap between HBM and shared memory is 20x, with similar differences in latency and bandwidth.**

This gap is the single biggest game in kernel optimization. In summary:

Maximize reuse of data fetched once from HBM within shared memory registers.

This statement precisely summarizes what FlashAttention does.

Instead of writing and re-reading the attention score matrix to HBM, it's broken into tiles and processed entirely within shared memory.

The algorithm hasn't changed; it's the **location where data resides** that has changed.

KV Cache (HBM -> CPU DRAM -> SSD) extends the table one step further down. The fact that PCIe is about 2% of HBM's bandwidth determines the break-even point for that hierarchy, as seen in this table.

When using CPU memory, carefully consider the bandwidth, losses, and total memory capacity.

### Coalesced Access - How 32 Threads Read Memory

**When warps read scattered addresses**: When 32 threads in a warp each read memory, how these 32 requests are grouped determines the effective bandwidth.

Memory is not read byte by byte. It's read in **fixed-size units called transactions (typically 32-128 byte sectors)**. Even if only 4 bytes are needed, 32 bytes are fetched.

**Conditions for Coalescing**

```
Good case — Sequential access
Thread:   0    1    2    3   ...  31
Address:  0x00 0x04 0x08 0x0C ... 0x7C
        └────────── Contiguous 128 bytes ──────────┘
        → 1 transaction of 128 bytes handles all 32 requests ✅

Bad case — Strided access (32-byte interval)
Thread:   0      1      2         31
Address:  0x000  0x020  0x040  ... 0x3E0
        → Separate transaction per thread → 32 transactions ❌
        → Needs 128 bytes, but actually reads 1,024 bytes (12.5% efficiency)
```

Coalescing refers to the state where a warp's requests are concentrated in contiguous addresses, processed by a smaller number of transactions.

The HBM bandwidth of 3.35TB/s is for perfectly coalesced access. If access is scattered, the effective bandwidth can drop to half, a quarter, or even an eighth.

In LLM decoding, which is memory-bound, if effective bandwidth halves, **speed halves.** Without changing the algorithm, simply changing data batching can yield or lose 2x performance.

This manifests in KV Cache layout. Paged KV Cache has physically scattered blocks, so a naive PagedAttention implementation won't coalesce accesses. Therefore, attention kernels handling Paged KV design the layout to **place blocks contiguously internally** and **jump only at block boundaries.** The block size of 16 tokens is also related to this transaction size.

#### How to Diagnose

You can observe the following metrics in Nsight Compute:

```bash
ncu --metrics l1tex__t_sectors_pipe_lsu_mem_global_op_ld.sum,\
l1tex__t_requests_pipe_lsu_mem_global_op_ld.sum ./my_program
```

Sectors / Requests is the coalescing efficiency. Ideally, 4 sectors per 1 request (128 bytes = 32 bytes x 4). If the ratio increases, it indicates scattered access.

### Shared Memory Bank Conflicts

Shared memory is fast, but not perfectly parallel.

Shared memory is fast, but 32 threads cannot simultaneously read from any address.

#### 32 Banks

Shared memory is divided into **32 banks**, each 4 bytes wide.

The rule for assigning addresses to banks is simple:

```
Bank Number = (Byte Address / 4) % 32

Address:  0    4    8   12  ...  124  128  132
Bank:     0    1    2    3  ...   31    0    1   ← Cycles every 128 bytes
```

**Each bank can handle only one access per cycle.** If 32 threads in a warp access 32 different banks, it completes in one cycle. If two threads access **different addresses** in the same bank, it's serialized into two cycles.

It's like having 32 counters. If all 32 people go to different counters, it's done immediately. But if they all flock to one counter, 32 people will wait sequentially.

```c
// Reading a 32x32 float tile in shared memory column-wise
__shared__ float tile[32][32];

// Thread i reads tile[i][0] (column access)
// Address = i × 32 × 4 bytes = i × 128 bytes
// Bank = (i × 128 / 4) % 32 = (i × 32) % 32 = 0
//        → All 32 threads access Bank 0! 32-way conflict ❌ 32x slower
```

Row-wise access `tile[0][i]` is perfectly distributed, but column-wise access becomes the worst case.

And matrix multiplication kernels naturally read one matrix row-wise and the other column-wise, so this problem is inevitable.

#### Padding and Swizzling

Padding is a simple and classic method:

```c
__shared__ float tile[32][33];   // 33! One extra slot

// Bank = (i × 33 × 4 / 4) % 32 = (i × 33) % 32 = i % 32
//        → 32 threads evenly distributed across 32 banks ✅
```

By increasing the row length of the array by one, conflicts are eliminated. This technique uses 3% more memory but prevents a 32x slowdown.

Swizzling is the standard in modern kernels. Padding wastes memory and, more importantly, **breaks the alignment requirements of TMA and Tensor Cores.** Therefore, modern kernels use swizzling, which shuffles addresses with XOR operations.

```
Swizzled Address = Original Address XOR (Pattern derived from row number)
-> Logical data location remains the same, only physical bank assignment is redistributed
```

CUTLASS and FlashAttention use this method. Hopper's TMA **supports swizzle modes as a hardware feature.** If a swizzle pattern is specified in the TMA descriptor, it's automatically applied during transfer.

**Broadcast is not a conflict**

One important exception: if multiple threads read the same address in the same bank, it's not a conflict. The hardware reads it once and broadcasts it to all (broadcast).

This property is useful in practice. Placing constants or scale factors referenced by all threads in shared memory incurs no cost.

### HBM

This is also the bottleneck, but let's first understand why memory bandwidth dominates performance.

Decoding is a memory-bound operation. Let's confirm the basis for this.

If a 70B model is decoded in BF16 for one step:

```
Weights to read: 70B × 2 bytes = 140 GB
H100 Bandwidth:      3,350 GB/s
→ Minimum time = 140 / 3,350 ≈ 41.8 ms

Compute for the same step (batch 1):
2 × 70B × 1 = 140 GFLOP
H100 BF16 Compute Capability: 989 TFLOPS
→ Minimum time = 140 × 10⁹ / 989 × 10¹² ≈ 0.14 ms
```

41.8 ms vs 0.14 ms is about a 300x difference. This means computation finishes in 0.14 ms, but 41.8 ms are spent moving data. The GPU's compute units are idle 99.7% of the time.

This single calculation reveals:
-   **Why quantization is effective:** Reducing 140GB to 70GB halves the time. Compute remains the same, but it becomes 2x faster.
-   **Why increasing batch size feels "free":** Even with batch 32, only 140GB of weights are read. Compute increases 32x, but 0.14 ms becomes only 4.5 ms, which is still within 41.8 ms. So, practically, no latency issue is felt.
-   **Why speculative decoding is good:** It uses leftover compute resources to pre-verify multiple tokens, trading less memory usage for more compute resource utilization.

**HBM Spec Changes by Generation**

| Generation | Representative GPU | Capacity per Stack | Bandwidth per GPU | Era |
|------|-----------|------------:|--------------:|------|
| HBM2e | A100 | 16 GB | 2.0 TB/s | 2020 |
| HBM3 | H100 | 16 GB | 3.35 TB/s | 2022 |
| HBM3e | H200 | 24 GB | 4.8 TB/s | 2024 |
| HBM3e (8-high) | B200 | 24 GB | ~7.7 TB/s | 2024 |
| HBM3e (12-high) | B300 | 36 GB | 8 TB/s | 2025 |
| HBM4 | Rubin (VR200) | 36 GB | approx. 20 TB/s (target 22) | 2026 H2 |

From **HBM4** onwards, the interface width doubled from 1024 bits to 2048 bits per stack.

Pin speeds exceeding 11Gb/s mean bandwidth per stack surpasses 3TB/s. The Rubin GPU will feature 8 such stacks, totaling 288GB.

> What's practically important here is that Rubin's 22TB/s was the original target, but there are reports that initial shipments were adjusted to around 20TB/s if memory suppliers couldn't meet that speed. It's not uncommon for spec sheet numbers to change at the time of shipment in this field. When planning capacity, it's safer to allow for some buffer until actual benchmark results are available, rather than blindly trusting vendor announcements.

**Why Capacity is as Important as Bandwidth**

It's easy to forget capacity when talking about bandwidth, but capacity is obviously crucial in serving **because it determines the number of concurrent users.** Let's look at the KV cache calculation in the serving notes:

```
GPU Memory = Model Weights + KV Cache + Activations + Spare
             └ Fixed ┘   └ Proportional to user count ┘

Lower proportion of weights → More space for KV Cache → More concurrent users
```

This explains why a 50% capacity increase from B200 (192GB) to B300 (288GB) is a big deal. If a 1-trillion parameter model is loaded in FP4, it's about 500GB. Two cards in an NVL72 rack (72 cards) can hold the weights, and **the remaining 70 cards can be entirely used for KV cache and concurrent requests.**

### L2 Cache

This layer is often forgotten because L2 operates automatically, so programmers rarely need to worry about it.

However, in LLM serving, it unexpectedly impacts performance.

**When multiple requests read the same weights**, all 32 requests in a batch pass through the same layer. The weights read for the first request remain in L2 and are reused by the others.

H100's 50MB L2 is often sufficient to hold the weights of a single layer.

**Therefore, increasing batch size doesn't linearly increase bandwidth demand.** The physical basis for the earlier calculation that even with batch 32, only 140GB of weights are read, is the L2 and cache hierarchy.

#### Kernel-level Utilization

There are also means to explicitly control L2.

```c
// Hint to keep specific data resident in L2 (CUDA 11+)
cudaStreamSetAttribute(stream, cudaStreamAttributeAccessPolicyWindow, &attr);
```

This can be used to pin frequently reused small tensors (e.g., router weights, normalization parameters) to L2. However, its effectiveness depends on the workload, so it's an item to try after other optimizations.

#### Summary

When looking at a kernel, consider the memory optimization checklist in this order:

1.  **Has HBM access been reduced?** Incorrect handling can lead to several times performance loss. Diagnose by calculating total bytes moved.
2.  **Are global memory accesses coalesced?** Can lead to 2-8x performance loss. Check the sectors/requests ratio.
3.  **Are there shared memory bank conflicts?** Can lead to up to 32x performance loss. Check the `shared_ld_bank_conflict` metric.
4.  **Is the data type larger than necessary?** Can lead to 2-4x performance loss. Consider quantization.
5.  **Is L2 reuse leveraged?** Can lead to tens of percent performance loss. Check batch configuration and access order.

Point 1 is overwhelmingly important. Points 2-5 are meaningful only after point 1 is properly addressed. No matter how perfectly coalesced, it's less effective than reducing 100 reads to 10.

<br>

## Tensor Cores - Why Kernels Are Rewritten Every Generation

FlashAttention needs to be rewritten for every hardware generation.

Why is that? Let's find out the reasons for rewriting.

### The Emergence of Tensor Cores

**Matrix multiplication is inefficient with general compute units.**

Over 90% of deep learning operations are matrix multiplications. Performing matrix multiplication with typical FP32 units follows this flow:

```
For each multiply-add:
  1. Read element of A from register
  2. Read element of B from register
  3. Multiply
  4. Add to accumulator
  5. Write to register
→ Data movement (1, 2, 5) costs more than actual computation (3, 4)
```

**Since one instruction handles only one multiply-add**, the overhead of instruction issuance and register access is high relative to the amount of computation.

**The entire small matrix multiplication in one instruction**: Tensor Cores are dedicated units that handle **matrix multiply-accumulate (MMA) of small matrix blocks as a single instruction.**

```
D = A x B + C

Volta generation: 4x4 matrix units
Current generation: much larger tiles (e.g., 64 x 256 x 16)
```

One instruction performs thousands of multiply-adds, distributing instruction overhead. Internal data uses dedicated wiring, reducing register round trips.

**How much better is it? Here's a comparison for H100:**

```
FP32 General Compute Units:      67 TFLOPS
BF16 Tensor Cores:          989 TFLOPS   ← Approx. 15x
FP8 Tensor Cores:         1,979 TFLOPS   ← Approx. 30x
```

**Most deep learning performance comes from Tensor Cores.** Kernels using general compute units start with a 15x disadvantage. This is why using Tensor Cores is the first question in kernel design, which is obvious.

**It's difficult to use, though.** Tensor Cores require data to be in the exact layout and exact location.

It's hardware-defined which thread should hold which element.

These layout requirements change with each generation, which is why kernels need to be rewritten.

### Generational Evolution

This section is the core of this chapter. Following how each generation **solved bottlenecks of the previous generation** reveals the logic of evolution.

#### Volta (V100) - The Advent of Tensor Cores

`mma.sync` instruction, **warp-level synchronous execution**. 32 threads in a warp collaborate to perform a small matrix multiplication and wait until the instruction completes.

**Remaining Problem:** Since execution is synchronous, the warp cannot do anything while the Tensor Core computes. Fetching data in advance must be delegated to other warps.

#### Ampere (A100) - The Beginning of Asynchronous Copy

`cp.async` introduced. Copies data from global memory to shared memory **asynchronously without going through registers.**

**Why it's important:** Previously, data had to move from global -> register -> shared memory, consuming registers and causing threads to wait. Now, copies can be initiated while computation continues.

Software pipelining became standard here, with the next tile being fetched while the current tile is computed.

**Remaining Problem:** The copy itself still requires **all threads to participate in address calculation.** 30-40 registers per thread are consumed for address arithmetic.

#### Hopper (H100) - Two Major Changes

**WGMMA - Warp Group Matrix Multiply-Accumulate (Asynchronous)**

`wgmma.mma_sync`. Two things changed:
-   **Warp Group Unit:** 4 warps (128 threads) collaborate as a single unit. Larger tiles are processed with one instruction.
-   **Asynchronous:** After issuing the instruction, other tasks can be performed without waiting for the result.

The effect is significant. According to microbenchmark studies, **WGMMA achieves 95% of Hopper's theoretical performance, while backward-compatible `mma` only reaches 62.9%.** On the same hardware, performance can differ by a third depending on which instruction is used.

Another important change: WGMMA can **read operand A directly into shared memory.** This eliminates the need to move it to registers, reducing register pressure and improving occupancy as seen in the previous chapter.

**TMA - Tensor Memory Accelerator**

This is Hopper's true innovation. Even with `cp.async`, threads still calculated addresses. To fetch a tile from a multi-dimensional tensor, each thread had to calculate its assigned address and perform boundary checks.

**Solution:** Use the dedicated TMA hardware engine. The host creates a descriptor specifying tensor shape, stride, and swizzle mode. In the kernel, a single thread issues a `cp.async.bulk.tensor` instruction with only the tile coordinates. The rest is handled by the TMA engine.

```
Ampere Method
  All 128 threads calculate addresses → Each requests copy → 30-40 registers consumed

Hopper TMA Method
  1 thread says "Fetch tile at this coordinate" → TMA engine handles everything
  Remaining 127 threads perform computation in the meantime → Registers freed
```

> **Analogy** - When moving, instead of 128 employees each carrying their own items, it's like telling a dedicated logistics team, "Bring the items from the 3rd-floor master bedroom." The employees then do their main job.

**How much better is it:** If tile size exceeds 4KB, TMA saturates H100's 3.35TB/s HBM bandwidth. 30-40 registers per thread are also freed. In asynchronous pipeline measurements, up to 39.5% throughput improvement was reported for small blocks.

#### Combining Both - Warp Specialization

TMA and WGMMA are **different hardware units.** Therefore, they can and should run simultaneously for maximum performance. This leads to the standard structure of modern kernels.

```
Producer Warp Group          Consumer Warp Group
       │                                    │
  Fetch tile N+1 with TMA              Compute tile N with WGMMA
       │                                    │
  Signal "arrived" with mbarrier  ──────→  Wait for mbarrier, then proceed
       │                                    │
  (Repeat in a multi-stage circular buffer)

→ Data movement and computation completely overlap
```

Giving different roles to different warp groups within the same kernel is warp specialization. Before Hopper, all warps had to execute the same loop body, making this structure impossible.

This is why FlashAttention-3 is completely different code from FlashAttention-2.

The algorithm is the same, but the **execution structure is different.** FA3 achieving 840TFLOPS (approx. 85% of theoretical) on H100 is due to this structure, while FA2 achieved 50-73% of theoretical on A100.

### Blackwell B200/B300 - tcgen05 and Tensor Memory

As FP4/FP6 and other ultra-low precision Tensor Cores became too fast,

**Register file bandwidth became the bottleneck.** Operands couldn't be supplied at the speed required by the Tensor Cores.

Also, as tiles grew larger, **accumulators consumed registers.** This exacerbated the register pressure problem discussed in the previous chapter.

#### Solution 1. TMEM (Tensor Memory)

Accumulators were moved out of registers to **dedicated on-chip memory.**

```
Ampere/Hopper: Accumulator = Register (threads hold fragmented pieces)
Blackwell:     Accumulator = TMEM (separate 2D memory space within CTA scope)
```

For sm_100, it's a 128-lane x 512-column structure per CTA, 256KB per SM. **The register file is freed during MMA execution, only needed for issuance and finalization.**

> A useful mental model: TMEM has the same relationship to Tensor Cores as L1 cache has to ALUs.

However, programmers must manage it directly. Allocation, deallocation, and data copying must be explicit. To read results in the epilogue, `tcgen95.ld` must be used to bring them into registers. Each warp can only access a quarter of TMEM, so **the entire warp group is needed for the epilogue.**

> CTA stands for Cooperative Thread Array, a term used in NVIDIA CUDA models for thread blocks. It's a group of tens to hundreds of threads, allocated entirely to a GPU hardware core (Streaming Multiprocessor) for execution.

#### Solution 2. Single-Thread Issuance

`tcgen05.mma` **issues a single thread to represent the entire CTA.** This is a step beyond Hopper's warp-group scope. It's possible because all operands are in SMEM and TMEM, which are CTA-shared spaces.

#### CTA Pair

Within a cluster, two CTAs (e.g., 0 and 1, 4 and 5) whose ranks differ only in the last bit can pair up to perform a single MMA.

```
1SM Mode:  One CTA processes an M×N tile
2SM Mode:  A CTA pair processes a 2M×N tile
           A tiles are loaded differently by each
           B tiles are split and shared via DSMEM → Halves load amount
```

This reduces the SMEM capacity and bandwidth needed for the same computation. The largest tile, m256n128k64, cannot fit within a single CTA's TMEM budget, making **2SM mode** essential.

How much better is it? `tcgen05` delivers 2-4x the throughput of Hopper's WGMMA, depending on the data type.

History:
1.  **Volta:** Synchronous warp-level issuance, accumulators in registers, threads manually move data for matrix multiplication, basic form.
2.  **Ampere:** Synchronous warp-level, accumulators in registers, data movement evolved from manual thread control to `cp.async` asynchronous, solving pipelining issues.
3.  **Hopper:** Asynchronous warp-group-level, accumulators in registers. Data movement handled by TMA engine, freeing registers and enabling warp specialization.
4.  **Blackwell:** Single-thread issuance scope, accumulators in TMEM. Data movement handled by TMA + CTA pair, solving register bandwidth bottleneck.

GPU evolution is consistently moving in one direction. Data movement and accumulation are increasingly being detached from thread registers and moved to dedicated hardware and memory. Threads are increasingly left with only the role of issuing commands and receiving results.

### Practical Application Points

**Why do kernels take different paths depending on the hardware?**

Let's take SGLang's backend selection logic as an example. There's a branch:

```
Hopper (SM90)  → FlashAttention 3 (Leverages WGMMA + TMA warp specialization)
Blackwell (SM100/103) → TRT-LLM MHA or CUTLASS path (Leverages tcgen05)
Other / Fallback    → FlashInfer or Triton
```

**Blackwell Workstation Caveat**: Workstation-class Blackwell (SM120, RTX PRO) cannot exceed cluster size 1, so **tcgen05's CTA pair MMA cannot be used at all.**

As a result, FlashInfer's CUTLASS-Blackwell path, which uses `tcgen05`, is not taken, and it falls back to the Triton path. This means lower throughput.

Porting an SM100-specific kernel to SM120 is mechanically possible, but for the largest tiles, the **number of PTX instructions increases by about 256x**, and the achieved throughput remains at 40-70% of SM120's optimal. And SM120's optimal itself is only a fraction of SM100's.

> The term "Blackwell support" means something entirely different for SM100 (datacenter) and SM120 (workstation). When looking at benchmarks, it's important to check which one is being referred to.
>
> PTX is CUDA PTX (Parallel Thread Execution), which refers to a low-level virtual machine and instruction set architecture in GPU programming.
>
> A datacenter is a facility where thousands of servers from enterprise cloud service providers are gathered to process large-scale cloud online services 24/7. A workstation can be thought of as a tower-type or high-performance mobile device that a single person keeps under their desk.

#### What to Look for When Reading Kernel Source Code

If these things stand out when you open CUTLASS or FlashAttention source code, you've understood this chapter.

```cpp
// TMA descriptor creation (host side)
make_tma_copy(...)               → Specify tensor shape, stride, swizzle

// Multi-stage circular buffer
Stages = 4                       → Pipeline depth

// Barrier
cutlass::arch::NamedBarrier / mbarrier   → Producer-consumer synchronization

// Warp role branching
if (warp_group_idx == 0) { /* producer */ } else { /* consumer */ }

// Blackwell specific
tcgen05.mma / make_fragment_C()  → TMEM accumulator
cta_group::2                     → CTA pair mode
```

<br>

## Numerical Precision

Understanding why quantization works at the hardware level will be helpful.

### Floating-Point Structure

**First, you need to know the format names.**

There are E4M3 and E5M2 for FP8, and NVFP4 and MXFP4 are different. If you don't know what these names mean, you can't decide which one to choose.

A floating-point number consists of three parts:

```
[Sign S][Exponent E][Mantissa M]

Value = (-1)^S × 1.M × 2^(E - bias)
```

-   **Sign:** 1 bit, positive/negative.
-   **Exponent:** Determines the representable range. More bits allow for very large or very small numbers.
-   **Mantissa:** Determines precision. More bits allow for finer representation of values.

> The exponent is like the number of digits, and the mantissa is like the significant figures. The difference between "approx. 3×10²³" and "3.14159×10²³" is the difference in mantissa bits. The difference between "can represent up to 10²³" and "can represent up to 10³⁸" is the difference in exponent bits. Simply put, mantissa is how many decimal places, exponent is how large the number can be.

`E4M3` means 4 bits for the exponent and 3 bits for the mantissa. Adding 1 sign bit makes it 8 bits.

Knowing this rule, you can understand the table below:

| Format | Total Bits | Sign | Exponent | Mantissa | Characteristics |
|------|--------:|----:|----:|----:|------|
| FP32 | 32 | 1 | 8 | 23 | Baseline. Ample range and precision |
| TF32 | 19 (stored in 32 bits) | 1 | 8 | 10 | FP32 range + FP16 precision |
| FP16 | 16 | 1 | 5 | 10 | Good precision, narrow range |
| BF16 | 16 | 1 | 8 | 7 | Same range as FP32, sacrificed precision |
| FP8 E4M3 | 8 | 1 | 4 | 3 | Precision-first → Forward pass, weights |
| FP8 E5M2 | 8 | 1 | 5 | 2 | Range-first → Backward pass, gradients |
| FP4 E2M1 | 4 | 1 | 2 | 1 | Cannot be used alone (explained later) |

Why did BF16 push out FP16?

This is an important event that actually happened in deep learning.

FP16 has good precision with 10 mantissa bits, but a narrow range with only 5 exponent bits. The maximum representable value is about 65,504, so if gradients or activations exceed this value during training, they become infinite (overflow).

Therefore, FP16 training requires a cumbersome technique called **loss scaling**.

This is a troublesome technique where a large number is multiplied by the loss to bring gradients into a representable range, then divided back. If the scale value is chosen incorrectly, training can fail.

BF16, conversely, sacrifices precision by reducing mantissa to 7 bits but maintains 8 exponent bits, giving it the same range as FP32. Precision is worse, but there's no overflow, so loss scaling isn't needed.

> In conclusion, range (dynamic range) is more important than precision in deep learning. Neural networks can compensate for slightly inaccurate values through training, but divergence to infinity is unrecoverable. This principle is carried over to the design of microscaling formats discussed later.

### Microscaling - The Idea That Enabled 4-bit Usage

4 bits alone cannot represent anything.

FP4 E2M1 has 2 exponent bits and 1 mantissa bit, meaning it can only represent a total of 16 values, including the sign.

```
Values representable by E2M1:
0, ±0.5, ±1, ±1.5, ±2, ±3, ±4, ±6
```

It's impossible to represent neural network weights with these 16 values. Weights in some layers might be clustered around 0.0001, while others are around 10. The same 16 values cannot accommodate both.

**Give separate scales to small groups?**
-> The idea of microscaling is simple.

Values are grouped into small blocks, and each block shares a single scale factor. Actual value = 4-bit value x block scale.

```
Block (16 values)
[3, -2, 1, 4, ...]  ← Each stored in 4 bits
        × 0.0037    ← Block scale (stored separately)
= [0.0111, -0.0074, 0.0037, 0.0148, ...]  ← Reconstructed actual values
```

Neural network weights locally have similar magnitudes, so it's rare for 16 adjacent weights to differ by 1000x. By scaling at the block level, even 16 steps of 4 bits can approximate the distribution within that block quite well.

This means that while it can't handle divergence to infinity, it can cover insufficient expressiveness.

#### Differences Between MXFP4 and NVFP4

The idea is the same, but the **block size and scale format** differ. These two factors determine accuracy and overhead.

1.  **MXFP4 (OCP Standard)**: Element format is FP4 (E2M1). Block size is 32. Block scale is FP8 E8M0. There is no global scale, and scale overhead is low.
2.  **NVFP4 (NVIDIA)**: Element format is also FP4 (E2M1). Block size is 16. Block scale is FP8 E4M3. An additional FP32 global scale is added. Scale overhead is 2x.

Let's look at the difference between the two. First, block size is 32 vs 16. Smaller blocks mean values within them are more similar, so scaling works better. The probability of an outlier ruining the scale for the entire block also decreases.

The second difference is E8M0 vs E4M3. `E8M0` has 8 exponent bits and 0 mantissa bits, meaning it can only represent powers of 2. Scales like 0.5, 1, 2, 4, 8... are possible. `E4M3`, on the other hand, has 3 mantissa bits, allowing for intermediate values like 1.5x, 2.5x.

```
Suppose the maximum value in a block is 3.7.

E8M0 Scale: Can only use 4 (2^2)     → 3.7/4 = 0.925, only 92.5% of range used
E4M3 Scale: Can use 3.75      → 3.7/3.75 = 0.987, almost fully utilized
```

Most weight distributions do not require the extreme dynamic range of **E8M0**. Instead, a denser scale within its operating range is more beneficial. Therefore, even with the same block size, NVFP4 is better.

Next is the two-stage scaling difference. NVFP4 uses an **additional FP32 global scale for the entire tensor** on top of the block scale E4M3. This structure first matches the overall tensor magnitude, then uses block scaling for local adjustments.

#### How Big is the Difference Between the Two?

A study on FP4 training comparing scale formats across a 350M Llama-style model yielded clear results.

With a fixed block size of 16, E1M6 **completely diverged**, while E3M4 and E4M3 performed best. When comparing block sizes of 8, 16, 32, 64, 128, smaller was better, but below 16, there were diminishing returns.

Thus, NVFP4's choice of block 16 and E4M3 is experimentally validated. The general trend is summarized below:

-   In small block conditions, NVFP4 outperforms INT4 and MXFP4 in both accuracy and stability.
-   However, if MXFP4 uses calibration techniques, the gap narrows significantly. If block-wise Hadamard rotation is applied before quantization (like MR-GPTQ), outliers are distributed across all channels within a block, mitigating the worst-case scenario for 32 blocks.
-   Mixed precision, where only a few sensitive layers or outlier channels remain in FP8/BF16, is common. Cases reporting over 30% weight memory reduction with less than 1% accuracy loss using this method exist.

#### Hardware and Toolchain Bundling

The choice of format can determine which hardware it can run on.

-   NVFP4 is natively processed by NVIDIA Blackwell's 5th-generation Tensor Cores. Element grouping, dynamic scaling, and 4-bit matrix operations are automatically handled in hardware.
-   MXFP4 is an OCP standard, also natively supported by AMD CDNA4 (MI350 series).
-   On unsupported hardware, software emulation can actually be slower.

When deploying checkpoints, this choice constrains the user's hardware. This is why the format in which open-weight models release quantized versions impacts the ecosystem.

### Two Paths for Precision to Lead to Performance

Let's explore further **where exactly reducing bits makes things faster.**

#### Reduced Memory Movement

```
BF16 Weights 70B: 140 GB
FP8  Weights 70B:  70 GB   → Half the read time
FP4  Weights 70B:  35 GB   → Quarter the read time
```

In **memory-bound sections (decode)**, this path is almost everything. Compute remains the same, but data movement is reduced, making it faster by that amount.

#### Increased Tensor Core Throughput

Lower precision generally allows Tensor Cores to process more operations simultaneously. Typically, halving the bit width doubles throughput.

```
B200 (dense)
BF16:  2.25 PFLOPS
FP8 :  4.5  PFLOPS   (2x)
FP4 :  9    PFLOPS   (4x)
```

In **compute-bound sections (prefill, large batches)**, this optimization is observed.

In conclusion, based on GB200 measurements, switching from BF16 attention + FP8 MoE to FP8 attention + NVFP4 MoE results in prefill improvement from 18,471 to 26,156 tokens/sec and decode improvement from 9,087 to 13,386 tokens/sec.

**Prefill improvement of 1.42x** and **decode improvement of 1.47x** are results obtained due to increased Tensor Core throughput and reduced memory movement, respectively.

Lowering precision reduces memory movement and simultaneously increases compute capability.

One might expect the **point where it transitions from memory-bound to compute-bound** to shift, but it actually doesn't. Why?

<br>

## Roofline Model

The concepts we've learned so far were preparation. In this chapter, those concepts will be combined into a single, calculable model.

Once you understand the Roofline model, you'll be able to determine **what a given operation is bottlenecked by** without a profiler.

### Arithmetic Intensity

**How to find the bottleneck:** Suppose a kernel is slow. Is the cause insufficient computation or inability to fetch data? Without making this determination, you cannot set the direction for optimization. If it's a compute problem, optimizing memory will have no effect.

**How many computations per byte?** We need to define Arithmetic Intensity (AI).

```
Number of operations performed (FLOPs)
Arithmetic Intensity =  ─────────────────────────────
                        Amount of data moved (Bytes)

Unit: FLOP/Byte
```

> A simple analogy: imagine carrying ingredients from a pantry to a kitchen and then cooking. Arithmetic intensity is how many servings you make with one trip's worth of ingredients. If AI is low, you make one bite per ingredient, and the chef is idle. If AI is high, the porter is idle.

**e.g., Vector Addition**

```
C[i] = A[i] + B[i]   (FP32)

Operations: 1 FLOP
Movement: Read A 4 bytes + Read B 4 bytes + Write C 4 bytes = 12 bytes
AI = 1/12 ≈ 0.083 FLOP/Byte     ← Extremely memory-bound
```

**GEMV - Core Operation for Decode**

Batch B, weight matrix, P parameters, b bytes/parameter.

```
Operations: 2 × P × B FLOP   (1 multiply + 1 add)
Movement: P × b bytes     (Weights dominate. Activations are negligible)

AI = 2PB / (Pb) = 2B / b

BF16 (b=2): AI = B
FP8  (b=1): AI = 2B
FP4  (b=0.5): AI = 4B
```

BF16 decode with batch 1 has AI = 1 FLOP/Byte.

**GEMM - Core Operation for Prefill**

M x K, K x N matrix multiplication.

```
Operations: 2 × M × N × K FLOP
Movement: (MK + KN + MN) × b bytes

M = N = K = 4096, BF16
Operations = 2 × 4096³ ≈ 137 GFLOP
Movement = 3 × 4096² × 2 ≈ 100 MB
AI ≈ 1,370 FLOP/Byte      ← Strongly compute-bound
```

**The same matrix multiplication, but GEMV is 1 and GEMM is 1,370.** This 1,000x difference distinguishes the nature of prefill and decode.

In GEMM, one weight is read and reused N times. In GEMV (N=1), it's used once and discarded. Reuse count directly corresponds to arithmetic intensity.

> GEMM is General Matrix-Matrix multiplication (2D matrices), GEMV is General Matrix-Vector Multiplication (1D vector). GEMM is used for prefill, GEMV for decoding.

**Attention is an exception**, and this is the most important part.

Looking at the attention operation in the decode phase, with sequence length S and batch B:

```
Operations: Each request performs dot product with its S KV cache entries → 2 × S × d × B FLOP
Movement: Each request's KV cache must be read → S × d × b × B bytes
                                        ↑ Increases proportionally with batch!

AI = 2Sd·B / (Sd·b·B) = 2/b      ← B cancels out
```

**No matter how much you increase the batch size, the arithmetic intensity of attention does not increase.** The reason is clear: weights are shared by all requests, but KV is different for each request. When batch size increases, weights are reused, but KV cache also increases. This is because reuse is not possible.

```
Batch Size →     4      16      64     128
FFN (GEMV) AI:   4      16      64     128    ← Increases ✅ Weights
Attention AI:      1       1       1       1    ← Stays the same ❌
```

This fact can explain many things:
-   **Why attention is a bottleneck even in large batches:** FFN transitions to compute-bound, but attention remains memory-bound.
-   **Why MLA and GQA are important:** Reducing KV cache is the only way to reduce attention's data movement.
-   **Why KV cache is effective for quantization:** Reducing `b` increases AI.
-   **Why prefill and decode are separated:** The two stages are bottlenecked by different resources, so optimal settings differ.
-   **Why B300 specifically emphasizes 2x attention performance:** This reflects in hardware design that attention remains a bottleneck.

### Drawing the Roofline

> The Roofline model is a performance analysis model that visually shows computer architecture AI performance analysis.

No kernel can exceed two limits simultaneously:

```
1. Compute Ceiling: Hardware's maximum FLOPS
2. Memory Ceiling: Bandwidth x Arithmetic Intensity
```

Let's confirm why the memory ceiling is like that. If bandwidth is 3,350 GB/s and AI is 2 FLOP/Byte, then it fetches 3,350 GB/s and computes 2 times per byte, so the maximum is 6,700 GFLOPS.

Thus, achievable performance cannot exceed bandwidth x AI.

```
Performance (FLOPS)
   ▲
   │           ┌─────────────────────  ① Compute Ceiling (horizontal)
   │          ╱
   │        ╱   ← ② Memory Ceiling (slope = bandwidth)
   │      ╱
   │    ╱
   │  ╱
   └────────┬──────────────────────→ Arithmetic Intensity (FLOP/Byte)
         Ridge Point
      ← Memory-bound │ Compute-bound →
```

The point where the two ceilings meet is called the ridge point.

```
Ridge Point = Max Compute Performance ÷ Memory Bandwidth   [FLOP/Byte]
```

This is a hardware property, not a workload property. All kernels on the same GPU share the same ridge point, and each kernel's position (left or right) is determined by its AI.

> Hardware is a physical device, workload is the amount of computation and resources consumed to process a specific task.

Ridge points of major GPUs:

```
A100  BF16:  312 TFLOPS / 2.04 TB/s ≈ 153 FLOP/Byte
H100  BF16:  989 TFLOPS / 3.35 TB/s ≈ 295
H200  BF16:  989 TFLOPS / 4.8  TB/s ≈ 206   ← Ridge lowers due to increased bandwidth only
B200  BF16: 2250 TFLOPS / 7.7  TB/s ≈ 292
B200  FP4 : 9000 TFLOPS / 7.7  TB/s ≈ 1169
MI355X BF16: 2500 TFLOPS / 8.0 TB/s ≈ 313
MI355X MXFP4:10100 TFLOPS / 8.0 TB/s ≈ 1263
```

**Compare H100 and H200.** Compute capability is the same, but bandwidth increased, lowering the ridge point from 295 to 206.

A lower ridge means **the compute ceiling is reached at a lower arithmetic intensity**, which is advantageous for memory-bound workloads. This is why H200 feels significantly faster than H100 in decode-centric serving.

> The general principle here is that high-arithmetic-intensity workloads like prefill/training desire machines with a high compute ceiling. Low-arithmetic-intensity workloads like decode desire machines with a lower ridge point, meaning relatively higher bandwidth. Higher total FLOPS does not necessarily mean faster decoding.

### Critical Batch Size

**What batch size should be set?**

In previous serving notes, we discussed that increasing batch size improves throughput but worsens latency.

However, throughput improvement is not infinite. Beyond a certain point, it becomes compute-bound, and benefits sharply diminish.

We need to be able to calculate that point.

#### Deriving the Equation for the Turning Point

The arithmetic intensity of decode FFN was `AI = 2B/b`. The batch size where this equals the ridge point is the turning point.

```
2B_crit / b = Ridge Point
B_crit = Ridge Point × b / 2
```

For H100 BF16:

```
B_crit = 295 × 2 / 2 = 295
```

Around batch 295, FFN transitions from memory-bound to compute-bound.

**Quantization does not change the critical batch size.**

Calculating for FP8 on the same H100:

```
FP8 Ridge Point = 1,979 TFLOPS / 3.35 TB/s ≈ 591
b = 1

B_crit = 591 × 1 / 2 = 295      ← Same as BF16.
```

If bit width is halved, data movement bytes halve, doubling AI. Simultaneously, Tensor Core throughput also doubles, doubling the ridge point. These two effects exactly cancel out, so batch size doesn't increase.

```
Quantization Effect Summary

What changes:   Absolute speed at the same batch size (2x, 4x faster) ✅
             Memory usage (half, quarter) ✅
What doesn't change: Memory-bound → Compute-bound transition point ❌
```

The idea that "now that I've switched to FP4, I can increase the batch size" is **incorrect from a computational perspective.** However, it is true that remaining memory allows for larger batches.

These two effects must be distinguished. More precisely, quantization frees up KV cache space, allowing for a higher batch limit, but it does not shift the transition point for computational efficiency itself.

Critical batch sizes per hardware:

```
A100:  approx. 153
H100:  approx. 295
H200:  approx. 206
B200:  approx. 292
MI355X: approx. 313
```

-   **If concurrent requests are much smaller than the critical batch size ->** Memory-bound region. Quantization and speculative decoding work well. Increasing batch size further is almost free.
-   **If concurrent requests are near or above the critical batch size? ->** Transitions to compute-bound. Speculative decoding benefits disappear. There's a recommendation to turn off EAGLE-3 if batch exceeds 32. The theoretical critical value is lower because speculative decoding verifies multiple tokens per step, effectively inflating the batch size by several times. With batch 32 and 5-token speculation, the effective batch is 160. Add attention bottleneck and draft model cost, and it's the worst case.

### Optimization Roofline

Let's summarize what each optimization technique does on the graph.

```
Performance
  ▲
  │              ┌────────── Compute Ceiling
  │         ╱────┘
  │    ╱───┘
  │ ╱
  └──────────────────────→ AI
```

| Technique | Movement on Graph | When Effective |
|------|------------------|-------------|
| Increase Batch | Move point to the right (AI increases) | Only when to the left of the ridge |
| Quantization | Move point to the right + Ceiling up | Both. But doesn't shift transition point |
| FlashAttention (Tiling) | Move point to the right (reduced data movement) | When memory-bound |
| KV Cache Compression (MLA/GQA) | Move attention point to the right | During attention phase |
| Speculative Decoding | Move point to the right (read once, multiple tokens) | When there's ample room to the left of the ridge |
| Faster Kernel (TMA, WGMMA) | Move point up towards the ceiling | When far from the ceiling |
| Occupancy Improvement | Move point up | When latency is exposed |
| Better GPU | Raise the ceiling itself | Only when already hitting the ceiling |

If you're far below the ceiling, there's no need to change the GPU; you should think about optimization.

It's a misconception that buying more GPUs solves the problem. Often, there's plenty of room on the existing GPU that's just not being utilized well.

Always follow these steps:

```
1. Calculate the dominant AI of the workload
        ↓
2. Compare with the hardware's ridge point
        ↓
   AI < Ridge → Memory-bound
        → Reduce data movement (quantization, tiling, cache reuse)
        → More computation is "free" (speculative decoding)
   AI > Ridge → Compute-bound
        → Check if Tensor Cores are being used correctly
        → Consider lower precision formats
        ↓
3. Check what percentage of the ceiling the actual performance reaches
        ↓
   70% or more → This direction is almost optimized. Look for other bottlenecks.
   Less than 30% → Suspect insufficient parallelism (occupancy), failed coalescing, bank conflicts.
```

### Roofline Limitations

This model is powerful but simplified. It ignores the following:

1.  **Cache Effects:** Data might actually remain in L2 and be reused, so actual data movement might be less than calculated. This is why measured performance can sometimes be better than theoretical predictions.
2.  **Latency and Parallelism:** The Roofline model assumes sufficient parallelism to completely hide latency. If occupancy is low, neither ceiling can be reached. In this case, the point on the Roofline is far below the ceiling, and the cause is parallelism, not AI.
3.  **Instruction Mix:** The effective ceiling differs depending on whether dedicated units (Tensor Cores) or general units are used. Understand the difference between WGMMA (95% of theoretical) and backward-compatible `mma` (62.9%).
4.  **Kernel Execution Overhead:** If there are many small kernels, launch overhead dominates. This is why CUDA graphs are needed.

> Therefore, the Roofline model is a tool for deciding where to look, not an accurate prediction tool. The correct sequence is to use calculations to guide direction and then verify with a profiler.

> Ridge Point: In the Roofline model, a performance analysis model, it's the turning point (vertex) where the hardware's maximum compute performance (Peak FLOPS) and maximum memory bandwidth (Peak Memory Bandwidth) limits intersect. Peak FLOPS / Peak Memory Bandwidth.

<br>

## 2026 Hardware Map

Apply the tools from the previous chapter to actual hardware. The numbers in this chapter will quickly become outdated, so **focus on how the axes moved rather than the numbers themselves.**

### Generational Summary Table

### 6.1 Generational Summary Table

| Item | H100 SXM | H200 SXM | B200 | B300 (Ultra) | Rubin VR200 | MI355X |
|---|---|---|---|---|---|---|
| Architecture | Hopper | Hopper | Blackwell | Blackwell Ultra | Rubin | CDNA 4 |
| Compute Capability | sm_90 | sm_90 | sm_100 | sm_103 | — | gfx950 |
| Memory | 80 GB HBM3 | 141 GB HBM3e | 180~192 GB HBM3e | 288 GB HBM3e | 288 GB HBM4 | 288 GB HBM3e |
| Bandwidth | 3.35 TB/s | 4.8 TB/s | 7.7~8 TB/s | 8 TB/s | approx. 20 TB/s | 8 TB/s |
| BF16 dense | 989 TF | 989 TF | 2.25 PF | — | — | 2.5 PF |
| FP8 dense | 1.98 PF | 1.98 PF | 4.5 PF | — | — | 5.0 PF |
| FP4 dense | Not supported | Not supported | 9 PF | 15 PF | 50 PF | 10.1 PF |
| TDP | 700 W | 700 W | 1,000 W | 1,400 W | — | 1,400 W |
| Scale-up Link | NVLink 4 (900 GB/s) | Same | NVLink 5 (1.8 TB/s) | Same | NVLink 6 | Infinity Fabric 4 |
| Status (Aug 2026) | Mainstay | Mainstay | Mainstay | Mainstay | H2 Mass Production Start | Shipping |

> PF and TF are units of AI compute performance. TF stands for TeraFLOPS (1 trillion operations per second), and PF stands for PetaFLOPS (1 quadrillion operations per second).

> Values are based on vendor public data and may vary depending on SKU and cooling method. In particular, B200 180GB (based on SXM6 measurements) and 192GB are used interchangeably.

### Changed Values by Generation

#### Hopper (H100 -> H200) - Bandwidth-only Refresh

H200 has the same compute units as H100. Only the memory changed from HBM3 to HBM3e, increasing capacity from 80 to 141GB and bandwidth from 3.35 to 4.8TB/s.

**From a Roofline perspective**, the compute ceiling remains the same, but the slope of the memory ceiling steepens. The ridge point drops from 295 to 206.

This means that for training or prefill-centric workloads, the gain over H100 is not significant. However, for decode-centric serving, the gain is direct. Bandwidth increased by 43%, so it's faster by that much in the memory-bound region. Capacity increased by 76%, so there's more space for KV cache. However, this also means more computation will be needed, which is a trade-off with compute.

#### Blackwell (B200) - Three Changes

1.  **Dual-die structure:** Two dies, reaching the reticle limit, are connected. Total 208 billion transistors. The two dies are connected by **NV-HBI at 10TB/s, appearing as a single GPU to software.**
2.  **Native FP4 support:** 5th-generation Tensor Cores and 2nd-generation Transformer Engine. Hardware directly handles NVFP4 microscaling as seen in the previous chapter.
3.  **tcgen05 programming model:** TMEM, single-thread issuance, CTA pair.

> Voxbi tcgen05 is a PTX instruction set programming model dedicated to Tensor Cores introduced in the Blackwell architecture. TMEM (Tensor Memory) directly records and manages computation results in a Tensor Core-dedicated hardware cache memory instead of registers. Single-thread issuance means that unlike previous generation instructions like mma.sync and wgmma, Tensor Core matrix operation instructions can be issued at a single-thread level. CTA pair means that two CTAs (thread blocks) in the same SM can collaborate to execute Tensor Core instructions and share data, a parallel computing feature.

**From a Roofline perspective, BF16** has a ridge of 292, almost identical to H100 (295). This is because both compute and bandwidth increased by similar factors. If FP4 is used, the ridge soars to 1169. This means FP4 significantly increases compute capability, but to fully utilize that capability, arithmetic intensity must also be high.

#### Blackwell Ultra (B300) - Re-tuned for Inference

B300 changed direction from B200.

-   Memory: 12-high stack, 288GB (50% increase over B200).
-   FP4 dense: 15 PFLOPS (1.5x over B200).
-   **Intentionally lowered FP64 performance.**
-   Separately enhanced attention compute capability.

**Designed with an inference bias, as a memory-intensive processor.** It sacrificed FP64 to concentrate silicon on low-precision efficiency, attention, and memory bandwidth.

**Why attention was separately enhanced:** As seen in the previous chapter, attention's arithmetic intensity does not increase with batch size, remaining a bottleneck. This is a case where hardware designers directly incorporated Roofline analysis results.

The GB300 NVL72 rack features 72 cards with 20.7TB of HBM3e, NVLink domain bandwidth of 130TB/s, FP4 1.1 ExaFLOPS, and power consumption of approximately 120kW.

#### Rubin (VR200)

The first samples were shipped in February 2026, with mass production units scheduled to go to cloud providers in the second half of the year.

-   One Rubin GPU: 50 PFLOPS FP4 (3.3x of B300).
-   **HBM4 288GB, target bandwidth 22 TB/s (actual initial shipments reported to be around 20TB/s).**
-   Two reticle-sized dies + 8 HBM4 stacks.
-   Vera CPU (88 Armv9.2 Olympus cores) replacing Grace.
-   NVLink 6, rack interconnect 260 TB/s.
-   Rack power approx. 190-230 kW (significant increase from Blackwell's 120-130 kW).

From a Roofline perspective, the FP4 ridge point is 50 PFLOPS = 50,000 TFLOPS, 20 TB/s = 20,000 GB/s, so the ridge is 2,500 FLOP/Byte. **This is more than double B200's FP4 ridge of 1,169.**

This implies that **compute capability has increased faster than bandwidth.** For memory-bound workloads, it becomes difficult to fully utilize the increased theoretical FLOPS. Decode performance will primarily improve by the bandwidth increase (8 -> 20 TB/s, 2.5x), while the 3.3x FP4 FLOPS will be used for prefill and large batches.

> Supply Reality - HBM4 supply for 2026 is virtually exhausted, and significant new production capacity is expected in 2027. From a power perspective, 190-230kW requires 800VDC power delivery and full liquid cooling. The hardware roadmap and actual procurement feasibility are different issues, and Blackwell is likely to remain the mainstay in 2026-2027.

#### AMD CDNA 4 (MI355X)

-   256 Compute Units, 1,024 Matrix Cores (AMD calls them Matrix Cores instead of Tensor Cores).
-   288GB HBM3e, 8 TB/s 8,192-bit interface.
-   64MB Infinity Cache (equivalent to L3, larger than NVIDIA's L2).
-   Native MXFP4/MXFP6/MXFP8 support, part of the OCP standard, not NVFP4.
-   8 Compute Dies + Infinity Fabric 4.

**Positioning:** Compared to B200, it leads in memory capacity (288 vs 196GB). Models exceeding 192GB (roughly 110B or more) can fit on one MI335X card, while B200 would require two or more. Fewer shards reduce communication overhead.

A point of caution: AMD's claims of 35x inference improvement for MI300X vs MI355X compare **MI300X's FP8 as a baseline to MI355X's FP4**. It's not an FP4 vs FP4 comparison. Always check the baseline when reading vendor figures.

Warp size is 64, while NVIDIA kernels are designed assuming 32. So, they don't port directly. Even if the ROCm ecosystem is mature, kernel-level work is still needed.

### Hardware Selection Criteria - Using the Roofline Model

This is the process of determining which hardware is better for a given workload based on the spec sheet.

```
Step 1 — Calculate the dominant arithmetic intensity of the workload
        Decode-centric?     → Low AI (1 to tens)
        Prefill/training-centric? → High AI (hundreds to thousands)
        Mixed?            → Weighted average by token ratio

Step 2 — Calculate the ridge point of the candidate GPU
        Ridge = Peak FLOPS for that precision ÷ Bandwidth

Step 3 — Decision
        AI ≪ Ridge  → Bandwidth determines performance. Compare price per bandwidth.
        AI ≫ Ridge  → Compute determines performance. Compare price per FLOPS.

Step 4 — Check capacity constraints
        Do model weights + KV cache for target concurrent users fit?
        (Use the KV cache formula from previous serving notes for this calculation)

Step 5 — Check software support
        Does the engine you plan to use take the optimal kernel path on that hardware?
        (Is it SM100 or SM120? How mature is ROCm support?)
```

Always check Step 5. Even if theoretical performance is good, you can't use that performance if there's no kernel for it.

<br>
