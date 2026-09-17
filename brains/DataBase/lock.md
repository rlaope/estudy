# 비관적 락, 낙관적 락

### 비관적 락

Repeatable Read, Serializable 정도의 격리성 수준을 제공한다.

트랜잭션이 시작될 때 Shared Lock or Exclusive Lock을 걸고 시작한다.

![](https://velog.velcdn.com/images/junsu1222/post/df009d23-1292-4ae8-a678-d12037062cf5/image.png)

JPA에서는 PESSIMISTIC_READ(Read Lock), PESSIMISTIC_WRITE(Write Lock), PESSIMISTIC_FORCE_INCREMENT(버전 정보 강제 증가)를 톨해서 비관적락을 사용할 수 있다.

**비관적 락으로 데드락이 발생하는 경우**
1. t1: x테이블의 1번 데이터 row에 Lock을 건다
2. t2: y테이블의 1번 데이터 row에 Lock을 건다
3. t1: y테이블의 1번 데이터 row에 접근한다 -> t2가 이미 lock을 걸어 대기한다.
4. t2: x테이블의 1번 데이터 row에 접근한다 -> t1이 이미 lock을 걸어 대기한다.


<br>

### 낙관적 락

낙관적 락은 version 등의 구분 컬럼을 이용한 것이고 DB에서 제공해주는 특징을 사용하는 것이 아닌 application 수준에서 잡아주는 lock이다.

![](https://velog.velcdn.com/images/junsu1222/post/595af105-1cc1-4591-bff1-da37989d802e/image.png)

낙관적락이 발생하는 경우 `ObjectOptimisticLockFailureException`이 throw 되고 애플리케이션에서 해당 예외를 핸들링해줘야한다.

JPA에서는 `@Version`이라는 어노테이션을 통해 낙관적 락을 사용할 수 있다. 즉, 낙관적 락은 자원에 락을 걸지 않고, 동시성 이슈가 발생하면 그때 처리하게 된다.

<br>

### 비교

낙관적 락과 비관적 락에 대해서 비교해보겠다. 

낙관적락은 데이터를 업데이트 하기 전에 조회하면서 애플리케이션 수준에서 락을 잡아주기에 성능적으로 비관적락보다 더 좋다. 또한 트랜잭션을 필요로하지 않는다. 그러나 충돌이 났다고 하면, 개발자가 수동으로 롤백처리를 해줘야한다.

비관적 락은 충돌 발생을 미연에 방지하고 데이터의 일관성을 유지한다. 동시 처리 성능 저하가 발생할 수 있어 낙관적 락 보다는 성능이 좋지 않으며, 데드락이 발생할 수 있는 문제가 있다.

**비관적 락을 사용하면 좋은 경우**
- 데이터 무결성이 중요(돈, 송금)
- 데이터 충돌이 많이 발생할 것으로 예상되는 경우

**낙관적 락을 사용하면 좋을 경우**
- 데이터 충돌이 자주 일어나지 않을 것으로 예상되는 경우
- 조회 작업이 많아 동시 접근 성능이 중요할 때


비관적 락을 적용하는 예시 코드를 보여주자면 예를들어 멤버들이 속한 그룹이 있다고 했을 때 멤버의 수를 구하는 비즈니스 로직에서 `groupRepository.getTotalGroupMemberCount(groupId)` 함수를 작성해보겠다. 해당 조회 쿼리에서 비관적 락을 적용할 수 있다. 다른 트랜잭션에서 읽기 부터 막아야하기에 PESSIMISTIC_WRITE를 해야한다.

```java
public Optional<Long> getTotalGroupMemberCount(final Long groupId) {
        return Optional.ofNullable(jpaQueryFactory
            .select(memberGroup.count())
            .from(memberGroup)
            .where(memberGroup.group.id.eq(groupId))
            .setLockMode(LockModeType.PESSIMISTIC_WRITE)
            .fetchOne()
        );
    }
```

그러나 where 조건문에 맞는 행들이 락이 걸리기 때문에, 다른 트랜잭션에서 해당 행들에 읽기, 쓰기 연산이 락이 풀리기 전까지는 수행하지 못하게 된다. 그룹원은 최대 30명 까지라고 가정했을때 30개의 행에 lock이 걸려버리게 되는 것이다. 현재 memberGroup 테이블에 읽기, 쓰기 연산이 있는 로직들이 다른 기능에서도 많이 사용된다면, 이는 동시 처리에 성능에 저하가 발생할 수 있다.

이제 이러한 문제를 해결하기 위해 낙관적 락 이외의 [[분산락]]에 대해서 알아보겠다.