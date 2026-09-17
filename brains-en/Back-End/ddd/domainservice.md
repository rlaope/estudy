# Features Requiring Multiple Aggregates and Domain Services

## Features Requiring Multiple Aggregates
> What should be done with domain concepts that cannot be implemented with a single aggregate?

Payment amount calculation logic cannot be implemented with a single aggregate.

When calculating the actual payment amount, the following aggregates are required:

- **Product Aggregate**: Requires the price of the purchased product, and shipping fees may be added depending on the product.
- **Order Aggregate**: Requires the quantity purchased for each product.
- **Discount Coupon Aggregate**: Discounts the total order amount according to the specified discount amount or percentage per coupon. Discount calculation becomes complex if there are constraints such as allowing multiple uses of discount coupons based on conditions or applying them only to products in a specified category.
- **Member Aggregate**: Additional discounts are possible based on membership grade.

Domain features that are ambiguous to place in a single aggregate should not be implemented in a specific aggregate.
An aggregate implementing functionality beyond its scope of responsibility increases its external dependencies and makes modifications difficult.
Furthermore, domain concepts that extend beyond the aggregate's scope become hidden within the aggregate and are not explicitly revealed.
This problem is solved using **Domain Services**.

<br>

## Domain Services
> A domain service changes the state of an aggregate or calculates a value from its state.
Domain services are used to **express domain logic located in the domain layer** as follows:

- **Calculation Logic**: Calculation logic that requires multiple aggregates or is too complex to be placed in a single aggregate.
- **Domain Logic Requiring External System Integration**: Domain logic that needs to use other systems for implementation.

## Calculation Logic and Domain Services
> While application services handle application logic, domain services handle domain logic.
Unlike components such as aggregates or values in the domain layer, a domain service implements only logic without state.
The entity using a domain service can be an aggregate or an application service.
If an aggregate object uses a domain service, the application service passes the domain service to the aggregate object.

```java
public class OrderService {
	private DiscountCalculationService discountCalculationService;

	@Transactional
	public OrderNo placeOrder(OrderRequest orderRequest) {
		OrderNo orderno = orderRepository.nextId();
		Order order = createOrder(orderNo, orderRequest);
		orderRepository.save(order);
		// 응용 서비스 실행 후 표현 영역에서 필요한 값 리턴
		return orderNo;
	}

	private Order createOrder(OrderNo orderNo, OrderRequest orderReq) {
		Member member =findMember(orderReq.getOrdererId());
		Order order = new Order(orderNo, orderReq.gerOrderLines(),
							orderReq.getCoupons(), createOrderer(member),
							orderReq.getShippingInfo());
		order.calculateAmounts(this.discountCalculationService, member.getGrade());
		return order;
	}
	...
}
```

Passing a domain service as a parameter when executing an aggregate's method means that the aggregate depends on the domain service.
If one is deeply involved with DI and AOP, they might want to handle this situation with dependency injection.
However, a domain object represents a conceptually single model using data methods composed of fields.
But injecting a domain service, which is unrelated to the data itself, as a field is merely an overreach.
Conversely, an aggregate can also be passed when executing a domain service function, as shown below.

```java
public class TransgerService {

	public void transfer(Account fromAcc, Account toAcc, Money amounts) {
		fromAcc.withdraw(amounts);
		toAcc.credit(amounts);
	}
	...
}
```

### Domain Service vs. Application Service
- **Domain Service**: Changes the state of an aggregate or calculates a value from an aggregate's state.
- **Application Service**: Handles transactions or flow control.

## External System Integration and Domain Services
> Integration features with external systems or other domains can also be domain services.

System integration is possible via HTTP API calls, but from the perspective of the domain, it can be seen as domain logic.
Therefore, it can be expressed as a domain service, but it should be viewed from a domain perspective, not from the perspective of integrating with another system.

## Package Location of Domain Services
> Since domain services express domain logic, they are located in the same package as other domain components.
![](https://user-images.githubusercontent.com/43809168/99547542-fe4db380-29fa-11eb-86f8-80e6801fc2b7.png)

If there are many domain services or if you want to explicitly distinguish them from other components like entities or values, you can place them under the `domain` package:
- domain
  - service
  - model
  - repository

... by separating them into sub-packages as shown.

## Domain Service Interfaces and Classes
> If the logic of a domain service is not fixed, the domain service can be implemented as an interface.
When implementing domain logic using an external system or a separate engine, the interface is located in the domain layer, and the implementation is located in the infrastructure layer.
If the implementation of a domain service depends on a specific implementation technology or executes an external system's API, the domain service should be abstracted with an interface.

![](https://user-images.githubusercontent.com/43809168/99547954-7b792880-29fb-11eb-9cec-e8974038d28b.png)
