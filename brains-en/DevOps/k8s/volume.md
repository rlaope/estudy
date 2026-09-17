# Volume - emptyDir, hostPath, PV/PVC

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fby3opf%2FbtqYQxNFeyU%2FHCEEJp53CvgAkCzIyW0nL0%2Fimg.png)

## empty Dir

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FEEKLg%2FbtqY5MCyP15%2FmE1vNFhYnHgCmzBevGCk80%2Fimg.png)

Volumes are used to share data between containers.

When a volume is first created, its content is empty, hence the name emptyDir.

Let's say Container1 is the web server and Container2 is the backend. Since Container2 also uses the Volume, both servers can conveniently use it without needing to transfer files between them.

This volume is created inside the pod, so if the pod encounters an issue and is recreated, the data will be lost.

Data written to the volume must be for temporary use only.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-volume-1
spec:
  containers:
  - name: container1
    image: kubetm/init
    volumeMounts:
    - name: empty-dir
      mountPath: /mount1
  - name: container2
    image: kubetm/init
    volumeMounts:
    - name: empty-dir
      mountPath: /mount2
  volumes:
  - name : empty-dir
    emptyDir: {}
```

- There are two containers.
- Both are mounting a volume.
- Looking at the mount paths, Container1 uses mount1, and Container2 uses mount2.
- Even if the paths are different, since the name is empty-dir, each container can use its desired path.
- In other words, both are using the same volume.

<br>

## hostPath

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb3o0DT%2FbtqY9Y956M3%2FgfALaPoPheneDP0ik3bDk0%2Fimg.png)

It uses a path on the node where pods are running as a volume.

The difference from emptyDir is that pods share the path, so even if a pod dies, the node's volume does not.

The problem is that if Pod2 dies and is recreated on Node2 instead of Node1, it cannot use Node1's volume.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcEA6dd%2FbtqY7nCv8s6%2FDtUZ4Qk1cMahSN4aBDrcLk%2Fimg.png)

When Node2 is created, it's possible to solve this by creating the same path and mounting paths between nodes, but Kubernetes doesn't do this automatically.

This involves an operator using Linux system mount technology -> it's not automated, so it's not stable.

hostPath is used when a pod needs to read or write data on the node it is allocated to.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-volume-3
spec:
  nodeSelector:
    kubernetes.io/hostname: k8s-node1
  containers:
  - name: container
    image: kubetm/init
    volumeMounts:
    - name: host-path
      mountPath: /mount1
  volumes:
  - name : host-path
    hostPath:
      path: /node-v
      type: DirectoryOrCreate
```

- When creating a pod, the container mounts a volume, and the mountPath is /mount1.
- The host-path volume for the path is at /node-v.
- The type of host-path is Directory.
- The host-path at /node-v must exist before the pod is created to avoid errors.
- hostPath is not for storing pod data but for pods to write to node data.

<br>

## PVC/PV

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fpkvrz%2FbtqY91eHlSw%2FeKkfTLmOQct4cCGKTvhyh1%2Fimg.png)

Persistent Volume Claim, Persistent Volume

PVC/PV provides persistent volumes to pods.

Volumes can be local or utilized remotely from external sources.

These are defined as PVs and connected accordingly.

Pods do not connect directly to PVs but rather through PVCs.

The reason pods don't connect directly to PVs is that Kubernetes manages areas by dividing them into User and Admin.

Admin refers to the Kubernetes operator, and User refers to the person responsible for creating and deploying services on pods.
