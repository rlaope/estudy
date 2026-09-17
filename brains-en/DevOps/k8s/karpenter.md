# Cluster Autoscaler, Karpenter

Cluster Autoscaler is an official sub-open-source project of [autoscaler](https://github.com/kubernetes/autoscaler).

It is a component that dynamically scales the worker nodes of a cluster.

It handles tasks such as creating new clusters when more worker node resources are needed, or deleting clusters when they are deemed unnecessary.

This operation is performed at the request of the Cloud Provider.

### Provisioning

In AWS, this operation is carried out through ASG (Auto Scale Group). CA continuously monitors the state of Pods, and if allocation consistently fails, it modifies the ASG Desired Capacity value of the Node Group to increase the number of worker nodes.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*xlvrYe_cv7ZXcscCvP5XEg.png)

1. There are Pods in a Pending state due to insufficient resources.
2. CA increases the Desired count of the ASG.
3. AWS ASG provisions new nodes.
4. kube-scheduler allocates the Pending Pods to the new nodes.

<br>

### Deprovisioning

For deprovisioning, if the resource utilization based on the node's allocatable resources is below 50 percent, it is considered a deprovisioning target.

After calculating whether Pods running on that node can be moved to another node, the node deletion proceeds, and then the ASG's Desired count is modified.

<br>

**Cluster Autoscaler provisions nodes by adjusting the ASG's Desired size as described above, which leads to a slow response time.**

To solve this problem, we can use Karpenter.

<br>

### Karpenter

Karpenter is an open-source project developed by AWS that provides automatic scaling functionality for Kubernetes worker nodes.

It performs a similar role to the CA mentioned earlier but allows for JIT (Just In Time) deployment without dependency on AWS resources.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*h_Rnwm0cJ--zeujr9aNQ_g.png)

As seen above, provisioning occurs independently of ASG, unlike CA.

Since node provisioning happens JIT when there aren't enough nodes to allocate Pods, Pods can be allocated more quickly.

Provisioning also requires specific conditions of the provisioner to be met for nodes to be provisioned. Let's look at those conditions.

1. Resource requests: If the resources required by the Pod are greater than what is available on the current nodes, the provisioner is triggered.
```yaml
spec:  
	containers:  
		- name: dotori-service
		image: k8s.gcr.io/pause  
		resources:  
			requests:  
				cpu: 2000m  
				memory: 2.5Gi  
			limits:  
				cpu: 2000m  
				memory: 2.5Gi
```

Since 2 CPU cores and 2.5Gi of memory are required above, an xlarge type node will be provisioned.

2. Node selection: Specify conditions for selecting desired nodes in `nodeSelector` to trigger the provisioner.

```yaml
spec:  
. template:  
	metadata:  
		labels:  
			app: nginx  
 .spec:  
	NodeSelector:  
		topology.kubernetes.io/zone: ap-northeast-2a  
		karpenter.sh/capacity-type: spot
```

The example above shows that an on-demand or spot provisioner matching the specified labels will operate, causing a new node to launch or a Pod to be allocated to an existing node that matches the labels.

For more details, please refer to [this article](https://medium.com/uplusdevu/karpenter%EC%99%80-empty-pod%EC%9D%84-%ED%99%9C%EC%9A%A9%ED%95%9C-%EC%8A%A4%EC%BC%80%EC%9D%BC%EB%A7%81-1-775737f265b3).

<br>

### Configuration Method

Specify the type of node to create when nodes are full using a provisioner.

Because the Provisioner was configured as shown on the left, the node created by Karpenter has the taint shown on the right.

```yaml
apiVersion: karpenter.sh/v1alpha5 
kind: Provisioner 
... 
spec: 
	taints: 
		- key: "dotori/server" 
		- value: "true" 
		- effect: "PreferNoSchedule"
```

```bash
$ kubectl get nodes -o json | jq '.items[].spec.taints' 
[ 
	{ 
		"effect": "PreferNoSchedule",
		"key": "dotori/server",
		"value": "true" 
	} 
]
```

For Pods to be scheduled on nodes created by Karpenter, configure `nodeSelector` and [tolerations](https://karpenter.sh/docs/concepts/scheduling/#taints-and-tolerations).

```yaml
# Schedule only on nodes with the Karpenter=enabled label
nodeSelector:
  Karpenter: enabled
# Configure tolerations to ignore the Karpenter taint
tolerations:
  - effect: "NoSchedule" 
    key: karpenter
    operator: "Equal"
    value: "true"
```
