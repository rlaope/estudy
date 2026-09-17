# ConfigMap, Secret

In Kubernetes, how can we handle situations where a pod's development environment is divided into dev and prod, or security connection (SSH) settings have different values for each environment, such as `false` on a development server and `true` in production?

This implies running different containers, but since there might be common values, configuring each one individually and deploying them as new containers would be cumbersome.

**A feature is needed to provide necessary environment configurations separately from the containers.**

Therefore, ConfigMap allows us to define and manage these environment settings externally from the containers.

ConfigMap stores data in a Key-Value format. (Since both key and value are stored as strings, even boolean values like `false` must be enclosed in quotes, like `'false'`.)

Secret is similar to ConfigMap, but its values must be encoded.

<br>

### Defining ConfigMap

The example below defines database configuration files as a ConfigMap.
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: config-dev
  namespace: default
data:
  DB_URL: localhost
  DB_USER: myuser
  DB_PASS: mypass
  DEBUG_INFO: debug
```
- Data can be stored in key-value format under `data`.

There are two ways to define data in a ConfigMap: you can directly input key-value pairs as constants, or you can use a file as the ConfigMap's value.

Key-value format can be defined as shown above.

### Defining with a File

When defining with a file, assuming you've created a `file.txt` file:

```bash
kubectl create configmap dev-file --from-file=./file.txt
```

As shown above, you specify the ConfigMap's name and the file from which to retrieve values.

A Secret is created as follows:
```bash
kubectl create secret generic sec-file --from-file=./file2.txt
```

A point to note here is that the content of the `.txt` file should not be encoded. Encoding occurs the moment the command is entered, so if the `.txt` file already contains encoded content, it will be double-encoded, causing issues.

### Using ConfigMap

Let's use the ConfigMap defined above. You can define the pod in a YAML file as follows:

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-name
spec:
  containers:
  - name: container-name
    valueFrom:
      configMapKeyRef:
        name: dev-file
        key: file.txt
  - name: container2-name
    valueFrom:
      secretKeyRef:
        name: sec-file
        key: file2.txt
```

Values are retrieved via `valueFrom`, and the key is specified. If `envFrom` is used, it retrieves key-value pairs in environment variable format.

<br>

### Mounting to a Volume

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-name
spec:
  containers:
  - name: container-name
    image: tmkube/init
    volumeMounts:
    - name: file-volume
      mountPath: /mount
  volumes:
  - name: file-volume
    configMap:
      name: dev-file
```

You can also mount ConfigMap data to a volume as shown above.

> You can retrieve values one by one using `env:`, or use `envFrom` to retrieve all of them.
