# Event - High Coupling Issue Between Systems

> If high coupling occurs between multiple bounded contexts, maintenance becomes difficult.

When a purchase is canceled in a shopping mall, a refund needs to be processed. The order domain object receives a domain service as a parameter and executes it within a domain function.

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

Alternatively, the refund function can be executed in an application service as follows.

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

Typically, an external payment system is used, and in this case, two problems can arise.

### Transaction Handling
- If the external service is not normal, it becomes ambiguous how to handle the transaction. If an exception occurs during the execution of the external refund service, it is correct to roll back the order cancellation transaction. However, it is also possible to change the order to a canceled state without unconditionally rolling back, and process the refund later.

- If a service is passed to a domain object, a design problem can arise where different domain logics get mixed. Furthermore, when adding features, the problem of mixed logic becomes greater, and transaction handling becomes more complex.

### Performance
- Directly affected by external service performance, if the response time of the external system processing refunds becomes long, the waiting time also increases proportionally.

The reason such problems occur is due to **high coupling** between multiple bounded contexts.

To eliminate high coupling, using events, especially asynchronous events, can significantly reduce the coupling between two systems.
