# EKS

EKS는 AWS Managed Kubernetes다.

AWS가 쿠버네티스의 클러스터를 생성, 삭제등 관리한다. 그렇기에 Managed라는 키워드가 붙어있다.

**사용자는 비용을 지불하고 비즈니스 로직을 쿠버네티스로 어떻게 관리할까만을 고민한면 된다.**

쿠버네티스 클러스터를 직접 설치하는 것과 AWS로 설치하는 것은 다음과 같은 점이 있다.
- 쿠버네티스가 AWS 환경에서 실행된다.
- control-plane을 AWS가 직접 관리한다.

AWS에서 쿠버네티스가 동작하기 때문에 대부분의 리소스들도 AWS의 리소스들로 활용된다. 워커 노드는 EC2, Fargate가 사용되고 네트워크는 VPC에 영향을 받으며 LoadBalancer 타입의 리소스를 사용하면 ELB를 사용하고 Ingress 리소스는 AWS Route53을 사용한다.

### EKS Network

EKS control-plane을 직접 관리합니다.

네트워크 관점으로 설명하면 EKS에 위치한 VPC는 AWS가 직접 관리합니다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbgAEfW%2Fbtsakgr9Zz4%2FDLCoaI3tLESC1pnx6Ublt1%2Fimg.png)


EKS를 생성할 때 공개 범위를 지정할 수 있는데 public, private EKS를 생성할 수 있습니다.

public은 말 그대로 개방되어있어 어디서든 EKS로 접근할 수 있고 private은 내부 네트워크에서만 접근할 수 있습니다.

kubernetes api에 접근하는 방식에 차이가 있는데, public은 네트워크 외부 망을 사용하기 때문에 추가적인 비용이 발생할 수 있습니다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbYZEMp%2Fbtsak3TMLdO%2Fs3Jdb9fcllKI0fqOpwm0uK%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fr7xny%2Fbtsakg6JTkG%2FFgoSqxDl90bGYTxP87FtjK%2Fimg.png)


### Managed Node Group

EKS는 워커 노드를 EC2 or Fargate로 생성할 수 있습니다.

EC2를 사용할 경우 Managed Node Group혹은 Self Managed Group을 선택해야합니다.

Managed Node Group으로 설정하게 되면 사용자가 생성할 때만 사용자가 옵션을 설정하고 그 이후는 AWS가 관리한다는 뜻이다.

