# Istio, Istio Architecture

Istio is a service mesh, a modernized service networking layer that provides a transparent, language-independent way to flexibly and easily automate application network functions.

It is a widely used solution for managing the various microservices that make up cloud-based applications.

The Istio service mesh also supports how these microservices communicate and share data with each other.

As organizations accelerate their move to the cloud, they are also modernizing applications as needed.

However, transitioning monolithic legacy apps to cloud-based apps can create challenges for DevOps teams.

Developers need to learn how to assemble apps using loosely coupled microservices to ensure mobility in the cloud.

At the same time, operations teams must manage new cloud-based apps within increasingly large hybrid and multi-cloud environments.

Istio enables these tasks.

<br>

## Istio
An open-source service mesh that helps organizations run distributed microservice-based apps anywhere.

- Adopting Istio can reduce the complexity of Kubernetes.
- It is a technology that helps easily configure network connections for each app in the distributed network environment of a microservice architecture.
- Istio is an open-source solution that uses Envoy as its Data Plane and controls it.

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2Fc747d893-511e-4261-9b0c-8eb0a4795267%2Fimage.png)

### Benefits of Istio Services
- Achieve consistent service networking
- Secure services through Istio benefits
- Improve application performance

> Istio is the most widely used open-source solution for network (service mesh) management in Kubernetes environments.

## Istio Architecture

All components are combined into a single process called istiod, Mixer is removed, and Pilot also performs Mixer's functions.

> istiod: A unified process of Pilot and Mixer, which are components of Istio
> Pilot: One of Istio's components, responsible for service discovery and load balancing
> Mixer: One of Istio's components, providing functions such as communication pattern analysis, policy enforcement, and telemetry collection

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2F32f0102e-e439-455b-bb78-eb7dd3196534%2Fimage.png)

<br>

## How Istio Implements a Service Mesh

The network was configured by adding proxy-style containers.

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2F7e634fe8-2879-47f6-ad3b-4021fd5b578a%2Fimage.png)

Istio Internal Configuration Method

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2Fccb28f59-4c61-4c61-8395-1b45a5870bb5%2Fimage.png)

-> Easily collect/visualize logs from all apps

<br>

## Istio's Core Internal Components

Istio consists of a Data Plane and a Control Plane.

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2Fefe5dd49-05a9-4a92-bf94-b2052ece325e%2Fimage.png)

### Data Plane
The part that actually receives and processes traffic.

Refers to proxies configured as Sidecars for services.

The Data Plane is controlled by the Control Plane.

Istio uses Envoy as a sidecar proxy.

> Sidecar: An auxiliary application that works alongside a primary application or service (logging, monitoring, authentication)
> Envoy: Acts as an intermediary for communication between clients and servers and enhances security.

### Envoy
- A lightweight L7-only proxy.
- Supports protocols such as HTTP and TCP.
- Supports features such as Circuit Breaker, Retry, and Timeout.

> Circuit Breaker: A pattern to prevent service failures due to faults in distributed systems
> Retry: A pattern to retry service calls in failure situations in distributed systems
> Timeout: A pattern to cancel a service call if it does not respond within a certain period in distributed systems

## Control Plane
Refers to the components that control the Data Plane.

It consists of Pilot, Mixer, Citadel, Galley, and other components.

### Pilot
- Manages configurations for Envoy.
- Provides Traffic Management features.
  - Service Discovery (the task of discovering Envoy Endpoints)
    - Traffic Retry
    - Circuit Breaker
    - Timeout

### Mixer
- Manages access control and policies across the entire Service Mesh.
- Collects monitoring metrics.

### Citadel
- A module responsible for security-related functions.
- Manages authentication features (e.g., TLS Certification)

### Galley
- Checks Istio configuration.
- Converts Kubernetes YAML files into a format Istio can understand.
