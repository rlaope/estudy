# NLB, ALB 생성과 삭제

EKS에 ALB, NLB를 생성해보겠다. 지난글에서 얘기한 ALB 컨트롤러 세팅이 완료된 가정하에 진행된다.

### AWS NLB 생성과 삭제

AWS NLB를 쿠버네티스로 생성하려면 loadbalancer service의 어노테이션과 loadbalancerClass를 설정해야 한다.

annotation은 AWS NLB옵션을 설정합니다. 자세한 내용은 [공식 사용 메뉴얼을 참조](https://kubernetes-sigs.github.io/aws-load-balancer-controller/v2.4/guide/service/nlb/)하길 바란다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FH9qC9%2FbtsbUpIgJ6D%2FDXf4Smxjs3tuB5YaTiIf3K%2Fimg.png)


```yaml
# kubectl apply nlb.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-nlb-test
spec:
  selector:
    matchLabels:
      app: nginx-nlb-test
  replicas: 1
  template:
    metadata:
      labels:
        app: nginx-nlb-test
    spec:
      containers:
        - name: nginx
          image: nginx:latest
          ports:
            - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: nginx-nlb-test
  annotations:
    service.beta.kubernetes.io/aws-load-balancer-nlb-target-type: ip
    service.beta.kubernetes.io/aws-load-balancer-scheme: internet-facing
    service.beta.kubernetes.io/aws-load-balancer-healthcheck-port: "80"
spec:
  type: LoadBalancer
  loadBalancerClass: service.k8s.aws/nlb
  selector:
    app: nginx-nlb-test
  ports:
    - port: 80
      targetPort: 80
      protocol: TCP
```

yaml을 배포하게 되면, aws console에서 네트워크 유형 ELB가 생성된다.

약 5분정도 기다리면 액티브 상태로 변한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FrDRbb%2FbtsbTU9Auer%2FXSrvOVyKeUV13PIqaZFXaK%2Fimg.png)

Active상태가 된 후, Network LoadBalancer DNS주소를 웹 브라우저에 접속하면 nginx디폴트 페이지가 보인다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FQhxRv%2Fbtsb5j8cx8h%2FLFeFVmKcQ8xkgOuRvviHzk%2Fimg.png)



```yaml
kubectl delete -f nlb.yaml
```

위 명령어로 방금 생성한 AWS NLB를 삭제할 수 있다.


<br>

### ALB 생성과 삭제

AWS ALB는 ingress spec으로 생성할 수 있다.

ingress annotation과 ingressClassName을 설정해야한다.

annotation은 AWS ALB 옵션으로 설정한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FccjiiH%2FbtsbSNXkEvv%2FWavoC6r26Zfjn3SYrepfc0%2Fimg.png)


```yaml
# kubectl apply -f alb.yaml

apiVersion: v1
kind: Pod
metadata:
  name: nginx-alb-test
  labels:
    app: nginx-alb-test
spec:
  containers:
  - name: nginx
    image: nginx
    ports:
    - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: nginx-alb-test
spec:
  selector:
    app: nginx-alb-test
  ports:
  - name: http
    port: 80
    targetPort: 80
  type: ClusterIP
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: nginx-alb-test
  annotations:
    alb.ingress.kubernetes.io/scheme: internet-facing
    alb.ingress.kubernetes.io/target-type: ip
spec:
  ingressClassName: alb
  rules:
  - http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: nginx-alb-test
            port:
              number: 80
```

위의 매니페스트를 kubectl apply로 배포하게 되면

aws console에서 Application 유형의 ELB가 생성되고 이것도 5분 기다리면 액티브 상태로 변한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FzoC7B%2Fbtsb6ZuW9E5%2FEk184FndhxvvOVqxuCtdq1%2Fimg.png)

이것도 ALB DNS주소로 접속하면 nginx 디폴트 브라우저가 보이고 `kubectl delete -f alb.yaml`을 통해서 삭제할 수 있다.