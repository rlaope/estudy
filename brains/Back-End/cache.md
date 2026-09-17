# 캐시 시스템 설계

cc. https://github.com/rlaope/cache-labs 여기에 실습 코드까지 넣어놓음. 해당 글은 개념 정리 및 접근성에 관한 추가 학습들을 진행함

> 로컬 캐시 + Redis 이중 캐시 구조를 설계하며 정리한 내용

---

## 목차

1. [어떤 값을 캐싱할 것인가](#1-어떤-값을-캐싱할-것인가)
2. [어떻게 구현할 것인가](#2-어떻게-구현할-것인가)
3. [캐시 일관성을 어떻게 지킬 것인가](#3-캐시-일관성을-어떻게-지킬-것인가)
4. [분산 환경에서의 캐시 설계](#4-분산-환경에서의-캐시-설계)

---

## 1. 어떤 값을 캐싱할 것인가

### 1.1 로컬 캐시(Local Cache)에 적합한 데이터

로컬 캐시는 애플리케이션 메모리(Heap 등)에 데이터를 직접 저장한다. 네트워크 I/O가 없으므로 속도가 압도적으로 빠르지만, 서버 간 데이터 복제가 어렵다는 특징이 있다.

- **극도로 높은 빈도로 조회되는 'Hot' 데이터**: 모든 요청마다 참조해야 하는 설정값이나 공통 코드 데이터.
- **변경 빈도가 매우 낮은 정적 데이터**: 시스템의 비즈니스 로직에 영향을 주는 정책 설정, 국가 코드, 카테고리 목록 등.
- **서버 간 데이터 불일치가 치명적이지 않은 데이터**: 약간의 시차(TTL 기반)가 발생해도 사용자 경험에 큰 영향을 주지 않는 UI 레이아웃 정보 등.
- **Redis Hotspot 방지용 데이터**: 특정 키에 대한 요청이 너무 많아 Redis 한 대의 CPU 성능을 초과할 때(Cache Stampede), 이를 완화하기 위해 로컬에 2차 캐싱을 수행한다.

> 근데 Redisson 문서 보면 로컬 캐시 쓰면 읽기 성능이 최대 45배까지 빨라진다고 함. 물론 벤더 문서라 좀 과장 섞였을 수 있는데, 네트워크 I/O 없애는 게 얼마나 큰지는 체감됨. "A common misconception is that Redis is always the fastest caching option. In reality, in-process caching is faster than Redis" 라는 말도 있던데 Redis 맹신은 금물인듯 애초에 기술맹신자체가금물이긴함

### 1.2 Redis 캐시에 적합한 데이터

Redis는 분산 환경에서 여러 서버 인스턴스가 공유하는 저장소다. 데이터 정합성을 유지해야 하거나, 로컬 메모리에 담기에는 큰 데이터를 다룰 때 필수적이다.

- **공유 상태(Shared State) 정보**: 로그인 세션, 유저 토큰, 장바구니 정보처럼 사용자가 어떤 서버에 접속하더라도 동일하게 유지되어야 하는 데이터.
- **실시간 카운팅 및 랭킹**: 분산 환경에서 정확한 합산이 필요한 좋아요 수, 실시간 인기 검색어 순위(Sorted Set 활용).
- **DB 부하 경감용 고빈도 쿼리 결과**: 조회 쿼리 비용이 비싸지만 여러 사용자가 공통으로 사용하는 검색 결과나 게시글 상세 정보.
- **분산 락(Distributed Lock) 및 속도 제한(Rate Limiting)**: 여러 노드에서 동시에 접근하는 자원을 보호하거나, API 호출 횟수를 제한하기 위한 지표 데이터.


### 1.3 캐싱하면 안 되는 데이터

캐시는 '성능'을 위해 '정합성'을 희생하거나 '추가 비용'을 지불하는 행위다. 아래의 경우는 캐싱의 득보다 실이 크다.

- **실시간 정합성이 생명인 금융/결제 데이터**: 계좌 잔액이나 결제 상태 등 단 1ms의 불일치도 허용되지 않는 데이터는 항상 DB(Source of Truth)를 직접 참조해야 한다.
- **재사용성이 없는 일회성 데이터**: 단 한 번만 조회되고 다시 쓰이지 않는 데이터(예: 특정 검색 필터 일회용 결과). 캐시 히트율(Hit Ratio)이 낮아 메모리 낭비만 초래한다.
- **보안에 민감한 개인정보(암호화되지 않은 PII)**: 메모리 덤프나 Redis 노출 시 치명적인 개인정보는 캐싱을 피하거나 강력한 암호화가 선행되어야 한다.
- **자주 바뀌는데 읽기 빈도는 낮은 데이터**: 쓰기 작업 시마다 캐시를 갱신(Invalidation)해야 하므로 오버헤드만 발생시키고 읽기 이득은 없다.

> 유명한 격언 중에 "There are only two hard things in Computer Science: cache invalidation and naming things" 라는 게 있음. 농담반 진담반인데, 캐시 안 해도 될 걸 캐시하면 진짜 invalidation 지옥 맛봄. 히트율 낮으면 그냥 안 하는 게 맞다고 봄.

### 1.4 자주 변경되는 데이터 처리 전략

데이터 변경이 잦은 경우, '어느 시점에 캐시를 깨뜨릴 것인가(Invalidation)'와 '어떻게 DB와 동기화할 것인가'가 핵심이다.

**Write-Through / Write-Around / Write-Back:**
- **Write-Through**: DB와 캐시에 동시에 데이터를 쓴다. 정합성은 좋지만 쓰기 지연시간이 증가한다.
- **Write-Back (Write-Behind)**: 캐시에만 먼저 쓰고, 일정 주기나 이벤트에 따라 DB에 비동기로 반영한다. 쓰기 성능은 최고지만 서버 장애 시 데이터 유실 위험이 있다.

> CodeAhoy 글에서 "Write-back gives max performance at the cost of consistency risk, while write-through gives strong consistency at cost of performance. Decide what the priority is." 라고 정리해둠. 결국 트레이드오프인데, 일부 개발자들은 피크 타임 버퍼용으로 Redis에 Write-Back 쓰기도 한다더라. 근데 장애나면 데이터 날아가니까 중요한 데이터엔 비추.

**Cache Invalidation vs Update:**
- 데이터 변경 시 기존 캐시를 **삭제(Evict)** 하는 것이 일반적이다. 수정(Update) 방식은 레이스 컨디션(Race Condition)으로 인해 잘못된 데이터가 캐시에 남을 위험이 크다.

**TTL(Time To Live) 전략:**
- 자주 변경되는 데이터일수록 TTL을 짧게 가져가되, 캐시 유효기간 만료 시점에 대량의 요청이 DB로 몰리는 상황(Cache Stampede)을 방지하기 위해 만료 시간을 무작위(Jitter)로 설정하는 기법을 사용한다.


**CDC(Change Data Capture) 활용:**
- DB의 트랜잭션 로그를 감지하여 메시지 큐(Kafka 등)를 통해 비동기적으로 캐시를 갱신하는 방식으로, 애플리케이션 로직과 캐시 갱신 로직을 분리하여 확장성을 높인다.

### cc

**로컬 캐시 vs Redis 캐시**
- [Redis + Local Cache: Implementation and Best Practices](https://medium.com/@max980203/redis-local-cache-implementation-and-best-practices-f63ddee2654a) - 이중 캐시 구현 실전 가이드
- [Redis Cache vs. In-Memory Cache: When to Use What](https://blog.nashtechglobal.com/redis-cache-vs-in-memory-cache-when-to-use-what/) - 로컬/분산 캐시 선택 기준
- [Distributed Caching - Redis.io](https://redis.io/glossary/distributed-caching/) - Redis 공식 분산 캐시 문서

**캐시 쓰기 전략 (Write-Through / Write-Back / Write-Around)**
- [Caching Strategies and How to Choose the Right One](https://codeahoy.com/2017/08/11/caching-strategies-and-how-to-choose-the-right-one/) - 캐시 전략 비교 분석
- [Cache Invalidation Strategies - Design Gurus](https://www.designgurus.io/blog/cache-invalidation-strategies) - 캐시 무효화 전략 심층 가이드
- [Cache Invalidation and Methods - GeeksforGeeks](https://www.geeksforgeeks.org/system-design/cache-invalidation-and-the-methods-to-invalidate-cache/) - 캐시 무효화 방법론

**Cache Stampede (Thundering Herd)**
- [Thundering Herd / Cache Stampede](https://distributed-computing-musings.com/2021/12/thundering-herd-cache-stampede/) - 문제 정의와 해결책
- [Cache Stampede & The Thundering Herd Problem](https://medium.com/@sonal.sadafal/cache-stampede-the-thundering-herd-problem-d31d579d93fd) - 실전 사례와 대응 패턴
- [Thundering Herd - Ehcache Documentation](https://www.ehcache.org/documentation/2.8/recipes/thunderingherd.html) - BlockingCache를 활용한 해결

---

## 2. 어떻게 구현할 것인가

### 2.1 L1 + L2 이중 캐시 구조 (Multi-level Cache)

단일 캐시의 한계를 극복하기 위해 로컬 캐시(L1)와 분산 캐시(L2)를 혼합하여 사용하는 전략이다.

**구조:**
- **L1 (Local Cache)**: 각 애플리케이션 서버의 메모리(Caffeine, Guava 등). 네트워크 비용이 0에 수렴하여 극도의 성능을 낸다.
- **L2 (Remote Cache)**: Redis와 같은 공용 저장소. 여러 서버가 데이터를 공유하며 데이터 일관성을 유지한다.

**조회 흐름:**
`애플리케이션 → L1 확인 (Hit 시 종료) → L2 확인 (Hit 시 L1에 복사 후 반환) → DB 확인 (Hit 시 L1/L2에 복사 후 반환)`

**동기화 이슈 (The Invalidation Problem):**
- L2의 데이터가 변경되었을 때 각 서버의 L1 데이터를 어떻게 무효화할 것인가가 핵심이다.
- **해결책**: Redis Pub/Sub 기능을 활용하여 특정 키가 변경될 때 모든 서버에 "L1 캐시 삭제" 메시지를 브로드캐스팅하는 구조를 주로 사용한다.

> L1-L2 구조 쓰면 Redis 장애 시에도 L1에서 버틸 수 있어서 가용성 측면에서도 이점이 있음. 다만 Pub/Sub 메시지 유실 가능성이 있어서 L1 TTL은 짧게 가져가는 게 안전함.

### 2.2 Cache-Aside 패턴 (Look-aside)

가장 범용적으로 사용되는 패턴으로, 애플리케이션이 캐시를 직접 관리하며 필요할 때만 데이터를 캐싱하는 방식이다.

**Read 로직:**
1. 캐시에 데이터가 있는지 확인(Hit)
2. 없으면(Miss) DB에서 조회
3. 조회한 데이터를 캐시에 저장 후 반환

**Write 로직 (중요):**
- **Update DB First, then Evict Cache**: DB를 먼저 업데이트하고 캐시를 삭제한다.
- **왜 삭제인가?** 캐시를 새로운 값으로 업데이트하는 방식은 여러 요청이 꼬일 경우(Race Condition) DB와 캐시의 데이터가 달라질 위험이 크다. 반면 삭제 방식은 다음 조회 시 DB에서 최신 데이터를 가져오므로 안전하다.

**특징:**
- 캐시 장애가 발생해도 시스템이 중단되지 않고 DB를 통해 서비스가 가능하다(Graceful Degradation).
- 다만 초기 요청 시 DB 부하가 집중될 수 있다(Cold Start 문제).

> Cache-Aside가 가장 무난한 선택인 이유가 있음. 캐시가 죽어도 서비스가 살아있으니까. Read-Through처럼 캐시가 DB 앞에 서는 구조는 캐시 장애 = 서비스 장애라서 부담이 큼.

### 2.3 TTL(Time To Live) 설정 전략

데이터의 신선도와 시스템 부하 사이의 균형을 맞추는 핵심 전략이다.

**Fixed TTL:**
- 데이터의 성격에 따라 1분, 1시간 등 고정된 시간을 부여한다.

**Jitter (Randomized TTL):**
- **문제**: 수많은 캐시 키의 만료 시간이 동일하면, 특정 시점에 동시에 만료되어 DB로 부하가 몰리는 Cache Stampede 현상이 발생한다.
- **해결**: 설정한 TTL에 5~10% 정도의 무작위 시간(Jitter)을 더해 만료 시점을 분산시킨다.

**Soft TTL vs Hard TTL:**
- **Hard TTL**: 시간이 지나면 데이터 즉시 삭제.
- **Soft TTL**: 만료가 임박했을 때 백그라운드에서 미리 갱신하여 사용자에게 항상 Hit된 데이터를 제공하는 방식(Refresh-ahead).

> TTL 설정은 "너무 길면 stale data, 너무 짧으면 hit rate 저하"의 줄다리기임. 처음엔 보수적으로 짧게 잡고 모니터링하면서 늘려가는 게 안전함. Jitter는 진짜 필수인데, 안 쓰면 새벽 배포 후 아침에 동시 만료돼서 DB 터지는 거 본 적 있음.

### 2.4 캐시 키 설계

캐시 키는 데이터의 식별자이자 관리를 위한 주소다. 명확하고 체계적인 네이밍 컨벤션이 필수적이다.

**계층 구조 사용:**
- `:` 또는 `::` 구분자를 사용하여 계층을 표현한다.
- 형식: `{서비스명}:{도메인}:{식별자}:{속성}`
- 예: `user-service:user:1234:profile`

**버전 관리 (Versioning):**
- 데이터 구조(DTO)가 변경될 경우 구버전 캐시로 인한 역직렬화 에러가 발생할 수 있다.
- 키에 버전을 포함하면 안전하게 배포할 수 있다.
- 예: `v1:user:1234` → `v2:user:1234`

**가독성과 길이의 균형:**
- 너무 길면 메모리를 낭비하고, 너무 짧으면 용도를 알 수 없다.
- 의미 있는 약어를 사용하되 중복되지 않도록 설계한다.

**Value의 크기 고려:**
- 키 설계 시 해당 키에 담길 Value의 크기도 고려해야 한다.
- Redis의 경우 너무 큰 Value(수십 MB)는 성능 저하의 원인이 되므로 리스트나 셋을 적절히 분할해야 한다.

> 키 네이밍 잘못하면 나중에 진짜 고생함. 특히 버전 관리 안 해두면 DTO 바꿀 때마다 역직렬화 터져서 롤백하게 됨. 처음부터 `v1:` 붙이는 습관 들이는 게 좋음. 그리고 Redis에서 `KEYS *` 명령어로 디버깅할 일이 많은데, 계층 구조 잘 잡아두면 `KEYS user-service:*` 이런 식으로 필터링이 쉬워짐.

---

## 3. 캐시 일관성을 어떻게 지킬 것인가

### 3.1 캐시 무효화 전략 (Cache Invalidation)

데이터가 변경되었을 때 캐시를 어떻게 처리할지에 대한 전략이다.

**Delete vs Update:**
- **Update**: DB 수정 시 캐시 값도 수정한다. 하지만 두 개의 요청이 동시에 발생할 경우, DB와 캐시의 최종 값이 달라지는 Race Condition 위험이 크다.
- **Delete (Evict)**: DB 수정 시 캐시를 아예 삭제한다. 다음 조회 시 DB에서 최신 데이터를 가져오므로 훨씬 안전하며, 실무에서 권장되는 방식이다.

**Transactional Messaging:**
- DB 업데이트는 성공했는데 캐시 삭제에 실패하면 정합성이 깨진다.
- 이를 방지하기 위해 DB 트랜잭션 성공 후 메시지 큐(Kafka, RabbitMQ)에 이벤트를 발행하여 확실히 캐시를 삭제하도록 보장한다.

**Double Deletion:**
- 분산 환경에서 발생할 수 있는 찰나의 정합성 오류를 막기 위해, DB 수정 직전에 한 번, 수정 완료 후 약간의 시간 차(Delay)를 두고 다시 한 번 캐시를 삭제하는 기법이다.

> Update 방식이 위험한 이유를 예로 들면: 요청 A가 값을 10으로, 요청 B가 값을 20으로 거의 동시에 수정한다고 치면, DB는 A→B 순서로 커밋돼서 최종값 20인데 캐시는 B→A 순서로 반영돼서 최종값 10이 되는 상황이 생길 수 있음. Delete 방식은 이런 걱정이 없음.

### 3.2 Write-Through vs Write-Behind (Write-Back)

쓰기 요청이 들어왔을 때 캐시와 DB를 어떻게 동기화할 것인가의 문제다.

| 패턴 | 동작 방식 | 장점 | 단점 |
|------|----------|------|------|
| **Write-Through** | DB와 캐시에 동시에 데이터를 씀 | 캐시가 항상 최신 상태를 유지 (강한 일관성) | 쓰기 지연 시간(Latency) 증가 |
| **Write-Behind** | 캐시에 먼저 쓰고, 나중에 DB에 비동기 반영 | 쓰기 성능이 매우 빠름 (대량 쓰기에 유리) | 캐시 장애 시 데이터 유실 위험 |

> Write-Behind는 게임 서버에서 자주 쓰는 패턴임. 유저 행동 로그 같은 건 실시간으로 DB에 안 써도 되니까. 근데 결제 같은 건 절대 Write-Behind 쓰면 안 됨. 서버 죽으면 결제 데이터 날아가는 거니까.

### 3.3 Cache Stampede 방지

대규모 트래픽 환경에서 특정 캐시 키가 만료되는 순간, 수많은 요청이 한꺼번에 DB로 몰려 서버가 마비되는 현상을 방지해야 한다.

**Jitter (Random TTL):**
- 모든 캐시의 만료 시간을 조금씩 다르게 설정하여(예: 10분 + 0~30초 랜덤) 만료 시점을 분산시킨다.

**Distributed Lock (Redis Lock):**
- 캐시가 만료되었을 때, 단 하나의 요청만 DB에 접근해 캐시를 갱신할 수 있도록 잠금(Lock)을 건다.
- 다른 요청들은 잠시 대기하거나 이전의 캐시 데이터를 반환받는다.

**PER (Probabilistic Early Recomputation):**
- 만료 시간이 다 되기 전에 확률적으로 미리 캐시를 갱신하는 알고리즘을 사용하여 '만료되는 순간' 자체를 없앤다.

> Stampede 한 번 터지면 DB가 순식간에 죽어버림. 특히 인기 상품 상세 페이지 같은 hot key가 만료될 때 위험함. Lock 방식은 확실하긴 한데 대기 시간이 생기고, PER은 구현이 조금 복잡함. 일단 Jitter부터 적용하고 모니터링 하는 게 현실적임.

### 3.4 로컬 캐시와 Redis 간 동기화

L1(Local)과 L2(Redis)를 모두 사용할 때, 한 서버에서 데이터를 수정하면 다른 서버의 L1 캐시는 '오래된 데이터(Stale Data)'가 된다.

**Pub/Sub 기반 무효화:**
1. 특정 서버가 DB 데이터를 수정하고 Redis(L2) 캐시를 삭제한다.
2. 동시에 Redis의 Pub/Sub 채널로 "Key-A 삭제" 메시지를 발행한다.
3. 이 채널을 구독 중인 모든 애플리케이션 서버가 자신의 로컬 캐시(L1)에서 "Key-A"를 삭제한다.

**짧은 L1 TTL 설정:**
- Pub/Sub 메시지 유실 가능성에 대비하여, L1 캐시의 TTL을 L2보다 훨씬 짧게(예: 10~30초) 설정해 자연스럽게 동기화되도록 보완한다.

**Redis Streams 활용:**
- Pub/Sub은 메시지 전달을 보장하지 않으므로, 더 높은 신뢰성이 필요하다면 Redis Streams나 Kafka를 통해 무효화 이벤트를 전달한다.

> Pub/Sub의 치명적인 단점이 "fire and forget"이라는 점임. 구독자가 연결이 끊겼다가 다시 붙으면 그 사이 메시지는 다 유실됨. 그래서 L1 TTL을 짧게 가져가는 게 필수임. 완벽한 동기화가 필요하면 Kafka 같은 메시지 브로커를 써야 하는데, 그러면 아키텍처가 복잡해지는 트레이드오프가 있음.

---

## 4. 분산 환경에서의 캐시 설계

### 4.1 Sticky Session 방식의 한계

Sticky Session은 특정 사용자의 요청을 항상 동일한 서버로 보내는 방식이다.

**부하 불균형 (Load Imbalance):**
- 특정 서버에 '헤비 유저'가 몰릴 경우, 특정 노드만 과부하가 걸리는 핫스팟 현상이 발생한다.

**확장성 저하 (Scalability Issue):**
- 서버를 증설하거나 감설할 때 세션 재분배가 어렵다.
- 서버 한 대가 장애로 내려가면 해당 서버에 할당된 모든 세션 데이터가 유실된다.

**유연성 부족:**
- 로드 밸런서가 세션 상태를 관리해야 하므로 인프라 복잡도가 증가하며, 스테이트리스(Stateless)한 설계 철학에 어긋난다.

> Sticky Session이 간편해 보여도 결국 서버를 스테이트풀하게 만드는 거라 확장성에서 발목 잡힘. 요즘 클라우드 환경에서 오토스케일링 쓰려면 스테이트리스가 기본인데, Sticky Session 쓰면 이게 안 됨.

### 4.2 Round-Robin / Random 라우팅에서의 문제점

요청이 매번 다른 서버로 전달되는 무상태(Stateless) 라우팅 환경에서의 이슈다.

**로컬 캐시 히트율(Hit Ratio) 저하:**
- 사용자 A의 데이터가 서버 1의 로컬 캐시에 저장되어 있어도, 다음 요청이 서버 2로 가면 캐시 미스(Miss)가 발생한다.

**데이터 불일치:**
- 서버 1에서 데이터를 업데이트하고 로컬 캐시를 갱신했더라도, 서버 2의 로컬 캐시에는 여전히 과거 데이터가 남아 있을 수 있어 사용자가 매번 다른 데이터를 보게 될 위험이 있다.

> 로컬 캐시만 쓰면서 Round-Robin 하면 캐시 효과가 반토막 남. 그래서 L1+L2 이중 캐시 구조가 필요한 거임. L2(Redis)에서 일관성을 보장하고, L1은 성능 부스트용으로만 쓰는 구조.

### 4.3 분산 캐시 무효화 전략 (Distributed Invalidation)

분산된 모든 서버의 캐시 상태를 동일하게 유지하기 위한 핵심 전략이다.

**중앙 집중형 무효화:**
- 데이터 변경 시 Redis(L2)와 같은 공용 저장소의 키를 삭제하고, 각 서버가 이를 인지하게 한다.

**이벤트 기반 무효화:**
- DB의 변경 사항을 감지(CDC)하거나 애플리케이션 레벨에서 변경 이벤트를 발행하여, 해당 데이터를 캐싱하고 있는 모든 노드에게 "만료" 신호를 보낸다.

**최종 일관성(Eventual Consistency) 수용:**
- 모든 노드를 실시간으로 동기화하는 것은 성능 비용이 매우 크다.
- 비즈니스 로직상 허용 가능하다면 짧은 TTL을 통해 시간이 지나면 자연스럽게 일치하도록 설계한다.

> Strong Consistency vs Eventual Consistency는 영원한 트레이드오프임. 실시간 동기화 하려면 매 요청마다 분산 락 걸어야 하는데 그러면 성능이 바닥남. 대부분의 서비스는 몇 초 정도의 불일치는 허용 가능하니까 Eventual Consistency + 짧은 TTL 조합이 현실적임.

### 4.4 Pub/Sub을 활용한 캐시 동기화

Redis의 Pub/Sub 기능을 활용하여 L1(로컬) 캐시 간의 정합성을 맞추는 실무적인 방법이다.

**작동 원리:**
1. 어느 한 서버에서 데이터 수정 발생 → DB 업데이트
2. 해당 서버가 Redis의 특정 채널(예: `cache-invalidation-topic`)에 "Key-X 삭제" 메시지를 발행(Publish)
3. 동일한 채널을 구독(Subscribe) 중인 모든 애플리케이션 서버들이 메시지를 수신하자마자 각자의 로컬 캐시에서 해당 키를 삭제

**주의점:**
- Redis Pub/Sub은 'Fire and Forget' 방식으로 메시지 전달을 100% 보장하지 않는다.
- 네트워크 장애로 메시지를 놓칠 경우를 대비해 짧은 로컬 TTL을 병행 설정하는 것이 필수적이다.

> Pub/Sub 구현 자체는 어렵지 않음. 근데 운영하다 보면 메시지 유실 케이스가 꼭 생김. 서버 재시작 중에 메시지 날아가거나, 네트워크 순단 때 놓치거나. 그래서 Pub/Sub만 믿으면 안 되고 L1 TTL을 10~30초로 짧게 가져가는 게 안전함.

### 4.5 Consistent Hashing

캐시 노드를 증설하거나 제거할 때 발생하는 대량의 캐시 미스를 방지하기 위한 알고리즘이다.

#### 전통적인 Hashing의 문제

`hash(key) % N`(노드 수) 방식을 쓰면, 노드 수가 하나만 변해도 거의 모든 키의 매핑 위치가 바뀌어 캐시 대란(Cache Miss Storm)이 일어난다.

```
예시: 노드 3대 → 4대로 증설 시

hash("user:1") = 10 → 10 % 3 = 1 (Node-1) → 10 % 4 = 2 (Node-2) ❌ 재배치
hash("user:2") = 15 → 15 % 3 = 0 (Node-0) → 15 % 4 = 3 (Node-3) ❌ 재배치
hash("user:3") = 12 → 12 % 3 = 0 (Node-0) → 12 % 4 = 0 (Node-0) ✅ 유지

→ 대부분의 키가 다른 노드로 이동하여 캐시 미스 폭발
```

#### Hash Ring의 동작 원리

**1. 링 구성:**
- 해시 함수의 출력 범위(예: 0 ~ 2^32-1)를 원형으로 연결한다고 상상한다.
- 0과 2^32-1이 연결되어 시계처럼 순환하는 구조다.

**2. 노드 배치:**
- 각 캐시 노드를 해시 함수에 통과시켜 링 위의 특정 위치에 배치한다.
- 예: `hash("Node-A") = 1000`, `hash("Node-B") = 5000`, `hash("Node-C") = 9000`

**3. 키 매핑:**
- 데이터 키도 동일한 해시 함수로 링 위의 위치를 결정한다.
- 해당 위치에서 **시계 방향으로 가장 먼저 만나는 노드**에 데이터를 저장한다.

```
Hash Ring 시각화 (0 ~ 10000 범위 가정):

        0/10000
           |
    9000 ──┼── 1000
   (Node-C) │  (Node-A)
           │
    7000 ──┼── 3000
           │
        5000
      (Node-B)

- key "user:1" → hash = 2000 → 시계방향 → Node-B (5000)
- key "user:2" → hash = 6000 → 시계방향 → Node-C (9000)
- key "user:3" → hash = 9500 → 시계방향 → Node-A (1000, 0을 넘어서)
```

#### 노드 추가/삭제 시 동작

**노드 추가 (Node-D at 7000):**
```
Before: key(6000) → Node-C(9000)
After:  key(6000) → Node-D(7000)  ← 이 키만 재배치

→ 5000~7000 사이의 키들만 Node-D로 이동
→ 나머지 키들은 영향 없음
```

**노드 삭제 (Node-B 제거):**
```
Before: key(2000) → Node-B(5000)
After:  key(2000) → Node-C(9000)  ← 이 키만 재배치

→ 1000~5000 사이의 키들만 Node-C로 이동
→ 나머지 키들은 영향 없음
```

> 전통적인 방식은 노드 하나 바뀌면 전체 키의 ~100%가 재배치되는데, Consistent Hashing은 평균적으로 K/N(전체 키 수 / 노드 수)만 재배치됨. 노드 10대 중 1대가 죽어도 10%만 영향받는 거임.

#### 가상 노드(Virtual Nodes)의 필요성

물리 노드만 사용하면 링 위의 배치가 불균등해질 수 있다.

**문제 상황:**
```
Node-A: 1000
Node-B: 1500  ← A와 B가 너무 가까움
Node-C: 9000

→ Node-A는 1000~1500 (500 범위)만 담당
→ Node-C는 1500~9000 (7500 범위)를 담당
→ 심각한 부하 불균형
```

**해결책 - 가상 노드:**
- 물리 노드 하나당 여러 개의 가상 노드를 링에 배치한다.
- 예: Node-A → Node-A#1, Node-A#2, Node-A#3 (각각 다른 해시값)

```
물리 노드 3대, 가상 노드 각 3개 = 총 9개 포인트

Node-A#1: 1000    Node-B#1: 2000    Node-C#1: 3000
Node-A#2: 4500    Node-B#2: 6000    Node-C#2: 7500
Node-A#3: 8000    Node-B#3: 9500    Node-C#3: 500

→ 링 전체에 고르게 분포되어 부하 균등화
```

> 실무에서는 물리 노드당 100~200개의 가상 노드를 쓰기도 함. 숫자가 많을수록 분포가 균등해지지만, 메모리와 탐색 비용이 증가하는 트레이드오프가 있음.

#### Java 구현 예시

```java
import java.util.*;
import java.util.concurrent.ConcurrentSkipListMap;
import java.security.MessageDigest;
import java.nio.charset.StandardCharsets;

public class ConsistentHashRing<T> {

    // TreeMap의 Thread-safe 버전, 정렬된 상태 유지
    private final ConcurrentSkipListMap<Long, T> ring = new ConcurrentSkipListMap<>();
    private final int numberOfVirtualNodes;
    private final Set<T> physicalNodes = new HashSet<>();

    public ConsistentHashRing(int numberOfVirtualNodes) {
        this.numberOfVirtualNodes = numberOfVirtualNodes;
    }

    /**
     * 물리 노드 추가 - 가상 노드들을 링에 배치
     */
    public void addNode(T node) {
        physicalNodes.add(node);
        for (int i = 0; i < numberOfVirtualNodes; i++) {
            // 가상 노드 이름: "NodeA#0", "NodeA#1", ...
            long hash = hash(node.toString() + "#" + i);
            ring.put(hash, node);
        }
    }

    /**
     * 물리 노드 제거 - 해당 가상 노드들을 링에서 제거
     */
    public void removeNode(T node) {
        physicalNodes.remove(node);
        for (int i = 0; i < numberOfVirtualNodes; i++) {
            long hash = hash(node.toString() + "#" + i);
            ring.remove(hash);
        }
    }

    /**
     * 키에 해당하는 노드 찾기 - 시계방향으로 가장 가까운 노드
     */
    public T getNode(String key) {
        if (ring.isEmpty()) {
            return null;
        }

        long hash = hash(key);

        // hash보다 크거나 같은 첫 번째 키를 찾음 (시계방향 탐색)
        Map.Entry<Long, T> entry = ring.ceilingEntry(hash);

        // 없으면 링의 처음으로 돌아감 (원형 구조)
        if (entry == null) {
            entry = ring.firstEntry();
        }

        return entry.getValue();
    }

    /**
     * MD5 해시 함수 - 균등한 분포를 위해 사용
     */
    private long hash(String key) {
        try {
            MessageDigest md = MessageDigest.getInstance("MD5");
            byte[] digest = md.digest(key.getBytes(StandardCharsets.UTF_8));
            // 앞 8바이트를 long으로 변환
            return ((long) (digest[0] & 0xFF) << 56)
                 | ((long) (digest[1] & 0xFF) << 48)
                 | ((long) (digest[2] & 0xFF) << 40)
                 | ((long) (digest[3] & 0xFF) << 32)
                 | ((long) (digest[4] & 0xFF) << 24)
                 | ((long) (digest[5] & 0xFF) << 16)
                 | ((long) (digest[6] & 0xFF) << 8)
                 | ((long) (digest[7] & 0xFF));
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public int getRingSize() {
        return ring.size();
    }

    public Set<T> getPhysicalNodes() {
        return Collections.unmodifiableSet(physicalNodes);
    }
}
```

**사용 예시:**

```java
public class ConsistentHashExample {
    public static void main(String[] args) {
        // 가상 노드 150개로 해시 링 생성
        ConsistentHashRing<String> hashRing = new ConsistentHashRing<>(150);

        // 캐시 서버 노드 추가
        hashRing.addNode("cache-server-1");
        hashRing.addNode("cache-server-2");
        hashRing.addNode("cache-server-3");

        // 키가 어느 노드로 매핑되는지 확인
        String[] keys = {"user:1001", "user:1002", "product:5001", "session:abc123"};

        System.out.println("=== 초기 상태 (3대) ===");
        Map<String, String> initialMapping = new HashMap<>();
        for (String key : keys) {
            String node = hashRing.getNode(key);
            initialMapping.put(key, node);
            System.out.println(key + " → " + node);
        }

        // 서버 1대 추가
        System.out.println("\n=== cache-server-4 추가 후 ===");
        hashRing.addNode("cache-server-4");

        int movedCount = 0;
        for (String key : keys) {
            String newNode = hashRing.getNode(key);
            String moved = newNode.equals(initialMapping.get(key)) ? "" : " ← MOVED";
            if (!moved.isEmpty()) movedCount++;
            System.out.println(key + " → " + newNode + moved);
        }
        System.out.println("이동된 키: " + movedCount + "/" + keys.length);

        // 서버 1대 제거
        System.out.println("\n=== cache-server-2 제거 후 ===");
        hashRing.removeNode("cache-server-2");

        for (String key : keys) {
            String node = hashRing.getNode(key);
            System.out.println(key + " → " + node);
        }
    }
}
```

**실행 결과 예시:**

```
=== 초기 상태 (3대) ===
user:1001 → cache-server-2
user:1002 → cache-server-1
product:5001 → cache-server-3
session:abc123 → cache-server-1

=== cache-server-4 추가 후 ===
user:1001 → cache-server-2
user:1002 → cache-server-4 ← MOVED
product:5001 → cache-server-3
session:abc123 → cache-server-1
이동된 키: 1/4

=== cache-server-2 제거 후 ===
user:1001 → cache-server-4
user:1002 → cache-server-4
product:5001 → cache-server-3
session:abc123 → cache-server-1
```

> 위 코드는 학습용 기본 구현임. 실제 프로덕션에서는 `ketama` 알고리즘을 구현한 라이브러리(예: SpyMemcached, Jedis의 ShardedJedis)를 쓰거나, Redis Cluster처럼 Hash Slot 방식을 쓰는 게 일반적임. 직접 구현하면 해시 함수 선택, 충돌 처리, Thread-safety 등 고려할 게 많아서 검증된 라이브러리 쓰는 게 안전함.

---
