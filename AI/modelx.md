# AI 모델 경량화 및 추론 가속 ONNX & TensorRT

Elastichserach와 HNSW를 통해 저번글에서 알아봣듯 벡터 간의 비교 검색 속도를 단축했다.

그런데, 사용자가 텍스트로 검색이 아닌 이미지로 검색을 요청하게 된다면?

이 이미지의 상품을 찾아줘라는 요청에 대해서 대응해야하는데.

고객에 업로드한 이미지를 pytorch 기반 RestNet-50이나 CLIP 인코더에 통과시켜 512차원의 벡터로 변환 inference 하는대만 cpu 기준 100~200ms가 소모된다.

db 검색이 아무리 빠르더라도 딥러닝 모델 연산 자체가 병목을 일으킨다면, 이 무거운 모델을 실서비스 환경에 맞게 어떻게 최적화해야할까

위 기반의 병목 문제를 해결하기 위해 도입하는 기술이 **모델 경량화 및 추론 가속**이다.

연구 목적으로 작성된 py 프레임워크를 벗어나 모델을 시스템 하드웨어 cpu/gpu 아키텍처에 맞게 변환하고 압축해 연산 속도를 극대화하는 엔지니어링 과정이다.

- **ONNX(Open Neural Network Exchange)**: pytorch, tensorflow등, 다양한 프레임워크를 만들어진 모델을 하나의 공통 포맷으로 변환해주는 개방형 표준 규격
- **ONNX Runtime(ORT)**: ONNX 포맷으로 변환된 모델을 cpp 기반의 환경에서 초고속으로 실행(추론) 해주는 추론 전용 엔진
- **TensorRT**: nvidia 에서 개발한 딥러닝 추론 최적화 엔진으로 gpu 하드웨어 tensor core의 잠재력을 100퍼센트 끌어내기 위한 연산 그래프를 병합하고 최적화했다.
- **Quantization(양자화)**: 모델의 가중치 weight 파라미터를 32비트 실수 FP32에서 16비트 FP16나 8비트 정수 INT8로 압축하여 정확도 손실을 최소화 하면서도 연산 속도와 메모리 효율을 높이는 기법이다.
- **Kernel Fusion(커널 병합)**: 여러개의 독립적인 연산 레이어(ex. Convolution + BatchNorm + ReLU)를 하나의 연산 블록으로 합쳐 메모리 읽기 쓰기 오버헤드를 제거하는 기술이다.

<br>

## 해결하고자 한 문제

**python과 프레임워크의 오버헤드**: pytorch는 연구와 학습 training에 최적화된 프레임워크다 모델을 추론할때 python 인터프리터의 GIL(Global Interpreter Lock) 제약과 동적 연산 그래프 할당 구조로 인해 불필요한 메모리 할당 스케줄링 오버헤드가 발생한다.

**하드웨어 최적화 부재**: pytorch에서 정의된 레이어들은 각각의 별도 gpu 메모리 접근을 요구하고 예를들어 3개의 연속된 레이어 conv -> BN -> ReLU를 통과하려면 데이터를 VRAM에서읽고 쓰는 행위를 3번 반복해야하며 이는 심각한 레이턴시를 유발한다.

### Solution

**cpp로 런타임 이관 onnx**: 모델의 가중치와 연산 구조 그래프만을 순수하게 추출하여 `.onnx` 파일로 저장하고, 추론 서버에서는 무거운 pytorch 의존성 없이 cpp로 작성된 가벼운 onnx runtime을 통해 추론을 실행해 파이썬 오버헤드를 제거한다.

**그래프 최적화 및 양자화 (tensorRT 연동)**: 추론 엔진은 모델이 배포될 실제 하드웨어(nvidia gpu)의 아키텍처를 분석하여, 개별 연산들을 하나로 병합한다. 또한 연산에 필요한 데이텉 타입 크기를 절반 fp16으로 줄여 한 번의 클럭 사이클에 두 배의 데이터를 처리할 수 있도록 만든다.

### 직관적인 예시로 이해하기 (옷가게 물류창고 비유)

- **PyTorch 모델 (연구용 알바생)**: 이 알바생은 본사 메뉴얼 python을 한 줄씩 읽어가며 옷을 검수한다 "1번 상자에서 옷을 꺼내고 (메모리 읽기), 2번으로 상자를 닫는다.. (메모리 쓰기), 그 이후 3번, 다시 옷을 펴서 태그 확인 (메모리 읽기) ..." 동작이 유연하긴한데 너무 느리다.
- **ONNX 변환 (표준화 작업)**: 본사 메뉴얼 의존을 없애고 알바생 머릿속에 최적화된 1장의 행동 지침서 (c++ runtime)를 직접 입력한다. 이제 본사에 물어볼 필요없이 기계적으로 빠르게 움직인다.
- **TensorRT 커널 병합 및 양자화**: 커널 병합: "옷꺼내기, 태그 확인, 포장하기" 라는 3번의 분절된 동작을 "상자에서 옷을꺼냄과 동시에 태그를 확인하고 바로 상자에넣기"라는 하나의 연속 동작 (단일 커널 연산)으로 합쳐버린다.
  - 양자화: 치수를 잴 때 소수점 6자리(FP32)까지 엄격하게 재던것을, 소수점 2자리(FP16)까지만 재도록 기준을 낮추고 정확도는 거의 떨어지지 않으면서 검수 속도느 2배로 빨라진다.

<br>

### 상세 동작 원리 및 구조화

딥러닝 모델이 ONNX 포멧으로 변환되고 런타임에서 최적화되는 low-level 구조다.

1. **Tracing(연산 그래프 추적)**: 파이토치의 `torch.onnx.export()`가 호출되면 더미 텐서를 모델에 통과시킨다. 데이터가 흐르는 경로(연산 순서, 노드, 간선)를 추적하여 정적인 연산그래프 computational graph를 그린다.
2. **Graph Optimization (런타임 그래프 최적화)**: onnx runtime 인스턴스가 실행될 때, 추적된 그래프를 분석하여 중복되거나 불필요한 노드를 제거한다 constant folding
3. **Execution Provider (백엔드 할당)**: ONNX Runtime은 시스템 환경을 감지해 가장 빠른 하드웨어 가속기 execution provider를 선택한다 gpu 환경이면 `TensorrtExecutionProvider`나 `CUDAExectuionProvider`를, cpu 환경이면 `CPUExecutionProvider`를 사용해 연산을 위임한다.
4. **Kerner Fusion(커널 병합 - TensorRT 환경 시)**: 여러 레이어를 묶어 단일 CUDA 커널로 컴파일한다. 이 과정을 통해 gpu의 글로벌 메모리(global memory) 접근 횟수가 획기적으로 감소하고 레지스터 내에서 연산이 끝나게된다.

> 여기서 여러 레이어를 묶는다고 했는데, 여기서 말하는 레이어는 Convolution(합성곱 연산), BatchNorm(정규화연산), ReLU(활성화 함수)같은 딥러닝 모델의 개별 수학 연산 블록을 뜻한다.
>
> 원래 파이토치같은 환경에서 gpu가 레이어 하나를 통과할 대마다 메모리에서 데이터를 읽기 계산 쓰기 반복작업을 진행하는데 예를들어 모델이 Conv $\rightarrow$ BatchNorm $\rightarrow$ ReLU 구조라면 gpu는 메모리 왕복을 3번하는거고 시간이 걸린다
>
> 이 여러 레이어를 묶어서 하나의 CUDA 커널로 컴파일(kernel fusion) 한다는 것은 데이터를 메모리에서 딱 1번만 읽어온뒤 gpu 내부에서 저 3가지 계산을 순식간에 다 끝내고 딱 1번만 저장하도록 동작 방식을 합쳐버린다는 뜻이다.
>
> 불필요한 메모리 왕복 시간이 완전히 사라지기 때문에 추론 속도가 극적으로 빨라진다.

<br>

### Example pytorch 모델을 onnx로 변환

가장 무거운 작업인 모델 변환 export를 오프라인 환경 개발 pc에서 미리 수행해 .onnx라는 정적 파일을 생성하는 로직이다.

```py
import torch
from transformers import CLIPModel

def export_clip_to_onnx(model_id="openai/clip-vit-base-patch32", output_path="clip_image_encoder.onnx"):
    """
    PyTorch로 로드된 CLIP 모델의 Image Encoder 부분만 
    독립적인 ONNX 파일로 추출하여 저장합니다.
    """
    print("1. PyTorch 모델 로드 중...")
    model = CLIPModel.from_pretrained(model_id)
    model.eval() # 반드시 평가 모드로 설정
    
    # 모델에 통과시킬 가짜 입력 데이터 (Dummy Input)
    # Batch Size=1, Color=3(RGB), Width=224, Height=224
    dummy_pixel_values = torch.randn(1, 3, 224, 224)
    
    print("2. ONNX Graph Tracing 시작...")
    # 이미지 인코더만 분리하여 ONNX로 내보내기
    torch.onnx.export(
        model.vision_model,                # 변환할 PyTorch 모델 (vision_model만 추출)
        dummy_pixel_values,                # 모델 구조를 추적하기 위한 더미 입력
        output_path,                       # 저장할 파일명
        export_params=True,                # 학습된 가중치(Weights) 포함 여부
        opset_version=14,                  # 호환성을 위한 ONNX 연산자 버전
        do_constant_folding=True,          # 상수 폴딩 최적화 적용
        input_names=['pixel_values'],      # 입력 노드 이름 정의
        output_names=['image_embeds'],     # 출력 노드 이름 정의
        dynamic_axes={                     # 배치 크기를 고정하지 않고 동적으로 받기 위한 설정
            'pixel_values': {0: 'batch_size'},
            'image_embeds': {0: 'batch_size'}
        }
    )
    print(f"✅ ONNX 변환 완료: {output_path}")

# export_clip_to_onnx()
```

실제 프로덕션 서버에서 pytorch 라이브러리를 아예 제거하고 오직 onnxruntime 라이브러리만 사용해서

초고속 추론을 수행해본다면 다음과 같이 서버를 구성해볼 수 있다.

```py
import onnxruntime as ort
import numpy as np
from PIL import Image
from torchvision import transforms

class ONNXImageEncoder:
    """
    PyTorch 없이 순수 C++ 기반 ONNX Runtime으로 
    이미지 벡터 임베딩을 추출하는 프로덕션 레벨 추론 엔진입니다.
    """
    def __init__(self, onnx_model_path="clip_image_encoder.onnx"):
        # 1. Execution Provider 설정 (하드웨어 가속기 선택)
        # GPU(TensorRT/CUDA)가 있으면 우선 사용, 없으면 CPU 사용
        providers = [
            'TensorrtExecutionProvider', 
            'CUDAExecutionProvider', 
            'CPUExecutionProvider'
        ]
        
        # 2. 인퍼런스 세션 생성 (모델 로드 및 그래프 최적화 수행)
        sess_options = ort.SessionOptions()
        sess_options.graph_optimization_level = ort.GraphOptimizationLevel.ORT_ENABLE_ALL
        
        self.session = ort.InferenceSession(
            onnx_model_path, 
            sess_options=sess_options, 
            providers=providers
        )
        
        # 입력 노드와 출력 노드 이름 캐싱
        self.input_name = self.session.get_inputs()[0].name
        
        # numpy 전용 가벼운 전처리 파이프라인 (PIL Image -> Numpy Array)
        self.preprocess = transforms.Compose([
            transforms.Resize((224, 224)),
            transforms.ToTensor(),
            transforms.Normalize(mean=[0.48145466, 0.4578275, 0.40821073], 
                                 std=[0.26862954, 0.26130258, 0.27577711])
        ])

    def encode_image(self, pil_image: Image.Image) -> np.ndarray:
        """단일 이미지를 받아 512차원 벡터를 반환합니다."""
        # 1. 이미지 전처리 및 차원 추가 (Batch size=1)
        # PyTorch 텐서 대신 순수 numpy 배열(float32)을 사용합니다.
        input_tensor = self.preprocess(pil_image).unsqueeze(0).numpy().astype(np.float32)
        
        # 2. ONNX Runtime 추론 실행
        # 입력 데이터를 딕셔너리 형태로 전달
        inputs = {self.input_name: input_tensor}
        outputs = self.session.run(None, inputs)
        
        # 3. 임베딩 결과 추출 (L2 정규화 적용)
        embeds = outputs[0]
        embeds = embeds / np.linalg.norm(embeds, axis=1, keepdims=True)
        return embeds

# --- 실무 API 라우터 사용 예시 ---
# onnx_encoder = ONNXImageEncoder("clip_image_encoder.onnx")
# 
# @app.post("/vectorize/image")
# def get_image_vector(file: UploadFile):
#     # 이미지 파일 로드
#     img = Image.open(file.file).convert("RGB")
#     
#     # PyTorch 통과 시 100ms 걸리던 추론이 
#     # ONNX+TensorRT 통과 시 약 10~20ms 수준으로 단축됨
#     vector = onnx_encoder.encode_image(img)
#     
#     return {"vector": vector.tolist()[0]}
```