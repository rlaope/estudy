# Java Exception Creation Cost, and How to Reduce It

When developing projects in Java, it's common to define and use `CustomException` classes for error handling consistency, readability, logging, debugging, and exception handling flexibility.

However, despite these advantages, Java has a problem where the cost of handling Exceptions is very high.

In this post, we'll explore the order in which the JVM handles Exceptions, why their creation cost is high, and finally, methods to reduce that cost.

<br>

## JVM Exception Handling Order

Referring to [this article](https://www.geeksforgeeks.org/exceptions-in-java/), when an Exception occurs, the JVM handles it as follows:

![](https://media.geeksforgeeks.org/wp-content/uploads/20230714113633/Exceptions-in-Java2-768.png)

1. **Exception Occurrence**: When an exception occurs, the JVM creates an exception object and traces the call stack of the method that threw the exception.
2. **Exception Object Propagation**: The JVM searches for exception handling code in the method that threw the exception. If no exception handling code is found, the exception object is propagated up the stack to the calling method.
3. **Exception Handling**: If the exception object is propagated to an ancestor method, the JVM searches for a `catch` block that can handle the exception. If none is found, it propagates further up.
4. **Exception Handling Failure**: If the exception object propagates all the way to the top-level method and no `catch` block can handle it, the JVM determines that the exception could not be handled and uses the `DefaultExceptionHandler` to process it.
5. **DefaultExceptionHandler Execution**: The DefaultExceptionHandler prints information about the exception object, handles the exception, or collects snapshot information to provide for debugging.

While it's best if an exception is handled immediately in the method where it occurred, if it's not, **the JVM will continue to traverse up the call stack in memory, searching for a method that can handle the exception.**

<br>

### Why Exceptions are Costly

As mentioned above, traversing the call stack is a cost, but the process of the `fillInStackTrace()` method iterating through the call stack to collect information such as class names, method names, and line numbers to create a stack trace can also be considered a cause of increased cost.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcvfD8q%2Fbtr3dbdvQIi%2FAdzJFLLWGbiCLyv2zpJ6qK%2Fimg.png)

`fillInStackTrace()` is an implemented method defined in the `Throwable` class and is designed to be called from the constructor.

All Exceptions inherit from `Throwable`, and therefore possess this method.

Generally, the time taken to generate a stack trace varies from a few milliseconds to several seconds. However, it's difficult to pinpoint an exact time as it depends on the environment where the exception occurred, the depth of the stack trace, the number of method calls in the stack frame, JVM version, and configuration.

What is certain, however, is that the deeper the stack trace, the longer it takes.

<br>

## How to Reduce Costs

I will introduce two methods.

### 1. Overriding fillInStackTrace()

CustomExceptions, excluding those provided by Java by default like NPE or OOM, are primarily used to prevent lower-level business logic from executing when a value is invalid, rather than for error tracing. Therefore, a stack trace is usually not needed.

```java
@Override
public synchronized Throwable fillInStackTrace() {
	return this;
}
```

Therefore, when simply controlling the subsequent flow with try-catch or handling exceptions with @ControllerAdvice in a Spring environment, you can override `fillInStackTrace()` to prevent saving the trace, thereby avoiding unnecessary performance degradation.

```java
public class DuplicateLoginException extends RuntimeException {
	public DuplicateLoginException(String message) {
		super(message);
	}

	@Override
	public synchronized Throwable fillInStackTrace() {
		return this;
    }
}
```

### Before

```java
kancho.realestate.comparingprices.exception.DuplicateLoginException: 이미 로그인한 상태입니다. at kancho.realestate.comparingprices.controller.UserController.validateDuplicateLogin(UserController.java:67) at kancho.realestate.comparingprices.controller.UserController.login(UserController.java:44) at java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke0(Native Method) at java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62) at java.base/jdk.internal.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43) at java.base/java.lang.reflect.Method.invoke(Method.java:566) at org.springframework.web.method.support.InvocableHandlerMethod.doInvoke(InvocableHandlerMethod.java:205) at org.springframework.web.method.support.InvocableHandlerMethod.invokeForRequest(InvocableHandlerMethod.java:150) at org.springframework.web.servlet.mvc.method.annotation.ServletInvocableHandlerMethod.invokeAndHandle(ServletInvocableHandlerMethod.java:117) at org.springframework.web.servlet.mvc.method.annotation.RequestMappingHandlerAdapter.invokeHandlerMethod(RequestMappingHandlerAdapter.java:895) at org.springframework.web.servlet.mvc.method.annotation.RequestMappingHandlerAdapter.handleInternal(RequestMappingHandlerAdapter.java:808) at org.springframework.web.servlet.mvc.method.AbstractHandlerMethodAdapter.handle(AbstractHandlerMethodAdapter.java:87) at org.springframework.web.servlet.DispatcherServlet.doDispatch(DispatcherServlet.java:1067) at org.springframework.web.servlet.DispatcherServlet.doService(DispatcherServlet.java:963) at org.springframework.web.servlet.FrameworkServlet.processRequest(FrameworkServlet.java:1006) at org.springframework.web.servlet.FrameworkServlet.doPost(FrameworkServlet.java:909) at javax.servlet.http.HttpServlet.service(HttpServlet.java:681) at org.springframework.web.servlet.FrameworkServlet.service(FrameworkServlet.java:883) at org.springframework.test.web.servlet.TestDispatcherServlet.service(TestDispatcherServlet.java:72) at javax.servlet.http.HttpServlet.service(HttpServlet.java:764) at org.springframework.mock.web.MockFilterChain$ServletFilterProxy.doFilter(MockFilterChain.java:167) at org.springframework.mock.web.MockFilterChain.doFilter(MockFilterChain.java:134) at org.springframework.web.filter.RequestContextFilter.doFilterInternal(RequestContextFilter.java:100) at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119) at org.springframework.mock.web.MockFilterChain.doFilter(MockFilterChain.java:134) at org.springframework.web.filter.FormContentFilter.doFilterInternal(FormContentFilter.java:93) at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119) at org.springframework.mock.web.MockFilterChain.doFilter(MockFilterChain.java:134) at ... 생략
```

### After

```java
kancho.realestate.comparingprices.exception.DuplicateLoginException: 이미 로그인한 상태
```

<br>

### Exception Caching
This involves pre-caching exceptions by declaring them as `static final`.

Caching and reusing exceptions as a form of constant value is more efficient than creating a new instance of the same exception type every time.

```java
public class CustomException extends RuntimeException {
	public static final CustomException INVALID_NICKNAME = new CustomException(ResponseType.INVALID_NICKNAME);
	public static final CustomException INVALID_PARAMETER = new CustomException(ResponseType.INVALID_PARAMETER);
	public static final CustomException INVALID_TOKEN = new CustomException(ResponseType.INVALID_TOKEN);     //생략
}
```

After configuring the Exception class to contain appropriate response messages or codes for exception scenarios, as shown above, you can then throw exceptions without the `new` keyword in exception-triggering situations, as follows:

```java
if (StringUtils.isBlank(parameter)) {
	throw WebtoonCoreException.INVALID_PARAMETER;
}
```
