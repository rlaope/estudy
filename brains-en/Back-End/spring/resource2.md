# ResourceLoader, ResourcePatternResolver, ApplicationContext and Resources Paths

## ResourceLoader
This is a technology used to access Resources within a Spring project.
-> It uses Resource implementations.

It is fundamentally implemented within the `ApplicationContext`.

It is utilized when there's a need to access files within the project (primarily files under the classpath).

Most predefined files are loaded automatically. However, it's used when additional files are needed.

```java
@Service
public class ResourceService {
    @Autowired //Automatically receives dependency. Injects itself.
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
An interface used to load `ResourceLoader` from the Spring `ApplicationContext`.

It automatically selects the `ResourceLoader` implementation based on location specifier patterns (`classpath:`, `file:`, `http:`).

```java
public interface ApplicationContext extends EnvironmentCapable, ListableBeanFactory, MierachicalBeanFactory, MessageSource, ApplicationEvenPublisher, ResourcePatternResolver {
    //Spring ApplicationContext interface
}
```

> `ResourcePatternResolver` is extended by `ApplicationContext`.


## ApplicationContext and Resource Paths

Methods for retrieving configuration values that make up the `ApplicationContext` (Spring's core configuration).

```java
//create applicationContext
ApplicationContext ctx = new ClassPathXmlApplicationContext("conf/appContext.xml");

ApplicationContext ctx = new FileSystemXmlApplicationContext("conf/appContext.xml");

ApplicationContext ctx = new FileSystemXmlApplicationContext("classpath:conf/appContext.xml");

//then use
Bear bear = (Bear) ctx.getBean("bear");
```

> Using XML is all in the past. Nowadays, configuration values are set based on annotations.
