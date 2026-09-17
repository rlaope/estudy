# QoS Classes

When a Kubernetes node runs low on memory resources, it needs to perform tasks such as terminating certain Pods or processes and reallocating resources to other Pods.

So, what criteria are used for this reallocation? Let's explore these criteria.

### Kubernetes OOM

Before diving into that, let's understand Kubernetes OOM (Out of Memory).

```bash
kubectl describe nodes {node_name} | grep -A9 Conditions
```

Each Kubernetes node has "conditions" that contain information about the node's abnormal states.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc4rJ7j%2Fbtrao550MVa%2FR9YC260EBREwKn5lYgBh10%2Fimg.png)

MemoryPressure is fundamentally designed in Kubelet to occur when available memory falls below 100Mi.

When MemoryPressure occurs, Kubelet prioritizes all Pods running on that node and evicts the Pods with the lowest priority.

Furthermore, _**no more Pods are scheduled to nodes where MemoryPressure is True.**_
At this point, Pod priority is determined by sorting based on **QoS class and memory usage**.

<br>

### QoS Classes

Kubernetes assigns a **QoS class to a Pod based on its Request and Limit** settings.

There are three QoS classes: **BestEffort, Burstable, and Guaranteed**.

### BestEffort

```yaml
apiVersion: v1 
kind: Pod 
metadata: 
	name: nginx-besteffort-pod 
spec:
.   containers: 
	- name: nginx-besteffort-pod 
	- image: nginx:latest
```

As shown above, a Pod becomes BestEffort when the resources section (request, limit) is not used.

```bash
kubectl describe pod nginx-besteffort-pod | grep QoS
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FCyteG%2FbtrakpxxYXK%2FrupKBK5d7coTXSZGKGg2UK%2Fimg.png)

If the resources section is not used as above, there are no resource limits, and if idle resources exist, the Pod can use all resources without restriction.

And since no request is set, there are no guaranteed resources.

_This means that, depending on the situation, it might use all available resources on the node, or it might not be able to use any resources at all._

BestEffort Pods are the first to have their resources reclaimed if an OOM event occurs on the node.

<br>

### Guaranteed

```yaml
apiVersion: v1
kind: Pod 
metadata: 
	name: nginx-guaranteed-pod 
spec: 
	containers: 
	- name: nginx-guaranteed-pod 
	- image: nginx:latest 
	resources:
		limits: 
			memory: "256Mi"
			cpu: "1000m" 
		requests: 
			memory: "256Mi" 
			cpu: "1000m"
```

As in the YAML file above, if the limit and request values are equal, the Pod is assigned the **Guaranteed** class.

```bash
kubectl describe pod nginx-guaranteed | grep QoS
```

Since the Pod's request and limit are equal, overcommit is not allowed, and resource usage is guaranteed to be limited.

- _All processes running within a Guaranteed class Pod have their default **OOM score (oom_score_adj) set to -998**._
- _If a Pod contains **multiple containers**, **all containers must have identical Request and Limit settings** to be classified as Guaranteed._

<br>

### Burstable

```yaml
apiVersion: v1 
kind: Pod 
metadata: 
	name: nginx-burstable-pod 
spec: 
	containers:
	- name: nginx-burstable-pod 
	- image: nginx:latest 
	resources: 
		limits:
			memory: "1024Mi" 
			cpu: "1000m"
		requests: 
			memory: "256Mi" 
			cpu: "500m"
```

```bash
kubectl describe pod nginx-burstable | grep QoS
```

As in the YAML file above, a Pod is Burstable when the limit is greater than the request (or if not all containers have identical request/limit settings in a multi-container Pod).

Simply put, it means that the Pod can use the requested resources, but can use up to the limit if the situation allows.

<br>

### Priority

As mentioned at the very beginning, Pods are managed according to their priority.

So, what is the priority order among the three QoS classes?

Typically, it's **Guaranteed > Burstable > BestEffort**.

"Typically?" implies there are exceptions. Sometimes, the priority between Burstable and BestEffort can change based on **memory usage**. (_That is, the more memory a Pod uses, the lower its priority._)

[[Pod LifeCycle(Phase, Condition)]]
