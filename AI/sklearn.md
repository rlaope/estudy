# sklearn

sklearn(scikit-learn)은 머신러닝에서 데이터 전처리, 모델 학습, 평가를 편리하게 할 수 있도록 많은 유용한 함수를 제공해준다.

주요 기능별로 몇가지 함수들을 정리해보겠다.

## 1. 데이터 분할

`train_test_split`

```py
from sklearn.model_selection from train_test_split
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size = 0.2, random_state = 42)
```

test_size 0.2는 훈련 세트와 테스트 세트의 비율을 8:2로 의미

random_state는 랜덤 밸류의 시드를 고정해서 같은 랜덤값으로 테스트를 용이하기 위해 고정해두는건데

42는 진짜 의미없고 걍 공대생들의 밈같은거라고 생각하면 편함. 대강 

https://brunch.co.kr/@smarter/97 **은하수를 여행하는 히치하이커를 위한 안내서** 라는 책에서 찾을 수 있고 슈퍼컴퓨터 DEEP THOUGHT는 삶과 우주, 그리고 모든것에 대한 닶은 42다 라고 하는데, 걍 밈인듯

그쪽 관습이라니 일단 확인하고 42로 써주자 ㅎㅎ..

<br>

## 2. 데이터 전처리

- `StandardScaler`: 평균 0, 분산 1로 표준화
- `MinMaxScaler`: 0~1 사이의 값 변환
- `RobustScaler`: 이상치에 강건한 스케일링
- `Normalizer`: 벡터 길이를 1로 맞춤

```py
from sklearn.preprocessing import StandardScaler
scaler = StandardScaler()
X_scaled = scaler.fit_transform(X)
```

이상치에 강건한 스케일링은 표준편차 기준 스케일링에서 (극단적으로 큰값이나 작은값)이 존재하면 평균과 표준편차가 꽤나 크게 왜곡되니까 RobustScaler는 평균 표준편차 대신 중앙값 median과 사분위 범위 IQR = Q3 - Q1을 활용함. 강건하다고 robust 표현

$$
x' = \frac{x - \text{median}(X)}{\text{IQR}(X)}
$$

$$
\text{IQR}(X) = Q_3 - Q_1
$$


 벡터 길이를 1로 맞추는건 데이터 벡터가 x1, x2, x3 라고 할때 이 벡터의 길이 norm는 유클리드 거리(2-노름)는

$$
\|\mathbf{x}\| = \sqrt{x_1^2 + x_2^2 + \cdots + x_n^2}
$$

$$
\mathbf{x'} = \frac{\mathbf{x}}{\|\mathbf{x}\|}
$$

정규화는 각 원소를 이길이로 나누어 벡터 길이를 1로 맞춤

예시

원래 벡터: $\mathbf{x} = [3, 4]$

벡터 길이: $\|\mathbf{x}\| = \sqrt{3^2 + 4^2} = 5$

정규화 후: $\mathbf{x'} = \left[\frac{3}{5}, \frac{4}{5}\right] = [0.6, 0.8]$

<br>

## 3. 특성 선택 / 차원 축소

- `PCA`: 주성분 분석
- `SelectKBest`: 상위 k개의 중요한 feature 선택
- `VarianceThreshold`: 분산이 낮은 feature 제거

```py
from sklearn.decomposition import PCA
pca = PCA(n_components=2)
X_pca = pca.fit_transform(X)
```

<br>

## 4. 모델 관련

- LogisticRegression, SVC, KNeighborsClassifier, RandomForestClassifier 등 다양한 분류기
- LinearRegression, Ridge, Lasso 등 회귀 모델

```py
from sklearn.linear_model import LogisticRegression
model = LogisticRegression()
model.fit(X_train, y_train)
```

<br>

## 5. 성능 평가
- accuracy_score, precision_score, recall_score, f1_score
- confusion_matrix : 혼동 행렬
- classification_report : 정밀도, 재현율, F1 종합 리포트

```py
from sklearn.metrics import accuracy_score, classification_report
print(accuracy_score(y_test, y_pred))
print(classification_report(y_test, y_pred))
```

<br>

## 6. 교차 검증

- cross_val_score : 교차 검증 점수
- GridSearchCV, RandomizedSearchCV : 하이퍼파라미터 튜닝

```py
from sklearn.model_selection import cross_val_score
scores = cross_val_score(model, X, y, cv=5)
```

<br>

## 7. 데이터셋 제공

- load_iris, load_digits, load_wine 등 내장 데이터셋
- make_classification, make_regression 등 가상 데이터 생성

```py
from sklearn.datasets import load_iris
iris = load_iris()
X, y = iris.data, iris.target
```