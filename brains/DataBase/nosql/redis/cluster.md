# Redis Cluster

Redis 는 기본적으로 In memory data store이며 많은 곳에서 사용된다.

미션 크리티컬한 애플리케이션에서도 레디스를 사용하는 경우가 있기에 가용성을 중요시하며 운영하는 것이 중요하다.

### 고가용성

몇몇 시스템은 높은 고가용성이 중요하다.

레디스같은 중요한 컴포넌트가 99%의 가용성을 유지한다면 1% 시간동안 시스템 장애가 발생할 수 있다. 그. 1%의 장애도 용납 못하는 시스템은 존재할 수 있다.

많은 시스템들은 최대한 안정적으로 유지되길 바라지만 우리가 예상할 .수 없는 문제들로 인해 결함이 발생할 수 있다. redis가 다운되더라도 큰 문제없이 동작하기 위해, 높은 가용성을 위해 우리는 redis cluster를 사용할 수 있다.

<br>

### Replication

기본적으로 높은 가용성을 보장하는 방법은 시스템을 중복으로 배치하는 것이고, 이러한 이중화 복제는 여러 복제본들을 동기화 하는 문제가 새로 생긴다. 

redis에서 레플리카를 구성할 때 어떻게 복제를 수행하는지 살펴보면 Primary Redis에 데이터가 있고, 새로운 복제본을 추가하는 경우에 Primary Redis를 fork하고 모든 데이터를 Secondary Redis로 전달한다.

> 이 과정에서 fork로 인한 메모리 사용량이 급증할 수 있으며 fork 이외의 데이터 전송 방법도 존재하기 때문에 트레이드 오프가 존재한다.

redis는 모든 복제본이 동기화되지 않아도 값을 변경하는 데 성공한다. 이는 redis의 성능을 위해 한 결정이며, replication은 eventually consistency를 보장하게 된다.

statement based replication을 사용하고 있으며, 비결정적인 몇몇 함수는 primary, replica에 따라 값이 달라질 수 있다. 이는 binlog 기반의 MySQL과는 다른 방식이다.

이러한 statement에는 각각의 id가 부여되며, primary, replica id가 갖다면 데이터가 전부 동기화 된 것으로 간주한다.

<br>

### Failover

고가용성을 위해 replication을 진행했기에 redis가 죽어도 시스템은 문제없이 동작한다.

> 사실 replication은 고가용성을 위함은 아니다. read의 경우에 replica에도 데이터가 있어 분산된 레디스 노드로도 처리할 수 있다. write를 primary가 충분히 버틸 수 있고 eventually consistency만 보장해도 괜찮다면 좋은 선택이다.

replica가 down된 경우에는 부하 분산을 위해 만들었다면 아쉽게 되었으나, 우리에게는 아직 primary가 살아있기 때문에 primary나 또다른 replica에서 작업을 처리할 수 있다.

만약 Primary가 죽게된다면? replica중 하나가 primary로 승격되고 그 승격되는 기준은 합의 알고리즘을 통해서 정해진다. replica가 primary랑 데이터가 같다는 보장은 없지만 동작은 한다. 하나였다면 동작 자체를 안했을 것이다.

#### Split Brain

분산 시스템에는 네트워크 파티션이 발생할 수 있다. primary가 아직 살아있거나 replica가 자신을 primary라고 생각할 수 있다. 만약 이런 상황에 레디스 클러스터는 잘 동작하지 않을 수 있다. (네트워크 파티션때문에 잘 살아있는 노드가 죽었다고 잘못판단하는 경우)

이러한 문제를 Split Brain 문제라고 하고 해당 문제는 [redis docs]([https://redis.io/docs/reference/cluster-spec/#failure-detection](https://redis.io/docs/reference/cluster-spec/#failure-detection)) 에서 분산 합의 알고리즘을 통해 모든 노드의 동의를 받아야 승격된다고 명시되어져있다. 

일시적으로 replica가 자신을 primary라고 인지할 수 있지만, 모든 노드의 동의를 받지 않았다면 승격 과정이 취소된다.

<br>

### Partitioning

Redis를 파티셔닝을 목적으로 사용할 때도 있다.

우리는 레디스의 메모리에 모든 데이터를 다 저장하기 힘들다. 커다란 시스템이라면 더더욱 그럴것이고... 이를 해결하기 위한 방법으로는 레디스를 안쓰는 것 외에, **데이터를 나누어서 저장**하는 방법을 택할 수 있다. 그런 방법을 우리는 파티셔닝, 샤딩 등으로 부른다.

데이터를 나누어서 처리하기 위해 어떠한 데이터가 노드에 위치되어있는지를 알아야한다. 우리는 주로 consistency hashing (일관된 해싱)을 사용한다. 간단하게 ring을 만들고 구간을 지정해 데이터의 위치를 방식이다.

**Redis Cluster는 기본적으로 일관된 해싱을 사용하지 않으며**, 그와 유사한 **해시 슬롯(Hash Slot)** 이라는 개념을 사용한다. Redis Cluster는 key의 대괄호 안의 (`KEY{HASH_TARGET}KEY)`)를 CRC16이라는 알고리즘으로 해싱한다. 그 결과는 0 ~ 16383까지의 결과값이 나오게 된다.

> 요청이 실패했을 경우에는 MOVED라는 결과 데이터를 저장해야하는 다른 노드 위치와 함께 반환되며, Redis Client가 다른 노드에 다시 쓰기 작업을 수행해야한다. 만약 이 내용이 클라이언트 라이브러리에서 지원되지 않는다면 직접 해야한다.

#### Re Partitioning


파티셔닝의 경우 노드가 부족해 새로운 노드를 추가하는 경우가 생기는데 그 경우 개발자가 직접 redis cluster에 등록해주어야하며, 새로운 노드가 맡아야하는 해시 값의 범위를 지정해주어야한다.

이 경우 다른 노드들과 해시 범위 조정이 일어나며 데이터 일부에 대한 replication이 진행된다.

<br>

### Gossip Protocol & BroadCast

위에서 redis cluster의 노드는 자신의 해시 범위가 벗어나는 데이터가 들어오게 된다면 MOVED라는 커맨드와 함께 새로운 destination을 반환한다. 또한 리파티셔닝을 하는 경우에 해시 키의 범위도 변경된다. 여기서 궁금한 점은 어떻게 다른 노드의 설정을 알 수 있는 것일까?

Redis는 `Gossip Protocol` 을 사용해 통신한다. gossip을 주고받는 동안 서로의 heart beat를 보내며 failure도 체크하고, 공유 되고 있는 설정이 업데이트 되었는지에 여부도 함께 전달한다.

깊게 파면 더 내용이 많지만 여기서는 노드끼리 데이터를 전달하는 방법이라고 알아두면 좋다.

Redis Cluster는 이러한 설정마다 **configEpoch**라는 단조 증가 값을 사용해 최신 상태를 판단하고 configEpoch 값이 더 작으면 그 설정은 변경된 것으로 간주하고 새로운 값을 적용한다.

**configEpoch 값은 failover시에도 증가되고, 개발자가 직접 리샤딩 하는 경우에도 증가한다.** configEpoch 전파가 충분히 빠르다면 이상적이지만, 실제로 그렇지 않다. 분산 시스템에서는 어떠한 이유로든 잘못된 상황이 발생할 수 있고, 동일한 configEpoch가 등장해 충돌될 수 있다. 이러한 문제를 해결해야한다. 다른 설정 2개가 존재하면 잘못 동작할 수도 있기 때문에..

분산 합의 알고리즘으로 failover 되는 경우에 모든 노드가 최종상태에 동의해야하기에 configEpoch는 고유한 값이 보장된다. 하지만 개발자가 직접 트리거하는 변경은 충돌을 야기할 수 있다. `CLUSTER FAILOVER [TAKEOVER]` command는 epoch를 강제로 변경하기에 충돌이 발생한다,

만약 이렇게 충돌이 발생한 경우라면 [configEpoch 충돌 해결 알고리즘](https://redis.io/docs/reference/cluster-spec/#configepoch-conflicts-resolution-algorithm)을 통해서 하나의 버전이 승리한다

1. 마스터 노드가 같은 configEpoch를 가진 또 다른 마스터 노드를 발견한다.
2. 같은 configEpoch를 가진 노드의 ID를 비교해 사전학적으로 작은 노드를 선출한다.
3. 선택된 노드의 currentEpoch를 1 증가시켜 새로운 configEpoch를 발행한다.

#### currentEpoch

configEpoch가 아닌 currentEpoch라는 개념이 등장했다. currentEpoch는 redis cluster의 브로드캐스트 알고리즘에서 중요한 역할을 한다.

분산 합의 알고리즘에는 여러가지가 있다. Paxos, Raft, ZAP과 같은 많은 알고리즘들이 상요된다. Paxos는 Cassandra에서, Raft는 etcd, ZAP은 Zookeeper Atomic Broadcast로 사용된다. 이러한 분산 합의 알고리즘은 리더 선출 과정에서 사용되며, 요청의 순서를 정할 때도 사용된다. (Total Order Broadcast)

**redis cluster의 경우 full mesh(노드가 다른 모든 노드와 연결되어있는 구조) 구조를 채택한다.** 이렇게 연결된 노드 사이에서 gossip protocol을 사용해 heartbeat를 체크하고 설정을 업데이트한다. 즉, 설정 변경을 gossip protocol로 broadcast한다.

각 노드는 포트를 열고, 이 포트를 노드끼리 데이터 교환을 한다. 이 과정에서 `CLUSTER MEET` 이라는 커맨드를 사용한다.

이렇게 노드끼리 통신하는 방법을 redis에서는  [Cluster Bus](https://redis.io/docs/reference/cluster-spec/#the-cluster-bus)라고 부르며, 연결되어 있는 방법은 Cluster Topology라고 한다. 이 케이스에서 Topology는 FullMesh가 된다.

데이터를 교환하는 경우에 설정도 같이 업데이트가 된다. **이러한 설정이 변경되었는지 configEpoch로도 알 수 있지만, 기본적으로 currentEpoch를 사용한다.** currentEpoch는 64bit unsigned int값이며 각 이벤트의 단조 증가 버전을 의미한다.

서로 가십을 주고받는 상황에 만약 자신의 currentEpoch가 더 작다면 더 큰 currentEpoch를 선택한다. 이렇게 가십을 주고받으면서 모든 노드는 가장 큰 값의 currentEpoch를 갖도록 합의한다.

#### Election, Promotion

failover 상황에서 어덯게 replica를 primary로 승격하는지 알아보자

선거 과정은 replica에서 진행한다. 가십을 주고받으면서 어떤 레플리카가 primary에서 보낸 heartbeat 실패가 확인되었다. primary가 down되었다고 할 때, 우리는 failover가 진행되어야함을 알 수 있고 매우 신속하게 또 정확하게 failover가 수행이 되어야 시스템에 문제가 생기지 않을 것이다.

```
1. replica의 primary가 fail 상태다.
2. primary가 0이아닌 수의 slot을 사용한다.
3. replication을 위한 primary와 replica의 link가 primary로부터 주어진 시간보다 길게 끊어지게 된다.
```

replica는 문제가 발견되었더라도 바로 선거를 시작하지 않고 아래 정의된 시간만큼 대기하고 선거를 시작한다. 정의에서 REPLICA_RANK는 replica가 primary 요청을 얼마나 많이 처리했는지를 나타낸다. 즉, primary의 offset과 replica의 offset 차이만큼 더 기다리게 된다. 따라서 replica와 primary의 데이터가 비슷할 수록 먼저 수행된다.

```c
DELAY = 500 milliseconds + random delay between 0 and 500 milliseconds + REPLICA_RANK * 1000 milliseconds.
```

replica는 드디어 선거를 시작한다. currentEpoch를 하나 올리고 Primary에 연결된 노드들에게 투표를 요청하고 이 요청은 가십을 주고받으며 진행된다. `FAILOVER_AUTH_REQUEST`를 모든 primary 노드들에게 전달한다. 그리고 `NODE_TIMEOUT`(최소 2초 이상)의 2배만큼 기다린다.

primary는 `FAILOVER_AUTH_ACK`를 통해 replica에 찬성을 응답하고 한번 찬성이 이루어지면 `NODE_TIMEOUT`의 두 배만큼 다른 replica에 투표할 수 없다. 이런 제한은 split brain을 완벽하게 막지는 못하지만, 문제를 해결하는데에는 도움이 된다.

replica는  `FAILOVER_AUTH_ACK`를 받으면 currentEpoch를 비교해 작은  `FAILOVER_AUTH_ACK`를 전부 무시한다. 따라서 가장 마지막에 전달된 투표 결과만 적용된다. replica 대부분이 primary로 부터 찬성을 받으면 선거에서 승리한다. 만약 일정한 시간안에 과반수의 선택을 받지 못했다면 선거가 중단되고 새로운 선거가 다시 시작된다.

primary 입장에서는 어떤 replica가 가장 적절한지 계산하지 않는다. replica들은 primary와 가까울 수록 더 빨리 선거를 시작할 것이고, 선거에서 이길 가능성이 높다. 최선의 replica를 primary가 선택하지 않으면 모든 선거에 꼭 찬성하는 것은 아니다. primary는 다음과 같은 조건이 충족되어야 선거에 찬성한다.

1. primary는 한 epoch에 한 번만 투표하고 찬성할 시, lastVoteEpoch라는 필드를 디스크에 저장하는 방식으로 보장한다. primary는 투표요청이 lastVoteEpoch보다 작은 경우에는 투표하지 않는다.
2. 투표할 권리를 가진 primary는 승격될 replica의 primary가 fail인 경우에만 투표한다.
3. primary는 currentEpoch보다 작은 투표 요청을 무시한다. 또한 요청과 같은 currentEpoch를 반환한다. replica는 새로운 투표 요청을 보낼 시, currentEpoch를 증가하기 때문에 예전 선거의 찬성 요청은 새로운 선거의 currentEpoch보다 작다는 것이 보장된다. 만약 이러지 않으면 투표 결과가 지연되는 경우 오래된 투표가 유효하다고 오해할 수 있다.

