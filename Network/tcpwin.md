# TCP Window Size, MSS, SACK Permmited

### Window Size

TCP Window Size는 한 번에 ACK 없이 보낼 수 있는 데이터의 양이다.

정확히는 수신자가 현재 받아줄 수 있는 버퍼 크기로 흐름 제어를 위한 장치다.

```
Window = 64KB
-> 송신자는 ACK를 받기 전까지 최대 64KB 전송 가능
```

tcp는 다음을 동시에 만족해야 한다.
- 송신자가 너무 빨리 보내면 수신자 버퍼 overflow 
- 너무 천천히 보내면 대역폭 낭비

그래서 수신자가 매 ack마다 말한다.

window size만큼 지금 요청을 받아줄 수 있다. 

이게 window size고 핵심 포인트는 수신자 기준이며, 송신자는 이 값을 절대 초과해서 보내면 안된다. 

```
실제 전송 가능량 = min(rwnd, cwnd)
```
- rwnd : 수신자 Window (Window Size)
- cwnd : 혼잡 윈도우 (송신자, congestion control)

window 필드는 tcp헤더에서 16비트다. 

```
최대 65,535 bytes ≈ 64KB
```

그래서 이 값만으로는 RTT가 큰 네트워크에서 절대 대역폭을 못쓴다. 그래서 나온게 window size

window scale 옵션으로 확장간으하며 syn/syn-ack에서 협상한다. 실제로 윈도우는 advertised_window << sclae으로 동작함.

```
Advertised Window = 64KB
Scale = 7
→ 실제 Window = 64KB × 2⁷ = 8MB
```

리눅스에서 실제 조정하는 방식은 커널 자동 튜닝 방법으로 진행한다.

```
net.ipv4.tcp_window_scaling = 1
net.ipv4.tcp_rmem = 4096 87380 6291456
net.ipv4.tcp_wmem = 4096 65536 6291456
```

rmem wmem 스페이스바 순으로 min default max순이고 직접 고정보단 이 범위를 넉넉히 주는게 핵심이다.

애플리케이션 레벨 콛드로 socket.setReceiveBufferSize를 해도 되긴하는데 음.. 특정 테스크만 그렇게 돌기를 원한다면 쓸수있을듯하다.


### MSS (Max Segment Size)

mss는 tcp 세그먼트 하나에 실을 수 있는 최대 payload 크기다. 즉 한 패킷당 최대 데이터 크기

```
MTU = 1500
IP Header = 20
TCP Header = 20
→ MSS = 1460 bytes
```

보통 이렇고, ip fragmentation을 피하기위해, 네트워크 장비 부담 감소와 재전송 비용 감소를 위해 사용된다.

패킷 크기 단위 조절 장치다.

핵심 포인트는 mss는 송신자가 지키는 상한이고 연결 수립시 양쪽이 교환한다.

mss가 작으면 패킷 수가 늘고 오버헤드가 증가한다.

| 구분    | Window Size | MSS           |
| ----- | ----------- | ------------- |
| 의미    | 한 번에 보내는 총량 | 패킷 하나 크기      |
| 제어 대상 | 흐름 제어       | 패킷 단위         |
| 누가 정함 | 수신자         | 양쪽 협상         |
| 영향    | RTT 활용률     | Fragmentation |

### SACK Permmited

SACK(Selective ACK)는 어느 구간이 빠졌는지 정확히 알려주는 기능을 함.

기본적으로 tcp ack는 여기가지 다 받았다. 만 말한다. SACK는 10~20kb까지 받았고 20~30kb는 못받았고, 30~40kb는 받았다를 말할수있는거다.

#### SACK가 없으면

손실이 발생했을때 필요하며 하나 빠지면 뒤에 받은것들도 다 버린것처럼 처리해 대량 재전송한다.


#### SACK가 있으면

빠진 구간만 재전송이 가능해 대역폭 절약이 가능하다. RTT 낭비가 감소한다.

언제 협상 되는가는 SYN/SYN-ACK 단게에서 옵션으로 협상한다 양쪽 다 SACK Permmited가 있어야 활성화 됨.

```
MSS  → 패킷 하나 크기
Window Size → 동시에 날리는 총량
SACK → 손실 시 재전송 전략
```