# PCIe Lane Structure, GPUDirect Storage, NUMA Optimization

Let's start with a question. "When pushing terabytes of AI training data stored in a Linux file system to GPU VRAM, why must this data necessarily pass through the host CPU's main memory RAM? Can't I/O devices directly exchange data without the CPU's permission?"

Within a single server node, physical communication between storage, NICs, and GPUs occurs via the **motherboard's PCIe (Peripheral Component Interconnect Express)** bus.

- **PCIe Lanes and Topology**: This is a serial bus standard that connects all high-speed devices on the motherboard. GPUs typically connect to the CPU using 16 lanes (x16) and have a bidirectional bandwidth of 64GB/s based on PCIe Gen4.
- **NUMA (Non-Uniform Memory Access)**: In multi-socket (2 or more CPUs) servers, each CPU has its own local RAM bank and PCIe slots. If a specific CPU needs to access memory or a GPU connected to another CPU, it must traverse an inter-socket interconnect (UPI/QPI) bus, leading to a sharp increase in latency.
- **GPUDirect Storage (GDS)**: This is an NVIDIA hardware technology that creates a shortcut in the data path, allowing data from storage (NVMe SSD) to go directly to the GPU's VRAM via a PCIe switch, bypassing the CPU's system memory RAM.


<br>

## Physical/Architectural Bottlenecks

When transferring large datasets to the GPU using traditional POSIX I/O structures, three critical bottlenecks occur at the system level, often in combination.

- **Bounce Buffer Copy Bottleneck**: When the Linux kernel reads data from NVMe, it first stores it in the kernel's page cache, then performs a `memcpy` to copy it to a user-space buffer. Subsequently, the CUDA driver copies this data again to GPU VRAM. This process causes a single piece of data to traverse the PCIe bus twice (Storage -> RAM -> GPU), effectively halving PCIe bandwidth and wasting host CPU cycles.
- **GPU Starvation (Data Under-supply)**: Due to the overhead of the CPU copying data and managing virtual memory mappings, it cannot keep up with the GPU's computational speed. This leads to a situation where a GPU, capable of processing thousands of images per second, stops computation and idles, waiting for data to arrive.
- **NUMA Cross-Socket Penalty**: If an NVMe SSD is connected to CPU socket 0 and a GPU is connected to CPU socket 1, data, after being loaded into host RAM, must traverse the narrow inter-CPU communication bus (UPI). In this scenario, bandwidth drops by 30-50%, and latency skyrockets.

### Solution

To eliminate these OS layer and hardware routing inefficiencies, GPUDirect (P2P DMA) technology was introduced.

- **Peer-to-Peer (P2P) DMA (Direct Memory Access)**: Originally, DMA is a technology that allows devices to access RAM without CPU intervention. P2P DMA goes a step further, excluding even host RAM, and enables two devices (NVMe, GPU) connected to the PCIe bus to directly read and write data to each other's memory addresses (BAR, Base Address Register).
- **Kernel VFS Bypass (cuFile API)**: NVIDIA provides a dedicated API called `cuFile` instead of the traditional file reading system call (`read()`). This bypasses the complex file system layers of the Linux kernel and directly issues hardware I/O commands to the NVMe controller.

### Hardware Datapath Memory Layer Operation Principle

This is the exact path data takes at the motherboard circuit level when GPUDirect Storage is enabled.

1. **Memory Address Mapping:** The user application (pytorch) exposes the physical address of the GPU VRAM where data will be received to the PCIe space (BAR1) and notifies the NVMe controller of this address.
2. **I/O Command Issuance**: The host CPU does not participate in data copying at all. It merely issues a control command to the NVMe controller's Submission Queue, instructing it to send specific SSD block data to the previously provided GPU BAR address, and then moves on to other thread operations.
3. **PCIe Switch Routing of Data**: The NVMe device pushes data onto the PCIe lanes. This electrical packet does not travel up to the CPU socket (Root Complex). The **PCIe switch chipset**, located at the bottom of the motherboard, inspects the packet's destination address. It determines, "Oh? The destination isn't system RAM, it's a GPU," and within the switch, it redirects the packet directly towards the GPU slot.
4. **VRAM Settlement**: The packet is stored in HBM via the GPU's internal memory controller. Since the bounce buffer is completely eliminated, bandwidth waste becomes zero, and latency is reduced to hardware limits.

<br>

## System Low-Level Metric Profiling

Let's look at an example of diagnosing the physical wiring topology between multiple GPUs, NUMA nodes, NICs, and storage when connected to a bare-metal server or instance.

```bash
root@ai-server:~# nvidia-smi topo -m

        GPU0    GPU1    NIC0    CPU Affinity    NUMA Affinity
GPU0     X      PIX     NODE    0-15,32-47      0
GPU1    PIX      X      NODE    0-15,32-47      0
NIC0    NODE    NODE     X      16-31,48-63     1

Legend:
  X    = Self
  SYS  = Connection traversing PCIe as well as the SMP interconnect between NUMA nodes (e.g., QPI/UPI)
  NODE = Connection traversing PCIe as well as the interconnect between PCIe Host Bridges within a NUMA node
  PHB  = Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)
  PIX  = Connection traversing up to a maximum of 1 PCIe bridge (PCIe Switch)
```

- The relationship between `GPU0` and `GPU1` (`PIX`): This means the two GPUs are connected via the same PCIe switch (PIX), allowing them to communicate peer-to-peer without involving the CPU, resulting in fast communication speeds.
- The relationship between `GPU0` and `NIC0` (network card) (`NODE`): They are bound to different NUMA nodes (GPU to NUMA 0, NIC to NUMA 1). For data to move between them, it must traverse the `SYS/NODE` bus, i.e., the inter-host CPU bus, leading to a critical NUMA cross-socket penalty. This results in the worst performance when using GPUDirect RDMA.

Additionally, when running training processes in a backend infrastructure,
there's a tuning method that involves forcing resource allocation at the operating system level, considering the hardware topology.
This is called topology-aware process binding (Affinity Pinning) using `numactl`.
As observed in the topology log above, when using a specific GPU, the OS scheduler should be pinned to use only the CPU cores and system RAM physically connected to the same NUMA node as that GPU.

```bash
# Incorrect execution (OS arbitrarily schedules threads to NUMA 0 and 1, causing bottlenecks)
python train.py --gpu 0

# Optimized execution: 
# Since GPU 0 is bound to NUMA 0, 
# --cpunodebind=0 (force CPU cores to use only node 0)
# --membind=0 (force memory banks to use only node 0) options are applied to completely prevent cross-socket penalties.
numactl --cpunodebind=0 --membind=0 python train.py --gpu 0
```

**Monitoring GDS activation is also possible.**

Additionally, NVIDIA's `gdsio` benchmark tool can be used to verify whether storage I/O actually bypasses the CPU bounce buffer.
When the kernel's `O_DIRECT` flag and `cuFile` are properly engaged, system monitoring metrics will show CPU utilization dropping below 5% while I/O throughput skyrockets to GB/s.
