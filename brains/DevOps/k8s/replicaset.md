# ReplicaSet

쿠버네티스의 워크로드 리소스중 레플리카셋에 대해서 알아보도록 하겠다.

> 워크로드란? **시스템이 처리하는 일의 종류와 양을 말한다.** 시스템이 동작하는데에 필요한 컴퓨팅 시스템 cpu, memory 자원등이다.

그리고 현대에는 파드를 하나만 배포하여 관리하지 않는다. 고가용성을 보장하기위해 여러 파드를 배포하여 서비스한다. (3개 정도)

한 노드에 pod가 3개가 존재해야된다고 설정하게 되면, 한 파드가 장애가 나서 다운 됐을시에 새로운 파드를 만들어 재배포한다. 그렇게 설정된 파드의 개수를 유지하게 된다.

그리고 이렇게 개수를 유지해주는 워크로드 리소스가 바로 **레플리카셋(ReplicaSet)**이다.

즉, 배포 규격을 정의하고 그 규격에 정의된 수 만큼을 보장하는 역할을 한다.

쿠버네티스에서 사용되는 중요한 개념중 하나가 바로 **선언적 구성**이다.

"파드 3개를 새로 생성한다."가 아니라 "파드 3개를 유지한다." 특정한 상태를 유지 선언 하는 것이 쿠버네티스의 시스템 구성 개념이다.

> 과거에는 레플리케이션 컨트롤러(Replication Controller)가 이를 담당했으나, 쿠버네티스 `1.2` 기준으로는 레플리카셋으로 이 역할이 대체되었다.

레플리카셋은 다음과 같이 `yaml`을 통해 선언이 가능하다.

```yaml
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: frontend
  labels:
    app: guestbook
    tier: frontend
spec:
  replicas: 3
  selector:
    matchLabels:
      tier: frontend
  template:
    metadata:
      labels:
        tier: frontend
    spec:
      containers:
      - name: php-redis
        image: gcr.io/google_samples/gb-frontend:v3
```

- `apiVersion`이 `apps/v1`인 점에 주의할 것. 파드, 서비스와 달리 복수의 객체를 다루는 레플리카셋, 디플로이먼트 등에서는 `apps/v1`을 명시해야 한다.
- `spec.selector.matchLabels`, `spec.template.metadata.labels`의 키-값 쌍은 반드시 동일해야 한다.

대부분의 다른 쿠버네티스 객체들과 마찬가지로, `get` `describe` `delete` 등의 기본 커맨드를 활용 가능하다.


```bash
kubectl get rs
kubectl get replicaset

kubectl describe rs/frontend
kubectl edit rs/frontend

# 레플리카셋 신규 배포
kubectl create -f replicaset.yaml

# 레플리카셋 신규/수정 배포
kubectl apply -f replicaset.yaml

# 레플리카셋 명세가 포함된 yaml 파일을 새로 적용
kubectl replace -f replicaset.yaml

# yaml 파일 바탕으로 배포된 레플리카셋의 레플리카 수를 6으로 조정
kubectl scale --replicas=6 -f replicaset.yaml

# myapp 이름의 레플리카셋이 가진 레플리카 수를 6으로 조정
kubectl scale --replicas=6 replicaset myapp
```


### Template, Replicas, Selector

추가로 위와 같은 세 가지 구성 요소를 알아보겠다.

위의 구성요소중 template, replicas는 현재 deprecated된 Replication Controller에 포함되어있다.
그리고 ReplicaSet은 ReplicationController에서 Selector가 추가되어진 것이다.

- `Template`: Template는 파드가 죽게 되어 다시 재생성 해야할 때 어떤 파드를 띄울 것인지 명시하는 것이다.
- `Replicas`: Replicas는 스케일아웃(인) 기능을 수행하는 컴포넌트다.
- `Selector`: Selector는 replication controller의 label match 기능과 더불어 좀 더 세밀한 조건 설정을 할 수 있다 matchExpression로 가능

> 컨트롤러 - 쿠버네티스의 컨트롤러는 다음과 같은 4가지를 관리한다. 오토 힐링, 오토 스케일링, 소프트웨어 업데이트, Job(특정 시기에 필요한 정적인 작업)

