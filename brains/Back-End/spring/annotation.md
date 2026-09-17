# Spring의 대표적인 @Annotation과 역할

### Annotation 이란 ?
- 클래스와 메서드에 추가하여 다양한 기능을 부여하는 역할을 합니다.
- 어노테이션을 활용하여 Spring framework는 해당 클래스가 어떤 역할인지 정하기도 하고, 빈을 주입하기도 하며, 자동으로 getter나 setter를 생성하기도 합니다.
- 특별한 의미를 부여하거나 기능을 부여하는 등 다양한 역할을 수행할 수 있습니다.

<br>

## Spring 대표 Annotation

### @Component
개발자가 생성한 Class 를 Spring Bean 으로 등록할 때 사용하는 Annotation 입니다. Spring은 해당 Annotation을 보고 Spring Bean으로 등록합니다.
```java
@Component(value="hello")
public class Hello {
  public hi() {
    System.out.println("안녕");
  }
}
```

### @ComponentScan
Spring Framework는 @Component , @Service , @Repository , @Controller, @Configuration 중 1개라도 등록된 클래스를 찾으면, Context에 bean으로 등록합니다. @ComponentScan Annotation이 있는 클래스의 하위 Bean을 등록 될 클래스들을 스캔하여 Bean으로 등록해줍니다.

### @Bean
- 개발자가 제어 불가능한 외부 라이브러리와 같은 것들을 Bean으로 만들 때 사용합니다.

### @Controller
Spring 에게 해당 클래스가 Controller 역할을 한다고 명시하기 위해 사용하는 어노테이션 입니다.
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
Request의 header 값을 가져올 수 잇으며, 해당 Annotation을 쓴 메소드의 파라미터에 사용합니다.
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
@RequestMapping(value ="")와 같은 형태로 작성하며, 요청 들어온 URI의 요청과 Annotation value 값이 일치하면 해당 클래스나 메소드가 실행됩니다. Controller 객체 안의 메서드와 클래스에 적용 가능하며, 아래와 같이 사용합니다.

- 클래스 단위에 사용하면 하위 메소드에 모두 적용됩니다.
- 메소드에 적용되면 해당 메소드에서 지정한 방식으로 URI를 처리합니다.

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
URL에 전달되는 파라미터를 메소드의 인자와 매칭시켜, 파라미터를 받아서 처리할 수 있는 Annotation으로 아래와 같이 사용합니다. JSON 형식의 Body를 MessageConverter를 통해 자바 객체로 변환 시킵니다.

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
- Body에 전달되는 데이터를 메소드의 인자와 매칭시켜, 데이터를 받아 처리할 수 있는 Annotation으로 아래와 같이 사용합니다.
- 클라이언트가 보내는 HTTP 요청 본문을 Java 오브젝트로 변환합니다. 아래와 같이 사용합니다.

클라이언트가 body에 json or xml 과 같은 형태로 값(주로 객체)를 전송하면, 해당 내용을 자바 오브젝트로 변환합니다.

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
- 클라이언트가 전송하는 HTTP parameter, Body 내용을 Setter 함수를 통해 1:1로 객체에 데이터를 연결(바인딩) 합니다.
- RequestBody와 다르게 HTTP Body 내용은 `multipart/form-data` 형태를 요구합니다.
- @RequestBody가 JSON을 받는 것과 달리 @ModenAttribute의 경우에는 json을 받아 처리할 수 없습니다.

### @ResponseBody
@ResponseBody는 메소드에 리턴되는 값이  View로 출력되지 않고 HTTP Response Body에 직접 쓰여지게 됩니다. return 시에 json , xml 과 같은 데이터를 리턴합니다.
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
Spring Framework에서 빈 객체를 주입받기 위한 방법은 크게 3가지가 있습니다. Bean을 주입받기 위하여 @Autowired 를 사용합니다. Spring Framework가 클래스를 보고 타입에 맞게 Bean을 주입합니다.(Type을 먼저 확인 후 없으면 Name 확인)

- @Autowired
- 생성자(@AllArgsConstructor)
- setter

### @GetMapping
RequestMapping(Method=RequestMethod.GET)과 똑같은 역할을 하며, 아래와 같이 사용됩니다.
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
RequestMapping(Method=RequestMethod.POST)과 똑같은 역할을 하며, 아래와 같이 사용합니다.
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
Spring Boot Test에 필요한 의존성을 제공해줍니다.
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
JUnit에서 테스트 할 대상을 표시합니다.