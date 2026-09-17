# Spring boot 프로젝트 profile 설정하기

Spring Profiles는 애플리케이션 설정을 특정 환경에서만 적용되게 하거나, 환경 별(local, test, production 등)로 다르게 적용할 때 사용된다.
  
properties를 사용하면, 환경별로 각각 다른 파일을 만들어 설정한다.
  
`application-dev.properties`라는 파일을 만들고, 이 파일은 개발 환경에서만 사용할 설정값을 작성한다.

```properties
# application-dev.properties

spring.datasource.url=mysql://[개발환경IP]:3306/[개발DB]
spring.datasource.username=[DB접속 USER NAME]
spring.datasource.password=[DB접속 PASSWORD]
```

그리고 `application-production.properties`라는 파일을 따로 작성해 실제 운영서버에 맞는 값을 작성한다.  
  
이렇게 properties를 사용하면 환경별로 각각의 파일을 생성해주어야 한다.

```properties
# application-production.properties

spring.datasource.url=mysql://[실제운영서버IP]:3306/[실제DB]
spring.datasource.username=[DB접속 USER NAME]
spring.datasource.password=[DB접속 PASSWORD]
```

## YAML을 사용하면 한 파일로 작성이 가능하다
YAML 형식의 장점은 ---을 이용해 파일을 분할할 수 있다는 것. 한 파일로 여러 프로필 설정이 가능하다.

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

## 활설 프로필 설정

`spring-profiles-active`를 이용해 기본 활성 프로필을 설정할 수 있고, 아래 예시대로 애플리케이션을 실행할 때 -D 옵션으로 활성 프로필을 설정해 실행이 가능하다.
  
`spring-profiles-active`로 활성 프로필 설정

```yml
spring:
  profiles:
    active: local # local profile로 실행된다
```

-D 옵션으로 설정

```
$  java -jar -Dspring.profiles.active=production demo-0.0.1-SNAPSHOT.jar
```

++ 아니면 OS 환경변수에 SPRING_PROFILES-ACTIVE 설정을 할 수도 있다.
  
++ 또다른 방법으로는 SpringApplication.setAdditionalProfiles(...)를 애플리케이션이 실행되기 전에 호출하여 활성 프로필을 설정하는 것도 가능하다.

## include

`spring.profiles.include` 속성을 이용하면 해당 프로필을 포함해서 실행할 수 있다. 아래처럼 설정하면 prod 프로필로 실행할 때 proddb와 prodmq 프로필도 활성화되어 실행된다. (proddb, prodmq에 설정된 값을 포함해 실행하게 됨)

```yml
spring:
    profiles : prod
    profiles.include :
      - proddb
      - prodmq
```