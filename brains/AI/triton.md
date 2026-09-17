# Triton Inference Server

동적 배치로 단일 모델의 처리량을 높였다. 하지만 실제 서비스가 커지면서

이미지 검색용 CLIP 모델, 유해 이미지 필터링 모델, 상품 추천 모델 등 여러 개의 ai 모델을 동시에 운영해야 하는 상황이 발생했다.

각각의 모델을 별도의 서버 프로세스로 띄우면 GPU 메모리 파편화가 발생하고 인프라 비용이 폭증한다.

단일 GPU 자원을 여러 개의 서로 다른 모델이 충돌 없이 공유하며, 프로세스간 통신 오버헤드까지 완전히 제거하려면 어떻게 아키텍처를 설계해야할까?

이러한 다중 모델 환경의 운영 병목을 해결하기위해 Triton Inference Server가 제공하는 핵심 아키텍처인 Concurrent Execution, Shared Memory 통신 기술들을 다룬다.

- **Concurrent Model Execution (다중 모델 동시 실행)**: 단일 Triton 프로세스 내에서 ONNX, TensorRT, PyTorch 등 서로 다른 프레임워크로 작성된 여러 모델을 동시에 메모리에 올려두고 GPU의 남는 스트림자원을 쪼개어 병렬로 추론을 실행하는 기능이다.
- **Model Ensemble (모델 앙상블)**: 서버 내부에서 여러 모델의 추론 파이프라인(ex. 전처리 모델 -> 추론 모델 -> 후처리 모델)을 논리적인 하나의 가상 모델로 묶어, 클라이언트가 한 번의 api 호출만으로 전체 파이프라인을 실행하게 만드는 기능이다.
- **Shared Memory (공유 메모리)**: 클라이언트 웹 서버와 Triton 서버가 동일한 물리적 호스트에 있을 경우 데이터를 네트워크 패킷으로 변환하여 전송하지 않고 os 레벨의 시스템 메모리 RAM 주소만 교환하여 데이터를 읽는 zero-copy 통신 기법이다.

<br>

## The Problem

좀 더 구체화 해서 알아보고 해결하고자 한 문제를 알아보자

- **GPU 활용도 저하 및 메모리 파편화**: 여러 개의 ai 모델을 운영할 때, 각 모델마다 독립된 컨테이너나 서버 프로세스를 할당하면 각 프레임워크 런타임 (ex. pytorch 프로세스)이 GPU 메모리 VRAM을 개별적으로 선점해버린다. 정작 연산이 필요할 때는 가용 메모리가 부족해지거나, 한 모델이 쉬고 있을 때 다른 모델이 그 자원을 끌어다 쓰지 못하는 자원 낭비가 발생한다.
- **Network 직렬화 오버헤드**: 웹 서버에서 ai서버로 10mb 크기의 이미지 텐서를 HTTP or gRPC로 전송할 때 데이터 바이트 배열로 직렬화 하고 반대쪽에서 역직렬화 하는 과정에서 막대한 cpu 사이클과 지연시간 network io가 소모된다.


**즉 모델별로 서버 프로세스를 개별 구축하면 GPU 자원이 낭비되고 메모리 파편화가 발생하는 문제가 존재하며 대용량 텐서 데이터를 컨테이너간에 주고받을 때 직렬화/역직렬화에 막대한 시스템 자원과 시간이 소모되는 한계가 있었다.**

예를들면 이런 문제다, 24GB VRAM을 가진 GPU 서버에 4GB 짜리 모델 A, B, C를 각각 별도의 API 서버로 띄웠다. 모델 A에만 트래픽이 몰릴 경우 모델 B, C가 점유한 8GB의 VRAM은 놀고 있음에도 모델 A는 메모리 부족으로 서버가 다운되며 모델 A의 결과를 모델 B로 전달하기 위해 데이터를 네트워크로 복사하느라 수십 ms의 통신 지연이 발생한다.

### The Solution

문제 해결방식은 아래와 같다.

- **통합 백엔드를 통한 GPU 컨텍스트 스위칭**: 단일 Triton 인스턴스가 C++ 기반의 Backend API를 통해서 여러 프레임워크 런타임을 중앙에서 통제한다. 내부 스케줄러가 여러 모델의 연산 명령을 GPU의 여러 스트림(CUDA Stream)에 동시 할당하여 낭비되는 코어 없이 병렬 실행을 보장한다.
- **System/CUDA Shared Memory 포인터 전달**: 클라이언트가 OS의 공유 메모리 공간에서 이미지 텐서를 기록한 뒤, Triton 서버에는 데이터 자체가 아닌 공유 메모리 ID, offset 정보 만으로 몇 바이트의 문자열을 전달한다. Triton은 해당 메모리 주소에 직접 접근하여 복사 없이 데이터를 읽어 추론을 수행한다.

직관적인 예시를 보충해보면

- **기존 파이프라인 독립 서버 및 직렬 통신**은 [클라이언트 -> 네트워크 복사 -> 전처리 서버 -> 네트워크 복사 -> AI 추론 서버 -> 네트워크 복사 -> 클라이언트.] 즉 데이터가 이동할때마다 패킷을 포장하고 뜯는 작업의 반복이다.
- **Triton 앙상블 및 공유 메모리 구조**: 클라이언트가 데이터를 공유 창고에 넣고 창고 열쇠 번호만 Triton에 전달한다. Triton 내부에 구성된 앙상블 파이프라인 (전처리 -> ai 추론)은 창고 안에서 데이터를 직접 꺼내서 단일 프로세스 내에서 연속적으로 작업을 끝마친 뒤에 결과물도 창고에 넣어둔다.

<br>

## 동작 원리 구조화 

다중 모델 환경에서 Triton이 트래픽을 처리하는 내부 구조다.

1. **Model Repository(모델 저장소)**: Triton은 부팅시 지정된 폴더 repository 구조를 스캔한다. 하나의 폴더 안에 ONNX, TensorRT, Python 백엔드 등 형태가 다른 여러 모델을 디렉토리 별로 배치하면 즉시 메모리에 동시 로드된다.
2. **Ensemble Scheduler (앙상블 스케줄러)**: DAG(Directed Acyclic Graph) 형태로 모델간의 의존성을 정의한다. 클라이언트가 앙상블 모델을 호출하면 스케줄러가 알아서 데이터 플로우 (ex. Model A Output -> Model B Input)를 제어하며 내부 메모리 내에서만 텐서를 전달한다.
3. **Shared Memroy Manager**: OS 커널이 관리하는 `shm` 영역을 register하고 클라이언트가 이 영역에 데이터를 쓰고 쿼리를 날리면 Backend C API 가 해당 포인터를 참조하여 연산 하드웨어 GPU로 데이터를 직접 밀어넣는다.

### Example

원리 이해용으로 Model Repository 구조 및 앙상블 설정을 해보겠다.

Triton은 코드가 아닌 엄격한 디렉토리 구조와 `config.pbtxt`로 모델 간의 파이프라인(앙상블)을 구축한다.

이를 통해서 웹 서버는 단 한번의 호출만으로 여러 모델을 거치는 파이프라인을 실행할 수 있다.

```
# Triton Model Repository 디렉토리 구조 예시
model_repository/
├── image_preprocess_model/     # 1. 이미지 전처리 (Python 백엔드)
│   ├── 1/model.py
│   └── config.pbtxt
├── clip_encoder_model/         # 2. 이미지 임베딩 (ONNX 백엔드)
│   ├── 1/model.onnx
│   └── config.pbtxt
└── search_pipeline_ensemble/   # 3. 앙상블 가상 모델 (1번과 2번을 연결)
    ├── 1/empty.txt             # 앙상블은 로직이 없으므로 빈 파일
    └── config.pbtxt
```

```proto
# search_pipeline_ensemble/config.pbtxt
# 전처리 모델과 임베딩 모델을 하나로 묶는 앙상블 스케줄러 설정

name: "search_pipeline_ensemble"
platform: "ensemble"

# 입력: 클라이언트가 보내는 원본 바이너리 이미지
input [ { name: "RAW_IMAGE", data_type: TYPE_UINT8, dims: [ -1 ] } ]
# 출력: 최종적으로 얻게 되는 512차원 벡터
output [ { name: "FINAL_VECTOR", data_type: TYPE_FP32, dims: [ 512 ] } ]

ensemble_scheduling {
  step [
    {
      model_name: "image_preprocess_model" # 첫 번째 실행 모델
      model_version: -1
      input_map { key: "image_bytes", value: "RAW_IMAGE" }
      output_map { key: "preprocessed_tensor", value: "PREPROCESSED_DATA" }
    },
    {
      model_name: "clip_encoder_model"     # 두 번째 실행 모델
      model_version: -1
      # 이전 단계의 출력(PREPROCESSED_DATA)을 이 모델의 입력으로 직접 주입
      input_map { key: "pixel_values", value: "PREPROCESSED_DATA" }
      output_map { key: "image_embeds", value: "FINAL_VECTOR" }
    }
  ]
}
```

이어서 웹서버 api와 triton 서버가 동일한 물리적 인스턴스 (혹은 같은 공유 메모리 볼륨을 마운트한 파드)에 있을 때, 네트워크 직렬화 없이 OS 시스템 공유 메모리로 데이터를 교환하는 고성능 통신 파이썬 클라이언트 코드도 같이 보면

```py
import numpy as np
import tritonclient.grpc as grpcclient
import tritonclient.utils.shared_memory as shm
from PIL import Image

class SharedMemoryTritonClient:
    """
    네트워크 패킷 직렬화 없이 OS 공유 메모리(Shared Memory)를 사용하여
    Triton 서버와 데이터를 교환하는 초고속 클라이언트.
    """
    def __init__(self, triton_url="localhost:8001"):
        self.client = grpcclient.InferenceServerClient(url=triton_url)
        self.model_name = "clip_encoder_model"
        
        # 텐서 크기 계산 (Batch=1, C=3, H=224, W=224, float32=4bytes)
        self.byte_size = 1 * 3 * 224 * 224 * 4
        
        # 1. OS 레벨 공유 메모리 생성 및 매핑 (이름: "input_shm")
        self.shm_handle = shm.create_shared_memory_region(
            "input_shm", "/input_shm", self.byte_size
        )
        
        # 2. 생성한 공유 메모리를 Triton 서버에 등록 (Registration)
        # 이제 Triton은 "/input_shm" 이름을 통해 이 메모리 영역에 직접 접근 가능합니다.
        self.client.register_system_shared_memory(
            "input_shm", "/input_shm", self.byte_size
        )

    def request_embedding_zerocopy(self, preprocessed_image_array: np.ndarray) -> np.ndarray:
        """공유 메모리에 데이터를 쓰고 포인터 기반으로 추론을 요청합니다."""
        
        # 1. 데이터를 공유 메모리 영역에 직접 쓰기 (Copy-in)
        # 네트워크 전송이 아니라 단순히 로컬 RAM에 쓰는 과정입니다.
        shm.set_shared_memory_region(self.shm_handle, [preprocessed_image_array])
        
        # 2. Triton 입력 객체 생성 (데이터 자체를 담지 않음)
        triton_input = grpcclient.InferInput("pixel_values", preprocessed_image_array.shape, "FP32")
        
        # 핵심: 데이터 배열 대신, 등록해둔 공유 메모리 이름("input_shm")과 오프셋만 설정
        triton_input.set_shared_memory("input_shm", self.byte_size)
        
        triton_output = grpcclient.InferRequestedOutput("image_embeds")
        
        # 3. 추론 요청 
        # 페이로드에는 데이터가 없으므로 네트워크 I/O 지연 시간이 0에 가깝습니다.
        response = self.client.infer(
            model_name=self.model_name,
            inputs=[triton_input],
            outputs=[triton_output]
        )
        
        # 4. 결과 반환
        return response.as_numpy("image_embeds")

    def cleanup(self):
        """서버 종료 시 공유 메모리 할당 해제"""
        self.client.unregister_system_shared_memory("input_shm")
        shm.destroy_shared_memory_region(self.shm_handle)

# --- 실무 사용 흐름 ---
# client = SharedMemoryTritonClient()
# img_tensor = preprocess(Image.open("test.jpg"))  # [1, 3, 224, 224] float32
# 
# # 직렬화/역직렬화 오버헤드 없이 즉시 추론 결과 획득
# vector = client.request_embedding_zerocopy(img_tensor)
```

https://arxiv.org/html/2602.00053v 이 링크는 Scalable and Secure AI Inference in Healthcare: A Comparative Benchmarking of FastAPI and Triton Inference Server on Kubernetes 라는 논문으로 이곳에 벤치마킹한 결과가 있는데 

| Framework | Hardware | Batch Mode | Batch Size | p50 Latency (ms) | p95 Latency (ms) | Throughput (req/s) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **FastAPI** | CPU | None | 1 | 22 | 45 | 450 |
| **Triton** | GPU | No Batching | 1 | 28 | 52 | 420 |
| **Triton** | GPU | Dynamic | 16 | 34 | 60 | 780 |

p95 p99같은 레이턴시는 조금 더 느려졌고 dynamic batching을 적용하지 않으면 처리량도 더 떨어진 모습이지만 dynamic batching을 활성화 한 이후에는 처리량이 거의 두배 가까이 향상햇으며 레이턴시는 조금 떨어질 수 있어도 대규모 트래픽 환경에서는 더욱 나은 트레이드오프라고 볼 수 있다.