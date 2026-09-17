# Spring's Representative @Annotations and Their Roles

### What is an Annotation?
- They serve to add various functionalities to classes and methods.
- By utilizing annotations, the Spring framework can define the role of a class, inject beans, and automatically generate getters or setters.
- They can perform various roles, such as assigning special meaning or providing functionality.

<br>

## Spring's Representative Annotations

### @Component
This annotation is used when a developer wants to register a created Class as a Spring Bean. Spring recognizes this annotation and registers the class as a Spring Bean.
```java
@Component(value="hello")
public class Hello {
  public hi() {
    System.out.println("안녕");
  }
}
```

### @ComponentScan
The Spring Framework registers classes annotated with at least one of @Component, @Service, @Repository, @Controller, or @Configuration as beans in the Context. The @ComponentScan annotation scans classes beneath the annotated class and registers them as beans.

### @Bean
- Used to create beans from external libraries or other components that the developer cannot directly control.

### @Controller
This annotation is used to explicitly tell Spring that the class serves as a Controller.
```java
@Controller
@RequestMapping("/user") // 이 클래스는 /user로 들어오는 요청을 모두 처리합니다.
public class UserController {
  @RequestMapping(method = RequestMethod.GET)
  public String getUser(Model model) {
    // GET method , /user 요청 처리
  }
}
```

### @RequestHeader
It can retrieve header values from a request and is used on method parameters where this annotation is applied.
```java
@Controller
@RequestMapping("/user")
public class UserController {
  @RequestMapping(method = RequestMethod.GET)
  public String getUser(@RequestHeader(value = "Accept-Language") String acceptLanguage) {
    // GET Method , /user 요청을 처리
  }
}
```

### @RequestMapping
It is written in the form @RequestMapping(value =""), and if the incoming URI request matches the annotation's value, the corresponding class or method is executed. It can be applied to methods and classes within a Controller object, as shown below.

- When used at the class level, it applies to all methods within that class.
- When applied to a method, it processes the URI in the manner specified by that method.

```java
@Controller                   // 이 Class는 Controller 역할을 합니다
@RequestMapping("/user")      // 이 Class는 /user로 들어오는 요청을 모두 처리합니다.
public class UserController {
    @RequestMapping(method = RequestMethod.GET)
    public String getUser(Model model) {
        //  GET method, /user 요청을 처리
    }
    @RequestMapping(method = RequestMethod.POST)
    public String addUser(Model model) {
        //  POST method, /user 요청을 처리
    }
    @RequestMapping(value = "/info", method = RequestMethod.GET)
    public String addUser(Model model) {
        //  GET method, /user/info 요청을 처리
    }
}
```

### @RequestParam
This annotation matches parameters passed in the URL with method arguments, allowing parameters to be received and processed, as shown below. It converts a JSON-formatted Body into a Java object via a MessageConverter.

```java
@Controller                   // 이 Class는 Controller 역할을 합니다
@RequestMapping("/user")      // 이 Class는 /user로 들어오는 요청을 모두 처리합니다.
public class UserController {
    @RequestMapping(method = RequestMethod.GET)
    public String getUser(@RequestParam String nickname, @RequestParam(name="old") String age) {
        // GET method, /user 요청을 처리
        // https://naver.com?nickname=dog&old=10
        String sub = nickname + "_" + age;
        ...
    }
}
```

### RequestBody
- This annotation matches data passed in the body with method arguments, allowing data to be received and processed, as shown below.
- It converts the HTTP request body sent by the client into a Java object. It is used as follows.

When a client sends values (primarily objects) in JSON or XML format in the body, this converts the content into a Java object.

```java
@Controller                   // 이 Class는 Controller 역할을 합니다
@RequestMapping("/user")      // 이 Class는 /user로 들어오는 요청을 모두 처리합니다.
public class UserController {
    @RequestMapping(method = RequestMethod.POST)
    public String addUser(@RequestBody User user) {
        //  POST method, /user 요청을 처리
        String sub_name = user.name;
        String sub_old = user.old;
    }
}
```

### @ModelAttribute
- It connects (binds) HTTP parameters and Body content sent by the client to an object 1:1 via setter functions.
- Unlike RequestBody, the HTTP Body content requires `multipart/form-data` format.
- Unlike @RequestBody, which accepts JSON, @ModelAttribute cannot process JSON.

### @ResponseBody
@ResponseBody ensures that the return value of a method is not rendered as a View but is written directly to the HTTP Response Body. When returning, it returns data such as JSON or XML.
```java
@Controller                   // 이 Class는 Controller 역할을 합니다
@RequestMapping("/user")      // 이 Class는 /user로 들어오는 요청을 모두 처리합니다.
public class UserController {
    @RequestMapping(method = RequestMethod.GET)
    @ResponseBody
    public String getUser(@RequestParam String nickname, @RequestParam(name="old") String age) {
        // GET method, /user 요청을 처리
        // https://naver.com?nickname=dog&old=10
        User user = new User();
        user.setName(nickname);
        user.setAge(age);
        return user;
    }
}
```

### @Autowired
There are three main ways to inject bean objects in the Spring Framework. @Autowired is used to inject beans. The Spring Framework inspects the class and injects the bean according to its type (it first checks the type, then the name if the type is not found).

- @Autowired
- Constructor (@AllArgsConstructor)
- Setter

### @GetMapping
It performs the same role as RequestMapping(Method=RequestMethod.GET) and is used as follows.
```java
@Controller                   // 이 Class는 Controller 역할을 합니다
@RequestMapping("/user")      // 이 Class는 /user로 들어오는 요청을 모두 처리합니다.
public class UserController {
    @GetMapping("/")
    public String getUser(Model model) {
        //  GET method, /user 요청을 처리
    }
    
    ////////////////////////////////////
    // 위와 아래 메소드는 동일하게 동작합니다. //
    ////////////////////////////////////

    @RequestMapping(method = RequestMethod.GET)
    public String getUser(Model model) {
        //  GET method, /user 요청을 처리
    }
}
```

### @PostMapping
It performs the same role as RequestMapping(Method=RequestMethod.POST) and is used as follows.
```java
@Controller                   // 이 Class는 Controller 역할을 합니다
@RequestMapping("/user")      // 이 Class는 /user로 들어오는 요청을 모두 처리합니다.
public class UserController {
    @RequestMapping(method = RequestMethod.POST)
    public String addUser(Model model) {
        //  POST method, /user 요청을 처리
    }

    ////////////////////////////////////
    // 위와 아래 메소드는 동일하게 동작합니다. //
    ////////////////////////////////////

    @PostMapping('/')
    public String addUser(Model model) {
        //  POST method, /user 요청을 처리
    }
}
```

### @SpringBootTest
Provides the necessary dependencies for Spring Boot Tests.
```java
// DemoApplicationTests.java
package com.example.demo;

import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;

@SpringBootTest
class DemoApplicationTests {

	@Test
	void contextLoads() {

	}

}
```

### @Test
Indicates the target for testing in JUnit.
