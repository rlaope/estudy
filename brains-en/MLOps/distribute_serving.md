# Distributed Serving and Communication

### Learning Objectives

- Calculate what collective communication operations actually send and receive, derive the transfer volume of all-reduce and all-to-all from tensor size and number of GPUs, and convert that value into its impact on latency.
- Read `nvidia-smi topo -m` output to determine which GPU pairs are connected via fast paths, and explore how this can be used to set parallelization placement values.
- Understand concepts like what NCCL chose between ring and tree algorithms, and why.
- Understand why inter-node KV transfer is a separate problem and select a transfer backend.
- Explain, in relation to transfer volume calculations, why EP (Expert Parallelism) should be confined within a node.

<br>

## Bandwidth Cliff

### Bandwidth Hierarchy and Node Boundaries

When moving data from a GPU, bandwidth drops stepwise as the destination gets further away.

```
                        Bandwidth            Notes
GPU 내부 HBM           3,350~8,000 GB/s   Self-memory
다이 간 NV-HBI         10,000 GB/s        Blackwell dual-die
NVLink 5               1,800 GB/s         Between GPUs within the same node
PCIe 5.0 x16              64 GB/s         CPU ↔ GPU
InfiniBand NDR            50 GB/s         Inter-node (400Gb/s port)
이더넷 100GbE             12.5 GB/s       Inter-node
```

Looking only at data transfer paths between GPUs, within a node it's 1,800 GB/s, while outside a node it's 50 GB/s, a 36x difference. Even PCIe is around 64 GB/s, so the moment you cross node boundaries, any path becomes slow.

If you calculate the ratios between adjacent paths, you can see a significant gap at the node boundary.

```
HBM → NVLink         2~4x
NVLink → PCIe        28x      ← Node boundary
PCIe → InfiniBand    1.3x
```

### 36x Latency Increase at Node Boundary

Let's take tensor parallelism as an example. Suppose an all-reduce occurs once per layer, handling a 100MB tensor across 8 GPUs.

Although the transfer volume will be derived later, using the result first, each GPU will send and receive 175MB.

```
NVLink 5 (1,800 GB/s)      175 MB ÷ 1,800 GB/s = 0.097 ms
InfiniBand NDR (50 GB/s)   175 MB ÷    50 GB/s = 3.5 ms
```

For an 80-layer model, this repeats 80 times per token.

```
NVLink path     0.097 × 80 =  7.8 ms
InfiniBand path 3.5   × 80 = 280 ms
```

Spending 280ms on communication when a single decode step takes a few milliseconds? In such a scenario, computation becomes meaningless, and this highlights why the **rule of confining tensor parallelism within a node** is so crucial.

### Guideline - Parallelization Placement Based on Communication Frequency

In any case, the core idea is to keep parallelization with frequent communication within a node.

```
Frequent communication    TP    all-reduce per layer      → Inside NVLink
                          EP    all-to-all per MoE layer   → Inside NVLink if possible

Infrequent communication    PP    Once at segment boundaries         → Can cross node boundaries
                          DP    Attention layers have no communication    → Can cross node boundaries
```

In an 8-GPU server, NVLink wiring ends within that chassis, so a 9th GPU would be in a different chassis, requiring InfiniBand to reach it.

Rack-scale configurations like GB200 NVL72 extend this range, connecting all 72 GPUs within a rack via NVSwitch, allowing any pair of the 72 GPUs to communicate directly via NVLink.

```
Standard 8-GPU server    NVLink-connected range  8 GPUs     InfiniBand from the 9th GPU
NVL72 rack         NVLink-connected range  72 GPUs    InfiniBand from the 73rd GPU
```

<br>

## Collective Communication

### Definition of Collective Communication

It is a communication where multiple GPUs **participate together to produce a single result**. Unlike point-to-point communication where one sends and one receives, all participants send and receive simultaneously.

The operations used differ for each parallelization type, as follows:

```
Operation            What it does                          Where it's used
all-reduce      Sums all values and ensures all have the same result   TP
all-gather      Gathers individual pieces so all possess the whole    TP column partitioning, DP
reduce-scatter  Sums all, then each keeps only its share        First half of all-reduce
all-to-all      Each sends different data to each other    EP
broadcast       Copies one GPU's data to all         Weight distribution
```

### all-reduce

Most frequently used in TP, where each GPU holds a partial sum, and all are summed to ensure everyone has the same final value.

```
Start
  GPU0: [a0]   GPU1: [a1]   GPU2: [a2]   GPU3: [a3]

End
  GPU0: [S]    GPU1: [S]    GPU2: [S]    GPU3: [S]
  S = a0 + a1 + a2 + a3
```

When a matrix is partitioned with TP, what each GPU computes is a partial sum, so this operation is required to complete it.

### all-to-all

Each GPU sends different data to every other GPU.

```
Start
  GPU0: [→G0][→G1][→G2][→G3]     Each cell for a different destination
  GPU1: [→G0][→G1][→G2][→G3]
  GPU2: [→G0][→G1][→G2][→G3]
  GPU3: [→G0][→G1][→G2][→G3]

End
  GPU0: [G0←][G1←][G2←][G3←]     Received from everyone
  ...
```

This pattern appears in MoE when sending tokens to their assigned experts. Since the router determines which token goes to which expert, the destinations vary.

### all-reduce vs all-to-all

TP uses all-reduce, and EP uses all-to-all. While the parallelization method dictates the choice, their differing natures mean different operational considerations.

Where all-to-all is better:

```
Transfer volume       all-reduce  Approx. 2S moved per GPU
                      all-to-all  Approx. S moved per GPU            Half

Scope of participation    all-reduce  All synchronize. If one rank is slow, all wait.
                      all-to-all  Only sender and receiver involved

What is moved    all-reduce  Computation result. All must have the same value.
                      all-to-all  Tokens. Only send to where needed.
```

In a structure using 8 out of 256 experts per token, if experts are partitioned and scattered across all GPUs, then combined with all-reduce, all 32 GPUs are involved in computation. If experts are assigned whole and tokens are sent via all-to-all, only the 8 GPUs holding those experts work, while others process different tokens. The MoE design, which aims to save computation by activating only a subset, is maintained thanks to this operation.

### Problem - Differences in Operation Characteristics

In terms of transfer volume alone, all-to-all is lighter. However, the points to consider here are:

```
Size prediction     all-reduce  Fixed tensor size. Can pre-allocate buffer and schedule.
                   all-to-all  Unknown how much goes to which GPU until router decides.

Symmetry        all-reduce  All send and receive the same amount.
                   all-to-all  Congestion on GPUs with popular experts. Slowest link determines overall performance.

Overlap with computation  all-reduce  Easy to overlap
                   all-to-all  Requires waiting for router output to start.
```

It's not due to high communication volume, but rather the challenge of unpredictability and imbalance that needs to be addressed.

### Solutions

1.  **Problem of unpredictable size:** Handled by specialized communication libraries. Implementations like DeepEP create kernels that initiate transfers as soon as router output is available, routing intra-server traffic via NVLink and inter-server traffic via RDMA to flow concurrently. Generic collective communication libraries cannot express this pattern, leading to separate implementations.
2.  **Problem of skewed load:** **Mitigated by re-calculating expert placement**. By observing routing statistics and **replicating popular experts across multiple GPUs**, tokens destined for them are split. SGLang's `--enable-eplb` provides this functionality.

```bash
--enable-eplb --eplb-algorithm deepseek --ep-num-redundant-experts 32
```

Expert slots increase from 256 to 288, increasing weight memory by 12.5%, but in return, bias is alleviated, reducing the waiting time for the slowest GPU.

3.  **Problem of difficulty overlapping with computation:** Circumvented by splitting batches. If a batch is divided into two, one part can communicate while the other computes, filling the waiting period for router output with computation from the other batch. This method can be used when batches are sufficiently large.

```
If not split   [Router][all-to-all wait][Expert computation][all-to-all wait]
If split          Batch A [Router][Comm.    ][Comp.    ][Comm.    ]
                 Batch B        [Router][Comm.    ][Comp.    ][Comm. ]
                              ↑ Fills each other's waiting periods
```

<br>

## Ring and Tree

### Bottleneck of Single GPU Centralized Approach

The simplest implementation of all-reduce is for all to send data to GPU 0, which then sums everything and broadcasts it back.

```
N GPUs, buffer size S

Amount received by GPU0   (N-1) × S
Amount sent by GPU0 (N-1) × S

N=8, S=100MB → GPU0 alone sends/receives 1.4GB
              The other 7 GPUs each send/receive 200MB
```

GPU 0's link would become a bottleneck, and increasing the number of GPUs would only worsen it (as N increases).

### Ring Algorithm

GPUs are arranged in a logical ring, data is split into N pieces, passed only to neighbors, and circulated twice.

```
Phase 1: reduce-scatter (N-1 times)
  Each GPU sends one piece to its right neighbor, accumulating.
  Upon completion, each GPU holds one complete piece of a different segment.

Phase 2: all-gather (N-1 times)
  Circulates the completed pieces once more to distribute to all.
```

In each phase, the amount sent by one GPU is one piece, i.e., S/N.

```
Total transfer volume = 2(N-1) phases × S/N = 2(N-1)/N × S

N=8,  S=100MB → 175 MB
N=32, S=100MB → 194 MB
```

Even as N increases, it converges to 2S. Unlike the single-GPU centralized approach of (N-1)S, it has become almost independent of the number of GPUs. This means that increasing the number of GPUs does not increase the bottleneck on a specific GPU.

### Ring Disadvantage with Small Messages

While the ring's transfer volume is almost independent of the number of GPUs, the number of phases still scales with the number of GPUs.

N-1 times for reduce-scatter, N-1 times for all-gather, totaling 2(N-1).

```
N=8   → 14 phases
N=32  → 62 phases
```

Each phase can only start after the previous one finishes. Since it's a structure where pieces received from a neighboring GPU are accumulated and then passed to the next neighbor, the order cannot be skipped, and thus a fixed overhead is incurred per phase.

```
Fixed overhead per phase
  Kernel execution preparation
  Synchronization with neighbors
  Link propagation delay
  → Totaling approx. 2μs for NVLink
```

If we divide the total time into two terms, it looks like this:

```
Total time = Phase overhead + Transfer cost
          = 2(N-1) × 2μs  +  2(N-1)/N × S ÷ 1,800 GB/
```

Let's try with N=32, varying the buffer size.

```
Buffer size S    Phase overhead    Transfer cost    Dominant factor
    64 KB      124 μs      0.07 μs     Phase overhead (1,800x)
     1 MB      124 μs      1.1  μs     Phase overhead (110x)
   100 MB      124 μs      108  μs     Similar
     1 GB      124 μs     1,078 μs     Transfer cost (8.7x)
```

**When the buffer is small, the time spent transitioning phases is significantly greater than the actual data transfer time.**

Transferring 64KB involves 62 synchronizations, leading to 99% of communication time being spent waiting.

### Tree Algorithm Solution

To reduce the number of phases, the neighbor-only passing structure must be abandoned. A tree algorithm arranges GPUs in a binary tree and moves data up and down.

```
Reduce phase (bottom-up)

        GPU0
       ╱    ╲
    GPU1    GPU2
    ╱  ╲    ╱  ╲
  G3   G4  G5   G6

  Depth 3 → All gather at GPU0 in 3 phases
  GPUs at the same depth proceed concurrently

Broadcast phase (top-down)
  3 phases in the reverse direction of the same tree
```

The depth of a binary tree is log₂N. While the ring passes through neighbors one by one, the tree reduces by half as it ascends.

```
Ring   Phases 2(N-1)      N=32 → 62 phases → 124 μs
Tree Phases 2log₂N      N=32 → 10 phases →  20 μs
```

In the previous table, for 64KB, the ring takes 124μs, while the tree finishes in 20μs.

### However, One Problem: Tree Cannot Fully Utilize Links

The tree is not always better. The ring utilizes all links because every GPU sends and receives simultaneously in each phase.

```
Ring    32 GPUs each send to their right neighbor     All 32 links active
Tree  At depth 1, only 2 links active        The rest wait
      At depth 2, 4 links; at depth 3, 8 links
```

Therefore, when the buffer size increases, transfer cost dominates, and in that range, the ring, which uses all links, is advantageous. NCCL recovers about half of this loss with a double-binary tree, but it cannot fully catch up to the ring.

```
Small buffer, phase overhead dominates -> Tree
Large buffer, transfer cost dominates -> Ring
```

NCCL automatically chooses based on message size; the boundary is approximately 256KiB, which is near the point where phase overhead and transfer cost swap dominance in the table above.

### Measured Bandwidth

Measured values for DGX H100 8 GPUs:

```
Ring (direct NVLink)        Approx. 700 GB/s
NVSwitch transitive partitioning utilization   Approx. 900 GB/s
```

The 700 GB/s for the ring is a value derived from a structure where adjacent GPUs communicate. An NVSwitch can connect all pairs simultaneously, but the ring cannot fully utilize this capability. Algorithms leveraging the switch can achieve up to 900 GB/s.

Measurements are done with nccl-tests.

```bash
$ ./build/all_reduce_perf -b 8 -e 1G -f 2 -g 8
#  size(B)  time(us)  algbw(GB/s)  busbw(GB/s)
   1048576    58.2      18.02        31.53
  16777216   238.4      70.38       123.16
 268435456  3021.7     121.36       212.38
```

- `-b 8`: Starting buffer size 8 bytes
- `-e 1G`: Ending buffer size 1GB
- `-f 2`: Measure by doubling the size
- `-g 8`: Use 8 GPUs

The difference between the two bandwidth columns should be noted.

```
algbw   Buffer size ÷ time. User-perceived value.
busbw   Actual data flowed through link ÷ time. Hardware utilization.
        For all-reduce, busbw = algbw × 2(N-1)/N
```

The value to compare with theoretical link speeds is `busbw`; even if `algbw` looks low, it's often normal.

<br>

## NCCL and Topology

NCCL (Nvidia Collective Communications Library) is a library that executes collective communication on GPUs. `init_process_group(backend='nccl')` in PyTorch uses this.

```
Topology detection   Investigates which GPUs are connected by which links
Algorithm selection   Chooses between ring/tree based on message size and topology
Kernel execution      Performs communication as a GPU kernel. Consumes some SMs.
```

The third point is significant in serving. Since communication consumes SMs, it contends for resources with computation. While computation doesn't completely stop during communication, both become slower.

### How to Check the Selected Algorithm

```bash
$ NCCL_DEBUG=INFO python train.py 2>&1 | grep -E "NCCL INFO (Ring|Trees|Channel|NET)"
NCCL INFO Channel 00/04 : 0 1 2 3 4 5 6 7
NCCL INFO Ring 00 : 7 -> 0 via NVLINK
NCCL INFO Trees [0] 1/-1/-1->0->-1
NCCL INFO NET/IB : Using [0]mlx5_0:1/RoCE
```

- `Ring 00 : 7 -> 0 via NVLINK`: Ring configuration and the link used by each hop. If it shows `via P2P` or `via SHM`, it means GPUs on the same server cannot use NVLink. This happens if cards are installed without NVLink bridges or if only a subset of GPUs are injected into a container, preventing ring formation.
- `Channel 00/04`: Number of parallel channels. More channels mean better link utilization.
- `NET/IB : Using [0]mlx5_0`: Inter-server path and NIC in use. If `NET/Socket` appears, it means InfiniBand was not found and it fell back to Ethernet.

### Persistent Environment Variable Settings

NCCL automatically selects based on topology, and the following variables are used for diagnosis:

- `NCCL_ALGO=Ring`: Forces the algorithm to ring.
- `NCCL_P2P_DISABLE=1`: Disables P2P, for problem isolation.
- `NCCL_IB_DISABLE=1`: Disables InfiniBand, falls back to Ethernet.
- `NCCL_DEBUG=INFO`: Prints logs. No performance impact.

Cases requiring persistent settings are twofold:

- `NCCL_SOCKET_IFNAME=eth0`: Specify when the server has multiple interfaces and NCCL picks a management interface.
- `NCCL_IB_HCA=mlx5_0,mlx5_1`: Explicitly specify when there are multiple NICs but only some are to be used.
