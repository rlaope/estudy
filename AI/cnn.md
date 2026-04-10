# CNN(Convolution Neural Network) 기반 특징 추출

SIFT, SURF, ORB등이 수학적 공식을 사람이 직접 깎아서 만든 방식이라면, CNN은 데이터로부터 모델 스스로 최적의 특징 추출 필터를 학습하는 현대 이미지 검색의 핵심 기술이다.

### CNN 기반 특징 추출

CNN 기반 특징 추출은 딥러닝 구조인 합성곱 신경망을 활용하여 원본 이미지에서 저차원의 기하학적 형태(선, 코너) 부터 고차원의 의미론적 정보 객체의 부위 등까지 계층적으로 파악해내고, 이를 수백에서 수천차원의 수치형 벡터 feature vector로 변환하는 기술이다.

#### 명칭의 유래 

- **Convolutional(합성곱)**: 이미지 처리에서 필터(커널)를 픽셀위로 슬라이딩 시키며 각 위치의 픽셀값과 필터의 가중치를 곱하고 더하는 수학적 연산을 주력으로 사용하기 때문에 convolution이라는 단어가들어간다
- **Neural Network(신경망)**: 뇌의 시각 피질이 사물을 인식할 때 단순한 선에서 복잡한 형태로 단계별 인식을 거치는 과정을 모방하여 인공 뉴런들은 깊은 층으로 쌓아올렸다는 의미다.

### 기존 방식의 문제 정의

**의미론적(semantic) 정보의 부재가 있었다.** SIFT나 SURF는 단순히 "밝기가 급변하는 뾰족한 점"을 찾을 뿐, 그것이 강아지의 눈인지 자동차의 바퀴인지 이해하지 못한다. 따라서 비슷한 질감이나 패턴을 가진 전혀 다른 물체를 오인하는 경우가 많다.

**복잡한 환경에서의 한계도 존재했다.** 조명이 극단적으로 변하거나, 객체의 일부가 가려지거나(occlusion) 찌그러지는등 비선형적인 변형이 일어나면 기존의 수작업 알고리즘들은 특징점을 전혀 찾지 못하는 fail 문제가 있었다.

### 문제 해결 방식

**"사람이 공식을 짜지 말고, 수백만장의 이미지를 컴퓨터에게 주어 스스로 공식 즉 필터의 가중치를 학습하게 만들자."**

위로부터 시작되었다. ImageNet과 같은 방대한 데이터셋으로 강아지 고양이 자동차등을 분류하도록 모델(VGG, ResNet 등)을 훈련시킨다.

모델은 정답을 맞히기 위해 오차 역전파 backpropagation 과정을 거치며 스스로 가장 객체를 잘 구분할 수 있는 형태의 필터들을 만들어낸다.

이미지 검색 과제에서 이렇게 훈련이 완료된 모델 pre-trained model에서 분류 classification을 담당하는 맨 마지막층만 잘라내고, 그 직전 층에서 출력되는 다차원 배열 feature map을 해당 이미지의 고유한 특징 벡터 descriptor로 그대로 뽑아다 사용한다.

이를 tranfer learning 관점의 Feature Extraction이라고 한다.

<br>

## 상세 컴포넌트 동작 원리 및 구조화

CNN 아키텍처는 보통 다음과 같은 레이어들이 반복되는 구조를 가진다.

### Convlution Layer 합성곱층

이미지의 공간적 특성을 추출한다.

3 x 3 or 5 x 5 크기의 학습 가능한 필터(커널)가 원본 이미지 또는 이전층의 결과물을 순회하며 합성곱 연산을 수행한다.

계층적 특징으로는 네트워크의 얕은 층(초반부)에서는 직선, 곡선, 색상대비 같은 저수준 low-level 특징을 뽑고, 깊은 층(후반부)로 갈 수록 눈, 코, 타이어 같은 고수준 특징이 합성된다

### Activation Function (활성화 함수 - 주로 ReLU)

합성곱 연산을 통과한 값 중 음수를 모두 0으로 만든다 $f(x) = \max(0, x)$

그 이유는 선형 연산 합성곱만 반복하게 된다면 아무리 층을 깊게 쌓아도 결국 하나의 거대한 선형 연산으로 치환되고, ReLU를 통해 비선형성을 부여해야 모델이 복잡한 패턴을 학습할 수 있다.

### Pooling Layer (풀링층 - 주로 Max Pooling)

특정 영역 2 x 2 에서 가장 큰 값만 남기고 나머지는 버려서 이미지 feature map의 해상도를 절반으로 줄인다.

연산량을 획기적으로 줄여주며, 객체가 이미지 내에서 살짝 이동하더라도 동일한 특징을 유지할 수 있는 불변성을 제공한다.

### Global Avaerage Pooling(GAP) or Fully Connected Layer (FC층)

깊은 층을 통과하며 추출된 3차원 형태의 Feature Map (ex. 7 x 7 x 2048)의 공간 정보를 평균내어(GAP) 1차원 벡터로 압착한다.

결과물은 최종적으로 512, 1024, 2048 차원의 수치형 벡터 float32 배열이 도출된다. 이 벡터가 LSH 알고리즘으로 해싱되어 elasticsearch에 색인되는 최종 결과물이 된다.

## Example

pytorch 기반 ResNet50 특징 벡터 추출

실무에서 이미지 검색기를 만들 때 가장 대중적으로 쓰이는 pytorch 프레임워크와 사전 학습된 ResNet50 모델을 사용하여 원본이미지를 2048차원 벡터로 추출하는 로직이다.

```py
import torch
import torchvision.models as models
import torchvision.transforms as transforms
from PIL import Image

def extract_cnn_features(image_path):
    # 1. Pre-trained 모델 로드 (ImageNet 데이터로 이미 학습된 ResNet50)
    # weights=models.ResNet50_Weights.DEFAULT로 최신 가중치 불러옴
    model = models.resnet50(weights=models.ResNet50_Weights.DEFAULT)
    
    # 2. 이미지 검색을 위한 아키텍처 수정
    # ResNet의 마지막 레이어(fc)는 1000개의 클래스 확률을 뱉는 분류기입니다.
    # 우리는 분류가 아니라 '특징 벡터'가 필요하므로 마지막 fc 레이어를 
    # 아무 작업도 하지 않는 Identity 레이어로 교체(잘라내기)합니다.
    import torch.nn as nn
    model.fc = nn.Identity()
    
    # 모델을 평가(추론) 모드로 전환 (Dropout, BatchNorm 등의 동작을 고정)
    model.eval()

    # 3. 이미지 전처리 파이프라인 (ResNet 학습 시와 동일한 환경을 맞춰주어야 함)
    preprocess = transforms.Compose([
        transforms.Resize(256),             # 짧은 축 기준 256으로 리사이즈
        transforms.CenterCrop(224),         # 중앙 224x224 영역 크롭
        transforms.ToTensor(),              # 픽셀값(0~255)을 텐서(0.0~1.0)로 변환
        transforms.Normalize(               # ImageNet 데이터의 평균과 표준편차로 정규화
            mean=[0.485, 0.456, 0.406], 
            std=[0.229, 0.224, 0.225]
        ),
    ])

    # 4. 이미지 로드 및 전처리 적용
    img = Image.open(image_path).convert('RGB')
    img_tensor = preprocess(img)
    
    # PyTorch 모델은 기본적으로 배치(Batch) 단위 처리를 가정하므로, 
    # 차원을 하나 추가해줍니다. (C, H, W) -> (B, C, H, W) 즉 (1, 3, 224, 224)
    input_batch = img_tensor.unsqueeze(0)

    # 5. 모델 추론 (특징 벡터 추출)
    # torch.no_grad(): 가중치 업데이트(학습)를 하지 않으므로 메모리와 연산 속도 절약
    with torch.no_grad():
        # 출력 결과: (1, 2048) 형태의 텐서
        features = model(input_batch)
    
    # Numpy 배열로 변환 및 1차원 평탄화
    feature_vector = features.numpy().flatten()
    
    print(f"추출된 특징 벡터의 형태 (Shape): {feature_vector.shape}")
    print(f"특징 벡터의 첫 10개 값: {feature_vector[:10]}")
    
    return feature_vector

# 사용 예시:
# vector = extract_cnn_features("query_image.jpg")
# 이 2048차원의 vector 데이터가 다음 주제인 'LSH 해싱 알고리즘'의 입력으로 들어갑니다.
```

SIFT 등은 이미지 한장당 여러개의 특징점 keypoints를 뽑는 반면에 일반적인 CNN 방식은 이미지 전체를 단일한 고차원 벡터로 압축해 낸다는 근본적 차이가 있다.