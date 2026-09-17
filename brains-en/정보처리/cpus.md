# CPU Scheduling

![Scheduling](./image/스케쥴링.png)

- The OS's task of managing priorities among processes that want to use the CPU => **creating policies on how much resource to allocate to which process**
- A technique to distribute resources as fairly as possible among processes, increasing throughput and CPU utilization, and minimizing overhead, response time, and waiting time
- There are **preemptive scheduling** and **non-preemptive scheduling**.
- Loading multiple processes into memory (multiprogramming), dividing CPU uptime appropriately (time-sharing), and distributing it to each process for execution

<br><br>

## Process State Transitions
  
![State Transition](./image/상태전이.png)
- New
This is the very first transition from New to Ready. A process is created.

- Dispatch
Dispatch refers to the transition from the Ready state to the Running state. This occurs when the job scheduler selects a process to be executed, and the executed process then occupies the CPU.

- Interrupt
Upon receiving an interrupt signal, the currently running process transitions to the Ready state, and a higher-priority process transitions to the Running state. (Each process is assigned a priority, and processes transition to either the Ready or Running state according to their priority.)

- I/O or Event Wait
If a process occupying the CPU needs to perform I/O processing, the running process changes from the Running state to the Waiting/Blocked state. The process that has transitioned to the Waiting state remains there until all I/O processing is complete. As the running process transitions to the Waiting state, another process that was in the Ready state transitions to the Running state. Furthermore, processes in the Waiting state are not assigned priority and cannot be selected by the scheduler.

- I/O or Event Completion
A process that has finished I/O processing transitions from the Waiting state to the Ready state, making it eligible for selection by the scheduler. Additionally, please remember that a process can also go through the Blocked state when it is terminated.

<br><br>

## Non-Preemptive vs. Preemptive Scheduling

### Non-Preemptive Scheduling
- This scheduling guarantees CPU execution until the process voluntarily relinquishes the CPU due to I/O requests or other reasons. It applies to the entire job execution time or a single CPU allocation.
- While it can fairly handle requests for all processes, a process performing a short task might have to wait until a long task finishes. (**convoy effect**)
- Suitable for specific process environments with little variation in processing time

<br>

### Preemptive Scheduling
- In time-sharing systems, this is a scheduling method where a higher-priority process forcibly interrupts the current process and reclaims the CPU, either because its time slice has expired or due to an interrupt or system call completion.
- It has the advantage of relatively fast response times, but processing time is difficult to predict, and it can cause overhead if high-priority processes continuously arrive.
- Useful in situations where high-priority processes need to be processed quickly, such as real-time response times or deadline-driven environments.

CPU Scheduling | Type
---|---
Non-Preemptive Scheduling | FCFS SJF Priority Scheduling
Preemptive Scheduling | RR SRT Multilevel Queue Multilevel Feedback Queue
