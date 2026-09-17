# Spring Scheduler를 사용해 일정 주기로 동작 수행

## Spring Scheduler
`@Scheduled`를 사용해서 일정한 시간 간격으로 특정 코드가 실행하게한다.

### Dependency

Spring boot starter에 기본적으로 의존 `org.springframework.scheduling`

### Enable Scheduling

Project Application Class에 `@EnabledScheduling` 추가

```java
@EnableScheduling 
@SpringBootApplication
public class SchedulerApplication {
    public static void main(String[] args) {
        SpringApplication.run(SchedulerApplication.class, args);
    }
}
```

scheduler를 사용할 Class에 `@Component`, Method에 `@Scheduled`추가 

- @Scheduled 규칙
  - Method는 void 타입으로
  - Method는 매개변수 사용 불가

### fixedDelay
해당 메서드가 끝나는 시간 기준, milliseconds 간격으로 실행
  
하나의 인스턴스만 항상 실행되도록 해야 할 상황에서 유용

```java
@Scheduled(fixedDelay = 1000)
// @Scheduled(fixedDelayString = "${fixedDelay.in.milliseconds}") // 문자열 milliseconds 사용 시
public void scheduleFixedDelayTask() throws InterruptedException {
    log.info("Fixed delay task - {}", System.currentTimeMillis() / 1000);
    Thread.sleep(5000);
}
```

### fixedRate

해당 메서드가 시작하는 시간 기준, milliseconds 간격으로 실행
  
병렬로 Scheduler를 사용할 경우, Class `@EnalbeAsync`, Method에 `@Async` 추가
  
모든 실행이 독립적인 경우에 유용

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

`initialDelay`값 이후 처음 실행 되고, `fixedDelay` 값에 따라 계속 실행

```java
@Scheduled(fixedDelay = 1000, initialDelay = 5000)
public void scheduleFixedRateWithInitialDelayTask() {
    long now = System.currentTimeMillis() / 1000;
    log.info("Fixed rate task with one second initial delay - {}", now);
}
```

### Cron
작업 예약으로 실행

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