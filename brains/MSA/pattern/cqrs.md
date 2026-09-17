# CQRS

Event Sourcing과 함께 알아두어야 할 개념으로 CQRS가 있다.

간단히 설명하면 Command(Create, Update, Delete)와 Query(Read)의 책임을 분리하는 것이다.

Event Sourcing과 연계시켜 생각해보면 Event는 Command 즉 상태가 변경되는 경우에만 발생하고 그 경우만 기록하면 된다.

단순 Query인 경우는 모델의 정보를 변화시키지 않기 때문에 그 기능을 분리하여 좀 더 손쉽게 Event Sourcing Pattern을 적용할 수 있다.

또한 Event Sourcing 개념에 의해 생성된 최신 데이터(snapshot)은 CQRS에서 Query의 개념으로 접근하여 조회하기 때문에 로직을 간략화하고 cache를 통해 조회속도를 손쉽게 개선할 수 있다.

![](https://mblogthumb-phinf.pstatic.net/MjAxOTAzMTVfMTgy/MDAxNTUyNjEyOTMzNjc1.LBvUFP4BzxGrTn7XOBrDSiWcRX6kheVsdhXkDO-mjGQg.cucZTWqlgezgeEVTC65-7BtuCvNbOSdgSOhICh3lG7Ig.PNG.rogman0/04.png?type=w800)

### 장점
- 마이크로 서비스에서 데이터 처리 이점이 있다. 특정 api를 통해서만 데이터를 변경할 수 있고, 조회 api를 이용해서 데이터를 읽을 수 있도록 분리한다.
- 읽기/쓰기 요청을 분리함으로 해서 성능을 개선할 수 있다.
- 데이터 이상을 동시에 쓰는 방식보다 더 쉽게 찾을 수 있다.
- 이벤트 소싱과 함께 사용되어 비동기 처리로 작업을 수행할 수 있다.