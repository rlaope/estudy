# Spring boot 내장 웹 서버 톰캣 설정, 다른 내장 웹 서버 설정하는 방법

## 스프링 부트 내장 웹 서버 설정

스프링 부트 프로젝트를 생성할 시 스프링부트에서는 내장 서블릿 컨테이너인 톰캣(tomcat)이 자동으로 설정됩니다.
  
스프링부트에서는 `ServletWebFactoryAutoConfiguration` 클래스에서 자동적으로 `Tomcat`, `Jetty` 같은 내장 웹서버에 대한 설정을 자동적으로 처리하며 스프링부트 사용자가 쉽게 웹 서버 관련 설정을 하지 않아도 웹 서버 프로젝트를 안에 포함되게 합니다.

```java
@Configuration
@AutoConfigureOrder(Ordered.HIGHEST_PRECEDENCE)
@ConditionalOnClass(ServletRequest.class)
@ConditionalOnWebApplication(type = Type.SERVLET)
@EnableConfigurationProperties(ServerProperties.class)
@Import({ ServletWebServerFactoryAutoConfiguration.BeanPostProcessorsRegistrar.class,
      ServletWebServerFactoryConfiguration.EmbeddedTomcat.class,
      ServletWebServerFactoryConfiguration.EmbeddedJetty.class,
      ServletWebServerFactoryConfiguration.EmbeddedUndertow.class })
public class ServletWebServerFactoryAutoConfiguration {

   @Bean
   public ServletWebServerFactoryCustomizer servletWebServerFactoryCustomizer(
         ServerProperties serverProperties) {
      return new ServletWebServerFactoryCustomizer(serverProperties);
   }

   @Bean
   @ConditionalOnClass(name = "org.apache.catalina.startup.Tomcat")
   public TomcatServletWebServerFactoryCustomizer tomcatServletWebServerFactoryCustomizer(
         ServerProperties serverProperties) {
      return new TomcatServletWebServerFactoryCustomizer(serverProperties);
   }
```

위 `ServletWebServerFactoryAutoConfiguration`클래스 코드를 보면 `TomcatServletWebServerFactoryCustomizer` 클래스를 통해 자동적으로 톰캣을 커스터마이징한다는 것을 알 수 있습니다.
  
또한 기존 스프링 프로젝트에서 web.xml에서 설정해야 했던 `DispatcherServlet` 관련 설정 또한 아래 `DispatcherServletAutoConfiguration` 클래스를 통해 처리하고 있습니다.

```java
@AutoConfigureOrder(Ordered.HIGHEST_PRECEDENCE)
@Configuration
@ConditionalOnWebApplication(type = Type.SERVLET)
@ConditionalOnClass(DispatcherServlet.class)
@AutoConfigureAfter(ServletWebServerFactoryAutoConfiguration.class)
public class DispatcherServletAutoConfiguration {
```

## 스프링부트에서 톰캣이 아닌 다른 내장 웹 서버를 쓰고자 할 때 설정 방법

스프링 부트 프로젝트를 생성하면 기본적으로 내장 웹 서버 톰캣을 사용하게 되어 있습니다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99BA394C5C1F381432)

만약 톰캣 말고 Jetty 같은 다른 내장 웹 서버를 사용하기 위해서는 아래와 같이 pom.xml의 설정값을 바꿔주어야합니다.

```xml
<dependencies>
    <dependency>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-web</artifactId>
        <exclusions>
            <exclusion>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-starter-tomcat</artifactId>
            </exclusion>
        </exclusions>
    </dependency>

    <dependency>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-jetty</artifactId>
    </dependency>

    <dependency>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-test</artifactId>
        <scope>test</scope>
    </dependency>
</dependencies>
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F995F26475C1F387E34)

## 스프링부트 내장 웹 서버 설정 방법

스프링부트에서 톰캣과 같은 내장 웹 서버를 설정하는 방법 중 하나로  
application.properties 파일에 설정값을 입력하는 방법이 있습니다.
```properties
server.port=7070
server.compression.enabled=true
```

위 설정파일을 가지고 스프링부트를 실행하면 7070 포트로 웹 어플리케이션 서버가 실행되는 것을 알 수 있습니다.
  
또한 `server.compression.enabled=true`를 입력할 시, css, html 등 압축해서 전송할 때 이득이 되는 포맷들을 자동으로 압축해서 보내는 것으로 설정됩니다.
  
아래는 위 설정 파일로 실제 포트가 지정되었는지 확인하는 코드입니다.


```java
@Component
public class PortListener implements ApplicationListener<ServletWebServerInitializedEvent> {
    @Override
    public void onApplicationEvent(ServletWebServerInitializedEvent event) {
        ServletWebServerApplicationContext applicationContext = event.getApplicationContext();
        System.out.println(applicationContext.getWebServer().getPort());
    }
}

```

```
2018-12-23 16:30:02.252  INFO 10396 --- [           main] o.s.web.servlet.DispatcherServlet        : Completed initialization in 7 ms
2018-12-23 16:30:02.274  INFO 10396 --- [           main] o.e.jetty.server.AbstractConnector       : Started ServerConnector@1450078a{HTTP/1.1,[http/1.1]}{0.0.0.0:7070}
2018-12-23 16:30:02.276  INFO 10396 --- [           main] o.s.b.web.embedded.jetty.JettyWebServer  : Jetty started on port(s) 7070 (http/1.1) with context path ''
7070
...
```