# Redis INCR

Redis INCR 명령은 atomic하게 동작하는 operation이다.

그렇기에 같은 데이터가 있을때 여러 세션에서 한 데이터에 대한 업데이트 작업을 진행하더라도

각각의 명령어들이 순차적으로 처리되기 때문에 정확하게 업데이트가 된다.

예를 들어서 두 개의 세션에서 한 데이터를 ++ 하는 연산을 하려할 때 동시에 1이라는 값을 읽었다면

세션1에서도 세션 2에서도 1 -> 2 이런식으로 업데이트를 진행하기 때문에 기댓값인 3을 이룰 수 없다.

그렇기에 우리는 이러한 증감 연산을 원자적으로 처리할 수 있도록 INCR를 사용해볼 수 있다.

INCR의 명령어는 atomic operation이므로 여러 요청이 들어오게 되더라도 각 요청에 대해 하나씩만 증가시키기 된다. race condition을 해결할 수 있다.

### 주의 할 점

예를 들어 자습 신청 서비스에서 redis getset을 통해 자습 신청 정보들을 저장하고 관리하며

자습신청을 할 때마다 데이터를 1씩 증가시킨다고 가정해보자.

보통 이런 양상으로 흘러 갈 것이다.

1. if(자습신청 최대 인원 수 확인 && 유저의 자습 가능여부 상태 확인)
2. 검증이 완료되었다면 유저를 자습상태로 전환, 자습 신청 현재 인원수 +1

그러나 여기서 1번 과정을 원자적으로 처리하지 않는다면? 여기서 여러 세션이 열려있어 동시에 get set 연산이 들어오게 된다면, 최대 인원을 초과해버리는 문제가 발생하게 될 것이다. race condition이 발생한다.

redis는 기본적으로 싱글스레드지만 io 멀티플렉싱을 통해 입력을 동시에 받는다. 그렇기에 각 세션마다 열려있는 연산들은 원자적으로 처리하지만, 여러 세션이 열려있는 상태에서는 원자성을 보장하지 못한다.

이는 레디스 공식 문서에도 언급되어 있다.

그렇기 때문에 이렇게 GET, INCR 하는 연산들을 하나의 트랜잭션과 Lock 메커니즘등을 적용해 묶어서 원자적으로 실행하도록 하거나, Lua Script를 통해 EVAL 명령어를 통해 레디스 레벨에서 연산의 원자성을 보장해 사용하도록 해결해볼 수 있다.

```lua
EVAL "
	local i = redis.call('get', 'i'); 
	i = i + 1;
	redis.call('set', 'i', i);
"
```

```
MULTI
... 조합할 명령어들
EXEC
```

```plaintext
❗️ 분산 락은 꼭 필요할 때만 써야 합니다.

최근 취준생분들의 포트폴리오에 분산락이 많이 보입니다.
학습용으로 사용해 본건 좋지만 문제 해결 관점에서
분산락을 어필하기란 쉽지 않을겁니다.

락을 건다는 것은 병목 지점을 만드는 것입니다.

물론 손 쉽게 동시성 문제를 해결할 수 있지만,
락을 걸지 않을 방법이 있다면 쓰지 않는게 낫습니다.

개인적으론 난이도와 성능의 트레이드오프라 생각합니다.

출처 - F-lab 대규모 처리 시 redis 연산을 atomic하게 보장하기
```

 [이 글l](https://hyperconnect.github.io/2019/11/15/redis-distributed-lock-1.html) 에서 RedissionLock을 예시로 들어주시는 부분이 있다. 여러 명령어를 조합하여 atomic하게 만들어 버그를 방지하면서 성능을 높일 수 있다고 한다.

<br>

### INCR Code 분석

```c
void incrDecrCommand(client *c, long long incr) {
    long long value, oldvalue;
    robj *o, *new;
    dictEntry *de;
    o = lookupKeyWriteWithDictEntry(c->db,c->argv[1],&de);
    if (checkType(c,o,OBJ_STRING)) return;
    if (getLongLongFromObjectOrReply(c,o,&value,NULL) != C_OK) return;

    oldvalue = value;
    if ((incr < 0 && oldvalue < 0 && incr < (LLONG_MIN-oldvalue)) ||
        (incr > 0 && oldvalue > 0 && incr > (LLONG_MAX-oldvalue))) {
        addReplyError(c,"increment or decrement would overflow");
        return;
    }
    value += incr;

    if (o && o->refcount == 1 && o->encoding == OBJ_ENCODING_INT &&
        (value < 0 || value >= OBJ_SHARED_INTEGERS) &&
        value >= LONG_MIN && value <= LONG_MAX)
    {
        new = o;
        o->ptr = (void*)((long)value);
    } else {
        new = createStringObjectFromLongLongForValue(value);
        if (o) {
            dbReplaceValueWithDictEntry(c->db,c->argv[1],new,de);
        } else {
            dbAdd(c->db,c->argv[1],new);
        }
    }
    signalModifiedKey(c,c->db,c->argv[1]);
    notifyKeyspaceEvent(NOTIFY_STRING,"incrby",c->argv[1],c->db->id);
    server.dirty++;
    addReplyLongLongFromStr(c,new);
}

void incrCommand(client *c) {
    incrDecrCommand(c,1);
}

void decrCommand(client *c) {
    incrDecrCommand(c,-1);
}

```

[Redis GitHub INCR](https://github.com/redis/redis/blob/unstable/src/t_string.c#L610) 코드를 확인해보면 다음과 같이 이루어져 있는데 한 번 분석해보자
1. 입력으로 들어온 key가 존재하는지 여부를 확인한다. 존재하지 않으면 그 키를 새로 생성하고 0으로 설정한다.
1. 입력으로 들어온 key의 값을 정수 값으로 변환하고 실패시 에러를 반환하며 정수값이게 된다면 증가 또는 감소 연산을 수행한다.
2. 변환하고 증가한 값을 오버플로우가 되었는지에 대한 여부를 체크한다.
3. 연산이 완료되면 데이터를 저장하며 클라이언트에게 값을 반환하고 마무리한다.

<br>

### Cluster 환경에서의 INCR

Redis를 활용하면 **가용성, 확장성, 대용량 처리** 등등 다양한 이유로 클러스터 환경으로 운영하는 것이 보통이다.

마스터가 2개 이상인 레디스 클러스터로 운영하게 된다면 incr 같은 명렁어에 대한 동시성 문제는 어떻게 될 것인가.

일단 레디스는 해시 슬롯을 기반으로 키를 여러 마스터 노드에 분산하기 때문에, 클러스터에는 총 16384개의 해시 슬롯이 존재하며, 각 마스터 노드는 이 슬롯들 중 일부를 담당한다.

그래서 각각의 노드들이 약 5000개정도를 담고 있다.

레디스에 입력된 모든 키는 클러스터에 저장될 때, 특정 해시 슬롯에 매핑되며, 해시 함수를 활용해 그 슬롯을 담당하는 마스터 노드로만 라우팅이 된다. 즉, 특정 키는 항상 같은 마스터 노드에 저장되고 그 노드에서만 처리되기 때문에 다른 마스터노드는 해당 데이터에 접근하지 않는다.

레디스에서는 클라이언트가 찾는 데이터가 없는 슬롯에 접속하게 되면 MOVE를 통해 리다이렉션해 올바른 노드로 안내하게 된다. (이러한 클러스터간 올바른 해시슬롯을 갖고있는 여부를 확인할 수 있는 이유는 노드들이 실시간으로 설정 정보를 공유하고 인지하는 raft 분산 합의 알고리즘 덕분이다.)

그렇기에 결과적으로 같은 데이터를 바라보고있는 구조가 아니기 때문에 incr이 원자적으로 동작하게 된다.

**물론 active-active 이중화 환경이라면 말이 달라질 수 있겠다. 데이터를 잘 미러링하는 것이 중요한 포인트라고 볼 수 있겠다.**