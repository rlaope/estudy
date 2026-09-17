# 벡터공간과 내적

## 벡터와 벡터공간 Vectors and Vector Spaces

**원하는 모든 색을 만들어내는 3가지 기본 색상 규칙과 규칙적인 도화지**라고 생각하면 된다.

그림을 그릴 대 빨강, 초록, 파랑 3가지 물감만 있으면 이들을 섞어서 세상의 거의 모든 색을 만들어낼 수 있다.

- 빨강 2방울 초록 1방울을 넣는 행동을 숫자 $(2, 1, 0)$이라는 화살표(벡터)로 나타낼 수 있다.
- 물감을 2배로 늘리거나(스칼라배), 두 가지 섞은 물감을 다시 합쳐도(덧셈) 여전히 도화지 위에서 표현 가능한 색상이라는 틀 안에서 머무른다.

이처럼 기본 재료를 **몇 개 더하거나 늘려서 만들 수 있는 모든 결과물들의 모임** 그리고 **아무리 섞고 늘려도 그 영역을 벗어나지 않는 안전한 공간을 수학에서는 벡터공간(Vector Space) 이라고 부른다.**

### 수학적 및 공학적 원리

벡터의 정의는 수학에서 단순히 '화살표'에 국한되는 것이 아니다.

**덧셈과 스칼라배 (실수 곱) 라는 두 가지 연산이 정의되는 모든 대상을 벡터라고 한다.**

n개의 숫자가 순서대로 나열된 n차원 원소의 집합을 보통 다음과같이 표현한다.

$$\mathbf{x} = \begin{bmatrix} x_1 \\ x_2 \\ \vdots \\ x_n \end{bmatrix} \in \mathbb{R}^n$$

### 벡터공간의 조건 (닫혀있음)

집합 $V$가 벡터공간이 되기 위해서는 집합 임의의 벡터  $\mathbf{u}, \mathbf{v}$와 스칼라 $c$에 대해 다음 두 연산 결과가 다시 집합 $V$ 안에 존재(닫혀있음, Closure) 해야하며, 8가지 선형 공리(교현, 결합, 항등원, 역원 존재등)을 만족해야한다. 

1. 덧셈에 대해 닫혀있음: $\mathbf{u} + \mathbf{v} \in V$
2. 스칼라배에 대해 닫혀있음: $c\mathbf{u} \in V$

### 기저 Basis와 차원 Dimension

- **선형결합 (Linear Combination):** 벡터들에 스칼라를 곱해 더한 형태다. ($c_1\mathbf{v}_1 + c_2\mathbf{v}_2 + \dots + c_k\mathbf{v}_k$)
- **선형독립 (Linear Independence):** 모여있는 벡터중 어느 하나도 다른 벡터들의 조합으로 만들어낼 수 없는 상태다. (쓸데없는 중복이 없는 상태)
- **기저 (Basis):** 공간 전체를 만들어낼 수 있으면서(Span) 서로 완벽히 독립인 벡터들의 최소 집합이다.
- **차원 (Dimension):** 해당 공간의 기저에 속한 벡터의 개수다.

$$\text{차원 } \dim(V) = \text{기저 벡터의 개수}$$

```
[2차원 공간 ℝ²의 표준기저]
e₁ = [1, 0]ᵀ (X축 방향 기본 단위)
e₂ = [0, 1]ᵀ (Y축 방향 기본 단위)
-> e₁과 e₂의 선형결합 c₁e₁ + c₂e₂ 로 2차원 평면의 모든 점을 표현 가능.
```

### AI 엔지니어링 맥락

- **임베딩 공간 (Embedding Space):** AI는 단어, 이미지, 음성 등의 데이터를 고차원 벡터공간의 한 점으로 변환한다. 예를들어 LLM(대형 언어 모델)은 하나의 단어 4,096차원 또는 12,288차원의 벡터공간 $\mathbb{R}^{d}$상의 좌표로 표현한다.
- **잠재 공간 (Latent Space):** 딥러닝 인코더가 고차원 데이터 (ex. 1024 x 1024 image)의 본질적인 특징만 추출하여 저차원의 부분공간(Subspace)로 압축해 담아두는 공간이다.
- **차원의 축소:** 데이터가 존재하는 차원이 너무 크면 연산량이 폭발하므로, 공간의 기저를 재구성하여 중요한 차원만 남기는 기법(PCA 등)이 필수적으로 활용된다.

<br>

## 벡터의 내적 Inner Product

**햇빛을 비추었을때 바닥에 생기는 그림자의 길이와 바닥 화살표의 곱**

두 개의 화살표 A, B가 있을때 A 화살표를 바로 위에서 빛을 주면 B화살표위에 A그림자가 생긴다.

- 두 화살표가 같은 방향을 향할수록 그림자가 길어져서 내적값 결과 숫자가 매우 커진다.
- 두 화살표가 수직 90도를 이루면 그림자가 전혀 생기지 않으므로 내적값은 정확히 0이 된다.
- 두 화살표가 반대 방향을 향하면 내적 값은 음수가 된다.

즉 내적은 두 화살표가 **얼마나 같은 방향을 바라보는가 즉 유사한가**를 나타내는 단 하나의 숫자를 계산하는 방법이다.

### 대수적 정의 Algebaric Definition

n차원 공간의 두 벡터 $\mathbf{u} = [u_1, u_2, \dots, u_n]^T$와 $\mathbf{v} = [v_1, v_2, \dots, v_n]^T$의 내적은 각 성분끼리 곱한 뒤 모두 더한 스칼라 값이다. 

$$\mathbf{u} \cdot \mathbf{v} = \mathbf{u}^T \mathbf{v} = \sum_{i=1}^{n} u_i v_i = u_1 v_1 + u_2 v_2 + \dots + u_n v_n$$


### 기하학적 정의 Geometric Definition

두 벡터의 길이 이(L2 Norm, $\Vert{}\mathbf{u}\Vert{}$)와 사이 각도 $\theta$를 이용한 정의다.

$$\mathbf{u} \cdot \mathbf{v} = \Vert{}\mathbf{u}\Vert{} \Vert{}\mathbf{v}\Vert{} \cos\theta$$$$\text{단, } \Vert{}\mathbf{u}\Vert{} = \sqrt{u_1^2 + u_2^2 + \dots + u_n^2} = \sqrt{\mathbf{u} \cdot \mathbf{u}}$$

### 코사인 유사도 

벡터의 길이에 영향을 받지 않고 오직 **방향의 유사성만 측정**하기 위해 내적 값을 두 벡터의 길이 곱으로 나누어 정규화한다. 값의 범위는 -1 ~ 1

$$\text{Cosine Similarity}(\mathbf{u}, \mathbf{v}) = \frac{\mathbf{u} \cdot \mathbf{v}}{\Vert{}\mathbf{u}\Vert{} \Vert{}\mathbf{v}\Vert{}} = \cos\theta$$

### 직교성 Orthogonality

두 벡터의 내적이 0이면 두 벡터는 Orthogonal 한다고 하며, 이는 두 데이터가 서로 완전히 독립적이며 이미 상관관계가 없다는 것을 뜻한다.

$$\mathbf{u} \cdot \mathbf{v} = 0 \iff \mathbf{u} \perp \mathbf{v}$$

### AI 엔지니어링 맥락

- **Self-Attention (Transformer / LLM)**: 트랜스포머 모델에서 단어 간의 연관성을 계산할떄 Query 벡터와 Key 벡터의 내적을 수행한다. 이를 Attention Score $\text{Attention Score} = Q K^T$ 라고 부르며 내적 값이 클수록 모델은 두 단어가 문맥상 강하게 연결되어있다고 판단한다.
- **Vector Search, RAG (검색 증강 생성)**: 사용자의 질문 벡터와 VectorDB에 저장된 문서 벡터들 간의 내적(또는 코사인 유사도)를 빠르게 계산하여 가장 연관성 높은 문서를 찾아낸다.
- **GPU 연산 가속**: 내적 연산은 행렬곱 ($A B^T$)의 기본 단위이므로, TensorCore등 하드웨어를 통해 병렬로 신속하게 처리된다.

<br>

## 벡터의 미분 Vector Calculus & Gradient

경사하강법을 생각해보자 비유하면 산꼭대기에서 가장 가파른 내리막길의 방향과 경사도를 알려주는 건데,

컴퓨터가 지도 위 2차원의 공간의 한 지점에 서있고 그 지점의 높이가 손실 오차 즉 Loss를 의미한다고 해보자

우리의 목표는 오차가 가장 적은 산골짜기 밑바닥으로 내려가는 것이고.

이때 내 발밑의 지형 기울기를 계산하여 어느 방향으로 발을 내딛어야 가장 빠르게 아래로 내려갈 수 있는지를 화살표의 형태로 알려주는 수학적 도구가 바로 백터의 미분 Gradient 이다.

### 스칼라를 벡터로 미분: Gradient


입력이 n차원 벡터 $\mathbf{x} = [x_1, x_2, \dots, x_n]^T$ 이고 출력이 하나의 스칼라 값 값 $f(\mathbf{x})$인 함수가 있을 때, 각 성분에 대한 편미분(Partial Derivative)을 모아 만든 벡터를 **그레디언트**라고 한다. $\nabla f$ (나블라 $f$)로 표기한다.

$$\nabla f(\mathbf{x}) = \frac{\partial f}{\partial \mathbf{x}} = \begin{bmatrix} \frac{\partial f}{\partial x_1} \\ \frac{\partial f}{\partial x_2} \\ \vdots \\ \frac{\partial f}{\partial x_n} \end{bmatrix}$$

편미분은 변수가 2개 이상일때 다변수 함수에서 특정 변수 하나만 변하는 값을 보고 나머지는 상수로 취급해 미분하는 것, 즉 특정 변수의 편쪽만 들어서 미분하는것이다. 변수 개많을때 바뀌는거 하나하나 고려해서 구하면 한 input이 바뀌었을때 구하는게 불가능하니, 경우의수 하나로 확정해두고 보자는 뜻

**성질**로는  $\nabla f(\mathbf{x})$ 벡터는 함수 $f$의 값이 가장 가파르게 증가하는 방향을 가리킨다, 따라서 최소값을 찾기 위해서는 그 반대방향인 $-\nabla f(\mathbf{x})$로 이동해야한다.

### 벡터를 벡터로 미분: 야코비얀 행렬

입력도 n차원 벡터 $\mathbf{x}$이고, 출력도 $m$차원 벡터 $\mathbf{f}(\mathbf{x}) = [f_1(\mathbf{x}), f_2(\mathbf{x}), \dots, f_m(\mathbf{x})]^T$인 경우에는 모든 입력의 편미분을 모은 m x n 행렬을 야코비안이라고 부른다.

$$J = \frac{\partial \mathbf{f}}{\partial \mathbf{x}} = \begin{bmatrix}  \frac{\partial f_1}{\partial x_1} & \frac{\partial f_1}{\partial x_2} & \dots & \frac{\partial f_1}{\partial x_n} \\ \frac{\partial f_2}{\partial x_1} & \frac{\partial f_2}{\partial x_2} & \dots & \frac{\partial f_2}{\partial x_n} \\ \vdots & \vdots & \ddots & \vdots \\ \frac{\partial f_m}{\partial x_1} & \frac{\partial f_m}{\partial x_2} & \dots & \frac{\partial f_m}{\partial x_n} \end{bmatrix}$$

### 주요 벡터 미분 공식

행렬/벡터 연산을 다룰 때 자주 쓰이는 미분 형태다. ($\mathbf{A}$가 대칭행렬일 때)

1. / $\frac{\partial}{\partial \mathbf{x}} (\mathbf{a}^T \mathbf{x}) = \mathbf{a}$
2. $\frac{\partial}{\partial \mathbf{x}} (\mathbf{x}^T \mathbf{A} \mathbf{x}) = 2\math$bf{A}\mathbf{x}$  (이차형식 미분)

### AI 엔지니어링 맥락

- **Gradient Descent**: 경사하강법이다 인공지능 모델의 가중치 벡터 $\mathbf{w}$를 손실함수 $L(\mathbf{w})$가 최소가 되는 방향으로 업데이트하는 핵심 규칙이다. 

$$\mathbf{w}_{t+1} = \mathbf{w}_t - \eta \nabla L(\mathbf{w}_t) \quad (\eta: \text{학습률, Learning Rate})$$

- **Backpropagation**: 딥러닝의 출력층에서 오차 스칼라값을 신경망의 수많은 가중치 벡터들에 대해 연쇄 법칙 chain rule을 적용하여 미분값을 구하는 과정이고 이때 수식적으로 야코비안 행렬과 그레디언트 연쇄곱이 일어난다.
- **Pytorch Autograd Engine**: 프레임워크 내부에는 각 연산 그래프 노드마다 스칼라 손실값에 대한 가중치 벡터의 미분 Vector-Jacobian Product를 자동으로 계산하여 최적화를 수행한다.