# Apache Kafka 주요 요소2 Zookeeper, Broker + 클러스터링 해보기

![](https://velog.velcdn.com/images/kidae92/post/a642368e-df63-4d4e-80d3-1b3c3f8d58b9/image.png)

## Broker, Zookeeper

### Kafka Broker
Kafka Broker는 Partition에 대한 Read 및 Write를 관리하는 소프트웨어
  
Kafka Server라고 부르기도 하며, Topic내의 Partition들을 분산, 유지 및 관리
  
각각의 Broker 들은 ID로 식별됨 (단 ID는 숫자)
  
Topic의 일부 Partition들을 포함 -> Topic 데이터의 일부분(Partition)을 갖을 뿐 데이터 전체를 갖고 있지 않음

Kafak Cluster: 여러 개의 Broker들로 구성되며, CLient는 특정 Broker에 연결하면 전체 클러스터에 연결됨
  
최소 3대 이상의 Broker를 하나의 Cluster로 구성해야지만 4대 이상을 권장한다고 함

### Kafka Broker ID와 Partition ID 의 관계 그리고 Bootstrap Servers

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2Fd5fd952d-80a8-4cd3-8ccb-cc97835c83e5%2Fimage.png)

Broker ID와 Partition ID간에는 아무런 관계가 없음
  
Topic을 구성하는 Partition들은 여러 Broker에 분산됨
  
Topic 생성시 kafka가 자동으로 Topic을 구성하는 전체 Partition들은 모든 Broker에게 할당해주고 분배해 줌
  
모든 Kafka Broker는 Bootstrap 서버라고 부름
  
하나의 Broker에만 연결하면 Cluster 전체에 연결됨 -> 하지만, 특정 Broker 장애를 대비하여, 전체 Broker List(IP, port)를 파라미터로 입력하는 것을 권장한다고 함
  
각각의 Broker는 모든 Broker, Topic, Partition에 대해서 알고 있음 (Meta data)

### Zookeeper
![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F3a1d0069-8701-425d-9357-f4adedccd63d%2Fimage.png)

Zookeeper는 Broker를 관리 (Broker 들의 목록/설정을 관리)하는 소프트웨어
  
Zookeeper 없이는 Kafka가 작동할 수 없으며, Zookeeper는 홀수의 서버로 작동하게 설계되어 있음(최소 3, 권장 5)
  
Zookeeper에는 Leader(writes)가 있고 나머지 서버는 Follower(reads)
  
Zookper는 분산형 Configuration 정보 유지, 분산 동기화 서비스를 제공하고 대용량 분산 시스템을 위한 네이밍 레지스트리를 제공하는 소프트웨어다.
  
분산 작업을 제어하기 위한 Tree 형태의 데이터 저장소 -> Zookeeper를 사용하여 멀티 Kafka Broker들 간의 정보(변경 사항 포함) 공유, 동기화 등을 수행한다.
  
Ensemble은 Zookeeper 서버의 클러스터 Quorum(쿼럼)은 "정족수"이며, 합의체가 의사를 진행시키거나 의결을 하는데 필요한 최소한도의 인원수를 뜻한다.
  
분산 코디네이션 환경에서 예상치 못한 장애가 발생해도 분산 시스템의 일관성을 유지시키기 위해서 사용한다.
  
ex) Ensemble이 3대로 구성되었다면 Quorum은 2, 즉 Zookeeper 1대가 장애가 발생하더라도 정삭 동작, Ensemble이 5대로 구성되었다면 Quorum은 3, 즉 Zookeeper 2대가 장애가 발생하더라도 정상 동작

## 클러스터링 해보기
CentOS8에서 docker-compose를 사용해 컬러스터링을 해보았다.  
3개의 Zookeeper와 3개의 Broker, AKHQ라는 Broker 데이터 리소스 모니터링 툴이다.  
이미지는 Confluent에서 제공하는 이미지를 사용한다.

1. etc/hosts
![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2Fe4d4b576-a4d3-4b17-a4ec-707d2081a3c3%2Fimage.png)
kafka1 ~ 3까지 등록을 해준다

2. dofker-compose -f 파일명.yml up

```yml
version: '3'
services:
  zookeeper-1:
    hostname: zookeeper1
    image: confluentinc/cp-zookeeper:6.2.0
    environment:
      ZOOKEEPER_SERVER_ID: 1
      ZOOKEEPER_CLIENT_PORT: 12181
      ZOOKEEPER_DATA_DIR: /zookeeper/data
      ZOOKEEPER_SERVERS: zookeeper1:22888:23888;zookeeper2:32888:33888;zookeeper3:42888:43888
    ports:
      - 12181:12181
      - 22888:22888
      - 23888:23888
    volumes:
      - ./zookeeper/data/1:/zookeeper/data

  zookeeper-2:
    hostname: zookeeper2
    image: confluentinc/cp-zookeeper:6.2.0
    environment:
      ZOOKEEPER_SERVER_ID: 2
      ZOOKEEPER_CLIENT_PORT: 22181
      ZOOKEEPER_DATA_DIR: /zookeeper/data
      ZOOKEEPER_SERVERS: zookeeper1:22888:23888;zookeeper2:32888:33888;zookeeper3:42888:43888
    ports:
      - 22181:22181
      - 32888:32888
      - 33888:33888
    volumes:
      - ./zookeeper/data/2:/zookeeper/data

  zookeeper-3:
    hostname: zookeeper3
    image: confluentinc/cp-zookeeper:6.2.0
    environment:
      ZOOKEEPER_SERVER_ID: 3
      ZOOKEEPER_CLIENT_PORT: 32181
      ZOOKEEPER_DATA_DIR: /zookeeper/data
      ZOOKEEPER_SERVERS: zookeeper1:22888:23888;zookeeper2:32888:33888;zookeeper3:42888:43888
    ports:
      - 32181:32181
      - 42888:42888
      - 43888:43888
    volumes:
      - ./zookeeper/data/3:/zookeeper/data

  kafka-1:
    image: confluentinc/cp-kafka:6.2.0
    hostname: kafka1
    depends_on:
      - zookeeper-1
      - zookeeper-2
      - zookeeper-3
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper1:12181,zookeeper2:22181,zookeeper3:32181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka1:19092
      KAFKA_LOG_DIRS: /kafka
    ports:
      - 19092:19092
    volumes:
      - ./kafka/logs/1:/kafka

  kafka-2:
    image: confluentinc/cp-kafka:6.2.0
    hostname: kafka2
    depends_on:
      - zookeeper-1
      - zookeeper-2
      - zookeeper-3
    environment:
      KAFKA_BROKER_ID: 2
      KAFKA_ZOOKEEPER_CONNECT: zookeeper1:12181,zookeeper2:22181,zookeeper3:32181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka2:29092
      KAFKA_LOG_DIRS: /kafka
    ports:
      - 29092:29092
    volumes:
      - ./kafka/logs/2:/kafka

  kafka-3:
    image: confluentinc/cp-kafka:6.2.0
    hostname: kafka3
    depends_on:
      - zookeeper-1
      - zookeeper-2
      - zookeeper-3
    environment:
      KAFKA_BROKER_ID: 3
      KAFKA_ZOOKEEPER_CONNECT: zookeeper1:12181,zookeeper2:22181,zookeeper3:32181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka3:39092
      KAFKA_LOG_DIRS: /kafka
    ports:
      - 39092:39092
    volumes:
      - ./kafka/logs/3:/kafka

  akhq:
    image: tchiotludo/akhq:latest
    hostname: akhq
    depends_on:
      - kafka-1
      - kafka-2
      - kafka-3
    environment:
      AKHQ_CONFIGURATION: |
        akhq:
          connections:
            kafka:
              properties:
                bootstrap.servers: kafka1:19092,kafka2:29092,kafka3:39092
    ports:
      - 8080:8080
```

3. 디렉토리 권한
볼륨 설정을 한 경로에 권한 문제로 Kafka가 종료되는 상황이 있다. 권한 변경을 통해 해결 가능

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F32f79d6a-8dc2-4d07-a9b8-809dc2a87da9%2Fimage.png)

다시 kafka 컨테이너를 실행해주면 정상 작동함 

4. confluent 설치 - 사용할 Command를 제공해준다.

```
$ curl -O http://packages.confluent.io/archive/7.0/confluent-7.0.1.tar.gz
$ tar -zxvf confluent-7.0.1.tar.gz
$ cd confluent-7.0.1/bin
```

#topic 생성
`./kafka-topics --bootstrap-server localhost:19092 --create --topic test --partitions 2 --replication-factor 3`  
#test라는 토픽으로 producer가 메시지를 보낼 수 있는 shell이 실행됨  
`./kafka-console-producer --bootstrap-server localhost:19092 --topic test`  
#test라는 토픽에서 consumer가 메시지를 받을 수 있는 것을 볼 수 있음  
`./kafka-console-consumer --bootstrap-server localhost:19092 --topic test`  

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F3a82f94e-7c7d-4f85-9db4-79ecae6b96f3%2Fimage.png)

```
$ ./kafka-topics --describe --bootstrap-server localhost:19092 --topic test   # 브로커를 29092 39092 총 3개를 운영하고 있기 떄문에 아무거나 넣어도 상관없음
```