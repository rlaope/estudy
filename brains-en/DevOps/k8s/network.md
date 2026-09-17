# Kubernetes Network

### Goal
- Pod communication: Network configuration via CNI
- Service types: Differences and operating principles of ClusterIP, NodePort, LoadBalancer
- kube-proxy: Differences and reasons for choosing between iptables and IPVS modes
- CoreDNS: Service discovery in K8s

## Pod Communication - Network Configuration via CNI

In K8s, pods are assigned unique IP addresses, allowing them to communicate directly with other pods within the cluster.

The CNI (Container Network Interface) plugin is responsible for this network configuration.

**CNI**
- Role of assigning unique IP addresses to Pods
- Creates a virtual Ethernet interface (veth) in the pod's network namespace and connects it to the host network.
- Routing configuration: Configures routing for communication with other Pods within the cluster.
- Network policy application: Applies policies for network access control.

Pods communicate directly via veth pairs within the same node, and between different nodes, they communicate through an overlay network configured by the CNI plugin. For example, Flannel uses VXLAN to set up an overlay network.

<br>
## ClusterIP, NodePort, LoadBalancer

A K8s Service provides network abstraction for a set of Pods, offering stable network access.

Let's explore the main service types.

### ClusterIP (Default)
- Assigns a virtual IP accessible only from within the cluster.
- Used for communication between internal services.
- Operating principle: kube-proxy uses iptables or IPVS to route traffic coming to the ClusterIP to the corresponding pod.

### NodePort
- Opens a specific port on each node, allowing external access.
- Used when accessing services inside the cluster from outside.
- Operating principle: Traffic coming through a NodePort is internally forwarded to the ClusterIP and then routed to the corresponding Pod.

### LoadBalancer
- Provisions a cloud provider's load balancer to expose the service externally.
- Used for handling external traffic in cloud environments.
- A LoadBalancer internally creates a NodePort and ClusterIP, and the external load balancer forwards traffic through the NodePort.

<br>

## kube-proxy iptables, IPVS

kube-proxy runs on each node and is responsible for routing network traffic related to Services to the appropriate pods.

### iptables
- Routes traffic using iptables. (target: finds the destination by iterating through destination lists)
- Simple to configure and suitable for small-scale clusters.
- Performance degradation can occur with a large number of services.

### IPVS
- Routes traffic using the Linux kernel's IPVS. (hash-based algorithm)
- Provides high performance and scalability, supporting various load balancing algorithms.
- Configuration can be complex and requires kernel modules.

> + There is also the Cilium eBPF method. (a method that implements operations by writing kernel-level code)

<br>
## CoreDNS: Role of Service Discovery in K8s

CoreDNS is a DNS server that provides DNS-based service discovery within a K8s cluster.

- Service name resolution: Processes DNS queries in the format `service.namespace.svc.cluster.local` and returns the ClusterIP of the corresponding Service.
- Pod name resolution: Returns the IP of individual Pods when using Headless Services.
- External domain forwarding: Forwards domain queries outside the cluster to external DNS servers.

**Operating Principle**
- CoreDNS communicates with the K8s API server to collect Service and Pod information.
- Provides responses to DNS queries based on the collected information.
- It has a plugin-based architecture, allowing for the addition of various functionalities.

<br>

### Summary

Pod communication is supported by CNI plugins assigning unique IPs to pods and configuring an overlay network.

ClusterIP is used for internal service communication, NodePort for external access, and LoadBalancer for external exposure via a cloud load balancer.

kube-proxy's iptables mode is suitable for simple configurations, while IPVS provides high performance and scalability.

CoreDNS provides DNS-based service discovery, allowing services and pods within the cluster to be accessed by name.
