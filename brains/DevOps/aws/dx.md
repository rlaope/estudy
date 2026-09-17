# DX(Direct Connect)

AWS Direct Connect는 온프레미스(회사, idc)와 aws를 인터넷을 거치지 않고 전용선으로 직접 연결하는 서비스다.

s2s vpn이 인터넷 위 암호화된 터널이라면 dx는 아예 aws까지 가는 물리 전용망 라인

### 핵심 개념

**인터넷을 거치지 않는 전용 회선**
- isp나 통신사가 aws direct connection location까지 전용 광회선을 연결함
- 그 회선은 aws 네트워크로 직접 연결됨
- 그래서 지연(latancy)가 매우 안정적이고 packet loss도 거의 없는편

**VIF(Virtual Interface)로 AWS 서비스 접근** 

Direct Connect는 1개의 회선을 연결하고, 그 위에 VIF(가상 인터페이스)를 여러개 붙여서 사용한다.

- Private VIF -> VPC 내부 subnet과 연결
- Public VIF -> AWS 공용 서비스 (s3, dynamoDB, sqs등)에 직접 연결한다
- Transit VIF -> Transit Gateway 연결, 멀티 vpc 허브 구축

| 항목          | Site-to-Site VPN | Direct Connect    |
| ----------- | ---------------- | ----------------- |
| 경로          | 인터넷 기반           | 전용선 기반            |
| 안정성         | 변동 있음            | 매우 안정적            |
| 지연(latency) | 변동 있음            | 일정하고 짧음           |
| 대역폭         | 터널당 ~1.25Gbps    | 1Gbps ~ 100Gbps   |
| 보안          | IPsec 암호화 필요     | 전용회선 자체가 Private  |
| 비용          | 낮음(저렴)           | 매우 비쌈 (통신사/회선 비용) |
| 구축          | 빠름               | 느림(회선 개통 필요)      |

둘을 동시에 사용하는 경우도 있다. DX를 메인으로두고 s2s vpn을 backup 회선으로


### 구조

**Direct Connection Location**: AWS가 물리적으로 입주해있는 데이터 센터
- Seoul: KT 목동, IDC2, LG U+ 평촌, SK 브로드밴드 분당등
- 도쿄: Equinix, Colocation 시설 등

고객사는 해당 시설로 전용선을 끌어와서 aws와 연결함

L2/L3 구성으로는 보통 아래 방식 중 하나를 사용
- L2 연결: VLAN Trunk 기반
- L3 연결: BGP Peering으로 라우팅

AWS는 대부분 BGP 사용하도록 요구하긴함

목적으로는 대규모 트레픽 서비스나 온프레 db와 aws 애플리케이션 간 고속연결 혹은 대규모 데이터 마이그레이션 (TB ~ PB 단위), AWS와 기업 내부망을 하나의 큰 사설망처럼 묶기 등에 사용함

### 장애 대비

Direct Connect + S2S VPN(Failover) 동시 구성

- DX가 메인이고 S2S VPN을 백업으로 설정
- BGP 경로의 Preference를 조정해 DX를 우선사용
- DX가 귾기면 S2S VPN 경로사용

위처럼 구성한다.

SiteLink라는 여러 DX Location간 내부망을 만들때 쓰는기능도 있다.

![](https://docs.aws.amazon.com/images/whitepapers/latest/aws-vpc-connectivity-options/images/aws-direct-connect.png)

![](https://docs.aws.amazon.com/images/whitepapers/latest/aws-direct-connect-for-amazon-connect/images/physical-cross-connect.png)