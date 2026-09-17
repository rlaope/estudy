# Safepoint and Polling Page

### Safepoint

A safepoint refers to a point (or duration) where all running Java threads are temporarily paused in a safe state to perform an **internal global operation within the JVM**.

**What is commonly referred to as STW (Stop-The-World) is the state where all these threads have reached this Safepoint and are paused.**

#### Why is pausing necessary?

The most representative reason is GC. If GC is cleaning up heap memory or relocating objects, and application threads continue to run and write values to old memory addresses, the application's data would be completely corrupted.

Besides this, pausing all threads is necessary when performing tasks such as Deoptimization (discarding JIT-compiled code and reverting to the interpreter) or generating a Thread Dump (extracting the Call Stack of all threads).

#### What makes it safe?

Threads should not be forcibly stopped at any time at the OS level. At the precise moment a thread is paused, the JVM must be able to fully identify which values in CPU registers or the stack are actual object memory addresses (pointers) and which are simple numerical data (int, long).

A safepoint is reached when a thread arrives at a point where such pointer mapping information (which HotSpot calls `OopMap`) is perfectly prepared, allowing the GC to accurately track all references without confusion. Pausing at this point is a safe pause.

### Polling Page

So, how does the JVM get those numerous threads to pause at a safepoint on their own, without forcibly shutting them down?

This is where the Polling Page comes in.

The essence of a Polling Page is a memory page (typically 4KB in size) that acts as a global traffic light, allowing all threads to frequently check (poll) whether a safepoint request has arrived, with very low overhead.

#### Why is it needed and how does it work?

If a thread is running an infinite loop like `while(true)` and performing intensive mathematical calculations, it can never reach a safepoint unless an external signal tells it to stop.

Therefore, when the JIT Compiler translates code into machine language, it inserts checking code (Polling Code) that asks "Has a safepoint request arrived?" at the end of each loop iteration or method.

- **Inefficient Method (Software Check)**: if (safepoint_requested == true) { pause }
  - If such an if statement is inserted in every loop, the program's execution speed slows down due to conditional branching.
- **Hotspot's Elegant Method (using polling page)**
  - Normally, during initialization, the JVM creates a memory region called a polling page and sets it to a readable state.
  Threads peek at this page (`Load` instruction) every time they loop. Since it's simply readable, they continue their work at high speed without any issues, and the overhead is close to zero.
  - When a Safepoint is triggered, and the JVM master thread (VM Thread) needs to stop all threads, it closes the OS permission for this polling page, setting it to non-readable (PROT_NONE).
  - When application threads try to peek at it as usual, a Page Fault, specifically a `SIGSEGV` error, occurs because it's memory inaccessible at the hardware level.
  - When the OS notifies the JVM of this error, the JVM C++ signal handler recognizes it not as a genuine error but as a safepoint stop signal, and safely blocks (waits) the corresponding thread.

In summary, a safepoint's purpose is a **perfectly paused state** where the JVM can accurately understand thread states for internal operations.

A polling page is the fastest and most optimized hardware-based trap (trigger) that guides threads to immediately fall into that destination when the JVM desires, without disrupting their execution flow.
