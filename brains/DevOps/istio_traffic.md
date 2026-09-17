# Traffic Control & Security mTLS

서비스 메시의 진정한 가치는 단순한 연결을 넘어서 

실패를 견디는 능력 resiliency와 보이지 않는 보안 zero trust를 코드 한줄수정없이 인프라 계층에서 구현하는데있다.

### Resiliency: 무너지지 않는 시스템 설계

네트워크는 언제나 불완전하다. envoy는 백엔드의 서비스 상태를 실시간으로 감시하여 시스템 전체의 연쇄장애 cascading failure를 방지한다.

#### Circuit Breaing

envoy의 서킷 브레이커는 outlier detection(이상 수치 탐지) 기능을 통해서 작동한다.
- 특정 엔드포인트 pod가 연속적으로 5xx에러를 내뱉거나 응답시간이 너무 길어지면, envoy는 해당 팓드를 건강하지 않음으로 간주하고 일정 시간 동안 로드밸런싱 대상에서 제외한다.
- 효과: 장애가 발생한 서버로 요청이 계속 들어가 리소스 낭비되는것을 막고, 서버가 회복할 시간을 벌어준다.

#### Retry & Timeout

- Retry: 일시적인 네트워크 순발(glitch)로 인한 실패시, envoy가 스스로 재시도한다. (주의할점은 멱등성이 보장된 요청등에 신중하게 적용해야한다. 중복처리되거나 두개 쌓이면 망함)
- Timeout: 무한 대기는 시스템 전체의 스레드 고갈을 초래한다. 일정 시간이 지나면 envoy가 먼저 연결을 끊어 호출자를 보호한다.

```yaml
# 1. 서킷 브레이커 설정 (DestinationRule)
apiVersion: networking.istio.io/v1alpha3
kind: DestinationRule
metadata:
  name: payment-service-cb
spec:
  host: payment-service
  trafficPolicy:
    connectionPool:
      tcp:
        maxConnections: 100           # 최대 연결 수 (넘으면 차단)
      http:
        http1MaxPendingRequests: 100  # 대기열 최대 요청 수
        maxRequestsPerConnection: 10  # 커넥션 당 최대 요청 (메모리 누수 방지)
    outlierDetection:
      consecutive5xxErrors: 3         # 3번 연속 5xx 에러 발생 시 격리
      interval: 10s                   # 에러 체크 주기
      baseEjectionTime: 30s           # 격리 유지 시간 (처음 발생 시 30초)
      maxEjectionPercent: 50          # 전체 인스턴스 중 최대 50%까지만 격리

---
# 2. 리트라이 및 타임아웃 설정 (VirtualService)
apiVersion: networking.istio.io/v1alpha3
kind: VirtualService
metadata:
  name: payment-service-retry
spec:
  hosts:
  - payment-service
  http:
  - route:
    - destination:
        host: payment-service
    timeout: 3s                       # 전체 요청 타임아웃
    retries:
      attempts: 3                     # 최대 3번 재시도
      perTryTimeout: 1s               # 시도 당 타임아웃
      retryOn: "5xx,gateway-error,connect-failure" # 어떤 에러일 때 재시도할지
```

consecutive5xxError 값을 조정해서 언제부터 열지를 확인해야한다. 1개요청중 1개가 실패하면 에러율이 100%니까 이걸로 서킷을 열기엔 애매하니까

그리고 응답시간같은것도 jvm 시스템에서 jit 최적화가 아직 안된 웜업되지 않은 방금 업된 app이나 gc stw가 포함되어서 열린다거나 그런걸 잘 조정해보자.

그리고 좀 실패하더라도 유저경험 가용성이 중요하면 maxEjectionPercent 50 이것도 중요한데 모든 인스턴스가 에러를 낸다고 다 격리해버리면 트래픽을 아예 못받으니까 최소한의 가용성을 위해 50~100%미만으로 설정해도 괜찮다.


### mTLS

istio는 서비스 간 통신의 보안을 상향 평준화한다. 갭라자는 비즈니스 로직에만 집중하고 보안은 envoy가 전담한다.

mTLS(MutualTLS)는 기존의 TLS가 클라이언트가 서버를 얻는 구조라면, mTLS는 **서로가 서로를 확인하는 방식**이다.

- 인증서 발급: istiod(CA역할)가 각 파드 envoyu 사이드카에 짧은 주기의 인증서를 자동으로 발급하고 갱신한다.
- 암호화: 파드 간 통신은 envoy에 의해 자동 암호화되며, 인증서가 없는 외부 호출이나 정당하지 않은 접근은 L4 계층에서 즉시 거부된다.

AuthorizationPolicy(L7 접근 제어): 누가 무엇을 할 수 있는가를 정의한다.
- 세밀한 제어: 단순히 ip기반 차단이 아니라 serviceA는 serviceB의 /orders 경로로 post 요청만 보낼수 있다와 같은 정교한 규칙 설정이 가능함
- identity 기반: ip는 가변적이지만 istio는 서비스계정(ServiceAccount)를 기반으로 신원을 사용하므로 파드가 재생성되어 ip가 바뀌어도 보안 정책은 유지된다.

```yaml
# 1. mTLS 강제화 (PeerAuthentication)
# 이 네임스페이스 내의 모든 통신은 암호화된 mTLS만 허용
apiVersion: security.istio.io/v1beta1
kind: PeerAuthentication
metadata:
  name: default
  namespace: prod-namespace
spec:
  mtls:
    mode: STRICT  # PERMISSIVE는 일반 통신도 허용, STRICT는 mTLS만 허용

---
# 2. 서비스 신원 정의 (ServiceAccount)
apiVersion: v1
kind: ServiceAccount
metadata:
  name: order-service-sa
  namespace: prod-namespace

---
# 3. 상세 접근 제어 (AuthorizationPolicy)
apiVersion: security.istio.io/v1beta1
kind: AuthorizationPolicy
metadata:
  name: restrict-payment-access
  namespace: prod-namespace
spec:
  selector:
    matchLabels:
      app: payment-service           # 'payment-service'에 적용
  action: ALLOW                      # 조건에 맞는 경우만 허용 (화이트리스트)
  rules:
  - from:
    - source:
        principals: ["cluster.local/ns/prod-namespace/sa/order-service-sa"]
    to:
    - operation:
        methods: ["POST"]            # 오직 POST 요청만 허용
        paths: ["/pay/*"]            # 특정 경로만 허용
```

### Observability

circuit breaker, retry등을 적용했다면 반드시 prometheus 메트릭을 통해 정책이 의도대로 작동하는지 검증하자.

- `envoy_cluster_upstream_rq_pending_overflow`: 서킷브레이커에 의해 대기열이 꽉 차서 거부된 요청의 수 (max_pending_requests 임계치 상향 검토)
- `envoy_cluster_outlier_detection_ejections_active`: 현재 서킷브레이커에 의해 격리된 엔드포인트의 수 (해당 서비스의 인스턴스 헬스 체크)
- `istio_requests_total` (resposne code 503): 서킷 브레이커 작동시 envoy는 503을 반환한다. 로그의 Response Flag 교차 검증
