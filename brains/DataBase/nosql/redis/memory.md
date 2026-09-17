# Redis 메모리 관리, 최적화 방안

Redis는 데이터를 영구적으로 저장하는 AOF/RDB 방식의 DB 백업 용도로 사용되고 있으며, 주로 인메모리 캐시다보니 캐시 저장 용도로 많이 사용되고 있다.

레디스는 시스템 성능을 높이는데 많은 역할을 하는 좋은 솔루션이다. 그렇지만, 레디스는 *메모리 측면에서  제대로 관리하지 않게 된다면 이는 곧 장애로 이어지기도 한다.*

<br>

### Swap

레디스는 메모리에 데이터를 저장한다. 그렇기에 물리 메모리(RAM) 용량보다 더 많은 데이터를 사용하게 된다면 메모리 부족으로 swap 발생하여 Redis의 성능 저하를 일으킬 수 있다.

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_2.jpg)

위의 그림처럼 데이터를 물리 메모리의 용량보다 더 많이 사용하게 된다면 swap이 발생한다. 운영체제에서 swap space의 주요 기능은 물리메모리의 양이 가득차고 더 많은 메모리의 양이 필요할 때 backing store(디스크)의 가상 메모리에 대처하게 된다.

이는 실제 메모리가 부족하더라도 시스템다운 혹은 프로세스 Hang을 방지할 수 있어 보다 안정적일수는 있으나 **디스크의 공간을 메모리처럼 사용하게 되어 처리 속도가 매우 떨어져 성능 저하로 이어진다.**

### Redis OOM

swap이 없는 상태에서 redis 서버가 maxmemory의 양보다 더 많은 데이터를 사용한 경우, `Redis: OOM(Out of memory) command not allowed when used memory than "maxmemory"` 에러를 일으킬 수 있다.

개발자나 클라우드 엔지니어나 운영자 관리자는 성능을 높이기 위해 레디스 오픈소스를 캐시 용도로 도입하면서 메모리 측면의 예기치 못한 이슈들이 발생한다. 이러한 이슈들은 하드웨어 스팩과 비즈니스 프로세스에 맞게 적합한 아키텍처를 적용하여 해결할 수 있다.

위와 같은 문제들을 해결하기 위해서 어떠한 방안이 있는지 알아보겠다.

<br>

## Redis 메모리 관리

레디스는 메모리 관리를 위해서 `maxmemory`  옵션 설정을 통해서 메모리의 양을 그 이상의 범위를 사용하지 않도록 제한할 수 있다.

그리고 redis는 인메모리 db로 사용하며 일반적인 디스크 솔루션에 비해 적은 양의 데이터를 저장함으로써, 이로 인해 최대치로 저장하게 되면 redis의 메모리에 새로운 데이터가 들어오게 되고 기존의 데이터는 제거되는 방식인 eviction이 필요하다. eviction 정책은 redis의 `maxmemory-policy`옵션을 통해 만들수 있다.

### Maxmemory configuration

`Maxmemory configuration`은 설정된 메모리의 양까지 사용하도록 redis를 구성하는 것이다. 이는 `redis.conf`파일에서 설정할 수 있으며 런타임시 `CONFIG SET`명령을 사용해 maxmemory의 양을 지정할 수 있다.

**redis.conf**
```
maxmemory 100mb
```

**redis-cli**
```bash
config set maxmemory 107374182
```

> maxmemory를 0으로 설정하면 메모리 제한이 없는것이다. 이는 64비트 시스템의 기본 동작이며 32비트 시스템은 3GB의 메모리 제한을 사용한다. 즉 **64비트 시스템에서는 메모리 제한이 없다는 것**이다. 그래서 swap 영역까지 사용하지 않도록 maxmemory를 잘 설정해줘야한다.


### maxmemory-policy configuration

maxmemory 제한에 도달했을 때 redis가 따르는 eviction의 정확한 동작은 maxmemory-policy 정책에 따라 작동한다. redis에서 eviction을 위해 사용하는 방식 중 가장 일반적으로 사용되고 있는 LRU(Least Recently Used) 알고리즘(가장 오랫동안 사용하지 않은 값을 evict)이다. 보통 운영체제에서 페이지 교체 알고리즘으로도 많이 쓰인다.

운영체제에서 페이지 교체 알고리즘은 다음과 같다.

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_4.jpg)


redis에서 적용된 페이지 교체 알고리즘은 다음과 같다.

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_5.jpg)

redis에서는 os 페이지 교체 알고리즘 중에서 LRU, LFU, RANDOM 3가지만을 제공한다.


<br>

### Redis key eviction 프로세스

redis eviction 프로세스는 다음과 같이 진행된다.
1. client가 새로운 command를 실행하여, 더 많은 데이터가 추가된다.
2. redis는 메모리 사용량을 확인하고, 설정된 maxmemory 값보다 크면 정책에 따라 keys를 evict한다.
3. 새로운 command가 실행된다.

**이는 redis가 메모리를 회수하는 동안 write 증폭 문제를 야기할 수 있다.**

redis의 eviction 작동방식에서 maxmemory 제한이 설정되고 maxmemory-policy가 `noeviction` 인 경우 redis 메모리 회수 프로세스는 클라이언트가 새로운 command를 생성할 때마다 트리거된다.

ex) redis 서버가 항상 메모리 오버플로우 상태(used_memory > maxmemory)에서 작동하는 경우 메커니즘을 자주 트리거한다. 이는 서버 성능에 영향을 미치며, 또한 replicas가 연결된 경우, key 제거 작업이 replica 노드와 sync 되어 write 증폭 문제가 발생할 수 있다.

이로 인해서, 항상 used_memory < maxmemory 상태로 redis 서버가 실행되도록 구성하는것이 좋다.

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_6.jpg)



### Approximated LRU Algorithm

redis LRU 및 LFU eviction 정책은 근사값이라는 점을 유의하라

redis에서 LRU 알고리즘은 정확하게 구현되어있지 않으며 이는 redis가 eviction을 위해 가장 오랫동안 사용되지 않은 최상위 후보를 항상 선택할 수 없음을 의미한다.

**대신에, 소수의 keys를 샘플링하고 샘플링된 key들 중에서 가장 좋은(가장 오래된 엑세스 time을 가진) key를 eviction 하는 알고리즘 LRU Approximated 를 실행한다.**

redis에서는 `maxmemory-samples` 파라미터 설정에서 지정한 수의 key로 샘플링하여 LRU 알고리즘의 근사치를 계산할 수 있다. 샘플 수가 많을수록 메모리는 많이 사용되지만 정밀도가 올라가는 특징이 있다.

`maxmemroy-samples 5` 와 같이 파라미터는 redis.conf에서 조정이 가능하다.

redis가 실제 LRU를 사용하지 않는 이유는 더 많은 메모리가 필요하다고 cpu 사용량이 증가하기 때문이다.

![](https://s-core.co.kr/wp-content/uploads/2023/03/51_8.jpg)

위의 그래프에서는 redis의 실제 LRU와 근사치 LRU로 구분이 되어 있는데, redis 3.0에서는 eviction을 위한 best candidates를 선택하는 알고리즘이 개선되면서, 실제 LRU 알고리즘과 더 유사해졌다. **Redis 3.0에서 샘플링 수를 10으로 설정하면 좀 더 실제에 가까운 LRU의 기능과 유사해질 수 있다.****

단, 샘플 개수를 늘리면 실제 LRU 알고리즘과 유사해지니 주의하자, 적정 수준 이상으로 샘플 개수를 설정하면 redis의 cpu 사용량이 늘어나고 응답 속도가 늦어지는 리스크가 있으니

<br>

마무리 하면서, eviction 정책을 효과적으로 설정하기 위해 redis에서는 noeviction으로 메모리를 주기적으로 모니터링해 증성할 수 있고, 일정 기간 동안만 데이터를 유지한다면 `volatile-` 아니라면 `allkeys-`으로 정책을 고려할 수 있다. 따라서, 시스템에서는 비즈니스 로직에 맞는 최적의 메모리 관리에 대해 어떻게 관리할 것인지 면밀히 검토한 후 결정해야 한다

