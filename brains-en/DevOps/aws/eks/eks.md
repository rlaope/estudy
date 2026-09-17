# EKS

EKS is AWS Managed Kubernetes.

AWS manages the creation, deletion, and other aspects of Kubernetes clusters. This is why the keyword 'Managed' is attached.

**Users only need to pay the cost and focus on how to manage their business logic with Kubernetes.**

There are the following points when installing a Kubernetes cluster directly versus installing it with AWS:
- Kubernetes runs in the AWS environment.
- AWS directly manages the control-plane.

Since Kubernetes operates on AWS, most resources also leverage AWS resources. Worker nodes use EC2 or Fargate, the network is influenced by VPC, using LoadBalancer type resources utilizes ELB, and Ingress resources use AWS Route53.

### EKS Network

AWS directly manages the EKS control-plane.

From a networking perspective, the VPC where EKS is located is directly managed by AWS.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbgAEfW%2Fbtsakgr9Zz4%2FDLCoaI3tLESC1pnx6Ublt1%2Fimg.png)

When creating EKS, you can specify the public access scope, allowing you to create public or private EKS.

Public, as the name suggests, is open and accessible from anywhere, while private is only accessible from within the internal network.

There's a difference in how the Kubernetes API is accessed; public uses the external network, which may incur additional costs.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbYZEMp%2Fbtsak3TMLdO%2Fs3Jdb9fcllKI0fqOpwm0uK%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fr7xny%2Fbtsakg6JTkG%2FFgoSqxDl90bGYTxP87FtjK%2Fimg.png)

### Managed Node Group

EKS can create worker nodes using EC2 or Fargate.

If using EC2, you must choose either a Managed Node Group or a Self-Managed Group.

Setting up a Managed Node Group means that the user configures options only during creation, and AWS manages it thereafter.
