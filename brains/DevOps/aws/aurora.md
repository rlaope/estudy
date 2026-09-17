# Amazon Aurora, Aurora 아키텍처

## Aurora

고성능 상용 데이터베이스의 성능과 가용성에 오픈 소스 데이터베이스의 간편성과 비용 효율성을 결합하였으며 클라우드를 위해 구축된 MySQL 및 PostgreSQL 호환 관계형 데이터베이스다.

표준 MySQL 데이터베이스보다 최대 5배 빠르고 표준 PostgreSQL 데이터베이스보다 3배 빠르다.

1/10 비용으로 상용 데이터베이스의 보안, 가용성 및 안정성을 제공한다.

하드웨어 프로비저닝, 데이터베이스 설정, 패치 및 백업과 같은 시간 소모적인 관리 작업을 자동화하는 Amazon Relational Database Service (RDS)에서 Amazon Aurora의 모든 것을 관맇나다.

<br>

## Aurora 아키텍처

### Single-Master

![](https://user-images.githubusercontent.com/28394879/141725880-66b8285c-0f21-45e1-a751-20ab4f5d92b7.png)

### Multi-Master

![](https://user-images.githubusercontent.com/28394879/141935073-90caf347-f14e-4603-a77b-e02e83b1d49d.png)

<br>

## Aurora의 특징

MySQL/PostgreSQL 지원

두 가지 모드
- 다수의 노드로 읽기 쓰기가 가능한 Multi-Master
- 한 개의 쓰기 전용 노드와 다수의 읽기 전용 노드 (Aurora Replicas) 구성의 Single-Master

용량의 자동 증감: 10G 부터 시작하여 10GB 단위로 용량 증가 (최대 128TB)

연산 능력: 96vCPU와 768GB 까지 증가 가능(db.r5.24.xlarge)

데이터의 분산 저장: 각 AZ마다 2개의 데이터 복제본 저장 x 최소 3개이상의 AZ = 최소 6개의 복제본
- 3개 이상을 잃어버리기 전엔 쓰기 능력 유지
- 4개 이상을 잃어버리기 전에는 읽기 능력 유지
- 손실된 복제본은 자가 치유: 지속적으로 손실된 부분을 검사 후 복구
- Quorum 모델 사용

<br>

## Sigle-Master 모드

한 대의 Writer 인스턴스와 다수의 읽기 전용 인스턴스로 구성

총 15개의 Replica 생성 가능

Async 복제

하나의 리전안에 생성 가능

Writer가 다운될 경우 자동으로 Replica중 하나가 Writer로 Failover -> 데이터 손실 없이 Failover시 메인으로 승격 가능

고가용성을 확보

<br>

## Aurora Global Database

전 세계의 모든 리전에서 1초내의 지연시간으로 데이터 엑세스 가능

재해복구용도로 활용 가능
- 유사시 보조 리전중 하나를 승격으로 메인으로 활용
- 1초의 RPO(복구 목표 지점)
- 1분 미만의 RTO(복구 목표 시간)

보조 리전에는 총 16개의 Read 전용 노드 생성 가능 (원래는 15개)

![](https://user-images.githubusercontent.com/28394879/141939570-65e285fb-8ffc-447c-afc3-9a29d3c00a20.png)

<br>

## 병렬 쿼리

다수의 읽기 노드를 통해 쿼리를 병렬로 처리하는 모드
- 빠름
- 부하 분산(CPU, Memory)

MySQL 5.6/5.7에서만 지원한다.

남은 인스턴스 (db.t2, db.t3등) 에서는 지원하지 않는다.

<br>

## Aurora의 백업

읽기 복제본(Read Replica) 지원 (Aurora Replica와 다른 개념)
- MySQL DB의 Binary log 복제
- 단 다른 리전에서만 생성 가능

RDS와 마찬가지로 자동/수동 백업 가능
- 자동 백업의 경우 1~35일 동안 보관(S3에 보관)
- 수동 백업(스냅샷) 가능
- 백업 데이터를 복원할 경우 다른 데이터베이스를 생성

<br>

## Aurora 데이터베이스 클로닝

기존의 데이터베이스에서 새로운 데이터베이스를 복제
- 스냅샷을 통해 새로운 데이터베이스를 생성하는 것 보다 빠르고 저렴함

Copy-On-Write 프로토콜 사용
- 기존 클러스터를 삭제해도 제대로 동작

<br>

## Backtrack

기존의 DB를 특정 시점으로 되돌리는 것(새로운 DB가 아닌 기존 DB)

- DB관리의 실수를 쉽게 만회 가능(예: Where 없는 Delete)
- 새로운 DB를 생성하는 것 보다 훨씬 빠름
- 앞 뒤로 시점을 이동할 수 있기 때문에 원하는 지점을 빠르게 찾을 수 있음

Backtrack Window
- Target Backtrack Window
  - 어느 시점 만큼 DB를 되돌리기 위한 데이터를 저장할 것인지
    - 지정 시점 이전으로는 Backtrack 불가능
- Actual Backtrack Window
  - 실제 시간을 얼만큼 되돌릴 것인지
  - Target Backtrack Window보다 작아야한다.

Backtrack 활성화시 시간당 DB의 변화를 저장
- 저장된 용량만큼 비용 지불
- DB 변화가 많을수록 많은 로그 = 많은 비용
- DB 로그가 너무 많아 Actual Backtrack Window가 Target Backtrack Window 설정값 보다 작을 경우, 알림을 준다.

MySQL만 가능

Aurora 생성시 Backtrack을 설정한 DB만 Backtrack 가능
- 스냅샷을 복구하거나 Clone을 통해 기능 활성화 가능

Multi-Master 상태에서는 Backtrack 불가능

<br>

## Multi-Master

최대 4개의 노드가 읽기 쓰기를 담당
- 각 노드는 독립적: 정지/재부팅/삭제 등에 서로 영향을 받지 않음

지속적인 가용성을 제공

주로 Multitenant 혹은 Sharding이 적용된 애플리케이션에 좋은 성능