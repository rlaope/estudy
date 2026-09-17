# Perform actions periodically using Spring Scheduler

## Spring Scheduler
Uses `@Scheduled` to execute specific code at regular intervals.

### Dependency

Relies on `org.springframework.scheduling` by default in Spring Boot starter.

### Enable Scheduling

Add `@EnableScheduling` to the Project Application Class.

```java
@EnableScheduling 
@SpringBootApplication
public class SchedulerApplication {
    public static void main(String[] args) {
        SpringApplication.run(SchedulerApplication.class, args);
    }
}
```

Add `@Component` to the class that will use the scheduler, and `@Scheduled` to the method.

- @Scheduled Rules
  - Method must be of `void` type.
  - Method cannot use parameters.

### fixedDelay
Executes at `milliseconds` intervals, based on when the method finishes.
  
Useful in situations where only one instance should always be running.

```java
@Scheduled(fixedDelay = 1000)
// @Scheduled(fixedDelayString = "${fixedDelay.in.milliseconds}") // 문자열 milliseconds 사용 시
public void scheduleFixedDelayTask() throws InterruptedException {
    log.info("Fixed delay task - {}", System.currentTimeMillis() / 1000);
    Thread.sleep(5000);
}
```

### fixedRate

Executes at `milliseconds` intervals, based on when the method starts.
  
When using Scheduler in parallel, add `@EnableAsync` to the Class and `@Async` to the Method.
  
Useful when all executions are independent.

```java
@Async
@Scheduled(fixedRate = 1000)
// @Scheduled(fixedRateString = "${fixedRate.in.milliseconds}")  // 문자열 milliseconds 사용 시
public void scheduleFixedRateTask() throws InterruptedException {
    log.info("Fixed rate task - {}", System.currentTimeMillis() / 1000);
    Thread.sleep(5000);
}
```

### initialDelay + fixedDelay

First execution occurs after the `initialDelay` value, then continues to execute according to the `fixedDelay` value.

```java
@Scheduled(fixedDelay = 1000, initialDelay = 5000)
public void scheduleFixedRateWithInitialDelayTask() {
    long now = System.currentTimeMillis() / 1000;
    log.info("Fixed rate task with one second initial delay - {}", now);
}
```

### Cron
Executes as a scheduled task.

```java
@Scheduled(cron = "0 15 10 15 * ?") // 매월 15일 오전 10시 15분에 실행
// @Scheduled(cron = "0 15 10 15 11 ?") // 11월 15일 오전 10시 15분에 실행
// @Scheduled(cron = "${cron.expression}")
// @Scheduled(cron = "0 15 10 15 * ?", zone = "Europe/Paris") // timezone 설정
public void scheduleTaskUsingCronExpression() {
    long now = System.currentTimeMillis() / 1000;
    log.info("schedule tasks using cron jobs - {}", now);
}
```
