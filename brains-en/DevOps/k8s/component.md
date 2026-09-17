# Kubernetes Components

Kubernetes consists of a cluster made up of multiple nodes (e.g., virtual servers / Virtual Machines).

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdFM2jb%2Fbtq4jEL7V4O%2FsqemIpdhQNWEIQBYuvSr0K%2Fimg.png)

This is where the concept of a Node comes in.

In the diagram above, a Node refers to a single VM. Kubernetes consists of Worker Nodes that run containerized applications and a Master Node that manages those Worker Nodes.

Both Worker Nodes and Master Nodes can be composed of multiple instances, and to use Kubernetes, you must have at least one Worker Node.

<br>

## Master Node Components
The Master Node is responsible for making overall decisions regarding the cluster, as well as detecting and responding to events.

The Master Node is structured as shown in the following diagram.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FvFgU9%2Fbtq4cfujghF%2F86KkmJEn2ed0WCwsATHR9K%2Fimg.png)

### Components
- kube-apiserver: Handles all requests.
- kube-controller-manager: Manages various controllers (replication/deployment/state, etc.).
- kube-scheduler: Selects an appropriate Worker Node based on the situation.
- etcd: A storage for data within the cluster.

<br>

## Worker Node Components
The Worker Node is responsible for running and maintaining containerized applications.

The Worker Node is structured as shown in the following diagram.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc94mmX%2Fbtq4dcjdjIz%2FwO5ri20IjKWAk1wJYICYK1%2Fimg.png)

### Components
- pod: A group of containerized applications.
- kubelet: Checks and manages the state of pods assigned to the Node.
- kube-proxy: Manages the network connections to pods.
