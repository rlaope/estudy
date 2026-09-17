# 이벤트, 핸들러, 디스패처

## 이벤트 개요
> 이벤트를 통해 도메인간 의존을 제거하고 후처리를 쉽게 할 수 있다.

이벤트라는 용어는 **과거에 벌어진 어떤 것**을 의미한다.  
  
이벤트가 발생했다는 것은 상태가 변경됐다는 것을 의미한다.  
  
이벤트가 발생하는 것에서 끝나지 않고 그 이벤트에 반응하여 원하는 동작을 수행하는 기능을 구현한다.  
  
~ 할대, ~가 발생하면, 만약 ~ 하면과 같은 요구사항은 도메인의 상태 변경과 관련된 경우가 많고 이벤트로 구현할 수 있다.  
  
예를 들어 주문 취소를 할 때 이메일을 보낸다 라는 요구사항에서 주문을 취소할 때는 주문이 취소 상태로 바뀌는 것을 의미하고 '주문 취소됨 이벤트'를 활용해 구현할 수 있다.  


## 이벤트 관련 구성 요소
도메인 모델에 이벤트를 도입하기 위해 다음 네 개의 구성요소가 필요하다
- 이벤트
- 생성 주체
- 디스패치 (퍼블리셔)
- 핸들러 (구독자)

![](https://user-images.githubusercontent.com/42582516/160224240-ca76a9a2-321c-456b-a27b-d745d5b36b0e.png)

### 이벤트 생성 주체
- 이벤트 생성 주체는 엔티티, 밸류, 도메인 서비스와 같은 도메인 객체다
- 도메인 로직을 실행해서 상태가 바뀌면 관련 이벤트를 발생시킨다.

### 이벤트 핸들러
- 발생한 이벤트에 반응한다.
- 이벤트에 담긴 데이터를 이용해 원하는 기능을 실행한다.

### 이벤트 디스패처
- 이벤트 생성 주체와 이벤트 핸들러를 연결해준다.
- 이벤트 디스패처의 구현 방식에 따라 이벤트 생성과 처리를 동기나 비동기로 실행하게 된다.

## 이벤트의 구성
이벤트는 다음과 같은 정보를 담는다.
- 이벤트 종류 - 클래스 이름으로 이벤트 종류를 표현
- 이벤트 발생 시간
- 추가 데이터 - 주문번호, 신규 배송지 정보 등 이벤트와 관련된 정보

다음과 같이 이벤트는 과거 시제를 사용한 클래스다.

```java
public class ShippingInfoChangedEvent {
  
  private String orderNumber;
  private long timestamp;
  private ShippingInfo newShippingInfo;

  // 생성자, getter
}
```

이벤트를 생성하는 주체가 되는 애그리거트는 다음과 같다.  
Eventes.raise()는 디스패처를 통해 이벤트를 전파하는 기능을 제공한다.

```java
public class Order {
  public void changeShippingInfo(ShippingInfo newShippingInfo) {
    verifyNotYetShipped();
    setShippingInfo(newShippingInfo);
    Events.raise(new ShippingInfoChangedEvent(number, newShippingInfo));
  }
  ...
}
```
디스패처로부터 필요한 작업을 수행하는 핸들러는 다음과 같이 구현할 수 있다.

```java
public class ShippingInfoChangedHandler {

  @EventListener(ShippingInfoChangedEvent.class)
  public void handle(ShppingInfoChangedEvent evt) {
    shippingInfoSynchronizer.sync(
      evt.getOrderNumber(),
      evt,getNewShippingInfo());
  }
  ...
}
```

이벤트는 이벤트 핸들러가 작업을 수행하는 데 필요한 데이터를 담아야 한다.  
이벤트는 데이터를 담아야 하지만 그렇다고 이벤트 자체와 관련 없는 데이터를 포함할 필요는 없다.

## 이벤트 용도
이벤트는 크게 두 가지 용도로 쓰인다.
- 트리거
- 데이터 동기화

### 트리거
도메인의 상태가 바뀔 때 다른 후처리가  필요하면 후 처리를 실행하기 위한 트리거로 이벤트를 사용할 수 있다.  
예매 결과를 SMS로 통지하거나 주문을 취소했을 때 환불을 하는 후처리를 이벤트를 통해 구현할 수 있다.  
다음과 같이 추가 기능이 생긴다면 해당 기능을 처리하는 핸들러를 만들면 해결된다.

![](https://user-images.githubusercontent.com/43809168/99668375-fe0fef80-2ab0-11eb-8e78-7bcbd78e84c8.png)

## 이벤트, 핸들러, 디스패처 구현
> 스프링의 ApplicationEventPublisher를 이용해 쉽게 구현한다.

- 이벤트 클래스: 이벤트를 표현한다.
- 디스패처: 스프링이 제공하는 ApplicationEventPublisher를 이용한다.
- Events: 이벤트를 발행한다. 이벤트 발행을 위해 ApplicationEventPublisher를 이용한다.
- 이벤트 핸들러: 이벤트를 수신해서 처리한다. 스프링이 제공하는 기능을 사용한다.

## 이벤트 클래스
이벤트는 과거에 벌어진 상태 변화나 사건을 의미하므로 클래스 이름은 과거 시제를 사용해야 한다.  
모든 이벤트가 공통으로 갖는 프로퍼티가 존재한다면 관련 상위 클래스를 만들 수도 있다.

## Events 클래스와 ApplicationEventPublisher
이벤트 발생과 출판을 위해 스프링이 제공하는 ApplicationEventPublisher를 사용한다.  
스프링 컨테이너는 ApplicationEventPublisher도 된다.  

```java
public class Events {
  private static ApplicationEventPublisher publisher;
  
  static void setPublisher(ApplicationEventPublisher publisher) {
    Events.publisher = publisher;
  }
  
  public static void raise(Object event) {
    if (publisher != null) {
      publisher.publishEvent(event);
    }
  }
}
```

Events 클래스는 ApplicationEventPublisher를 이용해 이벤트를 발생시킨다.  
Events 클래스의 setPublihser() 메서드에 이벤트 퍼블리셔를 전달하기 위해 스프링 설정 클래스를 작성한다.

```java
@Configuration
public class EventConfiguration {
  private ApplicationContext applicationContext;
  
  @Bean
  public InitializingBean eventsInitializer() {
    return () -> Events.setPublisher(applicationContext);
  }
}
```

InitializingBean 타입은 스프링 빈 객체를 초기화할 때 사용하는 인터페이스로, ApplicationEventPublisher를 상속받은 ApplicationContext를 전달해 Events 클래스를 초기화한다.

## 이벤트 발생과 이벤트 핸들러
해당 도메인 로직을 수행하고 Events.raise()를 이용해 관련 이벤트를 발생시킨다.  
발생시킨 이벤트를 처리하기 위해 핸들러를 다음과 같이 구현한다.  

```java
@Service
public class OrderCanceledEventHandler() {
  private RefundService refundService;
  
  @EventListener(OrderCanceledEvent.class)
  public void handle(OrderCanceledEvent event) {
    refundService.refund(event.getOrderNumber());
  }
}
```

## 흐름 정리
이벤트 처리의 흐름
1. 도메인 기능을 실행한다.
2. 도메인 기능은 Events.raise()를 이용해 이벤트를 발생시킨다.
3. Events.raise()는 스프링이 제공하는 ApplicationEventPublisher를 이용해서 이벤트를 출판한다.
4. ApplicationEventPublisher는 @EventListener(이벤트타입.class) 어노테이션이 붙은 메서드를 찾아 실행한다.

도메인 상태 변경과 이벤트 핸들러는 같은 트랜잭션 범위에서 실행된다.