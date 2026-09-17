# VPC

## VPC란

Amazon Virtual Private Cloud(VPC)를 사용하면 클라우드에서 논리적으로 격리된 공간을 프로비저닝 하여 고객이 정의하는 가상 네트워크에서 AWS 리소스를 시작할 수 있다.
  
IP 주소 범위 선택, 서브넷 생성, 라우팅 테이블 및 넽워크 게이트 구성 등 가상 네트워킹 환경을 완벽하게 제어할 수 있다.
  
VPC에서 IPv4, IPv6를 모두 사용하여 리소스와 애플리케이션에 안전하고 쉽게 엑세스할 수 있다.

## VPC 종류

### Default VPC
- 계정 생성 시 자동으로 셋업 (모든 리전에)
- 모든 서브넷의 인터넷 접그닝 가능
- EC2가 public ip, private ip 모두 가지고 있음
- 삭제시 복구 불가


### Custom VPC
- 새로 만들어야 한다.
- Default VPC의 특징을 가지고 있지 않다.

### VPC를 사용하여 할 수 있는 일들
- EC2를 실행 가능
- 서브넷을 구성 가능
- 보안 설정 가능 (ip block, 인터넷에 노출되지 않은 EC2 구성 등)

### VPC Peering VPC간에 연결
- Transitive Peering 불가능: 한 다리 건너 연결 되어 있다고 해서 Peering이 된 것은 아님

### VPC Flow log
- VPC의 로그를 CloudWatch에서 저장 가능

### IP 대역 지정 가능
- 리전에 하나: 다른 리전으로 확장 불가능

<br>

## VPC의 구성 요소
![](https://user-images.githubusercontent.com/28394879/141058705-4ac55134-69e5-441a-b1ba-3b4f71c90e28.png)

1. Availablity Zone
2. Subnet
3. Internet Gate Way
4. Network Access Control List/Security Group
5. Route Table
6. Network Address Translation Instance/NAT Gateway
7. Bastion Host
8. VPC Endpoint

### Availablity Zone
물리적으로 분리되어 있는 인프라가 모여 있는 데이터 센터  
  
고가용성을 위해서 항상 일정 거리 이상 떨어져 있다.
  
하나의 리전은 2개 이상의 AZ로 이루어져 있다.
- 계정 1의 AZ-A는 계정 2의 AZ-A와 다른곳에 있다.

![](https://user-images.githubusercontent.com/28394879/141059214-0bf68399-1fb8-4a6e-9b75-83d29d2cb893.png)

### Subnet
VPC의 하위 단위
  
하나의 AZ에만 생성이 가능 -> 다른 AZ로 확장이 불가능
- 하나의 AZ에는 여러 Subnet 생성 가능
  
**Private Subnet** : 인터넷에 접근 이 불가능한 서브넷
  
**Public Subnet**: 인터넷에 접근 가능한 서브넷
  
CIDR block range 설정 가능

> CIDR : Class Inter-Domain Routing 클래스 없는 도메인 간 라우팅 기법

### Internet Gateway(IGW)
인터넷으로 나가는 경로
  
고가용성이 확보되어 있음
  
IGW로 연결되어 있지 않은 서브넷 = Private Subnet
  
Route Table에서 연결해줘야 함

### NACL/Security Group
검문소
  
NACL -> Stateless, SG -> Stateful 
  
기본적으로 VPC 생성시에 만들어줌
  
Deny는 NACL에서만 가능

### Route Table
![](https://user-images.githubusercontent.com/28394879/141064479-4e31b75a-e564-40a1-8574-306f150a2def.png)
트래픽이 어디로 가야 하는지 알려주는 이정표
  
기본적으로 VPC 생성시 만들어준다.

### NAT Instance/NAT Gateway

![](https://user-images.githubusercontent.com/28394879/141087041-97f1c809-eb26-4950-88ea-5b900c7637e6.png)

Private Instace가 외부의 인터넷과 통신하기 위한 통로
  
NAT Instance는 단일 Instance / NAT Gateway는 AWS에서 제공하는 서비스
  
NAT Instance를 사용할 때 SOurce/Destination Check를 해제해야 한다.
  
NAT Instance는 Publi Subnet에 있어야 함

### Bastion Host
![](https://user-images.githubusercontent.com/28394879/141087702-b6cdb535-04ea-4c4a-8624-3cf642539183.png)

Private Instance에 접근하기 위한 인스턴스
  
Public Subnet에 위치해야함

### VPC Endpoint
VPC 엔드포인트를 통해 인터넷 게이트웨이, NAT 디바이스, VPN 연결 또는 AWS Direct Connect 연결을 필요로 하지 않고 AWS PrivateLink 구동 지원 AWS 서비스 및 VPC 엔드포인트 서비스에 비공개로 연결 가능
  
VPC의 인스턴스는 서비스의 리소스와 통신하는 퍼블릭 IP주소를 필요로 하지 않는다.
  
VPC와 기타 서비스간의 트래픽은 아마존 네트워크를 벗어나지 않는다.

### VPC Endpoint 종류

![](https://user-images.githubusercontent.com/28394879/141247694-0059b7ae-aa55-4f35-96e1-03e15351161e.png)

- Interface EndPoint: ENI(Elastic Network Interface) 기반
  - Private ip를 만들어 서비스로 연결
  - 많은 서비스들을 지원 SQS, SNS, Kinesis, Sagemaker 등

![](https://user-images.githubusercontent.com/28394879/141247612-e2e9ef57-147f-4889-81c2-c4b2d6ab6293.png)

- Gateway Endpoint: 라우팅 테이블에서 경로의 대상으로 지정하여 사용
  - S3, DynamoDB 지원