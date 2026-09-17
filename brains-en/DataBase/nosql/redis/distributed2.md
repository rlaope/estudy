# Redis Distributed Lock, Example Spring Boot

A distributed lock is a method to control concurrency even in distributed server or database environments. This post will be written under the assumption that [[Redis]] has been chosen as the distributed lock solution.

It's not strictly necessary to use Redis; it can also be implemented using named locks in MySQL. However, the reason for using Redis is that, fundamentally, Redis, which uses memory, can acquire and release locks faster than disk-based databases. Therefore, we aim to control concurrency with Redis.

### Lettuce vs Redisson

Lettuce and Redisson are clients available for implementing distributed locks in Redis.

Implementing with Lettuce has several drawbacks, primarily that it always uses spin locks. To acquire a lock with a spin lock, it continuously sends `SETNX` commands to Redis. This puts a significant load on Redis.

Redisson, on the other hand, uses a pub/sub mechanism. When a lock is released, subscribed clients receive a signal that the lock has been released and then attempt to acquire it.

When using Lettuce, there's no built-in timeout implementation, which can lead to issues like locks never being released or clients entering an infinite loop if they fail to acquire a lock. To resolve such problems, timeouts must be implemented at the application level. Redisson, however, supports settings like timeouts through its dedicated Lock interface, making it convenient and safe to use.

<br>

### Example

```java
 implementation 'org.redisson:redisson-spring-boot-starter:3.30.0'
```

Add the dependency. Then, complete the basic setup through the properties and configuration defined for connecting to Redis.

```java
@ConfigurationProperties(prefix = "spring.data.redis")
public record RedisProperties(
    String host,
    int port
) {

}
```

```java
@Configuration
@RequiredArgsConstructor
public class RedissonConfig {

    private static final String REDISSON_HOST_PREFIX = "redis://";
    private static final String DIVISION = ":";
    private final RedisProperties redisProperties;

    @Bean
    public RedissonClient redissonClient() {
        Config config = new Config();
        config.useSingleServer().setAddress(
            REDISSON_HOST_PREFIX + redisProperties.host() + DIVISION + redisProperties.port());
        return Redisson.create(config);
    }
}
```

Next, add an annotation above the method where the Redis distributed lock will be applied. Since it will be handled with AOP, create a custom annotation and write an aspect that reads this annotation to add additional functionality.

```java
@Target({ElementType.METHOD, ElementType.TYPE})
@Retention(RetentionPolicy.RUNTIME)
@Documented
public @interface RedissonLock {

    String value();

    long waitTime() default 5000L;

    long leaseTime() default 3000L;
}
```

```java
@Slf4j
@Aspect
@Component
@RequiredArgsConstructor
public class RedissonLockAspect {

    private static final String DIVISION = ":";
    private final RedissonClient redissonClient;

    @Around("@annotation(RedissonLock)")
    public Object redissonLock(ProceedingJoinPoint joinPoint) throws Throwable {
        MethodSignature signature = (MethodSignature) joinPoint.getSignature();
        Method method = signature.getMethod();
        RedissonLock redissonLock = method.getAnnotation(RedissonLock.class);

        String lockKey =
            method.getName() + DIVISION + RedisLockSpELParser.getLockKey(signature.getParameterNames(),
                joinPoint.getArgs(), redissonLock.value());

        long waitTime = redissonLock.waitTime();
        long leaseTime = redissonLock.leaseTime();

        RLock lock = redissonClient.getLock(lockKey);
        boolean isLocked = false;

        try {
            isLocked = lock.tryLock(waitTime, leaseTime, MILLISECONDS);
            if (isLocked) {
                log.info("락을 얻는데 성공하였습니다. (락 키 : {})", lockKey);
                return joinPoint.proceed();
            } else {
                log.error("락을 얻는데 실패하였습니다. (락 키 : {})", lockKey);
                throw new RedisLockException(ErrorCode.REDIS_FAILED_GET_LOCK_ERROR);
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            log.error("락을 얻는데 인터럽트가 발생하였습니다. (락 키 : {})", lockKey);
            throw new RedisLockException(ErrorCode.REDIS_INTERRUPT_ERROR);
        } finally {
            if (isLocked) {
                lock.unlock();
                log.info("락을 해제하는데 성공하였습니다. (락 키 : {})", lockKey);
            }
        }
    }

}
```

```java
public final class RedisLockSpELParser {

    public static Object getLockKey(String[] parameterNames, Object[] args, String key) {
        SpelExpressionParser parser = new SpelExpressionParser();
        StandardEvaluationContext context = new StandardEvaluationContext();

        for (int i = 0; i < parameterNames.length; i++) {
            context.setVariable(parameterNames[i], args[i]);
        }

        return parser.parseExpression(key).getValue(context, Object.class);
    }

}
```

For the `acceptGroupInvite` method, only `@RedissonLock(value = "#groupId")` was added. This means the timeout settings will use the default values we defined earlier.

A point to note here is that when using Redis distributed locks, **the lock must always be released after the transaction.**

The reason is that if client 1 holds a lock and client 2 is waiting to acquire it, client 2 could acquire the lock before client 1's transaction commits, potentially breaking data consistency. -> If you are managing transactions via `transactionTemplate`, the lock will be released after the block finishes, the commit occurs, and data is returned.

### Test

I will test this by spinning up a Redis container.

```java
testImplementation 'org.testcontainers:testcontainers:1.19.1'
testImplementation 'org.testcontainers:junit-jupiter:1.19.1'
testImplementation 'com.redis:testcontainers-redis:2.2.2'
```

```java
public abstract class RedissonTestContainer {

    private static final String REDIS_IMAGE = "redis:alpine";
    private static final int REDIS_PORT = 6379;
    private static final RedisContainer REDIS_CONTAINER;

    static {
        REDIS_CONTAINER = new RedisContainer(
            DockerImageName.parse(REDIS_IMAGE)).withExposedPorts(REDIS_PORT);
        REDIS_CONTAINER.start();
    }

    @DynamicPropertySource
    private static void registerRedisProperties(DynamicPropertyRegistry registry) {
        registry.add("spring.data.redis.host", REDIS_CONTAINER::getHost);
        registry.add("spring.data.redis.port", () -> REDIS_CONTAINER.getMappedPort(REDIS_PORT)
            .toString());
    }
}
```

```java
@DataRedisTest
public abstract class RedissonTest extends RedissonTestContainer {

}
```

```java
@TestConstructor(autowireMode = AutowireMode.ALL)
class RedisConcurrencyTest extends RedissonTest {

    private static final int MAX_THREADS_COUNT = 10;

    private final MemberRepository memberRepository = mock(MemberRepository.class);
    private final GroupRepository groupRepository = mock(GroupRepository.class);
    private final MemberGroupRepository memberGroupRepository = mock(MemberGroupRepository.class);
    private final GroupInviteRepository groupInviteRepository = mock(GroupInviteRepository.class);
    private final SocialNotificationManager socialNotificationManager = mock(
        SocialNotificationManager.class);

    private final MemberGroupCommandService groupMemberCommandService = spy(
        new MemberGroupCommandService(
            memberRepository,
            groupRepository,
            memberGroupRepository,
            groupInviteRepository,
            TestTransactionTemplate.spied(),
            socialNotificationManager)
    );

    private final MemberGroupFacade memberGroupFacade = spy(
        new MemberGroupFacade(groupMemberCommandService, socialNotificationManager)
    );


    @Test
    void 사용자는_그룹_초대_요청을_수락할_때_레디스_분산락을_통해_동기적으로_처리한다() throws InterruptedException {
        //given
        ExecutorService executorService = Executors.newFixedThreadPool(MAX_THREADS_COUNT);
        CountDownLatch latch = new CountDownLatch(MAX_THREADS_COUNT);
        AtomicInteger countCheck = new AtomicInteger(MAX_THREADS_COUNT);

        Long memberId = 1L;
        Long groupId = 1L;

        Member groupMember = MemberFixture.member(1);

        given(groupRepository.getTotalGroupMemberCount(groupId))
            .willReturn(Optional.of(10L));
        given(memberRepository.findMemberById(memberId))
            .willReturn(Optional.of(groupMember));
        given(groupRepository.findGroupById(groupId)).willReturn(
            Optional.of(GroupFixture.group()));
        given(memberGroupRepository.findGroupOwnerId(groupId))
            .willReturn(Optional.of(2L));

        given(groupInviteRepository.deleteGroupInviteByGroupIdAndGroupOwnerIdAndGroupMemberId(
            groupId, 2L, memberId)).willReturn(1);

        //when
        for (int i = 0; i < MAX_THREADS_COUNT; i++) {
            executorService.submit(() -> {
                try {
                    memberGroupFacade.acceptGroupInvite(memberId, groupId);
                } finally {
                    latch.countDown();
                    int expectedCount = countCheck.decrementAndGet();
                    assertThat(expectedCount).isEqualTo(latch.getCount());
                }
            });
        }

        latch.await();
        executorService.shutdown();

        //then
        verify(groupInviteRepository,
            times(MAX_THREADS_COUNT)).deleteGroupInviteByGroupIdAndGroupOwnerIdAndGroupMemberId(
            anyLong(), anyLong(), anyLong());
        verify(memberGroupRepository, times(MAX_THREADS_COUNT)).save(any());
    }
```

Thus, distributed locks can be applied with Redis to solve concurrency issues, or pessimistic and optimistic locks can be used. In Java, thread locks can also be implemented with `ReentrantLock` and `Semaphore`. If there's only one application instance, these methods might be more suitable. It could be different if there are multiple instances.

<br>

### Transaction Handling in Redis Distributed Locks

Acquiring and releasing locks via Redis distributed locks is good. However, what if the lock is released before the transaction commits, even though it should be processed together with the transaction? Or what if the transaction commits but the lock isn't released? This could lead to critical data consistency issues.

To prevent **lost updates**, several solutions exist:
1. Use atomic operations
2. Explicit locking
3. Automatic detection of lost updates
4. Compare-and-set operations

Explicit locking is very expensive for transactions that update multiple tables, as it involves locking several tables. Atomic operations and automatic detection of lost updates are DBMS-dependent and thus don't fit well with ORMs.

Therefore, the Compare-and-set (CAS) approach seems promising. In JPA, CAS operations can be easily performed using `@OptimisticLocking`. This involves versioning each transaction and comparing versions before committing the transaction.

One might think, **"Then why not just use optimistic locking supported only at the DB level, without distributed locks?"** However, without distributed locks, concurrent transactions would fail without waiting (because there's no lock). Implementing logic to compare versions and roll back transactions, or retry logic, at the code level would increase code complexity.

Furthermore, the transaction consistency issues mentioned above are not that frequent when actually using Redis distributed locks. Therefore, lock management can be handled by Redis, and versioning can be applied only when a data update loss issue occurs.
