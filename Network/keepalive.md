# HTTP Keep-Alive, TCP Keep-Alive

### HTTP Keep-Alive

HTTP 계층마다 하나의 tcp 연결을 여러 http 요청에 재사용하기 위한 기능이다.

즉, 매 요청마다 TCP 3-way handshake를 다시 하지 않도록 해주는 성능 최적화다.

### 동작 방식

- client http 요청 헤더에 Connection: keep-alive를 포함시킨다,
- 서버가 연결을 당분간 유지한다.
- 같은 tcp 연결 위에서 여러 http request/response를 주고 받는다.
  서버는 Idle Timeout(ex. 60s) 지나면 연결을 끊는다.

목적은 RTT 감소와 레이턴시 개선, 서버와 클라이언트간 연결 재활용을 통한 성능 향상.

### 특징

- 애플리케이션 계층(HTTP) 기능
- 연결이 얼마나 더 남아 있을지는 서버 설정에 의존한다.
- 연결이 끊어졌는지 감지하는 기능은 없다
- 요청이 없으면 Idle로만 남는다.

<br>

### TCP Keep-Alive

TCP 소켓이 죽었는지(상대가 사라졌는지) 확인하기 위한 네트워크 레벨 기능이다.

즉, 시스템이 저 연결이 아직 살아있나를 체크하는 용도다.

### 동작 방식

- 커널이 일정 시간이 지나면 keepalive probe 패킷을 전송한다.
- 상대에게 응답이 없으면 재전송한다.
- 지정횟수 실패하면 커널이 소켓을 강제 종료한다(ETIMEDOUT)


### 목적
- 오래된 데드 커넥션을 제거한다
- NAT/방화벽이 idle connection을 드랍하지 않도록 유지한다.
- 네트워크 단절 시 빠르게 감지한다.

### 특징
- 전송 tcp 기능
- 매우 긴 주기 기본값
  - `tcp_keepalive_time` (기본 7200초, 2시간)
  - `tcp_keepalive_intvl`
  - `tcp_keepalive_probes`

| 항목                        | HTTP Keep-Alive          | TCP Keep-Alive       |
| ------------------------- | ------------------------ | -------------------- |
| 계층                        | Application(HTTP)        | Transport(TCP)       |
| 목적                        | 연결 재사용(성능 향상)            | 연결 생존 여부 체크          |
| 타이밍                       | 애플리케이션이 설정한 Idle Timeout | OS 커널 설정 기반(수분~수시간)  |
| 사용하는 패킷                   | HTTP 요청/응답               | TCP keepalive probe  |
| 끊길 때                      | 서버가 Idle Timeout 시도중     | probe 실패 시 커널이 소켓 종료 |
| NAT/Firewall idle drop 방지 | 불가능                      | 가능                   |


- HTTP Keep-Alive = 재사용
  - 연결을 계속 쓰기 위해 유지
  - 성능을 위한 기능
  - 연결이 죽었는지 감지하는 역할 없음
- TCP Keep-Alive = 생존확인
  - 상대가 살아있는지 확인
  - 네트워크 단절, 서버 다운을 감지
  - 커널이 관리