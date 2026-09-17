# DaemonSet, Job, CronJob


![](https://velog.velcdn.com/images/hyun6ik/post/a164acb9-c248-44f1-9b20-251cbb4dd9b9/image.png)

### DaemonSet

**데몬셋은 모든 노드에, 혹은 특정 레이블을 가진 노드에 무조건 하나씩 동일한 파드를 구동하게 해주는 리소스다.**

해당되는 노드에 오직 단 하나의 파드만 배치한다는 점에서 디플로이먼트, 스테이트풀셋과 다른점을 가지고있다.

데몬셋이 구동중이면 클러스터에 노드가 추가될 때 파드가 배치되지만 삭제된 노드에 파드가 다른 노드에 재배치 되지는 않는다.

따라서 별도의 레플리케이션 설정을 하지 않는다.

_데몬셋은 주로 워커 노드에 리소스 모니터링용 애플리케이션, 또는 로그 수집기를 배포할 때 쓰인다._

![](https://velog.velcdn.com/images%2Fhyun6ik%2Fpost%2F837ef552-e18a-49c4-8195-4829d9dc245a%2Fimage.png)




```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: fluentd-elasticsearch
  namespace: kube-system
  labels:
    k8s-app: fluentd-logging
spec:
  selector:
    matchLabels:
      name: fluentd-elasticsearch
  template:
    metadata:
      labels:
        name: fluentd-elasticsearch
    spec:
      containers:
      - name: fluentd-elasticsearch
        image: quay.io/fluentd_elasticsearch/fluentd:v2.5.2
        volumeMounts:
        - name: varlog
          mountPath: /var/log
        - name: varlibdockercontainers
          mountPath: /var/lib/docker/containers
          readOnly: true
      terminationGracePeriodSeconds: 30
      volumes:
      - name: varlog
        hostPath:
          path: /var/log
      - name: varlibdockercontainers
        hostPath:
          path: /var/lib/docker/containers
```

- `kind`의 값은 반드시 `DaemonSet`이어야 한다. (대소문자 구분 필수)
- 배포할 파드에 대한 정보는 `spec.template` 아래에 위치한다.
- `spec.selector.matchLabels`, `spec.template.metadata.labels`에 명시된 키-값 쌍은 모두 동일해야 한다.
- 만약 특정 조건을 가진 노드에 데몬셋 파드를 배치하고 싶다면 `spec.template.spec.tolerations` 항목을 이용한다. 이에 대한 자세한 정보는 [이 문서](https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/?ref=seongjin.me)를 참고하자.
- 만약 특정 레이블값을 가진 노드에만 데몬셋 파드를 배치하고 싶다면 `spec.template.spec.nodeSelector` 항목을 이용한다. 이에 대해서는 [이 문서](https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/?ref=seongjin.me#running-pods-on-select-nodes)를 참고하자.

<br>

### Job, CronJob

![](https://velog.velcdn.com/images%2Fhyun6ik%2Fpost%2F4d692328-123e-4c62-98f5-638238a9cc45%2Fimage.png)


Job은 하나 이상의 Pod를 지정하고 지정한 수의 Pod를 성공적으로 실행하도록 하는 컨트롤러다.

백업이나 특정 배치 파일들처럼 한번 실행하고 종료되는 성격의 작업에 사용한다.

프로세스를 사용하지 않으면 Pod를 종료한다.(삭제X, 자원을 사용하지 않는 상태)

Job은 다음과 같은 구성요소가 있다.
- `Completions`: 파드가 지정한 개수 이상 처리를 성공할 경우 잡이 성공한다.
- `Parallelism`: 한번에 지정한 개수의 Pod를 병렬로 처리한다.
- `ActiveDeadkineSeconds`: 지정된 시간이 되면 Job이 종료되고 실행중이던 모든 파드 종료한다.


CronJob은 Job들을 주기적인 시간에 따라 생성하는 컨트롤러다. Job하나 단위로 사용하는 경우는 드물다, CronJob을 이용하여 특정 시간에 Job을 생성하는 식으로 사용한다.

그리고 `concurrencyPolicy` 설정을 통해서 Job의 동시 실행처리를 정의할 수 있다.

- `Allow(기본값)`: CronJob은 동시에 실행되는 Job을 허용
- `Forbid`: 동시 실행을 허용하지 않음  기존 작업이 진행중이라면 새로운 Job을 건너뜀
- `Replace`: 기존의 작업을 새로운 작업으로 교체
