# Spring ArgumentResolver를 이용한 유연성 있는 파라미터 처리

서비스를 운영해다보면 다양한 종류의 데이터를 받게 된다.  
  
그럴 때마다 Controller 부분에서 이를 전처리하게 되는데,  
이렇게 되면 각 컨트롤러에서 전처리해야하는 코드를 함수화 하거나 Util 클래스를 만들고 이를 의존성 주입을 해야하는게 그나마 코드를 줄일 수 있는 방법이다.
  
그러나 Utils를 사용해도 매번 함수를 호출해야 하는 불편함이 있다.  
그렇게 되면 코드가 중복되고, 이것이 커지면 역시 코드가 난잡해지게 된다.
  
Spring에서는 이러한 파라미터를 공통으로 처리할 수 있도록 구현된 인터페이스가 있는데, 그것이 바로 Argument Resolver다.

## Spring ArgumentResolver
API 엔드 포인트로부터 들어온 데이터를 가공하여 필요한 데이터만 뽑는 등의 로직이 필요할 경우 사용하는 `SpringArgumentResolver`는 `HandlerMethodArgumentResolver`를 상송해 애플리케이션에 맞는 새로운 Resolver를 만들고, 애플리케이션 실행시, Resolver 리스트에 추가함으로서 적용할 수 있다.

![](https://platanus.me/wp-content/uploads/2021/09/e3ad38f09c0c4b71848c67a31c84d05b.png)

Spring MVC는 다음과 같은 흐름으로 요청 처리가 이루어진다.

- 사용자가 웹 브라우저를 통해 요청하면 DispatcherServlet이 이를 받는다.
- DispatcherServlet은 해당 요청에 맞는 URI를 HanlderMapping에서 검색한다.
  - 이때 RequestMapping으로 구현한 API를 찾게 되는데, 이들은 `RequestMappingHandlerAdapter`가 모두 가지고 있다.
  - 원하는 Mapping을 찾은 경우, 첫 번째로 Intercepter를 처리한다.
  - Argument Resolver 처리
  - Message Converter 처리
- Controller Method Invoke

## 예시

실제 Spring Framework에서 어떻게 사용할 수 있는지 보겠다.
  
예시로 어떤 문자열의 값을 주어도 클라이언트의 브라우저 정보를 화면에 띄우는 API다.

```java
@Target(ElementType.PARAMETER)
@Retention(RetentionPolicy.RUNTIME)
public @interface UserInfo {
}
```


먼저 파라미터 구분을 위한 어노테이션 한 개를 만들어줍니다. 이 어노테이션은 API 엔드포인트에서 특정 데이터임을 가리키기 위한 용도로 사용됩니다.

```java
@RestController
public cass BrowserUserController {

    @GetMapping("/")
    public String getBrowser(@UserInfo String clientInfo){
        return clinetInfo;
    }
}
```

API는 클라이언트가 주는 문자열 정보를 그대로 반환하도록 한다.

```java
@Component
public class BrowserUserArgumentResolver implements HandlerMethodArgumentResolver {
    
    // 호출되는 컨트롤러의 파라미터 값을 검사하는 콜백 함수

    @Override
    public boolean supportsParameter(MethodParameter parameter) {
        return parameter.getParameterAnnotation(UserInfo.class) != null
            && parameter.getParamterType().equals(String.class);
    }

    // supportsParameter 콜백 함수에서 true를 반환했을 경우 호출되는 함수
    @Override
    public Object resolveArgument(MethodParameter parameter, ModelAndViewContainer mavContainer, NativeWebRequest webRequest, WebDataBinderFactory binderFactory) throws Exception {
        HttpServletRequest request = (HttpServletRequest)webRequest.getNativeRequest();
        return request.getHeader("User-Agent");
    }

}
    
```

이제 특정 argument 타입에 대한 Resolver를 생성해준다. 우리는 이를 위해서 어노테이션을 만들었으므로 `HandlerMethodArgumentResolver`를 상속받은 후 supportParameter 콜백 함수에서 우리가 만든 어노테이션인지 반환하도록 함수를 구현하면 됩니다.
  
마지막으로 resolveArgument 함수에서 해당 파라미터가 어떻게 반환되는지를 구현해주면 끝입니다.  
  
우리는 어떤 데이터 값이 들어와도 브라우저 정보만을 주면 되기 때문에 Request 파라미터에서 getHeader를 이용해 헤더 값을 반환하도록 합니다.

```java
@RequiredArgsConstructor
@SpringBootApplication
public class ArgumentexampleApplication extends WebMvcConfigurationSupport {
    
    private final BrowserUserArgumentResolver loginUserArgumentResolver;

    @Override
    protected void addArgumentResolvers(List<HandlerMethodArgumentResolver> argumentResolvers) {
        super.addArgumentResolvers(argumentResolvers);
        argumentResolvers.add(loginUserArgumentResolver);
    }

    public static void main(String[] args) {
        SpringApplication.run(ArgumentexampleApplication.class, args);
    }
}
```

이제 애플리케이션을 위해서 정의한 Resolver를 추가해야한다. 이를 위해 애플리케이션 메인 클래스에 `WebMvcConfigurationSupport` 클래스를 상속하고, addArgumentResolver 함수를 재정의하여 해당 파라미터에 있는 argumentResolver 리스트위에서 만든 Resolver를 추가해주면 된다.