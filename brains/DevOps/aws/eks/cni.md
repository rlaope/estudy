# EKS CNI

> CNI는 Container Network Interface의 약어로 컨테이너간 네트워크 통신을 제어할 수 있는 플러그인을 만들기 위한 표준이다.

쿠버네티스는 파드의 IP할당과 네트워크 설정을 CNI에게 위임하고 있다.

그리고 AWS는 VPC CNI를 개발했고 EKS는 해당 CNI를 사용하고 있다.

**VPC기능(security group, vpc flow 등)**을 사용할 수 있는 것이다.

EKS를 설치하면 자동으로 VPC CNI 파드가 생성이 된다. DeamonSet형태로 생성되며 이름은 aws-node다.

`kubectl -n kube-system get ds aws-node`를 입력해서 확인해보면

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FoZvZj%2FbtsbmYjLgmB%2FfyOl6FKPoKCFK7obAMhMuK%2Fimg.png)

이렇게 띄워져 있는 것을 확인할 수 있다.

### VPC CNI 기능

VPC CNI의 핵심 기능은 파드 IP를 할당하고 파드간 통신을 담당한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FtL1bv%2Fbtsa8iic4Ot%2FdHwLskVINhu2jGarQnIKfK%2Fimg.png)

### EKS가 파드에 IP를 할당하는 방식

EKS는 생성된 파드에 IP를 할당하는 방식을 알아보자 VPC CNI는 파드에 아이피를 할당할 때 노드 속 서브넷 IP 대역을 할당한다. nodeA의 CIDR이 10.0.10.0/24라면 nodeA의 파드의 아이피는 10.0.10.0/24 대역에 속하게 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb8v5Th%2FbtsaXmzqOxY%2FWuK3Gre5nV4ZkAWNbzF4U1%2Fimg.png)

CNI 관점이 아닌 AWS의 관점으로 할당 과정을 살펴보게 된다면, VPC CNI가 할당한 IP는 결국 ENI에 IP가 할당 된 것이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbykqvu%2Fbtsa9JGwhfw%2FpKOwybx9V6SNwlI4o7Vy7K%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fchk3OQ%2FbtsaVeuUliJ%2FSkew4LkjgGII7IXu1WIC21%2Fimg.png)

[AWS블로그](https://aws.amazon.com/ko/blogs/containers/amazon-vpc-cni-increases-pods-per-node-limits/)를 참고해서 동작 과정을 살펴보면 VPC CNI에 있는 L-PAM 데몬이 IP를 할당하게 된다.




[[EKS]]