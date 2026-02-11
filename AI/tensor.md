# Tensor

딥러닝이라는 거대한 함수의 가장 작은 단위이자, 모든 데이터의 그릇인 Tensor를 알아보자.

텐서는 단순한 n차원 배열일 뿐이지만, 딥러닝에서ㅓ는 차원 Dimension과 형상 Shape이 곧 로직의 전부라고 볼 수 있다.

### 텐서의 차원과 형상

텐서의 계층 구조를 알아보자. 

데이터가 깊어질수록 이름이 달라지는데, 하지만 결국은 숫자의 묶음일 뿐이다.

- **스칼라**: int, float 변수하나 0차원 변수를 의미한다.
- **벡터**: 1차원 Array, List 등을 의미한다 [1, 2, 3, 4] 
- **행렬**: 2차원 Table, Excel등을 의미한다 [[1, 2], [3, 4]]
- **텐서**: 3차원 이상이고 3차원 이상이 중첩된 배열, 이미지 데이터나 동영상 등이 있다.

### Shape

shape, 형상이 왜 중요할까? 개발할때 typeError가 발생하듯

딥러닝에서는 런타임 에러가 90% shape mismatch에 사용된다.

Shape는 각 차원에 요소가 몇 개 있는지를 나타내는 튜플이다. 예시로 (3, 244, 244) -> 3개의 채널(RGB)가 있고 가로세로가 244픽셀인 이미지. 라는 뜻

딥러닝 모델은 수만개의 숫자를 한꺼번에 계산한다. 이때 입력 데이터의 shape과 모델 가중치 weight의 shape이 톱니바퀴처럼 딱 맞아야 행렬 곱셈이 가능하다.

### 실전 데이터 shape ex

딥러닝 엔진 PyTorch, TensorFlow이 데이터를 먹는 고정된 형식이다

- **이미지 예시: Batch Size, Channels, Height, Width**
  - 64, 3, 28, 28 -> 28x28 사이즈의 컬러 이미지 64장을 묶음으로 처리하겠다는 뜻.
- **텍스트 예시: BatchSize, Sequence Length, Embedding Size**
  - 32, 10, 512: 10개의 단어로 된 문장 32개를 처리하며, 각 단어는 512차원의 숫자로 표현된다는 뜻

```py
import torch

# 1. 텐서 생성 (2x3 행렬)
x = torch.tensor([[1, 2, 3], [4, 5, 6]])
print(f"Shape: {x.shape}") # torch.Size([2, 3])

# 2. View/Reshape (가장 중요: 데이터는 그대로 두고 모양만 변경)
# 2x3을 6x1로 바꿈
y = x.view(6, 1) 
print(f"Reshaped: {y.shape}") # torch.Size([6, 1])

# 3. 차원 늘리기 (Unsqueeze) - 모델 입력을 위해 차원 맞출 때 자주 사용
z = x.unsqueeze(0) 
print(f"Added Dimension: {z.shape}") # torch.Size([1, 2, 3])
```

> 딥러닝 코드를 보다가 이해가 안 가면 무조건 print(data.shape)를 찍으세요. 데이터가 어떻게 변형되는지 추적하는 것이 디버깅의 시작이자 끝입니다.