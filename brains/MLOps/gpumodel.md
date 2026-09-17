# GPU 위에 모델이 뜨는 과정

**프로세스, 드라이버, 컨테이너, 서빙 엔진이 맞물리는 시점을 알아보자.**

`nvidia-smi` 출력을 보고 무슨일이 벌어지는지 무슨 프로세스가 얼마를 잡고있는지 그 메모리가 왜 그만큼인지 gpu가 일하는중인지 대기중인지를 판단하고

컨테이너 안에서 gpu가 안보일때 어느층이 끊겼는지 짚고 드라이브인지 컨테이너 툴킷인지 이미지인지를 판단할 수 있는 방법이 무엇일까

서빙 엔진의 기동 로그를 순서대로 해석해서 가중치 로드부터 kv cache선점 그리고 그래프캡처까지 각 단계가 무엇을 하는지 어디서 멈췄는지를 좁힐수있는 능력을 얻어보자.

<br>

## GPU를 사용한다는 것

### GPU의 접근 3층 구조

gpu는 리눅스에서 **장치 파일**이다.

프로세스가 gpu를 쓴다는 것은 그 파일을 열어 명령을 밀어넣는 일이다.

```
$ ls -l /dev/nvidia*
crw-rw-rw- 1 root root 195,   0  /dev/nvidia0        ← 0번 GPU
crw-rw-rw- 1 root root 195, 255  /dev/nvidiactl      ← 제어용
crw-rw-rw- 1 root root 234,   0  /dev/nvidia-uvm     ← 통합 메모리
crw-rw-rw- 1 root root 234,   1  /dev/nvidia-uvm-tools
```

이 파일 위에 세 개의 층이 얹힌다.

```
┌──────────────────────────────────────┐
│ 애플리케이션   vLLM, SGLang, PyTorch  │
├──────────────────────────────────────┤
│ CUDA 런타임    libcudart.so           │  앱과 함께 배포됨
│               cuBLAS, cuDNN 등        │  버전이 앱에 묶임
├──────────────────────────────────────┤
│ 드라이버 API   libcuda.so             │  드라이버 설치 시 함께
│                                       │  호스트에 하나만 존재
├──────────────────────────────────────┤
│ 커널 드라이버   nvidia.ko              │  커널 모듈
├──────────────────────────────────────┤
│ 장치 파일      /dev/nvidia*            │
└──────────────────────────────────────┘
```

중간에 경계 하나가 있는데 `libcuda.so`는 드라이버에 속하고 흐스트에 하나만 존재한다.

`libcudart.so`와 그 위 애플리케이션은 들고 다닌다. 

### CUDA 컨텍스트

프로세스가 gpu를 처음 쓰려고 하면 드라이버가 **CUDA 컨텍스트**를 만든다.

프로세스마다 하나씩 gpu마다 따로 생긴다, 컨텍스트가 들고있는 정보는 이런데

```
GPU 가상 주소 공간      cudaMalloc이 반환하는 포인터가 사는 곳
로드된 커널 코드         이 프로세스가 쓸 GPU 함수들
스트림과 이벤트          비동기 작업 큐
메모리 할당 장부         무엇을 얼마나 잡았는지
```

**process간 격리가 된다** A프로세스가 받던 gpu포인터를 B프로세스가 역참조할 수 없다는 뜻인데 프로세스가 죽으면 컨텍스트가 사라지고 잡고있던 gpu메모리가 전부 회수가된다

컨텍스트 생성 자체가 비용이라 수백 mb의 gpu메모리를 먹고 수백 밀리초가 된다 그래서 서빙프로세스는 기동시 한 번 만들고 계속 사용한다.

### 한 GPU에 여러 프로세스가 붙을 때

기본 설정에서는 여러 프로세스가 같은 gpu에 컨텍스트를 만들 수 있다 다만 **동시에 계산하지는 않는다.** 드라이버가 시분할로 컨텍스트를 번갈아 줄인다.

```
기본 (시분할)     프로세스 A [계산] → 컨텍스트 전환 → 프로세스 B [계산] → ...
                 전환 비용이 있고, 각자 메모리를 따로 잡음

MPS              여러 프로세스의 커널이 하나의 컨텍스트에서 동시 실행
                 전환 비용 없음. 격리는 약함

MIG              GPU를 하드웨어 수준에서 쪼갬
                 메모리와 연산 유닛이 물리적으로 분리. 격리 강함
                 RTX PRO 6000 서버 에디션은 24GB 인스턴스 4개까지
```

**서빙에서는 대개 셋 다 안쓰고** gpu하나에 서빙 프로세스를 하나 물려 그 프로세스가 내부에서 요청을 배칭한다 컨텍스트 전환보다 배칭이 효율적이기 때문이다.

<br>

## 프로세스가 gpu를 잡는 순서

### 기동 시퀀스

서빙 프로세스가 뜰 때 벌어지는 일을 순서대로 보면 이렇다

```
1. 프로세스 시작
   libcuda.so 로드, /dev/nvidiactl 열기

2. 장치 열거
   몇 장이 보이는지, 각각 컴퓨트 능력과 메모리가 얼마인지 조회

3. 컨텍스트 생성
   /dev/nvidia0 열기, GPU 가상 주소 공간 확보
   → 이 시점부터 nvidia-smi에 프로세스가 보임

4. 가중치 로드
   디스크 → 호스트 메모리 → GPU 메모리
   safetensors를 mmap으로 읽고 레이어 단위로 복사

5. 메모리 프로파일링
   더미 입력으로 포워드를 한 번 돌려 활성값 최대 사용량 측정

6. KV 캐시 풀 선점
   (전체 메모리 × 사용률) − 가중치 − 활성값 = KV 몫
   그만큼을 통째로 잡아두고 내부에서 블록 단위로 재분배

7. CUDA 그래프 캡처
   자주 쓰는 배치 크기별로 커널 실행 시퀀스를 미리 녹화

8. HTTP 서버 시작
   소켓 바인드, 요청 수락 시작
```

### 기동 로그 읽기

vLLM 기동 로그 핵심 줄들

```log
INFO  Loading model weights took 52.3021 GB
      → 4단계. 가중치가 실제로 차지한 양

INFO  Memory profiling results:
      total_gpu_memory=95.00GiB
      model_weights=52.30GiB
      non_torch_memory=0.85GiB      ← 컨텍스트, 커널 코드 등
      PyTorch_activation_peak=3.12GiB
      gpu_memory_utilization=0.90
      → 5단계. 무엇이 얼마를 먹는지 분해

INFO  GPU KV cache size: 29,184 tokens
INFO  Maximum concurrency for 32,768 tokens per request: 0.89x
      → 6단계. 남은 몫으로 몇 토큰을 담을 수 있는지
      → 0.89x는 32K 요청 하나도 다 못 담는다는 경고

INFO  Capturing CUDA graphs: 100%|██████| 35/35
      → 7단계. 배치 크기 35종에 대해 녹화

INFO  Starting vLLM API server on http://0.0.0.0:8000
      → 8단계. 이제 요청을 받음
```

`Maximum concurrency`가 1.0 미만이면 설정이 잘못된 것이다 이 값은 **KV 캐시 몫 요청 하나가 요구하는 길이로 나눈 값으로** 이 서버가 최대 컨텍스트짜리 요청을 몇 개까지 동시에 담을 수 있는지를 뜻한다

```
KV 캐시 토큰 수 ÷ max-model-len = Maximum concurrency

29,184 ÷ 32,768 = 0.89x
```

값에 따라 상태가 셋으로 갈린다

```
1.0 미만    최대 길이 요청 하나도 못 담음
            → 그런 요청이 오면 거부되거나 잘림
            → 짧은 요청만 오면 당분간 돌지만, 긴 요청이 오는 순간 터짐

1.0 ~ 2.0   요청 하나는 담기지만 동시 처리가 사실상 안 됨
            → 배칭이 안 되니 GPU가 놀고 처리량이 바닥

4.0 이상    여러 요청을 겹쳐 처리 가능. 배칭이 의미를 가짐
```

1.0 미만인데도 서버가 뜨는 경우가 있어서 위험하다 실제 요청이 대부분 2k토큰이면 한동안 문제없이 돌다가 30K짜리 들어오면 그때 처음 실패하게 되는것이다

개발 환경에서 안걸리고 운영에서 걸릴수있는 전형적인 안티패턴이다.

컨텍스트 길이를 줄이거나 양자화를 걸거나 gpu를 늘려야한다.

```
분자를 키운다   양자화로 가중치를 줄여 KV 몫 확보     효과 큼
               --gpu-memory-utilization 상향        효과 작음
               KV 캐시 자체를 FP8로                  약 2배
               GPU를 늘려 TP                        선형

분모를 줄인다   --max-model-len 하향                 실제 필요 길이를 확인하고
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

- **CUDA Version:** 드라이버가 지원하는 최대 cuda버전으로 설치된 툴킷 버전이 아니다 이걸 툴킷 버전으로 오해하는 경우가 흔하다
- **Memory Usage:** 컨테이너 안에서 실행해도 호스트 전체 기준으로 나온다 다른 컨테이너가 쓰는 몫까지 합산되어 보인다
- **GPU-Util** 지난 샘플링 구간에 커널이 하나라도 돌고있던 시간의 비율로 얼마나 많은 코어가 일햇는가가 아닌 커널 하나가 SM하나만쓰여 계속 돌아도 100퍼로 표시된다
- **Compute M:** Default면 여러 프로세스가컨텍스트를 만들수있다 Exclusive_Process면 하나만 가능하다
- **Processes:** gpu에 cuda 컨텍스트를 만든 프로세스 목록이다 각 행이 프로세스 하나고 마지막열은 그프로세스가 잡은 gpu메모리다. 컨테이너 안에 PID 네임스페이스가 달라 이 목록이 비어보이거나 PID로 나온다. Type 열은 용도를 나타내고. C는 연산(CUDA), G는 그래픽, C+G는 둘 둘다. 서빙 프로세스는 C로 나타낸다.

GPU-Util의 의미를 오해하면 안된다. 94%인데 처리량이 낮다면 코어가 놀고있을 가능성이 크고 실제 활용도를 보려면 별도 지표가 필요하다

```bash
# SM 점유율과 메모리 대역폭까지
nvidia-smi dmon -s pucvmet -d 1

# 항목
#   sm    SM 활성 비율
#   mem   메모리 대역폭 사용 비율
#   pwr   전력
#   mclk/pclk  메모리·코어 클럭
```

<br>

## 컨테이너로 띄우기

### 문제 - 컨테이너 안에 드라이버가 없음

컨테이너는 커널 호스트와 공유하되 파일시스템은 격리하기에 컨테이너 안에는 `/dev/nvidia*`도 `libcuda.so`도 없다.

드라이버를 이미지에 넣는 방법은 쓰지 못한다 `libcuda.so`는 호스트의 커널 모듈 버전과 정확히 맞아야하는데, 이미지에 박아두면 호스트 드라이버를 업데이트하는 순간 깨진다

### 해결법

NVIDIA Container Toolkit이 컨테이너 시작 시점에 호스트의 장치 노드와 드라이버 라이브러리를 컨테이너 안으로 밀어넣는다.

```
호스트                            컨테이너 (주입 후)
/dev/nvidia0            ──────→  /dev/nvidia0
/dev/nvidiactl          ──────→  /dev/nvidiactl
/dev/nvidia-uvm         ──────→  /dev/nvidia-uvm
libcuda.so.580.178.04   ──────→  /usr/lib/.../libcuda.so
nvidia-smi              ──────→  /usr/bin/nvidia-smi

이미지가 들고 오는 것
  libcudart.so, cuBLAS, cuDNN, PyTorch, 서빙 엔진
```

드라이버층은 호스트에서 주입되고 런타임층은 이미지에 들어있다

그래서 이미지에 cuda 툴킷을 넣는것은 정상이고 드라이버를 넣는 것은 잘못인 것이다.

```bash
# 툴킷 설치
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey \
  | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg
curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list \
  | sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' \
  | sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
sudo apt update && sudo apt install -y nvidia-container-toolkit

# 도커에 런타임 등록
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker

# 확인
docker run --rm --gpus all nvidia/cuda:12.9.2-base-ubuntu22.04 nvidia-smi
```

드라이버를 업데이트할때마다 `nvidia-ctk runtime configure`를 다시 돌리고 도커를 재시작해야한다 안하면 `--gpus` 호출이 실패하기 시작한다 이게 이 스택의 상시 유지보수 항목

### CDI, 최근의 표준 경로

`--gpus` 플래그는 도커 전용 훅으로 동작한다. 요즘은 CDA(Container Device Inference)라는 벤더 중립 명세로 옮겨가는중이고 k8s, podman, docker가 이쪽을 쓴다

```bash
# CDI 명세 생성 (드라이버 설정이 바뀌면 다시 생성)
sudo nvidia-ctk cdi generate --output=/etc/cdi/nvidia.yaml

# 사용
docker run --rm --device nvidia.com/gpu=0 <이미지> nvidia-smi
```

명세 파일에 이 장치를 쓰려면 어떤 노드와 라이브러리가 필요한지 적어두고 CDI를 아는 런타임이 그걸 읽어 주입한다. `/var/run/cdi`에 생성했다면 재부팅 시 지워지므로 `/etc/cdi`를 쓰는편이 낫다

### 배포 - 서빙 컨테이너 기동

```bash 
docker run --rm -d \ #일회성 검증이라 --rm넣어둠
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
# 도커 표준 플래그
-d                      백그라운드 실행. 안 붙이면 터미널이 물림
--name vllm             이름 지정. docker logs vllm 처럼 참조할 때 씀
--restart unless-stopped  프로세스가 죽으면 자동 재시작
-p 8000:8000            호스트 8000 → 컨테이너 8000 포트 연결
-v /models:/models:ro   호스트 디렉터리를 읽기 전용으로 마운트
-e HF_HOME=...          환경변수. 여기서는 모델 캐시 위치
```

`--rm`은 서빙에서 안쓴다 컨테이너가 멈추면 자동 삭제하는 플래그인데 테스트용으로 띄운거라 일단 넣어뒀다 docker logs도 볼수없으니 일회성 검증에서만 쓰자

- `--ipc=host`, `--shm-size`: 텐서 병렬이나 데이터 로더가 프로세스간 공유메모리를 쓴다 도커 기본값 64MB로는 부족해서 기동중 멈춘다
- `-v /models:ro`: 가중치 이미지에 넣으면 이미지가 수십 GB가 된다. 볼륨으로 붙이고 읽기 전용으로 건다.
- `gpus '"device=0"'`: 0번 카드만 컨테이너에 주입한다 따옴표가 두 겹인 이유는 셸이 한 겹을 먹기 때문에 --gpus all이면 따옴표가 필요없다.

<br>

## 요청 플로우

HTTP부터 gpu까지 확인해보자 프로세스는 하나인데 안에서 두 종류의 일이 돌게 된다 소켓을 받는쪽과 gpu에게 명령을 내리는 쪽이다.

```
클라이언트
   │ HTTP POST /v1/chat/completions
   ▼
[소켓 → HTTP 파서]              CPU 스레드
   │ 토큰화
   ▼
[스케줄러 큐]                    CPU
   │ 대기 중인 요청들 중 이번 스텝에 넣을 것을 고름
   │ KV 캐시 블록 할당
   ▼
[배치 구성]                      여러 요청을 하나로 묶음
   │
   ▼
[커널 런치]                      CPU가 GPU 스트림에 명령을 밀어 넣음
   │                            ← 비동기. CPU는 곧바로 반환
   ▼
[GPU 실행]                       스트림에 쌓인 커널을 순서대로 실행
   │
   ▼
[결과 동기화]                     CPU가 결과를 읽어감
   │ 디토큰화
   ▼
[스트리밍 응답]                   SSE로 토큰 하나씩 전송
```

**커널런치가 비동기라는 점이 구조의 핵심인데** cpu는 gpu 명령을 넣고 바로 돌아와 다음 배치를 준비한다

gpu가 현재 배치를 계산하는동안 cpu가 다음 배치의 스케줄링이 끝나면 gpu는 쉬지않고 이어서 일을 한다

반대로 cpu쪽이 느리면 gpu가 명령을 기다리며 논다 작은 모델일수록 이 현상이 잘나타난다 커널 하나가 3ms인데 스케줄링이 3ms면 gpu가 절반말 일을 한다

### 개념 - 스트림

스트림은 gpu에 보내는 명령큐로 같은 스트림에 넣은 커널은 순서대로 실행되고, 다른 스트림끼리는 겹칠 수 있다.

```
스트림 0  [커널A][커널B][커널C]        순서 보장
스트림 1        [복사D][커널E]         스트림 0과 동시 진행 가능
```

서빙 엔진은 보통 계산용과 ㅅ복사용 스트림을 나눠 가중치가 kv를 옮기는 동안 계산이 멈추지 않게 한다

### 포트가 열려도 준비 안된 상태 함정

기동 시퀀스에서 http서버가 마지막이라고 했지만, 첫 요청은 여전히 느리다. 캡처되지 않은 배치크기가 들어오거나 아직 안 쓴 커널이 jit컴파일 되기 때문이다.

```
첫 요청        수 초. JIT 컴파일과 캐시 미스
10~30 요청     점차 안정화
이후           정상 성능
```

**벤치마크할때 워밍업을 안하면 이 구간을 오염시키니** 배포 시에 헬쳌 통과한 직후 트래픽을 밀어넣으면 초기 지연이 튄다

<br>

## 학습을 돌릴 때 달라지는 것

### 메모리 구성의 변화

서빙과 학습은 gpu 메모리를 쓰는 방식이 다르다

```
서빙                          학습
─────────────────────────────────────────────────
가중치                        가중치
활성값 (작음)                  활성값 (역전파용으로 전부 보관)
KV 캐시 (대부분)               그래디언트 (가중치와 같은 크기)
                             옵티마이저 상태 (Adam이면 가중치의 2배)
```

**Adam으로 전체 파인튜닝을 하면 가중치의 네 배가 필요하다**

가중치 자체, 그래디언트, 그리고 옵티마이저의 1, 2차 모멘트다 27B모델을 BF16으로 튜닝하려면 54GB x 4 = 216GB이고, 96GB 카드 한장으로는 안된다

그래서 단일 카드에서는 대개 LoRA를 사용한다. 원본 가중치를 얼려두고 작은 어댑터 만으로 학습하므로 그래디언트와 옵티마이저의 상태가 어댑터 크기로 줄어든다.

```
27B 모델, 96GB 카드

전체 파인튜닝 (Adam, BF16)    216GB  불가
LoRA (BF16 베이스)             약 60GB  가능
QLoRA (4비트 베이스)           약 25GB  여유 있음
```

### 실행 패턴의 차이

```
서빙    요청이 산발적으로 들어옴. 배치 구성이 계속 바뀜
        지연이 지표. 스케줄러가 복잡함

학습    데이터가 미리 있음. 배치가 고정
        처리량만 지표. 파이프라인을 꽉 채울 수 있음
```

학습에서 파이프라인 병렬이 쓸만하고 추론에서 잘 안쓰이는 이유인데 미리 쪼개 밀어넣을 데이터가 있으면 gpu가 노는 구간을 메울 수 있다.

### 배포 - 학습 컨테이너

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
--rm -it              한 번 쓰고 버리는 대화형 셸. 서빙과 달리 상주 프로세스가 아님
--gpus all            보이는 카드를 전부 주입. 학습은 대개 여러 장을 함께 씀
--ipc=host            워커 프로세스 간 공유 메모리. 데이터 로더가 씀
--shm-size=32g        서빙(16g)보다 크게. 데이터 로더 워커가 배치를 여기 올림
--ulimit memlock=-1   페이지 고정 메모리 제한 해제
--ulimit stack=64MB   스레드 스택 크기 상향
-v /data -v /out      데이터셋 입력과 체크포인트 출력
```

`--ulimit memlock`이 학습 전용 항목으로 gpu가 cpu메모리를 직접 읽으려면 그 페이지가 디스크로 스왑되지 않아야하고 이런 메모리를 페이지 고정(pinned) 메모리라고 한다. 통신 라이브러리와 데이터 로더가 이걸 크게잡는데 리눅스 기본 제한이 64kb라 바로 걸린다

```
$ ulimit -l
64                    ← KB 단위. 64KB

증상   NCCL 초기화 실패, 또는 "cannot allocate pinned memory"
원인   요구량이 제한을 넘음
대응   -1 로 제한 해제
```

`stack`은 워커 스레드가 깊은 재귀나 큰 지역변수를 쓸때 기본 8mb로 부족한 경우가 있어 올려둔다 67108864는 64mb이다.


<br>

## 진단

### 증상별 확인 순서

**컨테이너에서 gpu가 안보인다면**

```bash
nvidia-smi                                    # 호스트 드라이버 확인
docker info | grep -i runtime                 # nvidia 런타임 등록 확인
docker run --rm --gpus all nvidia/cuda:12.9.2-base-ubuntu22.04 nvidia-smi
```

세 단계중 어디서 끊기는지 원인이 갈리는데 첫 단계가 실패하면 드라이버, 둘째면 툴킷 설정, 셋째면 이미지나 cdi명세다 드라이버를 최근에 업데이트했다면 `nvidia-ctk runtime confiugre`를 다시 돌린다.

**메모리가 부족하다고 뜰때**

```bash
nvidia-smi --query-compute-apps=pid,used_memory --format=csv
```

죽은 프로셋가 메모리를 붙들고있는 경우가 있다 컨테이너를 지웠는데도 메모리가 안돌아가면 호스트에 좀비 프로세스가 남은것이다

**느린데 gpu-util은 높다면**

`nvidia-smi dmon -s pum`으로 `sm`열을 볼때 gpu-util이 90%대인데 `sm`이 낮으면 gpu를 제대로 못 채우고있다는 뜻이다 배치가 너무 작거나 cpu쪽 스케줄링이 병목이다.

**시간이 지날수록 느려진다면**

```bash
nvidia-smi -q -d PERFORMANCE | grep -A5 "Clocks Throttle"
```

`SW Power Cap` or `HW Thermal Slowdown`이 Active면 전력 온도 제한에 걸린것으로 600w 카드를 쓰면서 케이스 흡배기가 부족하면 흔하다.

**Xid Error**

```bash
sudo dmesg | grep -i xid
```

xid는 드라이버가 커널 록에 남기는 하드웨어 오류 코드로 번호마다 의미가 다르고 반복되면 하드웨어나 드라이버 문제일 가능성이 크다

#### RTX PRO 6000 Blackwell 쓸때 알아둘 것

워크스테이션급 blackwell은 컴퓨트 능력이 sm_120으로, 데이터 센터 blackwell(sm_100)과 다르다. 번호가 크다고 상위호환이 아니다

```
sm_100용으로 컴파일된 커널은 sm_120에서 안 돎
→ 프레임워크가 느린 폴백 경로를 타거나 실행이 실패
→ NVFP4 MoE 커널이 Marlin 백엔드로 떨어지는 사례가 보고됨
```

카드 자체 스펙은 이렇다

```
메모리        96GB GDDR7 ECC, 512비트
대역폭        1,792 GB/s (워크스테이션) / 1,597 GB/s (서버)
TDP          400~600W 구성 가능
MIG          서버 에디션은 24GB 인스턴스 4개까지
실사용 가능    드라이버 오버헤드 제외 약 86GB
```

**공식 서빙 이미지가 sm_120을 제대로 지원하는지 먼저 확인하는 편이 안전하다**

8B모델을 FP8로 올렸는데 90gb를 먹는다면 양자화 커널이 폴백된거고 이미지나 빌드 옵션을 바꿔야한다

<br>

## 가상 실습: 서빙

지금까지 내용을 토대로 실습을 진행해보자

```
장비    Ubuntu 24.04, RTX PRO 6000 Blackwell 96GB 1장, RAM 128GB
목표    Qwen3.6-27B를 8000 포트에 띄우고 요청이 도는 것까지 확인
```

### 1단계 - 아무것도 없는상태

```bash
$ nvidia-smi
Command 'nvidia-smi' not found

$ lspci | grep -i nvidia
01:00.0 VGA compatible controller: NVIDIA Corporation Device 2bb1 (rev a1)
```

카드는 꽂혀있고 드라이버가 없다 `lspci`에 보이는데 `nvidia-smi`가 없으면 이 상태다

설치가능한 드라이버 목록을 먼저 보자

```bash
$ ubuntu-drivers devices
vendor   : NVIDIA Corporation
model    : RTX PRO 6000 Blackwell
driver   : nvidia-driver-580-server - distro non-free recommended
driver   : nvidia-driver-580        - distro non-free
driver   : nvidia-driver-575-server - distro non-free
```

```
580        드라이버 브랜치 번호. 클수록 최신이고 새 GPU를 지원
           Blackwell 계열은 570 이상이어야 인식됨

-server    데이터센터용 변형
            · 디스플레이 출력 관련 구성 요소가 빠짐
            · 브랜치가 더 오래 유지보수됨
            (없는 쪽은 데스크톱용. 그래픽 스택이 함께 들어감)

recommended  배포판이 이 카드에 권장하는 것
```

헤드리스 서버에 서빙만 올릴거면 `-server`로 해야한다 모니터를 붙여 쓰거나 워크스테이션으로도 쓴다면 `-server`가 없는쪽을 고른다

```bash
$ sudo apt update && sudo apt install -y nvidia-driver-580-server
$ sudo reboot
```

### 2단계 - 드라이버 확인

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

장치파일이 생겼다 4MiB는 드라이버 자체가 쓰는 몫이고 아무도 컨텍스트를 안만든 상태다.

지속 모드를 켜둔다 안켜면 프로세스가 없을 때 드라이버가 언로드되어 다음 기동이 몇 초 느려진다

```bash
$ sudo nvidia-smi -pm 1
Enabled persistence mode for GPU 00000000:01:00.0.
```

### 3단계 - 도커와 툴킷

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

마지막 명령에서 호스트와 같은 출력이 나오면 주입이 정상이다.

### 4단계 - 첫 시도, 공유메모리 부족

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

도커 기본 `/dev/shm`이 64MB로 가중치 로딩이나 워커간통신 공유 메모리를 쓰는데 여기서 막힌다

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

기동이 5단계 프로파일링 까지 갔다가 6단계에서 멈췄다 

```
전체            95.00 GB
사용률 0.90     85.50 GB 까지만 씀
  가중치        54.12
  컨텍스트 등    0.91
  활성값 최대    4.38
  ────────────────────
  KV 몫         26.09 GB  →  8,192 토큰

요청 하나가 32,768 토큰을 요구
8,192 ÷ 32,768 = 0.25x  →  요청 하나도 못 담음
```

BF16으로 올렸더니 가중치만 54GB다. 세 가지 선택지가 있는데

```
컨텍스트를 줄인다        --max-model-len 8192   → 요청 하나만 겨우
사용률을 올린다          0.90 → 0.95            → 약 4.7GB 추가. 부족
가중치를 줄인다          FP8 양자화             → 54GB → 27GB
```

세 번째가 유일하게 의미있는 폭이다

### 6단계 - 양자화

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

가중치가 절반이 됨녀서 kv몫이 26gb에서 60gb로 늘었고 토큰이 8,192개에서 176,128로 21배가 되엇다 `5.31x`는 32k 요청 다섯개를 동시에 받을 수 있다는 뜻이다.

`init engine took 94.22 seconds`중 47초가 그래프 캡처다 재기동이 잦은 환경이면 이시간을 감안해야한다.

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
87.9 GB 점유    가중치 27 + KV 풀 60 + 나머지. 요청이 없어도 이미 잡혀 있음
GPU-Util 0%     메모리는 잡았지만 계산은 안 하는 중
전력 78W        유휴. 부하가 걸리면 400W대로 올라감
```

**KV 풀은 미리 통째로 잡고 내부에서 재분배한다 요청 없이도 87GB가 잡혀보이는게 정상**

### 8단계 - 요청 보내기

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

4.8초는 정상이 아닙니다. 다시 보내면 이렇게 된다.

```bash
$ time curl -s ... | jq -r '.choices[0].message.content' > /dev/null
real    0m2.103s

$ time curl -s ... | jq -r '.choices[0].message.content' > /dev/null
real    0m2.088s
```

첫 요청의 2.7초 초과분이 jit 컴파일과 캐시 워밍이다 벤치마크나 헬스체크를 설계할때 이 구간을 빼야한다.

### 9단계 - 부하를 주고 관찰

터미널 하나에 모니터를 띄운다

```bash
$ nvidia-smi dmon -s pucm -d 2
# gpu   pwr  gtemp   sm   mem   enc   dec  mclk  pclk
    0    76     41     0     0     0     0  1500  2100
```

다른 터미널에서 부하를 걸어보자

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

`sm 97` `mem 71`값을 보면 sm이 거의 포화고 메모리 대역폭도 70퍼대면 gpu를 제대로 쓰고있는 상태로 만약 sm이 30퍼대인데 gpu util이 100퍼대로 나오면 커널이 gpu를 못채우고 있다는 신호로 알아두자

벤치마크 결과

```
============ Serving Benchmark Result ============
Successful requests:                     300
Request throughput (req/s):              3.41
Output token throughput (tok/s):         1746.2
Median TTFT (ms):                        387.4
Median ITL (ms):                         13.2
==================================================
```

### 10단계 - sm_120 폴백 확인

여기까지 왔음ㄴ 도는것은 확인됐고 남은것은 제 성능이나오는지 이다.

워크스테이션 blackwell은 커널이 느린 경로로 떨어지는 경우가 있다

```bash
$ docker logs vllm 2>&1 | grep -iE "backend|fallback|marlin|capability"
INFO  Detected device capability: 12.0 (sm_120)
INFO  Using MarlinLinearKernel for FP8 quantization
WARNING  Native FP8 kernels unavailable for sm_120, falling back
```

이 로그가 보이면 전용 커널 대신 범용 경로를 타고있는 것으로 카드가 낼 수 있는 성능의 일부만 쓰는 상태고 확인할 것은 셋이다.

```
□ 이미지 버전이 sm_120을 지원하는 릴리스인가
□ 그 조합에 맞는 양자화 형식인가 (FP8 / NVFP4 / AWQ 중)
□ 소스 빌드 시 TORCH_CUDA_ARCH_LIST에 12.0이 들어갔는가
```

**판단 기준을 하나로 두면 편한데** 8b급 모델을 fp8로 올렸는데 20gb를 넘게 안걸린것.

### 총 정리 - 실습에서 거른것들

```
증상                        원인                     확인 지점
──────────────────────────────────────────────────────────────
shared memory 오류          도커 기본 shm 64MB       --ipc=host 또는 --shm-size
Maximum concurrency 0.25x   가중치가 KV 몫을 잠식     프로파일링 로그의 분해 항목
첫 요청 4.8초               JIT 컴파일, 캐시 미스     두 번째 요청과 비교
GPU-Util은 높은데 느림       커널이 GPU를 못 채움      dmon의 sm 열
FP8인데 메모리를 많이 먹음    sm_120 커널 폴백         기동 로그의 backend 줄
```


<br>

## 명령어 모음

```bash
# 장치와 드라이버
nvidia-smi                                    # 전체 상태
nvidia-smi -L                                 # GPU 목록과 UUID
nvidia-smi -q -d MEMORY,POWER,TEMPERATURE     # 항목별 상세
nvidia-smi topo -m                            # GPU 간 연결 토폴로지

# 실시간 모니터링
nvidia-smi dmon -s pucvmet -d 1               # 1초 간격
nvidia-smi --query-gpu=timestamp,utilization.gpu,memory.used,power.draw \
  --format=csv -l 1                           # CSV로 로깅

# 프로세스
nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv
sudo fuser -v /dev/nvidia*                    # 장치를 붙들고 있는 프로세스

# 컨테이너
docker run --rm --gpus all <이미지> nvidia-smi
sudo nvidia-ctk runtime configure --runtime=docker
sudo nvidia-ctk cdi generate --output=/etc/cdi/nvidia.yaml

# 전력 제한 (전력당 성능을 올리고 싶을 때)
sudo nvidia-smi -pl 450                       # 와트

# 지속 모드 (기동 지연 감소)
sudo nvidia-smi -pm 1
```