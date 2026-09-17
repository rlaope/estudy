# Scheduler, Controller

### goal
- Scheduler: Pod -> Node Mapping 원리
- Controller Flow: Deployment ~ Pod 생성까지의 흐름
- ReplicaSet의 Pod유지 및 self healing 메커니즘

## K8s Scheduler는 Pod를 어떤 기준으로 Node에 할당하는가

k8s Scheduler는 Pending 상태의 Pod를 적절한 Node에 바인딩하는 역할을 수행한다.

단순히 빈 노드에 배치를 하는것이 아니라 다양한 리소스 제약 조건, score 기반 우선순위를 고려해 스케줄링이 이루어진다.

### 스케쥴링 흐름

모든 Node중에 Pod를 수용할 수 잇는 후보 Node들을 먼저 걸러낸다.

filtering -> scoring 과정을 통해 우선순위를 정한다.

**1단계 필터링 - 대표 필터 조건**
- Nodes Status가 READY인지
- Node taint를 toleration으로 수용 가능한지
- Pod의 resource 요청(cpu/mem)을 수용할 만큼 Node에 여유가 있는지
- Affinity/Anity-Affinity 조건 충족 여부 확인
- volume, topology, zone 조건

**2단계 Scoring** (기본적으로 점수는 0  ~ 100)
- LeastRequestPriority: CPU/Memory 사용률이 가장 낮은 Node 선호
- BalancedResourceAllocation: 리소스간 불균형이 적은 Node 선호
- NodeAffinityPriority: affinity 조건을 만족하는 정도
- ImageLocalityPriority: 필요한 이미지가 이미 캐시된 Node 선호
최종적으로 가장 높은 점수를 받은 Node가 선택된다.

### 바인딩 과정
- Schuedler는 api 서버로부터 Pending 상태의 Pod를 watch한다.
- Filter + Scoring 이후 적절한 노드를 찾으면 PodSpec.nodeName 필드를 설정해 Pod를 해당 Node에 바인딩한다.


### Deployment Resource를 생성했을 때 파드가 생성되기 까지의 흐름
```
1. kubectl applyh -f deployment.yaml
2. API server -> etcd 저장
3. Deployment Controller 감지 (kube-controller-manager)
4. ReplicaSet 생성 -> Pod 생성 요청
6. Scheduler 감지 -> 노드 할당
7. kubelet -> 실제 파드 실행
```

## ReplicaSet pod self-healing

ReplicaSet은 항상 지정된 replicas 수를 유지하려고 한다.

```yaml
# ex deployment.yaml
...
spec:
  replicas: 3
```
-> pod가 2개만 살아있다면 1개를 새로 생성
-> 4개라면 하나를 종료

### Self-Healing 메커니즘
- ReplicaSet Controller는 API Server의 상태를 지속적으로 WATCH함.
- Pod가 삭제되거나 CrashLoopBackOff가 발생해 Ready가 아니게 되면 이를 감지하고 Pod를 재생성
- `label selector`를 기준으로 현재 살아있는 pod 수를 계산하며 조정


### 실시간 감지 방식
- kubec-controller-manager는 etcd의 상태를 주기적으로 poll하거나 이벤트 기반 WATCH를 사용해 ReplicaSet, Pod 상태 변화를 실시간 감지함
  
**실패 복구 시나리오 예시**
1. Node가 다운 -> 해당 Node의 Pod가 삭제됨
2. ReplicaSet은 이를 감지 -> 새로운 Pod를 다른 Node에 생성
3. kube-scheduler가 Node 할당 -> kubelet이 재실행

### 요약

Scheduler는 Pending Pod를 Node에 할당할때 Filtering, Scoring 순으로 우선순위 산출후 배치함

Deployment 생성 후 Pod 실행은 Deployment -> ReplicaSet -> Pod -> Scheduler -> Kubelet 실행 순임.

ReplicaSet의 동작 원리는 spec.replicas 수 유지 + Pod 상태 검사 -> Self-Healing


