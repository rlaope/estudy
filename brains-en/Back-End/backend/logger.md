# Log Management Every Developer Must Do: Slf4j Logger

### A Quick Look at Logging
In production systems, instead of using system consoles like `System.out.println()` to output necessary information, logs are output using a separate logging library.

> Note that there are many logging-related libraries, and going deep into them is endless, so here we will only cover the minimum usage.

### Logging Libraries
When using Spring Boot libraries, the **Spring Boot Library (spring-boot-starter-logging)** is included.
The Spring Boot logging library uses the following logging libraries by default:

- [SLF4J](http://www.slf4j.org)
- [Logback](http://logback.qos.ch)

- There are numerous logging libraries such as Logback, Log4J, and Log4J2, and `SLF4J` provides an interface to integrate them.
- Simply put, SLF4J is an interface, and you can choose a logging library like Logback as its implementation.
- In practice, most people use Logback, which Spring Boot provides by default.

```java
package hello.springmvc.basic;

import lombok.extern.slf4j.Slf4j;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

//@Slf4j 롬복을 사용한 로그 선언
@RestController
public class LogTestController {

     private final Logger log = LoggerFactory. getLogger(getClass()); // 로그 선언

     @RequestMapping("/log-test")
     public String logTest() {
       String name = "Spring";
 
       log.trace("trace log={}", name);
       log.debug("debug log={}", name);
       log.info(" info log={}", name);
       log.warn(" warn log={}", name);
       log.error("error log={}", name);

        //로그를 사용하지 않아도 a+b 계산 로직이 먼저 실행됨, 이런 방식으로 사용하면 X

       log.debug("String concat log=" + name);
       return "ok";
    }
}
```

#### Mapping Information
- `@RestController`
  - If the return value of `@Controller` is a String, it is recognized as a view name. Thus, **the view is found and rendered.**
  - `@RestController` does not look for a view as a return value. Instead, it directly writes to the HTTP message body. Therefore, you can receive an "ok" message as the execution result.

#### Testing
- Check the log output format
  - Time, log, level, process ID, thread name, class name, log message
- Let's change the log level settings and see the output.
  - Level : `TRACE > DEBUG > INFO > WARN > ERROR`
  - Development servers output DEBUG
  - Production servers output INFO

<br>

### Log Level Configuration
```yml
#전체 로그 레벨 설정(기본 info)
logging.level.root=info

#hello.springmvc 패키지와 그 하위 로그 레벨 설정
logging.level.hello.springmvc=debug
```

### Correct Log Usage
- `log.debug("data"+data)`
  - Even if the log output level is set to info, "data = " + data in that code will actually be executed.
  - Consequently, string concatenation occurs.
- `log.debug("data={}",data)`
  - If the log output level is set to info, nothing happens. Therefore, meaningless operations like the previous example do not occur.

### Advantages of Using Logs
- You can view additional information such as thread information and class names, and adjust the output format.
- Depending on the log level, you can adjust logs according to the situation, such as outputting all logs on development servers and not outputting them on production servers.
- Logs can be stored in separate locations, such as files or networks, not just output to the system out console. Especially when saving to files, it's possible to split logs daily or by specific size.
- Performance is also better than a regular System.out (due to internal buffering, multi-threading, etc.). Therefore, logs must be used in practice.

#### For Further Study
- For more detailed information about logs, search for slf4j, logback.
  - SLF4J - http://www.slf4j.org
  - Logback - http://logback.qos.ch
- Refer to the following for logging features provided by Spring Boot.
  - https://docs.spring.io/spring-boot/docs/current/reference/html/spring-bootfeatures.html#boot-features-logging
