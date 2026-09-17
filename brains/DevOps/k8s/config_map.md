# ConfigMap, Secret

쿠버네티스에서 파드의 개발 환경을 dev, prod로 나눈다거나 보안 접속(SSH) 설정을 개발서버에서는 false, 운영에선 true와 같이 각 환경마다 다른 값을 가지고 있을 때 어떻게 처리할 수 있을까?

각기 다른 컨테이너를 운영한다는 것인데 공통된 값도 있을 수 있으니 하나하나 설정해서 새로운 컨테이너로 배포하는 것은 번거러운 일이다.

**필요한 환경설정 내용을 컨테이너와 분리해서 제공해 주기 위한 기능**이 필요하다.

그렇기에 이러한 설정 env 들을 정의해서 컨테이너 외부에서 관리하고, 사용할 수 있는 것이 바로 ConfigMap이다.

ConfigMap은 Key - Value의 형태로 데이터를 저장한다. (key, value는 모두 string형으로 저장되기 때문에 boolean과 같은 값도 'false' 이런식으로 커테이션('')으로 감싸주어야한다.)

Secret은 ConfigMap과 같은데 value값이 인코딩되어있어야한다.

<br>

### ConfigMap 정의

아래 예시는 데이터베이스의 설정 파일들을 ConfigMap으로 정의한 것이다.
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
- `data` 아래에 key-value 형식으로 데이터를 저장하면 된다.

config map에 데이터를 정의하는 방법은 상수형으로 Key-Value형식을 직접 입력할 수도 있고 file을 직접 config map의 값으로 집어넣을 수도 있다.

key-value형식은 마찬가지로 위와 같이 정의하면 된다.

### 파일로 정의

파일 저장 형식은 file.txt 파일을 작성했다고 했을때 

```bash
kubectl create configmap dev-file --from-file=./file.txt
```

다음과 같이 config맵의 이름을 지정해주고 값을 가져올 파일을 지정한다.


시크릿은 다음과 같이 작성한다.
```bash
kubectl create secret generic sec-file --from-file=./file2.txt
```

여기서 주의할 점은 txt파일의 내용은 인코딩하면 안된다 명령어를 입력한 순간 인코딩이 진행되어있기 때문에 이미 인코딩한 내용이 txt파일에 들어있다면 두번 인코딩하게 되는 문제가 발생한다.

### ConfigMap 사용

위의 정의한 컨피그맵을 사용해보겠다. 다음과 같이 파드를 yaml 파일에 정의하면 된다.

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

valueFrom을 통해서 값을 불러와 key를 지정했다. envFrom을 가져오면 env형식의 key-value를 불러오게 된다.

<br>

### 볼륨에 마운트

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

다음과 같이 컨피그맵에 데이터를 볼륨에 마운트 시킬 수도 있다.

> env: 를 사용해서 값을 하나씩만 가져올 수도 있고 전부 가져오려면 envFrom을 사용한다.


