# Namespace, ResourceQuota, LimitRange

## Namespace

**Namespace는 쿠버네티스의 하나의 클러스터를 여러개의 논리적인 단위로 분할하는 기술이다.**

특정 애플리케이션을 띄우기위한 파드들의 집합을 분리하기 위해 사용된다.

여러 팀들이 네임스페이스를 공유할 수 있으며 쿼터 설정을 통해 자원 사용량을 제한할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbuPumC%2Fbtrp0ZP1djI%2FwbU2g3aGqCaBq2zo8rZPq0%2Fimg.jpg)

쿠버네티스는 클러스터 다음으로 분리단위가 크기 때문에 가장 먼저 검토해야할 단위기도 하다.

네임스페이스는 여러개의 팀이 구성되어있거나, 프로젝트의 많은 사용자가 있는 환경에 적합하다. 유저 수가 적거나 수십명밖에 되지 않은 프로젝트는 사용하기에 적합하지 않다.

쿠버네티스를 처음 설치하면 기본으로 몇 개의 네임스페이스가 생성된다. `kubectl` 명령어로 현재 생성되어 있는 네임스페이스를 확인할 수 있다.

```
$ kubectl get namespaces
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNilL4%2Fbtrp0J7pdiQ%2F4RiKT9f14sJuyREy0ke9y1%2Fimg.png)

- `default`: 네임스페이스를 지정하지않으면 default 네임 스페이스에 파드가 저장된다.
- `kube-system`: 쿠버네티스 관리 시스템에서 관리하는 네임스페이스로, 쿠버네티스 **관리용 파드**가 설정되어 있다.
- `kube-public`: 클러스터 안 모든 사용자가 읽을 수 있는 네임스페이스, 보통 **클러스터 사용량** 같은 정보를 이 네임스페이스에서 관리합니다. 클러스터를 사용하는 모두가 볼 수 있기 때문입니다.
- `kube-node-lease`: 각 노드의 임대 오브젝트(Lease Object, 혹은 워크로드 리소스)들을 관리하는 네임 스페이스, 1.13 이후 알파 기능으로 추가됨

namespace를 생성하는 방법은 두 가지로 yaml파일과 명령어를 사용하는 방법 이렇게 두 가지가 있다

### yaml 
```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: <insert-namespace-name-here>
```

```bash
kubectl create -f ./my-namespace.yaml
```

### cli
```
kubectl create namespace <insert-namespace-name-here>
```


<br>

## ResourceQuota

리소스 쿼터는 네임스페이스별 사용할 수 있는 자원을 제한하는 역할을 한다.

네임스페이스들은 클러스터들의 리소스들을 공유하며 사용한다.

그렇기에 한 네임스페이스가 자원들을 독점하는 상황때문에 다른 네임스페이스에서 자원을 사용하지 못하는 문제 상황이 발생할 수가 있다.

그렇기에 각 네임스페이스별 사용할 수 있는 최대 자원을 제한함으로써 문제를 해결하는 것이다.

```yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: rq-1
  namespace: nm-3
spec:
  hard:
    requests.memory: 1Gi
    limits.memory: 1Gi
```

리소스 쿼터는 요청으로 들어온 파드의 자원량과 이미 실행되고 있는 파드의 자원량을 합쳐 limit가 넘는다면 파드를 생성하지 않고 실패처리하고, 넘지 않아야만 파드를 생성한다.

> ResourceQuota를 사용하면 request, limit 값을 지정해주지 않으면 파드가 생성되지 않는다.
<br>

### LimitRange

컨테이너는 쿠버네티스 클러스터에서 무제한 컴퓨팅 리소스로 실행되어진다.

리소스 쿼터를 사용하면 클러스터 관리자는 네임스페이스별로 리소스 사용량을 조절할 수 있다.

그러나, 각 네임스페이스안에 한 파드가 제한된 사용량 내에서 자원을 독점할 수 있는 문제가 있다.

그래서 **네임스페이스의 파드별로 리소스 상한량을 지정하는 설정하는 기능**이 리미트 레인지다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F1pWEh%2Fbtrp5AWfQzD%2FaSD5z6JADxzyzEzKSBKMJK%2Fimg.png)

```yaml
apiVersion: v1
kind: LimitRange
metadata:
  name: lr-1
spec:
  limits:
  - type: Container
    min:
      memory: 0.1Gi
    max:
      memory: 0.4Gi
    maxLimitRequestRatio:
      memory: 3
    defaultRequest:
      memory: 0.1Gi
    default:
      memory: 0.2Gi
```

위와 같이 지정할 수 있으며 `maxLimitRequestRatio`는 request와 limit의 배수 비율이며

request, limit을 지정하지 않으면 default 값으로 지정된다.
 

> 여기서 주의해야할 점은 LimitRange를 여러개 사용해야 된다면, 예측 불가능한 동작(default 값이 다른 LimitRange에 지정된 값으로 설정되어 별개의 LimitRange에 설정된 max를 초과한다거나)이 발생할 수 있기 때문에 주의해서 사용해야한다.


[[Kubernetes Cluster]] [[Control Plane]] [[Pod]] [[Service]]