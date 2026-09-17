# 연산자 융합, 양자화

### 연산자 융합 Operator Fusion

여러개의 연속된 연산 operator을 하나의 gpu 커널로 합쳐서 실행하는 최적화 기법이다.

하드웨어적 최적화 원리르 보면 gpu 연산에서 많은 병목은 연산 성능FLOPS가 아니라 메모리 대역폭 memory bandwidth에서 발생한다.

- **기존방식**: Conv 실행후 VRAM에 결과를 저장 -> ReLU 실행을 위해 VRAM에 다시 로드 -> 결과 저장, 이 과정에서 불필요한 VRAM IO가 발생한다
- **Fusion 적용**: Conv 연산 직후, 데이터가 아직 gpu 내부의 register, L1 Cache에 머물러 있을때 즉시 ReLU를 실행시킨다.
- VRAM 접근 횟수를 획기적으로 줄여 메모리 대역폭 병목 문제를 해결한다.

### 양자화 Quantization

모델의 가중치와 활성화 함수 결과값, 정밀도를 낮추는 기법이다.

FP32(32-bit Floating Point)를 FP16, BF16 또는 INT8(8-bit integer)로 변환한다.

INT8 양자화는 일반적으로 다음과 같은 선형 변환식을 사용한다.

$$Q = \text{clamp}\left(\text{round}\left(\frac{R}{S} + Z\right), Q_{\min}, Q_{\max}\right)$$

(R: RealValue, S: Scale factor, Z: Zero-point)

**최적화 이점**
1. **메모리 사용량 감소:** FP32 -> INT8 변환 시 모델의 크기가 1/4로 줄어든다.
2. **대역폭 절감:** 동일한 시간에 더 많은 데이터를 메모리에서 연산 유닛으로 전송할 수 있다.
3. **처리량 향상:** 현대 gpu(NVIDIA Tensor Cores, Apple Neural Engine등)는 저정밀도 연산에 특화된 하드웨어 가속기를 가지고 있어, INT8 연산시 FP32보다 몇 배 빠른 연산 속도를 제공한다.

### Kernel Optimization

특정 하드웨어의 아키텍처 특성을 극단적으로 활용하도록 커널 코드를 튜닝하는 단계다.

- **Loop Unrolling**: 반복문 제어 오버헤드를 줄이기 위해 루프를 물리적으로 펼치는 기법이다.
- **Tiling (Blocking)**: 데이터를 작은 타일 단위로 나누어 Shared Memory 영역에 올린뒤 연산하여 L2 Cache Miss를 최소화한다.
- **Vectorization:** 단일 명령어로 여러 데이터를 처리하는 SIMD(Single Instruction Multiple Data) 인스트럭션을 명시적으로 사용해 연산 밀도를 높인다.

### 추론 엔진의 역할 (ONNX Runtime, TensorRT)

ONNX Runtime이나 TensorRT같은 추론엔진은 사용자가 짠 모델 그래프를 분석해 위 기법을 자동으로 적용한다.

1. **Graph Optimization:** 불필요한 노드를 제거하거나 상수로 치환한다.
2. **Layout Transformation:** 하드웨어가 선호하는 데이터 레이아웃 (예: NCHW -> NHWC)으로 자동 변경한다.
3. **Static Memory Planning:** 실행 전 메모리 할당을 미리 계산해 런타임시 malloc/free 오버헤드를 제거한다.