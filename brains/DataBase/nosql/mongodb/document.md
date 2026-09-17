# Document Database

Document Database는 다음과 같은 이점을 제공합니다.

### High Performance

몽고디비는 높은 성능의 데이터 지속성을 제공하고 있습니다.

- 내장(Embeded) 데이터 모델은 데이터베이스 시스템의 I/O 활동을 감소시킵니다.
- Index를 통한 빠른 쿼리와 keys를 내장 Document와 Array에 추가할 수 있습니다.

### Rich Query Language

몽고디비는 풍부한 쿼리 표현식을 제공하고 있습니다.


### High Availability

replica set이라고 불리는 몽고디비의 replication은 다음 기능을 제공하고 있습니다.

- automic failover(특정 노드에 결함이 발생하면 다른 노드가 작업을 대신 수행)
- data redundancy(replica set은 몽고디비의 서버 그룹으로 동일한 데이터 모음을 제공하여 데이터의 신뢰성과 높은 데이터 가용성을 제공합니다.)


### Horizontal Scaleablity

몽고디비는 수평적 확장을 제공하고 다음의 핵심 기능을 제공해요

- Sharding 머신들 간의 데이터 분산
- 몽고디비는 3.4부터 shard key 기반의 zones를 생성합니다. 균형있는 클러스터 내에서는 몽고디비는 읽기와 쓰기 작업을 해당 shard의 zone에 직접 접근하여 작업하도록 합니다.


### Support for Multiple Storage Engines
몽고디비는 다양한 저장엔진을 제공합니다.

- WiredTiger Storage Engine
- MMAPv1 Storage Engine

이외에도 다양한 외부 엔진이 추가할 수 있습니다.
