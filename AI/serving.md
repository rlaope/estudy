# AI 서빙 아키텍처 (Triton Inference Server & Dynamic Batching)

ONNX, TensorRT를 적용해서 모델 자체의 연산 속도를 줄였다고 하자.

그런데, 이벤트 기간에 사용자가 100명이 동시에 이미지 검색을 요청하게 된다면 기존의 웹서버 아키텍처는 한 번에 하나의 요청만 순차적으로 처리하거나 스레드 100개를 만들어 gpu에 동시에 접근하려다 oom으로 서버가 다운 될 것이다.

100개의 요청을 가장 안전하고 빠르게 처리하는 인프라 구조를 설계해본다면 어떻게 할 수 있을까?

이 문제를 해결하기 위해 도입하는 기술이 일반 웹 서버와 AI 추론 서버를 물리적/논리적으로 분리하는 마이크로서비스 구조와 전용 서빙 엔진이다.

- **Triton Inference Server**: NVIDIA에서 개발한 오픈소스 AI 모델 추론 전용 서버. 여러 모델을 동시에 띄우고, 클라이언트의 요청을 효율적으로 스케줄링해 gpu 활용도를 극대화 한다.
- **Dynamic Batching(동적 배치)**: 1초 또는 설정한 찰나의 시간동안 개별적으로 들어오는 여러 사용자의 요청을 모아서 한 번에 거대한 묶음으로 gpu에 던져 병렬 처리하는 최적화 기술
- **gRPC**: 구글에서 개발한 고성능 원격 프로시저 호출 프레임워크로 무거운 이미지나 벡터 데이터를 json 형태가 아닌 바이너리 포맷으로 압축해 http2 기반으로 초고속 전송한다.

<br>

## The Problem

해결하고자 한 문제를 다시 알아보자. 

**웹 서버와 AI 모델 리소스 충돌**: 웹 서버는 가벼온 io 작업과 네트워크 통신에 특화되어 있다. 이 서버 메모리에 수 기가 바이트짜리 딥러닝 모델을 띄워놓고 무거운 행렬 연산을 수행하면, 스레드가 블로킹되어 일반적인 상품 조회 api까지 통째로 응답이 지연되는 병목이 발생한다.

**순차 처리로 인한 gpu 낭비**: gpu는 한 번에 수백개의 데이터를 병렬로 계산할 수 있는 코어를 가지고 있다. 하지만 일반 api 서버 구조처럼 고객 1명의 이미지 1장 gpu에 넣고 계산하는 것을 반복하면 gpu 스펙의 5퍼센트도 쓰지못하고 나머지 코어는 놀게 된다.

**즉, 웹 서버가 무거운 AI 모델을 직접 들고있으면 일반적인 비즈니스 로직 처리까지 전부 지연되는 아키텍처 결함이 존재했고, 개별 사용자의 요청을 건건이 처리하다 보니 gpu가 효율적으로 쓰지 못해 트래픽이 몰리면 서버가 쉽게 뻗어버리는 한계가 있었다.**

예를들면 고객 10명이 동시에 이미지 검색 버튼을 눌렀을 때, 기존 방식은 GPU가 1번 고객의 이미지를 처리하는 10ms 동안 나머지 9명은 대기열에 갇혀 무작정 기다려야했다. 마지막 10번 고객은 최소 100ms를 대기하게 되는것

### Solution

문제 해결 방식을 알아보자

- **추론 전용 서버 분리 (Separation of Concerns)**: 웹 서버에서는 AI 모델을 완전히 삭제한다. 웹 서버는 사용자의 요청을 받으면 그 데이터(이미지)를 Triton 서버에 넘기고 결과만 받아오는 proxy를 수행한다.
- **동적 디스패치 dynamic batching 활용**: Triton 서버는 1번 고객부터 10번까지 요청이 0.0005총내에 연달아 들어오면 이 10장을 하나의 텐서로 묶어버린다(batch size = 10) 그리고 gpu에 한 번만 연산을 시켜 10명분의 결과를 동시에 뽑아 다시 쪼개 웹 서버 스레드에게 돌려준다

직관적인 예시로 예를들어보자 옷가게라고 치면

- **과거의 구조(동적 배치가 없을때)**: 카운터 직원 (웹 서버)이 손님 1명에게 반품할 옷 1벌을 받고 직접 창고까지 걸어가 옷을 걸고 (gpu 연산) 온다. 손님 10명이 연속으로 오면 1명당 직원이 1벌을 들고 창고를 왔다갔다 해야한다. 비효율적임
- **Triton의 동적 배치 구조**: 이제 카운터 직원 (웹서버)과 창고 점단 직원(Triton)을 분리했고 카운터 직원은 옷을 받으면 뒤쪽 바구니에 던지고, 창고 직원은 **바구니에 옷이 10벌 모이거나 옷이 들어온지 3초가 지나면 그 바구니를 통째로 들고 한 번만 창고로 다녀온다.** 한 번의 움직임으로 10명의 업무를 동시에 처리하여 처리량이 10배로 뛴다.

### 동작 원리 

실서비스 마이크로서비스 분리된 AI 파이프라인 데이터 흐름이다.

1. **클라이언트 요청**: 모바일 앱에서 서버로 이미지 검색을 요청한다.
2. **웹 서버 (API Gateway)**: 이미지 바이너리로 읽은 후, gRPC 프로토콜을 사용해 네트워크 너머에 있는 Triton 서버로 인퍼런스 요청을 쏜다.
3. **Triton Queueing & Batching**: Triton 서버는 즉시 연산하지 않고 요청을 내부 큐에 잠시 담고 설정된 지연시간 (예: `max_queue_delay_microseconds: 5000` = 5ms) 동안 기다리며 들어온 요청들을 하나의 다차원 배열로 묶는다.
4. **Backend Execution**: 묶인 배치를 최적화된 엔진 onnx runtime or TensorRT에 통과시킨다.
5. **결과 분배**: 출력된 배치 결과를 다시 개별 요청 단위로 쪼개서 대기하고 있던 웹 서버의 각 스레드에 gRPC로 반환한다.

### Example

Triton 서버를 구동할때 모델과 함께 폴더에 넣어주는 `config.pbtxt` 설정 파일을 보자 이 몇줄의 설정이 전체 시스템의 처리량을 좌지우지한다.

```pbtxt
# clip_image_encoder 모델의 설정 파일 (config.pbtxt)

name: "clip_image_encoder"
platform: "onnxruntime_onnx"  # ONNX 모델임을 선언
max_batch_size: 32            # 한 번에 묶을 수 있는 최대 요청(이미지) 수

# 입력 텐서 규격 정의 (Batch 차원은 Triton이 관리하므로 생략)
input [
  {
    name: "pixel_values"
    data_type: TYPE_FP32
    dims: [ 3, 224, 224 ]
  }
]

# 출력 텐서 규격 정의
output [
  {
    name: "image_embeds"
    data_type: TYPE_FP32
    dims: [ 512 ]
  }
]

# 핵심: 동적 배치(Dynamic Batching) 활성화
dynamic_batching {
  # 최대 5ms(5000 마이크로초)까지만 다른 요청이 오길 기다렸다가 배치를 묶어 출발함
  max_queue_delay_microseconds: 5000
}
```

FastAPI 측에서 Triton 서버로 이미지 추론을 요청하는 통신부는 다음과 같이 구현되어있다.

핵심은 무거운 연산이 완전히 외부로 위임된다는 것에 있다.

```py
import numpy as np
import tritonclient.grpc as grpcclient
from PIL import Image
from torchvision import transforms

class TritonInferenceClient:
    """
    웹 서버(API 서버)에서 동작하며, Triton 서버와 gRPC로 통신하는 클라이언트입니다.
    """
    def __init__(self, triton_url="triton-server:8001"):
        # Triton 서버와의 고속 gRPC 연결 생성
        self.client = grpcclient.InferenceServerClient(url=triton_url)
        self.model_name = "clip_image_encoder"
        
        # 웹 서버는 가벼운 리사이즈 및 정규화(CPU 작업)만 수행합니다.
        self.preprocess = transforms.Compose([
            transforms.Resize((224, 224)),
            transforms.ToTensor(),
            transforms.Normalize(mean=[0.481, 0.457, 0.408], std=[0.268, 0.261, 0.275])
        ])

    def request_embedding(self, pil_image: Image.Image) -> list:
        """단일 이미지를 Triton에 보내 512차원 벡터를 받아옵니다."""
        
        # 1. 입력 데이터 준비 (Shape: [1, 3, 224, 224], Type: FP32)
        input_data = self.preprocess(pil_image).unsqueeze(0).numpy().astype(np.float32)
        
        # 2. Triton gRPC 입력/출력 객체 생성
        # config.pbtxt 에 정의된 name과 동일하게 맞추어야 합니다.
        triton_input = grpcclient.InferInput("pixel_values", input_data.shape, "FP32")
        triton_input.set_data_from_numpy(input_data)
        
        triton_output = grpcclient.InferRequestedOutput("image_embeds")
        
        # 3. 네트워크 너머의 Triton 서버로 추론 요청 (동적 배치 큐에 합류됨)
        response = self.client.infer(
            model_name=self.model_name,
            inputs=[triton_input],
            outputs=[triton_output]
        )
        
        # 4. 서버로부터 넘어온 결과물(Numpy 배열) 파싱 및 반환
        result_vector = response.as_numpy("image_embeds")
        
        return result_vector.tolist()[0]

# --- 웹 API 라우터 실행 환경 ---
# triton_engine = TritonInferenceClient("192.168.1.100:8001")
#
# @app.post("/api/v1/search/image")
# def search_by_image(file: UploadFile):
#     img = Image.open(file.file).convert("RGB")
#     
#     # 무거운 연산은 Triton 서버가 처리하고 벡터값만 반환받음
#     vector = triton_engine.request_embedding(img)
#     
#     # 이후 Elasticsearch 8.x 로 벡터 넘겨서 검색 (Step 4)
#     results = es_hybrid_search(vector)
#     return {"results": results}
```