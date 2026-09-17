# Desired State

쿠버네티스의 핵심은 Desired State라고 할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FczxeKT%2Fbtq4db5AceN%2FFJQINxjdjbGA2P5J4kWa6K%2Fimg.png)

쿠버네티스는 원하는 상태를 계속 체크하고 문제가 있다면 자동으로 조치합니다. 그렇다면 원하는 상태는 무엇일까요?

원하는 상태란 관리자가 바라는 환경을 의미하고, 좀 더 구체적으로 얼마나 많은 웹서버가 구동되고 있으면 좋은지, 몇 번 포트로 서비스하기를 원하는 등을 말합니다.

쿠버네티스는 복잡하고 다양한 작업을 하지만 자세히 들여다보면, 현재 상태를 모니터링하면서 관리자가 설정한 원하는 상태를 유지하려고 내부적으로 이런저런 작업을 단순화 하는 단순한 로직을 가지고 있습니다.

<br>

## 원하는 상태 설정

쿠버네티스에서는 YAML, JSON 파일을 사용해, 원하는 애플리케이션의 상태 (Desired State)를 명시적으로 설정할 수 있습니다.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
    name: nginx-deployment
spec:
    selector:
        matchLables:
            app: nginx
    replicas: 2
    template:
        metadata:
            lables:
                app: nginx
        spec:
            containers:
            - name: nginx
            image: nginx:1.14.2
            ports:
            - containerPort: 80
```

작성을 완료하고 나면, 쿠버네티스가 작성 파일에 정의된 사양에 따라 컨테이너의 라이프사이클을 관리합니다. 

쿠버네티스를 사용하면 시스템으로 컨테이너 기반 애플리케이션 및 서비스의 설정, 라이프사이클, 스케일 등을 관리할 수 있게 됩니다.

<br>

## 어떠한 것들을 설정할 수 있을까?
쿠버네티스는 다양한 요소들을 설정할 수 있습니다.

쿠버네티스는 `Pod`을 배포할 때, 컨테이너에 필요한 각 리소스의 양을 선택적으로 지정할 수 있습니다.

일반적으로는 CPU, 메모리(RAM) 그리고 다양한 것들을 제한할 수 있습니다.

```yaml
apiVersion: v1
kind: Pod
metadata:
    name: frontend
spec:
    containers:
    - name: app
      image: images.my-company.example/app:v4
      resources:
        requests:
            memory: "64Mi"
            cpu: "250m"
        limits:
            memory: "128Mi"
            cpu: "500m"
    - name: log-aggregator
      image: image.mycompany.example/log-aggregator:v6
      resources:
        requests:
            memory: "64Mi"
            cpu: "250m"
        limits:
            memory: "128Mi"
            cpu: "500m"
    
```

YAML 파일에 다음과 같이 컨테이너에 대한 리소스 제한(limit)을 지정하면, 설정한 제한보다 많은 리소스를 사용할 수 없도록 해당 제한을 적용합니다.  
그뿐만 아니라, 해당 시스템 리소스의 최소 요청량 또한 설정할 수 있습니다. 