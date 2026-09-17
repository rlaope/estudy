# EKS ALB Controller

To create an ALB or NLB using Kubernetes specs in EKS, you must create an ALB Controller. This is because AWS load balancers use the AWS API.

If you try to create an LB-type resource without an ALB Controller, the creation process will get stuck in a Pending state.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FvuGej%2FbtsbVdHDPDG%2F9MvY8LlUnztkIaH5uGEGu0%2Fimg.png)

### How it Works

The ALB Controller watches the Kubernetes API Server to detect ALB events.

When an event occurs, it performs ALB operations using the AWS API.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FsBeRg%2FbtsbTvIQ2S0%2FBX9F6XG4UkcNuSnxZ6uQrK%2Fimg.png)

### Prerequisites

To use the ALB Controller, subnet tags must be configured correctly.

If the tags do not exist, the ALB Controller will not function properly.

When EKS is installed using `eksctl`, subnet tags are automatically configured.

- private subnet: kubernetes.io/role/internal-elb = 1
- public subnet: kubernetes.io/role/elb = 1

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FzFAsM%2Fbtscjkyw8t4%2FGlyY9ETyR6STIo1HRnKeLk%2Fimg.png)

Let's also look into AWS authentication information and the EKS OIDC Provider.

Since the ALB Controller uses the AWS API, it requires AWS authentication credentials.

If authentication information is hardcoded, there's a security risk if the pod is compromised and credentials are stolen.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdgljPL%2FbtsbXItLGh8%2F5bPdAIpXFepqEwsVMksDK0%2Fimg.png)

Therefore, it is safer to use temporary credentials by assigning IAM roles per pod using IRSA (IAM Roles for Service Accounts).

**When creating temporary credentials, the EKS OIDC provider is used.**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FpiPKj%2FbtsbVdAMemT%2FTTRJkaxiuIeI9KZ8Vc3JeK%2Fimg.png)

Refer to the [EKS official documentation](https://docs.aws.amazon.com/ko_kr/eks/latest/userguide/aws-load-balancer-controller.html) to install the provider as shown below.

```bash
CLUSTER_NAME="baisc-cluster"
eksctl utils associate-iam-oidc-provider --cluster ${CLUSTER_NAME} --approve
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fblbwh7%2FbtsbUPmvGEP%2FUBp3bcZc4CbN2EBevtPIc0%2Fimg.png)

<br>

### ALB Controller Installation

I will now install the AWS ALB Controller using Helm and AWS IRSA.

Create an IAM policy to be used by the ALB Controller pod.

The IAM policy was downloaded from the [EKS official documentation](https://docs.aws.amazon.com/ko_kr/eks/latest/userguide/aws-load-balancer-controller.html).

```bash
curl -O https://raw.githubusercontent.com/kubernetes-sigs/aws-load-balancer-controller/v2.4.7/docs/install/iam_policy.json
```

Create the IAM policy using AWS CLI.

```bash
aws iam create-policy \
    --policy-name AWSLoadBalancerControllerIAMPolicy \
    --policy-document file://iam_policy.json
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb7kVwa%2FbtsbVesXMjB%2FpdHWNjBC4ZbCMHo2aXUeek%2Fimg.png)

Use the `eksctl` command to create the IAM role and Kubernetes service account for the ALB Controller.

The ARN of the previously created IAM Policy is also required.

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

This way, the Kubernetes service account and AWS IAM role have been created.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FuFA4K%2FbtsbSNJONMW%2FLRH7HWHRiZ2auFfJc2WJb1%2Fimg.png)

Now that the ALB Controller is ready to be created, I will install it using Helm charts.

First, add the EKS Helm chart.

```bash
helm repo add eks https://aws.github.io/eks-charts
helm repo update
```

Release it with the `helm install` or `helm upgrade` command.

You must set the EKS cluster name in the Helm values.

Since the service account has already been created, `create` is set to `false` to prevent duplicate creation.

```bash
CLUSTER_NAME="${your_eks_cluster_name}"
helm upgrade --install aws-load-balancer-controller eks/aws-load-balancer-controller \
  -n kube-system \
  --set clusterName=${CLUSTER_NAME} \
  --set serviceAccount.create=false \
  --set serviceAccount.name=aws-load-balancer-controller
```

Check if the ALB Controller pod is running in the `kube-system` namespace.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fea32c0%2FbtscfAO4vuC%2FGWc1bt9b01ZyMoMWkpplI1%2Fimg.png)

Installation is now complete.
