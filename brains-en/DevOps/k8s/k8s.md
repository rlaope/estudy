# Kubernetes

Kubernetes is an open-source **container orchestration platform** that automates many of the manual processes involved in deploying, managing, and scaling containerized applications.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbLZ8uO%2Fbtq3zm09FZP%2FMU11nVoRgob7mrVkp8gKXk%2Fimg.png)

### Container Orchestration
It is a technology that automatically deploys, manages, and scales multiple containers.

This enhances the availability and scalability of applications. (Other orchestration tools besides Kubernetes: Docker Swarm, Apache Mesos, etc.)

<br>

## Kubernetes Cluster

Groups of hosts running Linux containers can be clustered together, and Kubernetes allows for easy and efficient management of these clusters.

Kubernetes clusters can extend hosts across on-premises, public, private, or hybrid clouds.

For these reasons, Kubernetes is an ideal platform for hosting cloud-native applications that require rapid scaling, such as real-time data streaming via Apache Kafka.

<br>

## Why is Kubernetes necessary?

Using Kubernetes enables the elastic execution of containerized application environments.

In a production environment, you need to manage the containers running your applications and ensure there is no downtime.

For example, if a container goes down, another container must be restarted to minimize downtime.

What if these actions could be handled by a system?

It is Kubernetes' role to ensure these issues are **managed by the system**.

<br>

## Features provided by Kubernetes

Let's briefly look at the features Kubernetes provides.

- Service discovery and load balancing - Expose containers using DNS names or their own IP addresses.
- Storage orchestration - Automatically mount your desired storage system, such as local storage, public cloud providers, etc.
- Automated rollouts and rollbacks - Describe your desired state and change the current state to the desired state at a controlled rate.
- Automated bin packing - Provide each container with the CPU and memory it needs.
- Automated self-healing - Restart failed containers and replace containers.
- Secret and configuration management - Store and manage sensitive information like passwords, OAuth tokens, and SSH keys.
