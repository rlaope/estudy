# 이벤트 - 시스템 간 강결합 문제

> 여러 바운디드 컨텍스트 간 강결합이 발생하면 유지보수가 힘들다.

쇼핑몰에서 구매를 취소하면 환불을 처리해야 하는데 주문 도메인 객체에서 도메인 서비스를 파라미터로 전달받고 도메인 기능에서 도메인 서비스를 실행한다.  

```java
public class Order {
  ...
  // 외부 서비스를 실행하기 위해 도메인 서비스를 파라미터로 전달받음
  public void cancel(RefundService refundService) {
    verifyNotYetShipped();
    this.state = OrderState.CANCELED;

    this.refundStatus = State.REFUND_STARTED;
    try {
      refundSvc.refund(getPaymentId());
      this.refundStatus = State.REFUND_COMPLETED;
    } catch(Exception ex) {
      ...
    }
  }
  ...
}
```

다음과 같이 응용 서비스에서 환불 기능을 실행할 수도 있다.

```java
public class CancelOrderService {

  private RefundService refundService;
  
  @Transactional
  public void cancel(OrderNo orderNo) {
    Order order = findOrder(orderBo);
    order.cancel();
    
    order.refundStarted();
    try {
      refundService.refund(order.getParymentId());
      order.refundCompleted();
    } catch(Exception ex) {
      ...
    }
  }
  ...
}
```

보통 외부에 있는 결제 시스템을 사용하는데 이때 두 가지 문제가 발생할 수 있다.

### 트랜잭션 처리
- 외부 서비스가 정상이 아닐 경우 트랜잭션 처리를 어떻게 해야 할지 예매해진다.
  외부의 환불 서비스를 실행하는 과정에서 익셉션이 발생하면 주문 취소 트랜잭션을 롤백하는 것이 맞다.
  하지만 무조건 롤백하지 않고 주문 취소 상태로 변경하고 환불만 나중에 하는 방식도 가능하다.

- 도메인 객체에 서비스를 전달하면 서로 다른 도메인 로직이 섞이는 설계상 문제가 나타날 수 있다. 
  이에 더해 기능을 추가할 때 로직이 섞이는 문제가 더 커지고 트랜잭션 처리가 더 복잡해진다.


### 성능
- 외부 서비스 성능에 직접적인 영향을 받아 환불을 처리하는 외부 시스템의 응답 시간이 길어지면 그만큼 대기 시간도 길어진다.

이와 같은 문제가 발생하는 이유는 여러 바운디드 컨텍스트 간의 **강결합(high coupling)** 때문이다.
  
강결합을 없애기 위해 이벤트를 사용하면, 특히 비동기 이벤트는 두 시스템 간의 결합을 크게 낮출 수 있다.
