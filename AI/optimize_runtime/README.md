# 대용량 트래픽 대응 AI 서빙 인프라 구현 및 최적화

아래를 학습합니다.

- ONNXRuntime 및 TensorRT API를 활용하여 모델 가중치를 변환하고 추론 지연 시간(Latency)을 최적화하는 파이프라인 구현
- Triton Inference Server의 config.pbtxt를 튜닝하여 Dynamic Batching 기반의 GPU 연산 효율 극대화 환경 설정
- vLLM 엔진의 gpu_memory_utilization 및 max_num_batched_tokens 파라미터를 제어하여 PagedAttention 기반 메모리 최적화 테스트
- Python FastAPI와 gRPC를 결합하여 C++ 기반 코어 추론 엔진과 통신하는 고성능 비동기 API 레이어 구현
- Continuous Batching 알고리즘 특성을 분석하고, 동시성 요청 시 발생하는 GPU 유휴 상태(Idle)를 줄이는 스케줄링 로직 테스트
- Nsight Systems 및 PyTorch Profiler를 활용하여 대규모 언어 모델 서빙 시 발생