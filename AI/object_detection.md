# 객체 탐지 (Object Detection)

객체 탐지는 이미지 내에 존재하는 특정 사물 (사람, 자동차, 의자) 종류를 식별 classification 함과 동시에, 그 사물이 이미지 내의 어디에 위치하는지 **네모 박스 boudning box**로 좌표를 찾아내는 Localization 기술이다.

### 해결하고자 한 문제

이전 주제에 CNN(ResNet) 등은 이미지 전체를 보고 이 사진은 강아지 사진이다! 같은 분류하는데 뛰어나지만,

강아지가 사진 좌측 하단에 있다. 같은 위치 정보는 소실된다.

예를들어 실제 가구 검색 서비스에서 사용자가 방 전체를 찍어서 올린다. 

AI가 사진 전체를 뭉뚱그려 검색하면 배경의 벽지나 바닥 패턴 때문에 엉뚱한 검색 결과가 나온다.

따라서 사용자가 원하는 가구만 정확히 네모 박스로 찾아내어 잘라내는(crop) 전처리가 필수적이다.

### 문제 해결 방식의 진화 2-Stage vs 1-Stage

초기에는 딥러닝 모델이 너무나도 무거워서 두 단계로 나눠서 문제를 풀었었다.

하지만 실시간 처리에 관한 요구사항이 커지면서 한 번에 푸는 방식으로 진화했다.

**2-Stage 모델 대표로는 Faster R-CNN**이 있는데 

1. 동작은 1단계로 이미지에서 물체가 있을만한 영역 region proposal을 수천개 먼저 찾아내고
2. 2단계로 그 영역들만 잘라내어 CNN에 넣고 무슨 물체인지 맞춘다.

정확도는 매우 높지만, 속도가 느려 실시간 영상 처리에는 부적합하다.

**1-Stage 모델 대표로는 YOLO - You Only Look Once**가 있고

1. 동작은 이미지를 바둑판 모양의 그리드 Grid로 쪼갠다. CNN 모델이 이미지를 단 한번 Only Look Once 스윽 보고, 각 그리드 칸마다 여기에 물체가 있는가? 크기는 얼마인가? 무슨 물체인가?를 동시에 계산해낸다.
2. 특징으로는 정확도는 2-Stage보다 아주 약간 낮지만, 속도가 수십배이상 빨라 자율주행이나 CCTV 모바일 앱 등 표준으로 자리잡았다.

<br>

## YOLO 동작 원리

YOLO 모델이 이미지를 분석하면, 모델은 확신이 없기 때문에 물체 하나당 수십 개의 어지러운 네모박스들을 마구잡이로 뱉어낸다.

이를 깔끔하게 정리하는 두 가지 필수 후처리 Post-processing 알고리즘이 있다.

#### 1. **Confidence Threshold (신뢰도 임계값 자르기)**

각 박스에는 `이 안에 물체가 있을 확률 x 그 물체가 강아지일 확률` 을 곱한 Confidence Score 신뢰도 점수가 0 ~ 1 사이로 매겨진다.

점수가 0.5 즉 50% 이하인 박스는 쳐다보지도 않고 버린다라고 설정해 가짜 박스를 1차로 날려버린다.

#### 2. NMS(Non-Maximum Suppression, 비최대 억제)

위 과정을 거쳐도 하나의 강아지 위에 0.9박스 0.85등 여러개의 박스가 겹쳐서 남는다.

NMS는 겹쳐있는 박스들 중 가장 점수가 높은 maximum 박스 하나만 남기고 나머지는 억제 suppression 하여 지워버리는 알고리즘이다.

이때 두 박스가 얼마나 겹쳤는지를 IoU Intersection over Union, 교집합/합집합의 비율이라는 지표로 측정한다.


### Exmaple

pytorch랑 `torchvision` 라이브러리를 사용해 사전 학습된 Faster R-CNN 모델로 이미지를 추론하고 임계값 threshold으로 결과를 정리하는 코드다

```py
import torch
import torchvision
from torchvision.models.detection import fasterrcnn_resnet50_fpn, FasterRCNN_ResNet50_FPN_Weights
from PIL import Image
import torchvision.transforms.functional as F

def detect_objects(image_path, confidence_threshold=0.8):
    # 1. 사전 학습된 Faster R-CNN 모델 로드 (가장 강력하고 범용적인 모델 중 하나)
    # weights를 지정하여 ImageNet 데이터로 학습된 가중치를 불러옵니다.
    weights = FasterRCNN_ResNet50_FPN_Weights.DEFAULT
    model = fasterrcnn_resnet50_fpn(weights=weights)
    
    # 모델을 평가(추론) 모드로 전환
    model.eval()

    # 2. 이미지 로드 및 전처리 (0~1 사이의 텐서로 변환)
    image = Image.open(image_path).convert("RGB")
    image_tensor = F.to_tensor(image).unsqueeze(0) # 배치 차원(Batch dimension) 추가

    # 3. 모델 추론 진행
    with torch.no_grad():
        # 모델의 출력은 'boxes'(좌표), 'labels'(클래스 번호), 'scores'(신뢰도) 형태의 딕셔너리 리스트입니다.
        predictions = model(image_tensor)

    # 4. 후처리: Confidence Threshold 적용
    pred = predictions[0] # 단일 이미지이므로 첫 번째 배치 결과만 사용
    
    # 신뢰도(score)가 임계값보다 높은 객체들의 인덱스만 추출
    keep_indices = pred['scores'] > confidence_threshold
    
    # 임계값을 통과한 최종 데이터만 필터링
    final_boxes = pred['boxes'][keep_indices]
    final_scores = pred['scores'][keep_indices]
    final_labels = pred['labels'][keep_indices]
    
    # (참고: torchvision의 Faster R-CNN 모델 내부에 NMS 로직이 이미 포함되어 
    # 자동으로 적용되어 나오기 때문에, 사용자가 별도로 NMS 함수를 호출할 필요가 없습니다.)

    print(f"찾아낸 유효한 객체 수: {len(final_boxes)}개")
    
    # 결과 출력
    for i in range(len(final_boxes)):
        # COCO 데이터셋의 클래스 라벨(1=사람, 2=자전거 등)로 맵핑 가능
        label_idx = final_labels[i].item()
        score = final_scores[i].item()
        box = final_boxes[i].tolist() # [x_min, y_min, x_max, y_max] 형태
        
        print(f"[{i+1}] 클래스 ID: {label_idx} | 신뢰도: {score:.2f} | 좌표: {box}")

    return final_boxes, final_labels, final_scores

# 사용 예시:
# boxes, labels, scores = detect_objects("living_room.jpg", confidence_threshold=0.7)
```
