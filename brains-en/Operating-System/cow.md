# Copy-on-Write (CoW) and `fork()` Memory Optimization

### The `fork()` Dilemma

In Linux/Unix environments, `fork()` is the system call used to create a new process.

In principle, when `fork()` is called, all of the parent process's memory space (code, data, heap, stack) must be copied exactly to the child process.

Historically, `fork()` was implemented such that when the command was executed, it would allocate the same amount of physical memory as the parent process and copy all data.

If a parent process uses 2GB of memory, creating a child process would immediately require allocating an additional 2GB of physical memory and copying all data. This leads to enormous memory waste and context switching delays.

### Copy-on-Write CoW

To solve the above problem, the OS introduced the CoW technique, which **performs a copy only when a write operation occurs**.

**Initial Sharing (Space Saving)**: When `fork()` is called, the OS does not copy physical memory. Instead, it only creates a **page table** for the child process, making both the parent and child point to the same physical memory pages.

**Permission Change**: At this point, the access permissions for all shared memory pages are changed to **read-only**. If both processes only read data, they continue to share physical memory in this state.

**Page Fault and Copy (Upon Write)**: If either the parent or child process attempts to **write data** to a specific memory region.

- Since the page is set to read-only, a protection fault occurs in the CPU (MMU), causing the OS to enter kernel mode.
- The OS confirms that this fault is due to CoW and **allocates new physical memory and copies the data only for that specific page (typically 4KB).**
- Afterward, it maps the page table of the process that attempted the write to the new physical page, grants write permission, and then resumes the process.

### Synergy with the `exec()` System Call

In most cases, a process calls the `exec()` system call immediately after `fork()` to completely overwrite its memory space with new program code.

Without CoW, 2GB would have to be copied entirely at the time of `fork()`. If `exec()` were called just 1 millisecond later, all that painstakingly copied 2GB would be discarded, and new data would be loaded, leading to extreme resource waste.

Thanks to CoW, only a very small number of pages that are actually modified (primarily stack variables) need to be copied, allowing processes to be created at high speed.

#### Why does `exec()` discard 2GB?

Understanding the essence of `exec()` might clarify why 2GB is discarded and why CoW is necessary, so I'll elaborate further.

`exec()` is **overwriting the process's internal state**.

The sole role of this system call is to completely clear out the **current process's existing memory space (code, data, heap, stack)** and overwrite it with the source code of an entirely new program read from disk.

It maintains the process's shell, such as its PID, but completely replaces its core (the program).

#### `fork()` -> `exec()` without CoW

Let's imagine a scenario where a parent process is using 2GB of physical memory and wants to create a child process to execute the `ls` command (a new program).

1.  **Brute-force Copy with `fork()`**: The OS allocates an additional 2GB of physical memory for the child process and copies every single bit of the parent's memory to the child. This process wastes enormous CPU resources and consumes memory bandwidth.
2.  **Load New Program (`exec`)**: The child process calls `exec("ls")` to execute the `ls` command.
3.  **Discard 2GB**: `exec` considers the 2GB of data, which was just painstakingly copied, as data that won't be used anyway, so it deallocates all of it. Then, it loads the `ls` program code, which is only a few MBs, into memory.

Ultimately, this results in the waste of copying 2GB, which would be deleted just 1 millisecond later, causing the entire system to lag.

This is because `fork()` originally performed a direct copy. In the early days, `fork()` worked this way. Now, when reading, memory is continuously shared, and the moment a write is attempted, CoW detaches and copies only a single page into physical memory to create independent spaces.
