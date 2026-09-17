# How a Model Loads onto a GPU

**Let's explore when processes, drivers, containers, and serving engines interact.**

By looking at `nvidia-smi` output, we can determine what's happening, which processes are consuming how much memory, why that amount of memory is being used, and whether the GPU is active or idle.

When the GPU isn't visible inside a container, how can we pinpoint which layer is broken—is it the driver, the container toolkit, or the image?

Let's gain the ability to sequentially interpret serving engine startup logs, from weight loading to KV cache preemption and graph capture, to understand what each step does and where it might have stalled.

<br>

## Using a GPU

### GPU's 3-Layer Access Structure

In Linux, a GPU is a **device file**.

For a process to use a GPU means opening that file and pushing commands into it.

```
$ ls -l /dev/nvidia*
crw-rw-rw- 1 root root 195,   0  /dev/nvidia0        ← GPU 0
crw-rw-rw- 1 root root 195, 255  /dev/nvidiactl      ← For control
crw-rw-rw- 1 root root 234,   0  /dev/nvidia-uvm     ← Unified memory
crw-rw-rw- 1 root root 234,   1  /dev/nvidia-uvm-tools
```

Three layers are built on top of this file.

```
┌──────────────────────────────────────┐
│ Application    vLLM, SGLang, PyTorch  │
├──────────────────────────────────────┤
│ CUDA Runtime   libcudart.so           │  Distributed with the app
│               cuBLAS, cuDNN, etc.     │  Version tied to the app
├──────────────────────────────────────┤
│ Driver API     libcuda.so             │  Included with driver installation
│                                       │  Only one exists on the host
├──────────────────────────────────────┤
│ Kernel Driver   nvidia.ko              │  Kernel module
├──────────────────────────────────────┤
│ Device File    /dev/nvidia*            │
└──────────────────────────────────────┘
```

There's a boundary in the middle: `libcuda.so` belongs to the driver and only one exists on the host.

`libcudart.so` and the applications above it are portable.

### CUDA Context

When a process first attempts to use a GPU, the driver creates a **CUDA context**.

One context is created per process and per GPU. The information held by a context includes:

```
GPU virtual address space      Where pointers returned by cudaMalloc reside
Loaded kernel code             GPU functions this process will use
Streams and events             Asynchronous task queues
Memory allocation ledger       What was allocated and how much
```

**Isolation between processes is maintained.** This means process B cannot dereference a GPU pointer received by process A. If a process dies, its context disappears, and all GPU memory it held is reclaimed.

Context creation itself is costly, consuming hundreds of MB of GPU memory and taking hundreds of milliseconds. Therefore, a serving process creates it once during startup and continues to use it.

### When Multiple Processes Attach to One GPU

By default, multiple processes can create contexts on the same GPU. However, they **do not compute simultaneously.** The driver time-shares contexts by switching between them.

```
Default (Time-sharing)    Process A [compute] → Context switch → Process B [compute] → ...
                          Incurs switching overhead, each allocates memory separately

MPS                       Kernels from multiple processes run concurrently within a single context
                          No switching overhead. Weaker isolation

MIG                       Splits the GPU at a hardware level
                          Memory and compute units are physically isolated. Strong isolation
                          RTX PRO 6000 server edition supports up to 4 instances of 24GB
```

**For serving, typically none of these are used.** Instead, one serving process is assigned to one GPU, and that process batches requests internally, as batching is more efficient than context switching.

<br>

## Order of GPU Acquisition by a Process

### Startup Sequence

When a serving process starts, the following events occur in sequence:

```
1. Process start
   Load libcuda.so, open /dev/nvidiactl

2. Device enumeration
   Query how many devices are visible, and their compute capability and memory

3. Context creation
   Open /dev/nvidia0, secure GPU virtual address space
   → From this point, the process appears in nvidia-smi

4. Weight loading
   Disk → Host memory → GPU memory
   Read safetensors via mmap and copy layer by layer

5. Memory profiling
   Perform a forward pass with dummy input to measure peak activation usage

6. KV cache pool preemption
   (Total memory × utilization) − weights − activations = KV share
   Allocate that entire amount and redistribute internally in blocks

7. CUDA graph capture
   Pre-record kernel execution sequences for frequently used batch sizes

8. HTTP server start
   Bind socket, start accepting requests
```

### Reading Startup Logs

Key lines from vLLM startup logs:

```log
INFO  Loading model weights took 52.3021 GB
      → Step 4. Actual amount of weights consumed

INFO  Memory profiling results:
      total_gpu_memory=95.00GiB
      model_weights=52.30GiB
      non_torch_memory=0.85GiB      ← Context, kernel code, etc.
      PyTorch_activation_peak=3.12GiB
      gpu_memory_utilization=0.90
      → Step 5. Breakdown of what consumes how much

INFO  GPU KV cache size: 29,184 tokens
INFO  Maximum concurrency for 32,768 tokens per request: 0.89x
      → Step 6. How many tokens can be held with the remaining share
      → 0.89x is a warning that even one 32K request cannot be fully accommodated

INFO  Capturing CUDA graphs: 100%|██████| 35/35
      → Step 7. Recording for 35 types of batch sizes

INFO  Starting vLLM API server on http://0.0.0.0:8000
      → Step 8. Now accepting requests
```

If `Maximum concurrency` is less than 1.0, the configuration is incorrect. This value is **the KV cache share divided by the length required by one request**, indicating how many maximum-context requests this server can handle concurrently.

```
KV cache tokens ÷ max-model-len = Maximum concurrency

29,184 ÷ 32,768 = 0.89x
```

The state falls into three categories based on the value:

```
Less than 1.0   Cannot accommodate even one max-length request
                → Such requests will be rejected or truncated
                → Short requests might run for a while, but it will crash when a long request arrives

1.0 to 2.0      One request can be accommodated, but concurrent processing is virtually impossible
                → No batching, so the GPU is idle and throughput is low

4.0 or more     Multiple requests can be processed concurrently. Batching becomes meaningful
```

It's dangerous because the server can sometimes start even if it's less than 1.0. If most actual requests are 2K tokens, it will run without issues for a while, but then fail the first time a 30K request comes in.

This is a typical anti-pattern that goes unnoticed in development but surfaces in production.

You need to reduce the context length, apply quantization, or add more GPUs.

```
Increase the numerator   Quantize to reduce weights and secure KV share     Significant effect
                         Increase --gpu-memory-utilization                  Small effect
                         Use FP8 for KV cache itself                        Approx. 2x
                         Add GPUs for TP                                    Linear

Decrease the denominator --max-model-len                                   Check actual required length
```

### nvidia-smi

```
$ nvidia-smi
+-----------------------------------------------------------------------+
| NVIDIA-SMI 580.178.04    Driver Version: 580.178.04  CUDA Version: 13.0|
|-----------------------------------------+----------------------------+
| GPU  Name              Persistence-M    | Bus-Id        Disp.A       |
| Fan  Temp  Perf  Pwr:Usage/Cap          |         Memory-Usage       |
|                                         |          GPU-Util  Compute M|
|=========================================+============================|
|   0  RTX PRO 6000 Blackwell         Off | 00000000:01:00.0 Off       |
| 30%   62C    P0    412W / 600W          |  87234MiB / 97887MiB       |
|                                         |       94%      Default     |
+-----------------------------------------+----------------------------+

+-----------------------------------------------------------------------+
| Processes:                                                            |
|  GPU   PID   Type   Process name                        GPU Memory    |
|=======================================================================|
|    0  12847     C   /usr/bin/python3                     87180MiB     |
+-----------------------------------------------------------------------+
```

-   **CUDA Version:** This is the maximum CUDA version supported by the driver, not the installed toolkit version. It's common to misunderstand this as the toolkit version.
-   **Memory Usage:** Even when run inside a container, it's shown based on the entire host. It includes memory used by other containers.
-   **GPU-Util:** This is the ratio of time during the last sampling interval that at least one kernel was running. It doesn't indicate how many cores were active; even if one kernel runs continuously using only one SM, it will show 100%.
-   **Compute M:** If "Default," multiple processes can create contexts. If "Exclusive_Process," only one is allowed.
-   **Processes:** This is a list of processes that have created a CUDA context on the GPU. Each row is one process, and the last column is the GPU memory consumed by that process. Due to different PID namespaces inside containers, this list might appear empty or show PIDs that don't match the host. The Type column indicates usage: C for compute (CUDA), G for graphics, C+G for both. Serving processes are shown as C.

Do not misunderstand the meaning of GPU-Util. If it's 94% but throughput is low, it's likely that cores are idle, and separate metrics are needed to see actual utilization.

```bash
# Including SM occupancy and memory bandwidth
nvidia-smi dmon -s pucvmet -d 1

# Items
#   sm    SM active ratio
#   mem   Memory bandwidth usage ratio
#   pwr   Power
#   mclk/pclk  Memory/core clock
```

<br>

## Deploying with Containers

### Problem - No Driver Inside the Container

Containers share the kernel with the host but isolate the filesystem, so there are no `/dev/nvidia*` or `libcuda.so` files inside the container.

The method of putting the driver into the image cannot be used. `libcuda.so` must exactly match the host's kernel module version, and embedding it in the image would break it the moment the host driver is updated.

### Solution

The NVIDIA Container Toolkit injects the host's device nodes and driver libraries into the container at container startup.

```
Host                            Container (after injection)
/dev/nvidia0            ──────→  /dev/nvidia0
/dev/nvidiactl          ──────→  /dev/nvidiactl
/dev/nvidia-uvm         ──────→  /dev/nvidia-uvm
libcuda.so.580.178.04   ──────→  /usr/lib/.../libcuda.so
nvidia-smi              ──────→  /usr/bin/nvidia-smi

What the image brings
  libcudart.so, cuBLAS, cuDNN, PyTorch, serving engine
```

The driver layer is injected from the host, and the runtime layer is included in the image.

Therefore, including the CUDA toolkit in the image is normal, but including the driver is incorrect.

```bash
# Install toolkit
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey \
  | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg
curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list \
  | sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' \
  | sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
sudo apt update && sudo apt install -y nvidia-container-toolkit

# Register runtime with Docker
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker

# Verify
docker run --rm --gpus all nvidia/cuda:12.9.2-base-ubuntu22.04 nvidia-smi
```

Every time the driver is updated, you must re-run `nvidia-ctk runtime configure` and restart Docker. If you don't, `--gpus` calls will start failing. This is a constant maintenance item for this stack.

### CDI, the Recent Standard Path

The `--gpus` flag operates as a Docker-specific hook. Nowadays, there's a shift towards CDA (Container Device Inference), a vendor-neutral specification used by Kubernetes, Podman, and Docker.

```bash
# Generate CDI specification (regenerate if driver settings change)
sudo nvidia-ctk cdi generate --output=/etc/cdi/nvidia.yaml

# Usage
docker run --rm --device nvidia.com/gpu=0 <image> nvidia-smi
```

The specification file describes which nodes and libraries are needed to use this device, and a CDI-aware runtime reads and injects them. If generated in `/var/run/cdi`, it will be deleted on reboot, so `/etc/cdi` is preferable.

### Deployment - Starting the Serving Container

```bash
docker run --rm -d \ # --rm is included for one-time verification
  --gpus '"device=0"' \
  --ipc=host \
  --shm-size=16g \
  -p 8000:8000 \
  -v /models:/models:ro \
  -e HF_HOME=/models/hf \
  vllm/vllm-openai:latest \
    --model /models/Qwen3.6-27B \
    --max-model-len 32768 \
    --gpu-memory-utilization 0.90
```

```
# Docker standard flags
-d                      Run in background. If not attached, the terminal will be occupied
--name vllm             Assign a name. Used for referencing, e.g., docker logs vllm
--restart unless-stopped  Automatically restart if the process dies
-p 8000:8000            Map host port 8000 to container port 8000
-v /models:/models:ro   Mount host directory as read-only
-e HF_HOME=...          Environment variable. Here, the model cache location
```

`--rm` is not used in serving. It's a flag to automatically delete the container when it stops, but it's included here for testing purposes. Since `docker logs` also won't be available, use it only for one-time verification.

-   `--ipc=host`, `--shm-size`: Tensor parallelism or data loaders use inter-process shared memory. Docker's default 64MB is insufficient, causing startup failures.
-   `-v /models:ro`: If weights are put into the image, the image becomes tens of GBs. Mount it as a volume and set it to read-only.
-   `gpus '"device=0"'`: Injects only card 0 into the container. The double quotes are because the shell consumes one layer. If using `--gpus all`, quotes are not needed.

<br>

## Request Flow

Let's examine the flow from HTTP to the GPU. There's one process, but two types of work run within it: one for receiving sockets and one for issuing commands to the GPU.

```
Client
   │ HTTP POST /v1/chat/completions
   ▼
[Socket → HTTP Parser]           CPU thread
   │ Tokenization
   ▼
[Scheduler Queue]                CPU
   │ Selects requests from the waiting queue to include in the current step
   │ KV cache block allocation
   ▼
[Batch Composition]              Groups multiple requests into one
   │
   ▼
[Kernel Launch]                  CPU pushes commands to the GPU stream
   │                            ← Asynchronous. CPU returns immediately
   ▼
[GPU Execution]                  Executes kernels accumulated in the stream sequentially
   │
   ▼
[Result Synchronization]         CPU reads the results
   │ Detokenization
   ▼
[Streaming Response]             Sends tokens one by one via SSE
```

**The asynchronous nature of kernel launch is central to the architecture.** The CPU pushes GPU commands and immediately returns to prepare the next batch.

While the GPU computes the current batch, if the CPU finishes scheduling the next batch, the GPU continues working without pause.

Conversely, if the CPU is slow, the GPU waits for commands and idles. This phenomenon is more pronounced with smaller models. If one kernel takes 3ms and scheduling takes 3ms, the GPU only works half the time.

### Concept - Stream

A stream is a command queue sent to the GPU. Kernels placed in the same stream execute sequentially, while kernels in different streams can overlap.

```
Stream 0  [KernelA][KernelB][KernelC]        Order guaranteed
Stream 1        [CopyD][KernelE]             Can run concurrently with Stream 0
```

Serving engines typically separate compute and copy streams to prevent computation from halting while weights or KV are being moved.

### Pitfall: Port Open but Not Ready

Although the HTTP server is the last step in the startup sequence, the first request is still slow. This is because an uncaptured batch size might arrive, or an unused kernel might be JIT-compiled.

```
First request   Several seconds. JIT compilation and cache miss
10-30 requests  Gradually stabilizes
After that      Normal performance
```

**If you don't warm up during benchmarking, this phase will contaminate the results.** Pushing traffic immediately after a health check passes during deployment will cause initial latency spikes.

<br>

## Differences When Running Training

### Changes in Memory Configuration

Serving and training use GPU memory differently.

```
Serving                         Training
─────────────────────────────────────────────────
Weights                         Weights
Activations (small)             Activations (all stored for backpropagation)
KV Cache (most)                 Gradients (same size as weights)
                                Optimizer state (2x weights for Adam)
```

**Full fine-tuning with Adam requires four times the weights.**

This includes the weights themselves, gradients, and the first and second moments of the optimizer. To tune a 27B model with BF16, it would require 54GB x 4 = 216GB, which cannot be done with a single 96GB card.

Therefore, LoRA is typically used on a single card. By freezing the original weights and training only a small adapter, the gradients and optimizer state are reduced to the size of the adapter.

```
27B model, 96GB card

Full fine-tuning (Adam, BF16)    216GB  Impossible
LoRA (BF16 base)                 Approx. 60GB  Possible
QLoRA (4-bit base)               Approx. 25GB  Plenty of room
```

### Differences in Execution Patterns

```
Serving    Requests arrive sporadically. Batch composition constantly changes
           Latency is the metric. Scheduler is complex

Training   Data is pre-available. Batches are fixed
           Throughput is the only metric. Pipeline can be fully utilized
```

This is why pipeline parallelism is useful in training but less so in inference: if there's pre-split data to push, GPU idle periods can be filled.

### Deployment - Training Container

```bash
docker run --rm -it \
  --gpus all \
  --ipc=host --shm-size=32g \
  --ulimit memlock=-1 --ulimit stack=67108864 \
  -v /data:/data -v /out:/out \
  nvcr.io/nvidia/pytorch:26.07-py3 \
  bash
```

```
--rm -it              One-time interactive shell to be discarded. Not a persistent process like serving
--gpus all            Inject all visible cards. Training typically uses multiple cards together
--ipc=host            Shared memory between worker processes. Used by data loaders
--shm-size=32g        Larger than serving (16g). Data loader workers stage batches here
--ulimit memlock=-1   Lift page-locked memory limit
--ulimit stack=64MB   Increase thread stack size
-v /data -v /out      Dataset input and checkpoint output
```

`--ulimit memlock` is a training-specific item. For the GPU to directly read CPU memory, those pages must not be swapped to disk; this memory is called page-locked (pinned) memory. Communication libraries and data loaders allocate a large amount of this, and the default Linux limit of 64KB is immediately hit.

```
$ ulimit -l
64                    ← In KB. 64KB

Symptom   NCCL initialization failure, or "cannot allocate pinned memory"
Cause     Requirement exceeds limit
Action    Set to -1 to lift the limit
```

`stack` is increased because worker threads sometimes need more than the default 8MB for deep recursion or large local variables. 67108864 is 64MB.

<br>

## Diagnostics

### Troubleshooting Steps by Symptom

**If the GPU is not visible in the container:**

```bash
nvidia-smi                                    # Check host driver
docker info | grep -i runtime                 # Check NVIDIA runtime registration
docker run --rm --gpus all nvidia/cuda:12.9.2-base-ubuntu22.04 nvidia-smi
```

The cause varies depending on where it breaks in these three steps: if the first step fails, it's the driver; if the second, it's the toolkit configuration; if the third, it's the image or CDI specification. If the driver was recently updated, re-run `nvidia-ctk runtime configure`.

**If "out of memory" appears:**

```bash
nvidia-smi --query-compute-apps=pid,used_memory --format=csv
```

Sometimes a dead process holds onto memory. If memory isn't reclaimed even after deleting the container, there's a zombie process on the host.

**If it's slow but gpu-util is high:**

When checking the `sm` column with `nvidia-smi dmon -s pum`, if `gpu-util` is in the 90% range but `sm` is low, it means the GPU is not being fully utilized. The batch size might be too small, or the CPU-side scheduling is a bottleneck.

**If it gets slower over time:**

```bash
nvidia-smi -q -d PERFORMANCE | grep -A5 "Clocks Throttle"
```

If `SW Power Cap` or `HW Thermal Slowdown` is Active, it means power or thermal limits have been hit. This is common when using a 600W card with insufficient case airflow.

**Xid Error:**

```bash
sudo dmesg | grep -i xid
```

Xid is a hardware error code that the driver logs in the kernel log. Each number has a different meaning, and repeated occurrences likely indicate a hardware or driver issue.

#### Things to Know When Using RTX PRO 6000 Blackwell

Workstation-grade Blackwell has a compute capability of sm_120, which differs from datacenter Blackwell (sm_100). A higher number does not imply backward compatibility.

```
Kernels compiled for sm_100 do not run on sm_120
→ Framework takes a slower fallback path or execution fails
→ Cases reported where NVFP4 MoE kernels fall back to Marlin backend
```

The card's specifications are as follows:

```
Memory        96GB GDDR7 ECC, 512-bit
Bandwidth     1,792 GB/s (workstation) / 1,597 GB/s (server)
TDP           400~600W configurable
MIG           Server edition supports up to 4 instances of 24GB
Usable        Approx. 86GB, excluding driver overhead
```

**It is safer to first check if the official serving image properly supports sm_120.**

If an 8B model loaded with FP8 consumes over 90GB, it indicates that the quantization kernel has fallen back, and the image or build options need to be changed.

<br>

## Virtual Lab: Serving

Let's conduct a practical exercise based on the content so far.

```
Equipment   Ubuntu 24.04, RTX PRO 6000 Blackwell 96GB (1 card), RAM 128GB
Goal        Deploy Qwen3.6-27B on port 8000 and verify requests are processed
```

### Step 1 - Initial State

```bash
$ nvidia-smi
Command 'nvidia-smi' not found

$ lspci | grep -i nvidia
01:00.0 VGA compatible controller: NVIDIA Corporation Device 2bb1 (rev a1)
```

The card is installed, but there's no driver. This is the state if `lspci` shows it but `nvidia-smi` is missing.

First, let's look at the available drivers.

```bash
$ ubuntu-drivers devices
vendor   : NVIDIA Corporation
model    : RTX PRO 6000 Blackwell
driver   : nvidia-driver-580-server - distro non-free recommended
driver   : nvidia-driver-580        - distro non-free
driver   : nvidia-driver-575-server - distro non-free
```

```
580        Driver branch number. Higher means newer and supports new GPUs
           Blackwell series requires 570 or higher for recognition

-server    Datacenter variant
            · Display output related components are omitted
            · Branch is maintained for longer
            (The non-server version is for desktops. Includes graphics stack)

recommended  What the distribution recommends for this card
```

If you're only deploying serving on a headless server, you should use `-server`. If you're using a monitor or also using it as a workstation, choose the non-server version.

```bash
$ sudo apt update && sudo apt install -y nvidia-driver-580-server
$ sudo reboot
```

### Step 2 - Driver Verification

```bash
$ nvidia-smi
+-----------------------------------------------------------------------+
| NVIDIA-SMI 580.178.04    Driver Version: 580.178.04  CUDA Version: 13.0|
|-----------------------------------------+----------------------------+
|   0  RTX PRO 6000 Blackwell         Off | 00000000:01:00.0 Off       |
| 30%   38C    P8     22W / 600W          |      4MiB / 97887MiB       |
|                                         |        0%      Default     |
+-----------------------------------------+----------------------------+

$ ls /dev/nvidia*
/dev/nvidia0  /dev/nvidiactl  /dev/nvidia-uvm  /dev/nvidia-uvm-tools
```

Device files have appeared. 4MiB is consumed by the driver itself, and no context has been created yet.

Enable persistence mode. If not enabled, the driver unloads when no processes are running, slowing down the next startup by a few seconds.

```bash
$ sudo nvidia-smi -pm 1
Enabled persistence mode for GPU 00000000:01:00.0.
```

### Step 3 - Docker and Toolkit

```bash
$ sudo apt install -y docker.io
$ curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey     | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg
$ curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list     | sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g'     | sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
$ sudo apt update && sudo apt install -y nvidia-container-toolkit

$ sudo nvidia-ctk runtime configure --runtime=docker
INFO[0000] Loading config from /etc/docker/daemon.json
INFO[0000] Wrote updated config to /etc/docker/daemon.json
INFO[0000] It is recommended that docker daemon be restarted.

$ sudo systemctl restart docker
$ docker run --rm --gpus all nvidia/cuda:12.9.2-base-ubuntu22.04 nvidia-smi
```

If the last command produces the same output as the host, injection is successful.

### Step 4 - First Attempt, Insufficient Shared Memory

```bash
$ docker run -d --name vllm \
    --gpus '"device=0"' \
    -p 8000:8000 \
    -v /models:/models:ro \
    vllm/vllm-openai:latest \
      --model /models/Qwen3.6-27B \
      --max-model-len 32768
```

```bash
$ docker logs -f vllm
INFO  Starting vLLM engine...
INFO  Loading safetensors checkpoint shards: 0% Completed
ERROR RuntimeError: unable to open shared memory object </torch_shm_1a2b>
      in read-write mode: No space left on device (28)
```

Docker's default `/dev/shm` is 64MB, which is insufficient for weight loading or inter-worker shared memory communication, causing it to fail here.

```bash
$ docker run -d --name vllm \
    --gpus '"device=0"' \
    --ipc=host --shm-size=16g \
    -p 8000:8000 \
    -v /models:/models:ro \
    vllm/vllm-openai:latest \
      --model /models/Qwen3.6-27B \
      --max-model-len 32768
```

```bash
$ docker logs -f vllm
INFO  Loading safetensors checkpoint shards: 100% Completed
INFO  Loading model weights took 54.12 GB
INFO  Memory profiling results:
        total_gpu_memory=95.00GiB
        model_weights=54.12GiB
        non_torch_memory=0.91GiB
        PyTorch_activation_peak=4.38GiB
        gpu_memory_utilization=0.90
INFO  GPU KV cache size: 8,192 tokens
INFO  Maximum concurrency for 32,768 tokens per request: 0.25x
ERROR ValueError: To serve at least one request with 32768 tokens,
      more KV cache is needed than available.
```

Startup proceeded to step 5 (profiling) but stopped at step 6.

```
Total           95.00 GB
Utilization 0.90  Only 85.50 GB used
  Weights       54.12
  Context, etc. 0.91
  Peak activations 4.38
  ────────────────────
  KV share      26.09 GB  →  8,192 tokens

One request requires 32,768 tokens
8,192 ÷ 32,768 = 0.25x  →  Cannot accommodate even one request
```

With BF16, weights alone consume 54GB. There are three options:

```
Reduce context length        --max-model-len 8192   → Barely one request
Increase utilization         0.90 → 0.95            → Approx. 4.7GB additional. Insufficient
Reduce weights               FP8 quantization       → 54GB → 27GB
```

The third option is the only one with a meaningful impact.

### Step 6 - Quantization

```bash
$ docker run -d --name vllm \
    --gpus '"device=0"' \
    --ipc=host --shm-size=16g \
    -p 8000:8000 \
    -v /models:/models:ro \
    vllm/vllm-openai:latest \
      --model /models/Qwen3.6-27B-FP8 \
      --max-model-len 32768 \
      --gpu-memory-utilization 0.92
```

```
INFO  Loading model weights took 27.31 GB
INFO  Memory profiling results:
        total_gpu_memory=95.00GiB
        model_weights=27.31GiB
        non_torch_memory=0.89GiB
        PyTorch_activation_peak=4.41GiB
        gpu_memory_utilization=0.92
INFO  GPU KV cache size: 176,128 tokens
INFO  Maximum concurrency for 32,768 tokens per request: 5.37x
INFO  Capturing CUDA graphs: 100%|██████████| 35/35 [00:47<00:00]
INFO  init engine took 94.22 seconds
INFO  Starting vLLM API server on http://0.0.0.0:8000
```

With weights halved, the KV share increased from 26GB to 60GB, and tokens increased from 8,192 to 176,128, a 21x increase. `5.31x` means it can handle five 32K requests concurrently.

Of `init engine took 94.22 seconds`, 47 seconds were for graph capture. This time needs to be considered in environments with frequent restarts.

```bash
$ nvidia-smi
|   0  RTX PRO 6000 Blackwell         Off | 00000000:01:00.0 Off       |
| 30%   41C    P0     78W / 600W          |  87920MiB / 97887MiB       |
|                                         |        0%      Default     |

+-----------------------------------------------------------------------+
|  GPU   PID   Type   Process name                        GPU Memory    |
|    0  31204     C   /usr/bin/python3                     87908MiB     |
+-----------------------------------------------------------------------+
```

```
87.9 GB occupied    Weights 27 + KV pool 60 + remainder. Already allocated even without requests
GPU-Util 0%         Memory is allocated, but no computation is happening
Power 78W           Idle. Will rise to 400W range under load
```

**The KV pool is preempted entirely and then redistributed internally. It's normal for 87GB to appear allocated even without requests.**

### Step 8 - Sending Requests

```bash
$ curl -s localhost:8000/v1/models | jq -r '.data[].id'
/models/Qwen3.6-27B-FP8

$ time curl -s localhost:8000/v1/chat/completions \
    -H 'Content-Type: application/json' \
    -d '{"model":"/models/Qwen3.6-27B-FP8",
         "messages":[{"role":"user","content":"파이썬으로 피보나치 함수를 짜줘"}],
         "max_tokens":200}' | jq -r '.choices[0].message.content' | head -3

def fib(n):
    if n <= 1:
        return n

real    0m4.812s
```

4.8 seconds is not normal. Sending it again yields:

```bash
$ time curl -s ... | jq -r '.choices[0].message.content' > /dev/null
real    0m2.103s

$ time curl -s ... | jq -r '.choices[0].message.content' > /dev/null
real    0m2.088s
```

The extra 2.7 seconds in the first request are due to JIT compilation and cache warming. This period should be excluded when designing benchmarks or health checks.

### Step 9 - Applying Load and Observing

Open a monitor in one terminal.

```bash
$ nvidia-smi dmon -s pucm -d 2
# gpu   pwr  gtemp   sm   mem   enc   dec  mclk  pclk
    0    76     41     0     0     0     0  1500  2100
```

Apply load in another terminal.

```bash
$ python -m vllm.entrypoints.openai.api_server --help > /dev/null 2>&1
$ vllm bench serve --backend openai --port 8000 \
    --model /models/Qwen3.6-27B-FP8 \
    --dataset-name random --random-input-len 2048 --random-output-len 512 \
    --num-prompts 300 --max-concurrency 24
```

```
# gpu   pwr  gtemp   sm   mem   enc   dec  mclk  pclk
    0   412     63    97    71     0     0  1500  2610
    0   428     65    98    73     0     0  1500  2610
    0   419     66    96    70     0     0  1500  2595
```

The `sm 97` and `mem 71` values indicate that SM is almost saturated and memory bandwidth is around 70%, meaning the GPU is being properly utilized. If `sm` is around 30% but `gpu util` shows 100%, it signals that the kernel is not fully utilizing the GPU.

Benchmark results:

```
============ Serving Benchmark Result ============
Successful requests:                     300
Request throughput (req/s):              3.41
Output token throughput (tok/s):         1746.2
Median TTFT (ms):                        387.4
Median ITL (ms):                         13.2
==================================================
```

### Step 10 - Verifying sm_120 Fallback

Having confirmed that it runs, the remaining question is whether it's performing optimally.

Workstation Blackwell GPUs can sometimes cause kernels to fall back to slower paths.

```bash
$ docker logs vllm 2>&1 | grep -iE "backend|fallback|marlin|capability"
INFO  Detected device capability: 12.0 (sm_120)
INFO  Using MarlinLinearKernel for FP8 quantization
WARNING  Native FP8 kernels unavailable for sm_120, falling back
```

If this log appears, it means a generic path is being used instead of a dedicated kernel, and the card is only delivering a fraction of its potential performance. Three things to check:

```
□ Is the image version a release that supports sm_120?
□ Is the quantization format (FP8 / NVFP4 / AWQ) appropriate for that combination?
□ Was 12.0 included in TORCH_CUDA_ARCH_LIST during source build?
```

**It's convenient to have one criterion for judgment:** if an 8B-class model loaded with FP8 doesn't consume more than 20GB.

### Summary - What was Filtered Out in the Lab

```
Symptom                        Cause                     Check Point
──────────────────────────────────────────────────────────────
Shared memory error            Docker default shm 64MB   --ipc=host or --shm-size
Maximum concurrency 0.25x      Weights encroaching KV share  Breakdown in profiling logs
First request 4.8 seconds      JIT compilation, cache miss  Compare with second request
Slow despite high GPU-Util     Kernel not fully utilizing GPU  'sm' column in dmon
FP8 consuming too much memory  sm_120 kernel fallback    'backend' line in startup logs
```

<br>

## Command Collection

```bash
# Devices and Drivers
nvidia-smi                                    # Overall status
nvidia-smi -L                                 # List of GPUs and UUIDs
nvidia-smi -q -d MEMORY,POWER,TEMPERATURE     # Detailed information by item
nvidia-smi topo -m                            # GPU connection topology

# Real-time Monitoring
nvidia-smi dmon -s pucvmet -d 1               # 1-second interval
nvidia-smi --query-gpu=timestamp,utilization.gpu,memory.used,power.draw \
  --format=csv -l 1                           # Log to CSV

# Processes
nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv
sudo fuser -v /dev/nvidia*                    # Processes holding device files

# Containers
docker run --rm --gpus all <image> nvidia-smi
sudo nvidia-ctk runtime configure --runtime=docker
sudo nvidia-ctk cdi generate --output=/etc/cdi/nvidia.yaml

# Power Limit (when wanting to increase performance per watt)
sudo nvidia-smi -pl 450                       # Watts

# Persistence Mode (reduces startup delay)
sudo nvidia-smi -pm 1
```
