# ResourceLoader, ResourcePatternResolver, ApplicationContext and Resources Paths

## ResourceLoader
스프링 프로젝트 내 Resource에 접근할 때 사용하는 기술이다.  
-> Resource 구현체를 사용한다.
  
기본적으로 applicationContext에 구현되어 있다.  
  
프로젝트내 파일(주로 classpath 하위 파일)에 접근할 일이 있을 경우 활용한다.
  
대부분의 사전정의된 파일들은 자동으로 로딩된다. 그러나, 추가로 필요한 파일이 있을 때 활용한다.

```java
@Service
public class ResourceService {
    @Autowired //자동으로 dependency를 받음. 자기 자신을 injection(삽입)함.
    ApplicationContext ctx;

    public void setResource() {
        Resource myTemplate = ctx.getResource("classpath:some/resource/path/myTemplate.txt");
            //ctx.getResource("file:/some/resource/path/myTemplae.txt);
            //ctx.getResource("http://myhost.com/resource/path/myTemplate.txt);
        
        //use myTemplate
    }
}
```

## ResourcePatternResolver
Spring ApplicationContext에서 ResourceLoader를 불러올 때 사용하는 인터페이스.  
  
위치 지정자 패턴(classpath:, file: http:)dp 따라 자동으로 ResourceLoader 구현체를 선택한다.

```java
public interface ApplicationContext extends EnvironmentCapable, ListableBeanFactory, MierachicalBeanFactory, MessageSource, ApplicationEvenPublisher, ResourcePatternResolver {
    //Spring ApplicationContext interface
}
```

> ApplicationContext에 ResourcePatternResolver가 extends 되어 있음.


## ApplicationContext and Resource Paths

applicationContext(스프링의 핵심 설정)을 이루는 설정값을 가져오는 방법들

```java
//create applicationContext
ApplicationContext ctx = new ClassPathXmlApplicationContext("conf/appContext.xml");

ApplicationContext ctx = new FileSystemXmlApplicationContext("conf/appContext.xml");

ApplicationContext ctx = new FileSystemXmlApplicationContext("classpath:conf/appContext.xml");

//then use
Bear bear = (Bear) ctx.getBean("bear");
```

> XML 사용하는 건 다 과거. 요즘은 그냥 어노테이션 기반으로 설정값을 잡는다.