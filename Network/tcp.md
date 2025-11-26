# TCP State Machine

TCP는 신뢰성있는 스트림 통신을 제공하는 프로토콜이다.

이 말은 네 가지 핵심 기능을 포함하는데

첫 번째, 순서 보장 ordered delivery로 패킷 1, 2, 3이 순서대로 보내졌다면 리눅스 커널의 tcp 스택은 반드시 같은 순서로 애플리케이션에 전달한다.

두 번째, 손실 복구 retransmission 패킷이 사라지면 ack가 오지 않기 때문에 송신자는 타이머 기반으로 재전송한다.

세 번째, 흐름 제어 flow control로 receiver 윈도우 사이즈를 통해 나 지금 버퍼가 부족하니 천천히 보내 이런 조절을 한다.

네 번째, 혼잡 제어 congestion control 네트워크 자체가 혼잡하면, 패킷 손실이 발생하므로 tcp는 linux 커널의 혼합제어 알고리즘 cubic등을 통해 전송속도를 조절한다.

이 4개가 합쳐진것이 tcp의 본질이다.

> cubic은 혼잡회피 알고리즘중 하나로 3차 함수 기반의 윈도우 크기 증가, 특정 시간동안 위도우 크기 증가속도를 조절해 네트워크 상황에 맞춰 효율적으로 대역폭을 사용한다. 안정성 및 공정성 BIC TCP(Binary Increase Congestion control)에서 발전한 형태로 BIC TCP의 강점인 안정성과 공정성을 유지하면서 단순화된 구조를 가진다.

<br>

### 커널 내부에서 tcp 소켓이 만들어지고 관리되는 과정

애플리케이션이 socket()을 호출하면 리눅스 커널에 다음이 생성된다.

1. struct socket
2. struct sock(inet_sock)
3. receive buffer / send buffer
4. TCP 상태 머신 (TCP state machine)

TCP는 상태 기반 프로토콜이기 때문에 각 연결은 ESTABLISHED, SYN_SENT, FIN_WAIT1, TIME_WAIT 같은 상태로 관리된다.

### 리눅스에서 패킷이 실제로 이동하는 경로

애플리케이션 -> 커널 -> NIC -> 스위치/라우터 -> 반대편 -> 리눅스 커널 -> 애플리케이션이 전체 한 사이클이다.

송신방향 (TX Path)

```perl
app write()
  ↓
socket send buffer
  ↓
TCP segmentation (MSS 단위로 자름)
  ↓
IP layer에서 routing 결정
  ↓
netfilter(OUTPUT) / iptables 로 통과
  ↓
qdisc (fq, fq_codel 등 커널 스케줄러)
  ↓
device driver → NIC
  ↓
LAN / 인터넷 전송
```

수신 방향 (RX Path)

```perl
NIC가 패킷 수신
  ↓
NAPI 인터럽트 → softirq 처리
  ↓
device driver가 커널로 패킷 전달
  ↓
IP layer에서 demux (목적지 포트 기반)
  ↓
netfilter (INPUT)
  ↓
TCP 재조립(순서 보정, 누락 패킷 확인)
  ↓
socket receive buffer
  ↓
app recv()
```

<br>

### TCP State Machine

TCP의 모든 동작은 상태 기반을 ㅗ정의된다.

연결 수립 3 way handsahke

```arduino
Client ---- SYN ----> Server
Client <--- SYN/ACK - Server
Client ---- ACK ----> Server
```

연결종료 4-way FIN

```css
A ---- FIN ----> B
A <--- ACK ---- B
B ---- FIN ----> A
B <--- ACK ---- A
```

이런 패턴을 기반으로 리눅스는 각 tcp 소켓을 상태별로 관리한다.

### 3 way handshake

1단계 syn으로 클라이언트가 다음 정보를 담아서 보낸다.
1. 내 시퀀스 번호 ISN
2. 연결하고 싶습니다 라는 플래그
3. 커널 소켓은 다음 상태로 둔다

```
SYN_SENT
```

2단계로 SYN-ACK

서버는 자신의 ISN(시퀀스 번호)를 보내면서 ACK로 응답한다.

```
Server State: SYN_RECV
```

이 상태에서 리눅스에서 half-open connection으로 관리되는데 백로그 큐(backlog queue)에 들어간다.

여기서 backlog queue는 연결 대기열로 tcp 서버가 클라이언트 연결 요청을 받으면 바로 애플리케이션에게 넘기지 못한다.

그래서 리눅스 커널 내부에 두 개의 대기열을 갖고있는데 SYN queue랑 accept queue로 각각 하프오픈큐와 풀오픈 큐로 생각하면 된다.

클라이언트가 연결요청하면 먼저 syn queue에 넣었다가 handshake가 끝나면 accept queue로 이동하고 애플리케이션이 accept()를 호출해 가져간다.

즉 tcp 3way handshake 과정에서 생성되는 임시 연결 상태들을 저장해두는곳이다.

3단계 ACK

클라이언트가 ACK를 보내면 서버는 연결을 정식적으로 ESTABLISHED 상태로 전환한다.

그리고 다음이 생성된다.

1. send buffer
2. receive buffer
3. congestion control state
4. RTT/RTO 계산용 변수들
5. window control 구조체

### 리눅스 커널에서 TCP 패킷이 실제로 이동하는 과정

syn 수신시 리눅스 동작

```
NIC
 ↓
driver
 ↓
netfilter PREROUTING
 ↓
IP layer: TCP packet demux
 ↓
TCP state machine: listen socket 검사
 ↓
SYN queue(syn backlog)에 저장
 ↓
SYN/ACK 생성하여 송신
```

최종 ack 수신시

```
NIC
 ↓
driver
 ↓
PREROUTING
 ↓
IP Layer
 ↓
TCP
 ↓
half-open → full ESTABLISHED 소켓으로 승격
 ↓
accept() queue로 이동
 ↓
app이 accept() 호출하면 해당 소켓 반환
```

![](https://miro.medium.com/1%2ArgXykunw7cXVE2OnnFH4YA.jpeg)