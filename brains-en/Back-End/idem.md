# Handling Duplicate Messages and Ensuring Idempotency During Kafka Partition Rebalancing

### At-Least-Once Delivery Guarantee and Idempotency

**At-Least-Once** is the message delivery semantic that Kafka adopts by default. It guarantees that messages sent by a producer will never be lost and will be delivered to consumers, even in the event of system failures. However, due to network delays or retry logic, it inherently carries the **risk of duplicate messages being delivered more than once.**

**Idempotency** refers to the property where performing the same operation multiple times results in the same final system state as performing it once. This is analogous to the mathematical concept of $f(f(x)) = f(x)$. In distributed systems, it means designing a system such that even if a client sends the same payment request twice, the payment is approved only once.

### Critical Duplicate Payments During Rebalancing

In environments where data consistency is absolute, such as payment systems, the following failure scenario can occur when scaling out consumers:

1.  **Message Processing Begins**: Consumer A reads a payment approval message for order number 123 from partition 1 and processes the payment completion in the database.
2.  **Offset Commit Delay and Scale-Out**: Just before Consumer A commits the offset to Kafka, indicating that it has read up to this point, traffic surges, and Consumer B is newly deployed.
3.  **Rebalancing Trigger**: With the addition of a new consumer, rebalancing occurs within the consumer group to redistribute partition ownership. During this process, Consumer A loses ownership of partition 1, stops its ongoing work, and fails.
4.  **Duplicate Processing (Failure Occurs)**: Consumer B, newly assigned partition 1, starts reading messages again from the previous offset that Consumer A failed to commit. As a result, Consumer B executes the payment approval logic for order number 123 one more time, leading to **the customer being charged twice.**

### Example

To defend against such duplicate messages, the message itself should include a unique `Idempotency Key`, such as an order number or transaction ID, which must be validated at the application level. The most reliable method is using a DB unique constraint.

```java
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.dao.DataIntegrityViolationException;

@Service
public class PaymentConsumerService {

    private final PaymentRepository paymentRepository;
    private final IdempotencyKeyRepository idempotencyKeyRepository;

    public PaymentConsumerService(PaymentRepository paymentRepository, IdempotencyKeyRepository idempotencyKeyRepository) {
        this.paymentRepository = paymentRepository;
        this.idempotencyKeyRepository = idempotencyKeyRepository;
    }

    @Transactional
    public void processPaymentMessage(PaymentMessage message) {
        String idempotencyKey = message.getOrderId();

        try {
            // 1. Attempt to save the idempotency key (utilizing DB's Unique Key constraint)
            // If the key already exists, DataIntegrityViolationException will be thrown.
            IdempotencyRecord record = new IdempotencyRecord(idempotencyKey, "PROCESSING");
            idempotencyKeyRepository.saveAndFlush(record);

            // 2. Execute actual business logic (e.g., payment approval)
            doPayment(message);

            // 3. Update status
            record.setStatus("COMPLETED");

        } catch (DataIntegrityViolationException e) {
            // Received duplicate message: Ignore as it's a duplicate due to rebalancing or retries, and complete normally (Ack).
            // Caution: Throwing an exception here can lead to infinite retries (Poison Pill).
            System.out.println("Duplicate message ignored for key: " + idempotencyKey);
        } catch (Exception e) {
            // For other business exceptions, roll back the transaction and perform appropriate error handling (e.g., send to DLQ).
            throw e;
        }
    }
}
```

Of course, since the source is a DB, there are physical limitations to the DB itself in large-scale environments. If we look for faster and more efficient alternatives, distributed locks are common, so I will explain in-memory idempotency control based on Redis.

#### Redis SETNX and Atomic Operations

**In-memory Idempotency Control**: This architecture leverages RAM-based KV stores instead of disk-based RDBMS to verify message duplication in milliseconds.

The `SETNX` operation is an atomic command provided by Redis that stores a value only if the key does not exist, returning success. If the key already exists, it returns failure. This allows two steps, lookup and storage, to be handled as a single operation without thread contention.

While idempotency guaranteed by DB's unique constraints, using DB control methods and IOPS bottlenecks or connection exhaustion, is perfect in terms of integrity, it comes with severe performance limitations.

1.  **IOPS Limit**: Databases inherently write data to disk. Recording an idempotency verification query (insert or select) for every Kafka message can hit storage limits.
2.  **Transaction Overhead & Lock**: The cost of opening and closing DB transactions is high, and duplicate data also incurs exception handling costs, degrading application performance.
3.  **Connection Pool**: If DB response times slow down, consumer threads may not receive connections back and will wait, eventually leading to a sharp drop in message processing throughput and a surge in partition lag, potentially causing cascading failures.

Therefore, a very light Redis access can be used to block duplicates as a first line of defense before DB transactions.

```java
import org.springframework.data.redis.core.StringRedisTemplate;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.time.Duration;

@Service
public class PaymentConsumerRedisService {

    private final StringRedisTemplate redisTemplate;
    private final PaymentService paymentService;

    public PaymentConsumerRedisService(StringRedisTemplate redisTemplate, PaymentService paymentService) {
        this.redisTemplate = redisTemplate;
        this.paymentService = paymentService;
    }

    public void processPaymentMessage(PaymentMessage message) {
        String idempotencyKey = "idemp:payment:" + message.getOrderId();

        // 1. Acquire an atomic lock using Redis SETNX (TTL set to 24 hours)
        // If the key does not exist, it saves and returns true; if it already exists, it returns false.
        Boolean isFirstRequest = redisTemplate.opsForValue()
                .setIfAbsent(idempotencyKey, "PROCESSING", Duration.ofHours(24));

        // 2. Duplicate validation logic
        if (Boolean.FALSE.equals(isFirstRequest)) {
            // Message is already being processed or completed by another consumer (or a thread before previous rebalancing).
            System.out.println("Duplicate message filtered by Redis: " + idempotencyKey);
            return; // Silently Ack and terminate (completely blocks DB access)
        }

        try {
            // 3. Execute actual business logic and DB reflection (separated into a distinct transaction)
            paymentService.executePaymentLogic(message);

            // 4. Update status to completed (optional)
            redisTemplate.opsForValue().set(idempotencyKey, "COMPLETED", Duration.ofHours(24));

        } catch (Exception e) {
            // In case of an unexpected server error, release the Redis lock for retry.
            redisTemplate.delete(idempotencyKey);
            throw e;
        }
    }
}
```

Even with the introduction of Redis to significantly reduce DB load, there are still tradeoffs that must be decided during architecture design.

-   The dilemma of TTL settings: Since idempotency keys cannot be stored permanently in memory, how long should the TTL be? If it's too short, there's a risk of duplicate processing for retransmitted identical messages. If it's too long, it could lead to OOM errors.
-   Consider fallback in case of Redis failure: Should it operate in a fail-fast manner or bypass? Consider scenarios where the Redis node itself is down.
    -   Bypass policy: If Redis doesn't respond, ignore it and use the DB directly.
    -   Fail-fast policy: In domains where consistency is absolute, like payments, if Redis goes down, immediately stop message processing, throw an exception, and do not commit broker offsets, waiting until the infrastructure is recovered.

### Kafka Consumer Group Status and Lag Check

Let's also look at monitoring. These commands are used to check if rebalancing is occurring frequently or if message processing is falling behind.

```bash
kafka-consumer-groups.sh \
  --bootstrap-server kafka-broker01:9092 \
  --group payment-consumer-group \
  --describe

GROUP                  TOPIC           PARTITION  CURRENT-OFFSET  LOG-END-OFFSET  LAG             CONSUMER-ID                                     HOST            CLIENT-ID
payment-consumer-group payment-events  0          10050           10050           0               consumer-payment-1-xyz  /10.0.1.10      consumer-payment-1
payment-consumer-group payment-events  1          20010           25000           4990            consumer-payment-2-abc  /10.0.1.11      consumer-payment-2
payment-consumer-group payment-events  2          15000           15000           0               -                                               -               -
```

-   `CURRENT-OFFSET`: The last committed position of the consumer.
-   `LOG-END-OFFSET`: The position of the most recent message produced by the producer.
-   `LAG`: The number of messages the consumer has not yet processed (`LOG-END-OFFSET` - `CURRENT-OFFSET`). In the results above, Partition 1 has a lag of 4990, and Partition 2 currently has no assigned consumer due to rebalancing or other reasons.

### Trade-offs and DLQ Operations After Resolution

While introducing idempotency defense logic into the application can prevent duplicate payments, as mentioned earlier, you also need to consider the load when using TTL or a DB.

**Dead Letter Queue (DLQ) operations** must also be performed. Messages that can never be processed, such as data format errors or missing mandatory parameters, may be received, not just idempotency duplicates. If these are continuously retried, the lag for that partition will increase indefinitely. Therefore, messages that exceed retry limits should be immediately moved from the main topic to a separate DLQ, and the offset should be committed normally to continue processing subsequent messages. Afterwards, developers need to analyze the DLQ, or asynchronous tasks might run, or alarms might be triggered, requiring corrective actions.
