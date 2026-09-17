# Istio Circuit Breaker

Unlike application-level code like resilience4j, setting up a circuit breaker in Istio (Envoy) requires defining a Kubernetes resource called `DestinationRule`.

It's important to note that Istio's circuit breaker uses a Poll Management approach rather than a State Machine.

### DestinationRule

First, Istio's circuit breaker operates in two main ways.
1. **Connection Pool Management**: Limits the number of concurrent connections (prevents overload at TCP/HTTP levels).
2. **Outlier Detection**: Excludes error-prone pods from the load balancing pool (what we commonly refer to as a circuit breaker).

Below is an example configuration that excludes a pod for 30 seconds if it encounters 5 consecutive errors.

```yaml
apiVersion: networking.istio.io/v1alpha3
kind: DestinationRule
metadata:
  name: my-service-circuit-breaker
spec:
  host: my-service.default.svc.cluster.local # Target service
  trafficPolicy:
    # 1. Connection Pool Settings (Overload Prevention)
    connectionPool:
      tcp:
        maxConnections: 100       # Limit max TCP connections
      http:
        http1MaxPendingRequests: 1024 # Limit queue size
        maxRequestsPerConnection: 10  # Max requests per connection

    # 2. Outlier Detection Settings (Actual Circuit Breaker Logic)
    outlierDetection:
      consecutive5xxErrors: 5     # If 5 consecutive 5xx errors occur
      interval: 10s               # Scan and evaluate every 10 seconds
      baseEjectionTime: 30s       # Exclude from load balancing pool for initial 30 seconds (Open)
      maxEjectionPercent: 100     # Allow up to 100% of all pods to be ejected (0% means it won't activate)
```

Istio's Half-Open Mechanism (How it works)

Resilience4j has a Half-Open state, which is used to check if it's okay to roll back when the circuit is open. How is it different in Istio?

In typical libraries, Closed is normal, Open is blocked, and Half-Open transitions to Closed if a single probe request succeeds after a certain period, or to Open if it fails.

### Istio Ejection

Istio does not have an explicit Half-Open state. Instead, it temporarily ejects a pod and then subtly reintroduces it.

1. Healthy (Closed): All pods are in the load balancing pool and receiving traffic.
2. Ejection (Open): If a specific pod meets the `consecutive5xxErrors` condition, Envoy immediately removes that pod from the load balancing pool. This state persists for `baseEjectionTime`.
3. Re-joining: After 30 seconds, Envoy reintroduces the pod into the load balancing pool without a separate probe process. This is Istio's version of half-open. If it immediately causes another error, it is ejected again. In this case, the time increases to `baseEjectionTime` x 2 (Exponential Backoff).

In summary, Istio's circuit breaker operates slightly differently from application-level mechanisms.

While application libraries manage states meticulously, for example, by sending test requests in a Half-Open state based on a State Machine,

Istio uses an outlier detection approach to temporarily eject problematic pods from the load balancing group and then reintroduces them after a certain period.

If you find it more efficient from the perspective of overall cluster health management (filtering out 'rotten' requests from the load balancing pool) rather than precise control over individual requests, then manage it at the infrastructure level.
