# Istio Circuit Breaker

Istio(Envoy)에서 서킷 브레이커를 설정하는 방법은 애플리케이션 코드 resilience4j등과 달리 `DestinitionRule` 이라는 k8s 리소스를 정의해야한다.

Istio의 서킷 브레이커는 State Machine이 아닌 Poll Management 방식이라는 차이를 알아두자.

### DestinationRule

먼저 istio의 서킷 브레이커는 크게 두 가지 방식으로 작동한다.
1. **Connection Pool 관리**: 동시 접속자수 제한(TCP/HTTP 레벨 과부화 방지)
2. **Outlier Detection (이상감지)**: 에러가 잦은 파드를 로드밸런싱 풀에서 제외 (우리가 흔히 말하는 서킷 브레이커)

이래는 5번 연속 에러가 나면 30초동안 제외시킨다는 설정 예시이다.

```yaml
apiVersion: networking.istio.io/v1alpha3
kind: DestinationRule
metadata:
  name: my-service-circuit-breaker
spec:
  host: my-service.default.svc.cluster.local # 대상 서비스
  trafficPolicy:
    # 1. 커넥션 풀 설정 (과부하 방지)
    connectionPool:
      tcp:
        maxConnections: 100       # 최대 TCP 연결 수 제한
      http:
        http1MaxPendingRequests: 1024 # 대기열(Queue) 크기 제한
        maxRequestsPerConnection: 10  # 커넥션 당 최대 요청 수

    # 2. 이상 감지 설정 (실질적인 서킷 브레이커 로직)
    outlierDetection:
      consecutive5xxErrors: 5     # 5번 연속 5xx 에러 발생 시
      interval: 10s               # 10초마다 스캔해서 판단
      baseEjectionTime: 30s       # 최초 30초 동안 로드밸런싱 풀에서 제외(Open)
      maxEjectionPercent: 100     # 전체 파드의 최대 100%까지 제외 허용 (0%면 작동 안 함)
```

Istio의 Half-Open 메커니즘(작동 원리)

Resilience4j에는 Half-Open이라는 state가 있고 서킷이 오픈되었을때 다시 롤백해도 괜찮은지

확인해보기 위한 상태로 사용된다. Istio에서는 어떻게 다를까?

일반적인 라이브러리는 Closed가 정상, Open이 차단된상태이며 Half-Open은 일정시간이후 딱 1개의 요청만 흘려보내서probe 성공하면 closed 실패하면 open 상태로 전이한다.

### Istio Ejection

Istio에서는 명시적인 Half Open이라는 상태값이 없다, 대신 잠깐 쫓아냈다가 슬그머니 다시 끼워주는 방식을 쓴다.

1. Healthy(Closed): 모든 파드가 로드밸런싱 풀에 들어잇고 트래픽을 받음
2. Ejection(Open): 특정 파드가 `consecutime5xxErrors` 조건을 충족하면, Envoy는 즉시 그 파드를 로드밸런싱 풀에서 뺀다. 이 상태가 `baseEjectionTime`동안 지속된다.
3. Re-joining: 30초가 지나면 envoy는 별도 probe 과정없이 해당 파드를 다시 로드밸런싱 풀에 넣는다 이게 istio식의 half-open이고 들어가자마자 또 에러를내면 다시 ejection한다. 이때는 시간이 baseEjectionTime x 2로 늘어난다. (Exponential Backoff)

정리하면 Istio의 서킷 브레이커는 애플리케이션 레벨 동작방식과 살짝 다르며 

애플리케이션 라이브러리는 State Machine 기반으로 Half-Open 상태에서 테스트 요청을 보내는등 정교하게 상태를 관리하는 반면

Istio는 이상치 감지 outlier detection 방식을 사용해 문제가 있는 파드를 로드밸런싱 그룹에서 잠시 축출 ejection 했다가, 일정 시간이 지나면 다시 그룹에 합류시키는 방식으로 동작한다.

개별 요청의 정교한 제어보다는 전체 클러스터의 위생관리(부하 분산 풀에서 썩은 요청 걸러내기) 관점에서는 더 효율적이라고 생각된다면 인프라 레벨에서 관리하자.