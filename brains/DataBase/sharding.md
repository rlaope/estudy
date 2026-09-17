# DataBase Sharding(분산처리)

### DataBase Sharding이란
DB에 데이터가 늘어나면 용량 이슈와 함께 성능이 저하되고 DB 시스템 전체에 문제가 생길 가능성이 높아지게 된다. 이를 방지하기 위해 여러 DB 분산처리 기법이 있는데, 그 중 Sharding 기법에 대해 알아보려고 한다.  
  
Sharding은 같은 테이블 스키마를 가진 데이터들을 다수의 DB에 분산하여 저장하는 Horizontal Partitioning 방법으로 해당 테이블의 인덱스 크기를 줄이고, 작업 동시성을 늘리는 방법이다.  
  
다만 DB Sharding을 적용하면 프로그래밍 복잡도 및 운영 복잡도 높아지기 때문에 다른 분산처리 방법을 먼저 고려한 후 대규모의 빅데이터관리의 경우에만 사용하는 것이 좋다고 한다.

- 물리적 Scailing: DB 서버 및 Storage를 물리적으로 Scaling 한다.
- Cache & DB Replication 적용 : Read가 많은 시스템일 경우 Cached 및 DB Replication 방법을 적용한다.
- (DB Replication : DBMS를 Master/Slave 구조로 나누어 Master는 Insert Update Deleterl기능을 수행하고 Slave DB는 실제 데이터를 복사하여 Select를 수행한다.)
- Vertical Partioning : Table의 일부 컬럼만을 자주 사용할 경우
- Hot & Cold Data 분리 : 사진, 동영상, 메일 등 보관 기간은 길지만 자주 접근하여 사용하지 않는 Cold 데이터는 별도의 DB로 분리한다.

<br>

### 제약사항
DB Sharding 방법은 수평 분할의 구조적 특성상 아래와 같은 제약사항을 지니고 있으며, 이를 고려한 설계가 필요하다.

- Shard간 Cross-Join이 불가능하기 때문에 역정규화를 감수해야한다.
- 하나의 Transaction에서 두개 이상의 Shard에 접근할 수 없다
- 모든 Shard에 대해 유니크한 Shard Key 생성이 중요하다. DBMS에서 제공되는 자동 증분 키는 유효하지 않으며, DB 외부에서 Shard Key를 관리하기 위한 별도의 어플리케이션 운영이 필요하다(라우터 사용)
- 안전성을 위해 각 Shard의 Replication Set 구성이 필요하다.
- 서비스 정지 없이 Shard의 Scaling이 가능하도록 설계해야 한다.

<br>

### Sharding 전략 종류

#### Hash Sharding
Shard Key를 Hashing한 결과로 DB를 선택한다. Hash 방법으로 Modular 연산 등이 있으며, 데이터 형태에 따라 Hash 함수를 잘 설계하는 것이 필요하다.  
주로 데이터량이 일정 수준에서 유지될 것으로 예상될 때 적용한다.

- 장점
  - 데이터가 각 Shard에 균일하게 분산된다.
- 단점
  - DB를 추가 증설하는 과정에서 Hash 함수가 바뀌어야하므로 이미 적재된 데이터의 재정렬이 필요하다.

#### Dynamic (Range Based) Sharding
Local Service를 이용해 Shard Key를 특정 범위 기준으로 분할한다. 데이터의 트래픽에 따라 기준을 동적으로 변경할 수 있다.

- 장점
  - 증설 시 Shard Key만 추가하면 되므로 재정렬 비용이 들지 않는다.
- 단점
  - 일부 DB에 데이터가 몰릴 수 있다.
  - 데이터 분할 범위 기준을 명확하게 설정되어야 한다.
  - Locator의 의존성이 커져 Locator 장애시 Shard 전체에 영향을 줄 수 있다.
  - 데이터 재배치 시 Locator의 Shard Key Table도 일치시켜줘야 한다.
  - 성능을 위해 Cache 및 Replication 실행시 Location의 잘못된 라우팅으로 데이터를 찾지 못해 에러가 발생할 수 있다.


#### Entity Group
테이블이 Key-Value 관계가 아닌 다양한 객체로 구성되어 있는 경우, 일정 관계에 있는 Entity들을 같은 Shard 내에 구성한다.

- 장점
  - 같은 물리적인 Shard 내에서 쿼리 진행 시 효율적이다.
  - 사용자 증가에 따른 확장성이 좋다
  - 하나의 Shard 내에서 강한 응집도를 가진다.
- 단점
  - 특정 파티션 간 쿼리 요구시 비효율적일 수 있다.