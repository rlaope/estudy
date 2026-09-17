# 여러 애그리거트가 필요한 기능과 도메인 서비스

## 여러 애그리거트가 필요한 기능
> 한 애그리거트로 구현할 수 없는 도메인 개념은 어떻게 해야 할까?

결제 금액 계산 로직은 한 애그리거트로 기능을 구현할 수 없다. 
  
실제 결제 금액을 계산할 때는 다음과 같은 애그리거트들이 필요하다.

- **상품 애그리거트**: 구매하는 상품의 가격이 필요하고 상품에 따라 배송비가 추가되기도 한다.
- **주문 애그리거트**: 상품별로 구매 개수가 필요하다.
- **할인 쿠폰 애그리거트**: 쿠폰별로 지정한 할인 금액이나 비율에 따라 주문 총 금액을 할인한다. 할인 쿠폰을 조건에 따라 중복 사용할 수 있다거나 지정한 카테고리의 상품에만 적용할 수 있다는 제약 조건이 있다면 할인 계산이 복잡해진다.
- **회원 애그리거트**: 회원 등급에 따라 추가 할인이 가능하다.

이렇게 한 애그리거트에 넣기 애매한 도메인 기능을 특정 애그리거트에 구현하면 안된다.  
애그리거트는 자신의 책임 범위를 넘어서는 기능을 구현하기 때문에 외부에 대한 의존이 높아지게 되고 수정을 어렵게 만든다.  
뿐만 아니라 애그리거트의 범위를 넘어서는 도메인 개며이 애그리거트에 숨어들어 명시적으로 들어나지 않는다.  
이런 문제를 **도메인 서비스**를 이용해 해결한다.

<br>

## 도메인 서비스
> 도메인 서비스는 애그리거트의 상태를 변경하거나 상태 값으로 계산한다.
도메인 서비스는 다음과 같이 **도메인 영역에 위치한 도메인 로직을 표현할 때 사용**한다.

- 계산 로직: 여러 애그리거트가 필요한 계산 로직이나 한 애그리거트에 넣기에는 다소 복잡한 계산 로직
- 외부 시스템 연동이 필요한 도메인 로직: 구현하기 위해 타 시스템을 사용해야 하는 도메인 로직

## 계산 로직과 도메인 서비스
> 응용 서비스나 응용 로직을 다룬다면 도메인 서비스는 도메인 로직을 다룬다.
도메인 영역의 애그리거트나 밸류와 같은 구성요소와 다른 점은 도메인 서비스는 상태 없이 로직만 구현한다.  
도메인 서비스를 사용하는 주체는 애그리거트가 될 수도 있고 응용 서비스가 될 수도 있다.  
애그리거트 객체에서 도메인 서비스를 사용한다면 응용 서비스가 애그리거트 객체로 도메인 서비스를 전달한다.

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

애그리거트의 메서드를 실행할 때 도메인 서비스를 파라미터로 전달하는 것은 애그리거트가 도메인 서비스에 의존한다는 것을 의미한다.  
DI, AOP에 빠져있으면 이 상황을 의존 주입으로 처리하고 싶어진다.  
하지만 도메인 객체는 필드로 구성된 데이터 메서드를 이용해 개념적으로 하나인 모델을 표현한다.  
하지만 데이터 자체와 관련이 없는 도메인 서비스를 필드로 주입하는 것은 욕심에 불과하다.  
반대로 다음과 같이 도메인 서비스 기능을 실행할 때 애그리거트를 전달하기도 한다.

```java
public class TransgerService {

	public void transfer(Account fromAcc, Account toAcc, Money amounts) {
		fromAcc.withdraw(amounts);
		toAcc.credit(amounts);
	}
	...
}
```

### 도메인 서비스 vs 응용 서비스
- 도메인 서비스: 애그리거트의 상태 변경 or 애그리거트의 상태 값 계산
- 응용 서비스: 트랜잭션 처리 or 흐름 제어

## 외부 시스템 연동과 도메인 서비스
> 외부 시스템이나 타 도메인과의 연동 기능도 도메인 서비스가 될 수 있다.

시스템 간 연동은 HTTP API 호출로도 가능하짐나 해당 도메인 입장에서는 도메인 로직으로 볼 수 있다.  
따라서 도메인 서비스로 표현할 수 있는데, 이때 타 시스템과 연동한다는 관점으로 보는 것이 아니라 도메인 관점에서 바라봐야한다.

## 도메인 서비스의 패키지 위치
> 도메인 서비스는 도메인 로직을 표현하므로 다른 도메인 구성요소와 동일한 패키지에 위치한다.
![](https://user-images.githubusercontent.com/43809168/99547542-fe4db380-29fa-11eb-86f8-80e6801fc2b7.png)

도메인 서비스의 개수가 많거나 엔티티나 밸류와 같은 다른 구성요소와 명시적으로 구분하고 싶다면 domain 패키지 밑에
- domain
  - service
  - model
  - repository

와 같이 하위 패키지를 구분하여 위치시켜도 됩니다.

## 도메인 서비스의 인터페이스와 클래스
> 도메인 서비스의 로직이 고정되어 있지 않은 경우 도메인 서비스를 인터페이스로 구현할 수 있습니다.
도메인 로직을 외부 시스템이나 별도 엔진을 이용해 구현할 때 인터페이스는 도메인 영역에, 구현체는 인프라스트럭쳐 영역에 위치합니다.  
도메인 서비스의 구현이 특정 구현 기술에 의존하거나 외부 시스템의 API를 실행한다면 도메인 서비스는 인터페이스로 추상화해야합니다.

![](https://user-images.githubusercontent.com/43809168/99547954-7b792880-29fb-11eb-9cec-e8974038d28b.png)