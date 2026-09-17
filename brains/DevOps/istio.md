# Istio, Istio Architecture

Istio는 애플리케이션 네트워크 기능을 유연하고 쉽게 자동화할 수 있는 투명한 언어 독립적 방법을 제공하는 현대화된 서비스 네트워킹 레이어인 서비스 메시입니다.

클라우드 기반 애플리케이션을 구성하는 다양한 마이크로서비스를 관리하는 데 널리 사용되는 솔루션입니다.

Istio 서비스 메시는 또한 이러한 마이크로서비스가 서로 통신하고 데이터를 공유하는 방법을 지원합니다.

조직이 클라우드로의 이동을 가속화하면서 필요에 따라 애플리케이션도 현대화하고 있습니다.

그러나 모놀리식 레거시 앱을 클라우드 기반 앱으로 전환하면 데브옵스팀의 문제가 발생할 수 있습니다.

개발자는 클라우드에서 이동성을 보장하기 위해 느슨하게 결합된 마이크로서비스를 사용하여 앱을 어셈블하는 방법을 학습해야 합니다.

동시에 운영팀은 점점 더 커지는 하이브리드 및 멀티 클라우드 환경 내에서 새로운 클라우드 기반 앱을 관리해야 합니다.

Istio를 사용하면 이러한 작업을 수행할 수 있습니다.

<br>

## Istio
조직에서 분산형 마이크로서비스 기반 앱을 어디서나 실행할 수 있도록 지원하는 오픈소스 서비스 메시다.

- Istio를 도입하면 쿠버네티스의 복잡성을 줄일 수 있다.
- 마이크로서비스 아키텍처의 분산 네트워크 환경에서 각 app들의 네트워크 연결을 쉽게 설정할 수 있도록 지원하는 기술이다
- Istio는 Envoy를 Data Plane으로 사용하고 이를 control 해주는 오픈 소스 솔루션이다.

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2Fc747d893-511e-4261-9b0c-8eb0a4795267%2Fimage.png)

### Istio 서비스의 이점
- 일관된 서비스 네트워킹 달성
- Istio 이점을 통한 서비스 보호
- 애플리케이션 성능 향상

> Kubernetes 환경의 네트워크(service mesh) 관리에서 가장 많이 사용되는 오픈 소스가 istio다.

## Istio Architecture

모든 컴포넌트가 istio라는 하나의 프로세스로 합쳐지고 Mixer가 없어지고 Pliot이 Mixer의 기능까지도 함께 수행하는 것으로 되어있다.

> istiod: Istio의 컴포넌트 중 하나인 Pilot, Mixer가 통합된 프로세스  
> Pilot: Istio의 구성 요소 중 하나로, 서비스 디스커버리 및 로드 밸런싱을 담당  
> Mixer: Istio의 구성 요소 중 하나로, 통신 패턴 분석, 정책 적용, 텔레메트리 수집 등의 기능을 제공

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2F32f0102e-e439-455b-bb78-eb7dd3196534%2Fimage.png)


<br>

## Istio에서 service mesh를 구현한 방식

proxy 방식의 container를 추가하여 네트워크를 구성하였다.

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2F7e634fe8-2879-47f6-ad3b-4021fd5b578a%2Fimage.png)

Istio 내부 구성 방식

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2Fccb28f59-4c61-4c61-8395-1b45a5870bb5%2Fimage.png)

-> 모든 app의 로그를 쉽게 수집/시각화 가능

<br>

## Istio의 내부 핵심 구성

Istio는 Data Plane과 Control Plane으로 구성되어있다.

![](https://velog.velcdn.com/images%2Fberyl%2Fpost%2Fefe5dd49-05a9-4a92-bf94-b2052ece325e%2Fimage.png)

### Data Plane
실제로 트래픽을 받아 처리해주는 파트.

Service의 Sidecar 형태로 구성된 프록시들을 가리킨다.

Data Plane은 Control Plane에 의해 통제된다.

Istio에서는 envoy를 sidecar proxy로 사용한다.

> Sidecar: 주요 애플리케이션 또는 서비스와 함께 작동하는 보조 애플리케이션 (로깅, 모니터링, 인증)  
> Envoy: 클라이언트와 서버간의 통신을 중개하고 보안성을 강화하는 역할을 한다.

### Envoy
경량화된 L7 전용 프록시다.

HTTP, TCP 등의 프로토콜을 지원한다.

Circuit Breaker, Retry, Timeout 등의 기능을 지원한다.

> Circuit Breaker: 분산 시스템에서 장애로 인한 서비스 장애를 방지하기 위한 패턴  
> Retry: 분산 시스템에서 장애 상황에서 서비스 호출을 다시 시도하는 패턴  
> Timeout: 분산 시스템에서 서비스 호출이 일정 시간 내에 응답하지 않을 때 호출을 취소하는 패턴

## Control Plane
Data Plane을 컨트롤하는 구성 요소를 가리킨다.

Pilot, Mixer, Citadel, Galley 등으로 구성되어 있다.

### Pilot
- envoy에 대한 설정 관리를 하는 역할을 한다.
- Traffic Management 기능을 제공한다.
  - Service Discovery (Envoy Endpoint를 알아내는 작업)
    - Traffic Retry
    - Circuit Breaker
    - Timeout

### Mixer
Service Mesh 전체 영역에서 Access 제어 및 정책을 관리한다.

모니터링 지표를 수집한다.

### Citadel
보안에 관련된 기능을 담당하는 모듈이다.

인증 기능 관리(TLS Certification 등)

### Galley
Istio configuration을 체크한다.

Kubernetes Yaml 파일을 istio가 이해할 수 있는 형태로 변환한다.


