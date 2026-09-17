# Spring AOP Library AspectJ

## AspectJ
A library essential for properly using AOP.

The AOP techniques (such as pointcut) supported by default Spring AOP are limited.

> Pointcut: One of the core concepts of AOP, it is a technique for intercepting the execution of a specific method or function. (It's about filtering)

### Aspect
Creating an Aspect
```java
import org.aspectj.lang.annotation.Aspcet;

@Aspect
@Component // For registering and using this Aspect as a Spring Bean.
public class UsefulAspect {
}
```

### Pointcut
Declare a Pointcut as follows.

```java
public class UsefulAspect {
	@Pointcut("execution(* transfer(..))") // Pointcut expression
	private void anyOldTransfer() {}
}
```
> Creates a pattern or condition to find the Join point where the Advice of that Aspect will be applied.

Pointcuts can be combined as follows.

```java
public class UsefulAspect {
	@Pointcut("execution(public * *(..)") // Pointcut targeting public methods
	private void anyPublicOperation() {}

	@Pointcut("within(com.xyz.myapp.tranding..*)") // Pointcut targeting a specific package
	private void inTranding() {}

	@Pointcut("anyPublicOperation() && inTranding()") // Pointcut combined with an AND (&&) condition
	private void trandingOperation() {}
}
```

### Advice
You can also execute a method right before a Pointcut, as follows.

```java
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.annotation.Before;

@Aspect
public class BeforeExample {
	@Before("com.xyz.myapp.CommonPointcuts.dataAccesOperation()")
	public void doAccessCheck() {
	}
}
```

You can execute a method after a return occurs from a predefined Pointcut, as follows.

```java
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.annotation.AfterReturning;

@Aspect
public class AfterReturningExample {
	@AfterReturning("com.xyz.myapp.CommonPointcuts.dataAccesOperation()")
	public void doAccessCheck() {
	}
}
```

You can perform necessary operations before/after a Pointcut, as follows.

```java
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.ProceedingJoinPoint;

@Aspect
public class AroundExample {
	@Around("com.xyz.myapp.CommonPointcuts.businessService()")
	public Object doBasicProfiling(ProceedingJoinPint pjp) throws Throwable {
		//start stopwatch
		Object retVal = pjp.proceed();
		//stop stopwatch
		return retVal
	}
}
```
