# 🔒 Understanding the Characteristics and Differences Between Servlet Filter and Spring Interceptor

#### Overview

There's a team of GSM students called Team.Lifestyle. During our team activities, we conducted an online study session.

This topic was chosen for the study.

Everyone who has studied Spring Security has probably heard of filters and interceptors at some point.

While we know they are different, many backend developers, including myself, can't properly explain their distinctions.

So, I'll summarize them all at once. Let's also look at them from an AOP perspective.

When developing web applications with Java, there are many common tasks that need to be handled.

### Servlet Filter

Simply put, a filter is reusable code that can modify HTTP requests and responses.

It exists as an object and is located between the client's request and the final resource, allowing modification of the client's request information.

Filters are a J2EE standard specification feature that provides the ability to perform additional tasks for all requests matching a URL pattern, before and after the request is delivered to the Dispatcher Servlet.

Since the Dispatcher Servlet is the front controller at the very front of Spring, filters are processed outside the scope of Spring.

In other words, they are managed by a web container like Tomcat, not the Spring container. (However, they can be registered as Spring Beans.)

They are processed before and after the Dispatcher Servlet.

![](image/filter_interceptor_0.png)

**Filter Flow**

HTTP Request -> WAS -> Filter -> Servlet -> Controller

When a filter is applied, the filter is called first, and then the servlet is called.

**Filter Restrictions**

HTTP Request -> WAS -> Filter -> Servlet -> Controller // Logged-in user

HTTP Request -> WAS -> Filter (If deemed an inappropriate request, servlet is not called) // Non-logged-in user

If a filter determines a request is inappropriate, it can terminate the process there.

**Filter Chain**

HTTP Request -> WAS -> Filter1 -> Filter2 -> Filter3 -> Servlet -> Controller

Filters can be chained in this manner.

For example, you can apply a logging filter and then add another filter to check login status.

#### Filter Interface and Methods

To add a filter, you must implement the javax.servlet.Filter interface, which has the following three methods:

1. init()

2. doFilter()

3. destroy()

```
public interface Filter {

    public default void init(FilterConfig filterConfig) throws ServletException {}

    public void doFilter(ServletRequest request, ServletResponse response,
            FilterChain chain) throws IOException, ServletException;

    public default void destroy() {}
}
```

**init()**

* The init method is used to initialize the filter object and add services.
* Once the web container calls the init method once to initialize the filter object, subsequent requests are processed through doFilter.

**doFilter()**

* doFilter is a method executed by the web container before all HTTP requests matching a URL pattern are passed to the Dispatcher Servlet.
* FilterChain is an argument to doFilter. Through FilterChain's doFilter, the request is passed to the next target.
* By inserting our necessary processing before and after chain.doFilter(), we can perform the desired operations.

**destroy()**

* The destroy method removes the filter object from service and releases the resources it uses.
* It is called only once by the web container, and doFilter is not used thereafter.

Example Code: Let's create a filter that logs user information from HTTP requests.

```
@Slf4j
public class LogFilter implements Filter {
	
    @Override
    public void doFilter(ServletReqeust req, ServletResponse res, FilterChain chain) throws IOException, ServletException {
    	log.info("Run LogFilter.doFilter()");
        
        // ServletRequest는 기능이 많지 않다 그래서 다운캐스팅을 사용한다.
        // HTTP 요청이 아닌 경우까지 고려해 만든 인터페이스기 때문.
        
        HttpServletRequest request = (HttpServletRequest) req;
        String requestURI = request.getRequestURI();
        String uuid = UUID.randomUUID().toString();
        
        try{
        	log.info("로그필터 Request {} , {}",uuid,requestURI);
            // 다음 필터가 있으면 호출 없으면 서블릿 호출 이 부분이 없으면 다음 단계가 진행이 되지 않는다.
            chain.doFilter(request, response); 
        } catch(Exception e) {
        	throw e;
        } finally{
        	log.info("로그필터 종료");
        }
    }
}
```

UUID.randomUUID().toString(): This is to distinguish requests.

*Once a Filter is created, it must be registered as a bean using FilterRegisterBean.*

### Interceptor

As a technology provided by Spring, an interceptor is a type of filter that can reference or process requests and responses before and after the Dispatcher Servlet calls a controller's handler (a method that should be executed according to the URL requested by the user, hereinafter referred to as handler).

The word "intercept" means to "snatch." True to its meaning, an interceptor snatches the Request object that enters the server due to a user's request before it reaches the controller's handler, allowing developers to perform additional desired tasks before sending it to the handler.

Unlike filters, which operate in the web container, interceptors operate within the Spring context.

The Dispatcher Servlet requests the handler mapping to find an appropriate controller, and as a result, it returns a Handler ExecutionChain. If one or more interceptors are registered in this execution chain, the controller is executed sequentially through the interceptors; otherwise, the controller is executed directly.

Since interceptors operate within the Spring context, they function after the request has passed through filters in the web context and been received by the Dispatcher Servlet, which acts as the front controller.

**Why use it?**

Interceptors are used for additional tasks before or after a specific controller's handler is executed.

(Additional tasks include login checks, permission checks, etc.)

Let me give an example.

Suppose you've written an admin controller handler that only developers can use.

To ensure only admin accounts can execute it, you would have to write session check code for each handler to verify if the accessing user is an administrator.

What if the number of handlers you need to write this for becomes large?

1. Memory waste, server load

This is because writing session check code for as many handlers as needed leads to a lot of repetitive code.

2. Concern about missing code

Since humans write the code, mistakes like omissions can occur. What if a session check is missed in a handler that accesses user information?

The thought alone is dreadful.

![](image/filter_interceptor_1.png)

The image above illustrates the sequence; an interceptor does not delegate requests to the controller.

#### Interceptor Interface and Methods

To add an interceptor, you must implement the org.springframework.web.servlet.HandlerInterceptor interface, which has the following three methods:

1. preHandle()

2. postHandle()

3. afterCompletion()

```
public interface HandlerInterceptor {

    default boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler)
        throws Exception {
        
        return true;
    }

    default void postHandle(HttpServletRequest request, HttpServletResponse response, Object handler,
        @Nullable ModelAndView modelAndView) throws Exception {
    }

    default void afterCompletion(HttpServletRequest request, HttpServletResponse response, Object handler,
        @Nullable Exception ex) throws Exception {
    }
}
```

**preHandle()**

* This method is executed before the controller is called.
* It can be used for preprocessing tasks that need to be handled before the controller, such as processing or adding request information.
* The third parameter of preHandle, the handler parameter, is an object of a new type called HandlerMethod, which maps to the controller bean found by the handler mapping and abstracts the information of the method annotated with @RequestMapping.
* Because it receives information about the handler to be executed as an argument, it allows for more detailed logic configuration compared to 'Servlet Filters'.
* The return value is boolean. If it returns true, preHandle() is executed, and then the handler is accessed.
* If it returns false, the operation is aborted, and the controller and remaining interceptors are not executed.

**postHandle()**

* This method is executed after the controller has been called.
* It is called after the handler has completed execution but before the view is rendered.
* It receives information of type ModelAndView as an argument. Therefore, it can reference or manipulate the model object's information that the controller worked on to pass view data. Recently, it is less frequently used when creating RestAPI-based controllers (@RestController) that provide data in JSON format.
* It is not executed if preHandle() returns false.
* If multiple interceptors are applied, preHandle() is called in reverse order.
* It is not processed during asynchronous request handling.
* If an exception occurs during processing in a layer below the controller, postHandle is not called.

**afterCompletion()**

* This method is executed after all operations, including the generation of the final result in all Views, have been completed.
* It is a suitable method for releasing resources used during request processing.
* It is not executed if preHandle() returns false.
* If multiple interceptors are applied, preHandle() is called in reverse order.
* It is not called during asynchronous request processing.
* Unlike postHandle(), afterCompletion is always called even if an exception occurs during processing in a layer below the controller.

I've implemented an interceptor that logs the operations of HandlerInterceptor.

```
@Slf4j
public class CustomInterceptor implements HandlerInterceptor {
 
    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler)
            throws Exception {
        
        log.info("preHandle1");
        
        return true;
    }
 
    @Override
    public void postHandle(HttpServletRequest request, HttpServletResponse response, Object handler,
            ModelAndView modelAndView) throws Exception {
        
        log.info("postHandle1");
        
    }
 
    @Override
    public void afterCompletion(HttpServletRequest request, HttpServletResponse response, Object handler, Exception ex)
            throws Exception {
        
        log.info("afterCompletion1");
        
    }    
    
}
```

#### Interceptor vs AOP

Instead of interceptors, you could also apply AOP by creating advice for cross-cutting concerns to be applied to controllers.

However, for the following reasons, it is better to use interceptors for cross-cutting concerns applied during the controller's invocation process:

1. Both controller types and execution methods vary, making it difficult to write pointcuts (selecting methods to apply).

2. Controller parameters and return values are not consistent.

3. It is difficult to obtain HttpServletRequest and HttpServletResponse objects in AOP, but they are passed as parameters to interceptors.

### Differences Between Filter and Interceptor

Filters and interceptors differ in the containers they are managed by and their ability to manipulate Request/Response objects, leading to different use cases.

![](image/filter_interceptor_2.png)

Some claim that Filters cannot be registered as Spring Beans and cannot receive bean injections, but this is an incorrect explanation. This is a very old notion; filters can currently be registered as Spring Beans and can be injected elsewhere or receive other bean injections.

**Ability to Manipulate Request, Response Objects**

Filters can manipulate Request and Response objects, but interceptors cannot.

Here, "manipulate" means replacing them with different objects, not just changing their internal state.

For a filter to call the next filter, filter chaining must be performed. During this process, the Request/Response objects are passed.

This means we can pass the Request/Response objects we want. Although it would result in an NPE (NullPointerException), you could even pass null.

```
public MyFilter implements Filter {

    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) {
        // The developer can pass different request and response objects
        chain.doFilter(request, response);       
    }
    
}
```

However, the processing flow for interceptors is different from filters.

The Dispatcher Servlet holds a list of multiple interceptors and executes them sequentially in a loop.

Returning true allows the next interceptor to execute or the request to be passed to the controller, while false aborts the request.

Therefore, we cannot pass different Request/Response objects. This is a clear distinction from filters.

```
public class MyInterceptor implements HandlerInterceptor {

    default boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) {
        // Cannot replace Request/Response; only a boolean value can be returned.
        return true;
    }

}
```

### Use Cases and Examples for Filter and Interceptor

**Filter Use Cases and Examples**

* Common security and authentication/authorization related tasks
* Logging and inspection for all requests
* Image/data compression and character encoding
* Features that should be decoupled from Spring

Filters can fundamentally handle **tasks that need to be processed globally, independent of Spring.**

A prime example is common security tasks. Since filters operate before interceptors, they can perform global security checks (e.g., XSS defense) and block requests if they are not valid. This prevents requests from reaching the Spring container, thereby enhancing stability.

Additionally, filters are suitable for **implementing features used throughout a web application**, such as image or data compression and character encoding. Filters are a much more powerful technology than interceptors in that they can manipulate the ServletRequest/ServletResponse objects passed to the next chain.

**Interceptor Use Cases and Examples**

* Detailed security and authentication/authorization common tasks
* Logging or inspection of API calls
* Data processing for controllers

Interceptors can handle **tasks that need to be processed globally in relation to client requests.**

Examples include client request-related tasks such as detailed authentication and authorization.

For instance, there might be cases where users from a specific group cannot use certain features.

Since these tasks need to be checked before reaching the controller, interceptors are suitable for handling them.

Furthermore, unlike filters, interceptors receive objects like HttpServletRequest/HttpServletResponse,

so they cannot manipulate the objects themselves. Instead, they can manipulate the internal properties of these objects, making them convenient for **processing information to be passed to the controller**. For example, user information retrieved based on a user ID can be added to the HttpServletRequest.

Additionally, we may need to record information about API calls for various purposes. In such cases, interceptors, which provide HttpServletRequest and Response, are convenient for recording client IP addresses and request details.

A prime example of a tool that uses Filters for authentication and authorization is Spring Security. One of Spring Security's characteristics is its independence from Spring MVC, which is due to its filter-based authentication/authorization processing.
