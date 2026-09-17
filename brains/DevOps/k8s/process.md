# 쿠버네티스의 동작 흐름

쿠버네티스에서는 새로운 pod을 만들기 위한 과정은 다음과 같다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F6lcPD%2Fbtq4juiLq67%2Fc8vAuQV7uTTx3ucchiPTPk%2Fimg.png)

1. Master Node의 kube-apiserver에 pod 생성을 요청한다.


2. kube-apiserver는 etcd에 새로운 상태를 저장한다.
3. kube-controller-manager에게 kube-apiserver가 etcd의 상태 변경을 확인하여, 새로운 pod 생성을 요청한다.
4. kube-controller-manager는 새로운 pod을 생성 (no assign)을 kube-apiserver에 전달하고, 이를 전달받은 kube-apiserver는 etcd에 저장
5. kube-scheduler는 kube-apiserver에 의해 pod(no assign)가 확인되면, 조건에 맞는 Worker Node를 찾아 해당 pod을 할당하기 위해 kube-apiserver는 etcd에 업데이트
6. 모든 WorkerNode의 kubelet은 자신의 Node에 할당되었지만, 생성되지 않은 pod이 있는지 체크하고 있다면 pod을 생성합니다.
7. 해당 Workder Node의 kubelet은 pod의 상태를 주기적으로 API 서버에 전달합니다.

흐름을 보면 각각의 컴포넌트들이 서로 개별적으로 통신하지 않고, API Server를 통해서 통신하는 것을 알 수 있습니다. 

또한 컴포넌트들은 각기 현재 상태를 체크하고 독립적으로 동작하게 됩니다. 

```zsh
kubectl [command] [type] [name] [flags]
```

### command
kubectl이 실행할 명령어. 예를 들어 `get`, `create`, `delete`, `apply` 등이 있다.

### type
명령어가 적용될 쿠버네티스의 오브젝트 유형. 예를 들어 pod, service, deployment 등이 있습니다.

### name
명령어가 적용될 쿠버네티스 오브젝트의 이름. 예를 들어 파드의 경우 파드의 이름을 지정합니다.

### flag
명령어 실행에 영향을 주는 옵션. 예를 들어 `--namespace`, `--selector`, `--output`등이 있습니다.