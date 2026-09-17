# 멀티 컨테이너 파드 디자인 패턴


### 멀티 컨테이너 파드란?

멀티 컨테이너 파드는 한 파드내에서 여러개의 컨테이너가 구동되어 있는 형태의 파드다.

여러 컨테이너들은 같은 IP를 가지고 있으며 서로 다른 포트를 공유하고 있다.

한 파드의 여러 컨테이너들은 서로 다른 목적, 성격을 가진 컨테이너를 배치하면 안된다.

이는 쿠버네티스의 설계사상에 맞지 않는 방식이기 때문이다.

그래서 멀티 컨테이너 파드는 **메인 프로세스를 네트워크 또는 스토리지의 밀접한 공유가 필요한 다른 컨테이너와 함께 운영하고자 할때** 고려하는 것이 바람직하다.

이러한 멀티 컨테이너 파드를 구축할때 권장되는 디자인 패턴들이 있으며 세 가지를 알아보겠다.

그리고 마지막에는 init 파드에 대해서도 함께 알아보도록 하겠다.

![](https://seongjin.me/content/images/2022/02/pod-design-patterns.png)


위와 같이 3가지의 패턴이 존재한다. 하나씩 알아보자

<br>

### 사이드카 (Sidecar) 컨테이너 

사이드카 컨테이너는 서로 다른 두 컨테이너가 같은 파일 시스템을 공유해 보조하는 패턴이다.

실제로 많은 수의 파드가 사이드카로 디자인 되어 있다.

![](https://seongjin.me/content/images/2022/02/sidecar-container.png)

예를 들면 nginx와 nginx의 로그를 수집하는 두 개의 컨테이너로 볼 수 있다.

1. 파드 안에 Nginx의 최신 로그를 스트리밍하는 사이드카 컨테이너를 따로 둔다.
2. 이 컨테이너가 Nginx 로그를 stdout 또는 stderr로 내보낸다.
3. 별도로 운영되는 Logging backend(logging agent pod) 또는 logrotate 같은 별도의 로그 관리 서비스가 여기에 접근하게 된다.

위의 예시에 대한 가장 간단한 형태의 구현물을 아래 YAML 형식으로 살펴보자.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx-sidecar
spec:
  containers:
  - name: nginx
    image: nginx
    ports:
    - containerPort: 80
    volumeMounts:
    - name: logs
      mountPath: /var/log/nginx
  - name: sidecar-access
    image: busybox
    args: [/bin/sh, -c, 'tail -n+1 -f /var/log/nginx/access.log']
    volumeMounts:
    - name: logs
      mountPath: /var/log/nginx
  - name: sidecar-error
    image: busybox
    args: [/bin/sh, -c, 'tail -n+1 -f /var/log/nginx/error.log']
    volumeMounts:
    - name: logs
      mountPath: /var/log/nginx
  volumes:
  - name: logs
    emptyDir: {}
```
- 80포트로 nginx 컨테이너를 띄운다.
- 두 개의 busybox 컨테이너를 추가로 띄운다.
- 이 `busybox` 컨테이너들은 각각 `nginx`의 `/var/log/nginx/`에 기록되는 `access.log`와 `error.log`를 스트리밍하는 사이드카 컨테이너로서 기능한다.
- 스트리밍 기능을 위해서 `tail -n+1 -f`라는 쉘 명령어를 구동하도록 작성해놓았다.
- 이렇게 스트리밍되는 로그들은 `kubectl logs nginx-sidecar [컨테이너명]` 명령어를 통해서 확인할 수 있다. (metadata.name)

<br>

### 어댑터 (Adapter) 컨테이너

어댑터 컨테이너는 주로 파드에 탑재된 특정 애플리케이션의 출력물 규격을 맞추도록 다듬는 용도로 사용된다.

![](https://seongjin.me/content/images/2022/02/adapter-container.png)

예시 상황으로 dataformat을 볼 수 있다. 

여러 오픈소스들을 사용하다보면 date를 가지고 올때 `YYYY-MM-DD`형식이나 `DD/MM/YYYY` 형식처럼 같은 데이터지만, 다른 규격의 출력물들을 얻을 수 있는 것을 확인할 수 있다.

이러한 출력물들의 규격을 맞추기 위한 목적으로 어댑터 컨테이너를 디자인한다.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: adapter-container-demo
spec:
  containers:
  - image: busybox
    command: ["/bin/sh"]
    args: ["-c", "while true; do echo $(date -u)'#This is log' >> /var/log/file.log; sleep 5;done"]
    name: main-container
    resources: {}
    volumeMounts:
    - name: var-logs
      mountPath: /var/log
  - image: bbachin1/adapter-node-server
    name: adapter-container
    imagePullPolicy: Always
    resources: {}
    ports:
      - containerPort: 3080
    volumeMounts:
    - name: var-logs
      mountPath: /var/log
  dnsPolicy: Default
  volumes:
  - name: var-logs
    emptyDir: {}
```

- 위의 예제는 `busybox` 이미지가 구동되는 메인 컨테이너의 로그를 출력한 뒤 로그 출력을 `bbachin1/adapter-node-server`가 받은 뒤 js파일로 포멧을 맞춘 뒤 출력하는 예제다.
- 이처럼 특정한 로그나 출력물을 보편적인 포맷으로 변환하여 주는 것이 어댑터 컨테이너의 주요한 용도 중 하나다.

<br>

### 앰버서더 (Ambassador) 컨테이너

앰버서더 컨테이너는 **파드의 외부 서비스와의 연결을 간소화** 해주는 목적으로 디자인 되었다.

메인 컨테이너가 수행해주어야 할 외부 통신을 대신 해주는 역할을 해주어 Ambassador(대사)라고 불리는 것이다.

![](https://seongjin.me/content/images/2022/02/ambassador-container.png)

이 구조에서 메인 컨테이너는 오직 앰버서더를 통해서만 외부 서비스와 통신할 수 있다.

**결국 메인 애플리케이션 컨테이너의 프록시 역할을 수행해주는 컨테이너라고 볼 수 있다.**


아래 예시는 [Magalix 사의 기술 블로그](https://www.magalix.com/blog?ref=seongjin.me)에서 소개된 내용으로, 파드 안의 `redis` 서버가 외부의 `redis` 인스턴스들에 접근할 수 있도록 프록시 역할의 컨테이너를 활용하는 사례를 구현한 것이다. 보다 자세한 설명은 [원작자의 블로그 게시물](https://www.magalix.com/blog/kubernetes-patterns-the-ambassador-pattern?ref=seongjin.me)에서 확인할 수 있다.

위 게시물에서 앰버서더 컨테이너가 포함된 전체 파드의 정의 내용(YAML)을 살펴보면 다음과 같다.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: ambassador-example
spec:
  containers:
  - name: redis-client
    image: redis
  - name: ambassador
    image: malexer/twemproxy
    env:
    - name: REDIS_SERVERS
      value: redis-st-0.redis-svc.default.svc.cluster.local:6379:1 redis-st-1.redis-svc.default.svc.cluster.local:6379:1
    ports:
    - containerPort: 6380
```

- 위 파드의 메인 프로세스는 `redis-client`로, 클라이언트 역할을 하는 `redis` 컨테이너다.
- 앰버서더 역할의 `ambassador` 컨테이너는 프록시 역할을 하며, `6380` 포트를 이용하여 `redis` 컨테이너와 `REDIS_SERVERS`로 명명된 네트워크 위치 간의 연결을 맡는다.
- `REDIS_SERVERS`로 명명된 위치는, `redis-svc` 서비스를 바라보는 `redis-st-0`과 `redis-st-1` 파드를 의미한다. 따라서 위의 설정대로 파드가 정상 구동되려면, `redis-svc` 서비스와 더불어 `redis-st`라는 이름의 스테이트풀셋(StatefulSet) 생성이 추가로 필요할 것이다. 이 리소스들에 대해서는 다음 글에서 자세히 다룰 것이다.

<br>

### 초기화 (Init) 컨테이너

**파드의 메인 컨테이너 실행 전에 일종의 초기화 작업을 진행하는 컨테이너다.**

메인 컨테이너의 파드가 실행이 되기 전에 필요한 환경 조건등을 걸어두고 완료되면 메인 컨테이너의 프로세스를 시작하도록 하는 컨테이너다.

1. 모든 초기화 컨테이너는 작업이 완료되면 **반드시 종료**되어야한다.
2. 초기화 컨테이너는 반드시 종료되어야하기 때문에 **프로브를 사용할 수 없다. **(livenessProbe, readinessProbe, startupProbe)
3. 만약 초기화 컨테이너를 여러개를 운영하게 된다면 초기화 컨테이너는 **순차적으로 실행**되게 된다.
4. 초기화 컨테이너의 초기화 작업이 실패하게 되었다면 **kubelet은 주기적으로 해당 컨테이너를 재시작한다.** 이 옵션은 `restartPolicy`를 통해 몇번 재시작할지 설정할 수 있으며 `Never`로 설정하게 된다면 해당 초기화 컨테이너가 실패한다면 Failure 상태로 종료되게 된다.


```yaml
apiVersion: v1
kind: Pod
metadata:
  name: myapp-pod
  labels:
    app: myapp
spec:
  containers:
  - name: myapp-container
    image: busybox:1.28
    command: ['sh', '-c', 'echo The app is running! && sleep 3600']
  initContainers:
  - name: init-myservice
    image: busybox:1.28
    command: ['sh', '-c', "until nslookup myservice.$(cat /var/run/secrets/kubernetes.io/serviceaccount/namespace).svc.cluster.local; do echo waiting for myservice; sleep 2; done"]
  - name: init-mydb
    image: busybox:1.28
    command: ['sh', '-c', "until nslookup mydb.$(cat /var/run/secrets/kubernetes.io/serviceaccount/namespace).svc.cluster.local; do echo waiting for mydb; sleep 2; done"]
```

위와 같은 예제에서 초기화 container init my-service, init-mydb 총 두개의 초기화 컨테이너를 선언했다.

둘의 역할은 myservice, mydb가 발견될 때까지 2초마다 로그를 출력하는 것이다

해당 파드를 배포한뒤 `kubectl get`으로 상태를 확인하면 다음과 같이 나오게된다.

```bash
NAME        READY     STATUS     RESTARTS   AGE
myapp-pod   0/1       Init:0/2   0          2m
```

READY는 0/1 상태로 아직 해당 컨테이너가 띄워지지 않았다는 것이다.

STATUS는 초기화 컨테이너가 아직 두개다 종료되지 않은 상태를 의미한다.

그럼 앞서 해당 yaml에 정의한 초기화 컨테이너들의 조건을 충족시켜주기 위해서
`kubectl apply -f`명령으로 배포한다.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: myservice
spec:
  ports:
  - protocol: TCP
    port: 80
    targetPort: 9376
---
apiVersion: v1
kind: Service
metadata:
  name: mydb
spec:
  ports:
  - protocol: TCP
    port: 80
    targetPort: 9377
```

서비스들의 배포 상태를 확인한 후, 아까 살펴봤던 파드 `myapp-pod`의 상태 정보를 다시 체크해보자. `READY` 항목과 `STATUS` 항목이 나란히 바뀌어 정상 구동 중임을 확인할 수 있을 것이다.

```bash
NAME        READY     STATUS    RESTARTS   AGE
myapp-pod   1/1       Running   0          5m
```

이후 Running 상태인 파드는 추후 초기화 조건이 달라지더라도 계속 Running 상태기 때문에 이 점을 유의하자


