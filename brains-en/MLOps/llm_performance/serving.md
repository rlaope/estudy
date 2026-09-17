# Production Model Serving and Capacity Planning

Beyond running large language models (LLMs) at a laboratory level,
the final hurdle engineers must overcome to bring them to the actual production stage is
**configuring an OpenAI-compatible API server** and performing capacity planning to accurately predict infrastructure resource requirements.

## How Many GPUs Are Needed to Handle 100 or 1,000 Concurrent Users?

To answer this question, we must first redefine "Concurrent Users" from an engineering perspective. While concurrent users in a web service simply refer to connected sockets,
the number of concurrent users for an LLM means the **active requests that are actually generating tokens (decoding) or processing prompts (prefilling) at that moment.**

**The key variables for hardware estimation** cannot be calculated solely based on the number of users; the following four key SLA variables must be combined.

1. **Target Model**: Model parameter size (e.g., Llama-3-8B vs 70B)
2. **Input Token Length($L_{in}$)**: Average length of user prompts
3. **Output Token Length($L_{out}$)**: Average length of model-generated output
4. **SLA-based Latency**: Target Time to First Token (TTFT) and Inter-Token Latency (ITL)

By reverse-calculating based on benchmark data, the maximum active batch size that a single GPU can handle in an optimized vLLM engine environment (with Continuous Batching + PagedAttention applied) can be derived.
Based on this, the required infrastructure quantity for scenarios with 100 and 1,000 concurrent users can be estimated.

<br>

## OpenAI-Compatible API Server and Docker Deployment

In a commercial environment, the inference engine should be isolated in an independent container, and endpoints must be exposed to be fully compatible with standard JavaScript or Python OpenAI SDKs.

```dockerfile
# Dockerfile
FROM nvidia/cuda:12.1.1-devel-ubuntu22.04

# 시스템 의존성 설치
RUN apt-get update && apt-get install -y \
    python3-pip \
    python3-dev \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 파이썬 환경 설정
RUN ln -s /usr/bin/python3 /usr/bin/python
RUN pip3 install --no-cache-dir --upgrade pip

# vLLM 및 관련 가속 라이브러리 설치
RUN pip3 install --no-cache-dir vllm==0.4.2 flash-attn==2.5.8

# 서비스 포트 오픈 (기본 vLLM API 포트: 8000, 모니터링 포트: 8000/metrics)
EXPOSE 8000

# 모델 캐시 디렉토리 마운트용 환경변수
ENV HF_HOME=/data/huggingface

# API 서버 실행 명령어
ENTRYPOINT ["python3", "-m", "vllm.entrypoints.openai.api_server"]
```

If we build a configuration using docker-compose that allocates all GPU resources on the hardware and enables an OpenAI-compatible API environment and Prometheus metric collection:

```yaml
# docker-compose.yml
version: '3.8'

services:
  llm-serving:
    build: .
    image: production-vllm-server:v1
    container_name: vllm-api-service
    environment:
      - CUDA_VISIBLE_DEVICES=0,1,2,3  # 4개의 GPU 사용
      - HF_HOME=/data/huggingface
    volumes:
      - /mnt/storage/models:/data/huggingface  # 호스트의 모델 스토리지 공유
    ports:
      - "8000:8000"
    ipc: host  # GPU 간 고속 공유 메모리(NCCL) 통신 활성화
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all  # 컨테이너에 모든 NVIDIA GPU 패스스루
              capabilities: [gpu]
    restart: unless-stopped
    command: >
      --model meta-llama/Meta-Llama-3-70B-Instruct-AWQ
      --quantization awq
      --tensor-parallel-size 4
      --gpu-memory-utilization 0.92
      --max-num-seqs 256
      --port 8000
```

<br>

## GPU Capacity Planning Theory and Derivation Formulas

To formulate an infrastructure capacity plan, engineers must calculate two key **barriers: the VRAM capacity required by weights + KV Cache, and the computational power to process incoming token traffic.**

### Available VRAM Capacity Required per Single Request ($M_{req}$)

This is the formula for the physical memory that must be dynamically secured to handle concurrent requests, in addition to the fixed allocation for model weights.

$$M_{weight} = \text{Parameter Size} \times \text{Bytes per Parameter}$$

$$M_{KVCache} = 2 \times 2 \times N_{layers} \times N_{heads} \times d_{head} \times (L_{in} + L_{out})$$

$$M_{total} = M_{weight} + (M_{KVCache} \times \text{Target Concurrency})$$

### Estimating Required GPU Quantity Based on Target Concurrent Throughput

This is a practical equipment estimation formula based on the system's token processing capability per second.

Total Generated Tokens Required per Second **Required System Throughput**

$$\text{Req Throughput (tokens/s)} = \text{동시 사용자 수} \times \text{사용자당 초당 필요 생성 속도 (예: 30 tokens/s)}$$

Required GPU Nodes

$$\text{Required GPUs} = \frac{\text{Req Throughput (tokens/s)}}{\text{단일 GPU의 최대 실측 Throughput (tokens/s)}}$$

<br>

## Operational Capacity Planning Architecture Guidelines

The following guidelines are a standard resource design document format that can be immediately submitted to the infrastructure design team, based on business requirements (100 vs. 1,000 concurrent users) and corporate budget (L4 cost-effective nodes vs. A100/H100 enterprise nodes).

### Capacity Planning by Hardware Specification apxmflrtm

(Conditions: Llama-3-8B FP16 model serving, average input 1,024 tokens / output 512 tokens, SLA: ITL < 25ms maintained)

### Criteria for Choosing Scale-up (A100) vs Scale-out (L4)

In the initial service phase with fewer than 100 concurrent users, rather than deploying expensive A100s,
bundling multiple smaller NVIDIA L4 accelerators into a Data Parallel distributed serving environment becomes an efficient alternative, reducing initial infrastructure CapEx by over 40%.

### Key to Handling Large-Scale Traffic of 1,000+ Users

When concurrent users reach 1,000, a single instance clearly exceeds its hardware limitations.
It is essential to implement a distributed architecture design that places intelligent load balancers like **NGINX** or **Ray LLM Router** in front of vLLM instances and routes requests to an active multi-node GPU cluster.
