# 이미지 특징점 추출 Feature Extraction

이미지 특징점 추출은 원본 이미지 데이터(2차원 배열 픽셀)에서 기하학적 형태나 픽셀 값의 변화가 두드러지는 유의미한 정보 Feature를 찾아내고, 이를 컴퓨터가 연산하고 비교할 수 있는 정량적인 다차원 벡터 배열 (Descriptor)로 변환하는 과정이다. 이미지 검색, 객체 인식, 파노라마 스타칭등 다양한 컴퓨터 비전 작업의 핵심 전처리 단계다.

- **Feature(특징)**: 이미지 내에서 배경이나 평탄한 영역과 구별되는 고유한 지점을 의미한다. 주로 코너, 엣지, 블롭 등을 지칭한다. (Blob은 밝거나 어두운 영역의 덩어리를 의미함)
- **Extraction (추출)**: 수백만 개의 픽셀 데이터를 모두 사용하는 대신에, 이미지의 본질적인 특성을 담고있는 데이터만을 뽑아낸다.

#### 특징점 추출이 해결하고자 하는 문제

**고차원의 저주와 연산량의 한계**: 1920 x 1080 해상도의 이미지는 약 200만 개의 픽셀을 가진다. 두 이미지를 픽셀 단위로 일일이 비교하는 것은 연산 비용이 극도로 높으며 실시간 처리가 불가능하다.

**픽셀 데이터의 취약성**: 동일한 객체를 찍은 사진이라도 조명, 카메라의 시점, 이미지의 크기 및 회전에 다라 픽셀의 격자 값은 완전히 달라진다. 원본 픽셀 값 자체를 검색에 사용하면 정확도가 현저히 떨어진다.

### 문제 해결 방식

픽셀의 절대적인 색상 값 대신에, 픽셀 주변의 **상대적인 변화율 (Gradient)나 지역적 구조 (Local Structure)**에 집중하여 문제를 해결한다.

평탄한 영역은 버리고 정보량이 많은 지점(ex. 두 개의 엣지가 교차하는 코너)만을 식별한 뒤, 해당 지점 주변 픽셀들의 방향성과 밝기 변화 패턴을 수학적인 히스토그램으로 모델링한다. 이를 통해 이미지가 회전하거나 크기가 변하더라도 일관된 벡터 값을 유지할 수 있는 **불변성 Invariance**을 확보한다.

### 동작 원리

특징점 추출 파이프라인은 크게 두 가지 저수준 low-level 컴포넌트로 나뉜다.

### 특징점 검출기 Feature Detector

이미지 내에서 특징이 될 만한 지점의 x, y 좌표를 찾는다.

작은 window 즉 픽셀블록을 이미지 전체에 슬라이딩 시키며 윈도우를 상화좌우로 이동했을 때 픽셀 값의 변화량이 가장 큰 지점을 찾는다.

픽셀 밝기 함수를 $I(x, y)$라 할때, 윈도우를 $(u, v)$ 만큼 이동 했을때의 픽셀값 차이의 제곱합 $E(u, v)$를 계산한다.

$$E(u, v) = \sum_{x,y} w(x, y) [I(x+u, y+v) - I(x, y)]^2$$

테일러 급수 전개와 고유값 Eigenvalue 분석을 통해 x축과 y축 양방향 모두 1차 미분값이 급격히 변하는 지점을 코너(특징점)로 정의한다.

### 특징 기술자 Feature Descriptor

검출기가 찾은 x,y 좌표를 중심으로 주변 픽셀 영역의 특성을 분석해 다차원 벡터로 인코딩한다.

특징점 주변 영역(Patch)을 분할하고, 각 구역 내 픽셀들의 Gradient(기울기) 크기와 방향을 계산하여 방향 히스토그램을 생성한다.

생성된 히스토그램의 Bin 값들을 일렬로 나열하여 수치 벡터를 생성한다.

두 이미지의 특징점이 동일한 객체에서 추출되었다면, 이 벡터간의 유클리디언 거리나 해밍거리가 매우 짧게 나타난다.

<br>

### Example OpenCV를 활용한 Feature Detection & Description

아래는 이미지의 특징점을 detection 찾고. 이를 벡터로 변환 description 하는과정을 보여준다.

고전적인 Harris Corner 방법론과, SIFT를 보완하여 널리 쓰이는 ORB(Oriented FAST and Rotated BRIEF) 알고리즘의 동작을 주석과 함께 상세히 구조화한다


```py
import cv2
import numpy as np

def extract_features_example(image_path):
    # 1. 이미지 로드 및 전처리
    # 특징점 추출은 주로 픽셀의 밝기 변화율(Gradient)을 기반으로 하므로,
    # 연산량 감소와 노이즈 최소화를 위해 Grayscale로 변환합니다.
    img = cv2.imread(image_path)
    if img is None:
        raise ValueError("이미지를 찾을 수 없습니다.")
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

    # ==========================================
    # [Component 1] 저수준 Corner Detection (Harris Corner)
    # 픽셀 변화율의 고유값을 수식적으로 계산하여 코너를 찾습니다.
    # ==========================================
    # float32 타입 캐스팅: 미분 계산 시 소수점 단위의 정밀도가 필요함
    gray_float = np.float32(gray)
    
    # cv2.cornerHarris 파라미터:
    # 1. gray_float: 입력 이미지
    # 2. blockSize=2: 코너 검출을 위해 고려할 이웃 픽셀 윈도우 크기 (2x2)
    # 3. ksize=3: 소벨(Sobel) 미분 연산자의 커널 크기 (X, Y 방향 Gradient 계산용)
    # 4. k=0.04: Harris Corner 방정식의 경험적 상수 (보통 0.04 ~ 0.06)
    dst = cv2.cornerHarris(gray_float, blockSize=2, ksize=3, k=0.04)
    
    # 노이즈를 제거하고 형태를 뚜렷하게 하기 위한 모폴로지 팽창(Dilate) 연산
    dst = cv2.dilate(dst, None)
    
    # 임계값(Threshold) 처리: 
    # 코너 응답 함수(R)의 결과값이 로컬 최대값의 1% 이상인 지점만 유효한 특징점으로 간주
    harris_corners = np.argwhere(dst > 0.01 * dst.max())
    print(f"Harris 코너 검출 개수: {len(harris_corners)}개")


    # ==========================================
    # [Component 2] Feature Detection + Description (ORB)
    # ORB는 FAST 알고리즘으로 코너(Detector)를 찾고, 
    # BRIEF 알고리즘으로 이진 벡터(Descriptor)를 생성하는 통합 파이프라인입니다.
    # ==========================================
    # ORB 객체 생성 (최대 검출 특징점 수를 500개로 제한)
    orb = cv2.ORB_create(nfeatures=500)
    
    # detectAndCompute 함수는 Detector와 Descriptor를 동시에 실행합니다.
    # keypoints: 특징점의 위치, 크기, 각도 정보를 담은 객체 리스트
    # descriptors: 특징점 주변의 정보를 인코딩한 2D Numpy 배열 (N x 32)
    keypoints, descriptors = orb.detectAndCompute(gray, None)
    
    print(f"ORB 추출된 특징점(Keypoint) 개수: {len(keypoints)}개")
    if descriptors is not None:
        print(f"Descriptor 행렬 형태(Shape): {descriptors.shape}")
        # 예: Shape가 (500, 32)라면 500개의 특징점이 각각 32바이트(256비트)의 해시 벡터로 기술되었음을 의미.
        # 이 벡터들이 LSH 해싱이나 Elasticsearch 벡터 검색의 입력값이 됩니다.
        
        # 첫 번째 특징점의 디스크립터 구조 확인 (Low-level 배열)
        print("첫 번째 특징점의 32바이트 디스크립터 (Binary Descriptor):")
        print(descriptors[0])
        
    return keypoints, descriptors

# 사용 예시 (실제 실행 시 로컬 이미지 경로로 치환 필요)
# kp, desc = extract_features_example("sample_image.jpg")
```