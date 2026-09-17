# Kafka 파티션 리밸런싱중 발생하는 중복 메시지 처리, 멱등성 보장

### At-Least-Once 전달 보장과 멱등성

**At-Least-Once (적어도 한 번 전달 보장)**은 Kafka가 기본적으로 채택하고 있는 메시지 전달 의미론 Delivery Semantics이다. 프로듀서가 보낸 메시지는 시스템 장애가 발생하더라도 절대 유실되지 않고 컨슈머에게 전달됨을 보장하지만, 네트워크 지연이나 재시도 로직으로 의해서 **동일한 메시지가 두 번 이상 전달될 중복 문제를 내포하고 있다.**

**멱등성 Idempotency**는 동일한 연산을 여러 번 반복해서 실행하더라도, 시스템의 최종 상태는 한 번 실행했을 때와 동일하게 유지되는 성질을 뜻한다. 수학의 $f(f(x)) = f(x)$ 개념과 동일하며, 분산 시스템에 서는 클라이언트가 같은 결제 요청을 두번 보내더라도 결제는 단 한번만 승인되도록 설계하는 것을 의미한다.

### 리밸런싱 중 발생하는 치명적인 중복 결제

결제 시스템과 같이 데이터 정합성이 절대적인 환경에서 컨슈머 스케일 아웃시 다음과 같은 장애 시나리오가 발생할 수 있다.

1. **메시지 처리 시작**: A 컨슈머가 1번 파티션에서 주문번호 123의 결제 승인 메시지를 읽어와 데이터베이스에 결제 완료 처리를 수행한다.
2. **오프셋 커밋 지연 및 스케일 아웃**: A 컨슈머가 결제를 처리하고 kafka에게 여기까지 읽었습니다 하고 offset commit을 날리기 직전, 트래픽이 폭증하여 b 컨슈머가 새로 투입된다.
3. **리밸런싱 트리거**: 새로운 컨슈머가 합류함에 따라 컨슈머 그룹 내에서 파티션 소유권을 재분배하는 리밸런싱이 발생한다. 이 과정에서 a컨슈머는 1번 파티션의 소유권을 읽고 하던 작업을 중단 및 실패한다.
4. **중복 처리 (장애 발생)**: 1번 파티션을 새로 할당받은 b컨슈머는 a컨슈머가 커밋하지 못한 이전 오프셋부터 메시지를 다시 읽어드린다. 결과적으로 b컨슈머가 주문번호 123의 결제 승인 로직을 한 번 더 실행하여 **고객에게 이중 결제가 청구된다.**

### Example

이러한 중복 메시지를 방어하기 위해서 메시지 자체에 고유한 `Idempotency Key` 예를들어 주문번호나 트랜잭션 id를 포함시켜 애플리케이션 레벨에서 이를 검증해야한다. 가장 확실한 방법은 db unique constraint이다.

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
            // 1. 멱등성 키 저장 시도 (DB의 Unique Key 제약 조건 활용)
            // 이미 존재하는 키라면 DataIntegrityViolationException 발생
            IdempotencyRecord record = new IdempotencyRecord(idempotencyKey, "PROCESSING");
            idempotencyKeyRepository.saveAndFlush(record);

            // 2. 실제 비즈니스 로직 수행 (결제 승인 등)
            doPayment(message);

            // 3. 상태 업데이트
            record.setStatus("COMPLETED");

        } catch (DataIntegrityViolationException e) {
            // 중복 메시지 수신: 리밸런싱이나 재시도로 인한 중복이므로 무시하고 정상 종료(Ack) 처리
            // 주의: 여기서 예외를 던지면 무한 재시도(Poison Pill)에 빠질 수 있습니다.
            System.out.println("Duplicate message ignored for key: " + idempotencyKey);
        } catch (Exception e) {
            // 기타 비즈니스 예외 발생 시에는 트랜잭션을 롤백하고 적절한 에러 처리 (DLQ 전송 등) 수행
            throw e;
        }
    }
}
```

물론 source가 db다 보니까 대규모 환경에서는 db 자체에 물리적 한계가 존재한다. 더 빠르고 효율적인 대안을 찾으면 보통 분산락인데 redis기반 인메모리 멱등성 제어에 대해서 설명해보겠다.

#### Redis SETNX와 원자적 연산

**인메모리 멱등성 제어**: 디스크 기반의 rdbms 대신 RAM기반에 kv store를 활용하여 메시지 중복 여부를 밀리초 단위로 검증하는 아키텍처다.

`SETNX` 연산은 redis에서 제공하는 원자적 명령어로 특정 키가 존재하지 않을때만 값을 저장하고 성공을 반환하며 이미 존재하면 실패를 반환한다. 이를 통해 조회와 저장이라는 두 단계를 스레드 경합없이 단일 연산으로 처리할 수 있다..

DB 제어 방식과 IOPS 병목 및 커넥션고갈같은 db의 unique constraint를 활용한 멱등성 보장은 무결성 측면에서 완벽하지만 심각한 성능 제약을 동반한다.

1. **IOPS** 한계: db는 본질적으로 디스크에 데이터를 기록해 모든 kafka 메시지마다 멱등성 검증 쿼리 insert or select를 기록하기엔 스토리지 한계에 도달
2. **transaction overhead & lock**: db 트랜잭션 여닫는 비용 자체가 크며 중복 데이터는 예외 처리 비용도 발생하고 애플리케이션 성능을 저하시킨다.
3. **Connection Pool**: db응답속도가 느려지면 컨슈머 스레드들이 커넥션을 반환받지 못하고 대기하게 되며 결국 메시지 처리량이 급감하고 파티션 Lag가 폭증하는 연쇄장애도 발생이 가능하다.

그래서 db 트랜잭션 전에 아주가벼운 redis 접근으로 중복여부를 1차 차단할 수 있다.

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
        
        // 1. Redis SETNX를 활용한 원자적 락 획득 (TTL 24시간 설정)
        // 키가 없으면 저장하고 true 반환, 이미 있으면 false 반환
        Boolean isFirstRequest = redisTemplate.opsForValue()
                .setIfAbsent(idempotencyKey, "PROCESSING", Duration.ofHours(24));

        // 2. 중복 검증 로직
        if (Boolean.FALSE.equals(isFirstRequest)) {
            // 이미 다른 컨슈머(또는 이전 리밸런싱 전 스레드)가 처리 중이거나 완료한 메시지
            System.out.println("Duplicate message filtered by Redis: " + idempotencyKey);
            return; // 조용히 Ack 처리하고 종료 (DB 접근 원천 차단)
        }

        try {
            // 3. 실제 비즈니스 로직 및 DB 반영 (별도의 트랜잭션으로 분리)
            paymentService.executePaymentLogic(message);
            
            // 4. 처리 완료 상태로 업데이트 (선택 사항)
            redisTemplate.opsForValue().set(idempotencyKey, "COMPLETED", Duration.ofHours(24));
            
        } catch (Exception e) {
            // 예기치 않은 서버 에러 시, 재시도를 위해 Redis 락 해제
            redisTemplate.delete(idempotencyKey);
            throw e;
        }
    }
}
```

redis를 도입하여 db 부하를 획기적으로 줄여도 아키텍처 설계시 반드시 결정해야할 트레이드오프들도 있다.

- TTL 설정의 딜레마같은 멱등성 키를 메모리에 영구 보관할 수 없으니 ttl은 얼마로 잡을지 너무 짧게 잡으면 재전송된 동일 메시지가 중복 처리될 위험도 있다. 그렇다고 길게잡으면 oom 날수도 있고
- redis 장애시 fallback도 생각해야한다 fail-fast하게 동작할지 bypass할지. redis노드 자체가 다운되었을 경우를 생각하자
  - Bypass 정책은 redis가 응답하지 않으면 무시하고 바로 db사용하도록
  - fail-fast 정책은 결제와 같이 정합성이 절대적인 도메인에서는 redis가 다운되면 메시지 처리를 즉각 중단 exception 발생시켜 브로커 오프셋 커밋을 하지 않아 인프라 복구가 될때까지 기다린다거나

### Kafak Consumer Group 상태 확인 및 Lag 확인

모니터링도 알아보자 리밸런싱이 빈번하게 발생하거나 메시지 처리가 밀리고 있는지 확인하기 위해서 사용하는 명령어들이다.

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

- `CURRENT-OFFSET`: 컨슈머가 마지막으로 커밋한 위치다
- `LOG-END-OFFSET`: 프로듀서가 생산한 가장 최신의 메시지 위치
- `LAG`: 컨슈머가 아직 처리하지 못한 메시지의 수 (`LOG-END-OFFSET` - `CURRENT-OFFSET`) 위 결과에서 파티션 1의 Lag가 4990으로 쌓이고 있으며 파티션2는 리밸런싱등의 이유로 현재 할당된 컨슈머가 없음을 확인할 수 있다.


### 해결 이후 트레이드 오프 및 DLQ 운영 방안

멱등성 방어 로직을 애플리케이션에 도입해 중복 결제는 막을 수 있지만. 아까 말한것처럼 ttl이나 db를 썼을때 부하도 확인해야하고

**Dead Letter Queue 운영**도 진행해야한다. 멱등성 중복이 아닌 데이터 포캣 오류나 필수 파라미터 누락등 영원히 처리할 수 없는 메시지가 유입될 수 있다. 이를 계속 재시도하면 해당 파티션의 Lag가 무한정 증가하고 따라서 재시도를 초과한 메시지는 즉시 메인 토픽에 빼내어 별도의 DLQ로 격리하고 오프셋은 정상적으로 커밋하여 다음 메시지 처리를 이어가야한다. 이후 개발자가 dlq를 분석하거나 비동기로 테스크가 돈다거나 알람을 준다거나 보정작업이 필요하다.