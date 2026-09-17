# K8s RBAC, Security Policy

### Goal
- RBAC: Role vs Cluster Role
- Pod Security와 네트워크 정책의 계층적 보안 설계
- `hostNetwork: true` 설정의 보안 리스크

## RBAC Role vs ClusterRole

RBAC은 Role Based Access Controld의 약자로 **사용자나 서비스 계정에 필요한 리소스에 최소 권한을 부여하는 k8s 권한 관리 시스템이다.**

다음 4가지 리소스로 구성된다
1. Role: 네임스페이스 범위의 집합
2. ClusterRole: 클러스터 전역 권한
3. RoleBinding: Role을 사용자 서비스 계정 연결 (네임스페이스 한정)
4. ClusterRoleBinding: ClusterRole을 사용자에 연결 (전역 또는 네임스페이스 지정 없이 사용 가능)

- Role은 **네임스페이스 한정**에 같은 네임스페이스 내 리소스가 적용되며 RoleBinding으로 바인딩
- ClusterRole은 **클러스터 전역 또는 네임스페이스 내** 모든 네임스패이스 or 노드/네트워크 등 클러스터 자원에 포함시킴, RoleBinding, ClusterRoleBinding으로 바인딩

### Example

```yaml
apiVersion: rback.authorization.k8s.io/v1
kind: Role
metadata:
	namespace: dev 
	name: pod-reader
rules:
	- apiGroups: [""]
	  resources: ["pods"]
	  verb: ["get", "list"]
```

```yaml
apiVersion: rback.authorization.k8s.io/v1
kind: ClusterRole
metadata:
	name: node-reader
rules:
	- apiGroups: [""]
	  resources: ["nodes"]
	  verbs: ["get", "list"]
```
> ClusterRole은 ClusterRoleBinding으로 클러스터 범위 자원 접근 허용시 필수다.

<br>

## Pod 보안정책(PSS), Network 정책 시큐리티 계층 분리

### PSS(PodSecurity Standard)

K8s 1.25+ 부터 PSP(PodSecurityPolicy) 폐지 후 PodSecurity Admission Controller를 통해 3단계 보안 정책을 적용한다.
- `privileged`: 제한 없음 - 운영 툴, cni 등
- `baseline`: 일반 워크로드 허용 - 대부분 앱
- `restricted`: 가장 엄격함 (root 없음 등) - 보안 민감 앱

> Pod의 `runAsNonRoot, allowPrivilegeEscalation, hostPath` mount등을 제한함

### NetworkPolicy
- Ingress/Egress 제어: pod간 통신 외부 통신 허용/차단 설정
- Label selector 기반: 어떤 파드 그룹간 통신 허용 여부 정의
- cidr/port 제한: 외부와의 통신 범위 지정 가능

### 계층적 분리 방법

- PSS(PodSecurity): 실행 권한 제어 (루트 권한, 호스트 마운트 금지) ex, runAsUser, capabilities 제한
- RBAC: API 리소스 접근 제어, ex. ConfigMap 수정 권한 제어
- NetworkPolicy: 네트워크 접근 제어 (Pod간 외부통신제어) ex. DB Pod는 App Pod에서만 접근가능 같은

<br>

### `hostNetwork: true` 보안 리스크

위 옵션은 pod가 호스트 네트워크 네임스페이스를 공유해 node의 ip와 동일한 네트워크 인터페이스를 사용한다.

### 보안 리스크
- Node-level port 충돌: host에서 이미 사용중인 포트와의 충돌 위험
- 네트워크 격리 무효화: pod간 트래픽이 동일 네트워크로 흘러 네트워크 정책이 무력화 가능함
- 호스트 리소스 접근: /proc, localhost, dns등과 같은 호스트 접근 가능성이 있음
- 감염 확산: 하나의 파드가 침해되면, node 전체가 위협받을 수 있음

hostNetwork 옵션을 활성화 하는 사례같은건 CNI, CoreDNS와 같은 인프라 레벨의 파드등에만 주로 사용함
- 예시: node-exporter, fluentbit, istio-cni 등등

<br>

### 요약

- Role은 namespace 한정, ClusterRole은 크러스터 전역 접근 허용
- PSS vs NetworkPolicy: PSS는 파드 실행 제약, NetworkPolicy는 네트워크 제어
- hostNetwork 위험: 포트 충돌, 네트워크 정책 우회, 보안 경계 무너질 위험 있음
