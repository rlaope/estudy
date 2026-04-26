# Triton Inference Server config.pbtxt 튜닝

질문으로 시작해보자,

수백 명의 사용자가 동시에 모바일 앱에서 **이미지 분석** 버튼을 누른다고 가정하자.

api 서버로 수백 개의 개별 요청이 들어올거고, 이 개별 요청들은 gpu에 하나씩 순서대로 밀어넣게 된다면?

막대한 연산 코어를 가진 비싼 gpu가 제대로 힘도 써보지 못하고 텅 빈 상태로 돌아가게 된다.

서버 측에서 개별 사용자들의 요청을 모아 하나의 거대한 덩어리로 만든뒤에 gpu에 던져줘야 효율적으로 다 꼼꼼하게 촘촘하게 쓸텐데...

이 낭비를 막고자 gpu의 throughtput를 한계치까지 끌어올리기위해 사용하는 기술이 **NVIDIA Triton Inference Server의 Dynamic Batching**이다.

- **Triton Inferernce Server**: NVIDIA 에서 개발한 오픈소스 ai 모델 서빙 플랫폼으로 TensorRT, ONNX, Pytorch등 다양한 포맷의 모델을 하나의 서버에서 로드하고 HTTP/gRPC 인터페이스로 클라이언트와 통신하며 자원 할당을 최적화한다.
- **GPU의 SIMT(Single Instruction, Multiple Threads) 아키텍처**: gpu는 수천개의 작은 코어로 이루어져있어 행렬 곱셈을 병렬로 처리하는데 특화되어 있다. 데이터가 1개 (B = 1)이든 16개이든 (B = 16) 이든 연산에 걸리는 물리적 시간은 거의 동일하다. 즉, 1개씩 16번 연산하는 것보다 16개를 한 번에 연산하는 것이 압도적으로 빠르다.
- **Dynamic Batching**: 클라이언트들은 각자 데이터 1개짜리 개별 요청을 비동기적으로 보내지만, 서빙 서버 Triton 내부의 스케줄러가 이 요청을 아주 짧은 시간(5ms) 동안 queue 에 모았다가 하나으 큰 텐서로 결합하여 gpu에 전달하는 기술이다.

<br>

## 문제 정의

클라이언트의 비동기적이고 개별적인 요청 패턴과 gpu의 대규모 병렬 처리 아키텍처 사이에 미스 매치가 존재했고, 요청을 1개씩 순차적으로 처리할 경우 gpu 활용률이 10% 미만으로 떨어지며, 대규모 트래픽에서 보틀낵이 발생하는 한계가 있었다.

ex.) "클라이언트 100명이 동시에 요청을 보냈을 때, 모델 1개의 요청을 추론하는데 10ms가 걸린다면 100번째 사용자는 앞선 99개의 요청이 처리될때까지 990ms를 큐에서 대기해야만 응답을 받을 수 있는 치명적인 지연이 발생하는 것이다."

### 해결 방식

- **스케줄러 개입 및 지연시간 허용**: Triton의 config.pbtxt 파일에서 dynamic_batching 옵션을 활성화하여 서버는 첫 요청이 들어오면 즉시 gpu로 보내지 않고 최대 지연 시간 `max_queue_delay_microseconds` 만큼 기다린다.
- **배치 최적화 (trade-off 조율)**: 기다리는 동안 목표한 최적의 배치 사이즈 `preferred_batch_size`에 도달하면 시간이 다 차지 않아도 즉시 gpu로 묶어서 전송한다. 이를 통해 **단일 요청 지연시간을 미세하게 희생하는 대신 시스템 전체 초당 처리량을 수십배 향상시키는 최적화 전략을 취한 것이다.**

<br>

## 상세 동작 원리 및 구조화 

각기 다른 시점에 도착한 개별 요청들이 Triton 큐에서 결합되어 단일 배치로 gpu 커널에 인가되는 논리적 흐름을 보자.

```mermaid
graph TD
    Client1[Client A: req 1] -->|t=0ms| TritonQueue[Triton Dynamic Batcher Queue\n(max_delay: 5ms, pref_batch: 4)]
    Client2[Client B: req 1] -->|t=1ms| TritonQueue
    Client3[Client C: req 1] -->|t=2ms| TritonQueue
    Client4[Client D: req 1] -->|t=4ms| TritonQueue
    
    TritonQueue -->|Batch Size = 4 달성 시 즉시 방출| BatchTensor[Batch Tensor Array\nShape: 4 x Dims]
    
    subgraph "Target GPU (e.g., TensorRT Engine)"
        BatchTensor --> GPU[GPU Tensor Cores\n단 1회의 행렬곱 연산 수행]
    end
    
    GPU -->|결과 분리 (Scatter)| TritonQueue
    TritonQueue -.->|응답 1| Client1
    TritonQueue -.->|응답 2| Client2
    TritonQueue -.->|응답 3| Client3
    TritonQueue -.->|응답 4| Client4
```

가장 기본적인 `config.pbtxt` 구조를 보자.

Triton이 모델을 로드하기 위해 반드시 읽어야하는 모델 설정 파일의 뼈대를 보면, (동적 파일 아직 없음)

```proto
# 모델 이름 (폴더명과 일치해야 함)
name: "resnet50_trt"
# 백엔드 엔진 지정 (TensorRT)
platform: "tensorrt_plan"
# 이 모델이 최대로 처리할 수 있는 배치 사이즈
max_batch_size: 32

# 모델의 입력 텐서 형태 (Batch 차원은 Triton이 알아서 관리하므로 생략하거나 -1로 처리)
input [
  {
    name: "input_tensor"
    data_type: TYPE_FP32
    dims: [ 3, 224, 224 ]
  }
]

# 모델의 출력 텐서 형태
output [
  {
    name: "output_tensor"
    data_type: TYPE_FP32
    dims: [ 1000 ]
  }
]
```

dynamic batching 및 gpu 동시성 극대화 설정도 보면

실제 프로덕션 환경에서 gpu 연산 효율을 한계까지 끌어 올리기위해 dynamic_batching, instance_group 파라미터를 세밀하게 튜닝한 구성이다.

```proto
name: "resnet50_trt"
platform: "tensorrt_plan"
max_batch_size: 64

input [ { name: "input", data_type: TYPE_FP32, dims: [ 3, 224, 224 ] } ]
output [ { name: "output", data_type: TYPE_FP32, dims: [ 1000 ] } ]

# [핵심 1] 동적 배칭(Dynamic Batching) 설정
dynamic_batching {
  # 큐에서 대기할 수 있는 최대 시간 (단위: 마이크로초). 
  # 예: 5000 = 5ms. 5ms가 지나면 배치가 덜 찼어도 무조건 GPU로 보냄.
  max_queue_delay_microseconds: 5000
  
  # TensorRT 엔진이 가장 효율적으로 연산할 수 있는 추천 배치 사이즈들.
  # 큐에 요청이 4개, 8개, 16개 쌓이는 순간 지연시간(5ms)이 안 지났어도 즉시 실행함.
  preferred_batch_size: [ 4, 8, 16, 32, 64 ]
}

# [핵심 2] 인스턴스 그룹 (Instance Group) 설정
# 하나의 GPU에 동일한 모델의 복제본(Instance)을 여러 개 띄워, 
# 데이터 복사(Host -> Device)와 추론 연산을 파이프라이닝(Pipelining)하여 GPU 유휴 상태를 없앰.
instance_group [
  {
    count: 2             # 메모리가 허락한다면 복제본을 2개 띄움
    kind: KIND_GPU       # GPU 메모리에 적재
    gpus: [ 0 ]          # 0번 GPU 사용
  }
]

# [선택] Model Warmup 설정
# 서버가 켜질 때 가짜 데이터를 한 번 흘려보내어 GPU 커널 초기화(Cold Start) 지연을 방지함
model_warmup [
  {
    name: "warmup_request"
    batch_size: 1
    inputs: {
      key: "input"
      value: { data_type: TYPE_FP32, dims: [ 3, 224, 224 ], zero_data: true }
    }
  }
]
```

위에서 `preferred_batch_size`는 해당 배치사이즈가 되면 바로 실행해버린다는 선호를 나타낸건데 여기서 궁금증이 생길수있다 4에서 실행시키는데 8에도 실행시킨다치면 8이되려면 4는 필연적으로 통과해야하는거 아닌가?

실제 대용량 트래픽 환경에서 인퍼런스 요청이 많이 들어오면 요청이 1개씩 이쁘게 들어오지 않는다. 4를 건너뛰고 바로 8이나 16으로 점프하는 일들이 매우 흔하게 발생한다.

찰나의 순간에 쏟아지는 트래픽 버스트때문에 수백만의 사용자가 0.0001초 차이로 동시에 버튼을 눌러 혹은 앞단 api서버가 10개짜리 데이터를 한번에 던지거나 할수있기때문이다. 

gpu가 일하는동안 쌓이는 대기열은 앞서 출발한 4개짜리 묶음을 gpu가 열심히 계산하는동안 10ms가 소요된다 치자, Triton은 다음 요청들을 계속 큐에 쌓고있다 gpu가 계산을 막 끝내고 다음 일거리주세요 하는 시점에 확인해보면 그 10ms 사이에 이미 큐에 20개의 요청이 밀려있을 수 있다. 그러면 바로 16개짜리 묶음을 만들어서 gpu에 던져준다.

결론적으로 저 숫자 배열은 1개씩 세면서 차례대로 기다리겠다는 뜻이 아니라 gpu가 일할 준비가된 딱 그 순간 큐를 열어보고 그 안에 쌓여있는 물량에 맞춰 저 숫자 단위로 뭉텅뭉텅 썰어서 가져가겠다는 기준점 역할을 한다.