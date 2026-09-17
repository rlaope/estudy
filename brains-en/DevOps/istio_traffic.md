# Traffic Control & Security mTLS

The true value of a service mesh goes beyond simple connectivity.

It lies in implementing resiliency (the ability to withstand failures) and zero trust security at the infrastructure layer, without modifying a single line of code.

### Resiliency: Designing an Unbreakable System

Networks are always imperfect. Envoy continuously monitors the health of backend services in real-time to prevent cascading failures across the entire system.

#### Circuit Breaking

Envoy's circuit breaker operates through its outlier detection feature.
- If a specific endpoint pod continuously returns 5xx errors or its response time becomes too long, Envoy considers that pod unhealthy and removes it from the load balancing pool for a certain period.
- Effect: This prevents requests from continuously going to a failed server, wasting resources, and gives the server time to recover.

#### Retry & Timeout

- Retry: In case of failure due to a temporary network glitch, Envoy automatically retries the request. (Caution: This should be applied carefully to idempotent requests, etc. Duplication or processing twice can lead to issues.)
- Timeout: Indefinite waiting can lead to thread exhaustion across the entire system. After a certain period, Envoy proactively cuts the connection to protect the caller.

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

You need to adjust the consecutive5xxError value to determine when to open the circuit. If 1 out of 1 request fails, the error rate is 100%, so opening the circuit based on that might be ambiguous.

Also, for response times, consider adjusting for cases like newly launched applications that haven't warmed up with JIT optimization in JVM systems, or when GC stop-the-world events are included.

Furthermore, if user experience availability is crucial even with some failures, maxEjectionPercent 50 is also important. If all instances are isolated when they report errors, no traffic will be received at all. So, setting it to less than 50-100% for minimal availability is acceptable.

### mTLS

Istio raises the security baseline for inter-service communication. Developers can focus solely on business logic, while Envoy handles security.

While traditional TLS involves a client authenticating a server, mTLS (Mutual TLS) is a method where **both parties verify each other**.

- Certificate Issuance: Istiod (acting as a CA) automatically issues and renews short-lived certificates to each pod's Envoy sidecar.
- Encryption: Inter-pod communication is automatically encrypted by Envoy, and external calls without certificates or unauthorized access are immediately rejected at the L4 layer.

AuthorizationPolicy (L7 Access Control): Defines who can do what.
- Granular Control: It allows for sophisticated rule settings, such as serviceA being able to send only POST requests to serviceB's /orders path, rather than just IP-based blocking.
- Identity-based: While IPs are mutable, Istio uses identities based on ServiceAccounts, so security policies remain consistent even if a pod is recreated and its IP changes.

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

If you've applied circuit breakers, retries, etc., you must verify that the policies are working as intended through Prometheus metrics.

- `envoy_cluster_upstream_rq_pending_overflow`: The number of requests rejected because the queue was full due to the circuit breaker (review increasing the max_pending_requests threshold).
- `envoy_cluster_outlier_detection_ejections_active`: The number of endpoints currently isolated by the circuit breaker (check the health of the corresponding service instances).
- `istio_requests_total` (response code 503): When the circuit breaker operates, Envoy returns a 503. Cross-verify with the Response Flag in the logs.
