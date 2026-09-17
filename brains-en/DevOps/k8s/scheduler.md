# Scheduler, Controller

### goal
- Scheduler: Pod -> Node Mapping Principle
- Controller Flow: From Deployment to Pod Creation
- ReplicaSet's Pod Maintenance and Self-Healing Mechanism

## How K8s Scheduler Assigns Pods to Nodes

The K8s Scheduler is responsible for binding Pending Pods to appropriate Nodes.

Scheduling is performed not by simply placing Pods on empty Nodes, but by considering various resource constraints and score-based priorities.

### Scheduling Flow

First, candidate Nodes capable of accommodating the Pod are filtered from all available Nodes.

Priorities are determined through a filtering -> scoring process.

**Phase 1 Filtering - Key Filter Conditions**
- Whether Node Status is READY
- Whether Node taints can be tolerated
- Whether the Node has enough resources to accommodate the Pod's resource requests (CPU/memory)
- Check if Affinity/Anti-Affinity conditions are met
- Volume, topology, zone conditions

**Phase 2 Scoring** (Scores typically range from 0 to 100)
- LeastRequestPriority: Prefers Nodes with the lowest CPU/Memory utilization
- BalancedResourceAllocation: Prefers Nodes with less resource imbalance
- NodeAffinityPriority: Degree to which affinity conditions are met
- ImageLocalityPriority: Prefers Nodes where the required image is already cached
Ultimately, the Node with the highest score is selected.

### Binding Process
- The Scheduler watches for Pending Pods from the API server.
- After filtering and scoring, once an appropriate Node is found, the Scheduler sets the `PodSpec.nodeName` field to bind the Pod to that Node.

### Flow from Deployment Resource Creation to Pod Creation
```
1. kubectl applyh -f deployment.yaml
2. API server -> etcd 저장
3. Deployment Controller 감지 (kube-controller-manager)
4. ReplicaSet 생성 -> Pod 생성 요청
6. Scheduler 감지 -> 노드 할당
7. kubelet -> 실제 파드 실행
```

## ReplicaSet Pod Self-Healing

A ReplicaSet always tries to maintain the specified number of replicas.

```yaml
# ex deployment.yaml
...
spec:
  replicas: 3
```
-> If only 2 Pods are alive, one new Pod is created.
-> If there are 4, one is terminated.

### Self-Healing Mechanism
- The ReplicaSet Controller continuously WATCHes the API Server's state.
- If a Pod is deleted or becomes NotReady due to a CrashLoopBackOff, it detects this and recreates the Pod.
- It calculates and adjusts the number of currently running Pods based on the `label selector`.

### Real-time Detection Method
- The kube-controller-manager periodically polls etcd's state or uses event-based WATCH to detect ReplicaSet and Pod status changes in real-time.

**Failure Recovery Scenario Example**
1. Node goes down -> Pods on that Node are deleted.
2. ReplicaSet detects this -> Creates a new Pod on another Node.
3. kube-scheduler assigns a Node -> kubelet restarts it.

### Summary

When the Scheduler assigns Pending Pods to Nodes, it calculates priorities through Filtering and Scoring before placement.

After Deployment creation, Pod execution follows the sequence: Deployment -> ReplicaSet -> Pod -> Scheduler -> Kubelet execution.

The operating principle of ReplicaSet is to maintain the `spec.replicas` count + Pod status check -> Self-Healing.
