# Production Model Serving과 Capacity Planning

대형 언어 모델 LLM을 실험실 수준에서 구동하는 것을 넘어서

실제 사용화 production 단계로 올리기 위헤 엔지니어가 거쳐야하는 최종 관문은

**OpenAI 호환 API 서버 구성**과 정확한 인프라 리소스 규모를 예측하는 용량 계획 Capacity Planning이다.

## 동시 사용자 100명, 1,000명을 처리하면 어떤 GPU가 몇 장 필요한가?

이 질문에 답하기 위해서는 먼저 동시 사용자 Concurrent Users에 대한 정의를

엔지니어링 관점으로 재해석 해야한다. 웹서비스의 동시 접속자는 단순히 소켓이 연결된 상태를 뜻하지만,

LLM의 동시 접속자 수는 **그 순간 실제 토큰을 생성(Decode)하거나 프롬프트를 처리(Prefill) 중인 활성 요청 Active Request**를 의미한다.

**하드웨어 산정의 핵심 변수는** 단순히 사용자 수만으로 계산할 수 없으며 다음 4가지 핵심 변수 SLA가 결합되어야 한다.

1. **Target Model**: 모델의 파라미터 크기 (ex. Llama-3-8B vs 70B)
2. **Input Token Length($L_{in}$)**: 유저 프롬프트의 평균 길이
3. **Output Token Length($L_{out}$)**: 모델이 생성할 평균 결과물 길이
4. **SLA 기준 지연 시간**: 목표로 하는 첫 토큰 지연 시간 TTFT 및 토큰당 생성 시간 ITL

벤치마크 데이터를 기반으로 역산하면 최적화된 vLLM 엔진 환경 (Continuous Batching + PagedAttention 적용)에서 단일 GPU 장비가 감당할 수 있는 최대 Active Batch 크기가 도출된다.

이를 기준으로 동시 사용자 100명, 1,000명의 시나리오의 필요한 인프라 수량을 산정할 수 있다.

<br>

## OpenAI 호환 API 서버 및 Docker 배포

상용 환경에서는 추론 엔진을 독립적인 컨테이너로 격리하고, 표준 자바스크립트나 파이선 OpenAI SDK와 완벽히 호환되도록 엔드포인트 노출을 해야한다.

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

하드웨어의 모든 GPU 자원을 할당하고, OpenAI 호환 API 환경 및 Prometheus용 메트릭 수집을 활성화하는 구성을 docker-compose로 구축해보면

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

## GPU Capacity Planning 이론 및 유도 공식

인프라 용량 계획을 세우기 위해 엔지니어가 계산해야하는 두 가지 핵심 **장벽은 가중치 + KV Cache가 요구하는 VRAM 용량과 유입되는 토큰 트래픽을 처리하는 연산 능력**이다.

### 단일 요청당 필요한 가용 VRAM 용량 ($M_{req}$)

모델 가중치 고정 할당량 외에, 동시 요청을 소화하기 위해 유동적으로 확보되어야하는 물리 메모리 공식이다.

$$M_{weight} = \text{Parameter Size} \times \text{Bytes per Parameter}$$

$$M_{KVCache} = 2 \times 2 \times N_{layers} \times N_{heads} \times d_{head} \times (L_{in} + L_{out})$$

$$M_{total} = M_{weight} + (M_{KVCache} \times \text{Target Concurrency})$$

### 목표 동시 처리량 기반 필요한 GPU 수량 산정

시스템에 초당 도출되는 토큰 처리 능력을 기반으로 한 실전 장비 산정 공식이다.

초당 필요한 총 생성 토큰량 **Required System Throughput**

$$\text{Req Throughput (tokens/s)} = \text{동시 사용자 수} \times \text{사용자당 초당 필요 생성 속도 (예: 30 tokens/s)}$$

필요한 gpu 노드수

$$\text{Required GPUs} = \frac{\text{Req Throughput (tokens/s)}}{\text{단일 GPU의 최대 실측 Throughput (tokens/s)}}$$

<br>

## 운영 가능한 Capacity Planning 아키텍처 가이드라인

아래 가이드는 비즈니스 요건 (동시 사용자 100명 vs 1,000명) 및 기업의 예산 (L4 가성비 노드 vs A100/H100 엔터프라이즈 노드)에 따라 인프라 설계 팀에 즉시 제출할 수 있는 리소스 설계서 표준 양식이다.

### 하드웨어 사양별 Capacity Planning apxmflrtm

(기준 조건: Llama-3-8B FP16 모델 서빙, 평균 인풋 1,024 토큰 / 아웃풋 512 토큰 설정, SLA: ITL < 25ms 유지)

### Scale-up (A100) vs Scale-out(L4) 선택 기준

동시 사용자 100명 미만인 초기 서비스 단계에서는 단가가 비싼 A100을 들여놓는 것 보다.

소형 가속기인 NVIDIA L4 여러대를 Data Parallel 분산 서빙 환경으로 묶는 것이 초기 인프라 구축 비용 CapEx를 40% 이상 절감하는 효율적인 대안이 된다.

### 1,000명 이상 초대규모 트래픽 대응의 핵심

동시 유저가 1,000명에 이르면 단일 인스턴스의 하드웨어 한계를 명확히 벗어난다.

vLLM 인스턴스 전면에 **NGINX** 혹은 **Ray LLM Router**와 같은 인텔리전트 로드 밸런서를 배치하고 가동중인 다중 노드 GPU 클러스터로 요청을 라우팅하는 분산 아키텍처 설계가 필수적으로 수반되어야 한다.
