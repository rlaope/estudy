# Envoy Proxy Internal Architecture

Istio는 정책을 내리는 두뇌역할일뿐이다, 실제 데이터 흐름을 제어하고 패킷을 쪼개서 전달하는 역할은 Envoy Proxy이다. 

서비스 메시 최적화 그리고 트러블 슈팅 90%는 envoy 내부를 이해하는것에 따라 달라진다.

### Threading Model

Envoy는 왜 압도적으로 빠를까? envoy는 single process, multi threaded 모델ㅇ르 사용한다.

각 스레드는 독립적인 event loop를 가지게 된다.

- Main Thread: 서버의 시작과 종료, xDS API 업데이트, 시그널 처리 등 관리 작업을 담당한다.
- Worker Threads: 실제 요청을 처리한다, 각 스레드는 non blocking 방식으로 작동하며, 하나의 커넥션은 생명주기 동안 특정 워커 스레드에 pin되어 컨텍스트 스위칭 비용을 최소화한다.

nginx는 멀티 프로세스 구조로 메모리 관리가 복잡하다. nodejs는 싱글 이벤트 루프라 멀티 코어 활용을 위한 클러스터링이 필요하다.

그러나 enovy는 단일 프로세스내에 코어수만큼 워커를 띄워 메모리 효율 및 성능을 동시에 잡는다.

### L3/L7/L7 Fitler Chain

Envoy로 들어온 모든 데이터는, fitler chain이라는 파이프랑인을 통과한다.

이 구조덕에 기능을 모듈식으로 확장이 가능한데 알아보자

1. Network Filter(L3/L4): ip, tcp 레벨의 데이터를 다룬다. (예, tls inspector, rbac 필터)
2. http connection manager(HCM): L4 데이터를 L7으로 해석하는 핵심 필터
3. http filter(L7): HTTP 헤더 수정, 라이팅, gRPC 변환, 압축 등을 수행한다. (ex. 
4. router filter)

필터 체인은 **순서**가 중요하다 인증 필터가 라우팅 필터보다 먼저 실행되어야만 무단 접근을 차단할 수 있다는 것을 생각해보면 된다.

### 4대 구성요소 LRCE

Enovy 설정을 분석할 때 가장 먼저 찾아야하는 4가지 핵심 객체다.

- **Listener**: 특정 ip port에서 요청을 대기하는 관문 (전화번호 수신 번호같은 느낌)
- **Route**: 요청의 host,path를 보고 어느 cluster로 보낼지 결정한다.
- **Cluster**: 동일한 기능을 하는 엔드포인트들의 논리적 집합이다.
- **Endpoint**: 실제 요청이 전달될 목적지 pod ip:port

### 503?

503 envoy를 통한 통신 장애시, access log를 보면 범인을 바로 잡을수있다. 로그에 찍히는 response flag를 확인해보자

- UH(No Healthy Upstream): 클러스터 내에 살아있는 엔드포인트가 없을때 (파드가 죽었거나 헬스체크 실패)
- NR(No Route Configured): 요청한 URL에 맞는 라우팅 규칙을 찾지 못했을 때,
- UF(Upstream Connection Failure): 타겟 서비스로부터 tcp 연결 자체가 실패했을때 (네트워크 이슈)
- UO(Upstream Overflow): 서킷 브레이커가 작동하여 요청이 차단되었을때

