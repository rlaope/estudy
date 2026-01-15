# Redis Serialization Sync

레디스에 json 형태의 오브젝트를 value로 두었다고 쳐보자 

예를들어서 userUid: 1 { "user": "khope" }

근데 여기서 만약에 user필드가 username이 되었다면 어떻게 처리해볼수있을까

### Duel Mapping

코드상에서 구 필드와 신필드를 모두 수용하도록 로직을 수정해볼까 db부하가 전혀 없긴할텐데.

read: redis에서 json읽고 username 필드가 있으면 그대로 사용한다. 만약 username이 있고 user만 있다면, user값을 username으로 간주해야할까?

write: 이후 새로운 데이터는 redis에서 무조건 username으로 저장해볼까?

```java
String json = redis.get(key);
UserDto user = objectMapper.readValue(json, UserDto.class);

if (user.getUsername() == null && user.getUser() != null) {
  user.setUsername(user.getUser()); 구 필드 값을 신 필드에 할당?
}
```

혹은 lua 스크립트로 in place update를 해볼까?

redis 내부에서 데이터를 직접 수정하는 방법인데 데이터를 꺼내서 애플리케이션으로 가져온뒤 다시 보내는 네트워크 비용을 줄이고 원자성 보장도 된다.

redis에 EVAL 명령어로 모든 키를 순회해 json 내부 키값을 변경할수있긴한데.. EVAL 괜찮으려나 single thread인데 redis는

```lua
-- Redis 내의 특정 패턴의 키를 찾아 'user' 필드를 'username'으로 변경
local keys = redis.call('KEYS', 'user_cache:*') -- 실제로는 SCAN 권장
for i, key in ipairs(keys) do
    local val = redis.call('GET', key)
    if val then
        local updated = string.gsub(val, '"user":', '"username":')
        redis.call('SET', key, updated, 'KEEPTTL') -- 기존 TTL 유지
    end
end
```

백그라운드에 마이그레이션 도구를 활용해보는것은?

배치 스크립트가 scan으로 redis key 읽어서 데이터변환 해주고 set해주는 배치.. ㅎㅎ 단순하긴한데 

<br>

### EVAL, KEYS, SCAN 괜찮은가

redis는 single thread로 동작하기 때문에 한번에 너무 많은 키를 처리하는 EVAL, KEYS는그 작업이 끝날떄까지 모든 요청을 대기하기 때문에 redis blocking이 발생가능

따라서 운영 환경에서는 절대 몰아서 처리하면 안됨

SCAN은 괜찮음 keys는 모든 키를 가져오지만 scan은 커서ㅂ기반으로 일정량 기본 10개만 가져오고 다음은 여기부터 찾으세요 하고 번호 반환함 그리고 넌블로킹이라 아주 짧은 시간만 점유해서 ㄱㅊ

### 안전한 동기호 ㅏ방식

싱글 스레드인 레디스에 무리를 주지않으면서도 필드명을 바꾸는 방법은

**애플리케이션에서 점진적 마이그레이션?**
- redis에 내부 로직을 돌리지않고 외부에서 조금씩 처리하는 방식으로 배치 애플리케이션을 작성한다치면 100개씩 업뎃 해주는거. 해당 키들의 데이터를 MGET(여러개읽어옴)으로 읽어온다. 
- 요구사항상 근데 필드명 바뀌는건 비교적 빈번하게 발생하진 않을거같기도한데 흠 아닌가. 외부연동시스템이라면 사전공지를 해주기때문에. 배치로 대처해야할까

