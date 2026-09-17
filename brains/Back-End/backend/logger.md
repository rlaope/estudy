# 개발자라면 꼭 해야하는 Log 관리, Slf4j Logger

### 로깅 간단히 알아보기
운영 시스템에서는 `System.out.println()` 같은 시스템 콘솔을 사용해서 피요한 정보를 출력하지 않고 별도의 로깅 라이브러리를 사용해 로그를 출력한다.  
  
> 참고로 로그 관련 라이브러리도 많고, 깊게 들어가면 끝이 없기 때문에, 여기서는 최소한의 사용방법만 알아본다

### 로깅 라이브러리
스프링 부트 라이브러리를 사용하면 **스프링 부트 라이브러리 (spring-boot-starter-logging)**가 함께 포함된다.  
스프링 부트 로깅 라이브러리는 기본으로 다음 로깅 라이브러리를 사용한다.

- [SLF4J](http://www.slf4j.org)
- [Logback](http://logback.qos.ch)

- 로그 라이브러리는 Logback, Log4J, Log4J2 등 수많은 라이브러리가 있는데, 그걸 통합하여 인터페이스를 제공하는 것이 바로 `SLF4J` 이다.
- 쉽게 이야기해서 SLF4J는 인터페이스고 그 구현체로 Logback 같은 로그 라이브러리를 선택하면 된다.
- 실무에서는 스프링 부트가 기본으로 제공하는 Logback을 대부분 사용한다.


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

#### 매핑 정보
- `@RestController`
  - `@Controller`는 반환 값이 String이면 뷰 이름으로 인식된다. 그래서 **뷰를 찾고 뷰가 랜더링 된다.*
  - `@RestController`는 반환 값으로 뷰를 찾는 것이 아니라. HTTP 메시지 바디에 바로 입력한다. 따라서 실행 결과로 ok 메시지를 받을 수 있다.

#### 테스트
- 로그가 출력되는 포멧 확인
  - 시간, 로그, 레벨, 프로세스 ID, 쓰레드 명, 클래스 명, 로그 메시지
- 로그 레벨 설정을 변경해서 출력 결과를 보자
  - Level : `TRACE > DEBUG > INFO > WARN > ERROR`
  - 개발 서버는 DEBUG 출력
  - 운영서버는 INFO 출력

<br>

### 로그 레벨 설정
```yml
#전체 로그 레벨 설정(기본 info)
logging.level.root=info

#hello.springmvc 패키지와 그 하위 로그 레벨 설정
logging.level.hello.springmvc=debug
```

### 올바른 로그 사용법
- `log.debug("data"+data)`
  - 로그 출력 레벨을 info로 설정해도 해당 코드에 있는 "data = " = data 가 실제로 실행되어 버린다.
  - 결과적으로 문자 더하기 연산 발생
- log.debug("data={}",data)
  - 로그 출력 레벨을 info로 설정하면 아무일도 발생하지 않는다. 따라서 앞과 같은 의미없는 연산이 발생하지 않는다.

### 로그 사용시 장점
- 쓰레드 정보, 클래스 이름 같은 부가 정보를 함께 볼 수 있고, 출력 모양을 조정할 수 있다.
- 로그 레벨에 따라 개발 서버에서는 모든 로그를 출력하고, 운영 서버에서는 출력하지 않는 등 로그를 상황에 맞게 조절가능하다.
- 시스템 아웃 콘솔에만 출력하는 것이 아니라, 파일이나 네트워크 등, 로그를 별도의 위치에 남길 수 있다. 특히 파일로 남길 때는 일별, 특정 용량에 따라 로그를 분할하는 것도 가능하다.
- 성능도 일반 System.out 보다 좋다(내부 버퍼링, 멀티 쓰레드 등등) 그래서 실무에서는 로그를 꼭 사용해야한다.

#### 더 공부하실 분
- 로그에 대해서 더 자세한 내용은 slf4j, logback을 검색해보자.  
  - SLF4J - http://www.slf4j.org
  - Logback - http://logback.qos.ch
- 스프링 부트가 제공하는 로그 기능은 다음을 참고하자.
  - https://docs.spring.io/spring-boot/docs/current/reference/html/spring-bootfeatures.html#boot-features-logging


