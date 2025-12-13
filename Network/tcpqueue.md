# TCP Syn, Accept Queue

TCP 3 way handshake 흐름에서 커널은 두 개의 큐를 명확히 분리해서 관리한다.

```
SYN  →  SYN Queue  →  SYN-ACK
ACK  →  Accept Queue  →  accept()
```

이 분리는 보안 + 성능 + 리소스 보호를 위한 구조다.

### SYN Queue (Half-open connection queue)

1. 상태 SYN_RECEIVED
2. 클라이언트가 syn을 보내고 syn-ack를 서버가 보냈지만 아직 최종 ack를 받지 못한 상태

즉, 연결이 완전히 성립되지 않은 반쯤 열린 연결을 담는 큐다.

tcp는 상태 기반의 프로토콜이기 때문에 syn을 받았다는 사실을 기억해야하기 때문에 해당 큐를 사용한다.

재전송 처리 작업에서도 쓸수있는데 ack가 안오면 syn-ack 재전송이 필요하기 때문이다.

syn flood (syn요청 막 갈겨서 서버 부하주는 공격법)의 방어 지점이 될 수 있다. 이 구간에서 악성 트래픽을 여기서 차단 가능하기 때문이다.

sync queue는 네트워크 레벨 방화벽 + tcp 상태 머신의 경계다.

#### 커널 관점에서 하는 일

- ISN(Inital Sequnece Number) 생성
- MSS, Window Scale, SACK Permitted 등 옵션 저장
- 타이머 등록 (재전송용)
- 큐가 가득 차면 SYN drop 또는 SYN Cookie

### Accept Queue (Fully-established connection queue)

- 상태는 ESTABLISHED
- 3 way handshake 오나료 상태
- 애플리케이션이 `accept()`를 호출하기 전까지 대기

애플리케이션은 즉시 accept를 하지 못하기 때문에 필요하다.
- 워커 스레드 부족
- gc, stw, cpu starvation등과 같은 이유

커널과 유저 공간의 속도 차이를 완충하기 위해서 존재하며 Back-pressure도 제공한다. 

accept를 못하면 새 연결을 제한하는 기능같은 백프레셔 역할

Accept Queue는 커널과 애플리케이션 사이의 버퍼라고 생각하면 된다.

### backlog

```
listen(fd, backlog)
```

여기서 backlog는 흔히 오해되지만 Accept Queue 크기의 상한값이다, Sync Queue는 별도 파라미터로 관리

리눅스에서는 실제 값이 다음으로 결정된다.

```
min(backlog, net.core.somaxconn)
```

### syc accept 이 둘을 분리하는 이유

| 이유  | 설명                                |
| --- | --------------------------------- |
| 보안  | SYN Flood를 Accept Queue까지 못 오게 차단 |
| 성능  | handshake 완료 전 연결을 가볍게 관리         |
| 안정성 | 앱 장애 시에도 커널이 완충                   |
| 공정성 | 네트워크 처리와 앱 처리 분리                  |

<br>

### 성능 튜닝 사례

#### syn 병목

syn flood + 정상 트래픽 혼재 환경이라 외부 트래픽이 급증하고 syn queue가 가득찬 상황 그리고 정상 클라이언트도 연결 실패상황이라고 가정해보자.

여기서 관측해볼 지표는 `SYN_RECV` 다량 증가, `ListenDrops`, `ListenOverflows` 증가등이 있다.

```
net.ipv4.tcp_max_syn_backlog = 8192
net.ipv4.tcp_syncookies = 1
```

backlog값을 올려주어 많은 syn들을 더욱 서용할수있게 해주며 공격 트래픽은 cookie로 흡수한다. 정상 ack시에만 accept queue에 진입하도록

여기서 syncookies는 서버와 클라이언트간에 암호화된 syn-ack만 받아주겠다는 옵션이라 생각하면 된다.

#### accept 병목

gc 또는 워커스레드 부족으로 accept호출이 지연되었다고 해보자. 클라이언트에서 connection timeout이 증가한다.

여기서는 `ESTABLISHED` 증가와 Accept Queue overflow를 확인해보고 cpu 여유있는지 확인해본다.

```
net.core.somaxconn = 4096
listen(backlog = 4096)
```

일단 병목은 application이므로 queue에서는 사이즈만 올려서 대기 connection들을 올려놓자 물론 애플리케이션 튜닝은 하긴해야지

accept 전용 스레드 분리나 gc튜닝으로 stw감소 -> 커널이 연결을 충분히 버퍼링, 앱 회복후 burst처리 가능, connection reset감소등의 해결이 된다.

즉 큐는 병목을 해결하는 것이 아니라 장애 전파를 막는다고 생각하면 편함


| 오해                     | 실제               |
| ---------------------- | ---------------- |
| backlog = SYN Queue 크기 | 아니다              |
| 큐 키우면 무조건 좋다           | 아니다 (메모리/공격면 증가) |
| accept 느리면 네트워크 문제     | 대부분 앱 문제         |
| SYN Flood는 L7 문제       | 아니다, L4 문제       |

- SYN Queue는 “연결을 시도하는 트래픽을 걸러내는 필터”
- Accept Queue는 “커널과 애플리케이션 사이의 완충지대”