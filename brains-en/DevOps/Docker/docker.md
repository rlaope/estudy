### Doker (Doker)

- One of many projects that support container technology
- Container technology existed before, but it became widely known because of Doker.
- De facto standard for container technology
- Ranked 2nd most popular cloud open source in 2014, announced by the Linux Foundation
- Usable on various operating systems
- Simplifies building, deploying, and running by packaging not only applications but also dependencies and file systems.
- Virtualization using kernel features like Linux namespaces and cgroups

![](image/도커.png)

Doker can be used with various cloud service models.
- Image: A single file created after installing necessary programs, libraries, and source code.
- Container: A virtual environment created by isolating and running an an image in an independent space.

![](./image/가상환경.png)

<br>

### Containers Solve It!
- Software components running on the same system often conflict or have various dependencies.
- Containers are a technology that isolates each microservice using virtual machines.
- Containers can run very fast because they don't implement all hardware like virtual machines.
- If a process issue occurs, the entire container must be adjusted, so it's best to run a single process per container.

![](./image/컨테이너.png)

By utilizing space not needed by the hypervisor, more resources can be invested in applications.

### Container Performance Comparison

![](./image/성능%20비교.png)

GFLOPS (GPU Floating point Operations Per Second) for Native, VM, and Container

GFLOPS (GPU Floating point Operations Per Second) is a unit primarily used to numerically represent computer performance.

![](./image/성능비교2.png)

### Technologies for Container Isolation
Linux Namespaces: Provides each process with an independent view of the system for file system mounts, networks, users, hostnames, etc.

![](./image/격리.png)

Linux Control Groups: Limits the amount of resources a process can consume.

![](./image/격리2.png)

### Limitations of Doker
As services grow, the number of containers to manage increases rapidly. Even with Doker, management becomes difficult, and deployment and container placement strategies like scale-in and scale-out are challenging.

![](./image/도커한계.png)
