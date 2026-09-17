# Hypervisor

A software layer installed on physical hardware that allows a physical machine to be divided into multiple virtual machines.

In other words, it refers to a logical platform for running multiple operating systems simultaneously on a host computer.

- A hypervisor is a program that can host multiple different virtual machines on a single piece of hardware.
- Each virtual machine or operating system can run its own programs because it appears to have the host hardware's processor, memory, and resources. The hypervisor is responsible for allocating these resources to the virtual systems.
- The operating system installed on a virtual machine is called a guest OS, and sometimes an instance.
- The hardware on which the hypervisor runs is called the host machine.
- A **hypervisor management console**, also abbreviated as VMM (Virtual Machine Monitor or Virtual Machine Manager), is computer software that helps easily manage virtual machines.

## Hypervisor Types

### Type 1 Native = Bare-Metal
It runs directly on the host hardware, controlling the hardware and managing guest virtual machines.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FssPGo%2Fbtq8hlvOZBP%2FRH7Ra6CxZNkYNNkQLjjtyK%2Fimg.png)

### Advantages
1. Fault Tolerance: If a physical server fails, the management software quickly migrates instances to another available server, preventing impact on the physical hardware. This allows maintenance and operations staff to perform repairs and replacements at their convenience.

2. RAM Overcommitment / Dynamic Allocation: When running multiple instances on a server, the total RAM allocated to virtual machines can be set to a value greater than the hardware's total physical memory capacity.

e.g., Xen, Oracle VM Server for SPARC, Oracle VM Server for x86, etc.

### Type 2 = Hosted

Type 2, also known as a hosted hypervisor, runs within a conventional OS, just like other applications on the system.

In this case, the guest OS runs as a process on the host, while the hypervisor separates the guest OS from the host OS.

Type 2 relies entirely on the host operating system for its operation. Even if the hypervisor running on the base operating system is secure, any issues with the base operating system will affect the entire system.

### Advantages
1. Since it's installed within the operating system, a hypervisor management console is not required.

### Disadvantages
1. It does not support RAM overcommitment/dynamic allocation, so care must be taken when allocating resources to virtual machines.

e.g., VMware, Workstation, etc.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdCn9NK%2Fbtq8ftasXs8%2FNdZ75zFfNKW91Kvkhzr6qK%2Fimg.png)
