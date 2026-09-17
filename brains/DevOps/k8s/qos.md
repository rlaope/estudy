# Qos Classes

쿠버네티스에서 노드의 메모리 자원이 부족해지면 어떤 파드나 프로세스를 종료해서 다른 파드에 재할당하는 작업과 같은 것들을 진행해야한다.

그럼 어떤 기준으로 재할당을 진행하는 것일까? 한 번 이 기준에 대해서 알아보도록 하자.

### Kubernetes OOM

그 전에 쿠버네티스 OOM(Out of Memory)에 대해서 알아보도록 하자.

```bash
kubectl describe nodes {node_name} | grep -A9 Conditions
```

쿠버네티스의 각 노드에는 노드의 이상 상태 정보를 포함하는 conditions라는 것이 있다 

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc4rJ7j%2Fbtrao550MVa%2FR9YC260EBREwKn5lYgBh10%2Fimg.png)

MemoryPressure는 기본적으로 가용 메모리가 100Mi 이하일 때 발생하도록 Kubelet에 설계되어있다.

MemoryPressure가 발생하게 되면 기본적으로 해당 노드에서 실행되고있던 모든 노드의 파드들을 우선순위를 매겨 가장 낮은 우선순위의 파드를 퇴거(evict)시킨다.

뿐만 아니라 _**MemoryPressure가 True인 노드에는 더 이상 Pod를 할당하지 않는다..**_
이때 Pod의 우선순위는 **QoS 클래스 및 메모리 사용량**에 따라 정렬되어 매겨진다.


<br>

### QoS Classes

쿠버네티스는 Pod의 **Request, Limit을 따라서 QoS 클래스**를 지정한다.

그리고 QoS클래스는 **BestEffort, Burstable, Guaranteed** 이렇게 3가지가 존재한다.

### BestEffort

```yaml
apiVersion: v1 
kind: Pod 
metadata: 
	name: nginx-besteffort-pod 
spec:
.   containers: 
	- name: nginx-besteffort-pod 
	- image: nginx:latest
```

아래와 같이 resources 항목(request, limit)을 사용하지 않을 때 BestEffort가 된다.

```bash
kubectl describe pod nginx-besteffort-pod | grep QoS
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FCyteG%2FbtrakpxxYXK%2FrupKBK5d7coTXSZGKGg2UK%2Fimg.png)

위와 같이 resources항목을 사용하지 않으면 한계된 자원이 없으며 유휴 자원이 존재하지 않는다면 제한 없이 모든 자원을 사용할 수 있게 된다. 

그리고 request를 설정하지 않았기 때문에 보장받을 수 있는 자원은 존재하지 않습니다.

_즉, 때에 따라서는 노드에 존재하는 모든 자원을 사용할 수도 있지만, 자원을 전혀 사용하지 못할 수도 있습니다._

BestEffort가 노드에서 OOM이 발생할 경우 가장 먼저 자원을 회수당한다.

<br>

###  Guaranteed

```yaml
apiVersion: v1
kind: Pod 
metadata: 
	name: nginx-guaranteed-pod 
spec: 
	containers: 
	- name: nginx-guaranteed-pod 
	- image: nginx:latest 
	resources:
		limits: 
			memory: "256Mi"
			cpu: "1000m" 
		requests: 
			memory: "256Mi" 
			cpu: "1000m"
```

위의 yaml 파일처럼 limit과 request의 값이 같으면 **Guaranteed**등급으로 매겨지게된다.

```bash
kubectl describe pod nginx-guaranteed | grep QoS
```

Pod의 request와 limit이 같기 때문에 오버 커밋이 허용되지 않으며 자원 사용을 한정적으로 보장받을 수 있게 된다.

- _Guaranteed 클래스 내부에서 실행되는 프로세스들은 모두 기본 **OOM 점수(oom_score_adj)가 -998**로 설정됩니다._
- _Pod 내 **여러 개의 컨테이너**가 존재한다면 **모든 컨테이너의 Request와 limit이 동일**해야 Guaranteed로 분류되게 됩니다._

<br>

### Burstable

```yaml
apiVersion: v1 
kind: Pod 
metadata: 
	name: nginx-burstable-pod 
spec: 
	containers:
	- name: nginx-burstable-pod 
	- image: nginx:latest 
	resources: 
		limits:
			memory: "1024Mi" 
			cpu: "1000m"
		requests: 
			memory: "256Mi" 
			cpu: "500m"
```

```bash
kubectl describe pod nginx-burstable | grep QoS
```

위의 yaml 파일처럼 limit이 request보다 클 때(멀티 컨테이너라면 다 같지 않을 때) Burstable이다.

간단하게 생각하면 request만큼 지정된 리소스를 사용할 수 있지만 상황에 따라서 limit 만큼 사용할 수 있다는 뜻이다.


<br>

### 우선순위

맨 위에서 우선순위에 따라서 파드를 관리한다고 언급했다.

그럼 위의 3가지 QoS 클래스에서 우선순위는 어떻게 될까?

보통은  **Guaranteed > Burstable > BestEffort** 순이다.

보통은? 이라면 아닐때도 있다는 뜻이다 가끔 상황에 따라서 Burstable과 BestEffort 간에는   
**메모리의 사용량**에 따라 우선순위가 바뀔 수 있습니다. ( _즉 , Pod가 메모리를 많이 사용할수록 우선순위가 낮아집니다. )_


[[Pod LifeCycle(Phase, Condition)]]