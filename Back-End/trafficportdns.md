# SNAT 포트 고갈, DNS 해상도 지연 문제 해결

**SNAT(Source Network Address Translation)**은 프라이빗 서브넷에 위치한 서버가 외부 인터넷 또는 다른 vpc와 통신하기 위해 출발지 source의 사설 ip를 NAT 게이트웨이등의 공인 ip로 변환하는 기술이다.

**Ephemeral Port** 임시 포트라고 하는데, 서버가 외부로 아웃바운드 연결(client 역할)을 맺을 때 운영체제가 임의로 할당하는 출발지 포트다. 리눅스 환경에서는 보통 32768 ~ 60999 범위를 사용하므로 약 28,000개의 포트가 존재한다.

**DNS Resolution (DNS 해상도)**는 api.github.com과 같은 도메인 네임을 실제 통신 가능한 ip 주소로 변환하는 과정이다. 애플리케이션은 통신을 시작하기 전에 반드시 이 과정을 거쳐야한다.

### 문제 정의: 자원은 넉넉한데 타임아웃이 발생함

초당 수천 건의 외부 api 호출이나 db 쿼리가 발생할 때, 서버 자원과 무관하게 통신이 실패하는 원인은 **주로 os의 네트워크 스택 한계**에 있다.

1. **SNAT 포트 고갈 Port Exhaustion**: HTTP 통신 후 연결을 끊으면 tcp 프로토콜의 안정성을 위해 해당 소켓은 즉시 삭제되지 않고 약 60초간 `TIME_WAIT` 상태로 대기한다. 외부 api 호출을 매번 새로운 커넥션으로 맺고 끊으면(connection 단절), 순식간에 28,000개의 심시 포트가 모두 TIME_WAIT 상태로 빠지게 되고 os는 더이상 할당할 포트가 없어 새로운 외부 통신을 시작조차 못하고 애플리케이션 단에서 타임아웃 예외가 발생한다
2. **DNS 해상도 지연 및 드롭**: 커넥션을 재사용하지 않으면 통신을 시도할 때마다 DNS 쿼리를 날려야하고 리눅스의 기본 DNS 질의는 UDP 기반이며, 트래픽 스파이크시 로컬 DNS 리졸버나 클라우드 제공자(예: aws route53)등 DNS 서버에서 쿼리 제한에 걸리거나 패킷이 유실될 수 있다. 리눅스는 DNS 응답을 받지 못하면 기본 5초를 대기한 후 재시도하므로 치명적인 latency가 발생한다.

### Example

**애플리케이션 레벨의 커넥션 풀 및 keep-alive 설정**

가장 확실한 해결책은 매번 포트를 열고 닫는대신, 미리 열어둔 커넥션을 재사용 하는것이다.

```java
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.http.impl.client.HttpClients;
import org.apache.http.impl.conn.PoolingHttpClientConnectionManager;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class HttpClientConfig {

    @Bean
    public CloseableHttpClient httpClient() {
        // 커넥션 풀 매니저 생성
        PoolingHttpClientConnectionManager connectionManager = new PoolingHttpClientConnectionManager();
        
        // 풀의 전체 최대 커넥션 수
        connectionManager.setMaxTotal(500);
        // 단일 목적지(Route/Domain) 당 최대 커넥션 수 (포트 고갈을 막는 핵심 설정)
        connectionManager.setDefaultMaxPerRoute(100);

        return HttpClients.custom()
                .setConnectionManager(connectionManager)
                // 서버가 명시하지 않더라도 기본적으로 20초간 Keep-Alive 유지
                .setKeepAliveStrategy((response, context) -> 20 * 1000) 
                .build();
    }
}
```

**네트워크 및 DNS 추적을 위한 os 필수 명령어**

장애 발생시 서버의 터미널에서 즉각적으로 원인 파악을 할 수 있는 명령어들이다. 

1. SNAT 포트 고갈시 TIME_WAIT 상태를 확인해보자 외부 통신후 닫히지 않고 대기중인 소켓의 총 개수를 확인하고 이 수치가 os가 설정한 임시 포트 한계치 약 28,000개가 보통. 에 근접했다면 포트 고갈이 진행중인것이다.

```bash
$ ss -tan state time-wait | wc -l
> 28451
```

2. 전체 tcp 소켓 상태 요약 확인 명령어. 서버의 전반적인 네트워크 연결 상태를 한 번에 파악한다. timewait 항목이 비정상적으로 높은지 점검하자.

```bash
$ ss -s

Total: 29102
TCP:   28900 (estab 120, closed 28550, orphaned 0, timewait 28451)

Transport Total     IP        IPv6
RAW       1         0         1
UDP       15        10        5
TCP       350       300       50
INET      366       310       56
FRAG      0         0         0
```

3. DNS 해상도 소요시간 latency 점검이다. 타겟 도메인의 ip를 확인하는 동시에 dns 서버로부터 응답을 받기까지 소요된 시간 Query time을 점검한다.

```bash
$ dig api.github.com

; <<>> DiG 9.16.1-Ubuntu <<>> api.github.com
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 54321
;; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 1

;; ANSWER SECTION:
api.github.com.         60      IN      A       140.82.112.5

;; Query time: 5012 msec  <-- (주의: 5초 이상 소요 시 DNS 지연/타임아웃 발생 중)
;; SERVER: 127.0.0.53#53(127.0.0.53)
;; WHEN: Sun Mar 29 14:10:00 KST 2026
;; MSG SIZE  rcvd: 59
```

4. 실시간 DNS 패킷 스니핑 (Throttling 및 유실 추적)이다. os 수준에서 오고가는 모든 DNS 질의(UDP 53포트) 패킷을 캡처한다. 애플리케이션이 불필요하게 동일한 dns 쿼리를 수백번씩 반복하고 있는지, 혹은 dns 서버가 응답을 주지 않아도 재시도 retransmission가 발생하는지 확인한다.

```bash
$ sudo tcpdump -i any -n port 53

tcpdump: verbose output suppressed, use -v or -vv for full protocol decode
listening on any, link-type LINUX_SLL (Linux cooked v1), capture size 262144 bytes
14:10:01.102341 IP 10.0.1.5.49152 > 168.126.63.1.53: 1234+ A? api.github.com. (32)
14:10:06.102500 IP 10.0.1.5.49152 > 168.126.63.1.53: 1234+ A? api.github.com. (32) <-- 5초 후 응답이 없어 동일한 쿼리 재시도
14:10:06.115000 IP 168.126.63.1.53 > 10.0.1.5.49152: 1234 1/0/0 A 140.82.112.5 (48) <-- 뒤늦은 응답 도착
```

<br>

### 트레이드 오프 모니터링 포인트

커넥션 풀과 keep-alive를 도입하여 문제를 해결한 뒤에는 다음 사항들을 관리해야하는데.

**타임아웃 및 재시도 최적화**: dns 지연이나 일시적인 네트워크 순단 발생시, 즉각적인 무한 재시도는 타겟 서버와 네트워크 인프라에 더 큰 부하를 준다. 지수 백오프 Exponential Backoff와 Jitter 난수대기시간 등으로 재시도 트래픽을 시간축으로 분산시켜야한다.

**커넥션 풀 모니터링**: 커넥션 풀을 과도하게 크게 설정하면 애플리케이션 메모리 낭비 뿐만아니라 타겟 db나 api 서버 측의 커넥션 제한이 걸려 또 다른 장애를 유발할 수 있다. APM 도구를 활용해 `Active Connections`과 `Idle Connections` 비율을 모니터링하여 최적의 `MaxPerRoute` 값을 도출해야한다.

**DNS 캐싱 (JVM 레벨)**: Java Kotlin 환경의 경우 networkaddress.cache.ttl 값을 튜닝해 애플리케이션 레벨에서 변환된 ip를 일정시간 캐싱하도록 설정하면 dns 서버로 향하는 쿼리 자체를 대폭 줄여 dns throttling을 예방할 수 있다.