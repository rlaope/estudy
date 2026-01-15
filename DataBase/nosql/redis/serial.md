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