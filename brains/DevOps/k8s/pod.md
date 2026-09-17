# Pod

Pod은 쿠버네티스의 가장 기본적인 배포 단위다.

마스터 노드에서는 워커노드로 Pod을 전달하고 워커노드에서는 Pod을 수행하는 구조다.

파드는 1개 이상의 컨테이너가 캡슐화 되어 클러스터 안에서 배포되는 가장 작은 단위라고 생각하면 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FctN0nC%2FbtqM5VlDbOg%2Fhz31qGnbzLxAnnsfThKwGk%2Fimg.png)

파드는 다음과 같은 특징들을 가지고 있다.

1. 기본적으로 하나의 파드에는 **하나 이상의 컨테이너**가 포함된다. 필요에 따라 여러개가 표함될 수도 있다.
2. 파드는 노드IP와 별개 고유 IP를 할당 받으며, 파드 안에 컨테이너들이 공유한다.
3. 파드 자체는 일반적으로 1개의 IP만을 가질 수 있다.(단, Multus CNI 이용 등 특정 조건에 한해 2개를 가질 수도 있긴하다.)
4. 파드 안에 컨테이너들은 동일한 볼륨을 연결할 수 있다.
5. 파드는 클러스터내에 배포 최소 단위이고 특정 네임스페이스 안에서 실행된다.
6. 파드는 기본적으로 **반영속적이다.**

> 반영속적이라는 말은, 결국 쿠버네티스에서 파드는 무언가 구동중인 상태를 유지하기 위해 동원되는 일회성 자원이며, 필요에 따라서 언제든 삭제될 수 있다는 것이다 이 점을 유념하자. 영속적자원으로는 [[Service]] 가 존재한다.

  
### 왜 Pod 단위로 묶어서 배포하는 것일까?

  
1. Pod 내부 컨테이너 간의 IP 및 Port 공유를 통한 용이성 향상

N개의 컨테이너가 한 개의 Pod에 탑재되어 배포된 어플리케이션을 고려해보자.

이 어플리케이션 안의 컨테이너끼리는 실시간으로 데이터를 교환하며, 그에 따라 상태를 업데이트 한다.

두 컨테이너는 별도의 IP호출 없이 localhost를 통해 통신이 가능하다.

## Container

Pod는 기본적으로 한 개 이상의 컨테이너들로 구성되어 있다.

Pod 안의 컨테이너들은 서비스가 서로 연결될 수 있도록 포트를 가지고 있다.

여기서, 한 컨테이너는 하나 이상의 포트를 가질 수 있지만, 한 포트를 여러 컨테이너가 공유하지는 못하게 됩니다.

Pod은 또한 생성될 때 고유 ip가 생성됩니다.

Pod를 IP를 통해서 접근할 경우 쿠버네티스 클러스터 내에서만 접근이 가능하고, 외부에서는 접근이 불가능합니다.

  
<br>

## Label

Pod 뿐만 아니라 모든 오브젝트에서 다 쓰입니다.

하지만, Pod에서 가장 많이 사용합니다.

Label을 사용하는 이유는 목적에 따라 오브젝트들을 분류할 수 있으며, 오브젝트들을 따로 연결하기 위해서 사용합니다.

구성은 키와 밸류 형태로 만들어집니다.
  
<br>

## Node Schedule

Pod은 여러 노드들 중 한 노드에 올라가야합니다.

직접 노드를 선택하는 방법과 쿠버네티스가 자동으로 선택하는 방법이 존재합니다.

쿠버네티스 스케쥴러가 판단하여 노드를 고를 수 있습니다
- CPU 사용량
- 메모리 사용량


### 파드 정의

단일 컨테이너 파드(파드 내에 컨테이너 한개)를 정의해보겠다. nginx 예시

명령어로도 할 수 있지만 이번에는 manifest로 작성해보겠다. yaml파일을 생성해준다.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx
  labels:
    app: nginx
spec:
  containers:
  - name: nginx-container
    image: nginx
```

- `apiVersion`:  객체 생성에 쓰일 쿠버네티스 api 버전을 의미한다. 객체 종류에 따라 버전이 조금 다른 것을 유의하고 파드와 서비스는 v1, 레플리카셋과 디플로이먼트등은 apps/v1으로 쓰인다.
- `metadata.labels`: 파드 구동에 영향을 주지 않는 순수 키-밸류 메타 데이터 영역이다. 원하는 조합으로 부여할 수 있다. 여러 파드가 돌아가는 환경에서 여러 작업(재배포 등)수행을 특정 파드들만 실행되도록 할 수 있다.
- `spec.containers`: 실제 파드에 담길 컨테이너의 속성을 정의한다. 도커 컴포즈와 유사한 내용이라고 생각하면 된다.

아래는 주요 명령어다 보고 외우자(ㅠㅠ)

```bash
# 클러스터 내 파드 목록 조회(default 네임스페이스)
kubectl get pods

# 클러스터 내 특정 네임스페이스의 파드 목록 조회
kubectl get pods -n <namespace>

# 파드 목록의 상세 조회(사설IP, 노드정보 포함)
kubectl get pods -o wide

# nginx 파드를 수정
kubectl edit pod nginx

# nginx 파드의 상세 정보 확인
kubectl describe pod nginx

# nginx 파드 내 구동 중인 컨테이너의 로그 확인
kubectl logs nginx				

# nginx 파드의 컨테이너에 접속하여 sh를 대화형으로 실행
kubectl exec -it nginx -- /bin/sh 

# nginx 파드 삭제
kubectl delete pod nginx
```

> 네임스페이스는 하나의 클러스터 안에서 다양한 워크로드 리소스들이 필요에 따라 격리하는 데 사용되는 요소다 나중에 더 알아보도록 하겠다.


멀티 컨테이너 파드 관리를 하려면 `containers`로 작성해 이 부분에 여러 컨테이너들을 정의하면 된다.

yaml 파일로 파드를 생성하는 방법은 

```bash
# 생성된 yaml 파일로 파드 구동(선언형 방식)
kubectl apply -f pod1.yaml

# 생성된 yaml 파일로 파드 구동(명령형 방식)
kubectl create -f pod1.yaml
```

하나 추가 팁으로 yaml 명세의 기본 골격을 파일 형태로 저장할 수도 있다.

```bash
# Redis 파드 명세를 yaml 파일로 생성
kubectl run redis --image=redis --dry-run=client -o yaml > redis.yaml
```

> 멀티 컨테이너 파드의 경우, 컨테이너 로그 확인 또는 접속시 컨테이너 명을 지정해줘야한다.