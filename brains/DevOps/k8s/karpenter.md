# Cluster Autoscaler, Karpenter

Clsuter Autoscaler는 [autoscaler](https://github.com/kubernetes/autoscaler)의 공식 서브 오픈소스 프로젝트다.

클러스터의 워커노드를 유동적으로 스케일링 해주는 컴포넌트다.

워커노드의 자원이 더 필요한 경우에 새로운 클러스터를 생성하거나, 필요하지 않다고 판단되면 클러스터를 삭제하는 등에 작업을 처리한다.

이 동작은 Cloud Provider의 요청에 의해 수행된다.

### Provisioning

AWS에서는 이 동작이 ASG(Auto Scale Group)을 통해서 이루어진다. CA는 Pod의 상태를 계속 관찰 하다가 지속해서 할당에 실패하게 된다면, Node Group의 ASG Desired Capacity 값을 수정하여 워커 노드의 개수를 증가하도록 설정한다.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*xlvrYe_cv7ZXcscCvP5XEw.png)

1. 자원 부족으로 Pending 상태의 파드가 존재한다.
2. CA는 ASG의 Desired 수를 증가시킨다.
3. AWS ASG는 새로운 노드를 프로비저닝한다.
4. kube-scheduler는 Pending된 파드를 새로운 노드에 할당한다.

<br>

### Deprovisioning

디프로비저닝의 경우에는 노드의 할당 가능한 리소스를 기준으로 사용률이 50퍼센트 이하인 경우 디프로비저닝 대상으로 간주한다.

해당 노드에서 실행중인 파드가 다른 노드로 옮겨갈 수 있는지 계산하여 노드의 삭제를 진행한 뒤 ASG의 Desired 수를 변경하게 된다.

<br>

**Cluster Autoscaler는 위와 같이 ASG의 Desired 사이즈를 조정하여 노드를 프로비저닝하므로, 반응 속도가 느린 문제가 있다.**

그리고 이러한 문제를 해결하기 위해서 우리는 Karpenter를 사용할 수 있다.

<br>

### Karpenter

Karpenter는 AWS가 개발한 쿠버네티스의 워커 노드 자동 확장 기능을 수행해주는 오픈소스 프로젝트다.

앞서 말한 CA와 비슷한 역할을 수행하지만 AWS 리소스의 의존없이 JIT(Just In Time)배포가 가능하다.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*h_Rnwm0cJ--zeujr9aNQ_g.png)

위를 보면 CA와 다르게 ASG와 관계 없이 프로비저닝 되는 것을 볼 수 있다.

파드를 할당할 수 있는 노드가 부족할 때 JIT으로 노드 프로비저닝이 일어나기 때문에 보다 빠르게 파드를 할당할 수 있다.

프로비저닝도 프로비저너의 특수한 조건에 만족해야 노드들이 프로비저닝되는데 그 조건들을 알아보겠다.

1. 리소스 요청: 파드에 띄워져있는 리소스가 현재 있는 노드보다 많이 필요한 경우, 프로비저너가 동작하게 지정한다.
```yaml
spec:  
	containers:  
		- name: dotori-service
		image: k8s.gcr.io/pause  
		resources:  
			requests:  
				cpu: 2000m  
				memory: 2.5Gi  
			limits:  
				cpu: 2000m  
				memory: 2.5Gi
```

위에서 cpu 2core와 2.5Gi 메모리량이 필요하기 때문에 xlarge의 타입의 노드가 프로비저닝된다.

2. 노드 선택: nodeSelector에서 원하는 노드를 선택하는 조건을 지정하여 프로비저너가 동작하도록 합니다.

```yaml
spec:  
. template:  
	metadata:  
		labels:  
			app: nginx  
 .spec:  
	NodeSelector:  
		topology.kubernetes.io/zone: ap-northeast-2a  
		karpenter.sh/capacity-type: spot
```

위의 예시는 지정된 레이블과 맞는 온디멘드나 혹은 스팟 provisioner가 동작해서 새로운 노드가 뜨거나 기존의 노드중 레이블에 매칭된 노드에 파드가 할당됩니다.

이 외에도 더 자세한 사항은 [이 글](https://medium.com/uplusdevu/karpenter%EC%99%80-empty-pod%EC%9D%84-%ED%99%9C%EC%9A%A9%ED%95%9C-%EC%8A%A4%EC%BC%80%EC%9D%BC%EB%A7%81-1-775737f265b3)을 참고해주세요

<br>

### 설정 방법

노드가 꽉 찼을 때 어떤 타입의 노드를 생성할 지 프로비저너로 지정한다.

Provisoner에 왼쪽과 같이 설정해주었기 때문에, Karpenter가 생성한 node에는 오른쪽과 같은 taint가 붙어있다.

```yaml
apiVersion: karpenter.sh/v1alpha5 
kind: Provisioner 
... 
spec: 
	taints: 
		- key: "dotori/server" 
		- value: "true" 
		- effect: "PreferNoSchedule"
```

```bash
$ kubectl get nodes -o json | jq '.items[].spec.taints' 
[ 
	{ 
		"effect": "PreferNoSchedule",
		"key": "dotori/server",
		"value": "true" 
	} 
]
```

Karpenter가 생성한 node 위에 스케줄링할 pod에 대해서 nodeSeletor와 [tolerations](https://karpenter.sh/docs/concepts/scheduling/#taints-and-tolerations)를 설정해준다.

```yaml
# Karpenter=enabled 라벨이 있는 node에만 스케줄링
nodeSelector:
  Karpenter: enabled
# Karpenter taint를 무시하도록 tolerations 설정
tolerations:
  - effect: "NoSchedule" 
    key: karpenter
    operator: "Equal"
    value: "true"
```


