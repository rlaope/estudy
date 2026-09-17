# PV/PVC, Dynamic Provisioning

### Goal
- When `WaitForConsumer` is needed during the PVC and PV binding process
- Concept of dynamic provisioning and required resources

## When `WaitForConsumer` is needed during the PVC and PV binding process

In k8s, a PVC (PersistentVolumeClaim) is an object that users use to request storage, and a PV (PersistentVolume) represents the actual storage resource.

PVC and PV binding typically occurs automatically when a PVC is created, but in certain situations, it may be necessary to delay the binding.

`WaitForFirstConsumer` is an option that sets the `volumeBindingMode` of a StorageClass, which delays PV binding until the Pod using that PVC is scheduled, rather than binding it immediately when the PVC is created.

### When needed
- **Storage with topology constraints**: For example, for storage accessible only to specific nodes or availability zones, the PV must be bound considering the location of the Pod using the PVC.
- **Pod scheduling constraints:** If a Pod must be scheduled on a specific node, storage accessible to that node must be bound.

```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: example-storage
provisioner: kubernetes.io/aws-ebs
volumeBindingMode: WaitForFirstConsumer
```

As shown above, by setting `volumeBindingMode` to `WaitForFirstConsumer`, the PVC will not be bound to a PV until the Pod using that PVC is created and scheduled.

> For example, when binding EBS storage in AWS EKS, EBS is AZ-dependent. If a PVC is set for AZ A before a Pod is placed, but the Pod is then placed in AZ B, a mount error could occur if `volumeBindingMode: Immediate` is used. `WaitForFirstConsumer` can help avoid such mount errors. Alternatively, `nodeSelector` or `affinity` can be used.

<br>

## Concept of dynamic provisioning and required resources

Dynamic provisioning is a feature where the cluster automatically creates and binds a PV when a user creates a PVC.

This allows administrators to automatically allocate storage based on user requests, without needing to pre-create PVs.

### Resource

```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: example-storage
provisioner: kubernetes.io/gce-pd
parameters:
  type: pd-ssd
```
- provisioner: The provisioner defined in the StorageClass is a plugin that creates the actual storage, such as `aws-ebs` or `gce-pd`.
- A PVC specifies which StorageClass to use via the `storageClassName` field.
```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: my-pvc
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 10Gi
  storageClassName: example-storage
```

1. The user creates a PVC.
2. The provisioner references the StorageClass specified in the PVC's `storageClassName` to create a PV.
3. The created PV is automatically bound to the PVC.

It's important to note that to use dynamic provisioning, the cluster must have an appropriate StorageClass and provisioner configured.

If the PVC's `storageClassName` is empty, the cluster's default StorageClass is used.

<br>

### Summary

To delay PVC and PV binding until Pod scheduling, ensuring storage and Pod location alignment, use `volumeBindingMode: WaitForFirstConsumer`.

Dynamic provisioning means that when a user creates a PVC, the cluster automatically creates and binds a PV.
