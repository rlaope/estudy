# Kubernetes Network

### Goal
- Pod간 통신: CNI를 통한 네트워크 구성
- Service 유형: ClusterIP, NodePort, LoadBalancer의 차이점과 동작 원리
- kube-proxy: iptables와 IPVS 모드의 차이점과 선택 이유
- CoreDNS: K8s에서의 서비스 디스커버리

## Pod간 통신 - CNI를 통한 네트워크 구성

k8s에서 pod는 고유한 ip 주소를 할당받아, 클러스터 내의 다른 pod들과 직접 통신할 수 있다.

이 네트워크 구성을 담당하는 것이 바로 CNI(Container Network Interface) 플러그인이다.

**CNI**
- Pod에 고유한 IP 주소를 할당하는 역할
- pod의 네트워크 네임스페이스에 가상 이더넷 인터페이스 veth를 생성하고 이를 호스트 네트워크에 연결함
- 라우팅 설정: 클러스터 내의 다른 Pod들과 통신을 위한 라우팅을 설정
- 네트워크 정책 적용: 네트워크 접근 제어를 위한 정책 적용

Pod간 통신 방식은 동일 노드 내에서는 veth 페어를 통해 직접 통신을 하고 다른 노드 간에는 cni 플러그인이 설정한 오버레이 네트워크를 통해 통신한다. 예를들어 Flannel은 VXLAN을 사용해 오버레이 네트워크를 구성한다.

<br>
## ClusterIP, NodePort, LoadBalancer

K8s의 Service는 Pod들의 집합에 대한 네트워크 추상화로 안정적인 네트워크 접근을 제공한다.

주요 서비스 유형들에 대해서 알아보자.

### ClusterIP (Default)
- 클러스터 내부에서만 접근 가능한 가상 IP를 할당함
- 내부 서비스 간 통신에 사용된다.
- 동작 원리: kube-proxy가 iptables or IPVS를 사용해 ClusterIP로 들어오는 트래픽을 해당하는 pod로 라우팅한다.

### NodePort
- 각 노드의 특정 포트를 열어 외부에서 접근할 수 있도록 한다.
- 외부에서 클러스터 내부의 서비를 접근할때 사용한다.
- 동작 원리: NodePort를 통해 들어오는 트래픽은 내부적으로 ClusterIP에 전달되어 해당하는 Pod로 라우팅된다.

### LoanBalancer
- 클라우드 제공자의 로드 밸런서를 프로비저닝해서 외부에 서비스를 노출시킨다.
- 클라우드 환경에서 외부 트래픽을 처리할 때 사용된다.
- LoadBalancer는 내부적으로 NodePort, ClusterIP를 생성하며, 외부 로드 밸런서는 NodePort를 통해 트래픽을 전달한다.

<br>

## kube-proxy iptables, IPVS

kube-proxy는 각 노드에서 실행되며 Service와 관련된 네트워크 트래픽을 적절한 pod로 라우팅하는 역할을 한다.

### iptables
- iptables를 사용해 트래픽을 라우팅함. (target : destinition리스트들을 순회하면서 목적지를 찾는 방식)
- 설정이 간단하고 작은 규모 클러스터에서 적합하다.
- 많은 수의 서비스가 있을수록 성능 저하가 발생할 수 있다.

### IPVS
- Linux 커널의 IPVS를 사용해 트래픽을 라우팅한다. (해시 기반 알고리즘)
- 높은 성능과 확장성을 제공해 다양한 로드 밸런싱 알고리즘을 지원한다.
- 설정이 복잡할 수 있으며 커널 모듈이 필요하다.

> + Cilium eBPF 방식도 있다. (커널 레벨 코드를 작성해 동작을 구현하는 방식)

<br>
## CoreDNS: K8s에서의 서비스 발견 역할

CoreDNS는 K8s 클러스터 내에서 DNS 기반 서비스 발견을 제공하는 DNS 서버다.

- 서비스 이름 해석: `service.namespace.svc.cluster.local` 형식의 DNS 쿼리를 처리하여 해당하는 Service의 ClusterIP를 반환한다
- Pod이름 해석: Headless Service를 사용하는 경우 개별 Pod의 IP를 반환한다.
- 외부 도메인 포워딩: 클러스터 외부의 도메인 쿼리를 외부 DNS 서버로 포워딩한다.

**동작 원리**
- CoreDNS는 K8s API 서버와 통신해 Service와 Pod 정보를 수집한다.
- 수집한 정보를 바탕으로 DNS 쿼리에 대한 응답을 제공한다.
- 플러그인 기반 구조로, 다양한 기능을 추가할 수 있다.

<br>

### 요약

Pod간 통신은 CNI 플러그인을 통해 pod가 고유 ip를 할당받고 오버레이 네트워크를 구성하여 통신을 지원한다.

ClusterIP는 서비스 내부 통신, NodePort는 외부 접근, LoadBalancer는 클라우드 로드 밸런서를 통한 외부 노출에 사용된다.

kube-proxy 모드 iptables는 간단한 설정에 적합 ipvs는 높은 성능과 확장성을 제공함

CoreDNS는 DNS기반 서비스 디스커버리를 제공하며 클러스터 내의 서비스와 pod를 이름으로 접근할 수 있도록 한다.

