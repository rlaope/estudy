# Kafka Consumer Offset 수동 commit 

### auto.offset.commit

Kafka의 컨슈머는 읽은 메시지의 위치를 추적하기 위해 offset을 커밋하는 역할을 담당한다. 이 offset은 kafka의 내부 토픽인 `__consumer_offsets`에 저장되며, 컨슈머는 commit이 발생할 때마다 이 offset 값을 갱신한다. 이 후 컨슈머는 offset값을 참조하여 다음에 처리할 레코드를 읽는다.

컨슈머가 offset을 commit하는 방식에는 자동 커밋과 수동 커밋이 있다. auto commit은 `auto.offset.commit` 설정이 true일때 활성화 되며 기본값이다.

만약 자동 커밋이 활성화되어 있다면, poll() 메서드가 실행되었을 때 수행 시간이 `auto.commit.interval.ms` 설정값을 초과하는 경우 자동으로 offset이 commit된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdwL8Ae%2FbtsFxYBH8a2%2FgrvVUopy9FYVYNbW9MtOtk%2Fimg.png)

**컨슈머는 poll()을 호출할 때마다 auto.commit.interval.ms에 정의한 커밋 할 시간인지 아닌지에 여부를 체크하고 poll 요청으로 가져온 마지막 오프셋을 커밋한다.**

Kafka 소스코드를 확인해보겠다.  [ConsumerCoordinator.java #L1214](https://github.com/a0x8o/kafka/blob/master/clients/src/main/java/org/apache/kafka/clients/consumer/internals/ConsumerCoordinator.java#L1214)

```java
public void maybeAutoCommitOffsetsAsync(long now) {
	if(autoCommitEnabled) {
		nextAutoCommitTimer.update(now);
		if(nextAutoCommitTimer.isExpired()) {
			nextAutoCommitTimer.reset(autoCommitIntervalMs);
			doAutoCommitOffsetsAsync();
		}
	}
}
```

1. **자동 커밋 활성화 여부 확인**: autoCommitEnabled 값을 통해 자동 커밋 기능이 활성화 되었는지 확인한다.
2. **타이머 업데이트**: 현재 시각을 기준으로 다음 자동 커밋이 예정된 타이머를 업데이트한다. update 메서드는 현재 시간과 마지막 오프셋 커밋 시간 사이의 경과시간을 계산하여, 다음 자동 커밋 실행까지 남은 시간을 조정하낟. 이 과정은 자동 커밋 주기를 관리하기 위한 것으로, 실제 커밋 작업이 시작되기 전에 현재 시간과 설정된 커밋 간격을 기반으로 타이머 상태를 최신화 한다.
3. **타이머 만료 여부 확인**: 업데이트된 타이머를 검사하여 설정된 자동 커밋 간격이 만료되었는지 확인한다. 만약 타이머가 만료되었다면(auto.commit.interval.ms 설정 시간 경과) 다음 단계로 진행한다
4. **타이머 리셋**: 타이머를 리셋하여 다음 자동 커밋 시점을 현재 시각으로부터 설정된 간격 이후로 조정한다.
5. **비동기 오프셋 커밋 실행**: 실제로 오프셋 커밋을 비동기로 수행하는 메서드를 호출한다.


<br>

### 자동 커밋 설정시 주의할 점

자동 커밋 설정은 편리하다 그러나 컨슈머 그룹의 리밸런싱이 발생할 때 메시지의 중복 처리나 유실 문제가 발생할 수 있다.

유실 문제에 대해서 얘기해보자. 자동 커밋 설정이 1초로 지정된 경우에 컨슈머는 정상적으로 `poll()`을 호출하여 메시지를 가져오고 처리한다. 그러나 메시지 처리 과정이 1초를 초과하게 된다면 문제가 발생하는데, 예를 들어 메시지를 DB에 저장하거나 다른 서비스에 전달하는 과정에서 네트워크 지연이나 장애같은 어떠한 이유로 1초를 초과했다면 이러한 상황에서 메시지 처리가 완료되기 전에 오프셋 커밋이 일어나게 되고, 그 후 메시지 처리에서 에러가 발생하는 경우 해당 메시지는 다시 처리될 수 없게 된다. 즉 메시지 유실이 일어난다.

메시지 중복도 같은 문제다 다음 예시와 같은 설정일 때 발생할 수 있다.
- enable.auto.commit = true
- auto.commit.interval.ms = 5000ms
- max.poll.records = 100

컨슈머 끼리 poll를 하여 최대 100개의 레코드를 가져온다. 그 후 이 중에서 30개의 레코드까지 처리한 상태에서 커밋이 이루어지기 전에 애플리케이션이 종료되거나 리밸런싱이 발생하면 컨슈머들이 재할당되며, 이후 컨슈머는 마지막으로 커밋된 offset부터 다시 데이터를 poll하는데 이미 성공적으로 처리했던 30개의 레코드를 다시 중복 처리한다.

**메시지 유실의 경우 Dead Letter Queue(DLT)** 구성으로 처리하지 못한 메시지를 별도로 관리하고 재처리하는 방식으로 해결할 수 있고, 메시지 중복의 경우에는 비즈니스 로직 내부에서 멱등성을 보장하는 로직을 통해 해결할 수는 있지만 이러한 문제들을 사전 방지하기 위하여 수동 커밋을 이용해볼 수 있다.

<br>

### 수동 커밋 설정

Kafka 컨슈머의 커밋을 수동으로 설정하기 위해서는 auto.offset.commit 설정뿐만 아니라 ack-mode 설정까지 이루어져야한다.

먼저 kafka 컨슈머 설정에서 ENABLE_AUTO_COMMIT_CONFIG를 false로 설정해 자동 커밋을 비활성화 할 수 있다. 

```java
private Map<String, Object> consumerConfigs() {
	Map<String, Object> configs = new HashMap<>();    
	// ...     
	configs.put(ConsumerConfig.ENABLE_AUTO_COMMIT_CONFIG, "false");     
	return configs; 
}
```

그러나 실제로는 enable.auto.commit을 false로 지정해도 커밋이 정상적으로 이루어져 Lag가 쌓이지 않는다. 그 이유는 **enable.auto.commit=false 설정에 의해 kafka 클라이언트의 자동 커밋은 비활성화 되지만, Spring은 기본적으로 오프셋 커밋을 활성화 상태로 두고 있기 때문이다.** 때문에 위 설정에는 자동 커밋 때처럼 auto.commit.interval.ms의 영향을 받지는 않지만 리스너 메서드가 완료되면 자동으로 커밋이 일어나게된다.

따라서 리스너 컨테이너의 ack mode까지 별도로 수정해야만 진정한 의미의 수동 커밋이 이루어질 수 있다. AckMode는 설정을 위한 여러 방법이 있지만 ContainerFactory를 통해 간단하게 설정이 가능하다.

```java
@Bean 
public ConcurrentKafkaListenerContainerFactory<String, String> testKafkaListenerContainerFactory() {
	ConcurrentKafkaListenerContainerFactory<String, String> testFactory = new ConcurrentKafkaListenerContainerFactory<>();
	testFactory.getContainerProperties().setAckMode(AckMode.MANUAL);
	testFactory.setConsumerFactory(testConsumerFactory());
	return testFactory; 
}
```

Ackmode default는 BATCH(poll() 메서드로 호출된 레코드가 모두 처리된 이후 커밋한다.)이기 때문에, enable.auto.commit=false로 지정하더라도 리스너 메서드가 종료될 때 스프링에서 자동 커밋이 수행하며 커밋이 일어나게 되었던 것이다.

따라서 MANUAL, MANUAL_IMMEDIATE로 지정해야지만 원래 가정했던 것 처럼 리스너에서 토픽을 소비하지만 offset commit은 일어나지 않고 Lag가 쌓이게 된다. 그리고 다음과 같이 `Acknowledgement.acknowledge()`를 호출해 커밋 시점을 직접적으로 적용할 수 있다.

>  MANUAL: Acknowledgement.acknowledge() 메서드가 호출되면 다음번 poll()때 커밋한다.
>  MANUAK_IMMEDIATE: Acknowledgement.acknowledge() 메서드가 호출되면 바로 커밋한다.




