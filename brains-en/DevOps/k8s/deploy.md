# K8s Deployment Strategies, Probes

### Goal

- Comparison of 4 Deployment Strategies and When to Use Them
- readinessProbe, livenessProbe

## Deployment Strategies: Rolling, Recreate, Canary, Blue/Green

1. Rolling Update (Default)
    1. Gradually deploys new versions of pods and gradually terminates existing pods.
    2. `maxSurge` and `maxUnavailable` values can be adjusted.
    3. Enables zero-downtime deployment and allows traffic to naturally shift to the new version.
    4. A disadvantage is that if issues arise, rollback detection can be slow.
    5. Recommended for services that don't require fast rollbacks, have consistent usage, or have low business impact.
2. Recreate
    1. Terminates all existing pods, then creates new versions of pods.
    2. Simple and clear, advantageous when state sharing is not possible.
    3. Zero-downtime deployment is not possible, leading to downtime.
    4. Suitable for services where concurrent execution is not possible due to database schema changes or where state sharing is prohibited (e.g., game servers).
3. Canary
    1. Sends a small percentage of traffic to the new version for testing -> gradually expands.
    2. Typically implemented using a combination of Service Label Selector, HPA, and Ingress.
    3. Allows observation of real user reactions and quick blocking if issues arise.
    4. Implementation complexity is high and automation is required.
    5. Suitable for services where real user feedback is crucial or version stability is important.
4. Blue/Green
    1. Deploys Blue (old version) and Green (new version) simultaneously and performs traffic switching.
    2. Rollbacks are very fast, and both old and new versions can be validated simultaneously.
    3. High resource consumption (as both versions must be running) and requires an external load balancer or service switch.
    4. Suitable for scenarios requiring high availability and switching after rigorous testing, with examples like A/B testing and maintaining parallel versions.

<br>

## readinessProbe, livenessProbe

`readinessProbe` **determines if a pod can receive traffic.** If it fails, the pod is removed from the Service and its Ready status is set to false.

`livenessProbe` **determines if the application itself is alive.** If it fails, the container is restarted, and the container's status is considered unhealthy. It detects process hangs and deadlocks.

### Role in Deployment Strategies
- Rolling Update
    - `readinessProbe` is crucial for ensuring traffic transition stability and preventing traffic from being sent to unprepared Pods.
    - `livenessProbe` contributes to automatic recovery of unhealthy pods during deployment.
- Recreate
    - `readinessProbe` serves as a criterion for determining normal startup, aiding in fault detection if startup fails.
- Canary/Blue-Green
    - `readinessProbes` are key for health checks before traffic distribution.
    - `livenessProbe` contributes to error recovery in the initial canary version.

<br>

### Summary

Rolling: gradual zero-downtime deployment. Recreate: traditional stop-and-deploy. Canary: test with a portion of traffic. Blue/Green: switch after full deployment.

`readinessProbe`: determines traffic reception capability. `livenessProbe`: triggers container recovery.

## Example Manifest

### Rolling Update

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rolling-deploy-sample
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1           # A maximum of 1 new Pod can be created additionally
      maxUnavailable: 1     # A maximum of 1 existing Pod can be taken down simultaneously
  selector:
    matchLabels:
      app: sample
  template:
    metadata:
      labels:
        app: sample
    spec:
      containers:
        - name: app
          image: nginx:1.25
          ports:
            - containerPort: 80
          readinessProbe:
            httpGet:
              path: /
              port: 80
            initialDelaySeconds: 5
            periodSeconds: 5
          livenessProbe:
            httpGet:
              path: /
              port: 80
            initialDelaySeconds: 15
            periodSeconds: 20
```

### Recreate

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: recreate-deploy-sample
spec:
  replicas: 2
  strategy:
    type: Recreate
  selector:
    matchLabels:
      app: sample-recreate
  template:
    metadata:
      labels:
        app: sample-recreate
    spec:
      containers:
        - name: app
          image: httpd:2.4
          ports:
            - containerPort: 80
          readinessProbe:
            httpGet:
              path: /
              port: 80
            initialDelaySeconds: 5
            periodSeconds: 5
```

### Canary Manual Version

Divides the same application into two Deployments, with the Service using the same label.

`weight` adjustment requires Ingress or Istio.

```yaml
# Stable version
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-stable
spec:
  replicas: 4
  selector:
    matchLabels:
      app: myapp
      version: stable
  template:
    metadata:
      labels:
        app: myapp
        version: v1
    spec:
      containers:
        - name: myapp
          image: myapp:1.0

---

# Canary version
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-canary
  version: v2
spec:
  replicas: 1
  selector:
    matchLabels:
      app: myapp
      version: canary
  template:
    metadata:
      labels:
        app: myapp
        version: canary
    spec:
      containers:
        - name: myapp
          image: myapp:1.1

---

# Service (looks only at app=myapp, regardless of version)
apiVersion: v1
kind: Service
metadata:
  name: myapp-svc
spec:
  selector:
    app: myapp
  ports:
    - port: 80
      targetPort: 80
```

```yaml
apiVersion: networking.istio.io/v1beta1
kind: DestinationRule
metadata:
  name: myapp-destination
spec:
  host: myapp-svc
  subsets:
    - name: v1
      labels:
        version: v1
    - name: v2
      labels:
        version: v2
```

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: myapp-virtualservice
spec:
  hosts:
    - myapp-svc
  http:
    - route:
        - destination:
            host: myapp-svc
            subset: v1
          weight: 80
        - destination:
            host: myapp-svc
            subset: v2
          weight: 20
```

### Blue/Green Traffic Switching Example

```yaml
# blue version
apiVersion: apps/v1
kind: Deployment
metadata:
  name: app-blue
spec:
  replicas: 3
  selector:
    matchLabels:
      app: app
      version: blue
  template:
    metadata:
      labels:
        app: app
        version: blue
    spec:
      containers:
        - name: app
          image: app:v1

# green version
apiVersion: apps/v1
kind: Deployment
metadata:
  name: app-green
spec:
  replicas: 3
  selector:
    matchLabels:
      app: app
      version: green
  template:
    metadata:
      labels:
        app: app
        version: green
    spec:
      containers:
        - name: app
          image: app:v2

# Service (traffic can be switched by changing only the version switch)
apiVersion: v1
kind: Service
metadata:
  name: app-svc
spec:
  selector:
    app: app
    version: green  # Currently connected to green. Changing to blue will rollback.
  ports:
    - port: 80
      targetPort: 80
```

### Probe Example

```yaml
# After 10 seconds, /ready is OK, /live is always OK
# However, after 30 seconds, a 500 error occurs at /live → kubelet induces a restart
readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 5

livenessProbe:
  httpGet:
    path: /live
    port: 8080
  initialDelaySeconds: 15
  periodSeconds: 10
```
