# 마이크로서비스에서 분산 트랜잭션 관리하기

분산 트랜잭션을 관리하는 방법으로는 크게 두가지가 있다.

- Two Phase Commit Protocol
- Eventual Consistency and Compensation (SAGA Pattern)

### Two Phase Commit Protocol

트랜잭션 처리와 데이터베이스 컴퓨터 네트워킹에서 정보가 성공적으로 수정되었음을 확인하는 ACP(Atomic Commit Protocol)이다.

트랜잭션 성공과 실패를 확인하고 이러한 작업들이 원자적으로 이루어질 수 있도록 조정하는 분산 알고리즘을 제공한다.

2PC(Two Phase Commit) 동작 과정을 살펴보면 **트랜잭션 관리자인 Coordinator**가 필요하고 이 외의 나머지 노드들은 cohort(or participants)라고 불린다.

![](https://camo.githubusercontent.com/9cb280c73cd9b93c311f1f55c8a6cef42b4cfee391ce50d5f4cd50c60c730cee/68747470733a2f2f646f63732e676f6f676c652e636f6d2f64726177696e67732f642f31684c73353650356e77675a566e5266764845397747386e4e4479425f366b475f6443546b394d6f305a304d2f7075623f773d35343326683d353230)

2pc는 작업 요청 단계와 커밋 단계 인 2 단계로 구분이 되어 있다.

#### Prepare Phase
1. Coordinator는 cohorts들에게 query to commit 메시지를 전송하고 응답이 끝나기를 기다린다.
2. cohorts들은 트랜잭션 지점을 설정하고 진행한 뒤에 커밋 준비를 한다. 실패하는 cohort가 발생할 경우에는 실행 취소를 위해 redo log와 undo log를 준비한다.
3. 각 cohorts들은 agreement 메시지를 전송한다. 작업 성공, 실패 여부를 나타낸다.

#### Commit Phase - Success

모든 cohorts들로부터 success agreement 메세지를 받았다면 성공이고 커밋을 실행한다.
1. Coordinator는 모든 cohorts들에게 커밋 메세지를 전송한다.
2. 각 cohorts들은 커밋 후 리소스 잠근을 해제한 후 coordinator에게 acknowledgement를 전송한다.
3. 모든 cohorts로 부터 ack를 받으면 작업이 완료된다.

#### Commit Phase - Failed

하나 이상의 cohorts 들로 부터 작업 실패 agreement 메시지를 받았거나 timeout이 발생하면 실패고 롤백을 수행한다.

1. coordinator는 모든 cohorts 들에게 rollback 메세지를 전송한다.
2. 각 cohorts 들은 undo log를 이용해서 롤백하고 리소스 잠금을 해제한 후에 ack를 전송한다.
3. 모든 cohorts로부터 ack를 받으면 트랜잭션을 복구한다.


2PC는 Blocking Protocol이기 때문에 Coordinator가 영구적으로 실패하면 트랜잭션을 영원히 해결짓지 못하는 cohorts들이 생길 수 있으며, 2PC는 DBMS간 분산 트랜잭션을 지원해야 적용이 가능한데, NoSQL은 지원하지 않고 함께 사용하는 DBMS가 동일해야한다.

그리고 2PC는 보통 하나의 엔드포인트를 통해 서비스 요청이 들어오고 내부적으로 DB가 분산되어 있을 때 사용하는 반면에 MSA 환경에서는 애플리케이션이 분산되어 있어 각각 다른 app간의 api 통신을 통해 서비스 요청이 이루어지기 때문에 구현이 어렵다.


<br>

### Saga Pattern

**Saga는 2PC와 다르게 트랜잭션 관리 주체가 DBMS가 아닌 애플리케이션이다.** MSA와 같이 애플리케이션이 분산되어 있을 때, 각 애플리케이션 하위에 존재하는 DB 로컬 트랜잭션 처리만 담당한다. 따라서 각각의 애플리케이션에 대해서 연속적인 트랜잭션 요청이 실패할 경우, 롤백 처리를 애플리케이션 단에서 구현해야한다.

**Saga 패턴은 Choreograph-based Saga와 Orchestration-based Saga로 두 종류가 있다.**

#### Choreograph-based Saga

**각 서비스마다 로컬 트랜잭션을 관리**하며 현재 상태를 바꾼 후 완료가 되었으면 완료 이벤트를 발생시켜 이벤트를 다음 트랜잭션을 관리하는 서비스에 전달하여 트랜잭션을 처리하는 방식으로 구현된다. 만약 트랜잭션이 롤백되어야할 경우, **보상 이벤트**를 발생시킴으로써 보상 트랜잭션이 실행될 수 있도록 하여 트랜잭션을 관리한다.


상품 주문 예제 ex.
- commit
	- order service가 order를 생성시키고 pending 상태로 놔둔 후 order created 이벤트를 발행한다.
	- customer service가 order created 이벤트를 consume하고 credit을 생성한 후 credit reserved 이벤트를 생성한다.
	- order service는 credit reserved 이벤트를 받은 후 pending 상태의 order를 approved로 변경해 트랜잭션을 커밋할 수 있도록 한다.
- rollback
	- order service가 order를 생성시키고 pending 상태로 놔둔 후 order created 이벤트를 생성한다.
	- customer service가 order created 이벤트를 받았지만 credit 제한이 걸려 credit을 생성할 수 없다면 credit limit exeeded 이벤트를 생성한다.
	- order service는 credit limit exeeded 이벤트를 받은 후 pending 상태의 order를 reject로 변경하여 트랜잭션을 롤백시킨다.

이렇게 구현하게 되면 장점으로는 별도의 오케스트레이션이 없어 성능상에 이점이 있게 된다 인스턴스를 만들지 않아도 되거나 별도의 오케스트레이터 서비스가 없어도 되기 때문이고 이에 따라 구현하기 쉽고 개념에 대해서 이해하기 쉽다는 장점이 있다

그러나 단점으로는 트랜잭션 시나리오가 하나 추가된다면 관리포인트가 늘어나는 단점이 존재하고 어떤 서비스가 어떤 이벤트를 송수신하는지 추측하기 어려우며 모든 서비스는 호출되는 각 서비스의 이벤트를 들어야한다.


#### Orchestration based Saga

하나의 책임을 가지는 여러 개의 서비스와 그 서비스들 간의 트랜잭션 처리를 담당하는 orchestrator가 존재한다. choreography-based saga 처럼 각 서비스가 서로 다른 서비스의 이벤트를 청취해야 하는 것 과는 다르게 orchestrator가 모든 서비스의 이벤트를 청취하고 엔드포인트를 트리거할 책임을 갖고 있다.

![](https://camo.githubusercontent.com/b199d7296ddb2085512dceae22fc5afa5d47406fa4e29726823f0f9aed94e707/68747470733a2f2f6d69726f2e6d656469756d2e636f6d2f6d61782f3638332f312a4f78666462667358324d377172763557735358414d672e706e67)

위의 다이어그램에서, order orchestrator는 command/reply 방식으로 각 서비스와 통신한다.

orchestration based saga에서는 orchestrator가 한 트랜잭션의 흐름을 모두 알고있다는 것을 알 수 있다. 만약 트랜잭션 에러가 난다면 그 에러로 인해 발생하는 이전의 대한 모든 것들을 롤백하는 책임 또한 오케스트레이터가 가지고 있다.

오케스트레이터가 각 변환이 command나 message에 해당하는 상태 시스템으로 볼 수 있으므로 orchestration based saga를 구현하는 방식 중 하나는 `State Machine Pattern`을 적용하는 것이다. State Machine Pattern은 구현하기 쉽기 때문에 잘 정의된 동작을 구조화하는 데 좋은 패턴이다.

orchestration based saga는 트랜잭션 시나리오에 변화가 생겨도 오케스트레이터만 변경하면 되기 때ㅜㅁㄴ에 유지보수에 용이하고 모든 서비스와 통신하기 때문에 서비스간 순환 참조도 피할 수 있다.

허나 구현하기 힘들다는 단점이있고 오케스트레이터에게 트랜잭션 관련된 로직들이 엄청 많이 쌓이는 비즈니스 로직이 추가된다면 유지보수에 엄청 힘들어 질 것이다. 그렇기에 orchestration based saga를 구현한다면 트랜잭션 순서에 관한 로직만 작성할 수 있도록 관리하자(only command or reply) 



