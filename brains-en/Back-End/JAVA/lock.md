# ReentrantLock, Condition

Let's learn how to synchronize threads using the ReentrantLock class, one of the lock classes in the `java.util.concurrent.locks` package.

### ReentrantLock

As a general lock, similar to `wait()` & `notify()` in `synchronized` blocks, `await()` & `signal()` can be used to release a lock under specific conditions and later reacquire it to perform subsequent operations.

While `synchronized` blocks are convenient because locks are automatically acquired and released, the ReentrantLock class offers various advanced features.

- Lock polling
- Can be used when the code extends beyond a single block.
- Timeout specification
- Allows selective notification of waiting threads by applying Conditions.
- Can interrupt threads in the waiting pool that are trying to acquire a lock.

ReentrantLock can be used when the above features are required.

```java
//Constructor 
ReentrantLock()
ReentrantLock(boolean fair)

//Method
void lock() // Acquires the lock
void unlock() // Releases the lock
boolean isLocked() // Checks if the lock is held
boolean tryLock() // Lock polling
boolean tryLock(long timeout, TimeUnit unit) throws InterruptedException
```

Unlike `synchronized` blocks, where lock acquisition and release are managed automatically, ReentrantLock requires explicit acquisition and release of the lock.

A typical `lock()` call blocks the thread until the lock is acquired, which can incur overhead due to context switching. However, if the critical section's execution time is very short, `tryLock()` can enable efficient locking through lock polling (spin lock). (Conversely, it should not be used if the waiting time is long.)

By setting a timeout for `tryLock()`, you can decide whether to retry the operation or give up if the lock is not acquired within the specified time.

```java
// Basic usage
class TestClass{
	private ReentrantLock lock = new ReentrantLock(); // Create Lock
    
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

`synchronized`'s `wait()` & `notify()` could not distinguish between thread types and would put them all together in the shared object's waiting pool, making selective notification impossible. However, using ReentrantLock and Condition allows threads to wait separately in distinct waiting pools based on their type, enabling selective notification.

```java
private ReentrantLock lock = new ReentrantLock(); // Create lock
// Create Condition from lock
private Condition forTask1 = lock.newCondition();
private Condition forTask2 = lock.newCondition();
```

<br>

Generally, ReentrantLock is measured to have better performance than `synchronized` blocks. Therefore, using ReentrantLock is recommended. However, unlike `synchronized`, you must explicitly release the lock. If you accidentally fail to release it (a code-level problem), it can lead to critical performance issues (infinite lock acquisition waiting).

If you are not familiar with the Lock interface or if `synchronized` blocks are sufficient for your requirements, it is advisable to use `synchronized` blocks.
