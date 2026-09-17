# DaemonSet, Job, CronJob


![](https://velog.velcdn.com/images/hyun6ik/post/a164acb9-c248-44f1-9b20-251cbb4dd9b9/image.png)

### DaemonSet

A DaemonSet is a resource that ensures a single, identical Pod runs on every node, or on nodes with a specific label.

It differs from Deployments and StatefulSets in that it places only one Pod on each eligible node.

When a DaemonSet is running, Pods are placed when a node is added to the cluster, but Pods on deleted nodes are not re-scheduled to other nodes.

Therefore, it does not require separate replication settings.

_DaemonSets are primarily used to deploy resource monitoring applications or log collectors on worker nodes._

![](https://velog.velcdn.com/images%2Fhyun6ik%2Fpost%2F837ef552-18a-49c4-8195-4829d3dc245a%2Fimage.png)


```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: fluentd-elasticsearch
  namespace: kube-system
  labels:
    k8s-app: fluentd-logging
spec:
  selector:
    matchLabels:
      name: fluentd-elasticsearch
  template:
    metadata:
      labels:
        name: fluentd-elasticsearch
    spec:
      containers:
      - name: fluentd-elasticsearch
        image: quay.io/fluentd_elasticsearch/fluentd:v2.5.2
        volumeMounts:
        - name: varlog
          mountPath: /var/log
        - name: varlibdockercontainers
          mountPath: /var/lib/docker/containers
          readOnly: true
      terminationGracePeriodSeconds: 30
      volumes:
      - name: varlog
        hostPath:
          path: /var/log
      - name: varlibdockercontainers
        hostPath:
          path: /var/lib/docker/containers
```

- The value of `kind` must be `DaemonSet`. (Case-sensitive)
- Information about the Pod to be deployed is located under `spec.template`.
- The key-value pairs specified in `spec.selector.matchLabels` and `spec.template.metadata.labels` must all be identical.
- If you want to place DaemonSet Pods on nodes with specific conditions, use the `spec.template.spec.tolerations` field. Refer to [this document](https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/?ref=seongjin.me) for more details.
- If you want to place DaemonSet Pods only on nodes with specific label values, use the `spec.template.spec.nodeSelector` field. Refer to [this document](https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/?ref=seongjin.me#running-pods-on-select-nodes) for more details.

<br>

### Job, CronJob

![](https://velog.velcdn.com/images%2Fhyun6ik%2Fpost%2F4d692328-123e-4c62-98f5-638238a9cc45%2Fimage.png)


A Job is a controller that specifies one or more Pods and ensures the specified number of Pods run successfully.

It is used for tasks that run once and then terminate, such as backups or specific batch files.

If the process is not in use, the Pod is terminated (not deleted, but in a state where it's not consuming resources).

A Job has the following components:
- `Completions`: The Job succeeds if the Pods successfully complete the specified number of tasks or more.
- `Parallelism`: Processes a specified number of Pods in parallel at once.
- `ActiveDeadlineSeconds`: When the specified time elapses, the Job terminates, and all running Pods are stopped.


A CronJob is a controller that creates Jobs periodically. It's rare to use a Job as a standalone unit; instead, CronJobs are used to create Jobs at specific times.

Additionally, you can define the concurrent execution handling of Jobs through the `concurrencyPolicy` setting.

- `Allow (default)`: The CronJob allows Jobs to run concurrently.
- `Forbid`: Does not allow concurrent execution. If an existing job is in progress, the new Job is skipped.
- `Replace`: Replaces the existing job with a new one.
