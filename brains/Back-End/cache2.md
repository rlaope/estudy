# 캐시 시스템 설계 구현

초당 수만명 api 요청에서 급격히 늘어난 트래픽은 서비스 서버에 부하를 줄 수 있고 서버 안정성에 문제가 발생할 수 있다.

서버가 트래픽을 처리하지 못하면 성능이 저하되고 응답시간이 늘어나는등 장애가 발생하는등에 대한 문제가 있을 수 있고

서비스의 품질이 떨어지게 되는것이다. 스레드가 밀리거나 장애가 연쇄되어 발생하거나 등등 

일단 1차로 sever -> db 구조에서 server -> remote cache -> db 구조로 (redis) 변경했다고 했을때 redis 자체에서도 cpu 부하때문에 죽을수도 있다.

메모리 사용량도 늘수있고. 그래서 리모트 캐시가 과부하가 되었을때 next step은 어떻게 해볼수있을까

### Remote Cache & Local Cache

피크 혹은 그냥 트래픽들이 많이 몰려 캐싱하는 데이터들이 많이 몰려 메모리가 부족해지거나

cpu가 커맨드를 느리게 처리하면(redis는 또 single thread 기반임) 레디스 자체가 죽어 디비로 트래픽이 쏠리는 캐시 스템피드가 발생할 수 있고

캐시 스템피드가 발생하면 디비가 죽을수도 있기때문에, 장애 연쇄가 될 수 있는 문제가 있다.

그래서 redis가 캐싱하는 데이터를 서버 로컬캐시에 캐싱해두어 레디스의 부하를 줄여볼 수는 있겠는데.

여기서도 또 고민해야할 포인트들이 많다.

- 어떤 데이터를 로컬 캐시에 둘것인가.
- 어떻게 동기화할 것인가

등등등.. 일단 해당 데이터에 캐시할 유형들을 생각해보면 두가지로 나뉠 수 있는데

전역적으로 사용되는 데이터 혹은 한 유저에 대한 유니크한 값등이 있다.

static data, user data라고 불러보겠다.

#### static data

일단 static data에 대해서는 트래픽이 늘어나면 한 키에 대해서 많은 get연산이 발생하게 될것이다 "product:static" 뭐 이런거라고 치자.

redis에 많은 읽기요청을 보내게 되면 redis cpu 부하가 걸릴거고 이로인해서 redis 커맨드 요청이 밀리고 지연이 발생한다.

즉 이 static data를 보는 모든곳에서 지연이 발생하는 것이다.

일단 static data에 대해서는 모든서버에 같은값으로 로컬 캐싱을 해두어도. user별로 고유한 값이 아니기 때문에

언스티키한 방식으로 유저의 요청이 라우팅되더라도 괜찮다. 일단 로컬캐시를 통해 redis 특정 hot key에 대한 부하는 줄여볼수있다가 1번

> 코드 구현은 아래에서 해보겠다.

#### user data

user data는 유저 별로 캐싱해야할 데이터다. 유저가 늘어나면 메모리 사용량도 당연히 늘고 많은 key가 생길거고 "user:1" .. "user:99999...." 라고 치면

쉽게는 데이터 자체를 압축해서 저장해둔다거나. 이러면 풀때 cpu좀 쓰겠지만 메모리는 아낄수있고, 압축푸는 서버에서 cpu를 사용하는거니 트레이드오프고

redis에 dto같은 데이터를 저장할때 json 깡으로 박기보단 압축을 해두자. (크기에 따라 압축할지말지 여부는 고려해야함)

그리고 과부화로 인해 데이터 처리를 못하게 되면 fallback로직으로 대체값을 일단 내려준다거나 circuit을 열어 차단해두자.

캐시 스탬피드가 어찌되었든 피해야하는 문제기때문에. 차라리 캐시레벨에서 빠르게 실패처리시키고 정상화 시키는게 낫다.

### 구현

정리해보면

- **static data:** 시스템 설정, 공지사항, 카테고리목록 같은 전역적인 값이며 로컬에 주력적으로 넣고 ttl도 상대적으로 길게 세팅해둬도 괜찮다. 이런 값들이 바뀐다면 pub/sub을 통해 무효화 시키고 모든 서버가 동일 값으로 캐싱하므로 효율이 극대화 된다.
- **user data:**: 유저 프로필, 포인트, 개인 설정등과 같은 유저 고유의 값이고 redis 부하 분산을 위한 보조 캐시로 쓰면 좋고 ttl은 매우 짧게두자 변경시 pubsub무효화는 필수고 잦은 요청시 redis io 절약이 된다.

#### Redis Pub/Sub 기반 무효화 설계

모든 서버는 동일한 redis topic을 구독하자.

특정 서버에서 데이터가 수정되면 무효화 메시지를 발행한다.

```
{
  "cacheType": "USER_CACHE or UNIVERSAL_CACHE",
  "key": "user:123",
  "originServer": "server-a-id"
}
```

Server A가 유저 정보를 수정하면 redis에 업데이트 한다.

Server A가 Pub/Sub Topic에 user:123이 무효화 되었다는 메시지를 발행한다.

Server B C D에서 메시지를 수신하고 각자의 caffeine(local cache)를 제거한다.

이후 해당 유저의 요청이 어느 서버로 가든 L1이 비어있으므로 redis에서 최신 데이터를 가져와 다시 L1에 채운다.


캐시 채우기

```java
@Configuration
@EnableCaching
public class CacheConfig {

    @Bean
    public CacheManager cacheManager() {
        SimpleCacheManager cacheManager = new SimpleCacheManager();
        
        // 1. 공통 데이터 캐시 (긴 TTL)
        CaffeineCache staticCache = new CaffeineCache("staticCache",
                Caffeine.newBuilder()
                        .expireAfterWrite(1, TimeUnit.HOURS)
                        .maximumSize(500)
                        .build());

        // 2. 유저 데이터 캐시 (짧은 TTL)
        CaffeineCache userCache = new CaffeineCache("userCache",
                Caffeine.newBuilder()
                        .expireAfterWrite(1, TimeUnit.MINUTES)
                        .maximumSize(10000)
                        .build());

        cacheManager.setCaches(Arrays.asList(staticCache, userCache));
        return cacheManager;
    }
}
```

```java
@Configuration
public class RedisConfig {

    @Bean
    RedisMessageListenerContainer container(RedisConnectionFactory connectionFactory,
                                            MessageListenerAdapter listenerAdapter) {
        RedisMessageListenerContainer container = new RedisMessageListenerContainer();
        container.setConnectionFactory(connectionFactory);
        container.addMessageListener(listenerAdapter, new PatternTopic("cache-invalidation"));
        return container;
    }

    @Bean
    MessageListenerAdapter listenerAdapter(CacheMessageSubscriber subscriber) {
        return new MessageListenerAdapter(subscriber, "onMessage");
    }
}

// 메시지 객체
@Data
@AllArgsConstructor
@NoArgsConstructor
public class CacheMessage implements Serializable {
    private String cacheName;
    private String key;
}
```

해당 레디스 pubsub subscriber를 정의하자

```java
@Component
@RequiredArgsConstructor
public class CacheMessageSubscriber {
    private final CacheManager cacheManager;

    public void onMessage(String message) {
        // 실제로는 JSON 역직렬화 로직 필요
        // ObjectMapper.readValue(message, CacheMessage.class)
        CacheMessage cacheMessage = deserialize(message); 
        
        Cache cache = cacheManager.getCache(cacheMessage.getCacheName());
        if (cache != null) {
            cache.evict(cacheMessage.getKey()); // 로컬 캐시 제거
            System.out.println("L1 캐시 무효화 완료: " + cacheMessage.getKey());
        }
    }
}
```

```java
@Service
@RequiredArgsConstructor
public class UserService {
    private final RedisTemplate<String, Object> redisTemplate;
    private final CacheManager cacheManager;

    @Cacheable(cacheNames = "userCache", key = "#userId")
    public UserProfile getUserProfile(String userId) {
        // L1 Miss 시 실행: Redis(L2)에서 조회하는 로직 (생략)
        return fetchFromRedis(userId);
    }

    public void updateUserProfile(String userId, UserProfile profile) {
        // 1. DB/Redis 업데이트
        updateInRedis(userId, profile);

        // 2. Pub/Sub 메시지 발행 (전체 서버 L1 무효화)
        redisTemplate.convertAndSend("cache-invalidation", new CacheMessage("userSpecificCache", userId));
    }
}
```

추가적으로 캐시 스탬피드를 방지하기 위해서 유저 고유 데이터는 Cacheable을 사용하되 redis 조회시 sync = true 옵션을 주어 여러 스레드가 동시에 레디스에 접근하지 못하도록 lock을 걸수도 있다.

```java
@Cacheable(cacheNames = "userSpecificCache", key = "#userId", sync = true)
```

추가적으로 메시지를 발행한 서버 자신도 구독메시지를 publisher가 subscriber라서 받게되는데 인식하지못하고 무한루프가 돌수도있다. evict는 성능에 큰 영향이 없어 자신이 수행하게 두거나 서버id를 메시지에 포함해 본인일경우 스킵하도록 구현해둘수도 있다.

네트워크 비용 vs 정합성을 고민하는건데 데이터 업데이트가 초당 수만건 발생하는 시스템이면 pub sub 메시지 자체가 네트워크 부하가 될 수 있어서 이 경우 pubsub을 포기하고 L1 Local cache ttl을 매우 짧게 가져가서 자연스레 만료되게 하는것이 유리할수도 있다.

Caffeine의 Weak Key를 사용하면 메모리 압박이 심할경우 weakKeys 옵션 키면 가비지 컬렉션시 캐시가 더 적극적으로 정리되도록 설정할수 있다고한다. 약한참조로 만드는듯?

<br>

일단은 read에 관한 캐싱은 어느정도 본거같고, 전역, 유저 고유별 캐시마다 다르게 설정값을 가져가면서도 변경이 발생하면 redis-pubsub을 통해 일관성을 맞췄다.

그럼 write에서는 어떻게할까? 바로 db에 write하는 연산이 많다면 그것도 그거대로 디비 부하를 주는거라 문제가 될 수 있다.

먼저 캐시 쓰기 전략을 알아보겠다.

- **Write Through:** 캐시, db에 동시에 데이터를 쓰고 장점으로는 데이터 일관성을 보장하지만 단점은 쓰기 지연시간이 증가한다. db부하도 같이감
- **Write-Back(Write-Behind):** redis에 먼저 쓰고 일정 시간/개수만큼 모아서 db에 비동기로 쏜다 db부하가 감소되고 쓰기 성능이 향상되지만 redis 장애시 데이터 유실 가능성이 있다.

db로 바로 쏘면 안된다는 제약 조건을 고려하면 write-back 전략이 어울려 보인다.

비스티키 환경이라고 생각하고 어느 서버에서 쓰기가 발생하더라도 다른 서버들의 L1 캐시가 즉시 무효화 되어야한다.

1. Request 유저가 데이터 변경 요청
2. L2 Write 해당 서버가 redis에 최신 데이터를 저장
3. Local Update 자신의 로컬 캐시를 업데이트
4. Pub/Sub Message로 redis pubsub 을통해 키가 바뀌었으니 다들 지워! 식의 메시지 전파
5. Write-Behind 변경된 키 정보를 redis list, set에 변경 대상으로 등록해둔다
6. DB Write 별도의 batch worker가 redis에서 데이터를 읽어 주기적으로 디비에 대량 업데이트를 해둔다.

user data라고 가정하면 쓰기가 비교적 static보다 빈번하기때문에 redis에 먼저 쓰고 db에 나중에 업데이트하는 방식을 고려하자.

static data면 쓰기가 매우 드물기때문에 유실시 영향이 크므로 이건 그냥 write through로 해두어도 괜찮을것같다. 일단 지금은 그렇다.

```java
@Service
@RequiredArgsConstructor
public class UserCacheService {
    private final RedisTemplate<String, Object> redisTemplate;
    private final CacheManager cacheManager;

    public void updateUserData(String userId, UserProfile profile) {
        // 1. Redis (L2) 업데이트
        String redisKey = "user:" + userId;
        redisTemplate.opsForValue().set(redisKey, profile);

        // 2. Local (L1) 업데이트 (현재 서버)
        cacheManager.getCache("userCache").put(userId, profile);

        // 3. 타 서버 L1 무효화 메시지 발행
        redisTemplate.convertAndSend("cache-invalidation", new CacheMessage("userCache", userId));

        // 4. Write-Behind를 위한 변경 로그 기록 (Set에 저장하여 중복 제거)
        redisTemplate.opsForSet().add("pending-db-updates", userId);
    }
}
```

스케쥴러를 통해 주기적으로 redis에 write해놓았던 데이터를 db에 persist한다.

```java
@Component
@RequiredArgsConstructor
public class WriteBehindWorker {
    private final RedisTemplate<String, Object> redisTemplate;
    private final UserRepository userRepository;

    @Scheduled(fixedDelay = 5000) // 5초마다 DB 업데이트
    public void persistToDb() {
        // 1. 업데이트 대상 키들을 한꺼번에 가져옴
        Set<Object> pendingUserIds = redisTemplate.opsForSet().members("pending-db-updates");
        if (pendingUserIds == null || pendingUserIds.isEmpty()) return;

        // 2. Redis에서 해당 데이터들 Multi-Get (네트워크 최적화)
        List<String> keys = pendingUserIds.stream().map(id -> "user:" + id).toList();
        List<Object> profiles = redisTemplate.opsForValue().multiGet(keys);

        // 3. DB Bulk Save (JPA saveAll 등 사용)
        // userRepository.saveAll(profiles...);
        System.out.println(profiles.size() + "건의 데이터를 DB에 일괄 저장했습니다.");

        // 4. 처리 완료된 키 삭제
        redisTemplate.delete("pending-db-updates");
    }
}
```

결과적으로 redis에 쓰기를 먼저하고 batch가 돌면서 내려주는 방식인건데, 장점은 부하를 덜지만 db에 바로 반영이 되지 않고

redis에 장애가 나면 유실이 될 수 있다는 것이다. cap이론에서 C/A를 조정하는 느낌인데, 성능을 얻을것인지 강한 일관성을 얻을것인지에대해 고민하고 결정해야할듯싶다.

근데 여기서 끝내지 않고 좀 더 문제를 고민해보자. cache에 쓰게된다면 db lock을 따로 잡지 않고 쓰는데, 여기서는 어떻게 동시성 이슈를 해결해볼 수 있을가.

따닥이 온다거나 중복 요청이 온다거나등에 대한 처리가 두번 이루어지면 정합성이 깨질 수 있다.

일단 **분산락**을 잡는 방식인데 여러 서버가 분산되어 있으므로 synchronized는 소용이 없다. redis의 분산락으로 특정 key(userid)에 대해 한 번의 하나의 쓰기 작업만 허용시킨다.

혹은 **Lua 스크립트**를 통해 원자적인 연산을 보장시키자. redis에 데이터를 쓰고 무효화 메시지를 보내고 write-behind 큐에 넣는 과정을 하나의 원자적 단위로 묶는다.

중간에 다른 요청이 끼어들지 못하게 redis내에서 lua 스크립트를 실행한다.

멱등성키를 적용해 방지해도 괜찮다 client가 요청마다 request-id를 보내고 서버는 처리 완료된 id를 짧은시간동안 레디스에 기록해 동일한 요청이 들어오면 무시하게 구현할 수 있다.

```java
@Service
@RequiredArgsConstructor
public class ConcurrentUserCacheService {
    private final RedissonClient redissonClient;
    private final RedisTemplate<String, Object> redisTemplate;
    private final CacheManager cacheManager;

    public void updateUserDataWithLock(String requestId, String userId, UserProfile profile) {
        String lockKey = "lock:user:" + userId;
        RLock lock = redissonClient.getLock(lockKey);

        try {
            // 1. 락 획득 시도 (최대 5초 대기, 10초간 점유)
            if (lock.tryLock(5, 10, TimeUnit.SECONDS)) {
                try {
                    // 2. 멱등성 체크 (중복 요청 방지)
                    String idempotencyKey = "req:" + requestId;
                    Boolean isFirstRequest = redisTemplate.opsForValue()
                            .setIfAbsent(idempotencyKey, "processing", Duration.ofMinutes(5));
                    
                    if (Boolean.FALSE.equals(isFirstRequest)) {
                        return; // 이미 처리된 요청이면 종료
                    }

                    // 3. 비즈니스 로직 및 캐시 업데이트 (원자적 수행 권장)
                    updateCacheAndPublish(userId, profile);
                    
                } finally {
                    lock.unlock(); // 락 해제
                }
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }

    private void updateCacheAndPublish(String userId, UserProfile profile) {
        String redisKey = "user:" + userId;
        
        // Redis 파이프라인이나 Lua 스크립트를 쓰면 더 안전합니다.
        redisTemplate.execute(new SessionCallback<Object>() {
            @Override
            public Object execute(RedisOperations operations) {
                operations.multi(); // 트랜잭션 시작
                operations.opsForValue().set(redisKey, profile);
                operations.opsForSet().add("pending-db-updates", userId);
                operations.convertAndSend("cache-invalidation", 
                    new CacheMessage("userCache", userId));
                return operations.exec();
            }
        });

        // 현재 서버의 L1 캐시도 즉시 업데이트
        cacheManager.getCache("userCache").put(userId, profile);
    }
}
```

1. 따닥 들어오면 redis 락을 먼저 선점한 서버만 로직을 수행하고 뒤늦게 도착한 요청은 락 획득후 idempotencyKey 체크 단계에서 거른다
2. 데이터를 쓰는 도중 다른 서버에서 조회가 오면 쓰기 서버가 아직 pub sub을 쏘기 전이라 로컬캐시의 구버전을 읽을 수 있는데 정합성이 극도로 중요하면 조회시도에서도 락을 걸거나 ttl을 짧게 걸어 무효화를 시켜 L2를 볼 가능성의 확률을 더 높여보자.
3. write behind 큐에 중복 데이터가 들어가면 일단 위 코드에서 `opsForSet().add()` 를통해 Set을 사용했고 중복을 자동으로 제거했다 list는 피하자.