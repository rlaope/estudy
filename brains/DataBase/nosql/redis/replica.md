# Redis important fact about replication

### Async Replication, WAIT

Redis는 낮은 대기시간과 높은 성능을 제공하기 위해 기본적으로 비동기적으로 master 데이터 복제를 한다.

복제의 신뢰도는 replication에서 주기적으로 마스터로부터 수신한 데이터의 양을 비동기적으로 확인해 어떤 명령 스트림들을 처리했는지 확인한다.

redis는 wait을 통해 특정 데이터들에 대해 동기적 복제를 요청할 수 있다.

<br>

### important facts about redis replication

위에서 언급한 바와 같이 레디스는 기본적으로 비동기 복제를 사용하며 하나의 마스터는 여러 복제본을 가질 수 있다.

replica는 또 다른 replica를 가질 수 있으며, 하위 replica를 마스터에 연결하는 것 외에도, 계단식 구조로 복제가 가능하다. (redis 4.0 부터 모든 하위 복제본은 마스터로부터 동일한 복제 스트림을 수신한다.)

복제는 master, replica에서 모두 non-blocking 동작하며, master에서 replica 동기화나 복제를 수행하는 중에도 쿼리를 계속 처리할 수 있다.

replication은 또 확장성을 위해 사용될 수 있으며, 여러 복제본을 읽기 전용 쿼리나 느린 On 연산용으로도 사용할 수 있다. 또한 데이터 안정성과 고가용성을 향상시키는 장점도 존재한다. (replica 노드들을 관리해줘야하는 비용은 늘어나지만)

master 노드에 쓰기 비용 절감 용으로 전체 데이터를 persistence하는 쓰기 비용을 절감하기 위해 복제를 사용할 수 있는데, master의 redis.conf를 지속하지 않도록 구성하고 간헐적으로 저장하거나 aof(append-only file)을 활성화한 replica를 통해 데이터를 디스크에 쓰는 방식으로 활용한다.

master a가 영속화 옵션 없이 replica에 복제만 하는 경우, a가 다운되었다가 재로딩 되었을때 이때 master a는 텅 비어있는 데이터 셋으로 로딩되고, 이를 replication 하는 노드들이 동기화하면서 이전 데이터셋을 손실시킬 수 있다. 메모리 데이터셋이여도 당연히 영속성이 필요한 데이터라면 꼭 영속옵션을 활성화 시키도록 하자.

<br>

### redis replication works

redis는 복제 작업이 다음과 같은 방식으로 동작한다.

1. redis master 노드는 고유한 Replication ID를 가지고 연결된 replica가 존재하지 않아도 Increment Offset을 가진다. 이는 복제가 이루어져 replica에 변경사항이 전달되면 증가한다.
2. replica가 master와 연결할 때 **psync 명령을 사용하여 이전 master와 replication id와 지금까지 처리한 offset을 전달해 데이터 복제 부분만 보낼 수 있게 된다.**
3. 만약 master의 버퍼에 백로그가 없거나 replication id가 유효하지 않은 id를 참조한다면 전체 동기화를 발생시킨다.
4. 전체 동기화가 발생하면 master에서는 백그라운드 저장 프로세스가 시작되어 Rdb를 생성하고 동시에 클라이언트로부터 받고 있는 새로운 쓰기 명령을 버퍼링한다.
5. 이후 백그라운드 저장이 완료되면 rdb 파일을 먼저 복제본에 전송하고 이후 버퍼에 담긴 쓰기 명령들을 보낸다.

<br>

### Allow writes only with n attached replicas

redis 2.8 부터 redis master에 n개의 replica가 연결된 경우에만 쓰기 쿼리를 수행한다.

하지만, redis는 비동기 복제를 사용하기 때문에 복제본이 특정 쓰기를 실제로 받았는지 보장할 수 없어 데이터가 유실될 수 있다.

이러한 문제에 대해 replica, master가 서로 `Ping-Pong`을 주고받으며 replica에 처리한 복제 스트림의 양을 확인한다.

그리고 master에서 각 replica로부터 마지막으로 핑을 받은 시간을 기억해두고 지연시간을 측정해 사용자나 지정한 n개의 복제본과 m초 미만의 지연의 경우엠나 쓰기 작업이 허용되도록 구성할 수 있다.

```
min-replicas-to-write <N>
min-replicas-max-lag <M>
```

조건이 충족하지 않으면 마스터는 오류로 응답하고 쓰기를 수행하지 않는다.

<br>

### How Redis Replication deals with expires on keys

Replica에서는 Key를 스스로 만료시키지 않는다. master, replica의 시계의 동기화로 의존한다면 데이터 일관성의 큰 결함을 초래할 수 있기 때문이다.

다음과 같은 기술을 통해 만료시킨다. **master에서 키를 만료 혹은 evict 되면 DEL 명령을 모든 복제본으로 전송해 제공한다.**

이러한 방식은 replica에서 DEL 을 제공받지 못한 경우 문제가 발생할 수 있다. 논리적으로 만료된 키가 유지될 수 있는데, 이를 처리하기 위해 논리적 시계를 사용해 일관성을 위반하지 않는 읽**기 작업에 대해서만 키 만료를 전달한다.**

<br>

### Partial sync after restarts and failovers

redis 4.0부터 페일오버 이후 마스터로 승격된 인스턴스는 이전 마스터의 복제본과 부분 동기화를 수행할 수 있다. 이 과정에서 승격된 replica는 이전 마스터의 replication id, offset을 통해 백로그 일부를 제공할 수 있다.

이때 승격된 master는 새로운 복제 id를 가지게 되며 새로운 아이디를 갖는 이유는 이전 master 복구가 돌아오면 중복되는 replication id, offset 쌍이 존재할 수 있기 때문이다.

graceful 종료 이후 재시작된 경우 rdb 파일에 master와 재동기화하는 데 필요한 정보를 저장해 쉽게 부분 동기화가 이루어질 수 있다.

aof 를 통해 재시작된 복제본은 부분 동기화를 수행할 수 없다고 한다. 그러므로 인스턴스를 종료하기 전에 rdb 지속성으로 전환한 다음 재시작을 수행하고 다시 aof 방식을 활성화 해야한다.


[[Redis Replication]]