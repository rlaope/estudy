# CPU Out-of-Order Execution, Barrier, Happens-Before

### Out-of-Order Execution

The code we write seems to execute line 1 then line 2, but the actual CPU and compiler **arbitrarily change the execution order** for performance.

This is called out-of-order execution.

The reason is that while the CPU waits to fetch specific data from memory (which is very slow), it's more efficient to process unrelated instructions that follow.

However, there's a problem: while it might not matter in a single-threaded environment if the result is the same, **in a multi-threaded environment**, the moment thread A changes the order of operations, thread B could disastrously read incorrect, i.e., unprepared, data.

For example, let me explain the classic case of **lazy object initialization**.

```java
class Work {
    int data = 0;
    boolean ready = false;

    // 스레드 A가 실행: 데이터 준비
    void writer() {
        data = 42;           // (1) Store (값 쓰기)
        ready = true;        // (2) Store (준비 완료 플래그 쓰기)
    }

    // 스레드 B가 실행: 데이터 사용
    void reader() {
        if (ready) {         // (3) Load (플래그 읽기)
            System.out.println(data); // (4) Load (값 읽기)
        }
    }
}
```

Common sense dictates that if `ready` is true, `data` should naturally be 42. However, in reality, 0 might be printed.

#### CPU and Compiler Tricks

From the CPU's perspective, processing operation (1) `data = 42` might take time to calculate the memory address and write to the cache. However, if operation (2) `ready = true` can be processed immediately, the CPU will execute (2) first for performance.

#### Hardware Differences (Reordering Scope)

- **ARM (Weak Model)**: The CPU can overtly reorder operations (1) and (2), and also (3) and (4). This means it might check the flag before even reading the data, and so on.
- **x86 (Strong Model)**: While the hardware guarantees the order of store-store operations, if code is reordered during JIT optimization, the same problem can ultimately arise.

This is where the concept of a Barrier becomes necessary. When the `volatile` keyword is attached to the `ready` variable, the JVM operates by inserting barriers as follows.

```java
volatie boolean ready = false;
```

For thread A's operation `data = 42`, a standard write operation, a StoreStore barrier is used to warn the CPU not to write `ready` before writing `data`, and then `ready` is written afterwards.

Similarly, in the `if (ready)` check below, a LoadLoad Barrier operates, ensuring `println` happens after the `volatile` read.

### The Nature of the Four Barriers

Instructions that prevent the CPU from reordering operations are called memory barriers.

`LoadLoad`, `StoreLoad`, etc., are types of these barriers, and here, it's easy to think of Load as read and Store as write.

- **LoadLoad**: Ensures that Read2 comes after Read1. Do not perform the next read before the previous read has completed.
- **StoreStore**: Ensures that Write2 comes after Write1. The previous data write must be completed before I write a value.
- **LoadStore**: Ensures that a write comes after a read. Do not overwrite with a new value before all data has been read.
- **StoreLoad**: Ensures that a read comes after a write. This is the strongest and heaviest barrier. Read only after the written data is visible to everyone.

In x86 general PC architectures, they are very smart, so the hardware automatically enforces the other three barriers except for `StoreLoad`. In contrast, ARM (iPhone, MacBook M-chip) ignores these for performance, thus requiring more explicit barriers.

### Happens-Before

Given how complex hardware operations are, developers cannot constantly worry about CPU instructions. Therefore, the Java Memory Model (JMM) created the **Happens-Before** rule.

- **Concept**: If operation A has a Happens-Before relationship with operation B, it's like a legal contract stating that all memory changes made in A must be visible in B.

Key rules include:

1.  **Thread Start**: All operations performed before a thread starts are visible to the started thread.
2.  **Volatile Write/Read**: When writing to a `volatile` variable, subsequent reads by other threads are guaranteed to see the latest value.
3.  **Locking**: Operations performed before releasing a monitor lock are all visible to other threads that acquire the lock.

### What are the benefits of maintaining load-store order?

Strictly maintaining this order is called a **Strong Memory Model**.

**The advantage is predictability.** Since code is visible to others in the order it was written, fewer bugs occur in multi-threaded programming.

**The disadvantage is performance.** Since the CPU gives up opportunities to process faster and waits, the limits of performance optimization become clear.

Conversely, **Weak Memory Models** like ARM allow arbitrary reordering, maximizing power efficiency and performance. Java, through the JVM, bridges this difference, ensuring we get consistent results on any CPU.
