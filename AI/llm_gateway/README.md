# LLM Gateway 및 멀티 테넌트 환경 아키텍처 구현

다음을 학습하는 공간.

- LiteLLM 프록시 서버를 구성하여 OpenAI API 및 vLLM 간의 단일 통합 인터페이스 제공 및 Fallback 라우팅 룰 구현
- FastAPI 미들웨어와 API Key Header 검증을 통해 테넌트(부서별) 요청을 논리적으로 분리하는 권한 인가 로직 개발
- Redis의 INCRBY 및 EXPIRE 명령어를 원자적(Atomic)으로 실행하는 Lua 스크립트를 작성하여 Token Bucket 기반 Rate Limiting 구현
- HTTP 429 상태 코드와 Retry-After 헤더를 활용하여 특정 부서의 트래픽 폭주(Noisy Neighbor)를 API 앞단에서 차단하는 방어 로직 구현
- FastAPI의 BackgroundTasks와 Langfuse SDK를 연동하여 사용자 응답 지연 없이 프롬프트 토큰 사용량을 로깅하는 비동기 관측성 파이프라인 구현
- LLM API 응답 메타데이터에서 입출력 토큰량을 추출하고 테넌트 ID와 맵핑하여 과금용(Chargeback) JSON 스키마로 가공하는 로직 작성