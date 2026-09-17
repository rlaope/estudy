# Application Logic, Infrastructure Beans, Container Infrastructure Beans

## Types of Beans

In Spring, beans are further categorized into three types.

- Application Logic Beans
- Application Infrastructure Beans
- Container Infrastructure Beans

### Application Logic Beans
These are beans created and destroyed as the application runs. In a typical backend application, components like services and controllers fulfill this role.

### Application Infrastructure Beans
These beans are also aligned with the application's lifecycle, but the difference is that they are created externally. An example is DataSource.

Application infrastructure beans help application logic beans perform their tasks.

### Container Infrastructure Beans
Container infrastructure beans refer to beans used to extend the functionality of the Spring container. Examples include classes like DefaultAdvisorAutoProxyCreator.

These classes provide extended functionalities, such as scanning container beans through features like bean post-processing.

For example, let's say you've defined beans using the `@Bean` annotation within a class annotated with `@Configuration`. It might seem like the `@Configuration` annotation itself performs the function of registering these beans in the container, but that's not the case. If you simply register this Configuration class as a bean, the internal beans within the class will not be registered in the container.

However, Spring's container infrastructure bean, the `ConfigurationClassPostProcessor` class, performs this task. (In XML, you could also group them with a dedicated tag like `<annotation-config>`.)

Similarly, for classes with `@Autowired`, container infrastructure beans like `AutowiredAnnotationBeanPostProcessor` handle them, and for `@PostConstruct`, container infrastructure beans like `CommonAnnotationBeanPostProcessor` handle them.
