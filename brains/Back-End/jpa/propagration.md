# JPA Propagation 트랜잭션 전파 단계

JPA Propagation은 트랜잭션 동작 도중 다른 트랜잭션을 호출하는 상황에 선택할 수 있는 옵션이다.
  
@Transactional의 propagation 속성을 통해 피호출 트랜잭션 입장에서는 호출한 쪽의 트랜잭션을 그대로 사용할 수도 있고, 새로운 트랜잭션을 생성할 수도 있다.
  
`REQURIED`: default 값이며 부모 트랜잭션 내에서 실행하여 부모 트랜잭션이 없을 경우 새로운 트랜잭션을 수행한다.  
`REQUIRED_NEW`: 기존에 트랜잭션이 있으면 그것을 중단하고, 새로운 것을 생성한다.
  
이 외에도 종류가 REQUIRED_NEW, SUPPORTS, MANDATORY, NOT_SUPPORT, NEVER, NESTED가 있다.

- MANDATORY: tx를 필요로 한다. 진행중인 tx가 없을경우 예외를 발생시킨다.
- SUPPORTS: tx를 필요로하지 않는다. 진행중인 tx가 있다면 사용한다.
- NOT_SUPPORTS: tx를 필요로 하지 않는다. 진행중인 tx가 있다면 중지하고 메서드가 종료된 후에 중지했던 tx를 다시 시작한다.
- NEVER: tx를 필요로하지 않는다. 진행중인 tx가 있다면 예외를 발생시킨다.
- NESTED: 진행중인 tx가 있다면 기존 tx에 중첩된 tx에서 메서드를 실행한다.