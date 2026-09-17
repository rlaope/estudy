# Event Sourcing

애플리케이션의 모든 상태 변화를 순서대로 이벤트로 보관하여 처리하는 개념이다.

이렇게 모든 상태를 이벤트의 흐름으로 처리함으로써 애플리케이션의 개발을 간소화하고 분산 환경에 적절하게 대응할 수 있다.

모든 이벤트는 시간 순으로 기록이 되며 각 서비스들은 자신이 필요한 이벤트만 보고 있다가 최신의 상태를 유지하면 된다.

즉, 회원관리 서비스는 회원의 생성 삭제, 또는 변경 이벤트만 참조하여 회원의 최신정보(snapshot)을 가지고 최신 정보를 유지할 수 있으며, 상품 관련 서비스는 상품의 주문, 배송 등과 관련된 이벤트만 참조(Listening)하고 최신 상태를 유지한다.

이러한 EventSourcing이 완벽하게 동작하려면 모든 이벤트는 기록되어야 하고, 이벤트를 받지 못하는 서비스가 없도록 재전송이 가능해야한다.

또한 이벤트는 자신의 서비스 또는 WAS 내에서만 받을 수 있는 것이 아니라 브로드캐스팅이 되어야 분산된 환경에서도 완벽하게 동작할 수 있어야 한다.

Event Sourcing의 이러한 특징 때문에 MSA 시스템을 적용할 때 Event 기반으로 구현하면 보다 손쉽게 구현이 가능하며, 각 서비스 간의 loosed coupling이 가능하다.

![](https://mblogthumb-phinf.pstatic.net/MjAxOTAzMTVfMTkw/MDAxNTUyNjEyOTE4NzI2.V9ICsBgEppRERJBNeQx_BJsX4_H2cOZxqxzI3MsQPAkg.Etp3aIy-zoip4VI4UO-AN3D08KN4cPsdeLJQTSWBHEkg.PNG.rogman0/03.png?type=w800)

이벤트 소싱은 위와 같이 데이터를 변경하는 작업이 있을 때 데이터가 있는 서비스에 요청하는 방식은 동일하다.

그러나 한 가지 다른 점이 있다면 **데이터를 가진 서비스가 바로 데이터를 조작하지 않는다.**

위 그림을 보면 몇가지 요소가 있다.
- Event Producer
- Event Consumer
- Event (Command)
- Event Store (Queue)
- Event Snapshot(빨간색 원)

대신에 위와 같은 이벤트를 발생시키고 Event Store에 저장한다.  

Kafka와 같은 메시지 브로커들이 Event Store가 될 수 있다. 그러면 이벤트 컨슈머가 해당 이벤트를 데이터베이스나 HDFS에 들어온 순서대로 적재를 한다.  

그리고 필요한 데이터를 변경하거나 반영하게 된다.