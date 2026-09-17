# 쿠버네티스 대시보드 토큰 발행 및 접속

> 대시보드를 실행하기 전 docker가 데몬에서 실행되어있어야하고 minikube를 실행해야합니다.


### Deploying the Dashboard UI

`kubectl apply -f https://raw.githubusercontent.com/kubernetes/dashboard/v2.7.0/aio/deploy/recommended.yaml`

### Command Line Proxy

`kubectl proxy`

<br>

## 대시보드 사용자 만들기
kubernetes-dashboard라는 네임스페이스에 admin-user라는 이름으로 서비스 계정을 생성할 것이다.

### ClusterRoleBinding

```sh
cat <<EOF | kubectl create -f -
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: admin-user
roleRef:
  apiGroup: rbac.authorization.k8s.io
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

## 대시보드 토큰 발행

`kubectl -n kubernetes-dashboard create token admin-user`를 입력해 토큰을 발행해보자

아래와 같이 토큰이 발급되는 것을 볼 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlFL58%2FbtrIn3HqWOu%2F9ViLWAlkfnKyvbJ6BIn6c0%2Fimg.png)