# Deployment

Deployment is **a resource that provides declarative updates for Pods and ReplicaSets.**

It is primarily used for managing application versions and deployments, including rolling updates.

> While a ReplicaSet is responsible for checking the status and maintaining the number of Pods, a Deployment can be seen as a higher-level object that encapsulates ReplicaSets, used for managing applications at a service level.

Pods deployed as Deployment resources all have identifiers in the format `<deployment-name>-<replicaset-unique-number>-<random-hash-value>`.

If the replica settings are changed, or if a Pod is deleted and redeployed, its identifier will also change.

In other words, the identifiers of Pods within a Deployment change dynamically depending on the situation.

<br>

### Deploying a Deployment

1. **Creating via CLI**
```bash
kubectl create deployment nginx --image=nginx:1.14.2 --replicas=3
```

- The above command is structured as `kubectl create deployment <deployment-name> --image=<image-name> --replicas=<number-of-replicas>`.
- It creates a Deployment named `<deployment-name>` that creates and maintains Pods with the image specified by `--image`, in the quantity specified by `--replicas`.
- The `--image` flag is mandatory. If `--replicas` is not specified, it defaults to 1.

<br>

2. **Creating with a YAML file** (After writing and saving the YAML file, execute `kubectl apply -f <filename.yaml>` in the terminal.)
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
  labels:
    app: nginx
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:1.14.2
        ports:
        - containerPort: 80
```

- The value of `kind` must be `Deployment` (case-sensitive).
- The number of replicas is entered in `spec.replicas`.
- Information about the Pods to be deployed is located under `spec.template`.
- Container image information for the Pod is entered under `spec.template.spec.containers`.
- To manage these deployed Pods collectively, key-value pairs are assigned via the `spec.selector.matchLabels` field.
- The key-value pairs specified in `spec.selector.matchLabels` and `spec.template.metadata.labels` must be identical.

<br>

### Changing Settings of a Deployed Deployment

Sometimes you need to update the image of a Pod already running, deployed by a Deployment,

or adjust the number of replicas.

You can change settings as follows.

#### Image Update

First, in a CLI environment, you can use the `kubectl set` command.

The command below updates the container image for the `nginx` app in `nginx-deployment` from `nginx:1.14.2` to `nginx:1.16.1`.

```bash
kubectl set image deployment/nginx-deployment nginx=nginx:1.16.1
```

Next, there's a method to modify the container image entry by fetching the YAML information of the deployed Deployment using the `kubectl edit` command. The item to modify is located at

`.spec.template.spec.containers[0].image`.

```bash
kubectl edit deployment/nginx-deployment
```

If you created it directly with a YAML file, you can also open that YAML file, change the `image` entry in `spec.template.spec.containers` to `nginx:1.16.1`, and then use the `kubectl apply` command.

#### Scaling

If you want to easily adjust the number of replicas in a CLI environment, you can use the `kubectl scale` command.

Executing the command below will immediately increase the number of replicas for `nginx-deployment` to 5.

```bash
kubectl scale deployment/nginx-deployment --replicas=5
```

However, the above command only applies changes explicitly, regardless of the content of the YAML file used for the actual deployment. Since there are also methods like `kubectl edit` or directly modifying and applying the YAML file used for the existing deployment, it's good to choose the method according to your purpose.
