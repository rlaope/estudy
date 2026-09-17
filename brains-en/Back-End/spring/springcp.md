# Connection Pool in Spring, HikariCP

In Java, connection pools are typically managed using the DataSource interface.

Spring provides automated techniques that eliminate the need for users to manage connections directly.

Before Spring Boot 2.0, tomcat-jdbc was used, but since 2.0, **Hikari CP** has been adopted as the default option.

## Hikari CP
![](https://user-images.githubusercontent.com/81006587/230904793-ca2415c1-8dc6-425e-9fab-5e8975c7e591.png)
![HikariCP](https://github.com/brettwooldridge/HikariCP-benchmark) Looking at the benchmarking page, it's clear that its performance is significantly better than other connection pool management frameworks. The reason HikariCP shows such fast performance lies in its connection pool management method.

Hikari manages Connections as PoolEntry objects, which wrap Connection objects, and uses a data structure called ConcurrentBag to manage them.

ConcurrentBag is designed to return an available (idle) Connection via the method HikariPool.getConnection() -> ConcurrentBag.borrow().

During this process, it stores information about the thread that requested the connection creation and uses this stored information for quick returns on subsequent accesses.

## Spring Configuration
In Spring, you can adjust Hikari CP's configuration values using a yml file.

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

The meaning of each setting is:

options
- maximum-pool-size: maximum pool size (default 10)
- connection-timeout: time limit
- connection-init-sql: SELECT 1
- validation-timeout: 2000
- minimum-idle: the minimum number of idle connections HikariCP maintains in the connection pool
- idle-timeout: maximum idle time for a connection
- max-lifetime: maximum lifetime (ms) of a connection in the pool after being closed
- auto-commit: whether to auto-commit (default: true)
