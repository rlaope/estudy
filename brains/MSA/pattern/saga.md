# Saga Pattern

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FLiCWK%2Fbtrx07FIYlZ%2F3Jt15icBW8abKVlW40UKd1%2Fimg.png)


Saga 패턴은 트랜잭션의 관리주체가 DBMS가 아닌 애플리케이션에 있다. app이 분산되어있을 때, 각 App 하위에 존재하는 DB는 local 트랜잭션 처리만 담당한다.


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlM0vB%2Fbtrx1ITg4Nj%2F4JO3jSBcbbsug1cc2n8hxk%2Fimg.png)


따라서 각각의 app에 대한 연속적인 트랜잭션 요청 및 실패의 경우에 rollback 처리(보상 트랜잭션)을 애플리케이션에서 구현해야 한다.

saga 패턴은 위 그림과 같이 연속적인 업데이트 연산으로 이루어져있으며, 전체 데이터가 동시에 영속화되는 것이 아니라 순차적인 단계로 트랜잭션이 이루어진다.

따라서 애플리케이션 비즈니스 로직에서 요구되는 마지막 트랜잭션이 끝났을 때, 데이터가 완전히 영속되었음을 인지하고 이를 종료한다.

Two Phase Commit과는 다르게 saga를 활용한 트랜잭션은 데이터 격리성을 보장해주지 않는다. 하지만 애플리케이션의 트랜잭션 관리를 통해 최종 일관성을 달성할 수 있기 때문에 분산되어있는 데이터베이스간에 정합성을 맞출 수 있다.

또한 트랜잭션 관리를 애플리케이션에서만 하기 때문에 DBMS를 다른 제품군으로 구성할 수 있는 장점이 있다.

하지만 이러한 일관성을 달성하기 위해서는 프로세스 수행 과정상 누락되는 작업이 없는지 면밀히 살펴야하며, 실패할 경우 에러 복구를 위한 보상 트랜잭션 처리에 누락이 없도록 설계해야한다.

<br>

## saga 패턴의 종류

### Choreography-Based Saga

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbJRsvG%2Fbtrx1HthITO%2F1RmKKioRKJYEILMpuhusO1%2Fimg.png)


Choreography-Based Saga는 자신이 보유한 서비스 내 Local 트랜잭션을 관리하여, 트랜잭션이 종료되면 이벤트를 발행한다.

만약 그 다음으로 수행해야할 트랜잭션이 있다면, 해당 트랜잭션을 수행해야하는 app에 완료 이벤트를 수신받고 다음 작업을 처리해야한다.

이때 Event는 Kafka와 같은 메시지 큐 서비스를 이용해 비동기 방식으로 전달할 수 있다.


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbQWoIC%2Fbtrx1Jdy2pS%2FgmtWWcOH2AkuQw3Hl71k9k%2Fimg.png)

각 app 별로 트랜잭션을 관리하는 로직이 있다. 따라서 중간에 트랜잭션이 실패하면, 해당 트랜잭션 취소 처리를 실패한 app에서 보상 이벤트를 발생하여 rollback을 시도한다.

위와 같은 구성은 구축하기 쉬운 장점이 있으나, 운영자의 입장에서 트랜잭션의 현재 상태를 알기가 어렵다.


### Orchestration-Based Saga

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdExzXU%2FbtrxXoWwu3A%2Fe2Yswn7fe6Kb4slODML6k1%2Fimg.pnghttps://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdExzXU%2FbtrxXoWwu3A%2Fe2Yswn7fe6Kb4slODML6k1%2Fimg.png)

Orchestration-Based Saga는 트랜잭션 처리를 위한 Saga 인스턴스(Manager)가 별도로 존재한다.

트랜잭션에 관여하는 모든 app은 Manager에 의하여 점진적으로 트랜잭션을 수행하여 결과를 Manager에게 전달한다.

비즈니스 로직상 마지막 트랜잭션이 끝나면 Manager를 종료하여 전체 트랜잭션 처리를 종료한다.

만약 중간에 실패하게 되면 Manager에서 보상 트랜잭션을 발동하여 일관성을 유지하도록 한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcebMKJ%2FbtrxYbWV7K6%2FtxEgJYtWdXRovRYiklw7xK%2Fimg.png)

모든 관리를 Manager가 호출하기 때문에 분산 트랜잭션의 중앙 집중화가 이루어진다.

따라서 서비스간의 복잡성이 줄어들고 구현 및 테스트가 상대적으로 쉽다.

또한 트랜잭션의 현재 상태를 Manager가 알고 있기 때문에 롤백을 쉽게 할 수 있는 것 또한 장점이다.

하지만 이를 관리하기 위한 Orchestrator 서비스가 추가되어야 하기 때문에 인프라 구현의 복잡성이 증가되는 단점이 존재한다.