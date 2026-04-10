# ORB(Oriented FAST and Rotated BRIEF)

ORB는 real-time task에서 feature extraction을 위해서 업계 표준처럼 여겨지는 알고리즘으로

ORB를 알기전 어떤 카테고리에 속하는 알고리즘인지 알아보자.

Harris, FAST, GFTT 등이 속한 feature detection인지 BRIEF, LBP등이 속한 feature descriptor인지.

정답은 ORB는 둘 다 할수 있다다. Feature detection을 통해서 keypoint를 추출하고, descriptor를 계산한다.

ORB는 keypoint, ORB descriptor를 모두 산출해 ORB feature를 최종결과물로 내는 알고리즘으로 Oriented FAST and Rotated BRIEF. detector인 FAST와 descriptor인 BRIEF가 모두 들어있다.

> - **feature detector:** 특징점 검출기로 이미지 내에서 특징이 될만한 지점의 x, y를 찾는것  
> - **feature descriptor:** 검출기가 찾은 x y 좌표를 중심으로 주변 픽셀 영역의 특성을 분석해 다차원 벡터로 인코딩 하는 것

즉 FAST로 keypoint를 뽑고 BRIEF로 descriptor를 계산할거라는 것.

그렇다면 ORB는 어떤 알고리즘이라고 다시 정의할 수 있을까. Feature Detection and Descripton 혹은 Feature Extraction 알고리즘이라고 정의할 수 있겠다.

다만 논문에서는 abstract에서 저자가 binary descriptor라고 정의를 내리고 있다. 실제로는 descriptor를 keypoint를 추출하는 feature detection을 포함하는 개념으로 사용하는 경우가 많아서.

그리고 여기서 중요한 포인트가 있는데 ORB의 장점을 rotation invariant와 resistant to noise로 정의한다.

이는 기존에 FAST, BRIEF에서 볼 수 없던 요소들이고 즉, 단순히 FAST, BRIEF를 결합해 새로운 이름을 붙인것은 아니고 두 알고리즘을 개선해 새로운 알고리즘으로 탄생시킨 것이다.

일단 기존 FAST, BRIEF 부터 살펴보자 1.0 기준으로

### FAST (Features from Accelerated Segment Test)

FAST는 2006년에 machine learning for high-speed corner detection 논문에서 소개되었다.

Accelerated Segement Test 중심 픽셀을 둘러싼 원형 궤적 Segement의 픽셀들을 검사 Test하되 불필요한 연산을 건너뛰어 가속화 accelerated 했다는 의미를 담고있다.

FAST는 robotics를 포함해 활용할 수 잇는 적은 환경에서 real-time task를 수행하고자 제안되었다.

#### 해결하고자 한 문제와 해결 방식

기존의 SFIT, SURF, Harris Corner 알고리즘은 성능은 좋지만 수학적 연산량이 너무 많았다. 이는 모바일 기기나 실시간 비디오 스트리밍 환경에서는 프레임 지연을 유발하는 병목이 되었다.

미분 연산을 완전히 버리는 방식으로 해결했는데, 대신 특정 픽셀 p가 코너가 되려면 **주변을 둘러싼 픽셀들이 p보다 확연히 더 밝거나 확연히 어두운 영역이 연속적으로 존재해야한다.**라는 직관적인 기하학적 규칙을 적용해 속도를 극대화했다.

그렇기에 적은 연산양과 빠른 속도가 알고리즘의 가장 큰 장점이자 contribution으로 볼 수 있다. FAST는 인접 pixel과 차이가 크면 corner/edge 등의 feature point라고 가정해 오로지 밝기 변화에만 집중한다.

1. **16-Pixel Circle (16픽셀 원형 탐색)**: 대상 픽셀 p를 중심으로 반지름이 3인 원을 그리면 정확히 16개의 주변 픽셀이 원주 위에 놓인다. 중심 픽셀의 밝기를 $I_p$, 임계값을 $t$라고 한다.
2. **연속성 검사(Segement Test)**: 16개의 픽셀 중 N개 보통 12갠데 이 이상의 연속된 픽셀이  $I_p + t$ 보다 밝거나, $I_p - t$ 보다 어두우면 코너로 판별한다.
3. **고속 거절(High-speed Test) 로직 (Shrot-circuit Evaluation)**: 16개를 다 검사하는 것은 비효율적이다. 배열의 십자 방향인 1, 9, 5, 13번 픽셀만 먼저 검사하고 코너가 맞다면 이 4개중 최소 3개는 조건을 만족해야한다. 만족하지 않으면 즉시 다음 픽셀로 넘어가며 early return, 이로인해 메모리 접근 횟수가 비약적으로 줄어든다.

### BRIEF (Binary Robust Independent Elementary Features)

BRIEF는 FAST등이 찾아낸 특징점 주변 영역을 분석하여, 컴퓨터가 비교할 수 있는 벡터 descriptor로 변환하는 알고리즘이다.

이 벡터를 실수 float이 아닌 0과 1로 이루어진 이진 문자열 binary string으로 생성하는 것이 핵심이다.

- **Binary(이진)**: 실수 배열이 아닌 비트 배열 생성
- **Independent Elementary Features**: 특징점 주변 픽셀들의 밝기를 Pair 쌍으로 묶어 독립적이교 기초적인 비교 연산만 수행한다는 의미이다.

#### 해결하고자 한 문제와 방식

SIFT 등의 디스크립터는 128차원 부동소수점 float32 벡터를 생성한다.

두 특징점이 같은지 비교하려면 각 차원별로 유클리디안 거리 euclidean distance를 계산해야하는데, 이는 제곱근 연산 ($\sqrt{\sum(x_i - y_i)^2}$)을 동반하여 cpu 사이클을 과도하게 소모하고 메모리 대역폭을 크게 차지했다.

특징점 주변의 특정 픽셀 A, B를 무작위로 선택해 A가 B보다 밝은가? 만 묻는다. 맞으면 1 아니면 0을 기록하고 이 짓을 256번 반복해 256비트 즉 32바이트 길이의 문자열로 만든다. 비교 연산은 하드웨어 단에서 가장 빠른 XOR 기반의 해밍 거리로 대체한다.

#### 상세 컴포넌트 동작 원리

1. **평활화 (Gaussian Smoothing)**: 픽셀 단위 비교는 노이즈에 매우 취약하므로 먼저 특징점 주변 영역 patch에 가우시안 블러를 적용해 노이즈를 제거한다.
2. **랜덤 페어 샘플링 (Random Pair Sampling)**: 특징점을 중심으로 하는 패치 내에서 픽셀 쌍 x y를 정해진 가우시안 분포 확률에 따라 256쌍을 선택한다. 이 선택 패턴은 메모리에 미리 캐싱해둔다.
3. **이진 벡터 생성**: 각 쌍에 대해 밝기 함수 I를 비교한다.

$$f(X, Y) = \begin{cases} 1 & \text{if } I(X) < I(Y) \\ 0 & \text{otherwise} \end{cases}$$

결과적으로 1011010... 형태의 256 비트 배열이 완성되고 두 디스크립터의 유사도는 xor연산이후 1의 개수를 세는 Popcount명령어로 단일 cpu 클럭 사이클 수준에서 처리된다.

<br>

## ORB Oriented FAST and Rotated BRIEF

다시 돌아와서 ORB는 OpenCV 연구진이 발표한 알고리즘으로 FAST Detector와 BRIEF Descriptor로 결합하고 이들이 갖던 치명적인 단점(크기 변화 및 회전에 취약함)을 해결한 실무 최적화 모델이다.

- **Oriented FAST:** 방향성 orientation 정보를 알지 못하는 기존 FAST 알고리즘에 객체의 회전 각도를 측정하는 기능을 추가함
- **Rotated BRIEF:** 이미지가 회전하면 픽셀 쌍의 위치도 틀어지는 BREIF의 단점을 해결하기 위해 검출된 방향에 맞춰 샘플링 패턴을 회전 시켰다.

### 문제 및 해결방법

FAST는 이미지 확대/축소시 코너를 인식하지 못하며, BRIEF는 이미지를 10도만 회전시켜도 이진 벡터가 완전히 달라져 매칭에 실패하는 문제들이 존재했고

특허 회피 즉 성능이 좋은 SIFT, SURF는 상업적 이용시 로열티를 내야하는 특허 알고리즘이였다 현재는 만료되었으나 그땐 큰 문제였고, OpenCV는 무료이면서 빠르고 성능 좋은 대체재가 필요했다.

- **Scale 크기 불변성**: 크기의 불변성으로 원본 이미지의 크기를 여러 단계로 줄인 이미지 파라미드를 만들어서 각 층에서 FAST를 수행한다
- **Rotation 회전 불변성**: 특징점 주변의 밝기 분포를 물리적인 질량을 취급하며 중심점과 질량 중심을 잇는 벡터로 방향성을 계산한다. Intensity Centroid 이후 BRIEF 이진 판별 패턴을 해당 각도만큼 미리 회전시킨 후 추출한다.

### 상세 컴포넌트 동작 원리 

1. **다중 해상도 탐색**: 이미지 피라미드 내에서 각 층에서 FAST로 특징점을 찾고 Harris 코너 응답 함수를 사용해 가장 품질 좋은 N개의 특징점만 남긴다 sorting & filtering
2. **Intensity Centroid(강도 중심점) 계산**: 특징점 주변 영역의 모멘트를 게산한다. 픽셀 밝기 $I(x, y)$를 질량으로 보고 x y축에 대한 중심점 $(C_x, C_y)$를 구한다. 특징점 좌표와 이 강도 중심점을 잇는 벡터의 각도 $\theta$를 아크탄젠트(atan2) 함수로 계산하여 해당 객체가 어느 방향으로 기울어져 있는지 알아낸다
3. **Steered BRIEF(조향된 이진 추출)**: 앞서 구한 각도만큼 회전 행렬 Rotation Matrix를 생성하고 BRIEF의 256개 픽셀 비교쌍 좌표들을 이 행렬에 곱해 똑같이 회전시킨다. 이미지가 거꾸로 뒤집혀잇어도 항상 객체의 정방향을 기준으로 픽셀 밝기를 비교하게 되어 동일한 이진 벡터를 도출해낸다.

### Example

아래 코드는 ORB를 생성하고 두 이미지 (원본과 회전/크기 변환된 이미지) 간의 특징점을 찾아 이진 비트 연산으로 고속 매칭하는 파이프라인이다.

```py
import cv2
import numpy as np

def orb_feature_matching(img1_path, img2_path):
    # 1. 이미지 로드 (Grayscale 연산이 유리함)
    img1 = cv2.imread(img1_path, cv2.IMREAD_GRAYSCALE) # 검색 대상 이미지
    img2 = cv2.imread(img2_path, cv2.IMREAD_GRAYSCALE) # 쿼리 이미지 (사용자가 올린 이미지)

    # 2. ORB 객체 생성
    # nfeatures: 추출할 최대 특징점 개수
    # scaleFactor: 피라미드 비율 (Scale 불변성을 위해 이미지를 1.2배씩 축소하며 탐색)
    # nlevels: 피라미드 층수
    orb = cv2.ORB_create(nfeatures=1000, scaleFactor=1.2, nlevels=8)

    # 3. Keypoints(검출된 x,y 위치 및 회전각도)와 Descriptors(추출된 256bit 이진 배열) 계산
    kp1, des1 = orb.detectAndCompute(img1, None)
    kp2, des2 = orb.detectAndCompute(img2, None)

    # des1, des2의 형태는 (N, 32) Numpy 배열이며 dtype은 uint8 입니다. (8bit * 32 = 256bit)
    
    # 4. 특징점 매칭 (Brute-Force Matcher)
    # ORB는 이진 디스크립터이므로 반드시 NORM_HAMMING (해밍 거리)를 사용해야 합니다.
    # L2(Euclidean)를 사용하면 비트열을 일반 숫자로 취급하여 로직이 완전히 붕괴됩니다.
    # crossCheck=True: 양방향으로 매칭된 결과만 반환하여 오탐(False Positive)을 줄임
    bf = cv2.BFMatcher(cv2.NORM_HAMMING, crossCheck=True)

    # 매칭 수행: des1과 des2 사이의 XOR 비트 연산을 통해 가장 거리가 짧은(유사한) 쌍을 찾음
    matches = bf.match(des1, des2)

    # 5. 매칭 결과를 거리(Distance) 기준으로 오름차순 정렬 (거리가 짧을수록 유사도가 높음)
    matches = sorted(matches, key=lambda x: x.distance)

    print(f"이미지1 특징점: {len(kp1)}개")
    print(f"이미지2 특징점: {len(kp2)}개")
    print(f"성공적으로 매칭된 쌍: {len(matches)}개")

    # 상위 50개의 매칭 결과만 시각화 연결선으로 그림
    result_img = cv2.drawMatches(img1, kp1, img2, kp2, matches[:50], None, 
                                 flags=cv2.DrawMatchesFlags_NOT_DRAW_SINGLE_POINTS)
                                 
    return result_img

# 사용 예시:
# result = orb_feature_matching("original.jpg", "rotated_query.jpg")
# cv2.imshow("ORB Matches", result)
# cv2.waitKey(0)
```

### Hamming Distance

Binary Descriptor는 Hamming Distance를 사용할 수 있는 강점이 있는데 이를 통해 descriptor 비교 연산을 빠르게 수행할 수 있다.

여기서 Hamming Distance란 쉽게 말하면 두 개의 문자열 벡터가 서로 같아지기 위해 얼마나 많은 element들이 바꿔야하는지를 나타내는 것이다.

> In information theory, the Hamming distance between two strings of equal length is the number of positions at which the corresponding symbols are different

길이가 같은 두 데이터 주로 0과 1로 이루어진 이진 문자열을 비교할 때 **같은 위치에 있는 값이 서로 다른 자리의 개수**를 뜻한다.

직관적으로 하나의 문자열을 다른 문자열로 완전히 똑같이 바꾸기위해 몇개의 비트를 뒤집어야해?

#### Example

두 개의 7비트 이진 배열이 있다고 가정해보겠다

- 1 0 1 1 1 0 1
- 1 0 0 1 0 0 1
- - - X - X - -

위 결과를 보녀 3 번째와 5번째 비트가 다르다. 이 경우 값이 다른 자리가 2이므로 두 데이터간의 **해밍 거리**는 2가 된다. 거리가 짧을수록 두 데이터가 매우 유사하다는 것을 의미한다.

컴퓨터 하드웨어는 이 해밍거리를 계산할때 가볍고 빠른 비트연산인 xor를 사용하기에 Popcount 1의 개수 세기 콜 하나로 결과값에서 1이 몇개인지만 세면 해밍 거리가 도출된다.

즉 cpu 연산 싸이클이 대폭 감소가 되엇다는것.