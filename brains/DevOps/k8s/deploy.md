# K8s 배포 전략, Probe

### Goal

- Deployment 배포 전략 4종 비교 및 사용 시점
- readinessProbe, livenessProbe

## Deployment 배포 전략: Rolling, Recreate, Canary, Blue/Green

1. Rolling Update(Default)
	1. 새로운 버전의 pod를 점진적으로 배포하고 기존의 파드를 점진적으로 종료
	2. maxSurge, maxUnavailable 수를 조정 가능함
	3. 무중단 배포가 가능하고 트래픽을 자연스럽게 신버전으로 이전시킬 수 있음
	4. 문제가 발생하면 롤백 탐지 지점이 느려질 수 있는 단점도 존재함
	5. 빠른 롤백이 필요하지 않으며 사용량이 일정하거나 비즈니스 영향력이 적은 서비스에 추천
2. Recreate
	1. 기존 파드를 모두 종료한 뒤, 새 버전의 Pod를 생성하는 방식
	2. 간단하고 명확하며 상태 공유가 불가능한 경우 유리함
	3. 무중단 배포가 불가능해 다운타임 발생
	4. db스키마 변경등으로 동시 실행이 불가능하거나 상태 공유 금지 서비스에 적합(ex 게임서버)
3. Canary
	1. 소수의 트래픽만 새로운 버전으로 보내 테스트 -> 점진적으로 확장
	2. 일반적으로 Service Label Selector HPA Ingress 조합으로 구현함
	3. 실제 사용자 반응 관찰이 가능하며 문제 발생시 빠르게 차단 가능함
	4. 구현 복잡도가 어렵고 자동화가 필요하긴함
	5. 실 사용자 피드백이 중요하거나 버전 안정성이 중요한 서비스에 적합
4. Blue/Green
	1. Blue(기존) Green(새버전)을 동시에 배포하고 트래픽 전환으로 수행함
	2. 롤백이 매우 빠르고 신구버전 동시에 검증이 가능함
	3. 리소스 소모가 크고(두개 다 띄워야해서) 외부 로드밸런서, 서비스 스위치가 필요함
	4. 고가용성 필수, 강력한 테스트 후 전환이 필요한 경우 적합하며 ab테스트, 버전 병렬 유지같은게 예시

<br>

## readinessProbe, livenessProbe

readinessProbe는 **트래픽 수신 가능 여부를 판단함** 실패시 Service에서 제외하며 Ready를 false로 변경함

livenessProbe는 **애플리케이션 자체 생존 여부를 판단함.** 실패시 컨테이너를 재시작하고 컨테이너 상태가 unhealthy로 간주함. 프로세스 hang, deadlock 감지

### 배포 전략에서의 역할
- Rolling Update
	- readinessProbe는 트래픽 전환 안정성 확보에 중요하고 준비되지 않은 Pod에 트래픽 전송을 방지함
	- livenessProbe는 배포 중 비정상 상태 pod 자동 복구에 기여함
- Recreate
	- readinessProbe는 정상 기동 여부 판단 기준이 되며, 기동 실패시 장애 감지에 도움
- Canary/Blue-Green
	- readinessProbes는 트래픽 분배 전 health 체크 핵심
	- livenessProbe는 초기 canary 버전 오류 복구에 기여함

<br>

### 요약

Rolling 점진적 무중단 배포, Recreate 전통적 중단 후 배포, Canary 일부 트래픽 테스트, Blue/Green 전체 전환후 스위치

readinessProbe 트래픽 수신 여부 판단, livenessProbe 컨테이너 복구 트리거


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
      maxSurge: 1           # 새 Pod은 최대 1개까지 추가로 생성 가능
      maxUnavailable: 1     # 기존 Pod 중 최대 1개까지 동시에 내려갈 수 있음
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

### Canary 수동 버전

동일한 앱 두개의 Deployment로 나누고 Service가 동일한 label

`weight` 조절은 Ingress나 Istio 필요

```yaml
# 기존 버전
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

# 카나리아 버전
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

# Service (버전 무관하게 app=myapp만 바라봄)
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

### Blue/Green 트래픽 전환 예시

```yaml
# blue 버전
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

# green 버전
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

# Service (버전 스위치만 바꾸면 트래픽 전환 가능)
apiVersion: v1
kind: Service
metadata:
  name: app-svc
spec:
  selector:
    app: app
    version: green  # 현재 green으로 연결됨. blue로 바꾸면 롤백
  ports:
    - port: 80
      targetPort: 80
```

### Probe Exmaple

```yaml
# 컨테이너가 10초 후 /ready는 OK, /live는 항상 OK
# 하지만 30초 후 /live에서 500 에러 발생 → kubelet이 재시작 유도
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