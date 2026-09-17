# Virtual Machine vs. Container

### Virtualization Concept
Virtualization refers to abstracting physical components (HW devices) into logical objects, allowing a single device to operate as if it were multiple devices, or conversely, combining multiple devices to provide them to users as a single shared resource. This makes it a core technology for implementing cloud computing.

Computing resources subject to virtualization include processors (CPU), memory (Memory), storage (Storage), and networks (Network). By virtualizing servers or devices composed of these, a high level of resource utilization (physical servers 10-15%, virtualization 70% or more) and distributed processing capabilities can be provided.

![](./image/가상화.jpeg)

<br>

### Virtual Machine

A replicated computing environment implemented through virtualization.

**Operational Objectives**
1. Running multiple types of operating systems or protocols simultaneously on a single piece of hardware.
2. Dividing a single hardware resource among multiple users.
3. Ensuring independence between divided systems through virtualization, without mutual interference.

### Hypervisor
An intermediary manager that manages shared computing resources and controls virtual machines.

### Container

Refers to a modular and isolated computing space or environment, which breaks free from system environment dependencies and operates stably.

Background of container technology: To solve the problem of developed programs causing various unexpected errors due to changes in the execution environment (= increased portability between computing environments).

<br>

### Differences between Virtual Machines and Containers

1. Containers do not require a hypervisor and guest OS in virtualization.
2. Instead, containers isolate processes at the OS level and run as modular program packages.
3. Therefore, containers are lighter and faster than virtual machines.
4. This makes it possible to run more applications more easily on a single physical server.

![](./image/containers-101.png)
