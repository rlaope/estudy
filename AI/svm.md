# 서포트 벡터 머신

서포트 벡터 머신은 두 클래스로부터 최대한 멀리 떨어져있는 **결정 경계**를 찾는 분류기다.

특정 조건을 만족하는 동시에 클래스를 분류하는 것을 목표로 한다.

결정 경계를 통해 어느 쪽에 속하는지 판단하는 것으로, 선형이나 비선형 분류, 회귀, 이상치 탐색에서도 사용할 수 있는 강력한 성능을 갖는 지도 학습 모델이다.

특히, 복잡한 분류에 잘 들어맞으며 데이터셋이 작거나 중간 크기에 적합하다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FdAu6hf%2FbtrcA1Z8wIF%2FAAAAAAAAAAAAAAAAAAAAAFT2-x_gdlbA1kNYkGwMd1SzztTstPGDt1iXwxxITqjf%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3DJIQ1LrJsYcgMKyZony1IusWvH2M%253D)

기본적으로 마진이라는 아이디어를 사용하는데, 마진은 두 데이터 클래스를 구분하는 도로의 경계를 뜻한다.

위 사진 가운데 실선은 두 데이터 클래스를 구분 짓는 결정 경계를 기준으로 가장 가까운 데이터와 접하여 점선이 있다.

여기서 결정 경계로부터 점선까지의 거리가 마진이다. 같은 의미로 도로의 폭을 뜻한다.

svm 분류기를 클래스 사이에 가장 폭이 넓은 도로를 찾는 것이 최적의 결정 경계를 찾는것이고, 최적의 결정 경계를 찾으면 도로 폭을 최대화하여 마진을 최대로 가질 수 있기 하며, 이를 마진 분류라고 한다.

<br>

### 서포트 벡터

마진을 통해 추가적으로 알 수 있는 것은 **결정 경계와 가장 가까이 있는 데이터들이 마진**을 결정하게 되는것이다.

이를 **서포트 벡터**라고 한다. 서포트 벡터는 도로 바깥쪽에 훈련 샘플을 더 추가해도 결정 경계에는 전혀 영향을 미치지 않으나, 도로 안쪽에 추가적인 데이터가 들어온다면, 결정 경계에 영향을 미치게 되며 마진 또한 달라질 수 있다.

### 하드 마진 분류, 소프트 마진 분류

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FdUJm9s%2Fbtrctx0u5QX%2FAAAAAAAAAAAAAAAAAAAAAB80mCRE0nDSxaCNZ94aMD_jGNEpHDO6dtcXFXwQj_rt%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3Dtsfvoxeng5xC%252BzAvzmTpDN2vkXg%253D)

마진 분류에는 두 가지 종류가 있는데 바로 하드 마진 분류와 소프트 마진 분류다.

하드 마진 분류는 **모든 샘플이 도로 바깥쪽에 분류**되어 왼쪽 사진과 같이 나타나는 것을 하드 마진 분류라고 한다.

하드 마진 분류는 훈련 세트가 선형적으로 구분되는 경우에만 가능하며, 이상치에 매우 민감한 특성을 가지고 있다.

예를 들어, 파란색 클래스에 빨간색 데이터가 섞여 있어 하드 마진을 찾을 수 없게 되거나, 데이터 이상치로 인한 일반화가 되지 않을 수 있다.

이러한 문제를 피하기 위해 마진을 가능한 넓게 유지하면서 마진 오류를 발생시키지 않는 적절한 균형을 잡아야한다.

이를 소프트 마진 분류라고 하며 오른쪽과 같이 나타낸다.

소프트 마진 분류는 그림처럼 도로폭으 ㄹ가능한한 넓게 유지하면서 이상치가 발생해도 어느정도 허용하는 것이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FblTXtv%2FbtrczdAmifH%2FAAAAAAAAAAAAAAAAAAAAACsGMqXSlePVwPcfoBmDmkckfGyQ_OkiO3rzWPbrHzQt%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3DPLTYTcCtEnj814Mo10G9fQZGCMM%253D)

사이킷 런에서는 이러한 마진 오류를 허용하는 하이퍼파라미터 C를 조정할 수 있으며 작을수록 마진 오류를 허용하며, 크면 클수록 허용하지 않는다.

```py
import numpy as np
from sklearn import datasets
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler
from sklearn.svm import LinearSVC

iris = datasets.load_iris()
X = iris["data"][:, (2, 3)] # 길이, 너비
y = (iris["target"] == 2).astype(np.float64) # 품종

scaler = StandardScaler()
svm_clf1 = LinearSVC(C=1, loss="hinge", random_state=42)
svm_clf2 = LinearSVC(C=2, loss="hinge", random_state=42) 

scaled_svm_clf1 = Pipeline([
        ("scaler", scaler),
        ("linear_svc", svm_clf1),
    ])
scaled_svm_clf2 = Pipeline([
        ("scaler", scaler),
        ("linear_svc", svm_clf2),
    ])

scaled_svm_clf1.fit(X, y)
scaled_svm_clf2.fit(X, y)
```

여기서 iris는 붓꽃 데이터 세트를 로드하는 사이킷런에서 제공하는 기능임

꽃잎의 길이와 너비르르 구분하는 예제이고 전체 입력 데이터중 2, 3 (4개 특성중 2번 꽃잎 길이와 꽃잎 너비)를 선택해서 분류함

```
[[1.4, 0.2],
 [1.4, 0.2],
 [4.7, 1.4],
 [5.1, 2.4],
 ...]
```

그리고 `loss="hinge"` 이 부분은 svm이 학습할때 사용하는 손실함수로 틀리면 벌점 여유있을때 손실 0인 구조다.

수식으로 표현하면 = `L=max(0,1−yi​⋅(w⋅xi​+b))`