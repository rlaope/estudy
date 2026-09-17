# [OS] Operating System Structure and Principles

## What is an Operating System?
> It is software that manages computer hardware (I/O devices, CPU) resources while simultaneously
> providing an environment for multiple applications to operate.

- The OS allocates necessary resources when programs require them.
- The OS controls programs that have been allocated resources to operate efficiently without interfering with each other.
- What an operating system does
  - Process management
  - Memory management
  - Storage management
  - Security

<br>
<br>

## Operating System Boot Process
When the computer is powered off, the CPU reads the contents stored in ROM. ROM is non-volatile memory that does not disappear even when the computer is powered off, and it stores information that needs to be continuously preserved. `POST` and the `bootloader` are stored within ROM.

POST is the first program executed when the computer is powered on, checking for any issues with the computer. The bootloader retrieves the OS program stored on the hard disk and transfers it to RAM.

> ### Bootloader
> Refers to a program that executes before the operating system starts up, completing `all necessary related tasks for the kernel to start correctly` and ultimately aiming to launch the operating system.

<br><br>

## Interrupt
Once the computer boots up, it executes the OS program and then waits for user input events. These events are called interrupts. When an interrupt occurs, the OS remembers the address of the current instruction. After the interrupt is handled, it returns to that address to execute the next instruction or goes back to a waiting state.

> ### Interrupt
> An interrupt literally means `interruption`, and it causes the CPU to pause its current program execution in various exceptional situations (such as `errors`, `input`, etc.) from I/O hardware or other devices, allowing the CPU to handle them.
> When an interrupt occurs, the task the CPU was performing is saved separately (e.g., in `CPU internal registers` or `main memory`), and the task that caused the interrupt is executed first.

<br><br>

## Operating System Structure
- Operating systems have continuously evolved since the DOS era.
- Fundamentally, a single CPU can only process one task at a time.
- Therefore, in the past, other tasks would simply wait until one task was finished, leading to low efficiency.

### Multiprogramming
This methodology emerged to solve such problems. The basic concept of multiprogramming is to allocate the CPU to another task for processing when one task is performing I/O. Conversely, when the CPU is performing a task, I/O devices are put into a waiting state, and I/O resources are allocated to tasks that require I/O.

### Multitasking
As technology advanced, processor speeds increased, but paradoxically, the waste of CPU resources grew larger. This is because I/O devices could not keep up with the increasing speed of processor processing, leading to greater processor waste.

The concept that emerged to solve this problem is `multitasking`. Multitasking assigns a `resource allocation time` to each task, and once that time expires, it `hands over resources to other tasks.` At this point, allocating time to each resource is `scheduling`, and there are `Job scheduling` and `CPU scheduling`.

<br>
<br>

## Operating System Principles
- The operating system is an `interrupt-driven` structure where, when a user request occurs, the OS appropriately distributes resources to process that request.

- Interrupts are broadly divided into two types: `H/W interrupts` and `S/W interrupts`.
  - `H/W interrupt` : Interrupts related to I/O, memory, and CPU
  - `S/W interrupt` includes Errors that can occur during program execution, and System Calls which are requests for operating system services.
