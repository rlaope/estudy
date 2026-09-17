# Spring AOP (Aspect Oriented Programming) Concepts

### Spring AOP

AOP, short for Aspect Oriented Programming, is also known as "관점 지향 프로그래밍" (aspect-oriented programming). Aspect-oriented programming, simply put, means dividing a logic into core concerns and cross-cutting concerns, and then modularizing each based on these concerns. Here, modularization refers to bundling common logic or functionality into a single unit.

For example, the core concern would be the main business logic we intend to apply. Cross-cutting concerns, on the other hand, could include database connections, logging, and file I/O, which are performed to execute the core logic.

In AOP, modularizing logic based on each concern means breaking down and modularizing code into parts. At this point, you might find code that is repeatedly used in different parts of the source code, which is called a cross-cutting concern.

![](./image/aop.png)

The purpose of AOP is to modularize such cross-cutting concerns into Aspects and reuse them by separating them from the core business logic.

### Key AOP Concepts
- Aspect: A module of cross-cutting concerns as described above. Primarily modularizes supplementary functionalities.
- Target: The place where an Aspect is applied (class, method, etc.).
- Advice: What actually needs to be done; the concrete implementation containing the actual supplementary functionality.
- JoinPoint: The point where Advice can be applied, an insertion point. It can be applied at various times, such as method entry, constructor call, or when retrieving a value from a field.
- PointCut: A detailed specification of a JoinPoint. It allows for more specific definition of where Advice should execute, such as 'call when method A is entered'.

### Spring AOP Characteristics
- An AOP implementation based on the Proxy Pattern. Proxy objects are used for access control and adding supplementary functionality.
- AOP can only be applied to Spring Beans.
- Its purpose is not to provide all AOP features, but to solve the most common problems in enterprise applications (duplicate code, hassle of writing proxy classes, increased complexity of relationships between objects, etc.) by integrating with Spring IoC.

<br>

### Spring AOP: @AOP

Add Dependency
```js
implementation 'org.springframework.boot:spring-boot-starter-aop'
```

Next, attach the `@Aspect` annotation to explicitly indicate that this class represents an Aspect, and register it as a Spring Bean by attaching `@Component`.

```java
@Component
@Aspect
public class PerfAspect {

  @Around("execution(* com.saelobi..*.EventService.*(..))")
  public Object logPerf(ProceedingJoinPoint pjp) throws Throwable{
    long begin = System.currentTimeMillis();
    Object retVal = pjp.proceed(); // 메서드 호출 자체를 감쌈
    System.out.println(System.currentTimeMillis() - begin);
    return retVal;
  }  
}
```

The `@Around` annotation means that it wraps the target method to execute a specific Advice. The Advice in the code above implements logic to measure the execution time of the target method. Additionally, `execution...` signifies that this Aspect will be applied to all methods of the `EventService` object within the package path under `com.saelobi`.

```java
public interface EventService {

    void createEvent();

    void publishEvent();

    void deleteEvent();
}
```

```java
@Component
public class SimpleEventService implements EventService {

    @Override
    public void createEvent() {
        try {
            Thread.sleep(1000);
        } catch(InterruptedException e) {
            e.printStackTrace();
        }
        System.out.println("Created an event");
    }

    @Override
    public void publishEvent() {
        try {
            Thread.sleep(1000);
        } catch (InterruptedException e){
            e.printStackTrace();;
        }
        System.out.println("Published an event");
    }

    public void deleteEvent() {
        System.out.println("Delete an event");
    }
}

```

```java
@Service
public class AppRunner implements ApplicationRunner {

    @Autowired
    EventService eventService;

    @Override
    public void run(ApplicationArguments args) throws Exception {
        eventService.createEvent();
        eventService.publishEvent();
        eventService.deleteEvent();
    }
}
```

```
Created an event
1003
Published an event
1000
Delete an event
0
```

Spring also provides a feature to apply an Aspect to a point marked with a specific annotation, rather than using path-based targeting.

```java
@Component
@Aspect
public class PerfAspect {

  @Around("@annotation(PerLogging)")
  public Object logPerf(ProceedingJoinPoint pjp) throws Throwable{
    long begin = System.currentTimeMillis();
    Object retVal = pjp.proceed(); // 메서드 호출 자체를 감쌈
    System.out.println(System.currentTimeMillis() - begin);
    return retVal;
  }
}
```

```java
@Target(ElementType.METHOD)
@Retention(RetentionPolicy.CLASS)
public @interface PerLogging {
}
```

```java
@Component
public class SimpleEventService implements EventService {

    @PerLogging
    @Override
    public void createEvent() {
        try {
            Thread.sleep(1000);
        } catch(InterruptedException e) {
            e.printStackTrace();
        }
        System.out.println("Created an event");
    }

    @Override
    public void publishEvent() {
        try {
            Thread.sleep(1000);
        } catch (InterruptedException e){
            e.printStackTrace();;
        }
        System.out.println("Published an event");
    }

    @PerLogging
    @Override
    public void deleteEvent() {
        System.out.println("Delete an event");
    }
}
```

```java
Created an event
1003
Published an event
Delete an event
0
```

From the output above, you can see that the Aspect was applied only to methods annotated with `@PerLogging`.

Similarly, it also provides a feature to apply to all methods of a Spring Bean.

```java
@Component
@Aspect
public class PerfAspect {

@Around("bean(simpleEventService)")
  public Object logPerf(ProceedingJoinPoint pjp) throws Throwable{
    long begin = System.currentTimeMillis();
    Object retVal = pjp.proceed(); // 메서드 호출 자체를 감쌈
    System.out.println(System.currentTimeMillis() - begin);
    return retVal;
  }
}
```

```java
@Component
public class SimpleEventService implements EventService {

    @Override
    public void createEvent() {
        try {
            Thread.sleep(1000);
        } catch(InterruptedException e) {
            e.printStackTrace();
        }
        System.out.println("Created an event");
    }

    @Override
    public void publishEvent() {
        try {
            Thread.sleep(1000);
        } catch (InterruptedException e){
            e.printStackTrace();;
        }
        System.out.println("Published an event");
    }
    
    @Override
    public void deleteEvent() {
        System.out.println("Delete an event");
    }
}
```

```java
@Service
public class AppRunner implements ApplicationRunner {

    @Autowired
    EventService eventService;

    @Override
    public void run(ApplicationArguments args) throws Exception {
        eventService.createEvent();
        eventService.publishEvent();
        eventService.deleteEvent();
    } }
```

```
Created an event
1002
Published an event
1001
Delete an event
0
```

From the output above, it can be seen that the Aspect has been added to all methods of `SimpleEventService`.

Besides `@Around`, there are other annotations that can specify the execution timing of an Aspect relative to the target method.

- @Before: Executes the advice functionality before the target method is called.
- @After: Executes the advice functionality after the target method completes, regardless of the target method's outcome (success or exception).
- @AfterReturning: Executes the advice functionality after a normal return, or if the target method throws an exception during execution.
- @Around: The advice wraps the target method, executing its functionality both before and after the target method call.
