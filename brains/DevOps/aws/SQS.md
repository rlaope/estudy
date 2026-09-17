# SQS

## AWS SQS란
- SQS는 Simple Queue Service의 약자
- 애플리케이션 간의 메시지를 전달하기 위한 아주 간단한 Queue라고 생각하면 된다.
- 지속성이 우수하고, 사용 가능한 보안 호스팅 대기열을 제공해, dead-letter queue, 표준 대기열, FIFO 대기열을 지원하고 있다.

## SQS와 MQ의 차이점
- SQS는 이름 그대로 **'간단한' 큐 서비스**이다. 다른 MQ에 존재하는 message routin, fan-out, distribution lists를 지원하지 않는다. **메시지 생산자가 만들어낸 메시지를 메시지 소비자가 가져갈 수 있게 해주는 것이 전부다.**
- Amazon MQ는 AMQP나 MQTT처럼 표준화된 여러 broadcast프로토콜을 완벽히 지원하는 fully managed 서비스이다. 복잡한 요구사항을 구현할 때 유용하며, AWS 외부에 있는 메시지 브로커를 AWS로 마이그레이션할 때 유용하다.


## SNS와 차이점

### SNS Simple Notification Service
1. publisher가 subscriber에게 메시지를 전송하는 관리형 서비스
2. publisher는 Topic에 메세지를 발행한다. Topic은 수 많은 Subscribers에게 전달될 수 있다. (fan out) 이때 전달 방식은 Lambda, SQS, Email 등 여러가지가 있다.
3. 다른 시스템들이 이벤트에 신경 쓰이는가? : Topic이 메세지를 publish하고 싶어하고, 사람들에게 발행됐다고 알리고 싶을 때

### SQS Simple Queue Service
1. 마이크로서비스, 분산 시스템 및 서버리스 애플리케이션을 쉽게 분리하고 확장할 수 있도록 지원하는 안전관리형 메세지 대기열 서비스
2. 시스템은 Queue로 부터 새로운 이벤트를 실시할 수 있다. Queue에 있는 메세지들은 한 명의 consumer 또는 하나의 서비스에서 실행된다.
3. 이 시스템이 이벤트에 신경쓰는가? : 내가 이벤트 수신자일 때.