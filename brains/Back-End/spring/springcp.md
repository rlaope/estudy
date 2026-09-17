# Spring 에서의 Connection Pool, HikariCP

자바에서는 기본적으로 DataSource 인터페이스를 사용해 커넥션풀을 관리한다.  
  
Spring에는 사용자가 직접 커넥션을 관리할 필요없이 자동화된 기법들을 제공하는데  
  
Springboot 2.0 이전에는 tomcat-jdbc를 사용하다, 2.0 이후 부터는 **Hikari CP**를 기본옵션으로 채택하고 있다.

## Hikari CP
![](https://user-images.githubusercontent.com/81006587/230904793-ca2415c1-8dc6-425e-9fab-5e8975c7e591.png)
![HikariCP](https://github.com/brettwooldridge/HikariCP-benchmark) 벤치마킹 페이지를 보면 다른 커넥션풀 관리 프레임워크들 보다 성능이 월등히 좋음을 알 수 있다. HikariCP가 빠른 성능을 보여주는 이유는 커넥션풀의 관리 방법에 있다.  
  
히카리는 Connection 객체를 한번에 래핑한 PoolEntry로 Connection을 관리하며, 이를 관리하는 ConcurrentBag이라는 구조체를 사용하고 있다.  
  
ConcurrentBag은 HikariPool.getConnection() -> ConcurrentBag.borrow() 라는 메서드를 통해 사용 가능한 (idle) Connection을 리턴하도록 되어있다.  
  
이 과정에서 커넥션 생성을 요청한 스레드의 정보를 저장해두고 다음에 접근시 저장된 정보를 이용해 빠르게 반환을 해준다.

## Spring 설정
스프링에서는 yml 파일로 hikari CP의 설정 값을 조정해줄 수 있다.

```yml
spring:
 datasource:
   url: jdbc:mysql://localhost:3306/world?serverTimeZone=UTC&CharacterEncoding=UTF-8
   username: root
   password: your_password
   hikari:
     maximum-pool-size: 10
     connection-timeout: 5000
     connection-init-sql: SELECT 1
     validation-timeout: 2000
     minimum-idle: 10
     idle-timeout: 600000
     max-lifetime: 1800000

server:
 port: 8000
```

각 설정의 의미는

options
- maximum-pool-size: 최대 pool size (default 10)
- connection-timeout: 시간 제한
- connection-init-sql: SELECT 1
- validation-timeout: 2000
- minimum-idle: 연결 풀에서 HikariCP가 유지 관리하는 최소 유휴 연결 수
- idle-timeout: 연결을 위한 최대 유휴 시간
- max-lifetime: 닫힌 후 pool에 있는 connection의 최대 수명(ms)
- auto-commit: auth commit 여부 (default: true)