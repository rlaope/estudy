# EKS ALB Controller

EKS에서 Kubernetes spec으로 ALB 혹은 NLB를 생성하려면 꼭 ALB Controller를 생성해야 한다. 그 이유는 AWS 로드밸런서는 AWS API를 사용하기 때문이다.

만약 ALB Controller가 없는 상태에서 LB 유형의 리소스를 생성하려한다면 생성단계가 Pending으로 멈추게 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FvuGej%2FbtsbVdHDPDG%2F9MvY8LlUnztkIaH5uGEGu0%2Fimg.png)

### 동작원리

ALB Controller는 Kubernetes API Server를 watch(감시)하여 ALB 이벤트 발생을 인지한다.

이벤트가 발생하면 AWS API를 사용하여 ALB 작업을 수행한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FsBeRg%2FbtsbTvIQ2S0%2FBX9F6XG4UkcNuSnxZ6uQrK%2Fimg.png)


### 실행 전제조건

ALB Controller를 사용하기 위해서는 서브넷 태그가 적절히 설정되어 있어야 한다.

태그가 존재하지 않는다면 ALB 컨트롤러는 제대로 동작하지 않는다.

eksctl을 사용하여 EKS를 설치하면, 서브넷 태그가 자동으로 설정되어 있다.

- private subnet: kubernetes.io/role/internal-elb = 1
- public subnet: kubernetes.io/role/elb = 1

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FzFAsM%2Fbtscjkyw8t4%2FGlyY9ETyR6STIo1HRnKeLk%2Fimg.png)

그리고 AWS 인증정보 EKS OIDC Provider에 대해서도 알아보자.

ALB controller는 AWS API를 사용하므로 AWS 인증정보가 필요하다.

인증 정보를 하드코딩하게 되면, pod가 해킹당하여 인증정보가 탈취당했을 때 보안 위험이 생긴다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdgljPL%2FbtsbXItLGh8%2F5bPdAIpXFepqEwsVMksDK0%2Fimg.png)

그래서 파드별로 IAM role을 할당하는 ISRA를 사용해 임시로 사용하는 임시 자격 증명을 사용하는게 안전하다.

**임시 자격 증명을 생성할 때 EKS OIDC provider를 사용한다.**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FpiPKj%2FbtsbVdAMemT%2FTTRJkaxiuIeI9KZ8Vc3JeK%2Fimg.png)

[EKS 공식문서](https://docs.aws.amazon.com/ko_kr/eks/latest/userguide/aws-load-balancer-controller.html) 를 참고하여 아래처럼 프로바이더를 설치한다.

```bash
CLUSTER_NAME="baisc-cluster"
eksctl utils associate-iam-oidc-provider --cluster ${CLUSTER_NAME} --approve
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fblbwh7%2FbtsbUPmvGEP%2FUBp3bcZc4CbN2EBevtPIc0%2Fimg.png)


<br>

### ALB Controller 설치

helm과 AWS ISRA를 사용해 AWS ALB controller를 설치해보겠다.

ALB Controller pod에 사용할 IAM policy를 생성한다.

IAM policy는 [EKS공식문서](https://docs.aws.amazon.com/ko_kr/eks/latest/userguide/aws-load-balancer-controller.html)에서 제공한 policy를 다운로드 받았다.

```bash
curl -O https://raw.githubusercontent.com/kubernetes-sigs/aws-load-balancer-controller/v2.4.7/docs/install/iam_policy.json
```

AWS CLI를 사용해 IAM policy를 생성한다.

```bash
aws iam create-policy \
    --policy-name AWSLoadBalancerControllerIAMPolicy \
    --policy-document file://iam_policy.json
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb7kVwa%2FbtsbVesXMjB%2FpdHWNjBC4ZbCMHo2aXUeek%2Fimg.png)

eksctl 명령어를 사용해 ALB Controller에 사용할 IAM role과 kubernetes serviceaccount를 생성한다.

이전에 생성한 IAM Policy arn도 함께 필요하다.

```bash
POLICY_ARN=$(aws iam list-policies --query 'Policies[?PolicyName==`AWSLoadBalancerControllerIAMPolicy`].Arn' --output text)
ROLE_NAME="AmazonEKSLoadBalancerControllerRole"
CLUSTER_NAME="${eks-cluster-name}"

eksctl create iamserviceaccount \
  --cluster ${CLUSTER_NAME} \
  --namespace=kube-system \
  --name=aws-load-balancer-controller \
  --role-name ${ROLE_NAME} \
  --attach-policy-arn=${POLICY_ARN} \
  --approve
```

이렇게 kubernetes serviceaccount와 aws iam role이 생성되었다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FuFA4K%2FbtsbSNJONMW%2FLRH7HWHRiZ2auFfJc2WJb1%2Fimg.png)


이제 ALB Controller를 생성할 준비가 끝이 났으니 helm charts를 이용해 ALB Controller를 설치해보겠다.

먼저, EKS helm chart를 추가한다.

```bash
helm repo add eks https://aws.github.io/eks-charts
helm repo update
```

helm install or update 명령어로 릴리즈한다. 

EKS 클러스터 이름을 helm values에 꼭 설정해야 한다.

serviceaccount는 이미 생성했으므로 중복 생성하지 않기 위해 false로 설정했다.

```bash
CLUSTER_NAME="${your_eks_cluster_name}"
helm upgrade --install aws-load-balancer-controller eks/aws-load-balancer-controller \
  -n kube-system \
  --set clusterName=${CLUSTER_NAME} \
  --set serviceAccount.create=false \
  --set serviceAccount.name=aws-load-balancer-controller
```

kube-system 네임스페이스에 ALB Controller 파드가 실행중인지 확인한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fea32c0%2FbtscfAO4vuC%2FGWc1bt9b01ZyMoMWkpplI1%2Fimg.png)

이렇게 설치가 완료되었다.

