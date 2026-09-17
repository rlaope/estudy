# Pod LifeCycle(Phase, Condition)

Pod는 생성부터 삭제까지 생명주기가 있으며, 다음과 같은 상태(phase)를 가진다

### Phase

- `Pending`: 파드의 작성을 기다리고 있는 상태, 컨테이너 이미지의 다운로드 등에 시간이 걸리는 경우가 있습니다.
- `Running`: 파드가 가동 중인 상태
- `Succeeded`: 파드 안의 컨테이너가 정상적으로 종료된 상태
- `Failed`: 파드 안의 컨테이너 중 하나의 컨테이너가 실패하여 종료된 상태
- `Unknown`: 어떤 이유로 파드와 통신할 수 없는 상태

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FLhLJv%2FbtrqVvAAFYB%2FkOuhB1yhS2PCcMfr1T7Zk0%2Fimg.png)

파드는 Pending 단계에서 시작해서, 기본 컨테이너 중 적어도 하나 이상이 OK로 시작하게 되면 Running 단계를 통과하고, 그런 다음 파드의 컨테이너가 실패로 종료되었는지 여부에 따라 Succeeded 또는 Failed 단계로 이동합니다.

현재 파드의 생명 주기는 다음 명령어로 확인할 수 있습니다.

`kubectl describe pods <파드이름>`

현재 Running 중인 Pod의 상태를 확인해보도록 하겠습니다.  

Status 항목을 살펴보면 현재 파드의 생명주기를 알 수 있습니다. 

```
[root@k8s-master ~]# kubectl describe pods nginx-deployment-69cfdf5bc7-gcpd2
Name:         nginx-deployment-69cfdf5bc7-gcpd2
Namespace:    default
Priority:     0
Node:         k8s-node2/192.168.56.32
Start Time:   Fri, 07 Jan 2022 13:35:49 +0000
Labels:       app=nginx-deployment
              pod-template-hash=69cfdf5bc7
Annotations:  cni.projectcalico.org/containerID: b55493deca71e924b1f262af9656954acd426c220b1c026a86de2f640688822f
              cni.projectcalico.org/podIP: 20.109.131.22/32
              cni.projectcalico.org/podIPs: 20.109.131.22/32
Status:       Running

# 중간 생략 
Conditions:
  Type              Status
  Initialized       True 
  Ready             True 
  ContainersReady   True 
  PodScheduled      True 
  
# 이후 생략
```


> Pod 생성한 후 Pod의 status 필드는 phase 필드를 포함하는 PodStatus 오브젝트로 정의된다.

### Condition

Status 항목을 보면 현재 파드가 Running 상태입니다. 그리고 **Conditions** 항목은 파드의 현재 상태 정보를 나타내며 Type과 Status로 구분되어 있습니다.

하나의 PodStatus를 가지며 통과한 파드나 통과하지 않은 파드들을 PodConditions 배열을 가진다.

kubelet은 다음과 같은 PodConditions를 관리한다.

- `PodScheduled`: 파드가 노드에 스케줄 되었다.
- `PodHasNetwork`: (알파 기능; 반드시 명시적으로 활성화해야 함) 샌드박스가 성공적으로 생성되고 네트워킹이 구성되었다.
- `ContainersReady`: 파드의 모든 컨테이너가 준비되었다.
- `Initialized`: 모든 초기화 컨테이너가 성공적으로 완료(completed)되었다.
- `Ready`: 파드는 요청을 처리할 수 있으며 일치하는 모든 서비스의 로드 밸런싱 풀에 추가되어야 한다.

노드에 파드를 할당하면 kubelet은 컨테이너 런타임 엔진을 통해 컨테이너를 생성을 시작한다.

컨테이너의 상태를 표시할 수 있는 상태는 3가지로 Waiting, Running 그리고 Terminated이 있다.

- `Waiting`: Waiting은 컨테이너가 시작하기 위해 완료해야하는 작업(이미지를 레지스트리에서 가져온다거나 시크릿 데이터를 적용한다거나)을 실행하고 있는 상태다.
- `Running`: 컨테이너가 정상적으로 시작되어 실행되고 있음을 나타낸다.
- `Terminated`: 컨테이너가 완료될 때 까지 실행되었거나 어떠한 이유로 실패했을때의 상태를 나타낸다.

kubectl을 사용하여 컨테이너가 Waiting 인 파드를 쿼리 하면, 컨테이너가 해당 상태에 있는 이유를 요약하는 Reason 필드도 표시된다.

kubectl을 사용해 Running 상태인 파드를 쿼리하면 해당 파드가 언제 실행되었는지 확인할 수 있다.

kubectl을 사용해 Terminated인 파드를 쿼리하면 종료 이유와 실행, 종료시간을 확인할 수 있다.