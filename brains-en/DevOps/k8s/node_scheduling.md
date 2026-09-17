# Node Scheduling

Scheduling is a feature used when a user specifies which node a Pod should be placed on, or when the Kubernetes scheduler determines it, and Kubernetes supports various scheduling methods.

### Specific Node Selection

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdWoT70%2FbtrrirDFmz8%2FVVzh0Zdw0g11ckz82Nw9OK%2Fimg.png)

There are methods like NodeName, NodeSelector, and NodeAffinity. Let's look at each one:

- `NodeName`: Assigns a Pod by specifying the name of the node.
- `NodeSelector`: Assigns a Pod by specifying a node's label (where key and value match).
- `NodeAffinity`: Similar to NodeSelector in specifying node labels, but allows for more flexible assignment by specifying keys.

NodeName has the advantage of being simple to specify, but due to the nature of Pods, their names change frequently, making it difficult to apply. Also, NodeSelector cannot assign a Pod if a specific label does not exist.

Therefore, the more flexible NodeAffinity is used, which includes features that complement NodeSelector. Multiple conditions can be specified through `matchExpressions` and `Required`, `Preferred` options.

- `matchExpressions`: Checks conditions for the specified label, such as Exists, In, and NotIn.
- `required`: If a node matching the specified condition does not exist, the Pod is not assigned.
- `preferred`: Even if a node matching the specified condition does not exist, the Pod can be assigned to another node. If a matching node exists, the scheduler preferentially assigns the Pod to that node.

<br>

### Pod Concentration and Dispersion

This method assigns Pods to nodes based on Pods, rather than nodes.

There are two methods:

![](https://blog.kakaocdn.net/dn/n60mL/btrrhRQhn9j/SYQH6O4ntEMic3aY43B7T0/img.png)

- `Pod Affinity`: A method to assign Pods to the same node. For example, if two Pods reference the same PV hostpath, they should be placed on the same node. It is used when it is more efficient for multiple Pods to run on the same Node for resource efficiency, such as network usage.
- `Anti Affinity`: Assigns Pods to different nodes. If Pods with a Master-Slave structure are assigned to the same node, they cannot perform failover if that node goes down, so they are placed on different nodes.

<br>

### Restricting Assignment to Specific Nodes

**Toleration/Taint**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FV8EjW%2FbtrrcNVLWoA%2FFvMxd6lbpckG66jYANa1Mk%2Fimg.png)

> Taint: Contamination Toleration: Immunity

This is a method to restrict which Pods can be placed on a specific node.

For example, if a cluster has nodes allocated for CPU and nodes for GPU, you might not want just any Pod to be placed on a GPU node.

Therefore, Pod assignment to specific nodes is restricted and controlled.

Thus, Taint/Toleration is used to control Pod assignment to nodes.

1. First, if a Taint is specified on a particular node, Pods will not be placed on that node.
2. To place a Pod on that node, a Toleration is required. Specific conditions can be added to the Toleration, allowing only Pods that match the conditions specified in the Taint to be placed.
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbIKu8K%2FbtrrdYPYZy8%2FhtiPUYQnzOAe2sIem6Pnk0%2Fimg.png)

> It's important to note here that Toleration does not actively seek out Taints for assignment; rather, a node selector must be specified for a Pod to be assigned to a node. Then, if there is a Taint on that assigned node, the Toleration is checked.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbZybk9%2FbtrriWQ2Lpl%2FOBqGbCjDh0plBJQxapjgDK%2Fimg.png)

Let's look at the Effect options as shown above. How are Pods handled if a Taint appears on a node where Pods are already running? NoSchedule and PreferNoSchedule keep the existing Pods. However, NoExecute deletes the existing Pods.
