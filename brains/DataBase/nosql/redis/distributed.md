# Redis Distributed Lock, RedLock


### Redis 분산락 방식 사용

https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb8XI8O%2FbtszM1YHVMa%2FMbKUTWapMhO8K7YkAg8cmK%2Fimg.png

분산락은 **Race Condition이 발생할 때 하나의 공유 자원에 접근할 때 데이터에 결함이 발생하지 않도록 원자성을 보장하는 기법이다.**

대표적으로 사용해볼 저장소는 Redis로 싱글스레드 기반의 NoSQL이다.

여러대의 서버들은 공통된 Redis를 바라보며 자신이 임계영역에 접근할 수 있는지 확인하고 이러한 부분에서 원자성을 확보한다.

> Redis도 물론 싱글스레드기반이기 때문에 단일 장애 지점이 될 수 있음을 고려하여 Failover용 추가 slave 자원을 구축해야한다.
> 

## Trade-off

Redis Client를 활용해서 락을 구현해 볼 것이다. Redis Client인 Redission, Lettuce를 고려해 볼 수 있다.

둘은 공통적으로 스핀락을 사용해 락을 얻으려고 시도하며 스핀락에 단점인 과도한 부하를 최소화하기 위하여 적절한 주기로 적당량의 요청을 보낸다.

이는 서버 측에서 구독한 클라이언트에게 락을 사용해도 된다고 알림을 주어 락의 획득 여부를 클라이언트가 요청해서 확인하지 않아도 되게 하는 기법이다.

> 스핀락? 락의 획득 여부를 계속해서 무한루프를 돌면서 시도하는 방법
> 

자 그러면 Lettuce vs Redission 어떤 기능을 사용해야할까?

### Lettuce

Lettuce는 Netty기반의 레디스 클라이언트며 요청을 넌블러킹으로 처리해 높은 성능을 자랑한다.

spring-data-redis를 추가했다면 기본적으로 redis client가 제공되는데 이것이 Lettuce기반이다.

- 장점으로는 redis 의존성을 추가하면 기본적으로 제공되므로 별도의 설정 없이 간단 구현 가능하다.
- 단점으로는 구현 방식에서 스핀락을 사용하기 때문에 레디스에 부하를 줄 수 있다.

### Redisson

Redisson은 pub/sub 기능을 제공한다.

이를 사용하면 스핀락 방식을 사용하지 않고 분산락을 구현할 수 있다.

직접 구현할 수도 있지만 redisson에서 이미 구현하여 제공해주고 있다.

```groovy
implementation 'org.redisson:redisson-spring-boot-starter:3.24.3'

```

다음과 같은 의존성을 추가하여 사용할 수 있다.

그렇기에 레디스에 부하를 덜 줄 수 있는 Redisson을 사용하겠다.

> 나중에 고려해봐야할 점 -> 서비스 내에서 클라이언트 클래스를 사용하지만 AOP를 통해 어노테이션 방식으로 락을 적용하는 라이브러리를 만들어봐도 괜찮을 것 같다.
> 

### RedLock

Redission, Lettuce의 락 방식과는 다르게 레드락 방식도 존재한다.

레드락은 N대의 Redis 서버가 있다고 가정할 때 **과반 수 이상의 노드가 Lock을 획득했다면 Lock을 획득한 것으로 간주한다.**

레드락은 다음과 같은 세가지 속성으로 모델링 하였다.

1. 상호 배제: 오직 한 순간에 하나의 작업자만이 락을 걸 수 있다.
2. 교착 상태 X: 락 이후, 어떠한 문제로 인해 락을 못 풀고 종료된 경우라도 다른 작업자가 락을 획득할 수 있어야한다.
3. 내결함성: Redis 노드가 작동하는 한, 모든 작업자는 락을 걸고 해체가 가능해야한다.

[distributed-locks](https://redis.io/docs/manual/patterns/distributed-locks/)

레드락의 동작 방식은 다음과 같이 진행이 된다.

1. 현재 시간 단위를 ms 단위로 구합니다.
2. 순차적으로 N대 서버에게 락 획득 요청을 보냅니다. 이 때 timeout의 시간은 락의 유효 시간보다 훨씬 작게 설정합니다. 만약 락의 유효시간이 10초라면 타임아웃은 5~50ms입니다. 위와같이 타임아웃을 짧게 사용하는 이유는 장애가 난 레디스 노드와의 통신 시간이 많아지는 것을 방지하기 위해서입니다.
3. Redis서버가 7대라고 했을때 **과반 수 이상의 레디스가 (4개) 락을 획득하기 위해 사용된 시간이 유효시간보다 작다면 락을 획득했다고 간주**합니다. 유효시간이 10초인데 락을 획득하는 시간이 7초면 성공 11초면 실패하는 것을 예시로 들 수 있습니다.
4. 락을 획득한 후의 유효시간은 `처음 설정된 유효시간 - 락을 획득하기 위해 사용된 시간`입니다. 유효시간이 10초고 락을 획득하기 위해 3초가 걸렸다면 7초 뒤에 락이 만료가 됩니다.
5. 락을 얻지 못했다면(과반수가 얻지 못했다면) 모든 레디스 서버에게 락을 해제하라고 요청을 보냅니다.

### 레드락의 한계?

```java
// THIS CODE IS BROKEN
public void writeData(String filename, String data) {
    Lock lock = lockService.acquireLock(filename);
    if (lock == null) {
        throw new RuntimeException("Failed to acquire lock");
    }

    try {
        File file = storage.readFile(filename);
        String updated = updateContents(file, data);
        storage.writeFile(filename, updated);
    } finally {
        lock.release();
    }
}

```

위의 코드는 레드락 방식을 사용하여 파일을 작성하는 로직을 가지고 있다.

여기에서는 문제가 있는데 그것을 시각화 한 다이어그램은 아래 사진과 같다.

!https://miro.medium.com/v2/resize:fit:1400/format:webp/1*wwwIk_UkqKwRJ5ED8Wag4w.png

클라이언트1이 락을 획득하게 됐는데 GC가 발생하여 STW가 발생하여 그 사이 애플리케이션이 중지되어 락이 만료가 되었다.

그 후 클라이언트2가 분산락을 획득하여 파일에 데이터를 작성한다.

클라이언트1이 그 이후 STW가 끝나고 데이터에 write를 하면 동시성 문제가 발생하게된다.

[Protecting a resource with a lock](https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html)

> 일반적으로 GC는 매우 빠르게 수행되지만 Stop-the-World GC는 드물게 잠금이 만료될 정도로 지속될 수 있다. Martin Kleppmann의 문서를 보면 GC 말고도 네트워크 지연이나 timing 이슈에 따라 RedLock이 깨질 수 있음을 알 수 있다.
> 

위와 같은 문제에 관한 해결은 위의 링크에서 Making the lock safe with fencing 파트에서 간단하게 알려주고 있는데, fencing token이라는 개념을 도입하여 간단히 잠금을 획득할 때 마다 증가하는 단순한 숫자스토리지 서버가 이 토큰의 값을 확인하고 쓰기 여부에 대해서 지정해주는 방식이다. 그러나 이러한 방식도 잠금을 획득할 때마다 일관성있는 펜싱 토큰을 생성해준다는 보장이 없다.

그리고 이러한 방식들에서는 추가적으로 더 논의 해 볼 점들이 있는데, 그 내용이 매우 많아 간단히 요약해보자면

1. 레드락과 같은 종류의 알고리즘에 대한 가장 실용적인 시스템 모델은 [신뢰할 수 없는 오류 감지기를 갖춘 비동기 모델](https://courses.csail.mit.edu/6.852/08/papers/CT96-JACM.pdf)이다. 쉽게 말해서 프로세스가 중지된다거나 패킷이 지연된다거나
2. 레드락 알고리즘에서 시간 관련된 시스템을 사용할 수 있는 유일한 사례가 존재하는데 이는 결과가 오지 않는 노드를 계속해서 기다리는 것을 방지할 때 이다. 그러나 TTL을 설정하고 TTL안에 응답이 오지 않았다는 것이 무조건 노드가 다운된 것이라고는 판단하면 안된다.

이러한 비동기식 알고리즘 시스템은 일반적으로 타이밍을 고려하지 않고 안전 속성이 보장되게 유지합니다.

시스템의 타이밍이 제 위치에 있더라도(프로세스 일시 중지, 네트워크 지연) 알고리즘 성능이 처참해질 수는 있지만 알고리즘은 결코 잘못된 결과를 만들지 않음을 의미합니다.

그러나 나쁜 타이밍으로 레드락 알고리즘이 깨질 수도 있으며 동시성 문제도 완벽하게 해결할 수 없기 때문에 그렇게 좋은 선택은 아니라고 생각됩니다.

관련 문제에 대해 상세하게 설명되어져있는 문서는 아까 위에서도 링크한 [Martin Kleppmann - How to do distributed locking](https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html) 문서를 참고했습니다.