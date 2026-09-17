# xDS Protocol, Control Plane (Istio Pilot)

Istiod(Control Plane)가 수천 개의 envoy proxy(data plane)을 일사불란하게 움직이게 하는 핵심은 **xDS API**이다 envoy가 매번 재시작없이 실시간으로 설정을 변경할 수 있는것은 이 프로토콜 덕분

### xDS Protocol

xDS는 Discovery Service의 약자로(왜 ds로 안했는지 모르겠지만 일단 x 붙이면 간지나는 일론머스크인지 닌텐도 모델하고 겹쳐서인지 anyway) enovy가 외부 **istiod로부터 자신의 설정값을 동적으로 받아오기 위한 api 모음이다**

- **LDS (Listener Discovery Service)**: 어떤 포트를 열고 기다릴지, 리스너 L4 필터 체인 설정
- **RDS (Route Discovery Service)**: 요청을 어디로 보낼지, 가상 호스트, 경로 기반 라우팅 규칙
- **CDS (Cluster Discovery Service)**: 목적지 서비스 그룹은? 역할로는 서비스 명칭, 로드배런싱 정책, TLS 정책등
- **EDS (Endpoint Discovery Service)**: 실제 목적지의 ip 주소는 무엇인지, 부하 분산할 실제 파드의 ip 목록

LRCE는 저번에 envoy 에서 알아본 4대 구성요소 맞음

계층적인 이유는 EDS(Endpoint)는 파드가 생성/삭제될때마다 매우 빈번하게 변한다. 반면 LDS(port)는 변하지 않는다.(Service) 이를 분리하여 변경된 부분만 효율적으로 전파시키기 위함이다 Incremental xDS라고도 함. 개간지

### Eventual Consistency: 설정 전파의 시차

사용자가 `k apply -f virtualservice.yaml`을 실행했을 때, 모든 envoy에 반영되기까지는 짧은 시차가 존재한다.

1. Config Watch: Istiod가 쿠버네티스 api 서버에서 설정 변경 감지
2. Push: Istiod는 변경된 내용을 분석하고 관련 있는 모든 Envoy에게 gRPC 스트림을 통해 xDS업데이트를 보낸다.
3. Ack/Nack: Envoy는 설정을 적용하고 성공(Ack) or 실패(Nack)를 응답한다.

수천개의 노드가 있는 대규모 클러스터에서는 이 과정을 최종 일관성 eventually consistency 문제가 발생할 수 있으며, 설정이 전파되는 동안 구버전과 신버전 설정이 혼재하는 찰나의 순간을 이해해야한다.


### Sidecar Injection

파드로 들어오는 트래픽을 어떻게 가로채가는걸까? 애플리케이션 코드를 수행하지 않고도 envoy가 모든 패킷을 가로챌 수 있는 비결은 iptables 조작에 잇다.

- Mutation: 파드가 생성될때 istio webhook이 `istio-proxy` 컨테이너와 `istio-init` 컨테이너를 자동 주입한다.
- init-container: istio-init은 파드 네트워크 네임 스페이스 내에서 iptables 규칙을 설정한다.
- interception: 이 규칙에 의해, 컨테이너로 들어오고 나가는 모든 tcp 트래픽은 강제로 15001(아웃바운드), 15006(인바운드) 포트에서 대기중인 envoy로 리다이렉트 된다.

### 디버깅

설정을 넣었는데 동작하지 않는다면, Control Plane의 의도, Data Plane의 실제 상태가 일치하는지 확인해야한다.

상태 확인: `istioctl proxy status`
- `SYNCED`: Istiod와 Envoy의 설정이 일치
- `STABE`: Istiod가 보냈으나 Envoy가 아직 적용중이거나 응답 없음

실제 설정 덤프: `istioctl proxy-config <type> <pod-name>`
- `clusters`: Envoy가 알고있는 백엔드 목록 확인
- `routes`: 라우팅 룰이 의도대로 생성되었는지 확인
- `endpoints`: 실제 트래픽이 갈 수 있는 ip 목록 확인

