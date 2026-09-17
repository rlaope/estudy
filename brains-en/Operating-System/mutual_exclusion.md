# Spinlock, Mutex, Semaphore + Mutex vs. Binary Semaphore

### What is a Race Condition?
A situation where the outcome can vary depending on the timing or access order when multiple processes/threads manipulate the same data concurrently.

### What is a Critical Section?
A code / part / region where a race condition can occur.

### What is Mutual Exclusion?
Mutual exclusion is a mechanism that allows only one process/thread to enter and execute within a critical section to ensure the consistency of shared data.

<br>

## Spinlock

A spinlock is a method of checking for a lock with the help of the CPU.

Simply put, it's a method where a thread continuously checks if it can acquire the lock, rather than waiting until it can occupy the lock.

Context switching does not occur, but the disadvantage is that it consumes a lot of CPU operations.

### TestAndSet
It uses the CPU's atomic instruction called TestAndSet.

- TestAndSet cannot be interfered with or interrupted during execution.
- It cannot be executed simultaneously for the same memory region.

<br>
## Mutex
Unlike a spinlock, a mutex involves acquiring a lock before entering the critical section. Once one thread acquires the lock and enters the critical section, if another thread tries to access it, that thread is put into a waiting state and placed in a queue.

Then, when the thread holding the lock finishes its work and releases the lock, another thread from the queue acquires the lock and performs its task.

### Is a mutex always better than a spinlock?
Not necessarily. A mutex involves context switching when changing the working thread.

Therefore, in a multi-core environment, if the work within the critical section finishes faster than a context switch, a spinlock can be more advantageous than a mutex.

However, it's rare for critical section work to finish faster than a context switch, so mutexes are generally preferred.

<br>

## Semaphore

A device that allows one or more processes/threads with a signal mechanism to access a critical section.

Semaphores can hold multiple values, not just 0 and 1.

In a semaphore's wait operation, the current lock's value is decremented by 1, and when signal is called, the value is incremented by 1. If the current semaphore's value is 0 or greater, the task can be executed.

Semaphores can be used to define an order.

Semaphores allow different threads/processes to manipulate the signal.

<br>

## Mutex vs. Binary Semaphore

While they might seem similar, there are differences.

A mutex allows only the thread that acquired the lock to release it, but a semaphore does not.

Also, a mutex has the property of priority inheritance, whereas a semaphore does not.

### Priority Inheritance
The CPU uses various methods for scheduling tasks.

Let's take an example using priority scheduling.

Suppose there are Process 1 and Process 2, and Process 1 has a higher priority.

If Process 2 holds a lock and the CPU allocates work to Process 1, Process 1 will have to wait until Process 2 finishes its work.

To resolve this quickly, the priority of the process holding the lock is raised.

This operation is called priority inheritance.

This means that even if you assign priorities as desired, the order cannot be guaranteed.

Therefore, using a semaphore can guarantee the order.

In conclusion:

- If only mutual exclusion is needed, use a mutex.
- If synchronization of execution order between tasks is needed, a semaphore is recommended.
