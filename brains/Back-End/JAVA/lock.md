# ReentrantLock, Condition

`java.util.concurrent.locks` package의 lock 클래스중 ReentrantLock 클래스를 사용해 스레드를 동기화 하는 방법을 알아보자

### ReentrantLock

일반적인 lock으로 synchronized 블럭의 wait() & notify() 처럼 await() & signal()을 이용해서 특정 조건에서 lock을 풀고 나중에 다시 lock을 얻어 이후의 작업을 수행할 수 있다.

synchronized 블럭은 자동으로 lock이 잠기고 풀리기 때문에 편리하지만, ReentrantLock 클래스를 이용하면 다양한 고급 기능이 사용이 가능하다.

- Lock polling 
- 코드가 단일 블록 형태를 넘어서는 경우 사용 가능
- 타임 아웃 지정
- Condition 적용으로 대기중인 쓰레드를 선별할 수 있다.
- lock획득을 위해 wating pool에 있는 스레드에게 Interrupt를 걸 수 있다.

위의 기능이 필요할때 ReentrantLock 사용이 가능하다.

```java
//Constructor 
ReentrantLock()
ReentrantLock(boolean fair)

//Method
void lock() // lock 잠금
void unlock() // lock 해제
boolean isLocked() // lock이 잠겼는지 확인
boolean tryLock() // lock polling
boolean tryLock(long timeout, TimeUnit unit) throws InterruptedException
```


자동으로 lock의 잠금이 해제가 관리되는 synchronized 블럭과 다르게, 명시적으로 락을 잠그고 해제한다.

일반적인 lock()은 얻을 때 까지 스레드를 block 시키므로 context switching에 따른 overhead가 발생할 수 있지만 critical section의 수행 시간이 매우 짧을 경우에는 tryLock()을 통해서 lock polling(spin lock)을 통해 효율적인 Locking이 가능하다. (반대로 대기시간이 길다면 사용하면 안됨)

tryLock()에 시간을 설정해, 지정된 시간동안 lock을 얻지 못하면 다시 작업을 시도할 것인지 포기할 것인지 결정이 가능하다.

```java
//기본적인 사용법
class TestClass{
	private ReentrantLock lock = new ReentrantLock(); // Lock 생성
    
    public testMethod(){
    	lock.lock();
        try{
        	//Critical Section
        } finally {
        	lock.unlock();
        }
    }
}
```

<br>

### Condition

synchronized의 wait() & notify()는 스레드의 종류를 구분하지 않고 공유 객체의 wating pool에 같이 몰아 넣어 선별적인 통지가 불가능했다. 그렇지만 ReentrantLock과 Condition을 사용하면 스레드의 종류에 따라 구분된 wating pool에서 따로 기다리도록 하여 선별적인 통지를 가능하게 한다.

```java
private ReentrantLock lock = new ReentrantLock(); // lock 생성
// lock으로 Condition 생성
private Condition forTask1 = lock.newCondition();
private Condition forTask2 = lock.newCondition();
```

<br>

일반적으로 synchronized 블럭보다 ReentrantLock이 더 성능이 좋게 측정되어 있다. 그렇기 때문에 ReentrantLock을 사용하는 것을 권장하지만, synchronized와 다르게 직접 lock을 해제해줘야 해서 실수로 해제하지 않거나 그렇게 된다면(code level problem) 성능에 치명적인 (무한 락 취득 대기) 문제가 발생할 수 있다.

Lock 인터페이스에 익숙하지 않거나 요구사항상 synchronized 블럭으로도 충분히 커버가 가능하다면 synchronized 블럭을 사용하도록 하자.