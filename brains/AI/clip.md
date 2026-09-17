# Metric Learning & CLIP

기존의 범용 이미지 분류 모델 (ResNet 등)이 가지는 한계를 극복하고 우리 서비스의 상품들을 미세한 차이까지 정확하게 구분해 내도록 AI 모델의 두뇌를 개조하는 과정을 알아보겠다. 더 나아가 이미지와 텍스트를 동시에 이해하는 검색 엔진의 근간이 된다.

- **Metric Learning (거리 학습)**: 딥러닝 모델이 추출한 고차원 벡터 공간에서, 비슷한 상품의 벡터는 가깝게 (거리 축소), 다른 상품의 벡터는 멀게(거리 확장) 배치하도록 강제로 학습시키는 기법이다.
- **Fine-tuning(미세 조정)**: 구글이나 메타가 대규모 일반 데이터로 미리 학습시켜둔 똑똑한 모델(pre-trained)를 가져와 우리 회사에 특화된 데이터로 추가 학습을 시키는 작업이다.
- **Triplet Loss(트리플렛 손실)**: 거리 학습에서 가장 널리 쓰이는 손실 함수로 기준 Anchor, 정답 Positive, 오답 Negative 세 가지를 한 쌍으로 묶어 모델을 채점한다.
- **CLIP(Contrastive Language-Image Pretraining):** OpenAI에서 발표한 모델로, 텍스트와 이미지를 별개의 시스템이 아니라 하나의 동일한 벡터 공간에 매핑하여 서로 교차 검색이 가능하게 만든 멀티모달 아키텍처다.

### 명칭의 유래

- **Metric Learning**: 수학적으로 공간상의 두 점 사이의 거리를 재는 기준을 metric(계량, 거리 함수)라고 부른다. 이 거리를 최적화하는 학습 learning 이라는 직관적인 의미
- **Triplet Loss**: 쌍둥이 Twin을 넘어선 세 쌍둥이 즉 3개의 데이터 기준, 정답, 오답을 한 묶음으로 사용하여 손실을 계산하기 때문에 붙여진 이름이다.
- **CLIP**: 텍스트와 이미지가 서로 짝이 맞는지 대조(Contrasive) 해가며 사전 학습 (Pre-training)을 수행했다는 의미의 약자다.

### The Problem

첫번째로 **도메인 특화 지식의 부재**가 있다. ImageNet으로 학습된 범용 ResNet 모델은 고양이와 의자는 기가막히게 구분하지만, 홈쇼핑에 등록된 빨간색 브이넥 니트, 빨간색 라운드 가디건을 주면 모델 입장에서는 둘다 그냥 빨간 옷이구나 하고 똑같은 벡터를 뱉는다.

두번째로 **교차 모달리티 검색의 불가능**인데, 사용자가 검색창에 나이키 에어포스 하얀색을 텍스트로 쳤을때 해당 db에 해당 텍스트가 태그되어 있지않으면 상품 이미지를 찾아낼 방법이 없다. 이미지 시스템과 텍스트 시스템이 완전히 단절되어있기 때문이다.

### 문제 해결 방식

**도메인 데이터로 Metric Learning을 수행하여** 홈쇼핑의 같은 상품(다른 각도 촬영)끼리는 당기고, 다른 상품끼리는 밀어내도록 Triplet Loss를 사용해 모델의 가중치를 파인튜닝한다. 이를 통해 모델은 옷의 미세한 패턴이나 재질에 집중하도록 진화한다.

**텍스트-이미지 동시 투영(CLIP)**은 이미지 인코더 ResNET/ViT와 텍스트 인코더 Transformer를 동시에 학습시킨다. 강아지 사진 벡터와 귀여운 강아지라는 텍스트 벡터가 코사인 유사도 1.0(완벽 일치)를 가지도록 강제하며 텍스트로 이미지를 검색하는 마법을 ㄱ현한다.

직관적인 예시로 비유해보겠다.

- **범용 모델(기존 ResNet-50)**외국에서 막 온 똑똑한 신입생 알바생이다 옷과 가구는 구분하지만 루즈핏 셔츠와 오버핏 셔츠의 미세한 차이는 모르고 그냥 큰 셔츠라고 서랍에 쑤셔 넣는다.
- **Metric Learning (도메인 파인튜닝)**: 사장님(엔지니어)이 이 알바생을 앉혀놓고 스파르타식으로 교육(학습)한다. 사진두개 Anchor, Positive를 보여주고 이 사진들이 각도만 다르지 같은 루즈핏 셔츠니까 서랍에 넣어! 라고 알려주고 이 사진 Negative는 비슷해보이지만 오버핏이니까 다른 서랍으로 치우라는 교육을 진행해주고 알바생이 우리 옷가게만의 디테일을 구분하는 전문가가 된다.
- **CLIP(멀티 모달)**: 기존의 알바생은 귀가 안들려서 사진 이미지를 보여줘야만 비슷한 옷을 찾아온다. 사장님이 알바생에게 한국어 텍스트와 옷 이미지를 매칭하는 법을 가르쳤고 이제 빨간색 코트!라고 말만해도 알바생은 머리속에서 완벽하게 상상하여 빨간 코트를 창고에서 바로 꺼내온다.


<br>

## 상세 동작 원리 및 구조화

Metric Learning의 핵심인 Triplet Loss의 수학적 메커니즘은 다음과 같이 구조화 된다.

#### 1. 데이터 샘플링: Batch 안에서 3장의 이미지를 뽑는다

- $A$ (Anchor): 쿼리로 사용할 기준 이미지
- $P$ (Positive): $A$와 같은 클래스(같은 상품)의 이미지
- $N$ (Negative): $A$와 다른 클래스(다른 상품)의 이미지


#### 2. 벡터 추출

3장의 이미지를 각각 CNN 모델에 통과시켜 임베딩 벡터를 얻는다.

#### 3. 거리 계산

기준점($A$)으로부터 정답($P$)까지의 거리 $d(A, P)$와 오답($N$)까지의 거리 $d(A, N)$을 계산

#### 4. 손실 Loss 평가 및 역전파

목표는 $d(A, P)$는 최대한 0에 가깝게 작아지고 $d(A, N)$는 최대한 커지게 만드는 것이다.

- 수식: $Loss = \max(0, d(A, P) - d(A, N) + margin)$
- 여기서 $margin$은 "정답과 오답 사이의 거리가 최소한 이 정도(예: 1.0)는 차이가 나야 해!"라고 강제하는 안전거리입니다.

### Example

홈쇼핑 텍스트 검색과 이미지 검색을 통합하기 위해 Hugging Face의 `transformers` 라이브러리를 활용해 CLIP 모델로 텍스트와 이미지 간의 교차 유사도 corss-modeal simuilarty를 게산하는 코드다.

```py
import torch
from PIL import Image
from transformers import CLIPProcessor, CLIPModel
import torch.nn.functional as F

def clip_multimodal_search(image_paths, search_query):
    """
    CLIP 모델을 사용하여, 주어진 이미지들이 텍스트 쿼리와 
    얼마나 연관성이 있는지(유사도)를 계산합니다.
    """
    
    # 1. 사전 학습된 범용 CLIP 모델 및 프로세서(전처리 장치) 로드
    # "openai/clip-vit-base-patch32"는 가장 대중적인 기본 모델입니다.
    model_id = "openai/clip-vit-base-patch32"
    model = CLIPModel.from_pretrained(model_id)
    processor = CLIPProcessor.from_pretrained(model_id)
    
    # 평가 모드 & 그래디언트 비활성화 (Step 1에서 배운 필수 최적화)
    model.eval()
    
    # 2. 이미지 파일들을 PIL 객체로 로드
    images = [Image.open(path).convert("RGB") for path in image_paths]
    
    # 3. 데이터 전처리 (텍스트 토큰화 및 이미지 리사이징/정규화 동시 수행)
    # return_tensors="pt"는 PyTorch 텐서 형식으로 반환하라는 의미입니다.
    inputs = processor(
        text=[search_query], 
        images=images, 
        return_tensors="pt", 
        padding=True
    )
    
    with torch.no_grad():
        # 4. 모델 추론 (텍스트 임베딩과 이미지 임베딩 추출)
        outputs = model(**inputs)
        
        # 5. 유사도 점수 계산
        # logits_per_image: (이미지 개수 x 텍스트 개수) 형태의 유사도 행렬
        # 코사인 유사도에 기반하여 스케일링된 값을 반환합니다.
        logits_per_image = outputs.logits_per_image 
        
        # 보기 쉽게 Softmax를 취해 확률(0~1)로 변환
        probs = logits_per_image.softmax(dim=0).squeeze()

    # 결과 출력
    print(f"텍스트 쿼리: '{search_query}'")
    for i, path in enumerate(image_paths):
        # 배열 형태에 따라 인덱싱 처리
        prob_score = probs[i].item() if probs.dim() > 0 else probs.item()
        print(f" - [이미지 {i+1}] {path} 일치 확률: {prob_score * 100:.2f}%")
        
    return probs

# ==========================================
# 실행 시뮬레이션
# ==========================================
# 가상의 이미지 3장: 빨간_원피스.jpg, 파란_청바지.jpg, 검은_구두.jpg
# 실제 환경에서는 이 함수를 통해 사용자의 '검색어(Text)'와 DB의 '상품 이미지(Image)'를 직접 매칭합니다.
# 
# query = "화사한 봄에 어울리는 빨간색 원피스"
# results = clip_multimodal_search(
#     ["red_dress.jpg", "blue_jeans.jpg", "black_shoes.jpg"], 
#     query
# )
# 
# 예상 출력:
# 텍스트 쿼리: '화사한 봄에 어울리는 빨간색 원피스'
#  - [이미지 1] red_dress.jpg 일치 확률: 98.50%
#  - [이미지 2] blue_jeans.jpg 일치 확률: 1.20%
#  - [이미지 3] black_shoes.jpg 일치 확률: 0.30%
```

<br>

## CLIP

CLIP에 대해서 더 자세히 알아보겠다. 앞서 말한것처럼 CLIP은 텍스트(언어)와 이미지(시각)를 각각 따로 노는 데이터로 보지않고 **하나의 공유된 다차원 벡터 공간 (Shared Latent Space)에 함께 매핑하는 멀티모달 딥러닝 아키텍처다.**

이를 통해 텍스트로 이미지를 검색하거나 (Text-to-Image Search), 추가 학습 없이도 이미지가 무엇인지 텍스트로 분류(Zero-shot Classification)하는 현대적인 AI 검색 엔진의 근간 역할을 한다.

- **Multi-modal(멀티모달)**: 시각, 청각, 텍스트등 서로 다른 형태의 데이터를 동시에 받아들이고 처리할 수 있는 AI 기술
- **Shared Latent Space(공유 잠재 공간)**: 강아지 사진 이라는 시각적 벡터와 귀여운 강아지라는 텍스트 벡터가 수학적 공간상에서 같은 좌표(위치)에 모이도록 만든 가상의 다차원 공간이다.
- **Zero-shot(영샷)**: 모델을 실전에 투입할 때, 특정 도메인 (홈쇼핑 상품같은)에 대해 단 한 번의 추가 학습(Zero) 없이도 즉시 분류나 검색을 수행할 수 있는 강력한 일반화 능력이다.

#### CLIP 명칭의 유래

Contrastive Language-Image Pretraining으로 

대조적인(Contrastive) 학습할때 정답 쌍 (진짜 매칭되는 사진과 글)은 서로 당겨서 유사도를 높이고 오답 쌍(매칭되지 않은 사진과 글)은 대조시켜서 서로 밀어내는 방식을 사용했다는 뜻이며

Lanaguage-Image 언어-이미지 말 그대로 두 가지 다른 모달리티를 연결했다는 의미이며

Pretraining(사전 학습) 인터넷에 널려있는 4억개의 거대한 (이미지, 텍스트) 쌍을 이용해 미리 엄청난 규모로 학습을 마쳐두었다는 의미이다.

### 해결하고자 한 문제

**지도 학습 한계에 있는데** 기존 ResNet 같은 모델은 사람이 일일이 이건 고양이, 이건 의자 class 1, 2 라고 정답 label을 달아준 제한된 데이터셋(ImageNet등)으로 학습해야만 했다. 만약 스마트폰이라는 새로운 카테고라기 생긴다면 처음부터 다시 학습시켜야 했다.

**메타데이터(태그)의존 검색의 한계**도 존재하는데 텍스트로 이미지를 검색하려면, DB 관리자가 이미지마다 수작업으로 빨간색, 원피스, 여름용 같은 텍스트 태그를 입력해 두어야만 했다. 태그가 누락된 이미지는 영원히 검색되지 않는, 고립된 데이터가 되었다.

### 문제 해결 방식

**인터넷 스크래핑 자연어 학습**: 사람이 라벨을 달아주는 대신에, 인터넷 웹페이지에 있는 `<img>` 태그의 `alt`(대체텍스트) 속성처럼 자연스럽게 묶여있는 이미지, 설명글 쌍 4억개를 긁어모아 그대로 학습시켰다.

**Two-Tower 아키텍처와 유사도 극대화**: 텍스트를 읽는 두뇌 text encoder와 이미지를 보는 두뇌 image encoder를 분리하여 가동한 뒤, 같은 쌍에서 나온 결과물 벡터는 코사인 유사도가 1이 되도록 맞추는 대조 학습을 적용했다.

직관적인 예시로 옷가게 알바생으로 비유해보겠다.

- **과거의 알바생 (DB 태그 검색  & 범용 분류 모델)**: 손님이 시크한 블랙 가죽 자켓 찾아주세요! 라는 요청에 과거의 알바생은 옷 자체를 볼줄 모르고 오직 창고 직원이 옷소매에 붙여둔 바코드 텍스트(메타데이터)만 뒤진다. 만약 직원ㅇ이 실수로 블랙 자켓에 태그를 안달아두면 그 옷이 코앞에 있더라도 절대 못찾는다.
- **CLIP 알바생 (멀티모달 공유 공간 모델)**: 이 알바생은 출근하기 전에 패션 잡지나 인스타그램 블로그등을 4억건이나 읽고와서(Pretraining) 손님이 시크한 블랙 가죽 자켓이라고 말하면 알바생의 머릿속에 텍스트가 의미하는 **특유의 분위기와 질감(벡터 공간의 특정 좌표)**가 떠오른다. 알바생은 바코드 태그를 전혀 보지 않고. 창고에 걸린 수만 벌의 옷(이미지)들을 눈으로 스윽 훑어보며 자신의 머리속에 떠오른 느낌(텍스트 벡터)과 시각적으로 가장 일치하는 느낌(이미지 벡터)를 가진 옷을 즉시 꺼내온다. 태그가 없어도 상관없다. 이것이 CLIP의 교차검색 (Corss-modal Search)의 원리다.

### 상세 동작 원리 및 구조화 

CLIP이 4억개의 데이터를 학습하는 NxN 대조 학습 (Constrastive Learning) 메커니즘은 다음과 같이 병렬 처리로 이루어진다.

1. **배치 구성**: 한 번의 학습 단위(Batch)에 N개의 이미지, 텍스트 쌍을 밀어 넣는다 보통 N = 32,768이라는 거대한 크기를 사용한다.
2. **독립적 임베딩:**
   1. N개의 이미지는 image encoder(ViT or ResNet)을 통과하여 N개의 고차원 벡터($I_1, I_2, ..., I_n$)이 된다.
   2. N개의 텍스트는 Text Encoder(Transformer)를 통과하여 N개의 고차원 벡터 $T_1, T_2, ..., T_n$가 된다.
3. **행렬 곱셈 Matric Multiplication**: 이미지 벡터 행렬과 텍스트 벡터 행렬을 내적 Dot Product하여 N x N 크기의 유사도 행렬을 만든다.
4. **대각선 최적화 (Symmetric Cross-Entropy)** 
   1. 행렬의 대각선 성분 $I_1 \cdot T_1$, $I_2 \cdot T_2$ 등 진짜 짝궁은 내적값이 최대 1가 되도록 학습한다
   2. 행렬의 나머지 모든 오프-대각선 성분(서로 다른 짝궁)은 내적 값이 최소 0또는 음수가 되도록 강제로 밀어낸다.

텍스트 벡터와 이미지 벡터가 **수학적으로 내적 Dot Product되어 어떻게 확률 Softmax가 변하는지 low-level 연산을 직접 보여주는 Zero-shot Classification 코드다**

```py
import torch
from transformers import CLIPProcessor, CLIPModel
from PIL import Image
import torch.nn.functional as F

def clip_zero_shot_classification(image_path, candidate_texts):
    """
    미리 학습된 클래스가 없어도, 우리가 임의로 던져준 후보 텍스트 중
    이미지가 무엇에 가장 가까운지 수학적 내적을 통해 확률을 계산합니다.
    """
    model_id = "openai/clip-vit-base-patch32"
    model = CLIPModel.from_pretrained(model_id)
    processor = CLIPProcessor.from_pretrained(model_id)
    model.eval()

    image = Image.open(image_path).convert("RGB")
    
    # 1. 텐서 변환 및 전처리
    inputs = processor(text=candidate_texts, images=image, return_tensors="pt", padding=True)
    
    with torch.no_grad():
        # 2. 모델 내부의 인코더를 각각 호출하여 '독립적인 벡터(임베딩)'를 추출합니다.
        # Image Feature: [1, 512] 차원 (이미지 1장)
        image_features = model.get_image_features(pixel_values=inputs['pixel_values'])
        # Text Feature: [3, 512] 차원 (텍스트 3개)
        text_features = model.get_text_features(input_ids=inputs['input_ids'], attention_mask=inputs['attention_mask'])
        
        # 3. 벡터 정규화 (L2 Normalization)
        # 길이를 1로 맞추어, 내적(Dot Product)이 곧 코사인 유사도(Cosine Similarity)가 되게 만듭니다.
        image_features = image_features / image_features.norm(dim=-1, keepdim=True)
        text_features = text_features / text_features.norm(dim=-1, keepdim=True)
        
        # 4. 내적 연산 (행렬 곱: Matrix Multiplication)
        # [1, 512] @ [512, 3] = [1, 3] 차원의 유사도 점수(Logits)가 나옵니다.
        # model.logit_scale.exp()는 점수의 크기를 증폭시켜주는 CLIP 고유의 온도(Temperature) 파라미터입니다.
        logit_scale = model.logit_scale.exp()
        logits_per_image = logit_scale * image_features @ text_features.t()
        
        # 5. 확률 변환 (Softmax)
        # 유사도 점수를 합이 1(100%)이 되는 확률로 변환합니다.
        probs = logits_per_image.softmax(dim=1).squeeze()

    # 결과 분석
    print(f"대상 이미지: {image_path}")
    print("-" * 30)
    for i, text in enumerate(candidate_texts):
        print(f"후보 텍스트: '{text}' -> 내적 기반 일치 확률: {probs[i].item() * 100:.2f}%")

# ==========================================
# 실행 시뮬레이션
# ==========================================
# 기존의 ResNet이라면 "이건 홈쇼핑 가전카테고리 3번이야" 같이 정해진 클래스만 내뱉지만,
# CLIP은 우리가 현장에서 지어낸 어떤 텍스트 쿼리와도 실시간으로 유사도를 비교해 냅니다.
# 
# candidates = [
#     "A modern wooden dining table", 
#     "A red leather sofa", 
#     "An abstract oil painting"
# ]
# clip_zero_shot_classification("sample_furniture.jpg", candidates)
```

실제 이커머스 서비스에서 오프라인 이미지 색인 및 실시간 텍스트 인코딩 -> elasticsearch 전달 과정을 수행하는 파이프라인 컴포넌트 클래스를 보자

```py
import torch
from transformers import CLIPProcessor, CLIPTextModelWithProjection
from typing import List

class HomeShoppingSearchEncoder:
    """
    홈쇼핑 검색 백엔드에 탑재되는 실시간 텍스트 임베딩 전용 클래스.
    실시간 검색 시 무거운 이미지 인코더는 메모리에 올리지 않고 텍스트 인코더만 가동하여 응답 속도를 최적화합니다.
    """
    
    def __init__(self, model_name: str = "openai/clip-vit-base-patch32"):
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        
        # 메모리 최적화: Image Encoder를 제외하고 Text Encoder와 Projection Head만 로드합니다.
        self.text_model = CLIPTextModelWithProjection.from_pretrained(model_name).to(self.device)
        self.processor = CLIPProcessor.from_pretrained(model_name)
        
        self.text_model.eval()

    @torch.no_grad()
    def get_search_query_vector(self, search_query: str) -> List[float]:
        """
        고객이 입력한 검색어를 받아 Elasticsearch k-NN 쿼리에 넣을 수 있는
        [512] 차원의 실수 배열(List)로 변환합니다.
        """
        # 1. 고객의 검색어 전처리 (토큰화)
        inputs = self.processor(text=[search_query], return_tensors="pt", padding=True).to(self.device)
        
        # 2. 텍스트 임베딩 벡터 추출
        text_outputs = self.text_model(**inputs)
        text_embeds = text_outputs.text_embeds
        
        # 3. L2 정규화 (Elasticsearch 코사인 유사도 검색을 위해 필수)
        normalized_embeds = text_embeds / text_embeds.norm(dim=-1, keepdim=True)
        
        # 4. JSON 직렬화를 위해 PyTorch Tensor를 순수 Python List로 변환
        return normalized_embeds.squeeze().cpu().tolist()

# ==========================================
# 백엔드 API 컨트롤러 활용 예시
# ==========================================
# encoder = HomeShoppingSearchEncoder()
#
# @app.get("/search")
# def search_products(query: str):
#     # 1. 텍스트 쿼리를 512차원 벡터로 변환 (약 10~20ms 소모)
#     query_vector = encoder.get_search_query_vector(query)
#     
#     # 2. Elasticsearch에 직접 벡터 검색 쿼리(k-NN) 요청
#     es_query = {
#         "knn": {
#             "field": "image_vector", # 오프라인에서 미리 색인해둔 이미지 벡터 필드
#             "query_vector": query_vector,
#             "k": 20,
#             "num_candidates": 200
#         }
#     }
#     # results = es_client.search(index="home_shopping_products", body=es_query)
#     # return parse_results(results)
```