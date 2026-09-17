# SIFT, SURF

SIFT(Scale-Invariant Feature Transform)는 이미지의 회전이나 크기 변화, 심지어 약간의 시점 변화나 조명 변화에도 변하지 않는 강력한 특징점을 추출하고 기술하는 컴퓨터 비전의 고전적이고 핵심적인 알고리즘이다.

**Scale-Invariant** 크기의 불변 즉, 객체가 카메라에서 멀어지거나 가까워져 픽셀 상의 크기가 변하더라도 특징점으로 인식할 수 있음을 의미한다.

**Feature Transfrom** (특징 변환)은 이미지의 픽셀 데이터를 크기와 회전에 독립적인 128차원의 특징 벡터로 변환한다는 뜻이다.

### 해결하고자 한 문제와 해결 방식

기존의 코너 검출기 Harris Corner는 이미지를 확대하면 코너가 평탄한 엣지처럼 보이게 되어 더 이상 특징점으로 인식하지 못하는 치명적인 단점이 있었다. 즉, **두이미지의 촬영 거리나 배율이 다르면 같은 객체라도 매칭할 수 없었다.**

그래서 **객체의 크기가 다르다면, 이미지를 여러 크기로 줄여가며 비교하면 된다**는 아이더를 수학적으로 구현했다.

이미지 원본에 점진적으로 blur 처리를 하고 크기를 줄여가며 다수의 해상도 공간 scale space를 만든다.

이 공간들 사이의 차이를 구하여, 어떤 크기에서 보더라도 두드러지기 나타나는 극값 extrema를 찾아내어 크기에 불변하는 특징점을 확보한다.

### 상세 컴포넌트 동작 원리 및 구조화

SIFT 파이프라인은 크게 4가지 단계로 구조화 되어있다.

#### Scale-space Extrema Detection (크기 공간 극값 검출)

1. Scale Space 구축을 먼저한다. 이미지에 가우시안 블러를 점진적으로 강하게 적용하여 한 세트 octave를 만들고, 이미지의 해상도를 가로세로 절반으로 줄여 다음 세트를 만드는 과정을 반복한다. Gaussian Pyramid
2. DoG (Difference of Gaussians)라는 인접한 두 가우시안 블러 이미지의 픽셀 값 차리를 빼서 새로운 이미지들을 생성한다. $$D(x, y, \sigma) = L(x, y, k\sigma) - L(x, y, \sigma)$$
3. 극값 탐색은 DoG 이미지 상의 특정 픽셀이 같은 층의 주변 8개 픽셀, 그리고 위 아래층의 주변 18개 픽셀 총 26개 픽셀보다 모두 크거나 모두 작으면 Local Extrema 특징점 후보로 선정한다.

#### Keypoint Localization (특징점 위치 결정)

1. 찾은 후보중 노이즈에 해당하거나 매칭에 불리한 점들은 필터링한다.
2. 테일러 전개 taylor expansion을 통해 서브 픽셀 단위의 정확한 위치를 계산한다.
3. 평탄한 영역이나 엣지(선) 위에 있어 불안정한 점들은 주곡률 principal curvature 분석을 통해 탈락시킨다.

#### Orientation Assignment (방향 할당: 회전 불변성 확보)

1. 특징점이 위치한 영역 주변 픽셀들의 기울기 gradient 방향 direction과 크기 magnitude를 계산한다
2. 360도를 36개의 bin(구간)으로 나눈 히스토그램을 생성하고, 가장 많은 빈도를 차지하는 방향 (가장 강한 gradient 방향)을 해당 특징점의 기준 방향으로 설정한다

#### keypoint descriptor (특징 기술자 생성)

1. 특징점을 중심으로 16 x 16 픽셀 크기의 주변 영역을 가져온다
2. 이때 앞서 구한 기준 방향 만큼 이 영역을 회전시켜서 이미지가 돌아가 있더라도 항상 동일한 정방향을 바라보게 만든다. (회전 불변성)
3. 16 x 16 영역을 4 x 4 크기의 하위 블록 16개로 쪼갠다
4. 각 블록마다 8방향의 gradient 히스토그램을 계산한다.
5. 결과적으로 16 x 8 (블록 x 방향) = 128 차원의 실수형 벡터가 완성된다.

SIFT는 강력하지만 128 차원의 부동소수점 벡터를 연산해야하므로 ORB 이진 해밍 거리와 달리 매칭시 L2 거리 유클리디안을 사용해야한다.

```py
import cv2
import numpy as np

def extract_sift_features(image_path):
    # 1. 이미지 로드 (연산량 감소를 위해 Grayscale 변환)
    img = cv2.imread(image_path)
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

    # 2. SIFT 객체 생성
    # 파라미터 튜닝이 가능하지만 보통 기본값을 사용합니다.
    # nfeatures: 추출할 최대 특징점 수 (0은 무제한)
    # nOctaveLayers: 각 옥타브(해상도 단계)별 블러 이미지 수
    # contrastThreshold: 이 값이 클수록 대비가 낮은 약한 특징점은 무시됨
    sift = cv2.SIFT_create(nfeatures=500, contrastThreshold=0.04)

    # 3. 특징점 검출(Detection) 및 기술자 추출(Description) 동시에 수행
    keypoints, descriptors = sift.detectAndCompute(gray, None)

    print(f"SIFT 추출된 특징점 개수: {len(keypoints)}개")
    
    if descriptors is not None:
        # Descriptor의 Shape은 (N, 128)이며 데이터 타입은 float32 입니다.
        print(f"Descriptor 행렬 형태: {descriptors.shape}")
        print(f"Descriptor 데이터 타입: {descriptors.dtype}")
        
        # 첫 번째 특징점의 128차원 실수 벡터 확인
        print("\n첫 번째 특징점의 128차원 벡터 중 앞부분 10개 값:")
        print(descriptors[0][:10])

    # 4. 시각화 (특징점의 크기와 방향을 화면에 표시)
    # DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS 플래그를 사용하면
    # 각 특징점의 Scale(원의 크기)과 Orientation(원의 내부 선 방향)이 함께 그려집니다.
    img_with_keypoints = cv2.drawKeypoints(gray, keypoints, img.copy(), 
                                           flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)
    
    return img_with_keypoints, keypoints, descriptors

# 매칭을 할 때 주의점: SIFT는 이진 벡터가 아니므로 cv2.NORM_HAMMING이 아닌 cv2.NORM_L2를 사용합니다.
# bf = cv2.BFMatcher(cv2.NORM_L2, crossCheck=True)
# matches = bf.match(des1, des2)
```

그래서 성능은 매우 훌륭하지만 연산속도가 느리다는 단점이 존재한다. 그래서 이 SIFT 연산 병목을 수학적 트릭 (적분 이미지)로 해결한 SURF 라는 개념이 나오는데, 바로 SURF다

## SURF Speeded Up Robust Features

SURF는 SIFT와 마찬가지로 이미지의 크기 변화와 회전등에 불변하는 특징점을 추출하는 알고리즘이다.

하지만 연산 속도가 너무 느렸던 SIFT의 한계를 극복하기 위해 **적분 이미지 Integral Image**와 **박스 필터 BoxFilter**라는 강력한 수학적 근사화 기법을 도입해 처리속도 SIFT대비 3~7배가량 비약적으로 향상시킨것이 특징이다.

- **Speeded Up (가속화 된)**: 앞서 언급한 수학적 근사화를 통해 SIFT의 연산 병목을 해결하고 속도를 대폭 끌어올렸음을 강조한다.
- **Robust Features(강건한 특징)**: 연산은 단순화 했지만, 여전히 스케일(크기) 변화, 회전, 조명 변화, 약간의 시점 변화등의 방해요소에도 흔들리지 않고 동일한 특징점을 찾아낸다는 의미다.

### 해결하고자 한 문제 및 방식

**가우시안 블러의 엄청난 연산량**: SIFT는 스케일 공간을 만들기 위해 원본 이미지에 반복적으로 가우시안 블러 연산을 수행했어야한다. 픽셀 하나하나에 대해 복잡한 정규분포 가중치를 곱하고 더하는 과정은 cpu를 너무 많이 소모한다.

**이미지 리사이징 비용**: SIFT는 이미지를 여러층 octave로 절반씩 줄여가며 비교했고 이미지를 물리적으로 축소하는 연산 자체가 메모리 복사와 추가 연산을 발생시켰다.

**무거운 128차원 벡터**: SIFT의 128차원 디스크립터는 매칭 단계에서 너무나도 무거웠다.

그래서 **복잡한 곡선 가우시안을 단순한 사각형 박스필터로 근사화하고, 이미지를 줄이는 대신 필터의 크기를 키우자** 라는 방법으로 해결했다.

SURF는 가우시안 함수의 종모양 bell shape 커널을 사각형 형태의 box filter로 극단적 단순화 한것인데 그리고 특정 사각형 영역의 픽셀합을 단 4번의 덧셈 뺄셈 만으로 구하는 적분이미지 자료구조를 통해 필터링 속도를 O(1) 상수로 만들었다. 또한, 이미지를 축소하는 대신 필터의 크기를 9x9, 15x15, 21x21 순으로 점진적으로 키우면서 탐색하여 이미지 리사이징 비용을 완전히 없앴다.

### 동작원리

#### 적분 이미지 integral image 연산

개념은 (x, y) 위치의 적분 이미지 값 $I_{\Sigma}(x, y)$는 원본 이미지의 원점 0, 0부터 x, y 까지의 직사각형 영역 안에있는 모든 픽셀값의 합이다.

한 번 적분 이미지를 메모리에 만들어두면, 아무리 큰 사각형 형태의 필터 (Box Filter) 연산을 하더라도 4개의 꼭짓점 값만을 이용하여 단 3번의 산술 연산으로 영역의 합을 구할 수 있다. 필터 크기에 관계없이 연산 속도가 일정해진다.

#### Fast Hessian(고속 헤시안) 기반 특징점 검출

Hessian Matrix 헤시안 행렬은 SIFT의 DoG 대신 2차 편미분 행렬인 헤시안 행렬의 행렬식 determinant를 사용하여 주변보다픽셀 값 변화가 극심한 극값을 찾는다.

헤사안 헹렬의 가우시안 2차 미분 연산자를 사각형 형태의 Box Filter($D_{xx}, D_{yy}, D_{xy}$)로 대체하여 적분 이미지 위에서 즉각적으로 연산한다.

#### Orientation Assignment (방향 할당)

SIFT가 복잡한 gradient 히스토그램을 구했던 것과 달리, SURF는 특징점 주변 원형 영역에서 Haar Wavelet Response(하르 웨이블릿 응답)를 계산한다.

쉽게 말해 x y축 방향으로 사각형 패턴을 대어보고 밝기 변화량을 수치화한다. 이 역시 적분 이미지 덕분에 순식간에 계산되며, 가장 응답이 강한 부채꼴 방향을 기준으로 기준 방향을 설정한다.

#### 64차원 Descriptor (특징 기술자) 생성

특징점 주변을 4 x 4개의 하위 영역으로 나눈다 총 16개의 구역이고

각 구역마다 다시 수평/수직 Heaar Wavelet 응답을 계산하고, 이들의 방향성분과 합과 절댓값의 합 ($\sum d_x, \sum d_y, \sum |d_x|, \sum |d_y|$)을 추출한다.

16구역 x 4값 = 64 차원의 실수 벡터가 완성되고 디스크립터 크기가 SIFT의 절반 128 -> 64가 되어 매칭 속도가 두 배 이상 빨라졌다.

### Example

SURF 알고리즘은 오랫동안 특허로 보호받았기 때문에

opencv 기본 패키지 `opencv-python`에는 포함되지 않고 확장패키지 `opencv-contrib-python`의 `xfeatures2d` 모듈에 격리되어있다. 최근 특허가 만료되었으나, 버전 환경에 따라 다를수도 사용법이

```py
import cv2
import numpy as np

def extract_surf_features(image_path):
    # 1. 이미지 로드 및 Grayscale 변환
    img = cv2.imread(image_path)
    if img is None:
        raise ValueError("이미지를 찾을 수 없습니다.")
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

    try:
        # 2. SURF 객체 생성
        # hessianThreshold: 헤시안 행렬식의 임계값. 이 값이 클수록 더 강한 특징점(선명한 코너)만 추출됩니다.
        # 기본값은 보통 300 ~ 500 사이로 설정합니다.
        # 주의: OpenCV 버전에 따라 cv2.SURF_create() 또는 cv2.xfeatures2d.SURF_create()를 사용해야 합니다.
        surf = cv2.xfeatures2d.SURF_create(hessianThreshold=400)
        
        # 64차원이 아닌 128차원 벡터(Extended SURF)를 원할 경우 파라미터 변경
        # surf.setExtended(True) 

        # 3. 특징점 검출 및 디스크립터 계산
        keypoints, descriptors = surf.detectAndCompute(gray, None)

        print(f"SURF 추출된 특징점 개수: {len(keypoints)}개")
        
        if descriptors is not None:
            # 기본 SURF의 Descriptor Shape은 (N, 64)이며 데이터 타입은 float32 입니다.
            print(f"Descriptor 행렬 형태: {descriptors.shape}")
            
            # 첫 번째 특징점의 64차원 벡터 중 앞부분 일부 확인
            print("\n첫 번째 특징점의 64차원 벡터 중 앞 10개 값:")
            print(descriptors[0][:10])

        # 4. 시각화 
        # DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS를 통해 특징점의 크기(Scale)와 방향 표시
        img_with_keypoints = cv2.drawKeypoints(gray, keypoints, img.copy(), 
                                               flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)
        
        return img_with_keypoints, keypoints, descriptors

    except cv2.error as e:
        print("\n[오류] 현재 설치된 OpenCV 버전에서 SURF를 지원하지 않습니다.")
        print("SURF는 특허 문제로 opencv-contrib-python 패키지의 특정 버전에서만 사용 가능할 수 있습니다.")
        print(f"상세 에러: {e}")
        return None, None, None

# 매칭 단계에서는 SIFT와 동일하게 유클리디안 거리(NORM_L2)를 사용해야 합니다.
# bf = cv2.BFMatcher(cv2.NORM_L2, crossCheck=True)
# matches = bf.match(des1, des2)
```

<br>

## DoG vs Hessian Matrix

SIFT, SURF가 이미지에서 어떤 지점을 특징점 극값 extrema로 인식할 것인가를 결정하는 핵심 수학적 컴포넌트인데 두 방식 모두 이미지의 밝기가 급격히 변하는 블롭 blob(점 형태의 덩어리) 혹은 코너를 찾지만 접근 방식이 다르다

### DoG Differnece of Gaussian - SIFT

DoG는 1차 미분 Gradient의 변화량을 블러 이미지 간의 뺄셈으로 근사화 하여 구하는 방식이다

개념은 원본 이미지에 약한 가우시안 블러를 먹인뒤에 이미지와 강한 가우시안 블러를 먹인 이미지의 차이를 구한다

1. 블러처리는 이미지의 세밀한 노이즈를 없애고 큰 구조만 남긴다
2. 서로 다른 강도로 블러 처리된 두 이미지를 빼면 평탄한 배경은 값이 0이 되어 사라지고 밝기 변화가 두드러지는 엣지나 코너 부분만 윤곽선처럼 남게된다. 대역통과 필터 효과
3. 이렇게 얻어진 DoG 이미지 공간에서 주변 픽셀보다 값이 유독 크거나 작은 지점 극값을 특징점으로 찾는다

$$D(x, y, \sigma) = L(x, y, k\sigma) - L(x, y, \sigma)$$

여기서 L은 가우시안 블러가 적용된 이미지라고 생각하면 된다.

구현이 직관적이고 정확도가 높지만 블러 이미지를 여러장 만들어야해서 연산 비용이 크다는 특징이 있다.

### Hessian Matrix - SURF

헤시안 행렬은 픽셀 밝기 함수의 2차 편미분 곡률 curvature를 계산하여 특징점을 찾는 방식이다.

이미지 밝기를 3차원 지형의 높낮이로 가정할 때, 해당 지형이 얼마나 볼록하거나 오목한지를 측정한다.

1. 특정 픽셀 위치에서 $x$방향 2차 미분($L_{xx}$), $y$방향 2차 미분($L_{yy}$), 대각방향 미분($L_{xy}$) 값을 구한다.
2. 이 값들로 2x 행렬을 구성한다.

$$H = \begin{bmatrix} L_{xx} & L_{xy} \\ L_{xy} & L_{yy} \end{bmatrix}$$

3. 이 행렬의 행렬식(determinant) $\det(H) = L_{xx}L_{yy} - (L_{xy})^2$를 계산한다 행렬식 값이 양수이면서 크면 그 지점은 사방으로 볼록하거나 오목한 뚜렷한 특징점 blob임을 수학적으로 보장한다.

SURF에서는 이를 고속화목적으로 응용했는데 이 복잡한 2차 미분 가우시안 연산을 흑백의 단순한 **사각형 박스 필터**로 대체하는 것이고 그리고 적분 이미지를 사용해 박스 필터 연산을 o(1)로 처리함으로써 DoG보다 압도적으로 빨느 속도를 달성한다

요약하면 SIFT DoG는 흐린이미지 두장을 빼서 윤곽선을 찾는것이고

SURF Hessian Matrix는 2차 미분을 통해 지형의 곡률을 수학적으로 측정하되 연산은 사각형 박스로 퉁치자라는 수학적 최적화 접근이다.