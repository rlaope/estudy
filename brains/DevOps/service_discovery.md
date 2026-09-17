# MSA에서의 Service discovery 패턴

## MSA Service discovery 패턴
MSA와 같은 분산 환경은 서비스간의 원격 호출로 구성이 된다.
  
원격 서비스 호출은 IP 주소와 포트를 이용하는 방식이 된다.
  
클라우드 환경이 되면서 서비스가 오토 스케일링등에 의해서 동적으로 생성되거나 컨테이너 기반의 배포로 인해서, 서비스 IP가 동적으로 변경되는 일이 잦아졌다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99AD92335AD610DE16)

그래서 서비스 클라이언트가 서비스를 호출할 때 서비스의 위치(즉 IP, Port)를 알아낼 수 있는 기능이 필요한데, 이것을 바로 `서비스 디스커버리(Service discovery)` 라고한다.
  
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99E912455AD610DE09)

다음 그림을 보자 Service A의 인스턴스들이 생성이 될때, Service A에 대한 주소를 Service registry(서비스 등록 서버)에 등록해놓는다.
  
Service A를 호출하고자 하는 클라이언트는 Service registry에 Service A의 주소를 물어보고 등록된 주소를 받아서 그 주소로 서비스를 호출한다.

## Clinet discovery vs Server side discovery
이러한 Service discovery 기능을 구현하는 방법으로는 크게 client disvovery 방식과 server side discovery 방식이 잇다.
  
앞에서 설명한 server client가 service registry에서 서비스의 위치를 찾아서 호출하는 방식을 client side discovery라고 한다.
  
다른 접근 방법으로는 호출이 되는 서비스 앞에 일종의 proxy 서버(로드 밸런서)를 넣는 방식인데, 서비스 클라이언트는 이 로드밸런서를 호출하면 로드밸런서가 service registry로 부터 등록된 서비스의 위치를 리턴하고, 이를 기반으로 라우팅하는 방식이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99AF813E5AD610DE03)

가장 흔한 예제로는 클라우드에서 사용하는 로드밸런서를 생각하면 된다. AWS의 ELB나 구글 클라우드의 로드 밸런서가 대표적인 Server side discovery 방식에 해당한다.

## Service registry
그러면 서비스를 등록하는 Service registry는 어떻게 구현을 해야할까?  
  
가장 쉬운 방법으로는 DNS 레코드에 하나의 호스트명에 여러개의 IP를 등록하는 방식으로 구현이 가능하다.
  
그러나 DNS는 레코드 삭제시 업데이트되는 시간등이 소요되기 때문에, 그다지 적절한 방법은 아니기 때문에, 솔루션을 사용하는 방법이 있는데, ZooKeeper나 etcd와 같은 서비스를 이요할 수 있고 또는 Service discovery에 전문화된 솔루션으로는 Netfilx의 Eureka, Hashcorp의 Consul와 같은 서비스가 있다.

## 향상된 기능
Service discovery 기능은 기본적으로 서비스를 등록하고 등록된 서비스의 목록을 리턴하는 기능이지만, 지능화된 기능을 이용하여 조금 더 향상된 기능을 제공할 수 있다.
  
예를 들어 Service registry에 등록된 서비스들의 Health check를 통해 현재 서비스가 가능한 서비스를 판별한 후, 서비스가 가능한 서비스 목록만을 리턴을 한다던가.
  
서비스간의 부하 분산 비율을 조정하는 고급 기능을 추가할 수 있고, 서버 목록에서 Master/Slave 서버의 정보를 리턴한다던가. 또는 서버에 접속하기 위한 인증키 정보등을 리턴하는 기능등 다양한 기능으로 확장이 가능하다.