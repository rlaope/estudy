# IPsec

IPSec은 ip 계층에서 보안(기밀성, 무결성, 인증) 등을 보장하는 기술이다.

1. 원본이 맞는지? (Peer authentication)
2. 패킷이 중간에서 변경되지는 않았는지? (Integrity)
3. 누가 엿보지 못하게 할 수 있는지? (Confidentiality)
4. 중간자 공격(MITM), 재전송 공격 방지 (Replay protection)

즉 기존 ip 패킷을 안전하게 만드는 기술이다.

### IPsec 모드

ipsec 모드는 두가지가 존재하는데.

#### Transport Mode

ip 헤더를 그대로 두고 payload(id payload)만 암호화한다 주로 End to End (eg, host <-> host)

#### Tunnel Mode

전체 IP 패킷을 암호화하고 새 ip헤더를 씌운다 주로 s2s vpn, dx와 같이 gateways간 터널링을 한다.

AWS S2S VPN도 내부적으로 IPSec Tunnel Mode 사용함.

### IPSec 두 구성요소

**AH (Authentication Header)**
- 무결성 + 인증 제공
- 기밀성(암호화 제공)
- NAT 환경에서 잘 동작하진 못해서 요즘 거의 안씀


**ESP (Encapsulating Security Payload)**
- 기밀성(암호화)
- 무결성
- 인증
- NAT Traversal 가능
- 현대 IPSec의 실질 표준

거의 모든 aws / router / 방화벽 장비는 ESP만 사용함


### SA(Security Association)

SA= 한 방향으로 적용되는 보안 정책/키/암호 규칙
- 단방향임(Inbound, Outbound 각각 필요함)
- SPI(Security Parameter Index)로 식별
- 암호화 알고리즘, 키, 재생방지 윈도우 등 포함

중요한 이유는 IPsec 패킷이 들어오면 spi로 어느 SA규칙을 복호화할지 결정해야함

### IKE(Internet Key Exchange)의 전체 흐름 (IKEv2 중심)

IPsec 터널은 IKE -> SA 협상 -> 키 교환 -> ESP 데이터 송수신순으로 이루어짐
1. **IKE_SA_INIT**: Diffie-Hellman(DH)키 교환이후 암호화 알고리즘 선택, NAT 탐지(NAT-D), 양쪽 nonce(랜덤값)를 교환후 IKE SA를 생성한다. (양쪽 암호화 채널 확보)
2. **IKE_AUTH**: Peer 인증 (PSK, 인증서, EAP등), ID 확인, Child SA 생성 (ESP용 SA) 최종 IPsec SA 2개(in/out) 확정
3. **IKE 이후: CHILD_SA 생성/갱신**: 데이터 채널(ESP)를 위한 SA를 추가 생성하거나 교체(rekeying)

> DH키는 양단말이 세션키(대칭키)를 만들기 위해 사용하는 키를 만들수있는 재료값 공유키값임. 이게 탈취되면 비밀키 털리는거아니야 할수도 있는데 사실 방정식처럼 막 간단하게 만드는게 아니고 어떤 다른 방식으로 이론상 구할순있지만 걸리는시간이 우주가 끝나는게 더 빠를만큼 수학적으로 거대해서 ㄱㅊ다고함. 알아보는게 좋을듯

### NAT Traversal(NAT-T)

NAT 환경에서 IPsec이 바로 통과하지 못하기 때문에 UDP 45000으로 ESP 캡슐화

이후 NAT-D payload로 NAT 환경 여부 확인후 Keepalive(1바이트 패킷)으로 NAT 세션을 유지한다. 

**ESP 패킷 실제 구조**

```
Outer IP Header
  ↓
ESP Header (SPI + Sequence Number)
  ↓
IV (Initialization Vector)
  ↓
Encrypted Payload (Original IP header + payload)
  ↓
Padding / Pad Length / Next Header
  ↓
Integrity Check Value (ICV)
```

- SPI -> 어떤 SA로 복호화해야하는지 식별
- Sequence Number: Replay Attack 방지
- Encrypted Payload: 원본 IP 패킷 전체