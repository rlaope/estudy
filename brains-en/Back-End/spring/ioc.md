# IoC Container (Inversion of Control)

### IoC
- IoC translates to `control, inversion`.
- **IoC (Inversion of Control)** means that control over all objects, including their creation and lifecycle management, has been inverted.
- It is a `design pattern` for resolving component dependency relationships, configuration, and lifecycles.

### Container
> Container - A container typically manages the lifecycle of objects and provides additional functionality to created instances.

The Spring Framework also has a container that creates, manages, and is responsible for objects, and handles dependencies. This is the IoC Container (Spring Container).

**The container, not the developer, manages the instance lifecycle from creation to destruction. Since the framework (container) becomes the primary manager of objects, developers can focus on business logic, which is an advantage.**
- The IoC container is responsible for creating objects and managing dependencies.
- It has the authority over the creation, initialization, servicing, and destruction of POJOs.
- Developers can create POJOs directly, but they entrust this to the container.
- Developers can focus on business logic.
- TDD becomes easier as object creation code is eliminated.

> What is a POJO (Plain Old Java Object)?
> It primarily refers to a Java Object that does not follow a specific Java model, feature, or framework.
> Java Bean objects are typical examples. You can simply think of getters / setters.

<br>

### Classification of IoC

#### DL (Dependency Lookup) and DI (Dependency Injection)

- `DL` : Accessing a bean stored in a repository by using an API provided by the container to look up the bean.
- `DI` : The container automatically connects dependencies between classes based on bean configuration information.
  - Setter Injection
  - Constructor Injection
  - Field Injection

**DI is primarily used because DL increases container dependency.**

![DI](image/DI.png)

<br>

### Types of Spring Container (IoC)
Objects managed by the Spring container are called `Beans`, and in the sense that it manages these beans, the container is called a `Bean Factory`.

- When viewing the creation of objects and the runtime relationships between objects from a DI perspective, the container is called a `BeanFactory`.
- There is an **ApplicationContext** which adds various container functionalities to the BeanFactory.

### BeanFactory , ApplicationContext

#### 1. BeanFactory
- Classes that only implement interfaces in the BeanFactory family simply provide the functionality to create objects and handle DI within the container.
- It manages the registration, creation, lookup, and return of Beans.
- Implementing the Factory design pattern, BeanFactory is a class responsible for creating and distributing beans.
- The getBean() method for looking up beans is defined.

#### 2. ApplicationContext
- The functionality for registering, creating, looking up, and returning Beans is the same as BeanFactory.
- It additionally provides various supplementary features of Spring.

**Additional features provided beyond BeanFactory**
  - Manages internationalized text messages.
  - Provides comprehensive ways to load file resources like images.
  - Notifies beans registered as listeners about event occurrences.

![ac](image/ac.png)
> Therefore, in most applications, it is better to use ApplicationContext rather than BeanFactory.
