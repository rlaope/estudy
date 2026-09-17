# PV/PVC, Dynamic Provisioning

### Goal
- PVC와 PV의 바인딩 과정에서 `WaitForConsumer` 필요한 경우
- 동적 프로비저닝의 개념과 필요한 리소스

## PVC와 PV의 바인딩 과정에서 `WaitForConsumer` 필요한 경우

k8s에서 PVC(PersistenceVolumnClaim)는 사용자가 스토리지를 요청하는 객체이며, PV(PersistenceVolume)는 실제 스토리지 리소스를 나타낸다.

PVC와 PV의 바인딩은 일반적으로 PVC가 생성될 때 자동으로 이루어지지만, 특정 상황에서는 바인딩 시점을 지연시켜야 할 필요가 있음.

`WaitForFirstConsumer`는 StorageClass의 volumeBindingMode를 설정하는 옵션으로, PVC가 생성되었을 때 즉시 PV를 바인딩하지 않고, 해당 PVC를 사용하는 Pod가 스케줄링될 때까지 바인딩을 지연시킨다.

### 필요한 경우
- **토폴리지 제약이 있는 스토리지**: 예를 들어, 특정 노드나 가용 영역에만 접근 가능한 스토리지의 경우, PVC를 사용하는 Pod의 위치를 고려하여 PV를 바인딩 해야 된다.
- **Pod 스케쥴링 제약 조건:** 파드가 특정 노드에만 스케줄링이 되어야하는 경우 해당 노드에 접근 가능한 스토리지를 바인딩 해야된다.

```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: example-storage
provisioner: kubernetes.io/aws-ebs
volumeBindingMode: WaitForFirstConsumer
```

위처럼 volumeBindingMode를 WaitForFirstConsumer로 설정하면 PVC는 해당 PVC를 사용하는 pod가 생성되고 스케줄링 될때까지 PV와 바인딩되지 않습니다.

> 예를들어 aws EKS에서 ebs스토리지를 바인딩할때 ebs는 az에 종속이 될텐데. 파드가 배치되기 이전에 pvc를 a 가용영역으로 설정해두었다가 b에 배치가된다면 volumeBindingMode: Immediate라면 마운트 오류가 발생할 수 있으니 WaitForFirstConsumer를 통해 마운트 오류를 피해볼 수 있겠다. or nodeSelector, affinity

<br>

## Dynamic Provisioning의 개념과 필요한 리소스

동적 프로비저닝은 사용자가 pvc를 생성할 때, 클러스터가 자동으로 PV를 생성하여 바인딩하는 기능이다.

이를 통해 관리자는 사전에 PV를 생성할 필요 없이, 사용자 요청에 따라 스토리지를 자동으로 할당할 수 있다.

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
- provisioner: StorageClass에서 정의한 프로비저너는 실제 스토리지를 생성하는 플러그인이다 예를들어 aws-ebs, gce-pd등이 있다.
- pvc는 storageClassName 필드를 통해 어떤 StorageClass를 사용할지 지정한다.
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

1. 사용자가 PVC를 생성함
2. PVC의 storageClassName에 지정된 StorageClass를 참조해 해당 프로비저너가 PV를 생성함
3. 생성된 PV는 PVC와 자동으로 바인딩된다.

주의할점으로는 동적 프로비저닝을 사용하려면 클러스터에 적절한 StorageClass, provisioner가 설정되어 있어야 한다

PVC의 storageClassName이 비어있다면 클러스터의 기본 StorageClass가 사용된다.

<br>

### 요약

PVC, PV 바인딩을 Pod 스케줄링 시점까지 지연시켜, 스토리지와 Pod의 위치를 일치시키기 위해서는 `volumeBindingMode: WaitForFirstConsumer`를 사용하자

동적 프로비저닝은 사용자가 pvc를 생성하면, 클러스터가 자동으로 pv를 생성하여 바인딩한다.