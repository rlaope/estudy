# Deployment

Deployment는 **파드와 레플리카셋에 대한 선언적 업데이트를 제공하는 리소스다.**

롤링 업데이트를 비롯하여 애플리케이션의 버전 및 배포 관리에 주로 쓰인다.

> 레플리카셋이 파드들의 상태 체크와 개수 유지를 담당한다면, 디플로이먼트는 이 레플리카셋을 포함하는 상위 객체로서 애플리케이션의 서비스 단위의 관리를 위해 쓰인다고 보면 된다.

Deployment의 리소스로 배포된 파드들은 모두 `<디플로이먼트명>-<레플리카셋고유번호>-<랜덤해시값>` 형태의 식별자를 가진다.

만약 레플리카 설정이 변경되거나, 파드가 삭제 후 재배포된 상황이라면 식별자도 함께 달라진다.

즉, 디플로이먼트에서 파드들의 식별자는 모두 상황에 따라 유동적으로 변화한다.

<br>

### Deployment 배포하기

1. **CLI 방식으로 생성하기**
```bash
kubectl create deployment nginx --image=nginx:1.14.2 --replicas=3
```

- 위 명령은 `kubectl create deployment <디플로이먼트명> --image=<이미지명> --replicas=<레플리카수>` 형태로 구성된다.
- --image로 지정된 이미지의 파드를 --replicas의 수 만큼 생성하고 유지하는 `디플로이먼트명`의 디플로이먼트를 생성한다.
- --image 플래그는 필수다. --replicas는 지정하지 않을 경우 기본값인 1로 지정된다.

<br>

2. **Yaml 파일로 생성하기** (YAML 파일을 작성, 저장한 뒤 터미널 화면에서 `kubectl apply -f <파일명.yaml>`을 실행한다.)
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

- `kind`의 값은 반드시 `Deployment`여야 한다. (대소문자 구분 필수)
- 레플리카의 수는 `spec.replicas`에 입력한다.
- 배포할 파드의 정보들은 `spec.template`아래에 위치한다.
- 파드안에 들어갈 컨테이너의 이미지 정보는 `spec.template.spec.containers` 아래에 입력한다.
- 이렇게 배포된 파드들을 일괄 관리하기 위해 `spec.selector.matchLabels`항목을 통해서 키-값 쌍을 부여한다.
- `spec.selector.matchLabels`, `spec.template.metadata.labels`에 명시된 키-값 쌍은 반드시 동일해야 한다.

<br>

### 배포된 디플로이먼트의 설정 바꾸기

디플로이먼트로 배포되어 이미 구동중인 파드의 이미지를 업데이트하거나

레플리카의 수를 조정할 때가 있다.

아래와 같이 설정을 바꿀 수 있다.

#### 이미지 업데이트

우선 CLI 환경에서는 `kubectl set`명령을 사용할 수 있다.

아래 명령을 통해서 nginx-deployment의 nginx 앱에 대해 컨테이너 이미지를  `nginx:1.14.2`에서 `nginx:1.16.1`로 업데이트하는 내용이다.

```bash
kubectl set image deployment/nginx-deployment nginx=nginx:1.16.1
```

다음으로는, `kubectl edit` 명령으로 배포된 디플로이먼트의 YAML 정보를 불러와 컨테이너 이미지 항목을 수정하는 방법이 있다. 이때 수정해야 할 항목은

`.spec.template.spec.containers[0].image`에 위치해 있다.

```bash
kubectl edit deployment/nginx-deployment
```

만약 YAML 파일로 직접 생성한 경우라면, 해당 YAML 파일을 열어 `spec.template.spec.containers`에 `image` 항목을 `nginx:1.16.1`로 변경한 뒤 `kubectl apply` 명령을 사용해도 된다.


#### 스케일링

CLI환경에서 간단히 레플리카수를 조정하고 싶다면 `kubectl scale` 명령을 이용할 수 있다.

아래 명령을 실행하면 nginx-deployment의 레플리카 수가 5개로 즉시 늘어난다.

```bash
kubectl scale deployment/nginx-deployment --replicas=5
```

단, 위의 명령은 실제 배포에 쓰인 YAML 파일 내용과 무관하게 명시적으로만 적용되는 변화다. `kubectl edit`이나 기존 배포에 쓰인 YAML 파일을 직접 수정 후 적용하는 방법도 있으므로 용도에 따라 방법을 선택하면 좋다.

