# Node Scheduling

Pod를 생성할 때 해당 파드가 어떠한 노드에 배치되어야할지 유저가 지정하거나 쿠버네티스 스케줄러가 지정할 때 사용되는 기능이 스케줄링이며 쿠버네티스는 다양한 스케줄링 방법을 지원한다.

### 특정 Node 선택

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdWoT70%2FbtrrirDFmz8%2FVVzh0Zdw0g11ckz82Nw9OK%2Fimg.png)

NodeName, NodeSelector, NodeAffinity의 방법이 있는데. 하나하나 알아보면 다음과 같다.

- `NodeName`: 노드의 이름을 지정하여 할당한다.
- `NodeSelector`: Node의 label을 지정하여(key value가 같은 값) 할당한다.
- `NodeAffinity`: 노드의 label을 지정하는 것은 같지만 key를 지정하여 더욱 유연하게 할당이 가능하다.

NodeName은 지정하기 간단하다는 장점이 있지만 파드 특성상 파드 이름이 자주 바뀌며 적용하기 힘들다. 그리고 NodeSelector는 특정 라벨이 존재하지 않으면 할당하지 못한다.

그렇기에 더욱 유연한 NodeAffinity를 사용하는데, 이는 노드 셀렉터를 보완하여 사용하는 기능들이 있다. matchExpressions와 Required, Preferred 옵션을 통해 여러 조건을 지정할 수 있다.

- `matchExpressions`: 해당 라벨의 조건을 검사 Exists, In, NotIn과 같은 조건이 잇다.
- `required`: 지정한 조건의 노드가 존재하지 않는다면 파드를 할당하지 않는다.
- `preferred`: 지정한 조건의 노드가 존재하지 않아도 다른 파드에 할당이 가능하다. 조건의 노드가 존재하면 우선적으로 스케쥴러가 해당 노드에 파드를 할당한다.

<br>

### 파드간 집중 분산

노드가 아닌 파드를 기준으로 노드에 파드를 할당하는 방법이다.

이는 두 가지 방법이 존재한다

![](https://blog.kakaocdn.net/dn/n60mL/btrrhRQhn9j/SYQH6O4ntEMic3aY43B7T0/img.png)

- `Pod Affinity`: 같은 노드에 파드를 할당하는 방법이다. 예를 들어 같은 PV hostpath를 참조하고 있는 두 파드라면 같은 노드에 배치되어야한다. 다수의 Pod가 네트워크와 같은 리소스 효율을 위해 동일한 Node에 동작하는 것이 효율적일 때 사용한다.
- `Anti Affinity`: 서로 다른 노드에게 파드를 할당한다. Master, Slave 구조의 파드를 할당한다면 노드가 다운 되었을때 failover역할을 수행하지 못하므로 다른 노드에 배치한다.

<br>

### 특정 노드의 대한 할당 제한

**Toleration/Taint**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FV8EjW%2FbtrrcNVLWoA%2FFvMxd6lbpckG66jYANa1Mk%2Fimg.png)

> Taint: 오염 Toleration: 내성

특정 노드에 배치될 파드를 제한하는 방법이다.

예를들어 클러스터에 CPU가 할당된 노드와 GPU 노드가 배치되어있을때 GPU 노드에는 아무 파드나 배치하고 싶지 않을 수 있다.

그렇기에 특정 노드에 파드 할당을 제한하고 컨트롤한다.

그렇기에 Taint/Toleration을 사용해서 노드에 파드를 할당하는 것을 컨트롤 하는데

1. 먼저 특정 노드에 Taint를 지정하면 해당 노드에는 파드가 배치되지 않는다.
2. 해당 노드에 파드를 배치하려면 Toleration이 필요하다. Toleration에는 특정 조건을 추가할 수 있으며 Taint에 지정된 조건이 맞는 파드들만 배치되게 한다.
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbIKu8K%2FbtrrdYPYZy8%2FhtiPUYQnzOAe2sIem6Pnk0%2Fimg.png)

> 여기서 주의해야할 것은 Toleration은 Taint를 찾아 할당하는 것이 아니며 노드 셀렉터를 지정하여야 노드에 배정이 된다. 그리고 그 배정된 노드중에 Taint가 있으면 그때 Toleration을 검사하는 것이다.


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbZybk9%2FbtrriWQ2Lpl%2FOBqGbCjDh0plBJQxapjgDK%2Fimg.png)

위와 같이 Effect옵션에 대해서 알아보자면 노드에 파드가 있는 상태로 중간에 Taint가 생겼을 때 어떻게 파드를 처리할까? NoSchedule, PreferNoSchedule은 해당 파드를 유지한다. 그러나 NoExecute는 기존 파드를 삭제처리한다.


