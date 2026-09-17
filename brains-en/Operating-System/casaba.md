# CAS and the ABA Problem

Mutexes or semaphores require other threads to sleep in a waiting queue if someone holds the lock. (Context switching occurs -> significant overhead)

To solve this, lock-free synchronization, specifically CAS, emerged.

CAS is an atomic instruction provided by the CPU, and its operational logic is as follows.

1. Read the value from memory location V. (My expected old value E)
2. Attempt to update with the new value N that I calculated.
3. At this point, **compare whether the current value of memory V is still the same as the expected value E that I read earlier.**
4. If they are the same, it means nothing was touched, so replace it with the new value.
5. If they are different, it means another thread intercepted and changed the value while I was calculating, so abandon the update. (After failure, it usually retries from the beginning.)

This process is guaranteed to execute in one go at the CPU level without interruption.

```c
// CAS 명령어의 논리적 동작을 C 코드로 표현한 의사코드 (실제론 하나의 CPU 명령어로 동작)
bool CAS(int *memory_location, int expected_value, int new_value) {
    if (*memory_location == expected_value) {
        *memory_location = new_value;
        return true;  // 성공
    }
    return false; // 누군가 중간에 값을 바꿨음! 실패!
}
```

### ABA Problem

CAS operates on the premise that if the value is the same as before, nothing was touched, so it proceeds with the update. This premise can be completely wrong.

**Because just because the value is the same doesn't mean nothing was touched.**

1. Thread 1 reads value **A** from a shared variable and tries to change it to C.
2. Suddenly, Thread 1 is stopped by the OS scheduler. preempted
3. Thread 2 intervenes and changes the shared variable's value from A to B.
4. Thread 2 restores the variable's value from B back to A.
5. Thread 1 wakes up and executes CAS, checking if the current value is indeed A, which it read earlier. Based on the current state, it's A, so it changes it to C.

If it were just a number counter, it might be fine to proceed since the values are the same. But what if data structures like List or Stack, which deal with pointer memory addresses, had this problem?

What if the memory pointed to by address A was deallocated in the interim and reallocated to a different address? If only the superficial label A is the same, but the content has become completely corrupted garbage, CAS would let it pass.

The most common solution to prevent this problem is to **attach sequential versioning or a timestamp as a tag to the value.**

Instead of just checking if it's A, it checks if it's A and version 1. This is called Double-word CAS.

- Thread 1 reads A v1
- Thread 2 changes it to B v2, then back to A v3.
- The awakened Thread 1 attempts CAS, sees that the current value is A v3, not A v1, and CAS fails.

### Limitations of CAS

CAS is a powerful mechanism that allows concurrency control without using locks, but it's not a panacea.

Especially when contention between threads becomes severe, there are critical drawbacks.

Beyond ABA, let's look further:

#### CPU resource waste: busy waiting

The CAS instruction itself either succeeds or fails once and then finishes.

Therefore, to update the desired value, CAS must be called repeatedly within a `while` loop until it succeeds, which is known as a spinlock.

The problem is, what if 100 threads try to modify a single variable simultaneously? Only 1 succeeds, and 99 fail. These 99 threads don't enter a waiting state but instead occupy 100% of the CPU, continuously spinning in a `while` loop and retrying CAS. This can lead to significant CPU cycle waste.

#### Cache consistency traffic congestion in multi-core environments: cache ping-pong

Modern multi-core CPUs have high-speed L1/L2 caches for each core. Whether a CAS instruction fails or succeeds, the very attempt to compare and modify a memory value generates massive traffic on the CPU's internal bus.

**The problem is** that when multiple cores attempt CAS on the same memory address, the hardware, to maintain data consistency, forcibly invalidates the cache data of a specific core and transfers data to other cores. This process, where data bounces back and forth between cores like a ping-pong ball, is called cache ping-pong, and in severe cases, it can saturate memory bandwidth, slowing down the entire system.

#### Starvation and unfairness

In a starvation scenario, OS-managed mutexes or semaphores can usually ensure a certain degree of fairness by creating a waiting queue and allocating locks in order.

The problem is that a CAS loop has no order; instead, the thread whose value matches at the opportune moment of executing the CAS instruction "wins." A particular unlucky thread might continuously fail CAS because other threads keep changing the value, leading to an indefinite starvation state where it can never update.

### Limitation of single variable (address) manipulation

This is the most fundamental structural limitation: the CAS instruction provided by hardware guarantees atomicity for only a single memory address at a time, typically 32 or 64 bits in size.

The problem is that if you need to simultaneously update two separate memory regions, such as withdrawing from account A and depositing into account B in a bank transfer, a single general CAS instruction cannot easily guarantee atomicity.

To solve this, one must design extremely complex lock-free data structures or introduce advanced techniques like Software Transactional Memory, which drastically increases implementation difficulty.
