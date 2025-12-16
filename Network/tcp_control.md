# TCP Congestion Control

TCP 혼잡제어 Congestion Control은 데이터 전송시 네트워크가 처리할 수 있는 양을 초과하지 않도록 조절하여 네트워크 붕괴(Congestion Collapse)를 방지하는 기술이다.

특정 알고리즘(Reno, CUBIC등)으로 넘어가기전에 기초가 되는 작동원리먼저 알아보자.

### Flow Control vs Congestion Control

두 개념은 **데이터 전송량을 조절한다**는 점은 같지만, 보호 대상과 판단 기준이 다르다.

| 비교 항목 | 흐름 제어 (Flow Control) | 혼잡 제어 (Congestion Control) |
| :--- | :--- | :--- |
| **보호 대상** | **수신자 (Receiver)** | **네트워크 (Network)** |
| **목적** | 수신자의 버퍼 오버플로우 방지 | 네트워크 라우터/링크의 혼잡 붕괴 방지 |
| **주체** | 송신자가 수신자의 상태를 고려 | 송신자가 네트워크 상황을 추측 |
| **핵심 변수** | $rwnd$ (Receive Window) | $cwnd$ (Congestion Window) |
| **작동 방식** | 수신자가 헤더에 `window size`를 알림 | 송신자가 패킷 유실/지연을 감지하여 조절 |

핵심 공식은 송시낮가 보낼 수 있는 실제 윈도우 크기 (Sending Window)는 둘 중 작은 값을 따른다. ActualWindow = `min(rwnd, cwnd)`

### 주요 변수

이 세 가지 변수가 혼잡 제어 상태 머신을 움직이는 핵심이다.

- cwnd (Congestion Window, 혼잡 윈도우)
  - 송신자가 **네트워크가 현재 이만큼은 감당할 수 있다**고 추정하는 데이터의 크기 (mss 단위 혹은 byte)
  - 네트워크 상황에 따라 동적으로 변한다
- rwnd (Receive Window, 수신 윈도우)
  - 수신자가 내 버퍼에 이만큼 공간이 남았다고 알려주는 값으로 tcp 헤더에 포함된다.
- ssthresh(Slow Start Threshold, 임계값)
  - slow start 단계에서 congestion avoidance 단계로 전환되는 기준점이다
  - 일반적으로 패킷 손실이 발생하면 현재 cwnd의 절반으로 줄어든다. 이 단계라는것은 아래에서 알아보겠다.

### 4가지 핵심 단계 Phase

tcp 혼잡 제어는 아래 4단계를 순환하며 최적의 전송 속도를 찾는다.

#### 1. Slow Start(느린 시작)

- **원리**: 연결 초기에는 네트워크 용량을 모르므로, 아주 작은 크기 (1MSS)로 시작하여 지수적으로 윈도우를 키운다.
- **동작**: ack를 받을때마다 cwnd를 1씩 증가시킨다 (RTT마다 2배 증가함 1 2 4 8 16 ...)
- **종료조건**: cwnd가 ssthresh에 도달하면 종료하고 Congestion Avoidance 구간으로 넘어간다.
  - 이름은 Slow지만 사실 가장 급격하게 속도를 올리는 구간이다.

#### 2. Congestion Avoidance (혼잡 회피)

- **원리**: 임계값 ssthresh을 넘으면 네트워크가 혼잡해질 위험이 크므로, 조심스럽게 선형적으로(Linear) 윈도우를 키운다.
- **동작**: RTT마다 cwnd를 1MSS 만큼만 증가시킨다. (ACK마다 1/cwnd씩 증가함)
- AIMD(Additive Increase / Multiplicative Decrease):
  - AI(합산증가): 혼잡이 없으면 선형적으로 1씩 증가
  - MD(곱셈감소): 혼잡이 감지되면(패킷 손실) 윈도우를 반으로 줄임

#### 3. Fast Retransmit (빠른 재전송)

- **문제**: 패킷이 손실되었을 때, 타임아웃(RTO)까지 기다리는 것은 너무 길다
- **해결**: 수신자는 순서가 어긋난 패킷을 받으면, 마지막으로 잘 받은 패킷의 ack를 계속 보낸다.
- **동작**: 송신자가 3개의 중복 ack를 받으면 타임아웃을 기다리지 않고 즉시 해당 패킷을 재전송한다. 이것은 가벼운 혼잡으로 간주함

#### 4. Fast Recovery (빠른 회복)

- **원리**: Fast Retransmit 이후, 네트워크가 완전히 멈춘 것은 아니므로 처음(Slow Start)으로 돌아가지 않는다.
- **동작**
  - ssthresh를 현재 cwnd의 절반으로 줄인다.
  - cwnd를 새로운 ssthresh 값(절반이 된 값) + 3 MSS (중복 ack 개수 보정)로 설정한다.
  - Congestion Avoidance 단계에서 바로 진입하여 선형 증가를 시작한다
  - 참고: 타임아웃이 발생한 경우에는 심각한 혼잡으로 간주하여 cwnd를 1로 초기화하고 다시 slow start부터 한다.

### ACK Clocking

송신자는 타이머에 의존해서 패킷을 쏘는 것이 아니라, 돌아오는 ACK를 신호(clock) 삼아 다음 패킷을 전송한다.

**보존의 법칙** Conservation of Packets에 대해서 알아보면 ack가 도착했다는 것은 **네트워크 파이프에서 패킷 하나가 빠져나가 수신자에게 도착했다**는 뜻이다.

즉, 파이프에 빈공간이 하나 생겼으므로, 송신자는 그 빈자리에 새 패킷을 하나 밀어넣을수있다.

이를 통해 송신자는 별도의 복잡한 속도 계산 없이도 네트워크의 처리 속도(bottleneck link 속도)에 자연스럽게 맞춰 전송률을 조절하게 되고 이것을 self-clocking이라고 한다.

### 요약

시작: cwnd = 1, slow start(지수 증가)

임계값 도달: cwnd >= ssthresh, Congestion Avoidance(선형 증가)

패킷 손실 감지:
- 3 Dup ACKs: Fast Retransmit/Recovery -> cwnd를 절반으로 줄이고 Congestion Avoidance
- Timeout -> 심각한 혼잡이라 cwnd = 1, slow start 재진입

### cwnd를 1로 시작하는 기준이 커넥션인가?

맞다. 독립적인 관리: tcp 혼잡 제어 변수들 cwnd, ssthresh 등 tcp connection마다 독립적으로 유지된다.

예를 들어서, 내 컴퓨너에서 구글과 네이버를 동시에 열었다면 구글의 연결 상태가 좋다고 해서 네이버 연결의 cwnd가 같이 커지지 않는다. 각자 1혹은 초기값부터 시작해서 논다.

과거 이론 tcp tahoe/reno 등에서는 1MSS(약 1.5kb)로 시작하는것이 정석이였다.

하지만, 현대 웹에서는 초기 속도를 높이기 위해 IW10(Inital Window = 10MSS)즉 약 15kb 정도로 시작하는 경우가 대부분이다.

### 3-way handshake때 교환하는 window size와 관련?

이게 좀 큰 오해포인트인데 핸드셰이크때 오가는 값은 cwnd가 아니라 rwnd다.

tcp 헤더에 있는 window 필드: 이것은 rwnd(receive window)다.

즉 수신자가 나 지금 버퍼 공간이 이만큼 남았으니까 이 이상은 보내지마를 의미한다.

이것은 혼잡 제어가 아니라 흐름제어(flow control)을 위햇 ㅓ존재하는 값이다.

cwnd(congestion window)는 패킷 어디에도 적히지 않고 송신자의 os 커널 메모리에만 존재하는 비밀 변수다.

송신자는 수신자가 알려준 rwnd와 자신이 계산한 cwnd를 비교해서 더 작은 값을 기준으로 데이터를 보낸다.

### 1씩 증가하는데 이는 byte를 의미하는가?

ㄴㄴ 아니다. 1씩 증가하는것에 대한 단위는 MSS로 Maximum Segment Size를 의미한다.

TCP에서 혼잡 윈도우를 논할 때 1씩 증가시킨다는 것은 패킷 1개 분량을 의미한다.

MSS(Maximum Segment Size)
- 헤더를 제외하고 순수하게 담을 수 있는 최대 데이터 크기다.
- 보통 이더넷 환경에선 1,460byte (MTU 1500 - ip header 20 - tcp header 20)
- cwnd가 1이라는 것은 1460 byte를 한번에 보낼 수 있다는 것임. 물론 이건 변경할수있긴함
- ack를 받아 cwnd가 1->2가 되었다는 2940byte(2배)를 보낼수있다는 뜻임
- 네트워크 장비(라우터)들은 바이트 단위가 아닌 패킷 단위로 처리하고 버퍼링하기 때문에 혼잡 제어도 패킷 개수단위로 계산하는것이 효율적

### Congestion Collapse

혼잡 붕괴라고도 하고 혼잡 제어가 이 붕괴를 막기 위해서 존재한다.

송신자가 네트워크 상태를 전혀 고려하지 않고 수신자의 버퍼크기 rwnd 만큼의 데이터를 쏟아붙게 된다면 발생하는 연쇄반응으로

혼잡 제어가 없으면 positive feedback loop라는 악순환이 발생해 네트워크가 멈춘다.

1. 무제한 전송: 모든 송신자는 네트워크 상황과 관계없이 최대한의 속도(수신자가 허용하는 한계치)로 패킷을 쏟아냄
2. 병목 현상(bottleneck): 중간 라우터의 처리 용량을 초과하는 트래픽이 몰려 **라우터 버퍼(Queue)**에 가득 차게됨
3. 패킷 폐기(Packet Drop): 라우터는 더이상 받을 수 없는 패킷들을 무차별적으로 버림
4. 재전송 폭주 (Retransmission Storm)
   1. 송신자는 패킷이 사라졌으니 ack를 못받는다.
   2. 송신자는 어? 안갔네 하고 retry함
   3. 네트워크는 이미 막혀잇는데 기존 데이터에 재전송 데이터까지 더해져 트래픽이 수배 늘어남
5. Goodput의 0 수렴
   1. 네트워크 대역 99%가 이미 버려진 패킷을 다시 보내는 재전송 패킷으로 채워짐
   2. 실제 목적지에 도착해서 유의미하게 쓰이는 데이터(goodput)은 0이됨

결과적으로 케이블은 데이터로 꽉차서 뜨거운 상태인데 사용자는 이를 감지 못해서 아무것도 로딩되지 않음. 뻗어버린다는것

옛날이긴하지만 1896년 이때문에 인터넷 백본망 NSFNEST 속도가 34kbps에서 40bps로 떨어졌다함. 1000분의 1정도.. 혼잡 제어 알고리즘이 없어서 그랬고 이를 탑제하기 위해서 congestion control의 Slow Start, Congestion Avoidance 알고리즘을 만들어냈고 tcp에 탑재후 인터넷이 되살아남