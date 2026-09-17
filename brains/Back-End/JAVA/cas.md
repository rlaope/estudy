# CAS(Compare And Swap)

CAS는 cpu가 제공하는 **하드웨어 수준의 원자적 연산**이다.

이 연산은 락을 걸지 않고도 동시에 공유 데이터를 안전하게 변경할 수 있게 해준다.

다음과 같이 동작하는데 cas는 세가지 인자를 받는다.

```
CAS(메모리주소, 기대값, 새로운값)
```

그리고 아래처럼 동작하는데

1. 메모리 주소에 들어있는 현재 값이 기대값(expected)과 같은지 비교한다
2. 같으면 -> 새로운 값으로 new 교체 성공
3. 다르면 -> 실패 아무것도 하지 않는다.

이 전체 과정이 cpu 한 명령어로 원자적으로 실행되기에 즉, 동시에 여러 스레드가 실행되어도 중간에 끼어들 수 없다.

```java
int expected = 10;
int newValue = 11;

if (memory == expected) {
    memory = newValue;
    return true;
} else {
    return fase;
}
```

자바에서는 이 연산을 `Unsafe.compareAndSwapXXX()` or `AtomicInteger.compareAndSet()`, VarHandle로 제공한다.

- 장점: 락 없이 원자성을 보장하고 스레드 경합이 적고 빠름
- 단점: 실패시 재시도가 필요해 스핀 반복에 경쟁이 심하면 오히려 cpu 낭비임
- 특징: 락프리 알고리즘 구현의 기본 빌딩 블록임.

즉, cas는 잠그지 않고 경쟁을 해결하는 기술이며

Lightweight Lock이나 Atomic 패키지에서 대부분 CAS 기반으로 되어있다.

- biased lock 해제 이후 lightweight lock 단계에서 cas로 mark word를 교환한다.
- AtomicInteger.incrementAndGet() 내부적으로 compareAndSet()을 사용한다.

락 없이 상태 교환을 보장하므로 jvm의 락 소유권이나 상태를 빠르게 전환할 수 있는 기반이 되기 때문에 채택하게 된것이다.

### ConcurrentHashMap의 성능 개선 배경

기존 Hashtable / Collections.synchronizedMap 문제는 

단순하게 락 관리가 되어있었다 

```java
synchronized(
    // put/get 다검
)
```

즉, 전체 맵이 하나의 락을 걸었기 때문에 여러 스레드가 동시에 접근하면 항상 대기해야 했다.
-> 스레드 100개면 99개가 wating


ConcurrentHashMap의 핵심 개선포인트는
- jdk7까지는 segment 구조로
  - map을 여러 segment로 나누어 개별 락을 적용함
  - 동시에 여러 세그먼트들 접근 가능해 병렬성이 향상됨
  - put get 중에는 충돌이 없는 키는 동시에 처리 가능
- jdk8 이후 cas + synchronized 혼합
  - segment 구조 제거되고 노드 단위 finer-grained(더 세밀한 단위의) 제어
  - cas + synchronized 블록 + volatie을 함께 사용함
  - put get update 대부분 cas 기반으로 처리함

주요 성능 개선 원리 jdk 8 기준

CAS - 새로운 키 밸류 삽입시 버킷이 비어있으면 cas로 바로 삽입 (락 불필요)  
synchronized - 충돌이 난 동일 버킷 체인만 락  
volatie 변수 - 배열 참조 및 노드의 value 변경 가시성을 보장함.  
TreeBin - 체인이 길어지면 red-black tree 기반으로 변환해 탐색속도 O(log n)

기본값이 아마 8일겁니다 8지나면 레드블랙 트리로 해당 버킷에 데이터들을 관리 평소엔 리스트로 넣고

즉, 락을 전체가 아닌 필요한 부분에만 그리고 락이 필요없는 경우엔 cas로 대체해서 ConcurrentHashMap성능이 좋아짐 (물론 트리도 있고)