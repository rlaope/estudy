# DHCP(Dynamic Host Configuration Protocol)

IP를 할당하는 과정은 직접 입력하는 정적 할당, 외부 시스템으로부터 자동으로 입력 받는 방법은 동적 할당이라고 한다.

보통 사내 네트워크나 규모가 작은 네트워크에서는 IP를 정적으로 할당 받지만, 개인용 컴퓨터와 같은 광범위한 범위의 네트워크에서는 IP를 동적할당 방식으로 할당받는다. (물론 보안 설정과 같은 것을 위해 정적으로 해주는 곳도 있지만, 요즘은 동적할당의 보안을 강화하여 개선해나가고 있다.)

그리고 이러한 동적 할당을 도와주는 프로토콜이 바로 **DHCP(Dynamic Host Configuration Protocol)**이다.

DHCP는 BOOTP(Bootstrap Protocol)에서 개선된 버전이며 서로 호환된다. 

DHCP는 DHCP Client(67포트) DHCP Server(68포트) 이렇게 두개가 동작한다.

<br>

### DHCP 동작 방식

DHCP를 통해 IP를 할당받는 과정을 설명해보겠다.

1. `DHCP Discover`: Client가 DHCP Server를 찾기위하여 브로드캐스트 요청을 보낸다.
2. `DHCP Offer`: 요청을 받은 서버가 IP Pool에서 해당 네트워크 구성 정보(IP, Subnet Mask, Gateway, Lease Time, DHCP ID)를 보낸다.
3. `DHCP Request`: 제안을 받은  Client는 사용할 Host를 결정하고 응답을 한다.
4. `DHCP Ack`: 서버는 클라이언트로 부터 응답을 받으면 해당 호스트 정보들을 기록한다. 

이렇게 4개의 단계로 DHCP는 동작한다.

DHCP는 IP Pool로 여러 IP들을 관리하는데 일부러 의미없는 시스템에 IP를 할당받아 IP Pool이 비어있어 응답을 하지 못하게 만드는 상태 **DHCP Starvation DHCP** 기아현상을 만들게 하는 공격 **DHCP Starvation Attack**을 주의해야한다.

그리고 DHCP로 부터 할당 받은 IP는 보통 임대(Lease)했다고 하는데 LeaseTime동안 임대가 진행이 되지만 또 다시 요청을 하고 IP를 할당 받으려면 브로드캐스트를 진행해야해서 부하가 많이 든다.

그렇기에 갱신 작업을 통하여 IP Time을 갱신한다. 갱신시간이 50%가 지났다면 DHCP Request를 다시 서버에게 보내게되고 갱신 시간을 다시 재작성 하여 DHCP Ack를 보낸다.

<br>

### DHCP 릴레이

DHCP는 브로드캐스트로 동작하기 때문에 일반적으로 동일한 네트워크에서만 동작하게 된다.

그렇기에 DHCP를 여러 네트워크에서 사용하려면 대역폭 별로 나누어 여러개의 DHCP를 두어 사용하게 된다.

여기서 DHCP Relay Agent는 다른 네트워크 대역에서도 DHCP를 사용할 수 있도록 인도해주는 에이전트 역할을 해준다.

![](http://www.iorchard.net/_images/dhcp_relay_1.png)

DHCP 릴레이는 일반적인 DHCP와 다르게 Client -> Server 요청은 유니캐스트로 진행이 되며, Server -> Client 요청이 브로드캐스트다.