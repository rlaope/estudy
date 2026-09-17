# Recoverability


Recoverability는 트랜잭션이 실패했을 경우 회복 가능성을 말한다.

트랜잭션은 하드웨어, 소프트웨어 적 이슈등 다양한 이유로 중간에 실패한다.

이때도 트랜잭션은 원자성이 보장되어야 하기 때문에 트랜잭션이 실패하면 이전 상태로 복원이 가능해야 하고 이를 rollback이라고 표현한다.

한마디로 롤백에 이상현상이 없는가를 의미하며 DBMS는 schedule이 recoverable하도록 보장해야한다.

### irrecoverable Schedule

롤백해도 이전 상태로 회복이 불가능한 스케쥴을 의미한다.

![](https://velog.velcdn.com/images/j_user0719/post/d071091d-c762-46ee-aa7e-8dec75c05042/image.png)

더티리드는 위와 같이 트랜잭션 작업이 끝나지 않았지만 중간에 다른 트랜잭션이 작업 내용을 보는 경우를 의미한다.

결국 위와같이 더티 리드로 읽은 데이터를 가진 상태로 롤백하지 못하는 경우를 irrecoverable schedule이라고 한다.

<br>

### Recoverable Schedule

recoverable schedule이란 트랜잭션은 **자신이 읽고있는 데이터를 변경하고 있는 다른 트랜잭션들이 모두 커밋, 롤백 되기 전까지 커밋하지 않은 스케쥴을 의미한다.**

T1이 T2가 변경하고 쓴 값을 읽는 다면 T2 트랜잭션이(의존되는 트랜잭션) 완료되고 나서 값을 제어한다.

T1 -> T2 -> T3 -> T4 순으로 데이터릘 사용하고 있다면 T4가 커밋 혹은 롤백되기 이전의 트랜잭션들은 기다린다.

그러나 이러한 구조는 rollback을 할 때 비용이 크게 발생한다 여러 트랜잭션이 중첩되어 쌓여있기 때문

### Cascadelss Schedule

그렇게 위의 문제를 해결하기위해 다음과 같은 스케쥴이 나왔다. 이는 데이터를 쓰기한 트랜잭션이 커밋 혹은 롤백한 뒤에만 데이터를 읽을 수 있는 것이다.

결국 Read는 하지 않는 스케줄이고 Write까지는 막지 않는다.

### Strict Schedule

Cascadless 에서 write까지 함께 막는 스케쥴이 strict다.

W1(A) -> W2(A) -> W2 Commit -> W1 rollback 같은 순서라면, W1이 롤백하면서 W1이 발생하기 전의 데이터로 복구 되면서 W2가 커밋한 데이터가 사라지는 경우가 발생할 수 있다. 하지만 위 스케줄은 read자체가 없기 때문에 cascadeless 하다고 볼 수 있다.   여기에 추가적으로 보강해 읽지도 않을 뿐, 쓰지도 못하게 하는 경우를 strict scedule 라고 한다.

정리하면 recoverable schedule은 읽기 쓰기가 다 되며 의존되고 있는 트랜잭션이 커밋이나 롤백을 하지 않으면 의존하고있는 트랜잭션은 커밋할 수 없다.

cascadless는 커밋되지 않은 트랜잭션 데이터를 읽을 수 없으며 strict는 쓰기까지 막는다.