# Spring AOP Advice 순서 제어 방법

Spring AOP를 통해 예시로 서비스가 호출되었을때 시간을 측정하는 로그와 트랜잭션을 시작후 커밋, 롤백하는 advice들을 개발하고 적용했다고 가정해보자.

```java
@Slf4j
@Aspect
public class AspectSomething {
	@Around("Pointcuts.allOrder()")
	public Object doLog(ProceedingJoinPoint joinPoint) throws Throwable {
	  log.info("log {}", joinPoint.getSignature());
	  return joinPoint.proceed();
	}

	@Around("Pointcuts.allOrderAndAllDo()")
	public Object doTransaction(ProceedingJoinPoint joinPoint) throws Throwable {
	  // tx process
	  return joinPoint.proceed();
	}
}

```

여기서 비즈니스 로직에 대한 pointcut을 적용하여 서비스를 실행한다고 했을때, 다음과 같이 로그가 찍히게 된다.

```
[log] void example.aop.order.OrderService.orderItem(String)
[트랜잭션 시작 관련 로그] void example.aop.order.OrderService.orderItem(String)
[orderService 실행]
[트랜잭션 커밋 관련 로그] void example.aop.order.OrderService.orderItem(String)
[리소스 릴리즈 관련 로그] void example.aop.order.OrderService.orderItem(String)
```

여기서 log를 정확하게 찍기 위해서 트랜잭션 시작 이후에 로그를 출력하고 싶은데, 그렇게하기 위해서는 adivce들의 순서를 제어하여 log가 트랜잭션 이후에 실행하게 구현해야한다.

이를 어떻게 구현할 수 있을까?

`@Order` 어노테이션을 사용하여 구현해볼 수 있다고 생각할 수 있다.

```java
@Slf4j
@Aspect
public class AspectSomething {
    @Order(2)
    @Around("Pointcuts.allOrder()")
    public Object doLog(ProceedingJoinPoint joinPoint) throws Throwable {
      log.info("log {}", joinPoint.getSignature());
      return joinPoint.proceed();
    }
	
    @Order(1)
    @Around("Pointcuts.allOrderAndAllDo()")
    public Object doTransaction(ProceedingJoinPoint joinPoint) throws Throwable {
      // tx process
      return joinPoint.proceed();
    }
}
```

그러나 이렇게 해도 순서는 그대로인 것을 확인할 수 있다.

```
[log] void example.aop.order.OrderService.orderItem(String)
[트랜잭션 시작 관련 로그] void example.aop.order.OrderService.orderItem(String)
[orderService 실행]
[트랜잭션 커밋 관련 로그] void example.aop.order.OrderService.orderItem(String)
[리소스 릴리즈 관련 로그] void example.aop.order.OrderService.orderItem(String)
```

이유는 Order 단위를 메서드로 할 수 없고 class 단위로 적용할 수 있다. 그렇기 때문에 order를 메서드위에 붙여줘도 아무런 효과가 없었던 것이다.

그렇기 때문에 우리는 advice를 불편하겠지만 클래스 단위로 분리하여 사용해 볼 수 있다.

```java
@Slf4j
public class AspectOrder {

  @Aspect
  static static class LogAspect {
    @Order(2)
    @Around("Pointcuts.allOrder()")
    public Object doLog(ProceedingJoinPoint joinPoint) throws Throwable {
      log.info("log {}", joinPoint.getSignature());
      return joinPoint.proceed();
    }
  }

  @Aspect
  static static class TxAspect {
    @Order(2)
    @Around("Pointcuts.allOrderAndAllDo()")
    public Object doTransaction(ProceedingJoinPoint joinPoint) throws Throwable {
      // tx process
      return joinPoint.proceed();
    }
  }
}

```

이렇게 되면 정상적으로 순서가 바뀐 것을 확인할 수 있다. (역으로 aop를 실행할때도 지정된 순서에 따른 역순으로 동작하게 된다.)

```
[log] void example.aop.order.OrderService.orderItem(String)
[트랜잭션 시작 관련 로그] void example.aop.order.OrderService.orderItem(String)
[orderService 실행]
[트랜잭션 커밋 관련 로그] void example.aop.order.OrderService.orderItem(String)
[리소스 릴리즈 관련 로그] void example.aop.order.OrderService.orderItem(String)
```