# ReplicaSet

Let's learn about ReplicaSets, one of Kubernetes' workload resources.

> What is a workload? **It refers to the type and amount of work a system processes.** This includes computing system resources like CPU and memory needed for the system to operate.

And nowadays, pods are not deployed and managed individually. To ensure high availability, multiple pods are deployed to provide services (around 3).

If you configure a node to have 3 pods, and one pod fails and goes down, a new pod is created and redeployed. This maintains the configured number of pods.

And the workload resource that maintains this number is the **ReplicaSet**.

In other words, it defines the deployment specification and ensures that the number defined in that specification is maintained.

One of the important concepts used in Kubernetes is **declarative configuration**.

Instead of "create 3 new pods," it's "maintain 3 pods." Declaring and maintaining a specific state is the concept of system configuration in Kubernetes.

> In the past, the Replication Controller was responsible for this, but as of Kubernetes `1.2`, this role has been replaced by ReplicaSets.

A ReplicaSet can be declared via `yaml` as follows.

```yaml
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: frontend
  labels:
    app: guestbook
    tier: frontend
spec:
  replicas: 3
  selector:
    matchLabels:
      tier: frontend
  template:
    metadata:
      labels:
        tier: frontend
    spec:
      containers:
      - name: php-redis
        image: gcr.io/google_samples/gb-frontend:v3
```

- Note that `apiVersion` is `apps/v1`. Unlike Pods and Services, ReplicaSets, Deployments, and other resources that handle multiple objects must specify `apps/v1`.
- The key-value pairs in `spec.selector.matchLabels` and `spec.template.metadata.labels` must be identical.

Similar to most other Kubernetes objects, basic commands like `get`, `describe`, and `delete` can be used.

```bash
kubectl get rs
kubectl get replicaset

kubectl describe rs/frontend
kubectl edit rs/frontend

# Deploy a new ReplicaSet
kubectl create -f replicaset.yaml

# Deploy a new/updated ReplicaSet
kubectl apply -f replicaset.yaml

# Apply a new yaml file containing the ReplicaSet specification
kubectl replace -f replicaset.yaml

# Adjust the number of replicas for a deployed ReplicaSet to 6 based on a yaml file
kubectl scale --replicas=6 -f replicaset.yaml

# Adjust the number of replicas for the ReplicaSet named myapp to 6
kubectl scale --replicas=6 replicaset myapp
```

### Template, Replicas, Selector

Additionally, let's look at the three components mentioned above.

Among the components above, `template` and `replicas` are included in the now deprecated Replication Controller.
And ReplicaSet is the ReplicationController with a `Selector` added.

- `Template`: The Template specifies which pod to launch when a pod dies and needs to be recreated.
- `Replicas`: Replicas is a component that performs scale-out (or scale-in) functionality.
- `Selector`: The Selector allows for more detailed condition settings in addition to the replication controller's label match feature, which can be done with `matchExpression`.

> Controller - Kubernetes controllers manage the following four things: auto-healing, auto-scaling, software updates, and Jobs (static tasks needed at specific times).
