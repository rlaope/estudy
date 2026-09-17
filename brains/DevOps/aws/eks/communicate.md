# 파드간 통신 과정

### Pod간 통신

파드가 생성되면 네트워크 네임스페이스가 만들어진다.

파드 네트워크 네임스페이스와 루트 네임스페이스를 연결하기 위한 가상 인터페이스(veth)를 사용한다.

> coreDNS처럼 hostIP를 사용하는 일부 파드는 루트 네임스페이스를 사용한다.


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FybvA5%2Fbtr4DtpBE9V%2FgfEQsjlcEpT5JKBjHLkh20%2Fimg.png)

루트 네임스페이스에서 네트워크 인터페이스를 조회하면, 파드 네트워크 인터페이스에 연결된 가상 인터페이스가 보인다.

```bash
# 인터페이스 목록 확인
ip -c -br addr show
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Flwwy8%2Fbtr4xLK7Fki%2F9RX9kHeGb5CM4oj3E3kRi0%2Fimg.png)

<br>

### Route Table

쿠버네티스 네트워크 통신 방향은 라우트 테이블에 영향을 받는다.

파드 defaut gateway는 루트 네임스페이스에 있는 가상 네트워크 인터페이스로 빠져나간다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbcGdGv%2Fbtr4wNbs8Nv%2FOiskqKQaGbuYafs3KCQRyK%2Fimg.png)

루트 네임스페이스로 흘러간 트래픽은 자연스럽게 루트 네임스페이스의 라우트 테이블 영향을 받는다.

트래픽은 라우트 테이블에 의해 노드 내부로 갈지 외부로 갈지 결정된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbZEZTG%2Fbtr4vjaX1YF%2FSRG2xTSoIHVwxPlupLTwUk%2Fimg.png)

**이러한 이유로 파드가 생성되면 가상 네트워크 인터페이스와 라우트 테이블이 추가되는 것을 알 수 있다.**


<br>

### 파드의 외부 통신

파드가 외부와 통신하려면 SNAT(출발지 IP가 노드의 IP로 변경되는 것)가 된다.

SNAT은 `iptables`가 수행한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcwNjIK%2Fbtr4w539TqK%2FPo0EMsiYBCQDkURrcQP3G1%2Fimg.png)


<br>

### 로드 밸런서 타입 서비스

로드밸런서 타입 서비스를 생성,수정,삭제에 관해서는 `aws-load-balancer pod`가 제어한다.

해당 리소스는 api server와 aws api를 사용해 aws에 NLB(default)를 생성한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FpjzS2%2Fbtr4uQfHq3c%2FZ2Rj5iW4Hkv1eAdm6FSI01%2Fimg.png)


로드밸런서 타입 서비스는 결국 AWS NLB를 사용하므로, NLB 설정을 따르게 된다.

NLB가 파드에 접근하는 유형은 2가지다.

1. 노드(인스턴스)에 직접 접근: NLB에 들어온 요청은 노드로 전달된다. 노드에 들어온 서비스 트래픽은 iptables에 의해 제어되며 iptables는 kube-proxy가 설정한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fvwo2w%2Fbtr4xiWAZer%2FPiTK325WnX4ztIJSZ57jK0%2Fimg.png)


2. pod ip로 직접 접근: 유형 1에서는 파드에 접근하기 위해 많은 과정얼 거치지만 유형2에서는 파드에 직접 접근하는 방법이다. 서비스를 거치지않는다고 해서 서비스가 필요 없는 것은 아니며, 서비스에 설정된 엔드포인트를 NLB가 참조하여 pod에 직접 접근하게 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbkGNuE%2Fbtr4t88M3ee%2FckGHlfe2AoloYogIt0ltv1%2Fimg.png)

```yaml
kubectl apply -f deploy.yaml

# deploy.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: deploy-echo
spec:
  replicas: 2
  selector:
    matchLabels:
      app: deploy-websrv
  template:
    metadata:
      labels:
        app: deploy-websrv
    spec:
      terminationGracePeriodSeconds: 0
      containers:
      - name: akos-websrv
        image: k8s.gcr.io/echoserver:1.5
        ports:
        - containerPort: 8080
---
apiVersion: v1
kind: Service
metadata:
  name: svc-nlb-ip-type
  annotations:
    service.beta.kubernetes.io/aws-load-balancer-nlb-target-type: ip
    service.beta.kubernetes.io/aws-load-balancer-scheme: internet-facing
    service.beta.kubernetes.io/aws-load-balancer-healthcheck-port: "8080"
    service.beta.kubernetes.io/aws-load-balancer-cross-zone-load-balancing-enabled: "true"
spec:
  ports:
    - port: 80
      targetPort: 8080
      protocol: TCP
  type: LoadBalancer
  loadBalancerClass: service.k8s.aws/nlb
  selector:
    app: deploy-websrv
```

위와 같이 로드밸런서 타입에 서비스와 디플로이먼트를 생성했고 NLB의 유형은 IP로 설정했다. 유형은 annotations를 통해 설정할 수 있다.

<br>

### EKS 파드간 통신

위에서 설명한 것 처럼 다른 노드간 파드 통신은 VPC 통신이라는 것을 알 수 있다.

EKS pod IP노드는 subnet CIDR에 속한다. 결국 Real IP를 갖게된다.

파드가 vpc real ip를 갖게 됨으로써 다른 노드간 통신에 이점이 생긴다. 그 이점은 real ip 덕분에 파드간 오버레이 통신을 하지 않는 것이다.

<br>

### Overlay 통신

overlay 통신을 이해하기 위해 다음 예제를 준비 했다.

Calico CNI는 overlay 통신을 하는 대표적인 CNI다.

Calico CNI를 사용하면 파드간 통신임에도 불구하고, 출발지와 도착지 노드IP가 설정된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FrTx7O%2Fbtsa9z44UvW%2FzkGkG25GDUccSx4RZwn0p1%2Fimg.png)


Calico CNI를 설치하면, 파드 아이피는 쿠버네티스 클러스터에만 존재하는 가상 아이피다.

그래서 다른 노드에 있는 파드에 패킷을 전달하기 위해서 출발지와 도착지 아이피를 실제 존재하는 노드 아이피를 가지고 캡슐화 한다.

패킷은 받은 노드를 디캡슐화해서 파드 아이피를 획득한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc3F9Oa%2Fbtsa6dokVPi%2FCjyhbbg59TPXkBTnJ4hQR0%2Fimg.png)

<br>

### VPC CNI 통신


VPC CNI는 VPC의 real ip 이므로 캡슐화를 하지 않고 바로 통신한다. VPC 통신이기 때문에 AWS Route Table이 파드간 통신을 라우팅한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F5oKrt%2Fbtsa9zqtmFs%2F14TBAHNeURoNji6bOJ2ku0%2Fimg.png)

할당된 아이피는 AWS 콘솔에서 확인할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FteR0R%2Fbtsbmgyhbmq%2FV6noi2w2RUS1i1KkOKEjak%2Fimg.png)

