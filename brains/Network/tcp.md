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

<br>

### TCP Header

TCP 헤더는 tcp가 신뢰성을 구현하기 위해 반드시 필요한 정보들을 담고있는 구조인데

```
  0               15 16              31
 +-----------------+-------------------+
 |   Source Port   |   Dest Port       |
 +-----------------+-------------------+
 |           Sequence Number           |
 +-------------------------------------+
 |         Acknowledgment Number       |
 +--------+------+-------+------------+
 | Data   | Res  |Flags |  Window     |
 |Offset  |erved |       |   Size     |
 +--------+------+-------+------------+
 |       Checksum        | Urgent Ptr |
 +------------------------+-----------+
 |        Options (if any, variable)  |
 +-------------------------------------+
 |        Application Data ...        |
```

위와같은 구조고 tcp헤더(옵션 제외)는 20 byte다

![](https://www.pynetlabs.com/wp-content/uploads/2024/01/tcp-header-format.jpeg)


#### Source Port, Dest Portt

어떤 프로세스가 통신하는지 구분함 출발지 목적지

os 커널에서 소켓 매핑을 위해 사용됨 이값들은

#### Sequence Number

TCP 핵심으로 순서 보장을 위해 사용됨

송신자가 보낸 바이트 스트림의 시작위치를 나타냄

예시
- 처음 보낼때 ISN(Initial Sequence Number) = 1000
- 100바이트 보냈다면 그다음은 1100

Receive queue 에서는 이 번호를 통해 순서 재정렬(reassembly)를 진행함

#### Acknowledgement Number (32bit)

상대방으로부터 받은 바이트중 다음으로 받고싶은 바이트 번호를 의미함.

예시
- 내가 상대방으로 부터 1000~1999까지 수신했으면 ACK는 2000임

이 동작으로 tcp가 신뢰성있는 프로토콜이 되는기반임. 재전송도 ack 번호를 통해 판단할 수 있음

#### Data Offset (4bit)

tcp 헤더 길이를 나타내고 옵션이 있으면 헤더 길이가 증가하기 때문에 데이터 시작위치를 알려주는 필드

#### Flag (6 bit or 8bit depending on version)

tcp 동작을 제어하는 플래그

| Flag    | 설명                                     |
| ------- | -------------------------------------- |
| **SYN** | 연결 시작                                  |
| **ACK** | Acknowledgment 유효                      |
| **FIN** | 연결 종료 요청                               |
| **RST** | 연결 즉시 초기화                              |
| **PSH** | 버퍼링하지 말고 바로 위로 전달                      |
| **URG** | Urgent pointer 사용함                     |
| **ECE** | 혼잡 제어 Explicit Congestion Notification |
| **CWR** | ECN 응답, 혼잡 윈도우 감소                      |

핵심은 SYN ACK FIN RST다.

3way handshake랑 4way fin이 이 플래그로 이루어 지니까요

#### Window Size (16 bit)

Receiver가 보낼 수 있는 최대 수신량 (수신 버퍼 크기)를 의미함

즉, 

```
너 지금 최대 x바이트 만큼 더 보내도 돼
```

라는 의미이다. 이게 flow control(흐름제어)의 핵심이다. receive queue가 꽉 차면 window size = 0으로 보내서 송신을 막는다.

#### Checksum (16bit)

tcp 세그먼트가 손상되지 않았는지 검증함

응용 프로그램 레벨의 데이터 손상을 막기위한 엔드 투 엔드 오류 검증임.

NIC Offload 기능이 있을때에는 하드웨어가 계산하기도 함.

#### Urgent Pointer

URG 플래그와 함께 사용함. 거의 사용되지 않음

TCP 긴급 데이터 OOB 관련 기능이다.

#### Options (가변 길이)

TCP 확장 기능이 들어가는 공간이다.

중요 옵션들:

| 옵션           | 설명                                              |
| ------------ | ----------------------------------------------- |
| MSS          | Maximum Segment Size (상대방이 받아줄 수 있는 최대 세그먼트 크기) |
| Window Scale | Window 크기를 16bit 이상으로 확장                        |
| SACK         | Selective ACK (부분 재전송)                          |
| Timestamps   | RTT 측정, PAWS 보호                                 |
| Fast Open    | 핸드쉐이크 없이 데이터 전송 가능                              |


실제 패킷 예시

```css
Flags [S], seq 1234567890, win 64240, options [mss 1460,sackOK,TS val 12345 ecr 0, wscale 7]
```

- Flags[S] → SYN
- seq → 초기 시퀀스 번호
- win → 윈도우 크기
- options → MSS, SACK, timestamp, window scale 옵션

### TCP 헤더가 없으면 tcp가 불가능한 이유

TCP 헤더는 단순한 정보 구조가 아님. 이 핵심 기능들을 달성하기 위해 존재함

- 순서 보장 -> sequence number
- 신뢰성 -> acknowledgement number
- 흐름 제어 -> window size
- 혼잡 제어 -> ECN/ECE/CWR
- 연결 수립/해제 -> SYN/ACK/FIN
- 오류 검증 -> checksum
- 성능최적화 -> Options(MSS, Sack, Timestamp)

TCP 헤더가 없으면 절대로 만들어질 수 없다.