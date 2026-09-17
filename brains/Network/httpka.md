# HTTP Keep-Alive 

HTTP Keep-Alive는 한 TCP 연결이 맺어지게 된다면 특정 시간동안 해당 연결을 유지해

HTTP 통신마다 3-way-handshake를 최소화해 레이턴시를 줄이고 연결을 계속 쓸수있게 해주기위한 기능이다.

오늘은 더 깊게 keep-alive에 여러 케이스들이나 질문들을 던져 알아보는 시간을 가져보겠다.

### HTTP Keep-Alive가 TCP Keep-Alive보다 먼저 끊기는 이유?

HTTP Keep-Alive는 애플리케이션 계층에서 idle-timeout 기반으로 적극적으로 커넥션을 관리한다.

**서버는 리소스를 효율적으로 사용하기 위해서 일정 시간동안 요청이 없으면 연결을 닿는다.**

반면 **tcp keep-alive는 커널 레벨에서 연결 생존 여부를 확인하기 위한 소극적인 장치**라서 기본값이 2시간처럼 매우길다.

- HTTP Keep-Alive: 리소스 관리 목적 -> 짧은 idle timeout
- tcp Keep-Alive: 연결 생존 확인 목적 -> 긴 idle timeout


### HTTP Keep-Alive 연결이 많아지면 어떤 문제가 생기는가

서버는 연결 1개당 다음을 유지해야한다.
- file descriptor
- socket buffer
- thread 또는 event loop context

Keep Alive가 비정상적으로 많은 경우에는
- FD 고갈
- 스레드 블로킹 증가(블로킹 애플리케이션이라면)
- Eviction 지연으로 인한 스로틀링
- 로드 밸런서 연결수 증가

등이 발생할 수 있다. 그래서 대부분 서버는 `keepalive_timeout`, `max_keepalive_requests`, `worker_connections` 등을 제한한다.

### HTTP/1.1 Keep-Alive vs HTTP/2 Persistent Connection

HTTP/1.1 Keep-Alive는 단순히 하나의 tcp 연결 위에서 요청을 순차적으로 재사용하는 방식이다.

대역폭 효율화는 되지만 Head of Line Blocking 문제가 존재한다.

HTTP/2는 
- 단일 TCP 연결
- 멀티플렉싱(Stream ID기반)
- 헤더 압축(HPACK)

을 통해 하나의 tcp 연결에서 동시에 여러번 요청을 병렬 처리 한다.  
따라서 같은 Persistent Connection 구조라도 HTTP/2가 훨씬 효율적이다.

### Keep-Alive가 NAT/방화벽에서 드랍되는 이유는 무엇인가?

HTTP Keep-Alive는 TCP 패킷이 전혀 흐르지 않는 Idle 상태가 길게 지속된다.

NAT와 방화벽은 Idle 상태의 connection table entry를 보통 30~120초 내에 삭제한다.

HTTP Keep-Alive는 TCP 레벨에서 패킷을 보내지 않기 때문에 NAT 테이블에 refresh가 발생하지 않아

연결이 드랍될 수 있다. 이를 방지 하기 위해서는
- TCP Keep-Alive ON
- kernel keepalive time조정
- 애플리케이션 ping

이 필요하다.

### Keep-Alive 상태에서 서버가 비정상 종료되면 클라이언트는 어떻게 감지하는가?

HTTP Keep-Alive 자체만으로는 감지가 불가능하다.

TCP 레벨에서만 감지가 가능하다.

즉 서버가 크래시가 나면 -> TCP FIN/RST가 없음

클라이언트는 소켓이 죽었음을 모르고 다음 HTTP를 보내는 순간이 되어서야
- RST 수신
- ETIMEDOUT
- Broken Pipe
등을 통해서 인지하게 된다 그래서 실시간 서비스에서는 Keep-Alive만을 믿지 말고 heartbeat, ping, retry 전략이 필요하다.

### HTTP Keep-Alive에서 Timeout은 어떤 기준으로 조정해야 하는가

조정 기준은 다음 세 가지다.
1. 서비스 특성(요청 간격)
2. 서버 자원(FD, worker 수)
3. 로드 밸런서 idle timeout(NLB/ALB, ELB)

일반적으로 LB Idle Timeout 보다 조금 작게 설정하는것이 좋다.

예를 들어 ALB Idle Timeout이 60초라면 서버 KeepAlive는 55초등이 된다.

### Keep-Alive가 HTTP Pipeline과는 어떤 관련이 있을까?

HTTP/1.1에서 Keep-Alive는 파이프라이닝의 전제 조건이다.

하지만 파이프라이닝은 Head-of-Line-Blocking 문제 때문에 거의 사용되지 않는다.

HTTP/2 에서는 파이프라이닝이 아니라 멀티 플렉싱으로 구조 자체가 바뀌었다.

### 서버 Keep-Alive를 아예 끄는 상황은 어떤 경우일까?

다음 상황에서 끄는게 좋다.
- 매우 짧은 요청 (iot 서버같은?)
- 대량의 짧은-lived 커넥션으로 FD 부담이 심함
- 프록시 체인이 꼬여 커넥션 재사용이 위험한 경우
- 보안상 연결을 오래 유지할 필요가 없는 환경

실무에서는 보통 끄지 않지만, 고부하 iot에서는 종종 끈다.

### Keep-Alive 서비스가 latency에 미치는 영향은 무엇인가

Keep-Alive는 다음을 제거한다.
- tcp 3 way handshake 비용
- slow start 초기 구간

특히 handshake만 해도 1 rtt가 들어가므로 지연이 큰 wan 환경에서는 효과가 커진다.

### HTTP Keep-Alive와 Connection Pool의 차이

- HTTP Keep-Alive: 실제 OS 소켓이 살아있도록 유지
- Connection Pool: 애플리케이션이 소켓을 재사용하도록 관리하는 구조

서버와 클라이언트는 둘 다 connection pool을 구성한다. Keep-=Alive는 pool의 기반 기술이다.