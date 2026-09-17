# Identifying Failure Points and Designing C/A Systems

Integration with external systems is one of the most challenging aspects of distributed system design.

Networks are unreliable. "Never trust the network," and additionally, I operate on the premise of "never trust humans," meaning I don't rely on assurances like "this issue won't occur with the external system" and instead implement defensive logic. Anyway,

I will summarize strategies for ensuring availability while maintaining data consistency.

### Identifying Failure Points: Whose Fault Is It?

The most frequent issue in communication with external entities is likely timeouts, a realm of uncertainty where it's impossible to know if a request was sent, processed, or disconnected on its way back.

**To track status, use a correlation ID** to assign a unique ID to every request and transmit it to the external entity to match logs.

You can also classify timeout types through **timeout granularity**.
- Connection Timeout: Occurs when a connection to the external server cannot even be established (highly likely to be an issue on our side or a network problem).
- Read Timeout: Occurs when a connection is established but there is no response (external entity delay or failure).
- Write Timeout: Occurs when sending request data over the network, due to our system's network transmission speed issues or if the data is too large.

Through **bidirectional logging (Audit Log)**, storing raw request data sent out and response data received at the gateway level serves as the sole basis for dispute resolution and reconciliation, preventing arguments like 'we sent it, but they didn't receive it' or 'they sent it, but we didn't receive it.'

### Timeout Granularity

Implementing timeout granularity is done because it's often difficult to clearly distinguish whether the cause of a failure is a **network issue (our/network problem)** or a **peer's logic processing issue**.

I understand the concept of separation, but I'm curious about the implementation, so I'll design and implement a bit.

Here's an HttpClient example:

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

When a connection timeout occurs, it's safe to simply retry the connection. This is because it's a connection operation, so idempotency concerns are low.

If a read timeout occurs, that's where the problem lies. Since data might have already been reflected in the external entity's database, rather than a simple retry, an API call to check the status or a compensating transaction is necessary.

### Error Handling for Failure Point Identification

When a timeout occurs, different exception classes should be thrown to allow for distinct subsequent handling.

- ConnectionTimeoutException: Send a notification to the infrastructure team or open a circuit to block further requests.
- ReadTimeoutException: For this case, record it in the DB as an unpaid or in-progress status, and then publish an event or process it so that a status reconciliation batch runs after 5 minutes.

<br>

### Architectural Patterns for Consistency

In distributed systems, it's impossible to bundle DB updates or external API calls into a single transaction. There are key patterns to address this.

The **Transactional Outbox Pattern** is a technique for atomically processing local DB updates and message publishing.
1. When processing business logic, **store the result along with an outbox table** (within the same DB transaction).
2. A separate **message relayer** reads this table and sends messages to external systems.
3. This approach guarantees at least once execution without data loss, even if the external system fails.

**Idempotency** must be ensured, preventing external systems or our system from processing the same request multiple times when retries occur.

Idempotency Key: Always pass the same key during external calls, allowing the server to filter out duplicate requests.

<br>

### Disaster Recovery Mechanisms: Retry, DLQ, and Compensating Transactions

Let's revisit compensating transactions, retries, and other mechanisms mentioned earlier.

#### Retry Strategy

Simple retries can exacerbate the load on external systems, sometimes referred to as a retry storm.
- Exponential Backoff: A method of exponentially increasing the retry interval.
- Jitter: Introduces randomness to retry points to prevent multiple servers from retrying simultaneously.

#### Dead Letter Queue (DLQ)

Messages that fail after a defined number of retries are isolated in a DLQ.
- Prevents infinite loops and stores data for failure analysis.
- After recovery, messages in the DLQ are reprocessed manually or via automated scripts.

#### Saga Pattern

In a multi-step business process, if a failure occurs midway, it's a logical rollback process that cancels previously completed operations.

If it were a single system and bound within the same transaction block, such handling wouldn't be necessary. However, in a distributed system, if operations are independently partitioned via API calls and cannot be bound into an atomic transaction,

when a failure occurs, a compensating transaction must be initiated to perform rollback operations and normalize other tasks.

Implementation approaches could include services exchanging events in a choreography style, or an orchestrator managing the success/failure of each step and executing compensating transactions upon failure.

<br>

### Record Management and Gateway

The gateway, serving as the entry point for external integration, performs more than just a simple connection role.

- **Request Record**: Records request time, parameters, and correlation ID.
- **Response Snapshot**: Stores raw response data from external entities, preparing for potential parsing errors in our system.
- **Status Management**: Granular status for pending, success, fail, unknown.
- **Circuit Breaker**: Immediately blocks requests if an external system failure is detected, preventing resource exhaustion in our system.

<br>

### Handling Inconsistency: Reconciliation

It's difficult to guarantee 100% consistency with technical mechanisms alone, so a post-correction process is absolutely necessary.

Through **batch reconciliation**, if we can receive external entity processing history files or API data (whether it's a full API specification or just data) at a certain time the next day, we can compare that data with our DB through a synchronization task to cover any missing items.

Or, by using a **status check API**, for items remaining in an UNKNOWN state, we can call the external system's inquiry API after a certain period to synchronize their status.

The core idea is that the most dangerous state in external integration is not success or failure, but **unknown**. To address this, we ensure delivery with an outbox, log all records at the gateway, respond with Saga and DLQ, and finally, use batch reconciliation to ensure consistency.

### Implementation

Transactional outbox pattern for processing business logic and storing message dispatch records.

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

Data like this is stored when a transaction begins.

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

This prevents scenarios where the DB fails even if the API call succeeds, or conversely, the DB succeeds but the app crashes just before the API call. The outbox is recorded only after a DB commit, which can be problematic in situations where the DB is not readily available. After all, databases are relatively heavy.

Message Relay & Gateway: To process records, transmission, and outbox stored in the DB, an API is called. At this point, CDC (Debezium) or a Polling Publisher is used.

The gateway should not only forward data but also log raw original data.

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

To handle UNKNOWN states, mechanisms like DLQ/Saga must be used.

When an unknown state occurs, it's not about blindly retrying, but rather going through a process of checking the status.

1. **Retry**: Retry about 3 times for simple network outages (may vary by policy).
2. **DLQ**: Send failed retry messages to order.reply.dlq.
3. **Saga-State Check**: For items accumulated in the DLQ, do not immediately mark them as failed. Instead, first call a status inquiry API to the external entity, asking 'Has this order been received?'
   1. If yes, update our DB to success.
   2. If no, rollback with a compensating transaction.

Finally, let's perform a final reconciliation for items not processed in real-time through batch reconciliation.

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

### If Outbox DB Commit is Heavy

In large-scale systems, when DB processing and outbox table writes occur together within business logic, sharing DB sessions and disk I/O, if the write load reaches a threshold, DB commit delays occur, degrading overall system performance.

To address this, let's explore **three strategies for minimizing DB load, their core concepts, and code examples**.

### CDC

This is the concept of directly reading DB logs (WAL) without creating an outbox table.

The application only updates business tables, and all DB changes are recorded in a WAL (Write-Ahead Log) or Binary Log. Tools like Debezium capture these logs in real-time and push them to systems like Kafka.

There is no additional insert load on the DB, and transaction atomicity is guaranteed at the DB log level.

> This content was mentioned while reading 'Distributed System Design and Implementation Patterns: 30 Patterns.' While there are considerations regarding the reliability of WAL logs, and the process of agreeing on consistency for hardware, generation, version, etc. (areas for further learning), let's skip them for this article.

So, if the application performs an operation like `update order set status = complete`, the DB internally generates WAL logs, Debezium reads these logs, sends them to the `order-events` topic in Kafka, and a consumer reads from Kafka to make API calls. This is the process.

<br>

### Transactional Messaging

There's also a strategy of notifying the message broker before writing to the DB, which seems like a lightweight version of 2PC.

The concept is to send a 'half' message (like 'prepare' in 2PC) to the broker, which is not yet visible to consumers.

The local DB transaction is then executed, and based on the DB result, a commit or rollback is sent to the broker.

Its features include not requiring a DB outbox table, with the broker managing the state. If the app fails to complete the third step, the broker provides a 'check back' feature, asking the app what happened.

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

### Local Segmented Outbox (Local Storage)

This is the concept of writing to a fast local storage instead of a central RDB. Instead of writing the outbox to the main DB, it's recorded in a local RocksDB or a performance-optimized sharded DB where the app is running.

The operation involves the main DB recording the outbox in a very fast local storage after a successful update. If step 2 fails, the main DB rolls back. This might require distributed transactions, or logs can be kept on a best-effort basis and corrected later.

Since this is local, it's used for extremely high-performance scenarios, such as ad click logs where tens of thousands of transactions per second occur.

In conclusion, all three approaches address the issue of DB commit overhead by recording write history, similar to an outbox, outside of the main DB.
