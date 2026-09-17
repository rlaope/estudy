# Kubernetes API Resources

API resources are a type of object that Kubernetes can manage.

There are various types, including pod, configmap, node, and so on.

When these resources are objectified (instantiated), they are called `objects`.

This command can print all the API resources currently supported by the Kubernetes cluster.

You can check at a glance which resources can be created and managed.

```zsh
kubectl api-resources
```

The command below provides explanations about the specifications, uses, and purposes of these contents.

```zsh
kubectl explain pod
```

First, let's start Minikube. You'll see it run with very charming and cute emojis.

```zsh
minikube start
```

You can also check the cluster's status by running the status command.
```zsh
minikube status
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbZANSM%2FbtruYSkCVfG%2FgluihlOkkXQvKLKMmlZAs0%2Fimg.png)

When you run the `api-resources` command for the first time, a screen like the one below appears. Please note that the capture is not exhaustive; it appears as a very long list. You should examine the names and versions.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb1D9hp%2FbtruYTKC9pl%2FmxyzJnU588q6IY9Xfss9P1%2Fimg.png)

By entering the command below, you can check the status of all names in the current cluster using `kubectl`.

```zsh
kubectl get name --all-namespaces
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb672G8%2FbtruVJ927JT%2FV2pEiHts7cE78aKuXdGGUk%2Fimg.png)

Various objects are managed in YAML format. The root keys include `apiVersion` to check which group and version it belongs to, `kind` to identify what type of resource the object is, `metadata` which contains identification information (name, namespace, labels, etc.), and finally `spec` which describes what data the object desires.

A unique point is that depending on the API resource, you might use other attributes instead of `spec`, such as `data` (for configmap, secret), `rules` (for Role), `subjects`, and so on.

Important contents within `metadata` include `Labels` and `annotations`. All Kubernetes objects can have this information even if not explicitly defined. However, `Labels` are for the purpose of identifying objects (who owns it, what type it is, what app it is, etc.), and since internal features provide a Label Selector function, it's generally important to set them for identification. `annotations` values are slightly different; they are not for identification but are used for configuration purposes to determine how an object should be processed (read by Kubernetes add-ons).
