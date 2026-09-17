# Transactional Outbox Pattern

### Transactional Event

우리가 개발하는 서비스는 보통 데이터베이스를 업데이트하는 트랜잭션과 함께 메시지를 발행한다.

이 때 DB를 업데이트 하는 작업과 메시지 전송을 한 트랜잭션으로 묶지 않으면 문제가 발생할 수 있다.

이 두 작업(db update, event publish)이 서비스 내에서 원자적으로 수행되지 않으면 시스템 실패시 문제가 발생하기 때문이다.

![](https://velog.velcdn.com/images/eastperson/post/d068766f-1f11-451a-83d8-fc4a7187d8b4/image.png)

**2PC(Two Phase Commit)**

기존에는 원자성을 보장하기 위해 분산 트랜잭션을 사용했다. 분산 트랜잭션은 2pc를 이용하여 트랜잭션 참여자가 커밋 혹은 롤백 할 수 있도록 원자성을 보장하는 방법이다. 그러나 분산 트랜잭션은 단일 db에 비해서 10배 이상 성능 저하가 발생할 수 있다. 또한 동기 IPC(Inter-Process Communication) 형태라 가용성이 떨어지는 문제도 있다.

현대 아키텍처는 일관성보다 가용성을 우선시 한다. 이러한 이유로 분산 트랜잭션 방식은 현대 애플리케이션에 적합하지 않으며 최신 메시지 브로커들은 이러한 기능을 제공하지 않는다.

우리는 다음 두 가지를 보장해야한다.
1. 데이터베이스 트랜잭션이 커밋되기 전 메시지가 발행되어야 한다. 반대로 트랜잭션이 롤백되면 메시지를 더이상 보내지 않는다.
2. 메시지 서비스는 보낸 순서대로 브로커로 보내야한다.

<br>

### Transaction Outbox Pattern

위에서 언급한 트랜잭션, 메시지 발행 원자성에 관해 생길 수 있는 문제점을 해결하기 위해 우리는 DB를 업데이트 하는 트랜잭션의 일부로 데이터베이스에 메시지를 저장하는 방법을 생각해낼 수 있다.

그런 다음에 별도의 프로세스나 배치 프로그램이 저장된 이벤트를 읽어 메시지 브로커에 전송하는 것이다. 이 것이 바로 **Outbox Pattern이다.**

애플리케이션은 데이터베이스의 outbox 테이블에 메시지 내용을 저장한다. 다른 애플리케이션이나 프로세스는 outbox 테이블에서 데이터를 읽고 해당 데이터를 사용하여 작업을 수행할 수 있다. 실패시 완료될 때까지 다시 수행할 수 있다.

따라서 outbox pattern은 적어도 한 번 이상(at-least once) 메시지가 성공적으로 전송되었는지 확인할 수 있다.

![](https://velog.velcdn.com/images/eastperson/post/c67e25eb-6f91-467e-a31e-6b3e3108b4d8/image.png)

![](https://velog.velcdn.com/images/eastperson/post/3ff5b14d-f0bf-4dce-bcb3-e8d8c14eddfe/image.png)

여기서 outbox는 보낼 편지함이라는 뜻이 있다. 전송되지 않았거나 전송에 실패한 메시지들이 모여있는 보관함이라는 뜻이다. 메시지를 보낼 데이터를 저장하는 저장소로 따로 두는 것이다. 

![](https://velog.velcdn.com/images/eastperson/post/8315f545-08a9-4aec-8739-1f95a2cf2a76/image.png)

이 패턴의 구성요소는 다음과 같다.
- Sender - 메시지를 보내는 서비스
- Database - 엔티티, 메시지 Outbox를 저장하는 서비스
- Message Outbox - 관계형 db인 경우 메시지를 저장하는 테이블, NoSQL인 경우 데이터베이스 record의 프로퍼티
- Message relay - outbox에 저장된 메시지를 메시지 브로커로 보내는 서비스

이 패턴에서는 Message Realy 라는 별도의 프로세스가 추가도니다. outbox 테이블은 임시 메시지 큐의 역할을 하며 엔티티 업데이트와 함께 트랜잭션으로 묶인다. Message Relay는 outbox 테이블에 저장하는 데이터를 비동기적으로 읽어서 메시지를 발행하여 메시지 브로커에게 전달하는 역할을 하게 된다. 

outbox pattern의 message relay를 구현하는데는 Polling publisher, Transaction log tailing 두 가지 방식이 존재한다.

<br>

### Polling Publisher Pattern

RDBMS를 사용하는 애플리케이션에서 outbox 테이블에 삽입된 메시지를 발행하는 간단한 방법으로는 테이블을 폴링해서 미발행되는 데이터들을 조회하는 것이다.

메시지 릴레이는 이렇게 조회한 메시지를 하나씩 각자의 목적지 채널로 보내서 메시지 브로커에 발행한다. 그리고 나중에 outbox 테이블에서 메시지를 삭제한다.

![](https://velog.velcdn.com/images/eastperson/post/e4787feb-f33b-460e-9b76-b8a688197c44/image.png)

DB 폴링은 규모가 작은 경우 쓸 수 있는 쉬운 방법이다. 하지만 db를 자주 폴링하게 될 경우 비용이 발생하고 NoSQL DB는 쿼리 능력에 따라 사용 가능 여부가 결정되게 된다.

<br>

### Transaction Log Tailing Pattern

메시지 릴레이로 DB 트랜잭션 로그(커밋 로그)를 테일링 하는 방법이다.

애플리케이션에서 커밋된 업데이트는 각 DB의 트랜잭션 로그 항목(log entry)로 남게 된다.

트랜잭션 로그 마이너(transaction log miner)로 트랜잭션 로그를 읽어 변경분을 하나씩 메시지로 브로커에 발행하는 방법이다.

![](https://velog.velcdn.com/images/eastperson/post/e7044be2-03cb-444b-a5b8-7870bbef4863/image.png)

mysql의 경우 transaction log miner는 트랜잭션 로그 항목을 읽고 삽입된 메시지에 대응되는 각 로그 항목을 메시지로 전환하여 메시지 브로커에 발행한다.

RDBMS의 outbox table에 출력된 메시지 또는 nosql db에 레코드에 추가된 메시지를 발행할 수 있다.

MySQL mysqlbinlog, PostgreSQL WAL, Oracle redolog 등을 활용해 변경사항을 읽어서 구현할 수도 있고, 구현 난이도가 높아 관련 툴을 사용하는 경우도 존재한다. 관련 툴은 디비지움(Debezium), 링크드인 데이터버스(Linkedin Databus), DynamoDB Streams, 이벤추에이트 트램등이 존재한다.

<br>

### Outbox Pattern With Kafka Connect

Kafka-Connect는 Kafka 브로커 외에 별도의 서비스로 실행된다. 아래 그림에서는 PostgreSQL을 사용했고 엔티티 업데이트가 발생할 때, outbox 테이블에 레코드를 추가하는 모습이다. Kafka-Connect는 **런타임 시점에 데이터베이스 변경사항을 캡쳐하기 위해 Debezium 커넥터가 배포된다.**  Debezuim은 outbox table 데이터베이스의 write ahead log를 추적하고 사용자 정의 커넥터에 의해 정의된 토픽 메시지를 발행한다.

![](https://velog.velcdn.com/images/eastperson/post/aa7d325f-7888-459d-bb0f-23e4a1b86b8c/image.png)

이 방법은 적어도 한 번 (at least once)를 보장합니다. 커넥터가 다운되고 실행될 동안 동일한 이벤트를 여러번 게시할 때가 있는데, 따라서 컨슈머는 멱등성 상태여야하며 중복 이벤트가 다시 처리되지 않도록 해야합니다.

이와같이 로그의 변경된 데이터를 감지하는 작업을 **CDC(Change Data Capture)** 라고 합니다. Debezium은 MySQL Connector는 bonlog를 읽어 insert, update, delete 연산에 대한 변경 이벤트를 만들어 kafka topic으로 전송해줍니다. 따라서 DB에 수행된 모든 이벤트가 안정적으로 수집되고 이벤트를 발행시켜 정확한 순서가 보장됩니다.

