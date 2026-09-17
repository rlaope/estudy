# [Spring] Let's fully understand IoC, DI, Spring Container, and Bean 📗

![](image/iocdi_0.png)

### Overview

As a developer using the Spring Framework, I believe IoC, DI, Container, and Bean are four concepts you absolutely must understand.

Therefore, let's master them completely in this session.

### IoC (Inversion of Control)

**IoC stands for Inversion of Control, which translates to "inversion of control."**

**Literally, it means that the invocation of methods or objects is determined externally, rather than by the developer.**

**Advantages**

It reduces coupling between objects and allows for more flexible code by inverting object dependencies.

It improves readability, reduces code duplication, and makes maintenance easier.

Control over typical dependencies: The developer directly creates dependencies.

-> *Dependencies, simply put, are objects that another object needs to use. If you create them directly via constructors,*

*you could say you're directly creating and using your own dependencies.*

It refers to cases where dependencies are not created directly but are brought in from an external source.

Using this approach, developers build the necessary parts as if assembling them.

The final invocation of the assembled code is not controlled by the developer but occurs as determined internally by the framework.

This phenomenon is referred to as **Inversion of Control**.

![](image/iocdi_1.png)

### DI (Dependency Injection)

**DI is an abbreviation for Dependency Injection, which translates to "dependency injection."**

Literally, it's a method where objects are not created directly but are created externally and then injected.

*Dependency injection is a technique used to manage relationships between objects within Spring when inversion of control occurs.*

*Java typically uses interfaces to handle relationships between dependent objects as flexibly as possible.*

Dependency injection means that instead of directly creating or controlling dependent objects, the objects required by a specific object are

**determined externally and then connected**.

In other words, we just need to use interfaces that abstractly group class functionalities.

The rest is handled by Spring, which injects the objects (e.g., creating a Service interface and a ServiceImpl implementation, then declaring and using it as a Service).

Therefore, through such dependency injection, **coupling between modules is reduced, and flexibility is increased.**

### Spring Container

The Spring Container **manages the lifecycle of Java objects and provides additional functionalities to the created Java objects.**

The Java objects referred to here are called **Beans** in Spring. And the principles of **IoC** and **DI** are applied here.

**There are two types of Spring Containers: BeanFactory and ApplicationContext.**

**BeanFactory**

It is the most basic IoC container and class responsible for creating beans and configuring their dependencies.

BeanFactory plays a role in managing beans, including registering, creating, and retrieving them. Beans can be instantiated via the getBean() method.

**ApplicationContext**

ApplicationContext implements BeanFactory, so it can be thought of as an extended version of BeanFactory.

-> When we talk about BeanFactory, the focus is on the basic IoC functionality of creating beans and configuring relationships.

-> ApplicationContext, on the other hand, focuses on overseeing the control of bean creation, relationship configuration, and more, by referring to additional information.

The Spring Container creation process involves an empty Spring Container being created,

Spring Beans being registered (based on Java, XML, etc.), and subsequently, the dependencies of Spring Beans being configured (DI).

![](image/iocdi_2.png)

So, what exactly is a Bean here?

### Bean

**A Java object managed by the Spring IoC container is called a Bean.**

There are two ways to register Spring Beans with the container.

1. Bean registration method using Java annotations.

If the @Component annotation is registered, Spring checks the annotation and registers it as a bean automatically.

2. Direct registration in a Bean Configuration File.

Beans can be registered directly using the @Configuration and @Bean annotations.

#### References

<https://melonicedlatte.com/2021/07/11/232800.html>

[What is a Spring Bean? Concept Summary - Easy is Perfect

melonicedlatte.com](https://melonicedlatte.com/2021/07/11/232800.html)

<https://chanhuiseok.github.io/posts/spring-4/>

[[Spring] Spring's IoC (Inversion of Control) and Bean

Computer/IT/Algorithm Summary Blog

chanhuiseok.github.io](https://chanhuiseok.github.io/posts/spring-4/)

<https://dev-aiden.com/spring/Spring-Container/>

[[Spring] Spring Container

Spring Container

dev-aiden.com](https://dev-aiden.com/spring/Spring-Container/)
