# 장애 지점 식별 및 C/A 시스템 설계

외부 시스템과의 연계는 분산 시스템 설계에서 가장 까다로운 부분중에 하나인데.

네트워크는 신뢰할 수 없다. never trust network 및 추가적으로 나는 never trust 닝겐도 전제로 외부 시스템이 이 문제는 발생안할겁니다. 라는 조치를 믿지않고 방어로직을 두는편인데 어찌되었건.

데이터의 일관성을 유지하면서도 가용성을 확보하기위한 전략등등을 정리해보겠다.

### 장애 지점의 식별: 누구 잘못?

외부 기관과의 통신에서 가장 빈번한 문제는 timeout일건데, 요청이 갔는지, 처리되었는지, 아니면 돌아오다 끊겼는지 알수없는 불확실성의 공간이다.

**상태 추적을 하기위해선 이를 위한 corrlation ID**를 사용해서 모든 요청에 고유 id를 부여하고 이를 외부기관에 전닳해 로그를 매칭할 수 있다.

**timeout의 세분화**를 통해 타임아웃에 대한 유형을 분류해볼수도있고
- Connection Timeout: 외부 서버에 연결조차 안된 경우 (우리 측 혹은 네트워크 문제 가능성이 높음)
- Read Timeout: 연결은 되었으나 응답이 없는 경우(외부 기관의 지연 혹은 장애)
- Write Timeout: 요청 데이터를 네트워크로 보내는 시점 우리 시스템의 네트워크 전송 속도 문제나 데이터가 너무 큰 경우 발생

**양방향 로깅(Audit Log)**을 통해 gateway 레벨에서 나간 요청 데이터나 들어온 응답 데이터를 원본으로 저장해 추후에 우리는 보냈으니 저쪽은 안받았다 저쪽은 줬는데 우리는 못받았다 식의 분쟁 해결 reconciliation의 유일한 근거가 된다.

### 타임아웃 세분화 

타임아웃 세분화하여 구현하는 것은 장애의 원인이 **네트워크 문제인지(우리/망 문제)** 아니면 **상대 로직 처리 문제**인지 명확하게 구분하기 어려우니 하는것인데.

일단 나누는 것은 알겟고 어떻게 구현하는지 의문이 들어 조금 설계해보고 구현해보겠다.

httpClient 예시인데

```java
HttpClient httpClient = HttpClient.create()
    .option(ChannelOption.CONNECT_TIMEOUT_MILLIS, 3000) // 1. Connect Timeout (3초)
    .doOnConnected(conn -> 
        conn.addHandlerLast(new ReadTimeoutHandler(5000, TimeUnit.MILLISECONDS)) // 2. Read Timeout (5초)
            .addHandlerLast(new WriteTimeoutHandler(5000, TimeUnit.MILLISECONDS))); // 3. Write Timeout (5초)

WebClient webClient = WebClient.builder()
    .clientConnector(new ReactorClientHttpConnector(httpClient))
    .build();
```

connection timeout 발생시에는 단순히 다시 연결을 시도해야 안전하다. idempotency 걱정이 낮기 때문이다 연결하는 작업이기 때문에

read timeout 발생하게 된다면 이때가 문제인데 외부 기관 db에는 데이터가 반영되었을수도 있기 때문에 단순 재시도보다는 상태 확인 api 호출이나 보상 트랜잭션이 필요하다.

### 장애 지점 식별을 위한 error handling

타임아웃이 발생했을때 각각 다른 예외 클래스를 던지도록 하여 후속 처리를 다르게 가져가야한다.

- ConnectionTimeoutException: 인프라 팀에 알림을 보내거나 circuit을 열어서 추가 요청 차단
- ReadTimeoutException: 이 건은 미결제 또는 처리중 상태로 db에 기록하고 5분뒤에 상태 대조 배치가 돌도록 이벤트를 발행시키거나 처리해볼 수 있다.

<br>

### 일관성을 위한 아키텍처 패턴

분산 시스템에서는 db업데이트나 외부 api호출을 한 트랜잭션을 묶는것은 불가능하다 이를 해결하기 위한 주요 패턴들이 있는데.

**Transactional Outbox Pattern**이라는 로컬 db업데이트와 메시지 발행을 원자적으로 처리하는 기법이다.
1. 비즈니스 로직 처리시, 결과를 **outbox 테이블과 함께 저장**한다(같은 db 트랜잭션)
2. 별도의 **message relayer**가 이 테이블을 읽어 외부 시스템으로 메시지를 보낸다.
3. 이 방식은 외부 시스템 장애 시에도 데이터 유실없이 최소 한 번 실행을 보장한다 at least once

**멱등성 idempotency**를 보장해야하는데 재시도가 발생할 때 외부 시스템이나 우리 시스템이 동일한 요청을 중복 처리하지 않도록 해야된다.

Idempotency Key: 외부 호출시 항상 같은 키를 전달하여, 서버 측에서 중복 요청을 걸러내게 한다.

<br>

### 장애 복구 메커니즘 retry DLQ 그리고 보상 트랜잭션

아까 위에서 언급했던 보상트랜잭션이나 재시도 등등에 대해서 다시 알아보겠다.

#### 재시도 전략

단순한 반복은 외부 시스템에 가해지는 부하를 가중시킬수도 있다. retry storm이라고도 하는데
- Exponential Backoff: 재시도 간격을 지수 함수적으로 늘리는 방법
- Jitter: 재시도 지점에 무작위성을 부여하여 여러 서버가 동시에 재시도하는 것을 방지한다.

#### Dead Letter Queue(DLQ)

정해진 횟수만큼 재시도해서 실패한 메시지는 DLQ로 격리한다.
- 무한루프 방지 및 장애 원인 분석용 데이터 보관
- 장애 복구 후 DLQ의 메시지를 수동 혹은 자동화된 스크립트로 재처리한다.

#### Saga Pattern

여러 단계를 거치는 비즈니스 프로세스에서 중간에 실패했을때, 이미 완료된 앞선 작업들을 취소하는 논리적인 롤백 과정인데.

하나의 시스템이고 같은 트랜잭션 block에 묶여있었다면 이런 처리를 안해줘도 되지만 분산 시스템에서 api호출을 통해 서로의 작업을 독립적으로 분할되어 원자적인 트랜잭션에 묶을수없다면

실패했을때 롤백하는 작업등을 수행하는 보상 트랜잭션을 발생시켜 다른 작업들을 정상화 시켜줘야한다.

choreography하게 각 서비스가 이벤트를 주고받거나 orchestration 중앙 제어자가 각 단계의 성공 실패를 관리하고 실패시 보상 트랜잭션을 실행하는 등의 구현 방식이 있을것이다.


<br>

### 기록 관리 및 gateway

외부 연계의 관문이되는 gateway는 단순이 연결 통로이상의 역할을 수행한다.

- **Request Record**: 요청 시각, 파라미터, corrleation id를 기록
- **Response Snapshot**: 외부 기관의 응답 raw data를 저장, 우리 시스템의 파싱 오류 가능성 대비
- **Status Management**: pending, success, fail, unknown 상태 세분화
- **Circuit Breaker**: 외부 시스템 장애가 감지되면 즉시 차단하여 우리 시스템의 자원 고갈을 방지한다.

<br>

### 일관성이 깨졌을때 대응 reconciliation

기술적인 장치만으로 100% 일관성을 보장하기는 어렵다 따라서 사후 보정 프로세스가 반드시 필요하다

**배치 대조**를 통해 뭐 다음날 몇시 외부기관 처리 내역 파일 혹은 api로 제공받을수있다면 api 전문이면 전문 아무튼 그 데이터를 우리 디비와 대조하여 싱크하는 테스크를 통해 누락분을 커버한다거나

**상태 확인 api**를 통해 UNKNOWN 상태로 남은 건들에 대해 일정 시간후 외부 시스템의 조회 api를 호출해 상태를 동기화한다거나

핵심은 외부 연계에서 가장 위험한것은 성공 실패도 아닌 **알 수 없음**이라는 상태인 것이고 이를 위해 outbox로 전송을 보장하고 gateway에서 모든 기록을 남기며 saga dlq로 대응하되 최종적으로 배치 대조를 통해 정합성을 맞춰두기도 하자.

### 구현

비즈니스 로직 처리와 메시지 발송 기록을 저장하는 transaction outbox pattern

```sql
CREATE TABLE outbox (
    id UUID PRIMARY KEY,
    aggregate_type VARCHAR(255),
    aggregate_id VARCHAR(255),
    payload JSONB, -- 외부로 보낼 데이터
    status VARCHAR(50), -- READY, SENT, FAIL
    created_at TIMESTAMP
);
```

이런 느낌의 데이터를 트랜잭션이 시작할때 저장해두고

```java
@Transactional
public void createOrder(OrderRequest request) {
    // 1. 비즈니스 로직 처리
    Order order = orderRepository.save(new Order(request));

    // 2. Outbox에 기록 (메시지 발행 아님, DB 저장임)
    Outbox outbox = new Outbox(
        "ORDER", 
        order.getId(), 
        ObjectMapper.writeValueAsString(order), 
        Status.READY
    );
    outboxRepository.save(outbox);
}
```

이렇게하면 api 호출이 성공해도 db가 실패하거나 반대로 db는 성공했는데 api 호출직전에 앱이 죽는 경우를 방지한다. db 커밋이 되어야 outbox에 기록되는데 db가 여의치 않은 상황에서는 문제가 될수있다. 아무래도 디비는 상대적으로 무거우니까


Message Relay & Gateway 기록과 전송, db에 저장된 outbox를 처리하기 위해 api를 호출한다 이때 CDC(Debezium)를 쓰거나 Polling Publisher를 사용한다.

gateway는 단순히 전달하는것 뿐만 아니라 원본 데이터 raw data를 로깅해야한다.

```java
public Response callExternalSystem(Request req) {
    // 1. 요청 직전 로그 기록 (DB 또는 별도 로그 저장소)
    externalLogRepository.save(new ExternalLog(req.getId(), req.getPayload(), Status.PENDING));

    try {
        Response res = webClient.post()
            .uri("/external-api")
            .bodyValue(req)
            .retrieve()
            .bodyToMono(Response.class)
            .block(Duration.ofSeconds(5)); // Read Timeout 설정

        // 2. 성공 시 응답 기록
        updateLog(req.getId(), res, Status.SUCCESS);
        return res;
    } catch (WebClientResponseException e) { // 명확한 에러 (4xx, 5xx)
        updateLog(req.getId(), e.getResponseBodyAsString(), Status.FAIL);
        throw e;
    } catch (Exception e) { // Timeout 등 알 수 없는 에러
        updateLog(req.getId(), "TIMEOUT/UNKNOWN", Status.UNKNOWN);
        throw new UnknownStateException(e); // 중요: 별도 예외로 처리
    }
```

UNKNOWN 상태를 대응하려면 DLQ/Saga 같은 메커니즘을 사용해야하는데.

알수없음 상태가 발생하면 무작정 재시도가 아니라 상태를 확인하는 과정을 거쳐야한다.

1. **Retry**: 단순 네트워크 순단 3회정도 재시도(정책에따라다를순있음)
2. **DLQ**: 재시도 실패 메시지를 order.reply.dlq로 보낸다.
3. **Saga-State Check**: dlq에 쌓인 건들은 즉시 실패처리하지 않고 외부 기관에 이 주문이 들어왔니? 라고 묻는 상태 조회 api를 먼저 호출한다.
   1. 있으면 우리 디비를 success
   2. 없다면 보상 트랜잭션으로 rollback

그리고 마지막으로 배치 대조를 통해 실시간으로 처리되지 않은것들에 대한 마지막 대조를 해주자

```java
@Bean
public Step reconciliationStep() {
    return stepBuilderFactory.get("reconciliationStep")
        .<ExternalLog, Order>chunk(100)
        .reader(itemReader()) // 우리 DB의 'PENDING' 또는 'UNKNOWN' 상태 조회
        .processor(itemProcessor()) // 외부 API 호출해서 현재 상태 가져오기
        .writer(itemWriter()) // 결과에 따라 우리 DB 상태를 최종 업데이트
        .build();
}
```

### outbox db commit이 무겁다면

대규모 시스템에서는 비즈니스 로직에서 db처리와 outbox table write가 같이 들어가 db세션과 디스크 io를 공유해 쓰기 부하가 임계치에 도달하면 db commit 지연이 발생하고 전체 시스템 성능이 저하된다.

이를 해결하기 위해서 **DB 부하 최소화 전략 3가지와 핵심 개념 그리고 코드도 살펴보자**

### CDC

outbox테이블을 만들지 않고 DB의 로그를 직접 읽는 개념이다 WAL

애플리케이션은 비즈니스 테이블만 업데이트하고 db의 모든 변경사항을 WAL(Write-Ahead Log)나 Binary Log에 기록하는데 이 로그를 Debezium 같은 도구가 실시간으로 캡처하여 kafka같은곳에 던진다.

db에 추가적인 insert 부하가 전혀없고 트랜잭션의 원자성도 db 로그 수준에서 보장된다.

> 30가지 패턴으로 배우는 분산시스템 설계와 구현 패턴을 읽으면서 언급된 내용이고 또 이 WAL로그에 대한 신뢰성, hw, gen, version등에 대해서 일관성을 합의하는 과정등 고민할부분들이 있겠지만(학습할부분들이) 일단 이번글에선 패스하도록

그래서 application에서 update order set status = complete등의 operation을 했다면 db내부적으로 WAL로그를 생성하고 debezium이 로그를 읽어 kafka에 order-events 토픽 전송 및 cconsumer가 kafka를 읽어 api호출 이 프로세스다.

<br>

### Transaction Messageing

db를 쓰기전에 메시지 브로커에게 먼저 알리는 전략도 있는데 2pc 경량화 버전같긴한데.

개념은 브로커에 half(2pc에선 prepare 준비)메시지를 보내고 아직 컨슈머에겐 보이지 않는 상태다

로컬 db 트랜잭션을 이후에 실행하고 db결과에 따라 브로커에게 commit 혹은 rollback을 보낸다.

특징으로는 db outbox 테이블이 필요 없으며 브로커가 상태를 관리한다 만약 앱ㅇ이 3단계를 수행못하고 죽염ㄴ 브로커가 앱에 이거 어떻게 됐니라고 역으로 물어보는 check back 기능을 제공한다


(RocketMQ/KafkaTransaction)
```java
@Service
public class OrderService {

    @Transactional
    public void executeOrder(Order order) {
        // 1. Transactional Message 발행 (Half Message)
        // 실제로는 메시지 브로커가 제공하는 트랜잭션 리스너를 통해 제어됨
        rocketMQTemplate.sendMessageInTransaction("order-topic", 
            MessageBuilder.withPayload(order).build(), order);
    }
    
    // 2. 브로커가 DB 트랜잭션 성공 여부를 확인하기 위해 호출하는 콜백
    @RocketMQTransactionListener
    class OrderTransactionListener implements RocketMQLocalTransactionListener {
        @Override
        public RocketMQLocalTransactionState executeLocalTransaction(Message msg, Object arg) {
            try {
                // 실제 DB 비즈니스 로직 수행
                orderRepository.save((Order)arg);
                return RocketMQLocalTransactionState.COMMIT;
            } catch (Exception e) {
                return RocketMQLocalTransactionState.ROLLBACK;
            }
        }

        @Override
        public RocketMQLocalTransactionState checkLocalTransaction(Message msg) {
            // 3. 앱이 응답 없을 때 브로커가 "DB 들어갔니?"라고 다시 확인하는 로직
            String orderId = (String)msg.getHeaders().get("orderId");
            return orderRepository.existsById(orderId) ? 
                   RocketMQLocalTransactionState.COMMIT : RocketMQLocalTransactionState.ROLLBACK;
        }
    }
}
```

<br>

### Local Segmentented Outbox (Local Storage)

중앙 rdb대신 로컬에 빠른 저장소에 쓰는 개념인데 메인 db에 outbox를 쓰는 댓신 앱이 떠있는 로컬 rocksDB나 성능이 극대화된 샤딩 db에 outbox를 기록한다.

동작은 메인 db가 업데이트 성공이후 매우 빠른 로컬 저장소에 outbox를 기록 만약 2번이 실패하면 메인 db를 롤백한다 분산 트랜잭션이 필요할수도 있고 best effort 방식으로 로그남기고 사후 보정한다.

이건 로컬이라 극도의 성능 초당 수만건 이상의 트랜잭션이 발생하는 광고 클릭 로그나 그런것들에 쓰인다고 한다

결론적으로는 셋다 db commit이 문제라서 db이외에 outbox같은 write history를 기록하는 느낌이다.