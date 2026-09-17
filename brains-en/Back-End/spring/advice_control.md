# How to Control Spring AOP Advice Order

Let's assume we've developed and applied advice for measuring time when a service is called and for starting, committing, and rolling back transactions via Spring AOP.

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

When applying a pointcut to business logic and executing the service, the logs would appear as follows:

```
[log] void example.aop.order.OrderService.orderItem(String)
[Transaction start log] void example.aop.order.OrderService.orderItem(String)
[orderService execution]
[Transaction commit log] void example.aop.order.OrderService.orderItem(String)
[Resource release log] void example.aop.order.OrderService.orderItem(String)
```

To log accurately, we want to output the log after the transaction starts. To achieve this, we need to control the order of the advice so that the log executes after the transaction.

How can this be implemented?

One might think of implementing this using the `@Order` annotation.

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

However, even with this, the order remains the same.

```
[log] void example.aop.order.OrderService.orderItem(String)
[Transaction start log] void example.aop.order.OrderService.orderItem(String)
[orderService execution]
[Transaction commit log] void example.aop.order.OrderService.orderItem(String)
[Resource release log] void example.aop.order.OrderService.orderItem(String)
```

The reason is that `@Order` cannot be applied at the method level; it can only be applied at the class level. Therefore, attaching `@Order` to a method had no effect.

Because of this, we can, albeit inconveniently, separate the advice into different classes.

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

With this, you can confirm that the order has changed correctly. (Conversely, when AOP executes, it operates in reverse order according to the specified sequence.)

```
[log] void example.aop.order.OrderService.orderItem(String)
[Transaction start log] void example.aop.order.OrderService.orderItem(String)
[orderService execution]
[Transaction commit log] void example.aop.order.OrderService.orderItem(String)
[Resource release log] void example.aop.order.OrderService.orderItem(String)
```
