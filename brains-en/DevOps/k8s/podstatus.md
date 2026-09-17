# Pod Lifecycle, Restart Policy

We will explore the Pod lifecycle, restart policies, and Probes for container health checks.

### Pod Lifecycle

A Pod's lifecycle is divided into four phases.

1. `PENDING`: The Pod has been accepted by the cluster, but its containers have not yet started, and it has not been scheduled onto a node.
2. `RUNNING`: The Pod has been accepted by the cluster, its containers have started, and it is scheduled onto a node.
3. `SUCCEEDED`: This status indicates that a finite (static) job on the node has completed. It's the status shown when a node terminates, meaning the job finished successfully.
4. `FAILED`: This status indicates that a finite (static) job on the node has completed. It's the status shown when a node terminates, meaning the job terminated abnormally.

In addition to these, there are additional status (`.status.phase`) values assigned to Pods only in special cases.
- `unknown`: The Pod's status cannot be determined. This usually occurs due to node network issues.
- `terminating`: This can be seen when a Pod is deleted. Generally, a Pod has a 30-second grace period when deleted. If you want to delete it without this grace period, you can add `--force` to the `kubelet delete` option.

The Pod's lifecycle and current status are periodically monitored by **kubelet**. This monitored status information can be seen in the `STATUS` field of the `kubectl get pods` output. It can also be checked in the `.status.phase` field of the Pod object, using the following command:

```bash
kubectl get pod <파드명> -o jsonpath='{.status.phase}'
```

<br>

### Pod Restart Policy

Containers within a Pod have a **policy** for restarting when an error occurs.

This policy is categorized into 3 types and can be directly assigned via `spec.restartPolicy`.

1. `Always`: Always restarts the container, whether it terminates normally or abnormally. (default)
2. `Onfailure`: Restarts the container only if it terminates abnormally.
3. `Never`: Never restarts.

Container restart attempts occur exponentially, starting at 10-second intervals, up to a maximum of 300 seconds.

After 10s, after 20s, after 40s, after 80s...
