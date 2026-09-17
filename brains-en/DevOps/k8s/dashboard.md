# Kubernetes Dashboard Token Issuance and Access

> Before running the dashboard, Docker must be running as a daemon and minikube must be started.

### Deploying the Dashboard UI

`kubectl apply -f https://raw.githubusercontent.com/kubernetes/dashboard/v2.7.0/aio/deploy/recommended.yaml`

### Command Line Proxy

`kubectl proxy`

<br>

## Creating a Dashboard User
We will create a service account named `admin-user` in the `kubernetes-dashboard` namespace.

### ClusterRoleBinding

```sh
cat <<EOF | kubectl create -f -
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: admin-user
roleRef:
  apiGroup: rbac.authorization.k8s.io/v1
  kind: ClusterRole
  name: admin
subjects:
- kind: ServiceAccount
  name: admin-user
  namespace: kubernetes-dashboard
EOF
```

### ServiceAccount

```sh
cat <<EOF | kubectl create -f -
apiVersion: v1
kind: ServiceAccount
metadata:
  name: admin-user
  namespace: kubernetes-dashboard
EOF
```

<br>

## Issuing a Dashboard Token

Let's issue a token by entering `kubectl -n kubernetes-dashboard create token admin-user`.

You can see the token being issued as shown below.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlFL58%2FbtrIn3HqWOu%2F9ViLWAlkfnKyvbJ6BIn6c0%2Fimg.png)
