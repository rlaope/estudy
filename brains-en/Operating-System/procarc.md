# Process Architecture

Early computers would load and execute only one program in memory.

However, when attempting to run multiple programs concurrently (though this issue could also arise with a single program, it primarily emerged then), a problem known as memory collision occurred.

1. Code (instructions) and data getting mixed, leading to memory overwrites.
2. Collisions in variable or function addresses.
3. System errors caused by invading other programs' memory.

To solve this, the OS provided each process with an independent memory space, and to manage that space efficiently, it divided it into code, data, stack, and heap areas.

As mentioned above, a single running process typically consists of the following main areas.

1. Code Area: Stores machine instructions to be executed. It is read-only and can be shared among multiple processes.
2. Data Area: Stores global variables, static variables, etc. It is initialized at program startup and has a fixed size.
3. Heap: Stores dynamically allocated memory (e.g., via `new`, `malloc`). Its size changes at runtime and requires manual deallocation.
4. Stack: Stores local variables and parameters during function calls. It follows a LIFO structure and is automatically deallocated upon termination.

### Code Area

Compiled machine instructions are stored here. Being read-only prevents bugs where code is accidentally overwritten. Examples include function definition code.

### Data Area

Global variables like `int a = 10;` or static variables are stored here.

This data is loaded once at program start and persists until termination.

In other words, it can be seen as memory shared across the entire program.

### Heap Area

This is the space where objects created by dynamic allocation, such as `malloc()` or `new`, reside.

Programmers can specify its size directly and must deallocate it using `free` or `delete`.

Otherwise, memory leaks will occur.

### Stack Area

Local variables, parameters, and return addresses are stored here during function calls.

It's easy to manage as it's automatically deallocated when the function ends.

However, its size is limited, so be careful as very deep recursion or declaring large arrays can lead to a stack overflow.

**Consequently, this structure resolved the issue of inter-process collisions.**

The problem of code and data mixing and invading memory was solved by separating the code and data areas.

Lifecycle conflicts between global variables and function local variables were distinguished by data vs. stack.

The difficulty in managing data whose size changes at runtime was addressed by introducing the heap to support dynamic memory.

Conflicts in variable storage locations during multiple function calls were managed by the stack structure, which controls the call order.

In conclusion,

1. Memory protection
2. Efficient memory usage
3. Establishment of a stable program execution structure
4. Automation of function call and local variable management

This structure was born to achieve these goals.

Looking at this, I initially thought the problem statement was about preventing conflicts between different processes, but seeing these benefits, doesn't it seem like internal conflicts within a single process were also resolved?

Even with independent memory spaces for each process, issues arose where code, data, stack, and heap within a single process could become entangled.

For example, if many function calls occurred, the stack could grow and invade the heap area, or if global variables and code mixed, it could lead to code overwrites.

Therefore, the principle of distinguishing memory areas by their roles even within a process itself was applied, leading to the creation of the code, data, heap, and stack structure.

These issues could sufficiently occur even with a single process but didn't manifest often. It was by running multiple processes that the underlying problems, which could also occur with just one, were discovered.

After breaking down the initial problems that occurred with only one process by separating the structure,

they were resolved with OS-level technologies like virtual memory, MMU, and process isolation.

Ultimately, the problem of multiple processes colliding was resolved by the operating system through other means.
