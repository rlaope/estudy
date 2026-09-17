# Control Plane

In the Kubernetes Cluster section, we briefly looked at the components, but this time we will examine each component in more detail.

1.  `etcd`: A key-value database that stores configuration information within the cluster.
2.  `scheduler(kube-scheduler)`: A scheduler that performs optimal deployment within the cluster for each container required to run applications.
3.  `controller-manager`: A collection of processes that maintain and manage resources running in the cluster, such as Node, Deployment, and Service Account.
4.  `kube-dns`: A name server used to find specific DNS within the cluster (although `kube-dns` is shown in the diagram, it has been replaced by `CoreDNS` since Kubernetes version `1.12`).
5.  `api-server`: A component that manages the Kubernetes API required for inter-component communication within the cluster.

<br>

### etcd

**etcd is a database where information about each component of the cluster is stored in a key-value format.**

In other words, all core cluster data is stored here.

In Kubernetes, information such as how many nodes are in the cluster, which containers each pod holds, and how they operate on which node, are all recorded in etcd.

All information viewable via `kubectl` commands comes through etcd.

If changes are applied to the cluster via control commands, etcd is updated.

If etcd is lost, all components of the cluster are lost along with it.

When building a Kubernetes cluster, it is essential to consider how to ensure the high availability of **etcd**.

Kubernetes official documentation introduces two methods and commonly recommends a master node configuration with at least three control planes.

1.  **Stacked etcd topology**: This method configures the cluster in a stacked form where an etcd pod is created alongside each control plane node. It is simple to implement and consumes relatively fewer infrastructure resources, but if a node dies, both the control plane instance and etcd are lost, leading to redundancy issues. To offset this and maintain high availability, as many control plane nodes as possible should be operated simultaneously.
2.  **External etcd topology**: This method involves grouping etcd pods corresponding to each control plane node to create a separate cluster and linking them with `kube-apiserver`. Compared to method 1, it is less affected by redundancy issues due to node status, but since etcd members must be configured as a separate cluster for each node, it consumes twice the infrastructure resources and cost.

> `kubeadm` uses method 1 as the default, and in real-world production environments, a combination of method 1 with an L4 switch is generally preferred. If method 1 is followed in a cluster installed via `kubeadm`, etcd is also deployed as a pod named `etcd-master` within the `kube-system` namespace.

<br>

### Scheduler (kube-scheduler)

**It is the process that decides which node a pod should be placed on.**

It's important to note that it only makes the decision. The actual placement of the pod on the node is performed by kubelet.

The scheduler's priority for placing pods on nodes is as follows. Of course, the criteria applied here can be bypassed or changed as needed.

1.  Filtering based on the computing resources (CPU, Memory, etc.) required by the pod.
2.  Prioritizing based on the amount of remaining computing resources on the node after the pod is placed.

The scheduler typically exists as a `kube-controller-scheduler-master` pod in the `kube-system` namespace. The location of the file (YAML) defining this pod varies depending on the cluster build method.

-   If built with `kubeadm`, it exists at `/etc/kubernetes/manifests/kube-scheduler.yaml`.
-   If built with other methods, it exists at `/etc/systemd/system/kube-scheduler.service`.

<br>

### Controller Manager (kube-controller-manager)

**It is a process that monitors various resources running within the cluster and manages them to ensure smooth operation.**

It takes the form of controllers managing various components such as Node, ReplicaSet, Deployment, StatefulSet, DaemonSet, Service Account, Cronjob, and Namespace, all packaged into one. This aligns with Kubernetes' design philosophy of **functional distribution**.

The controller manager also typically exists as a `kube-controller-manager-master` pod in the `kube-system` namespace. The location of the YAML definition file for this pod varies depending on the cluster build method.

-   If built with `kubeadm`, it exists at `/etc/kubernetes/manifests/kube-controller-manager.yaml`.
-   If built with other methods, it exists at `/etc/systemd/system/kube-controller-manager.service`.

<br>

### API Server (kube-apiserver)

**It acts as a central access point that monitors each element of the cluster and enables tasks to be performed.**

Kubernetes is based on the design philosophy that "all communication is API-centric."

Therefore, Kubernetes ensures that all objects, in principle, look to the API and communicate only through the API.

The API server has stateless characteristics and uses etcd instead of maintaining sessions.

It checks data and processes tasks on an ad-hoc basis, and it can be scaled in parallel through replication.

It performs various roles such as **user authentication, request validation, data reception, etcd updates, scheduling, and kubelet communication**.

Its main functions are as follows.

-   `API Management`: It acts as a process that exposes and manages APIs on the server. Typically, port 6443 is opened for Kubernetes API access. However, there is also a method of using the open-source haproxy to forward access on port 16443 to the API server.
-   `Request Processing`: It processes individual API requests from clients. Most are in HTTP format, but some content may be processed as JSON.
-   `Internal Control Loop`: It also handles background tasks necessary for API operation. However, most of these tasks are typically performed by the controller manager.

The API server is also deployed as a pod within the `kube-system` namespace of the cluster.

The definition file path for this pod is as follows.

-   If built with `kubeadm`, it exists at `/etc/kubernetes/manifests/kube-apiserver.yaml`.
-   If built with other methods, it exists at `/etc/systemd/system/kube-apiserver.service.yaml`.

**How the API server operates in a cluster**

Let's assume a new pod is created using `kubectl`. In this case, the API server creates a new pod object and then updates etcd with the new pod information.

Meanwhile, other components monitor the API server and perform their respective roles according to the changed situation.

The flow of such operations can be examined in more detail as follows.
1.  The API server first creates the pod object.
2.  The API server updates etcd that a new pod has been created.
3.  The scheduler monitors the API server, recognizes that a new pod has been created, and identifies the node to deploy it on.
4.  The API server updates etcd with the information.
5.  The API server transmits the relevant information to the kubelet on the appropriate node.
6.  Kubelet creates the pod on that node and instructs the container runtime engine to deploy the application image.
7.  Kubelet updates the status to the API server, and the API server stores this status data in etcd.

Many operations in Kubernetes generally follow this flow. For example, if `kubectl get` is executed, the API server authenticates/validates the request, retrieves data from `etcd`, and returns it. This highlights the importance of the API server in the operation and management of cluster components.
