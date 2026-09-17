# Apache Kafka 주요 요소 Producer, Counsumer, Topic, Partition, Segment

## Topic, Producer, Consumer

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F4e6ab402-6016-4efd-b8ef-baf77a6627f7%2Fimage.png)

- Producer: 메시지를 생산해서 Kafka의 Topic으로 메시지를 보내는 애플리케이션
- Consumer: Topic의 메시지를 가져와서 소비하는 애플리케이션
- Consumer group: Topic의 메시지를 사용하기 위해 협력하는 Consumer 들의 집합
- 하나의 Consumer는 하나의 Consumer Group에 포함되며, Consumer Group 내의 Consumer 들은 협력하여 Topic의 메시지를 분산 병렬 처리함

## Producer, Consumer 기본 동작 방식

- Commit Log: 추가만 가능하고 변경 불가능한 데이터 스트럭처, Event(데이터)는 항상 로그 끝에 추가되고 변경되지 않음
- Offset: Commit Log에서 Event의 위치, 아래 그림에서는 0부터 10까지 offset을 볼 수 있음

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F0d37ccaa-a260-4a6f-bda9-137448e57250%2Fimage.png)

Producer와 Consumer는 서로 알지 못하며, Producer와 Consumer는 각각 고유의 속도로 Commit Log에 Write 및 Read를 수행
  
다른 Consumer Group에 속한 Consumer들은 서로 관련이 없으며, Commit Log에 있는 Event를 동시에 다른 위치에서 Read할 수 있음

## Commit Log에서 Event 위치

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F521237d9-96bc-415e-b08b-dfbeac58565c%2Fimage.png)

Producer가 Write하는 LOG-END-OFFSET과 Consumer Group의 Consumer가 Read하고 처리한 후에 Commit한 CURRENT-OFFSET과의 차이 발생할 수 있다.
  
### offset 차이가 발생하는 경우 몇 가지 문제 사항
1. 데이터 유실
2. 중복 처리
3. 순서 문제

## Logical View(Topic, Partition, Segment)

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2Fa99600f1-d62b-44c8-83b3-22c9f9e58e8f%2Fimage.png)

- Topic: Kafka 안에서 메시지가 저장되는 장소, 논리적인 표현
- Partition: Commit Log, 하나의 Topic은 하나 이상의 Partition으로 구성, 병렬처리를 위해서 다수의 Partition 사용
- Segment: 메시지가 저장되는 실제 물리 File, Segment File이 지정된 크기보다 크거나 지정된 기간보다 오래되면 새 파일이 열리고 메시지는 새 파일에 추가됨

## Physical View(Topic, Partition, Segment)
Topic 생성시 Partition 개수를 지정하고, 각 Partition은 Broker들에 분산되며 Segment File들로 구성된다.
  
Rolling Strategy: `Log.segment.bytes(default 1 GB), log.roll.hour(default 168 hours)`

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2Fcb5c4b43-e2c5-4189-a462-6e8aad86258b%2Fimage.png)

## Partition당 하나의 Active Segment
Partition당 오직 하나의 Segment가 Active 되어 있다 -> 데이터가 계속 쓰여지고 있는 중

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F523e209f-8df1-44c9-b922-ca19fe9bc4e7%2Fimage.png)