# Kafka ISR(In Sync Replica)

### Kafka Replication Factor

브로커에게 리플레키이션 팩터 수 만큼 토픽안의 파티션들을 복제하도록 하는 설정 값이다. 복제본이 생기기 때문에 리더와 팔로워(master, slave) 개념이 존재하고, 가장 중요한 것은 모든 읽기 쓰기는 리더를 통해서 일어난다는 것이다.

#### 리더, 팔로워

![](https://user-images.githubusercontent.com/45676906/211140249-e31556b5-ebf6-443b-a455-e3d08ab82940.png)

리더는 모든 데이터의 읽기 쓰기 작업을 처리하고, 팔로워는 주기적으로 리더를 보면서 데이터를 sync 한다. 이 방법으로 리플레키이션을 유지한다.

### ISR

프로듀서가 리더에게 메세지를 발행하면 팔로워가 해당 메세지를 복제하는데 여기서 팔로워에 문제가 생겨서 복제를 제대로 되지 않은 상황에서 리더가 문제가 생겨 failover하면 **데이터 정합성 문제**가 발생한다.

Kafka는 이러한 현상을 막기 위해서 `ISR(In Sync Replica)`라는 개념을 도입했다. 
ISR이 현재 리플리케이션되고있는 리플리케이션 그룹이라고 생각하면 된다/

**ISR 규칙에 대해서 알아보겠다.**
1. ISR에 속해있는 구성원만이 리더의 자격을 가질 수 있다. Replica 그룹으로 관리하며 ISR 그룹안의 구성원이 리더가 자신의 역할을 하지 못하게 되는 경우(예기치 못하게 죽는다던지..) **팔로워가 리더로 선정**되며 그 역할을 대신한다.
2. ISR 그룹의 리더는 팔로워들이 주기적으로 데이터를 확인하고 있는지 **일정 주기(replica.lag.time.max.ms)** 만큼 확인하고 요청이 오지 않으면, 리더는 해당 팔로워의 이상을 감지하고 해당 팔로워는 더 이상 리더의 역할을 대신할 수 없다고 판단하고 ISR 그룹에서 해당 팔로워를 추방시킨다. ISR에서 추방당한 팔로워는 추방과 동시에 리더 자격도 박탈당한다.
3. 이런 작업을 하기 위해 ISR의 리더와 팔로워간의 **데이터 동기화 작업을 매우 중요**하게 처리하고 있다. ISR을 통해 파티션을 관리함으로써 리플리케이션의 신뢰성을 높히고 있다.


![](https://user-images.githubusercontent.com/45676906/211140369-36f84961-662a-4c93-b636-80bd4109db57.png)

Producer가 리더에게 메세지를 보내면 팔로워가 리더로부터 데이터를 복제한다. 만약 여기 예제에서 Broker2는 복제에 성공했지만 Broker3는 복제에 실패했다고 가정하겠다.

일단 리더 브로커는 producer에게 ack를 보낸다.

![](https://user-images.githubusercontent.com/45676906/211140935-8450b400-bdd9-402c-a119-7b7f2a8d587c.png)

여기서 `replica.lag.time.max.ms`만큼의 확인 요청이 오지 않는다면, 문제가 있는 팔로워를 ISR 그룹에서 제외시킨다.

![](https://user-images.githubusercontent.com/45676906/211140553-6bf78afc-a3c3-42f2-a258-87cd281a7800.png)


아래는 logtopic003의 topic의 partition3 replica2인 경우 isr구조를 확인한 것이다.

```bash
bin/kafka-topics.sh --describe --bootstrap-server localhost:9092 --topic logtopic003 
Topic: logtopic003 PartitionCount: 3 ReplicationFactor: 2 
Configs: 
	Topic: logtopic003 Partition: 0 Leader: 2 Replicas: 2,1 Isr: 2,1 
	Topic: logtopic003 Partition: 1 Leader: 3 Replicas: 3,2 Isr: 3,2 
	Topic: logtopic003 Partition: 2 Leader: 1 Replicas: 1,3 Isr: 3,1
```


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fcdk7z6%2FbtqEamKhmSA%2FbjbSXdRf5xaknJOTbMV9Wk%2Fimg.png)

