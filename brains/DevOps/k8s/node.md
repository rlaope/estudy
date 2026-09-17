# Node Components

이번에 더 자세히 알아보겠다.

노드 컴포넌트는 각 **노드에서 파드와 컨테이너를 구동시키고 관리하기 위해 필요한 요소들을 의미한다.**

이들은 워커 노드 뿐만 아니라 마스터 노드에도 존재한다.

클러스터 제어에 필요한 컨트롤 플래인 컴포넌트 역시 개별 파드로 구동되므로 이들을 관리할 수 있는 도구가 필요하다고 볼 수 있다.

### kubelet

**클러스터의 각 노드에서 파드 안에 컨테이너들이 정상 구동되도록 조율**하는 에이전트다.

마스터 노드의 스케줄러가 파드를 노드에 할당하면 **kubelet**이 최종적으로 해당 파드에 컨테이너를 배치하게 된다. 

또한 **파드와 컨테이너의 상태를 주기적으로 체크하여 그 결과를 API 서버에 전송하는 역할**도 맡는다.


> kubeadm으로 클러스터를 구축할 경우에 kubelet이 포함되어 있지 않기 때문에 별도 설치를 진행해야하며 kubeadm, kubectl와 상호 호환성 문제가 일어나지 않도록 셋의 버전을 일치시켜야한다.
> 
> [kubernetes.io: Installing kubeadm, kubelet and kubectl](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/install-kubeadm/?ref=seongjin.me#installing-kubeadm-kubelet-and-kubectl)


<br>

### kube-proxy

**클러스터의 각 노드에서 구동되는 쿠버네티스의 네트워크 프록시다.**

쿠버네티스에서 Service라 불리는 객체로 들어온 내/외부의 트래픽을 어느 파드로 포워딩 할 것인지에 대한 **규칙을 생성하고 관리**하는 역할을 한다.

이를 위해 `kubeadm`은 모든 노드에 하나식 `kube-proxy` 파드를 데몬셋으로 배포하며 이 정보는 `kubectl get daemonset -n kube-system` 명령을 통해 확인할 수 있다.

쿠버네티스의 파드는 반영속적이기 때문에 노드의 IP는 항상 바뀌게 된다. 그렇기에 이런 환경에서 파드들 간의 상호 네트워킹을 보장할 수 있는 방법인 서비스(Service)를 이용한다.

> 서비스는 파드들을 통해 실행되고 있는 애플리케이션을 네트워크에 노출시키는 가상의 컴포넌트다.

**서비스는 파드나 컨테이너처럼 구체적인 실체가 없고, 오직 파드들 간의 네트워크 중계만 수행한다.**

노드/파드들의 목록을 관리하며 필요한 쪽으로 트래픽을 전달해주는 프록시 역할의 구현체라곡 보면 된다.

이를 위해 서비스 역시 클러스터로부터 내부 IP를 할당 받으며, 파드들 간의 연결을 담당하는 게이트웨이 역할을 해준다.

위와같은 역할을 하는 서비스로 파드들이 접근 가능하도록 해주는 프로세스가 **kube-proxy**인 것이다.

**kube-proxy**의 상세 역할은 쿠버네티스 `1.20` 버전을 전후하여 많이 변화되었다. 이전까지는 **kube-proxy**가 직접 user space proxy 역할을 수행했다면, 최근에는 **iptables**를 통해 netfilter를 조작하고 관리하는 역할까지만 담당하고 있다.

![](https://velog.velcdn.com/images/bsj1209/post/5c4757e7-80ca-4b9d-a587-4c0c79202c4a/image.png)


### Case 1 - user space proxy 역할 (1.20 이전)

특정 클라이언트 프로세스가 `10.3.241.152:80`로의 액세스 요청을 파드의 `veth0`인 `10.0.2.3`으로 보냈다고 가정해보자.

1. kube-proxy가 localhost 인터페이스에서 서비스의 요청을 받아내기 위해 10400포트(임의)를 연다.
2. kube-proxy가 (물리 서버의 커널에 있는) netfilter로 하여금 서비스 IP(`10.3.241.152:80`)로 들어오는 패킷을 kube-proxy 자신에게 라우팅되도록 설정한다.
3. kube-proxy로 들어온 요청을 실제 server pod  `<IP:Port>`(예: `10.0.2.2:8080`)로 요청을 전달한다.

위와 같은 경우라면, 클러스터의 모든 파드들이 서비스의 IP를 요청하면서 매번 kube-proxy를 불러내게 된다. 때문에 호스트 쪽에 부하가 많이 걸리고, 그만큼 비용도 소모된다.


### Case 2 - iptables를 통해 netfilter를 조작만 하는 역할 (1.20 이후)

이 경우에는 kube-proxy가 직접 proxy 역할을 수행하지 않고 그 역할을 전부 netfilter에게 맡긴다.

서비스 IP를 탐색하여 파드에게 전달하는 일은 모두 netfilter가 담당하며, kube-proxy는 단순히 netfilter의 규칙을 알맞게 수정하는 역할만 수행한다.

쿠버네티스는 새로운 서비스를 만들거나, 파드가 생기거나, 레플리카셋이 바뀌는 등 변화가 일어났을 때 해당 리소스 컨트롤러가 이를 인지하게 된다.

이때 컨트롤러는 kube-proxy에게 이 정보를 알려주고, kube-proxy는 리눅스 도구인 iptables를 통하여 netfilter에게 트래픽 분배 규칙을 그때그때 새로 전달하는 방식으로 동작한다.

<br>

### 컨테이너 런타임 엔진 (container runtime engine)

**클러스터 내부에 컨테이너 이미지를 가져오고 구동시키는 엔진**

파드가 노드 안에서 동작하려면 반드시  필요한 부분이다.

쿠버네티스 1.23 버전 기준으로 리눅스 OS에서 지원되는 런타임은 다음과 같다.

- [containerd](https://containerd.io/?ref=seongjin.me)
- [CRI-O](https://cri-o.io/?ref=seongjin.me)
- Docker Engine (deprecated; `1.24`에서 삭제 예정)
- [Mirantis Container Runtime](https://www.mirantis.com/software/container-runtime/?ref=seongjin.me)

#### Docker Engine 지원 중단 (v1.24 이후)

본래 쿠버네티스에서는 컨테이너 생성과 실행을 위한 런타임 엔진으로 도커(Docker)를 지원해 왔으나, [지난 2020년 12월에 `1.20` 버전 이후로 지원을 중단(deprecated)한다고 발표했다.](https://kubernetes.io/ko/blog/2020/12/02/dont-panic-kubernetes-and-docker/?ref=seongjin.me) 도커를 이용한 런타임 지원은 2022년 2월 기준으로 [곧 발표될 `1.24` 버전부터 완전히 중단될 예정](https://github.com/kubernetes/enhancements/tree/master/keps/sig-node/2221-remove-dockershim?ref=seongjin.me)이며, 현재는 `containerd`와 같은 대체 수단이 기본적으로 이용되고 있다.

런타임 엔진에서 도커가 제외된다는 것이 쿠버네티스 클러스터에서 도커 자체를 사용하지 못하게 된다는 뜻은 전혀 아니다. 오직 쿠버네티스에서 지원되는 런타임 엔진에서만 제외되는 것으로, 도커에서 생성된 이미지는 [OCI(Open Container Initiative)](https://opencontainers.org/?ref=seongjin.me)를 따르는 이미지이므로 이후에도 `containerd`와 `CRI-O` 등을 통해 문제 없이 구동 가능하다는 것이 The Linux Foundation 측의 공식 입장이다.