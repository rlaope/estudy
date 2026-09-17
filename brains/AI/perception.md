# 신경망 Perception

### 가중치

LLM은 수조개의 가중치가 모인 거대한 행렬 연산기라고 한다.

여기서 가중치?가 무엇일까.

**딥러닝 모델은 입력x에 출력 W를 곱하고 또 다른 숫자 b를 더하는 과정의 반복이다.**
- **가중치 W:** 입력 데이터중 어떤 데이터가 중요한지를 결정하는 중요도
- **편향 b:** 결과값이 얼마나 쉽게 활성화 될지 조절하는 민감도다.

$$y = Wx + b$$

### 활성화 함수 ReLU

단순히 Wx + b를 계속하면 아무리 층을 쌓아도 결국 하나의 직선 선형연산일 뿐이다. 복잡한 패턴을 배우려면 비선형성이 필요하다.

여기서 ReLU(Rectified Linear Unit)가 나온다.

$$f(x) = \max(0, x)$$

0보다 작으면 0을 내보내고 off, 0보다 크면 그대로 내보내는 on 아주 단순한 스위치이다.

이게 왜 중요하냐면 계산이 일단 엄청나게 빠르고 딥러닝 고질병인 기울기 소실이라는 문제를 어느정도 막아준다.

그럼 여기서 기울기 소실이 무엇일까?

### 기울기 소실 Gradient Vanishing

정보의 증발인데, 모델을 학습시킬때 결과값의 오차를 뒤로 전달하며 가중치를 수정한다. (역전파)

- **문제:** 층이 너무 깊으면, 뒤로 갈수록 수정해야할 기울기가 0에 가까워져 앞쪽층은 아예 학습이 안되는 현상이 발생한다.

줄서잇는데 뒤사람한테 밀지마세요! 했는데 완전 뒤에있는사람은 안들려서 밀수도 있는거랑 비슷한 개념이라고 보면됨

### Normalization

연산 과정에서 숫자가 너무 커지거나 작아지면 컴퓨터가 계산을 못한다.

Normalization 정규화는 레이어를 통과할때마다 숫자들이 특정 범위 0 ~ 1 사이에 예쁘게 모여있도록 강제로 깎고 다듬는 작업니다.

**RMSNorm**: 최근 Llama 같은 LLM에서 쓰는 방식인데 평균은 계산 안하고 제곱 평균 제곱근 root mean square를 사용해 속도를 높인 효율적인 정규화방식이 있다나 뭐라나


### 코드 예시

```py
import torch

# 1. 입력 데이터 (x)와 실제 정답 (y_target)
x = torch.tensor([2.0, 3.0])
y_target = torch.tensor([10.0]) # 우리가 원하는 목표값

# 2. 가중치(W)와 편향(b) 초기화 (처음엔 랜덤값)
# requires_grad=True는 이 숫자들이 학습을 통해 변할 '설정값'임을 명시
W = torch.randn(1, 2, requires_grad=True) 
b = torch.randn(1, requires_grad=True)

# 3. 모델 연산 (y = Wx + b)
y_pred = torch.matmul(W, x) + b

# 4. 활성화 함수 적용 (ReLU)
y_activated = torch.relu(y_pred)

# 5. 오차 계산 (Loss)
loss = (y_activated - y_target)**2

# 6. 역전파 (Backpropagation) -> 여기서 '기울기'를 계산함
loss.backward()

# 7. 가중치 업데이트 (학습)
with torch.no_grad():
    W -= 0.01 * W.grad # 오차를 줄이는 방향으로 W를 살짝 수정
    b -= 0.01 * b.grad
```
