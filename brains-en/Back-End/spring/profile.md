# Configuring Spring Boot Project Profiles

Spring Profiles are used to apply application configurations only in specific environments, or to apply different configurations per environment (e.g., local, test, production).
  
When using properties, you configure by creating a separate file for each environment.
  
Create a file named `application-dev.properties` and write the configuration values to be used only in the development environment in this file.

```properties
# application-dev.properties

spring.datasource.url=mysql://[개발환경IP]:3306/[개발DB]
spring.datasource.username=[DB접속 USER NAME]
spring.datasource.password=[DB접속 PASSWORD]
```

Then, create a separate file named `application-production.properties` and write the values appropriate for the actual production server.  
  
Thus, when using properties, you need to create a separate file for each environment.

```properties
# application-production.properties

spring.datasource.url=mysql://[실제운영서버IP]:3306/[실제DB]
spring.datasource.username=[DB접속 USER NAME]
spring.datasource.password=[DB접속 PASSWORD]
```

## YAML Allows Configuration in a Single File
The advantage of the YAML format is that you can split the file using ---. This allows configuring multiple profiles within a single file.

```yml
server:
    port: 9000    # 기본 포트 설정
---

spring:
    profiles: development
server:
    port: 9001    # 프로필마다 포트번호 다르게 설정

---

spring:
    profiles: production
server:
    port: 0
```

## Setting Active Profiles

You can set the default active profile using `spring-profiles-active`, and as shown in the example below, you can set and run the active profile using the -D option when executing the application.
  
Setting the active profile with `spring-profiles-active`

```yml
spring:
  profiles:
    active: local # local profile로 실행된다
```

Setting with -D option

```
$  java -jar -Dspring.profiles.active=production demo-0.0.1-SNAPSHOT.jar
```

++ Alternatively, you can set SPRING_PROFILES-ACTIVE in an OS environment variable.
  
++ Another method is to call SpringApplication.setAdditionalProfiles(...) before the application starts to set the active profiles.

## Include

Using the `spring.profiles.include` property, you can include specific profiles when running. As configured below, when the `prod` profile is run, the `proddb` and `prodmq` profiles will also be activated. (This means the values configured in `proddb` and `prodmq` will be included.)

```yml
spring:
    profiles : prod
    profiles.include :
      - proddb
      - prodmq
```
