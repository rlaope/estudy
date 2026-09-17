# TCP/IP Packet, TCP vs UDP

## TCP/IP Packet
TCP 세그먼트에는 IP 패킷의 출발지 IP와 목적지 IP 정보를 보완할 수 있는 출발지 port, 목적지 port, 전송 제어, 순서, 검증 정보등을 포함한다.

## TCP 
전송제어 프로토콜(Transmission Control Protocol)  
TCP는 같은 계층에 속한 UDP에 비해 상대적으로 신뢰할 수 있는 프로토콜

- 연결 지향 - TCP 3 way handshake(가상 연결)
- 데이터 전달 보증
- 순서 보장
- 신뢰할 수 있는 프로토콜

### 연결 지향 - TCP 3 way handshake(가상 연결)
TCP는 장치들 사이에 논리적인 접속을 성립하기 위하여 3 way handshake를 사용하는 연결지향형 프로토콜
![](https://velog.velcdn.com/images/mmmdo21/post/969064b5-5772-4b57-9495-112b61816766/image.png)

1. 클라이언트는 서버에 접속을 요청하는 SYN 패킷을 보냄
2. 서버는 SYN요청을 받고 클라이언트에게 요청을 수락한다는 ACK와 SYN가 설정된 패킷을 발송하고 클라이언트가 다시 ACK으로 응답하기를 기다림
3. 클라이언트가 서버에게 ACK를 보냄
   1. 이 이후로부터 연결이 성립되며 데이터를 전송할 수 있다.
   2. 만약 서버가 꺼져있다면 클라이언트가 SYN을 보내고 서버에서 응답이 없기 때문에 데이터를 보내지 않는다.
   3. 현재는 최적화가 이루어져 3번 ACK을 보낼때 데이터를 함께 보내기도 한다.

> SYN: Synchronize  
> ACK: Acknowledgement

### 데이터 전달 보증

![](https://velog.velcdn.com/images/mmmdo21/post/942e43a0-1161-4e3d-8c00-b8c7eeccfe93/image.png)

TCP는 데이터 전송이 성공적으로 이루어진다면 이에 대한 응답을 돌려주기 때문에 IP 패킷의 한계인 비연결성을 보완할 수 있다.

### 순서 보장
![](https://velog.velcdn.com/images/mmmdo21/post/cfaa75b1-b723-4e2d-b8ab-b1bac832f6a5/image.png)

만약 패킷이 순서대로 도착하지 않는다면 TCP 세그먼트에 있는 정보를 토대로 다시 패킷 전송을 요청할 수 있음
  
이를 통해 IP 패킷의 한계인 비신뢰성(순서를 보장하지 않음)을 보완할 수 있다.

<br>

## UDP

사용자 데이터그램 프로토콜 (User Datagram Protocol)
  
UDP는 IP 프로토콜에 port, 체크섬 필드 정보만 추가된 단순한 프로토콜

- 하얀 도화지에 비유됨(기능이 거의 없음)
  - HTTP3는 UDP를 사용하며 이미 여러 기능이 구현된 TCP보다는 하얀 도화지처럼 커스터마이징이 가능하다는 장점이 있음
- 비연결지향 - TCP 3 way handshake x
- 데이터 전달 보증 x
- 순서 보장 x
- 데이터 전달 및 순서가 보장되지 않지만, 단순하고 빠르다.
- 신뢰성 보다는 연속성이 중요한 서비스에 자주 사용된다. (ex 실시간 스트리밍)

TCP, UDP의 차이를 비유한다면, 좋은 기능이 다 들어있는 무거운 라이브러리와 필요한 기능만 들어있는 가벼운 라이브러로 비교할 수 있음

> 체크섬 (checksum)은 중복 검사의 한 형태로, 오류 정정을 통해, 공간(전자 통신)이나 시간(기억 장치)속에서 송신된 자료의 무결성을 보호하는 단순한 방법이다.

## TCP vs UDP
![](https://velog.velcdn.com/images/mmmdo21/post/25c362f2-57d3-49ca-8155-83fa1ae070bf/image.png)