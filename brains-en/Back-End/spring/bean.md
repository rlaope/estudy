# What is a Spring Bean?
**A Java object managed by the Spring container is called a Bean.**

- Inversion of Control, simply put, means entrusting the creation and control of objects to Spring, rather than the user.
- Until now, objects were created and methods were called using the `new` operator. However, when IoC is applied, the creation of these objects and the user's control are handed over to Spring.
- Users do not directly use objects created with `new`; instead, they use Java objects managed by Spring. These objects are called `Beans`.

<br>

## How to Register Spring Beans in the Spring Container

### 1 Component Scan and Automatic Dependency Injection

- If an `@Component` annotation is present, it is automatically registered as a Spring Bean.
- Annotations that include `@Component`, such as `@Controller`, `@Service`, and `@Repository`, are also automatically registered as Spring Beans.

![spring container](./image/bean.png)

Let's register a member controller and set up its dependencies so that it can use the member service and member repository.

```java
package hello.hellospring.controller;

import hello.hellospring.service.MemberService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;

@Controller
public class MemberController {

    private final MemberService memberService;

	@Autowired
    public MemberController(MemberService memberService) {
        this.memberService = memberService;
    }
}
```

- **@Autowired**
  - If a constructor has `@Autowired`, Spring finds and injects the related object from the Spring container.
  - Injecting object dependencies from an external source in this manner is called `Dependency Injection (DI)`.
  - In the relationship diagram above, it acts as an arrow representing the dependency relationship. (It can be omitted if there is only one constructor)


However, if you run the server in this state, an error will occur.
![springcontainer2](./image/bean2.png)

This is because `memberService` is not registered as a Spring Bean in the Spring container.
The `memberService` must also be registered as a Spring Bean.

```java
@Service
public class MemberService {

	private final MemberRepository memberRepository;

	@Autowired
	public MemberService(MemberRepository memberRepository) {
		this.memberRepository = memberRepository;
	}
}
```

The `memberRepository` must also be registered as a Spring Bean.

```java
@Repository
public class MemoryMemberRepository implements MemberRepository {}
```
![spring container](./image/bean.png)

The `memberService` and `memberRepository` are now registered as Spring Beans in the Spring container.

For reference, when Spring registers Spring Beans in the Spring container, it registers them as singletons by default (only one instance is registered and shared). Therefore, if they are the same Spring Bean, they are all the same instance. Although it's possible to configure them not to be singletons, singletons are used in most cases, except for special circumstances.

## Registering Spring Beans Directly with Java Code
- Spring Beans are registered using the `@Configuration` and `@Bean` annotations.
- Using `@Configuration` allows you to designate a Class that acts as a Configuration in a Spring project.

Create a `SpringConfig` file in the `src/main/java` subfolder and write the code.

```java
package hello.hellospring.service;

import hello.hellospring.repository.MemberRepository;
import hello.hellospring.repository.MemoryMemberRepository;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class SpringConfig {

    @Bean
    public  MemberService memberService() {
        return new MemberService(memberRepository());
    }

    @Bean
    public MemberRepository memberRepository() {
        return new MemoryMemberRepository();
    }
}
```

### Notes
- There is also a way to configure with XML, but it is not commonly used nowadays.
- There are three methods for DI: field injection, setter injection, and constructor injection. Constructor injection is recommended because dependencies rarely change dynamically during runtime.
- In practice, component scanning is primarily used for standardized code like controllers, services, and repositories. If the code is not standardized or if the implementation class needs to be changed depending on the situation, it is registered as a Spring Bean through configuration.
- DI via `@Autowired` only works for objects managed by Spring, such as `helloConroller` and `memberService`. It does not work for objects you create directly without registering them as Spring Beans.
