# Kubernetes Cluster

A Kubernetes cluster refers to a collection of physical or virtual nodes that host containerized applications.

Resources on a Kubernetes host are organized and managed in multiple cluster units.

It follows a configuration where a Master Node acts as the control plane, and administrators control the entire cluster through this Master Node.

Kubernetes clusters can be divided into **Master Nodes** and **Worker Nodes** depending on their purpose. Their roles are as follows:

- **Master Node**: Plays the role of commanding the fleet of containers. It requires optimal deployment, monitoring, and efficient tracking and management for each container, considering the available resource status of each Worker Node. The node that performs this role in a Kubernetes cluster is called the Master Node.
- **Worker Node**: Acts as a container loaded with various containers. It refers to the node where containers with different purposes and functionalities are actually deployed.

<br>

### Kubernetes Cluster Internal Structure

A Kubernetes cluster can only be accessed internally via its API.

Now, let's delve deeper into the internal structure of Kubernetes.

![](https://seongjin.me/content/images/2022/02/k8s-cluster.png)

The Kubernetes Master corresponds to the control plane included in the Master Node of the Kubernetes cluster.

The control plane is responsible for deploying and managing key components such as workload resources across the entire cluster. The components included are as follows:

1.  `etcd`: A key-value database that stores configuration information within the cluster.
2.  `scheduler(kube-scheduler)`: A scheduler that performs optimal deployment for each container required to run applications within the cluster.
3.  `controller-manager`: A collection of processes that maintain and manage resources running in the cluster, such as Node, Deployment, and Service Account.
4.  `kube-dns`: A name server used to find specific DNS within the cluster (although it's labeled `kube-dns` in the diagram, it has been replaced by `CoreDNS` since Kubernetes version `1.12`).
5.  `api-server`: A component that manages the Kubernetes API, which is necessary for inter-component communication within the cluster.

Next, the Node area on the right corresponds to the components for managing and running the Pods within the node. These are common to both Worker Nodes and Master Nodes. The components are as follows:

1.  `kubelet`: An agent that monitors API requests coming to each node in the cluster and manages each Pod to ensure it functions correctly.
2.  `kube-proxy`: A network proxy service that runs on each node within the cluster.
3.  `container-runtime-engine`: An engine for running containers within deployed Pods.

CLI Tool

- [**kubectl**](https://kubernetes.io/docs/reference/kubectl/overview/?ref=seongjin.me) : A tool used to directly control the cluster and its internal components in a CLI environment, used in the terminal as `kubectl [command] [TYPE] [NAME] [flags]`.
