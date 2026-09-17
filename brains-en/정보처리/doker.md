# Docker and Kubernetes

<br>

### Docker

**Container-based open-source virtualization platform**
- Allows fast deployment by splitting a large application into service units
- Containers do not affect each other
- Docker is a technical concept and a tool, a technology that launches and runs images in containers
- Optimized for managing a single container

> `Image `: Contains files and configuration values required for container execution
> `Container`: A technology where processes run in an isolated space

<br>
  
**Differences from Virtual Machines**
![doker](./image/doker.jpeg)
  
- `Virtual Machine` : Server - VMs on top of Hypervisor
- `Container` : Server - Host OS - Docker Engine - Containers
  
**Advantages**
1. Efficient and fast to deploy because it allocates only the necessary resources to containers and shares resources with the operating system
2. Furthermore, containers with different environments can be set up on the same server, and each container is independent!

<br>


### Kubernetes
**An open-source platform that automates Linux container operations and an orchestration tool for managing Docker**  
- A tool for managing Docker
- A service that manages containers based on Docker
- Optimized for managing multiple containers as service units

> Orchestration: A system that manages and coordinates the execution of multiple containers (scheduling, clustering, service discovery, load balancing, logging, and monitoring, etc.)
Other orchestration tools - Docker Swarm, EC2, Nomad, etc.

<br>

### Summary
`Docker` : A tool for launching and running a single image in a container
`Kubernetes` : A service that manages containers based on Docker
