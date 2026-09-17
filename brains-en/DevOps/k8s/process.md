# Kubernetes Operation Flow

In Kubernetes, the process for creating a new pod is as follows.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F6lcPD%2Fbtq4juiLq67%2Fc8vAuQV7uTTx3ucchiPTPk%2Fimg.png)

1. A request to create a pod is sent to the kube-apiserver on the Master Node.
2. The kube-apiserver stores the new state in etcd.
3. The kube-apiserver detects the state change in etcd and requests the kube-controller-manager to create a new pod.
4. The kube-controller-manager informs the kube-apiserver about the creation of a new pod (no assign), and the kube-apiserver, upon receiving this, stores it in etcd.
5. When the kube-scheduler identifies a pod (no assign) via the kube-apiserver, it finds a suitable Worker Node and updates etcd via the kube-apiserver to assign the pod to that node.
6. The kubelet on each Worker Node checks if there are any pods assigned to its Node but not yet created, and if so, it creates the pod.
7. The kubelet on that Worker Node periodically reports the pod's status to the API server.

Looking at the flow, we can see that each component communicates not directly with each other, but through the API Server.

Furthermore, each component checks its current state and operates independently.

```zsh
kubectl [command] [type] [name] [flags]
```

### command
The command to be executed by kubectl. Examples include `get`, `create`, `delete`, `apply`, etc.

### type
The type of Kubernetes object the command applies to. Examples include pod, service, deployment, etc.

### name
The name of the Kubernetes object the command applies to. For example, for a pod, you specify the pod's name.

### flag
Options that affect command execution. Examples include `--namespace`, `--selector`, `--output`, etc.
