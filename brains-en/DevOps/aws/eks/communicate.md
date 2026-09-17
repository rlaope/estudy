# Pod Communication Process

### Pod Communication

When a Pod is created, a network namespace is created.

A virtual interface (veth) is used to connect the Pod's network namespace with the root namespace.

> Some Pods, like coreDNS, which use hostIP, utilize the root namespace.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FybvA5%2Fbtr4DtpBE9V%2FgfEQsjlcEpT5JKBjHLkh20%2Fimg.png)

When you query network interfaces in the root namespace, you can see the virtual interface connected to the Pod's network interface.

```bash
# 인터페이스 목록 확인
ip -c -br addr show
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Flwwy8%2Fbtr4xLK7Fki%2F9RX9kHeGb5CM4oj3E3kRi0%2Fimg.png)

<br>

### Route Table

Kubernetes network communication direction is influenced by the route table.

A Pod's default gateway exits through the virtual network interface in the root namespace.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbcGdGv%2Fbtr4wNbs8Nv%2FOiskqKQaGbuYafs3KCQRyK%2Fimg.png)

Traffic flowing into the root namespace is naturally affected by the root namespace's route table.

The route table determines whether traffic goes inside or outside the node.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbZEZTG%2Fbtr4vjaX1YF%2FSRG2xTSoIHVwxPlupLTwUk%2Fimg.png)

**For these reasons, when a Pod is created, a virtual network interface and a route table are added.**

<br>

### Pod External Communication

For a Pod to communicate externally, SNAT (Source Network Address Translation, where the source IP changes to the node's IP) occurs.

`iptables` performs SNAT.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcwNjIK%2Fbtr4w539TqK%2FPo0EMsiYBCQDkURrcQP3G1%2Fimg.png)

<br>

### Load Balancer Type Service

The `aws-load-balancer pod` controls the creation, modification, and deletion of LoadBalancer type Services.

This resource uses the API server and AWS API to create an NLB (default) in AWS.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FpjzS2%2Fbtr4uQfHq3c%2FZ2Rj5iW4Hkv1eAdm6FSI01%2Fimg.png)

Since LoadBalancer type Services ultimately use AWS NLB, they follow NLB configurations.

There are two ways an NLB can access a Pod.

1. Direct access to the node (instance): Requests arriving at the NLB are forwarded to the node. Service traffic arriving at the node is controlled by iptables, which is configured by kube-proxy.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fvwo2w%2Fbtr4xiWAZer%2FPiTK325WnX4ztIJSZ57jK0%2Fimg.png)

2. Direct access via Pod IP: While type 1 involves many steps to access a Pod, type 2 is a method of directly accessing the Pod. Not going through the Service does not mean the Service is unnecessary; the NLB references the endpoints configured in the Service to directly access the Pod.

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

As shown above, a LoadBalancer type Service and Deployment were created, and the NLB type was set to IP. The type can be configured via annotations.

<br>

### EKS Pod Communication

As explained above, Pod communication between different nodes is VPC communication.

EKS Pod IPs belong to the subnet CIDR. Ultimately, they have Real IPs.

By having a VPC real IP, Pods gain an advantage in communication between different nodes. This advantage is that they do not perform overlay communication between Pods thanks to the real IP.

<br>

### Overlay Communication

To understand overlay communication, I've prepared the following example.

Calico CNI is a representative CNI that performs overlay communication.

When using Calico CNI, even for Pod-to-Pod communication, the source and destination Node IPs are set.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FrTx7O%2Fbtsa9z44UvW%2FzkGkG25GDUccSx4RZwn0p1%2Fimg.png)

When Calico CNI is installed, Pod IPs are virtual IPs that exist only within the Kubernetes cluster.

Therefore, to deliver packets to Pods on other nodes, the source and destination IPs are encapsulated with the actual existing Node IPs.

The receiving node decapsulates the packet to obtain the Pod IP.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc3F9Oa%2Fbtsa6dokVPi%2FCjyhbbg59TPXkBTnJ4hQR0%2Fimg.png)

<br>

### VPC CNI Communication

Since VPC CNI uses real IPs from the VPC, it communicates directly without encapsulation. Because it's VPC communication, the AWS Route Table routes Pod-to-Pod communication.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F5oKrt%2Fbtsa9zqtmFs%2F14TBAHNeURoNji6bOJ2ku0%2Fimg.png)

The assigned IPs can be checked in the AWS console.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FteR0R%2Fbtsbmgyhbmq%2FV6noi2w2RUS1i1KkOKEjak%2Fimg.png)
