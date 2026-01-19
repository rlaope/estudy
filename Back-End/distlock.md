# Distributed Lock 취득 후 master node가 다운되었을때

redis 기반의 distributed lock 기반 시스템으로 동작하고있는 시스템이 있다고 가정해보자.

redis master slave 복제 방식은 기본적으로 비동기로 동작해서 이로인해 다음과 같은 정합성 오류가 발생가능한데

1. client 1이 마스터 노드에 접속해 자원에 대한 락을 취득
2. 마스터 노드는 클라이언트에게 성공을 응답 but 해당 락 정보를 슬레이브 노드에 전달하기 직전에 다운되어버림
3. 레디스 센티널이나 클러스터가 이를 감지하고 failover
4. 새로운 마스터는 이전 마스터로부터 락 정보를 못받아 락이 없음
5. 이때 client2가 새로운 마스터에 동일한 자원에 대한 락을 요청해 발급 결국 double lock

### 1. Redlock

이 문제가 발생하는 이유는 마스터가 한대라서인데, 단일 마스터 장애가 전체 시스템 장애로 이어지지 않게

5개 이상의 독립적인 마스터로 사용하는 방식이 레드락 방식이다.

클라이언트가 모든 마스터 노드에 락 취득을 요청하고 그중 과반수 (n / 2 + 1) 이상의 노드에서 락 취득에 성공하고 소요된 시간이 락의 유효시간보다 짧을때만 최종적으로 락을 획득한걸로 간주한다

특정 노드 하나가 다운되어도 나머지 과반수 노드들이 락 정보를 유지하고 있으므로 데이터 유실에 대해 매우 안전하지만

단점은 여러 노드에 네트워크 요청을 보내야해서 비용이 비싸고 지연시간이 발생하며 인프라 환결설정이 복잡할수있다.

```java
// 독립적인 3개의 마스터 노드 설정 (권장 5개)
RLock lock1 = redissonClient1.getLock("resource_lock");
RLock lock2 = redissonClient2.getLock("resource_lock");
RLock lock3 = redissonClient3.getLock("resource_lock");

// Redlock 인터페이스를 통해 여러 락을 하나로 관리
RedissonMultiLock lock = new RedissonMultiLock(lock1, lock2, lock3);

try {
    // 10초 동안 대기하며 60초 동안 락을 점유 (과반수 성공 확인)
    boolean res = lock.tryLock(10, 60, TimeUnit.SECONDS);
    if (res) {
        // 비즈니스 로직 수행
    }
} catch (InterruptedException e) {
    Thread.currentThread().interrupt();
} finally {
    // 모든 마스터 노드에 해제 요청
    lock.unlock();
}
```

<br>

### WAIT

레디스는 master slave간에 불일치 문제를 wait으로 해결할수있는데 쉽게말하면 복제를 동기 방식처럼 동작시키는거다.

SET으로 락을 취득했다면 WAIT을 통해 설정한 수의 슬레이브 노드에 데이터가 실제 복제될때까지 마스터가 대기한다.

복잡한 알고리즘 없이 유실 가능성을 획기적으로 낮출 수 있지만 슬레이브가 응답할때까지 마스터는 대기해야하므로 레디스의 고성능 장점을 일부 포기해야한다.

네트워크 장애 등으로 복제가 지연되면 응답속도가 줄어드니

```java
RedisCommands<String, String> commands = connection.sync();

// 1. NX(존재하지 않을 때만), PX(밀리초 단위 만료시간) 옵션으로 락 취득
SetArgs setArgs = SetArgs.Builder.nx().px(30000);
String result = commands.set("lock:resource", "client_id_1", setArgs);

if ("OK".equals(result)) {
    // 2. WAIT 1 1000: 최소 1개의 슬레이브에 복제될 때까지 최대 1000ms 동안 대기
    // dispatch를 통해 직접 Redis 커맨드 실행 가능
    Long syncedSlaves = commands.dispatch(CommandType.WAIT, new IntegerOutput(codec), 
                                          new CommandArgs<>(codec).add(1).add(1000));
    
    if (syncedSlaves != null && syncedSlaves >= 1) {
        // 복제가 확인되었으므로 안전하게 로직 수행
        doBusinessLogic();
    } else {
        // 복제 확인 실패 시 락을 즉시 해제하고 실패 처리
        commands.del("lock:resource");
    }
}
```

<br>

### Fencing Token

애플리케이션에서 검증하는 방법이 있을 수 있는데 분산 시스템 전문가 마틴 클레판이 제안하는 방식이다

락 자체의 불완전함을 인정하고 최종 저장소 등에서 최종 검증을 수행하는 견고한 방법이다

1. 클라이언트가 락을 취득할때 단조 증가하는 토큰을 발급받는다 1 2 3 4
2. 클라이언트가 공유자원 db에 데이터를 쓸 때 토큰을 함께 보낸다
3. 저장소는 현재 기록된 토큰보다 낮은 번호의 요청이 들어오면 거부한다
4. 이는 마스터가 교체되어 이전 클라이언트의 락이 더이상 유효하지 않음을 의미한다

장점은 마스터가 교체되어 락이 중복 취득되어도 막을 수 있다가 있고 최종 데이터 정합성을 지킬수있지만

어찌되었든 db level에 write를 하는지라 성능은 좀 떨어질 수 있다.

```java
// 1. Redis에서 락과 함께 토큰(예: 34)을 얻었다고 가정
val currentToken: Long = 34

// 2. DB 업데이트 시 본인의 토큰이 최신인지 확인하며 실행
@Repository
interface ResourceRepository : JpaRepository<ResourceEntity, Long> {
    @Modifying
    @Query("""
        UPDATE ResourceEntity r 
        SET r.data = :newData, r.lastToken = :token 
        WHERE r.id = :id AND r.lastToken < :token
    """)
    fun updateWithFencing(id: Long, newData: String, token: Long): Int
}

// 서비스 레이어
val updatedRows = resourceRepository.updateWithFencing(resourceId, "value", currentToken)
if (updatedRows == 0) {
    // 본인보다 높은 토큰(예: 35)이 이미 처리됨 -> 무효한 작업으로 간주
    throw StaleLockException("이미 만료된 락 토큰입니다.")
}
```

<br>

### 일관성 우선 시스템

결과적으로 분산락의 신뢰성이나 절대적인 전략이 중요하다면 위의 셋 방법 모두 성능을 희생하고 일관성을 얻는것

CAP이론에서의 C/A를 조정하는 방식이다. CP(Consistency Partiton Toleration, 일관성 우선)를 보장하는 zookeeper, etcd를 고려해 C를 더욱 신경써볼수있고 이들은

quorum write(합의 알고리즘 기반 쓰기)라는 리더 노드에 쓰기 요청이 오면 과반수 노드에 데이터가 기록되어야만 성공을 응답하고 리더가 다운되어도 나머지 노드 중 하나는 반드시 최신 락 정보를 가지고있으며

강력한 리더 선출 메커니즘으로 Raft, ZAB으로 최신 데이터 노드를 선출하게 하며 락 정보가 유실된 노드가 리더가 되는 사고는 발생안한다.

세션과 임시노드로 클라이언트 서버간의 tcp세션이 끊기면 락이 즉시 해제되도록 설계되었으며 이 과정이 전체 노드에 일관되게 처리되기도 한다. 물론 성능은 redis보다 훨 떨어짐.

정답은 없으며 비즈니스 로직에 멱등성이 확보되어있다면 레드락쓰고 데이터 정합성이 매우 중요하다면 펜싱토큰 방식을 써도 된다 wait도 괜찮고. 어찌되었건 db lock for update보단 낫지않을까