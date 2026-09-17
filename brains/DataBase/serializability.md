# Serializability

트랜잭션을 동시에 실행할 경우 여러 이상 현상들이 발생할 수 있다.

이러한 이상 현상들을 일어나지 않도록 하는 중요한 특성이 Serializability, Recoverability다.

## Serializability

DBMS는 여러 사용자의 요청을 동시에 수행하는 것이 필수적이다. 

하지만 동시에 일어나는 경우 트랜잭션간의 간섭을 없애고 데이터 정합성을 일관성 있게 제공해야한다.

이를 Isolation을 보장한다고 표현하며 다른말로 하면 Serializabiltiy를 보장한다고도 한다.

Serializabiltiy는 직렬화라는 뜻을 가지고 있는데 이는 트랜잭션 요청을 순차적으로 직렬화시켜 트랜잭션의 간섭을 없애고 데이터간의 정합성을 맞추는 것을 의미한다.

그래서 이런 Serializability를 어떻게 적용할 수 있을까?

<br>

### Schedule

Schedule은 다수의 트랜잭션이 동시에 실행될 때 그 트랜잭션들에게 속한 오퍼레이션들의 실행 순서다.

트랜잭션들이 여러개 수행될때 각 트랜잭션의 오퍼레이션들이 순차적으로 실행되냐 아니냐에 따라서 `serial-schedule`, `non-serial-schedule` 로 나뉘게 된다.

1/ **non-serial-schedule은 인터리브 실행 기법을 통한 트랜잭션 내부 연산이 병렬로 실행되는 것을 의미한다.** 

> 인터리브 서로 다른 메모리 뱅크에 번갈아 가며 가용성을 높이는 메모리 기법이다. 더 자세히 알아보도록 하자 나중에..

이 방법은 병렬적으로 처리하기 때문에 동시성이 높아지는 장점이 있다.

다른 트랜잭션의 I/O 작업이 소요되는 동안 다른 작업을 수행할 수 있다.

그러나 데이터의 정합성을 보장하지 못하는 문제가 생긴다.

2. **serial-schedule은 트랜잭션을 구분해서 겹치지 않도록 하고, 한 트랜잭션이 모두 실행되야 다른 트랜잭션을 실행하는 기법이다.**

이 기법의 장점은 순서를 완벽하게 보장할 수 있지만 성능이 낮아진다. 이는 I/O 작업이 끝날때까지 다른 트랜잭션은 대기하고 있기 때문이다.


<br>

### Serializability Schedule

위의 두가지 방법은 매우 치명적인 트레이드 오프가 있으며 이를 해결하기 위해 성능 개선을 위해
non serial schedule을 사용하면서도 serial schedule과 동일한 결과가 나올 수 있도록 한다.

이를 `conflict serializable`이라고도 하고 `serial schedule`과 `conflict equivalent`하다고 한다.

conflict는 두개의 오퍼레이션이 충돌하는 것을 의미한다. 여기서 충돌은 아래 세 가지 조건을 만족한다.
1. 오퍼레이션이 서로 다른 두 트랜잭션에 속해야한다.
2. 오퍼레이션이 같은 데이터에 작업한다.
3. 둘 중 하나의 오퍼레이션이라도 쓰기 작업을 해야한다.

Conflict Equivalent는 아래의 두 조건을 충족한다.
1. 같은 트랜잭션 오퍼레이션들로 구성된 두 schedule
2. 양쪽 트랜잭션 내의 confliction operation 실행 순서가 동일

non serial schedule이 serial schedule과 conflict equivalant 하다면. non serial schedule을 적용해 serializability를 보장하며 성능도 향상이 되는 것을 볼 수 있다.
