# AWS RDS (Relation Database Service)

## RDS란?
Amazon Relational Database Service(RDS)를 사용하면 클라우드에서 관계형 데이터베이스를 간편하게 설정, 운영 및 확장할 수 있다.
  
하드웨어 프로비저닝, 데이터베이스 설정, 패치 및 백업과 같은 시간 소모적인 관리 작업을 자동화하면서 비용 효율적이고 크기 조정 가능한 용량을 제공한다.
  
사용자가 애플리케이션에 집중하여 애플리케이션에 필요한 빠른 성능, 고가용성, 보안 및 호환성을 제공할 수 있도록 지원한다.

![](https://user-images.githubusercontent.com/28394879/141259851-a09fe71d-97fd-4716-92de-9154f23cf694.png)

## RDS의 특징

### 관계형 데이터베이스를 제공하는 서비스
Relational Database Service: 관계형 데이터베이스
  
<--> NoSQL(DynamoDB, DocumentDB, ElasticCache)

### 가상 머신 위에서 동작
단 직접 시스템에 직접 로그인 불가능 -> OS 패치, 관리 등은 AWS의 역할

### RDS는 Serverless 서비스가 아님
단 Aurora Serverless는 말그대로 Serverless 서비스

### CloudWatch와 연동
DB 인스턴스의 모니터링(EC2와 동일)  
  
DB에서 발생하는 여러 로그(Error Log, General Log등)을 CloudWatch와 연동하여 확인 가능

### 내부에서는 EC2를 활용
VPC안에서 동작
- 기본적으로 public IP를 부여하지 않아 외부에서 접근 불가능
- 설정에 따라 public으로 오픈 가능 (DNS로 접근)

서브넷과 보안 그룹 지정 필요
  
EC2 타입의 지정 필요
  
스토리지는 EBS를 활용
- EBS 타입의 선택 필요
- 생성시 EBS의 용량을 지정해서 생성

Parameter Group: Root 유저만 설정 가능한 DB의 설정값들을 모아 그룹화한 개념
- DB 클러스터에 파라미터 그룹을 적용시켜 설정값을 적용

업데이트
- 마이너 버전 엔진 업데이트는 자동으로 업데이트 설정 가능
- 기타 업데이트의 경우 점검 시간(Maintenance Window)를 설정하여 특정 시간에 업데이트가 이루어질 수 있도록 설정 가능

## RDS의 인증 방법

### 전통적인 유저/패스워드 방식

AWS Secret Manager와 연동하여 자동 로테이션 가능

### IAM DB 인증
데이터베이스를 IAM 유저 Credential/Role을 통해 관리 가능

### Keberos 인증

### RDS의 가격 모델

컴퓨팅 파워 + 스토리지 용량 + 백업 용량 + 네트워크 비용
  
Reserved Instance 구매 가능
- EC2와 마찬가지로 일정 기간을 게약하여 저렴한 가격에 서비스를 이용

## RDS에서 제공하는 DB 엔진
- MS SQL SERVER
- Oracle
  - Oracle OLAP
- MySQL Server
- PostgreSQL
- MariaDB
- Amazon Aurora
- MS SQL Server, Oracle, Oracle OLAP는 오픈소스가 아니기 때문에 라이선스 비용 추가(자신의 라이선스 사용 가능)

## RDS의 암호화
- 암호화 지원
  - SQL 서버 혹은 Oracle에서는 TDE(Transparent Data Encryption) 지원
  - 모든 엔진에서 EBS 볼륨 암호화 지원
    - Default Master Key 혹은 생성한 Master Key 선택 가능
  - 자동 백업, 스냅샷, Read Replica 등에 적용됨

## RDS의 백업

### 자동 백업
매일마다 스냅샷을 만들고 트랜잭션 로그를 저장
  
데이터는 S3에 저장되며 데이터베이스의 크기 만큼의 공간 점유
  
저장된 데이터를 바탕으로 일정 기간 내에 특정 시간으로 롤백 가능 (다른 DB 클러스터를 새로 생성)
  
1~35일 까지 보관 지원
  
Backup을 시행할 때는 약간의 딜레이 발생 가능성
  
기본적으로 사용 상태로 설정되어 있음

### 수동 백업(DB 스냅샷)
유저, 혹은 다른 프로세스로 부터 요청에 따라 만들어지는 스냅샷
  
데이터베이스가 삭제된 이후에도 계속 보관
  
스냅샷의 복구는 항상 새로운 DB Instance를 생성하여 수행



## RDS Multi AZ
두 개 이상의 AZ에 걸쳐 데이터베이스를 구축하고 원본가 다른 DB(Standby)를 자동으로 동기화(Sync)  
- SQL Server, Oracle, MySQL, PosgreSQL, MariaDB에서 지원
- Aurora의 경우 다중 AZ를 설계 단계에서 지원

원본 DB의 장애 발생 시 자동으로 다른 DB가 원본으로 승격됨(DNS가 Standby DB로)
  
StandBy DB는 접근 불가능
  
퍼포먼스의 상승 효과가 아닌 안정성을 위한 서비스

![](https://user-images.githubusercontent.com/28394879/141416100-f7134381-f45f-4a7e-bff1-5086adf3d94a.png)

![](https://user-images.githubusercontent.com/28394879/141416236-cf001439-dbfc-4645-8585-fd05c4d9ce81.png)

## Read Replica(읽기 전용 복제본)

원래 데이터베이스의 읽기 전용 복제본을 생성(Async)
  
- 쓰기는 원본 데이터베이스, 읽기는 복제본에서 처리하여 워크로드 분산
- MySQL, PostgreSQL, MariaDB, Oracle, Aurora에서 지원

안정성이 아닌 퍼포먼스를 위한 서비스
  
총 5개 까지 생성 가능
  
각각의 복제본은 고유 DNS가 할당됨 -> 접근 가능
- 원본 DB 장애 발생 시 수동으로 DNS 변경이 필요함

복제본 자체에 Multi-AZ 설정 가능 (MySQL, PostgreSQL, MariaDB, Oracle)
  
Multi-AZ DB에 Read Replica 설정 가능
  
자동 백업이 활성화 되어 있어야 읽기 전용 복제본 생성 가능
  
각 DB의 엔진 버전이 다를 수 있음

![](https://user-images.githubusercontent.com/28394879/141417131-c92bc7c6-8f64-4644-b2c0-7eb8e2bd2fb6.png)

![](https://user-images.githubusercontent.com/28394879/141417813-182df6ec-a81b-4055-8014-e63a4e3e7a9a.png)

위 사진에서 복제를 끊고 아래 사진처럼 구성할 수 있다.

![](https://user-images.githubusercontent.com/28394879/141417884-68265c7c-25f5-497d-87de-1cae5cf13eb8.png)

## RDS Multi Region
다른 리전에 지속적으로 동기화 시키는 DB클러스터를 생성(Async 복제)
  
주로 로컬 퍼포먼스 혹은 DR(Disaster Recovery)시나리오로 활용
  
각 리전별로 자동 백업 가능
  
리전별로 Multi-AZ 가능

![](https://user-images.githubusercontent.com/28394879/141418965-f6e98382-43bf-4a2e-ba53-b7e2036d607c.png)