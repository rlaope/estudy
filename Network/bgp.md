# BGP(Border Gateway Protocol) Peering

두 네트워크 라우터가 서로 나는 이런 경로(CIDR)를 가지고 있다라고 교환하며 라우팅 정보를 자동으로 동기화하는 구조

S2S VPN, Direct Connect, 대형 ISP, 기업 네트워크에서 핵심적으로 사용되고 있음.

즉 두 라우터가 BGP 프로토콜로 서로 연결(지점 간 세션)하여 라우팅 정보를 자동으로 주고받는 관계

이 연결된 상태를 Peering 이라고 함

### 왜 필요한가

라우트를 자동으로 교환: static route처럼 일일이 사람이 넣지 않아도 됨.
여러 경로가 있을 때 최적 경로 선택: AS-PATH, MED, LOCAL_PREF 등 기준에 따라 자동 선택
장애시 자동 failover: Tunnel1 -> Tunnel2로 자동 전환이 가능함. (aws s2s vpn이 터널 2개를 둘 다 bgp로 하는이유)
대규모 네트워크에서 필수임: cidr을 자동 업데이트하기 때문에 인력소모가 없음

온프레 라우터
   (ASN 65001)
      │
   BGP Peering
      │
AWS VPN Gateway
   (ASN 64512)
위처럼 서로 BGP 세션을 맺고 우리쪽 CIDR을 AWS에 알리고 AWS VPC CIDR를 우리쪽에 자동으로 알려준다.

### BGP Peering이 만들어내는 요소

BGP 세션 (Neighbor)
neighbor 169.254.10.1 remote-as 64512
neighbor 169.254.10.1 ebgp-multihop 2
neighbor 169.254.10.1 activate

2. Prefix Advertise: 내가 가진 네트워크 대역을 상대에게 광고함
network 10.0.0.0/16

3. Prefix Receive: 상대가 광고한 경로를 라우팅 테이블에 반영

### AWS S2S VPN에서의 BGP Peering

각 vpn 터널마다 VTI/30 내부 IP가 제공됨

우리 라우터: 169.254.10.2/30
AWS 라우터: 169.254.10.1/30

이 둘이 BGP Peering을 맺음. aws는 기본적으로 ASN 64512. 온프레는 보통 65000~65535 대역사용함.

결과적으로
aws vpc cidr을 자동으로 온프레미스에 광고
온프레 cidr을 aws에 광고
터널1 장애시 장애2 자동 페일오버
라우팅 테이블 자동 갱신

 VTI는 Virtual Ternel Interface로 IPsec 터널을 하나의 가상 네트워크 인터페이스처럼 만드는 기술
 
eth0   → 물리 NIC
eth1   → 물리 NIC
vti0   → 가상 NIC (IPsec 터널)
즉 IPsec 터널에 ip 주소를 붙이고 라우팅 테이블을 넣을 수 있게 만든 인터페이스다.
VTI가 있으면 라우터에서 `route add 10.10.0.0/16 via vti0` 이렇게 다룰 수 있음. 그래서 라우팅 제어가 쉬움
aws, fortigate, palo alto다 vti권장.

| 항목        | Static Route | BGP Peering                |
| --------- | ------------ | -------------------------- |
| 라우트 추가 방식 | 사람이 수동 입력    | 자동 광고/수신                   |
| 장애 대응     | 사람이 변경       | 자동 Failover                |
| 확장성       | 작을 때만 OK     | 수백~수천 경로도 자동 관리            |
| AWS 권장    | X            | O (Site-to-Site VPN 기본 옵션) |

정리하면 BGP Peering은 두 라우터가 라우팅 정보를 자동으로 주고받는 연결이며, Failover·확장성·자동 라우팅 관리를 위해 Site-to-Site VPN에서 핵심적으로 사용되는 기술이다.