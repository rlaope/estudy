# Flexible Parameter Handling with Spring ArgumentResolver

When operating a service, you often receive various types of data.

Each time, this data needs to be pre-processed in the Controller.
In such cases, the only way to reduce code is to either encapsulate the pre-processing logic in a function or create a utility class and inject it as a dependency.

However, even with utility classes, there's the inconvenience of having to call a function every time.
This leads to duplicated code, and as it grows, the code becomes messy.

Spring provides an interface to handle such parameters in a common way, and that is the Argument Resolver.

## Spring ArgumentResolver
When logic is needed to process data received from an API endpoint, such as extracting only necessary information, `SpringArgumentResolver` is used. This can be applied by extending `HandlerMethodArgumentResolver` to create a new Resolver tailored to the application, and then adding it to the Resolver list when the application runs.

![](https://platanus.me/wp-content/uploads/2021/09/e3ad38f09c0c4b71848c67a31c84d05b.png)

Spring MVC processes requests in the following flow:

- When a user makes a request via a web browser, the DispatcherServlet receives it.
- The DispatcherServlet searches for the URI matching the request in the HandlerMapping.
  - At this point, it finds APIs implemented with RequestMapping, all of which are held by `RequestMappingHandlerAdapter`.
  - If the desired mapping is found, the Interceptor is processed first.
  - Argument Resolver processing
  - Message Converter processing
- Controller Method Invoke

## Example

Let's see how it can be used in a real Spring Framework application.

As an example, we'll create an API that displays the client's browser information on the screen, regardless of the string value provided.

```java
@Target(ElementType.PARAMETER)
@Retention(RetentionPolicy.RUNTIME)
public @interface UserInfo {
}
```

First, we create an annotation to distinguish parameters. This annotation is used at API endpoints to indicate a specific type of data.

```java
@RestController
public cass BrowserUserController {

    @GetMapping("/")
    public String getBrowser(@UserInfo String clientInfo){
        return clinetInfo;
    }
}
```

The API is designed to return the string information provided by the client as-is.

```java
@Component
public class BrowserUserArgumentResolver implements HandlerMethodArgumentResolver {
    
    // Callback function that checks the parameter value of the called controller

    @Override
    public boolean supportsParameter(MethodParameter parameter) {
        return parameter.getParameterAnnotation(UserInfo.class) != null
            && parameter.getParamterType().equals(String.class);
    }

    // Function called when the supportsParameter callback function returns true
    @Override
    public Object resolveArgument(MethodParameter parameter, ModelAndViewContainer mavContainer, NativeWebRequest webRequest, WebDataBinderFactory binderFactory) throws Exception {
        HttpServletRequest request = (HttpServletRequest)webRequest.getNativeRequest();
        return request.getHeader("User-Agent");
    }

}
    
```

Now, we create a Resolver for a specific argument type. Since we created an annotation for this purpose, we can implement the `supportParameter` callback function to return true if it's our custom annotation, after inheriting `HandlerMethodArgumentResolver`.

Finally, we implement how the parameter is returned in the `resolveArgument` function, and that's it.

Since we only need to provide browser information regardless of the data value received, we return the header value using `getHeader` from the Request parameter.

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

Now, we need to add the Resolver defined for the application. To do this, we extend the `WebMvcConfigurationSupport` class in the application's main class and override the `addArgumentResolvers` function to add the Resolver we created to the list of argumentResolvers in that parameter.
