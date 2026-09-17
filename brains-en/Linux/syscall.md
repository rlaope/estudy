# System Call Interface

In Linux, the System Call Interface is the sole channel for User Space and Kernel Space to utilize each other's functionalities.

When an application needs to access hardware or core OS functionalities, it must do so through a System Call. This process is not a simple function call but a heavy operation involving CPU privilege mode switching and kernel-internal execution.

As we've seen before, the reason system calls are necessary is that user space (general applications) cannot directly access hardware kernel data for security and system stability reasons. For example, direct disk access, network communication, memory allocation, or process creation are not allowed. Since all of these are impossible in User space, system calls are used.

**A system call is the official request process that delegates tasks to the kernel.**

### System Call Invocation Process

Let's assume an application calls `read(fd, buf, size)`.

On the surface, it looks like a simple function, but internally, it operates in a much more complex way, as follows.

#### Invoking libc's Wrapper Function

- In User Space, the `read()` function provided by glibc is called. However, glibc's `read` is not the actual read operation; it's merely a wrapper that places the system call number into a CPU register and raises a request to the kernel.

#### CPU Privilege Escalation

- The `read` function is not a simple function call; it performs the task of switching the CPU from Ring 3 to Ring 0.
- The traditional method uses the `int 0x80` interrupt; modern methods (x86-64) use the `syscall` instruction, and ARM64 uses `svc #0`.
- Although they might look unusual, the moment these instructions are executed, the CPU switches to privileged mode and enters kernel space.

#### Kernel's System Call Dispatcher

The kernel categorizes which kernel function to execute based on the system call number.

For example:

- 0: read
- 1: write
- 2: open
- 39: getpid
- 57: fork
- 63: uname

These number tables are called the system call table, and the entity called the dispatcher invokes `sys_read()`, which is the kernel's internal function for `read`.

#### Kernel Internal Logic Execution

`sys_read` performs the following:

1. Validates if the user memory (`buf`) is valid.
1. This is because it must prevent malicious processes from attempting to read kernel memory.
1. Safely copies data using `copy_from_user()`.
2. Checks if the file descriptor is valid.
3. Accesses the kernel file system (VFS).
4. Requests the disk driver to fetch file data.
5. Copies the result back to the User Space buffer.

All these processes are tens to hundreds of times more expensive than a User Space function call.
