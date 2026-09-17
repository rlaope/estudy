# Site to Site VPN

S2S VPN은 온프레미스 네트워크와 AWS VPC를 IPsec 기반으로 안정적으로 직접 연결하는

전용 터널 방식의 vpn 서비스다. 고객사 idc, 사내 센터, 지점망등과 aws 클라우드를 안전하게 하나의 네트워크처럼 연결가능하다.

### 구성요소

**Customer Gateway(CGW)**: 온프레미스 쪽 VPN 엔드포인트로 장비 종류는 다음중 하나면 된다
- 물리 방화벽/라우터 (Cisco, Juniper, Fortigate, Palo Alto등)
- 가상 라우터
- 소프트웨어 기반 VPN 장비

이 장비가 AWS와 IPSec IKEv1/v2로 보안 터널을 맺는다.

**Virtual Private Gateway(VGW) 또는 Transit Gateway(TGW)**

AWS쪽 VPN 종단이다
- 단일 VPC 대상으로는 VGW 를 주로 사용한다
- 여러 VPC, 멀티리전 연결, 허브-스포크 구조면 TGW를 사용한다.

**두개의 IPSec 터널**

AWS는 동일한 VPN 연결에 이중화 목적으로 두 개의 터널을 자동으로 제공한다.
- Tunnel 1, Tunnel 2
- 온프레미스 장비는 둘 중 하나로 트래픽을 보내고, 하나가 죽을때 자동 failover 된다.


### 동작 구조

1. IPsec 기반 암호화
   1. 1단계(IKE Phase 1): 서로 인증하고 SA(Security Association) 생성
   2. 2단계(IKE Phase 2): 실제 데이터 암호화 터널 생성
   3. 데이터는 ESP 프로토콜로 암호화되어 송수신 된다.
2. BGP를 통한 라우팅(선택)
   1. 정적 라우팅도 가능
   2. 하지만 대부분 BGP Peering을 사용해 경로 자동 교환
   3. BGP 사용 시 헬스체크 + failover 성능 우수

### 사용목적

사용목적으로는 온프레미스 데이터 센터와 aws간 안전한 통신이 목표며

dba, 내부 서버, 사내 시스템에서 aws db나 ai 호출, ERP, 업무 시스템과 통합

Direct Connect 장애 시 백업 링크로 활용

글로벌 기업 aws간 내부 라우팅 허브 구축 등이 있다.

### 한계

대역폭이 제한된다. 터널당 대력 1.25Gps 수준인데 실제로는 500~700Mbps 정도로 보는것이 보통이고 고성능이 필요하면 Direct Connect(DX)를 병행한다.

인터넷 기반이라 물리적인 인터넷 경로를 타기 때문에 레이턴시 변동이 존재하며 패킷 loss 가능성도 있다.

MTU 이슈도 있는데 IPsec encapsulation 때문에 유효 MTU가 줄어들어 조정ㅇ이 필요할 수 있다. 보통 1436정도?

### 구성 예시 흐름

1. 온프레미스 방화벽에 public ip가 있음
2. AWS에서 s2s vpn 생성
3. 자동으로 
   1. 터널 1, 2 endpoint ip
   2. IKE 설정
   3. Pre-Shared Key
   4. 라우팅정보 등 생성
4. 온프레 장비에 해당 설정을 적용
5. 터널 up
6. BGP Peering 맺고 라우팅 테이블 자동 동기화
7. 양쪽 네트워크 간 통신 가능

![](https://docs.aws.amazon.com/images/whitepapers/latest/aws-vpc-connectivity-options/images/redundant-aws-site-to-site-vpn-connections.png)

![](https://docs.aws.amazon.com/images/vpn/latest/s2svpn/images/cgw-high-level.png)