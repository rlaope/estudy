# 캐시 스템피드 현상 제어 (Distributed Lock, PER)

캐시 스템피드는 대규모 트래픽이 발생하는 상황에서 특정 데이터의 캐시 만료시간 TTL이 도래했을 때, 수 많은 동시 요청이 캐시 미스를 겪고 일제히 원본 데이터베이스로 데이터를 조회하러 가는 현상을 의미한다.

**분산락** Distributed Lock은 다수의 서버 또는 프로세스가 공유 자원 여기서는 디비 조회 및 캐시 갱신 권한에 접근할 때, 동시성 문제를 방지하기 위한 단 하나의 프로세스만 접근을 허용하도록 제어하는 동기화 메커니즘이고 주로 레디스 단일 스레드 특성과 원자적 연산을 활용해 구현한다.

여기서 분산락을 언급한 이유는 캐시 스탬피드로 인해 db가 죽는것을 막아주는 방패막이 락이다. 수만건의 요청이 몰릴때 캐시가 만료되는 순간을 비교해보자.

1. 락이 없을때
   1. 캐시 만료시점에 1만개의 요청이 들어온다고 가정
   2. 1만개의 스레드가 모두 캐시가 비어있는걸 확인
   3. 1만개의 스레드가 일제히 db로 똑같은 데이터 조회 쿼리를 사용. db는 동시에 커넥션과 cpu 부하를 견디지 못하고 뻗는다.
2. 분산락을 적용하면
   1. 동일하게 만료시점에 1만개의 요청
   2. 캐시가 비어있는걸보고 디비로 가려하지만 단 1개의 스레드만 접근 권한 획득
   3. 락을 획득한 1개의 스레드만 디비에 가고 나머지 9999개는 잠시 대기한다. db락으로 무겁진 않고 디비 자체보단 서버 자체에 커넥션을 더 오래들고있으므로서 서버 리소스를 희생하는 결정을 한것.

### DB 장애로 이어지는 구체적 상황

대뮤고 이벤트 페이지가 오후 12시에 오픈된다고 가정해보겠다.

1. **트래픽 집중 및 캐시 만료**: 오픈 직후 초당 50,000건의 요청이 인입되며, 메인 페이지 데이터를 제공하는 캐시의 TTL이 12시 5분 00초에 만료되었다.
2. **동시 캐시 미스 발생**: 12시 05분 00초에 인입된 수백~수천개의 요청 스레드가 동시에 캐시 저장소를 확인하고 데이터가 없음을 확인한다.
3. **db 커넥션 폭증**: 캐시 미스를 겪은 모든 스레드가 일제히 rdb로 무거운 쿼리를 전송한다.
4. **장애 전파 Cascading Failure**: RDB는 갑작스러운 수 천개의 동시 쿼리를 처리하지 못하고 cpu 사용률이 100%로 치솟으며 db 커넥션풀이 고갈된다. 결국 디비는 응답 불능 상태에 빠지고 이를 기다리던 애플리케이션 서버들의 스레드까지 대기 상태에 빠지면서 서비스 전체가 마비된다.

보통 구현할땐 redission이나 lettuce를 kotlin 환경에서 많이 쓴다. 레디스 클라이언트 라이브러리들이고 redission이 스핀락 방식이 아니라 pub sub 기반이라 부하가 덜있기도하고 인터페이스 제공도 잘 되어있어서 이걸 쓰는데

```kt
import org.redisson.api.RedissonClient
import java.util.concurrent.TimeUnit

class EventService(
    private val redissonClient: RedissonClient,
    private val eventRepository: EventRepository,
    private val cacheManager: CacheManager
) {
    fun getEventData(eventId: String): EventData? {
        val cacheKey = "event:$eventId"
        
        // 1. 캐시 조회 (Cache Hit)
        val cachedData = cacheManager.get(cacheKey)
        if (cachedData != null) return cachedData

        val lockKey = "lock:event:$eventId"
        val lock = redissonClient.getLock(lockKey)

        try {
            // 2. 락 획득 시도 (대기 시간 3초, 락 점유 시간 2초)
            val isLocked = lock.tryLock(3, 2, TimeUnit.SECONDS)
            if (isLocked) {
                // 3. Double Check: 락을 대기하던 스레드가 락을 획득한 경우, 
                // 앞선 스레드가 이미 캐시를 갱신했을 수 있으므로 다시 한번 확인합니다.
                val recheckedData = cacheManager.get(cacheKey)
                if (recheckedData != null) return recheckedData

                // 4. DB 조회 및 캐시 갱신 (단 하나의 스레드만 실행됨)
                val dbData = eventRepository.findById(eventId)
                if (dbData != null) {
                    cacheManager.set(cacheKey, dbData, 60, TimeUnit.SECONDS)
                }
                return dbData
            } else {
                // 락 획득 실패 시: 너무 많은 대기자가 있거나 락 타임아웃 발생
                // Fallback 전략 실행 (예: 예전 캐시 데이터 반환 또는 사용자에게 잠시 후 시도 안내)
                throw RuntimeException("현재 접속자가 많아 처리가 지연되고 있습니다.")
            }
        } finally {
            // 5. 락 해제 (자신이 획득한 락인지 확인 후 해제)
            if (lock.isLocked && lock.isHeldByCurrentThread) {
                lock.unlock()
            }
        }
    }
}
```

위와 같은 흐름으로 구현하고 redis-cli로 모니터링도 해보면

- `INFO memory`: Redis의 메모리 사용량, 파편화 비율, 그리고 캐시 공간 부족으로 인해 데이터가 방출된 횟수 `evicted_keys`를 확인해 인프라 용량으 적절성 확인
- `SLOWLOG get 10`: 실행 시간이 설정된 임계치를 초과한 쿼리를 조회한다. redis는 싱글스레드라 느린 쿼리 하나가 전체 시스템에 영향을 끼칠 수 있다.
- `MONITOR`: 서버에서는 처리되는 모든 명령을 실시간으로 스트리밍한다. **주의: 운영 환경에서 실행시 성능을 최대 50%까지 저하시킬 수 있어 장애 디버깅 등 극히 제한적인 상황이나 개발, 스테이징 환경에서만 짧게 사용하자.**

### 트레이드오프와 Fallback 전략

분산 락을 도입해서 캐시 스템피드로 인한 db 다운은 막을 수 있지만 새로운 고려사항도 있다.

- **레이턴시 트레이드오프**: 락을 획득하기 위한 redis와의 네트워크 통신 비용이 추가된다. 또한, 락을 얻지 못하도 대기하는 스레드들은 그 시간만큼 응답 지연이 발생하며 이는 톰캣등의 워커 스레드 고갈로 이루어질 수 있다. 적절한 waitTime이 필수
- **Redis 장애시 Fallback**: 분산락을 관리하는 레디스 자체의 장애가 날경우도 생각해야하는데, 서버 메모리 caffiene cache에 짧은 TTL에 로컬 캐시를 두어 redis 의존도를 낮추거나, circuit breaker & stale data 같은 redis가 응답하지 않으면 섴시열고 db쿼리 대신 비록 만료되었더라도 기존의 데이터를 반환해 시스템 생존을 우선시하거나 **PER** 분산락의 지연시간을 피하기위해 TTL이 완전히 끝나기전에 임의의 확률로 특정 요청이 백그라운드에서 캐시를 미리 갱신하게 만드는 기법도 고려할 수 있다.


### PER

분산락으로 스템피드를 막는것은 한계점이 있다.(대기 스레드 레이턴시 증가)

이를 보완하기위한 대안으로 PER이 잇는데 PER은 Probabilistic Early Recomputation으로 **락을 걸어 다른 스레드를 대기시키는 대신, 캐시가 만료되기 직전에 접근하는 스레드중 무작위 한명에게 선제적으로 캐시 갱신 임무를 부여하자**라는 아이디어에서 출발한 락 프리 알고리즘이다.

이론적으로 Varnish, Redis 환경에서 주로 구현되는 XFetch 알고리즘의 핵심 수식은 아래와 같다. 

$$t \ge t_{exp} - \Delta \cdot \beta \cdot \log(rand())$$

- $t$는 현재 시간 current time
- $t_{exp}$는 캐시의 실제 만료 시간
- $\Delta$는 db에서 데이터를 조회하여 캐시를 다시 계산하는데 걸리는 시간
- $\beta$는 확률을 조정하는 가중치 난수
- $rand()$는 0과 1사이의 난수

현재 시간 t가 만료 시간에 가까워질수록 그리고 쿼리에 걸리는 시간 델타가 길수록 수식이 참이 될 확률이 비약적으로 상승한다. 즉 캐시가 완전히 만료되기전에 한명은 백그라운드에서 조용히 db에 다녀와 ttl을 연장시켜놓기 때문에 대규모 캐시 미스와 스탬피드가 원천 차단된다.

여기서 깨알 궁금한점으로 TTL이 지나면 데이터가 사라진게 아닌가? 라는 의문을 가질수도있다. 캐시 미스가 낫다는건, 데이터가 없다는걸로도 해석할 수 있고 ttl이 만료되었다는걸로도 해석할 수 있으니까.

근데 결론은 PER은 TTL이 만료된 후에 복구하는 것이 아니라 만료되기 직전에 선제적으로 캐시를 갱신하는 방법으로 redis 기본 기능에만 의존해 ttl이 지나는 순간 데이터가 삭제되면 이 알고리즘은 성립할 수 없다.

그래서 보통 TTL을 논리적 TTL과 물리적 TTL로 분리하는 아키텍처를 사용하기도한다. 

#### 물리적 만료와 논리적 만료

PER을 구현할 때 일방적인 방식처럼 단순한 값을 캐시에 넣는것이 아니라 데이터의 메타정보를 포함한 객체 자체를 캐싱한다.

- **논리적 TTL**: 애플리케이션 레이어에서 이 데이터는 이 시점까지만 최신이다라고 간주하는 시간이고 캐시 페이로드 안에 값으로 저장한다.
- **물리적 TTL**: Redis 시스템에서 실제로 데이터 메모리를 회수하고 삭제하는 시간이다 논리적 TTL보다 훨씬 길게 설정하여 여유 공간을 둔다.

```json
{
  "data": { "eventName": "초특가 할인", "items": [...] },
  "logical_expire_at": 1711680000,  // 논리적 만료 시점 (예: 5분 뒤)
  "delta": 2.5 // DB 조회에 걸리는 예상 시간 (초)
}
```

그래서 실제 시나리오를 알아보자.

만약 논리적 만료시간이 오우 12시 5분 00초고 물리적 만료는 12시 10분이라 쳤을때

- **12시 04분 58초 논리적 만료 2초전**
  - 요청이 들어오고 애플리케이션은 데이터를 꺼내서 `logical_expire_at`을 확인한다.
  - 이때 PER 확률 공식이 작동한다 만료 시간에 가까워졌으므로 이 요청을 처리하는 스레드는 약 10% 확률로 당첨되고
  - 당첨된 스레드는 기존 캐시 데이터를 사용자에게 즉시 반환해 지연을 없앤다
  - 동시에 **백그라운드 비동기 스레드를 띄워 db에서 최신 데이터를 조회**하고 redis값을 덮어씌우며 논리적 만료 시간도 연장시킨다.
- **12시 05분 1초 논리적 만료 1초후 만약 갱신이 안되었다면**
  - 트래픽이 적거나 운이 나빠 앞선 단계에서 아무도 PER 공식에 당첨되지 않았다면
  - 이때 새로운 요청이 들어와도 캐시미스가 발생 안한다. 물리적 만료 12시 10분이 아직 남아있어서다.
  - 애플리케이션은 캐시를 읽고 논리적 시간이 지났음을 인지하며 이때는 100% 확률로 갱신 트리거가 발동하여 하나의 스레드가 디비를 조회하러간다
  - **중요**: db를 다녀오는동안 인입되는 다른 모든 요청 스레드들은 빈 캐시를 보고 튕기거나 락 대기를 하는것이 아닌 **이미 만료된 물리적으로만 남아있는 기존 캐시 데이터를** 일단 응답 받아 정상적으로 화면을 렌더링 하는것이다. 고객경험을 낮추고 시스템 생존률을 높이는 선택인것

### 데이터 패턴 및 아키텍처 의사결정 기준

캐시 제어 전략은 데이터 생성 비용과 정합성 요구수준에 따라 결정된다.

#### 복잡한 연산 데이터 High Recomputation Cost, Read-Heavy

- 통계 랭킹이나 메인화면 추천 상품 목록등 db에서 수많은 테이블들을 join하거나 집계해서 쿼리 실행이 1~3초이상 소요되는 데이터
- **결정:** 적절한 의사결정은 PER 또는 비동기 갱신
- **이유:** 이런 데이터에 분산락을 걸면 락을 획득하지 못한 수만개의 요청에 대한 스레드가 1~3초동안 락해제를 기다려야하고 이는 즉각적인 was 풀 고갈로 이어지며 per를 사용하면 캐시가 만료되기전 사용자에게 구버전 데이터를 빠르게 1ms 이내에 내려주고 백그라운드에서 여유롭게 무거운 쿼리를 실행해 캐시를 교체할 수 있음.

#### 절대적 정합성 데이터 Strict Consistency, Transactional

- 선착순 쿠폰 잔여 수량, 결제/포이늩 잔액, 한정판 상품 제고
- **결정:** 분산락 redlock 정도만
- **이유:** 이 패턴에서는 0.1초전의 구버전 데이터를 유저에게 보여주면 치명적인 비즈니스 장애 초과발급, 초과결제가 발생하고 PER의 선제적 갱신이나 구버전 데이터 반환은 절대 허용되지 않는다. 반드시 락을 통해 단 하나의 진실 공급원 SSOT를 유지하고 대기하지 못한 트래픽은 빠르게 실패처리해 현재 요청이 많다는 메시지를 띄우는것이 정석이다.

#### 예측 가능한 트래픽 스파이크 Predictable Spike

- 매일 자정에 오픈되는 타임 특가 이벤트나 정해진 시간에 업데이트 되는 일일 차트
- **결정:** Pre-warming 사전 적재
- **이유:** 만료와 동시에 트래픽이 몰릴것으로 100% 예상되면 PER이나 락에 의존하기 보다 이벤트 오픈 10분전 11시 50분에 batch나 worker 프로세스가 db조회를 해둬 미리 ttl이 없는 상태로 캐시를 채워둔다 미리 밀어 넣는 warming 방식임

이처럼 데이터들마다 정합성이 요구되는 방식이나 혹은 트래픽 처리가 요구되는 특성이 재각각이며 유저별로 고유한 형태라 키가 상대적으로 많고 변경이 많은 것인지, 혹은 글로벌 데이터고 변경이 적은것인지. 혹은 글로벌이면서 많은것인지 이런것들을 재서 적절하게 아키텍처를 결정해야한다.

### Example

데이터 본문과 함께 논리적 만료시간과 갱신 연산에 걸리는 delta를 함께 저장하는 객체다.

```kt
data class PerCachePayload<T>(
    val data: T,
    val logicalExpireAt: Long, // 논리적 만료 시간 (Epoch Milliseconds)
    val deltaMs: Long          // 데이터 연산(DB 조회 등)에 소요된 시간 (Milliseconds)
)
```

PER 캐시 매니저를 개발해두자 캐시를 조회하고 확률에 따라 비동기 갱신을 트리거한다.

```kt
import org.springframework.data.redis.core.RedisTemplate
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
import kotlin.math.ln
import kotlin.random.Random

class PerCacheManager(
    private val redisTemplate: RedisTemplate<String, Any>,
    // 백그라운드 비동기 갱신을 위한 전용 스레드 풀
    private val asyncExecutor: ExecutorService = Executors.newFixedThreadPool(10) 
) {
    // beta: 확률 가중치 (보통 1.0 사용, 높을수록 갱신이 더 일찍 발생)
    private val beta: Double = 1.0 

    fun <T> getWithPer(
        cacheKey: String,
        logicalTtlMs: Long,
        physicalTtlMs: Long,
        dbQueryBlock: () -> T // 캐시 미스 또는 갱신 시 실행할 DB 조회 로직
    ): T {
        val cachedValue = redisTemplate.opsForValue().get(cacheKey) as? PerCachePayload<T>

        val currentTime = System.currentTimeMillis()

        // 1. 캐시가 아예 없는 경우 (최초 접근 또는 물리적 만료)
        if (cachedValue == null) {
            return recomputeAndCache(cacheKey, logicalTtlMs, physicalTtlMs, dbQueryBlock).data
        }

        // 2. PER 공식 계산
        // currentTime >= logicalExpireAt - (deltaMs * beta * log(random))
        val randomValue = Random.nextDouble() // 0.0 ~ 1.0
        val earlyRecomputeThreshold = 
            cachedValue.logicalExpireAt - (cachedValue.deltaMs * beta * -ln(randomValue))

        if (currentTime >= earlyRecomputeThreshold) {
            // 3. 확률에 당첨되거나 이미 논리적 만료가 지난 경우: 비동기 갱신 트리거
            asyncExecutor.submit {
                try {
                    recomputeAndCache(cacheKey, logicalTtlMs, physicalTtlMs, dbQueryBlock)
                } catch (e: Exception) {
                    // 비동기 갱신 실패 로깅 (사용자 응답에는 영향을 주지 않음)
                }
            }
        }

        // 4. 기존 데이터(Stale Data 포함) 즉시 반환 -> 사용자 대기 없음(Zero Latency)
        return cachedValue.data
    }

    private fun <T> recomputeAndCache(
        cacheKey: String, 
        logicalTtlMs: Long, 
        physicalTtlMs: Long, 
        dbQueryBlock: () -> T
    ): PerCachePayload<T> {
        val startTime = System.currentTimeMillis()
        
        // 무거운 DB 쿼리 실행
        val freshData = dbQueryBlock() 
        val deltaMs = System.currentTimeMillis() - startTime

        val payload = PerCachePayload(
            data = freshData,
            logicalExpireAt = System.currentTimeMillis() + logicalTtlMs,
            deltaMs = deltaMs
        )

        // 물리적 TTL을 적용하여 Redis에 저장 (단위: 밀리초)
        redisTemplate.opsForValue().set(
            cacheKey, 
            payload, 
            java.time.Duration.ofMillis(physicalTtlMs)
        )

        return payload
    }
}
```

```kt
@Service
class EventService(
    private val perCacheManager: PerCacheManager,
    private val eventRepository: EventRepository
) {
    fun getEventData(eventId: String): EventData {
        return perCacheManager.getWithPer(
            cacheKey = "event:detail:$eventId",
            logicalTtlMs = 5 * 60 * 1000,  // 논리적 TTL: 5분
            physicalTtlMs = 15 * 60 * 1000, // 물리적 TTL: 15분 (논리적 만료 후 10분간 버퍼)
            dbQueryBlock = { 
                eventRepository.findComplexEventData(eventId) // 무거운 쿼리
            }
        )
    }
}
```

위에서도 신경써야할 점이 있는데 비동기 갱신하는 부분도 여러 트래픽이 물리면 동시에 PER 당첨되는 스레드들이 생길수있다. 이를 위해서 백그라운드 스레드 여러개 동시에 db를 조회하는 중복 갱신이 생길수있는데

이를 막기위해 비동기 갱신 불록 내부 또는 앞에 짧은 생명주기를 가진 분산락을 걸어 백그라운드 갱신 작업조차 단 하나의 스레드만 수행하도록 최적화할 수 있기도하다.

```kt
import org.springframework.data.redis.core.RedisTemplate
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
import java.time.Duration
import kotlin.math.ln
import kotlin.random.Random

class PerCacheManager(
    private val redisTemplate: RedisTemplate<String, Any>,
    private val asyncExecutor: ExecutorService = Executors.newFixedThreadPool(10) 
) {
    private val beta: Double = 1.0 

    fun <T> getWithPer(
        cacheKey: String,
        logicalTtlMs: Long,
        physicalTtlMs: Long,
        dbQueryBlock: () -> T
    ): T {
        val cachedValue = redisTemplate.opsForValue().get(cacheKey) as? PerCachePayload<T>
        val currentTime = System.currentTimeMillis()

        // 1. 캐시가 아예 없는 경우 (최초 접근) - 동기 방식으로 1명이 DB 조회
        if (cachedValue == null) {
            return recomputeAndCache(cacheKey, logicalTtlMs, physicalTtlMs, dbQueryBlock).data
        }

        // 2. PER 공식 계산
        val randomValue = Random.nextDouble()
        val earlyRecomputeThreshold = 
            cachedValue.logicalExpireAt - (cachedValue.deltaMs * beta * -ln(randomValue))

        // 3. 갱신 시점이 도래한 경우
        if (currentTime >= earlyRecomputeThreshold) {
            val lockKey = "lock:recompute:$cacheKey"
            
            // 핵심: SETNX를 활용한 비동기 갱신 락 획득 시도 (TTL 5초)
            // 이미 누군가 갱신 중이라면 false를 반환하여 중복 실행을 막습니다.
            val isLocked = redisTemplate.opsForValue()
                .setIfAbsent(lockKey, "LOCKED", Duration.ofSeconds(5)) ?: false

            if (isLocked) {
                asyncExecutor.submit {
                    try {
                        recomputeAndCache(cacheKey, logicalTtlMs, physicalTtlMs, dbQueryBlock)
                    } catch (e: Exception) {
                        // 에러 로깅
                    } finally {
                        // 갱신 완료 후 락 해제 (선택 사항)
                        // 삭제하지 않고 5초 TTL 만료를 기다리게 두면, 
                        // 갱신 직후 발생할 수 있는 찰나의 중복 트리거를 더 안전하게 막을 수 있습니다.
                        redisTemplate.delete(lockKey)
                    }
                }
            }
        }

        // 4. 기존 데이터 즉시 반환 (대기열 없음)
        return cachedValue.data
    }

    // recomputeAndCache 메서드는 이전과 동일합니다.
    // ...
}
```