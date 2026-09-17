# EKS CNI

> CNI, short for Container Network Interface, is a standard for creating plugins that control network communication between containers.

Kubernetes delegates IP assignment and network configuration for pods to the CNI.

AWS developed the VPC CNI, and EKS uses this CNI.

This allows the use of **VPC features (such as security groups, VPC flow logs, etc.)**.

When EKS is installed, VPC CNI pods are automatically created. They are created as a DaemonSet and named aws-node.

If you enter `kubectl -n kube-system get ds aws-node` to check, you will see:

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FoZvZj%2FbtsbmYjLgmB%2FfyOl6FKPoKCFK7obAMhMuK%2Fimg.png)

that it is running like this.

### VPC CNI Features

The core function of VPC CNI is to assign pod IPs and manage communication between pods.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FtL1bv%2Fbtsa8iic4Ot%2FdHwLskVINhu2jGarQnIKfK%2Fimg.png)

### How EKS Assigns IPs to Pods

Let's look at how EKS assigns IPs to created pods. When VPC CNI assigns an IP to a pod, it allocates an IP from the node's subnet CIDR range. If nodeA's CIDR is 10.0.10.0/24, then nodeA's pod IPs will belong to the 10.0.10.0/24 range.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb8v5Th%2FbtsaXmzqOxY%2FWuK3Gre5nV4ZkAWNbzF4U1%2Fimg.png)

From an AWS perspective, rather than a CNI perspective, the IP assigned by VPC CNI is ultimately an IP assigned to an ENI.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbykqvu%2Fbtsa9JGwhfw%2FpKOwybx9V6SNwlI4o7Vy7K%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fchk3OQ%2FbtsaVeuUliJ%2FSkew4LkjgGII7IXu1WIC21%2Fimg.png)

Referring to the [AWS blog](https://aws.amazon.com/ko/blogs/containers/amazon-vpc-cni-increases-pods-per-node-limits/), the L-PAM daemon within VPC CNI assigns IPs.

[[EKS]]
