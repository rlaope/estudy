# Node Components

Let's learn more about them this time.

Node components refer to the **elements required to run and manage pods and containers on each node.**

They exist not only on worker nodes but also on master nodes.

Control plane components, which are necessary for cluster control, also run as individual pods, so a tool to manage them is needed.

### kubelet

It is an agent that **orchestrates containers within pods to run correctly on each node of the cluster.**

When the master node's scheduler assigns a pod to a node, **kubelet** ultimately places the containers for that pod.

It also **periodically checks the status of pods and containers and sends the results to the API server.**

> When building a cluster with kubeadm, kubelet is not included, so it must be installed separately, and the versions of kubeadm, kubectl, and kubelet must be matched to avoid compatibility issues.
>
> [kubernetes.io: Installing kubeadm, kubelet and kubectl](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/install-kubeadm/?ref=seongjin.me#installing-kubeadm-kubelet-and-kubectl)

<br>

### kube-proxy

**It is Kubernetes' network proxy that runs on each node of the cluster.**

It **creates and manages rules** for forwarding internal/external traffic, which comes into an object called Service in Kubernetes, to which pod.

For this purpose, `kubeadm` deploys one `kube-proxy` pod as a DaemonSet on every node, and this information can be checked using the `kubectl get daemonset -n kube-system` command.

Kubernetes pods are ephemeral, so node IPs constantly change. Therefore, to ensure inter-pod networking in such an environment, Services are used.

> A Service is a virtual component that exposes applications running through pods to the network.

**A Service has no concrete entity like a pod or container; it only performs network relaying between pods.**

It can be seen as an implementation of a proxy that manages the list of nodes/pods and forwards traffic to the necessary destination.

For this, a Service also receives an internal IP from the cluster and acts as a gateway responsible for connections between pods.

The process that enables pods to access a Service performing the above role is **kube-proxy**.

The detailed role of **kube-proxy** has changed significantly around Kubernetes version `1.20`. Previously, **kube-proxy** directly performed the user space proxy role, but more recently, it only manages and manipulates netfilter via **iptables**.

![](https://velog.velcdn.com/images/bsj1209/post/5c4757e7-80ca-4b9d-a587-4c0c79202c4a/image.png)

### Case 1 - user space proxy role (before 1.20)

Let's assume a specific client process sent an access request targeting `10.3.241.152:80`, which was then sent to the pod's `veth0` at `10.0.2.3`.

1.  kube-proxy opens port 10400 (arbitrary) on the localhost interface to receive service requests.
2.  kube-proxy configures netfilter (in the physical server's kernel) to route packets arriving at the service IP (`10.3.241.152:80`) to kube-proxy itself.
3.  kube-proxy forwards the incoming request to the actual server pod `<IP:Port>` (e.g., `10.0.2.2:8080`).

In such a case, all pods in the cluster would call kube-proxy every time they request the Service's IP. This results in a heavy load on the host and consumes resources.

### Case 2 - role of only manipulating netfilter via iptables (after 1.20)

In this case, kube-proxy does not directly perform the proxy role but delegates it entirely to netfilter.

Netfilter handles all tasks of discovering the Service IP and delivering it to pods, while kube-proxy merely modifies netfilter's rules appropriately.

When changes occur in Kubernetes, such as creating a new Service, a pod being created, or a ReplicaSet changing, the corresponding resource controller becomes aware of it.

At this point, the controller informs kube-proxy of this information, and kube-proxy operates by dynamically delivering new traffic distribution rules to netfilter via the Linux tool iptables.

<br>

### Container Runtime Engine

**The engine that fetches and runs container images within the cluster.**

It is an essential component for pods to operate within a node.

As of Kubernetes version 1.23, the runtimes supported on Linux OS are as follows:

-   [containerd](https://containerd.io/?ref=seongjin.me)
-   [CRI-O](https://cri-o.io/?ref=seongjin.me)
-   Docker Engine (deprecated; scheduled for removal in `1.24`)
-   [Mirantis Container Runtime](https://www.mirantis.com/software/container-runtime/?ref=seongjin.me)

#### Docker Engine Deprecation (after v1.24)

Originally, Kubernetes supported Docker as the runtime engine for creating and running containers, but [in December 2020, it announced that support would be deprecated after version `1.20`.](https://kubernetes.io/ko/blog/2020/12/02/dont-panic-kubernetes-and-docker/?ref=seongjin.me) As of February 2022, runtime support using Docker is [scheduled to be completely discontinued starting with the upcoming `1.24` version](https://github.com/kubernetes/enhancements/tree/master/keps/sig-node/2221-remove-dockershim?ref=seongjin.me), and alternative solutions like `containerd` are now being used by default.

The exclusion of Docker from the runtime engine does not mean that Docker itself cannot be used in a Kubernetes cluster. It only means it is excluded from the runtime engines supported by Kubernetes. Images created with Docker follow the [OCI (Open Container Initiative)](https://opencontainers.org/?ref=seongjin.me) standard, so they can still be run without issues via `containerd` and `CRI-O`, which is the official stance of The Linux Foundation.
