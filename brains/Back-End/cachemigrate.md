# 캐시 무중단 마이그레이션 (Lazy Migration)

다음과 같은 전제상황을 예시로 들어보자.

트래픽은 일 평균 천만단위의 요청이고 피크타임엔 10배이상증가가 예상되고 읽기 쓰기 비율이 9 1이며 마이크로서비스 아키텍처에 서비스 인스턴스가 여러대 있는 환경에서

redis 메모리 사용량이 80%정도 되고있고 cache hit rate는 70정도 유지중이다. 50% 아래로 떨어지면 db 장애가 발생한다고 쳐보자.

redis cpu 사용량은 20정도 여유가 있는 상황이고.

여기서 하위 호환성이 깨지게되는 변경사항이 발생했다고 해보자. json 데이터 필드 변경같은

```
{ "name": "khope" } -> { "username": "khope" }
```

위처럼 변경사항이 생겼다고 해보자. 이때의 캐시시스템 무중단 배포를 생각해볼 것이다.

캐시를 바로 evict하게 되면 캐시 스탬피드 현상으로 인해 db에 트래픽이 쏠리게 될것이고 전면장애가 예상된다. 그러므로 생각하지 않도록 한다.

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Server A  │     │   Server B  │     │   Server C  │
│ ┌─────────┐ │     │ ┌─────────┐ │     │ ┌─────────┐ │
│ │L1 Cache │ │     │ │L1 Cache │ │     │ │L1 Cache │ │
│ │(Caffeine)│ │     │ │(Caffeine)│ │     │ │(Caffeine)│ │
│ └────┬────┘ │     │ └────┬────┘ │     │ └────┬────┘ │
└──────┼──────┘     └──────┼──────┘     └──────┼──────┘
       │                   │                   │
       └───────────────────┼───────────────────┘
                           │
                    ┌──────▼──────┐
                    │  L2 Cache   │
                    │   (Redis)   │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │  Database   │
                    └─────────────┘
```

일단 지금 이중 캐시로 되어있다. RTT 누적 지연 증가 및 redis 노드의 cpu 병목 가능성 redis 장애시 서비스 영향등을 고려했고

L1을 앞에 두게되면 Hot data는 네트워크 비용이 0으로 응답되고 레디스 부하분산에 레디스 순단시에도 L1 데이터로 버틸수있다.

물론 L1 캐시간의 불일치도 고민을 해야한다. 이건 이전글 캐시 시스템 설계 개념/구현편에서 다루었더니 간단하게 pub sub 기반 캐시 무효화를 선택했다는것으로만 알고가자.

+추가로 캐시데이터 유형별 관리 포인트, write behind, distributed lock 등도 거기서 언급됐으니 패스한다.

### 무중단 스키마 마이그레이션

아까 봤던 요구사항처럼 json 구조가 바뀌면 어떻게 무중단 마이그레이션을 할 것인가?

```
// 구버전 (V1)
{"id": 1, "name": "홍길동", "email": "hong@test.com"}

// 신버전 (V2)
{"id": 1, "username": "홍길동", "email": "hong@test.com"}
```

단순히 캐시 전체 flush 방법을 생각해볼수있고 (이건안함), TTL 만료를 기다려 볼수도있는데 이는 마이그레이션 기간이 너무 길 수 있다.

키의 버전값을 바꿀수도 있는데 이건 기존 값 x 2 혹은 그 아래여도 어쨋든 메모리가 더 필요하기 때문에 지금 전제 80퍼센트 메모리 사용량에선 힘들다.

그럼 어떻게 hit rate를 유지하면서도, 기존의 값들을 빠르고 정확하게 마이그레이션 해볼 수 있을까?

두 가지 방법을 적용해볼 수 있는데

- **Lazy Migration**: 읽을 때 변환하는 방식이다. (hit rate)
- **Background Migration**: redis cpu가 여유분이 있으니 백그라운드에서 조금씩 변환해주는게 같이 돌아 변환속도 늘려줌

```java
public <T> T getWithMigration(String key, Class<T> targetType) {
    String json = redisTemplate.opsForValue().get(key);
    if (json == null) return null;

    // V2 형식인지 확인
    if (json.contains("\"username\"")) {
        return objectMapper.readValue(json, targetType);
    }

    // V1 → V2 변환
    JsonNode node = objectMapper.readTree(json);
    if (node.has("name")) {
        ((ObjectNode) node).set("username", node.get("name"));
        ((ObjectNode) node).remove("name");

        // 변환된 데이터를 다시 저장 (TTL 유지)
        Long ttl = redisTemplate.getExpire(key, TimeUnit.SECONDS);
        String newJson = objectMapper.writeValueAsString(node);
        redisTemplate.opsForValue().set(key, newJson, ttl, TimeUnit.SECONDS);
    }

    return objectMapper.readValue(node.toString(), targetType);
}
```

이런식의 의사코드이다. v1이 들어왔는지 v2가 들어왔는지에 대한판단은 exception catch or parsing validate등 정책을 정해볼수는 있으나

예외도 비용이고 두개다 호환되게 필드명에 alias를 줄수도 있고, (물론 이건 기존필드 변경일때만 적용이고 새로 추가되거나 삭제되거나는 또 다른방식의 구현이지만 어쨋든)

결과적으론 v1의 데이터라면 일단 v2로 사용하고 해당 캐시를 새로 set해주는 방식이다. 이러면 hit rate를 줄이지 않고 해결해볼수있고.

+redis의 cpu는 여유분이 있으니까 백그라운드에서 선제적으로 마이그레이션을 진행해 기간을 단축해볼 수 있다. lua script로 예시를 들면

```lua
-- migrate_user.lua (Redis 서버에서 실행)
local key = KEYS[1]
local data = redis.call('GET', key)
if not data then return -1 end

local obj = cjson.decode(data)
if obj['username'] ~= nil then return 0 end  -- 이미 V2

if obj['name'] ~= nil then
    obj['username'] = obj['name']
    obj['name'] = nil

    local ttl = redis.call('TTL', key)
    local newData = cjson.encode(obj)
    if ttl > 0 then
        redis.call('SETEX', key, ttl, newData)
    else
        redis.call('SET', key, newData)
    end
    return 1  -- 마이그레이션 성공
end
return 0
```

이렇게 둘 수도 있고 혹은 java task서버에서 get set만하는 lua를 만들어두고 json parsing하는 로직은 거기서 하도록 해도 된다.

아무래도 레디스도 싱글스레드에 이벤트루프다보니까 저런 파싱같은 작업을 두기엔 조금 부담이 될수도 ㅇㅇ

저런 백그라운드 테스크를 트리거하는 부분에서도 cpu 사용률을 주기적으로 체크해 redis cpu에 부담이 가지 않도록 해주자.

```java
@Scheduled(fixedDelay = 10000)
public void migrateInBackground() {
    double cpuUsage = getRedisCpuUsage();

    // CPU 사용량에 따라 배치 크기 동적 조절
    int batchSize;
    if (cpuUsage > 70) batchSize = 0;       // 중단
    else if (cpuUsage > 60) batchSize = 50;  // 최소
    else if (cpuUsage > 40) batchSize = 100; // 보통
    else if (cpuUsage > 20) batchSize = 300; // 여유
    else batchSize = 500;                    // 매우 여유

    if (batchSize > 0) {
        scanAndMigrate(batchSize);
    }
}
```

lua로 작성한 이유는 네트워크 왕복 1회로 읽기 변환 쓰기 같이 할 수 있고 redis 서버에서 처리하며 multi/exec보다 효율적이라서 그렇다. (여기서 효율적은 간단함이라고 생각함)

자 일단, 어느정도 구현 방법은 알았고 배포 관련해서도 고민해봐야겠다. 분명 여러 서비스들이 해당 캐시값을 읽어서 사용할것이고.

서버는 n개니까 저런 캐시 관련 로직은 공통 라이브러리에 변경사항을 적용하고 배포를 한다고 가정한다.

일단 db 스키마에 변경이 있기전에 v1,v2 전부다 호환가능한 저 로직부터 먼저 배포가 나가야할 것이다.

```
Phase 1: 하위호환 코드 배포 (모든 서버)
         ↓
         V1, V2 둘 다 읽을 수 있는 상태
         ↓
Phase 2: DB 스키마 변경
         ↓
Phase 3: 새 데이터는 V2로 저장
         ↓
Phase 4: 백그라운드 마이그레이션 완료 확인
         ↓
Phase 5: V1 호환 코드 제거 (기술 부채 정리)
```

이런 캐시 변경사항은 카나리 자체가 빠르게 나가도 상관없다고 생각하는편이다.

추가적으로 신경써야할건 v1 v2가 같이 공존하는데, 만약 기존필드변경이 아니고 새로운 필드가 생겼는데 v1에서 v2 nullable이면 안되는값이 null로 식별된다거나 또 발생할 수 있을거같다.

이건 카나리를 천천히 해보면서 cache migration가 다 돌때까지 동기화 시간대신, 데이터의 정합성을 신경쓰는 전략으로 속도를 잡아도 괜찮고

추가로 어찌되었든 v1의 v2덮어쓰기 방지가 필요한거다. 예를들어 아까 json 예시에 age 키가 추가되었다고 했을때

모든서버 v1이 내가 모르는 필드라도 일단 유지하고 다시 저장하도록 변경해야한다. jackson 코드로 예시를 들면 `@JsonIgnoreProperties(ignoreUnknown=false)` 같은걸 달아두면 JsonAnySetter Getter랑 활용할 수 있는데

```java
// V1 코드에 미리 반영되어야 할 구조
public class UserDto {
    private String name;
    
    // 내가 모르는 필드(V2의 새 필드 등)를 담아두는 바구니
    private Map<String, Object> unknownFields = new HashMap<>();

    @JsonAnySetter
    public void setUnknownField(String key, Object value) {
        unknownFields.put(key, value);
    }

    @JsonAnyGetter
    public Map<String, Object> getUnknownFields() {
        return unknownFields;
    }
}
```

이런식으로 v1서버가 데이터를 읽고 쓸때 새로 추가된 age 필드를 건드리지않고 그대로 pass through 할수있게 개발해볼 순 있다.

추가로 write 시점에 해당값이 null인지를 체크하는 로직을 두고 실패처리한다거나 miss처리를 해서 local말고 global에서 본다거나 동작하도록 해볼수도 있을거다.

정리해보면

1. 모든서버 unknown 필드 보존 로직과 기본값 처리로직, 공존로직 및 레이지 마이그레이션 변경을 배포하고 v1
2. db 스키마 업데이트를 한다. nullable하게 추가해두고 not null로 유연하게 해볼수도 있지만 정책상 강하게 잡아야하는애면 x
3. v2 서버를 카나리로 배포한다 이제 v2가 새 필드를 써도 v1서버가 이를 지우지 않는다.
4. 모든 데이터가 마이그레이션 되면 db 애플리케이션 optional이나 공존하게 동작하는 로직을 다시 없애주자.

요약하면 v2를 배포하기 이전에 v1 서버가 새 데이터를 망가트리지 않게 만드는 필드 보존 배포가 선행되어야한다.

코드는 여기서 볼 수 있다. https://github.com/rlaope/cache-labs