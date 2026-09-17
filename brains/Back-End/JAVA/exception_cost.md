# Java의 예외 생성 비용, 비용 절감 방법

자바로 프로젝트를 진행할 때, 보통 에러 처리의 일관성과 가독성, 로깅, 디버깅, 예외 처리 유연성을 위해서 `CustomException` 클래스를 정의하여 자주 사용한다.

그러나 여러 이점들이 있음에도, 자바에서는 Exception의 처리 비용이 매우 비싸다는 문제가 있다.

이번 글에서는 JVM이 Exception을 처리하는 순서와 생성 비용이 비싼 이유, 마지막으로 비용 절감 방법에 대해서 알아보도록 하겠다.

<br>

## JVM Exception 처리 순서

[이 글](https://www.geeksforgeeks.org/exceptions-in-java/)을 참고해보면, Exception이 발생하면 다음과 같이 JVM에서 Exception을 수행한다.

![](https://media.geeksforgeeks.org/wp-content/uploads/20230714113633/Exceptions-in-Java2-768.png)


1. **예외 발생**: 예외가 발생하면 JVM은 예외 객체를 생성하고, 예외를 발생시킨 메서드의 호출 스택을 추적한다.
2. **예외 객체 전파**: JVM은 해당 예외를 발생시킨 메서드에서 예외 처리 코드를 찾는다. 예외 처리 코드가 없는 경우에는 예외 객체를 호출하여 스택의 상위 메서드로 전파시킨다.
3. **예외 처리**: 예외 객체가 상위 메서드로 전파되게 된다면, 해당 예외를 처리할 수 있는 catch 블록을 찾고, 없다면 다시 상위로 전파된다.
4. **예외 처리 실패**: 예외 객체가 최상위 메서드까지 전파되어도, 예외를 처리할 수 있는 catch 블록이 없는 경우 JVM은 예외를 처리하지 못한 것으로 판단하여 해당 예외를 처리할 수 있는 `DefaultExceptionHandler`를 사용해 예외를 처리한다.
5. **DefaultExceptionHandler 실행**: DefaultExceptionHandler는 예외 객체에 대한 정보를 출력하고 해당 예외를 처리하거나 스냅샷 정보를 수집하여 디버깅을 위한 정보로 제공한다.

예외가 발생한 메서드에서 바로 처리가 된다면 가장 좋지만, 바로 처리되지 못한다면 **JVM은 해당 예외를 처리할 수 있는 메서드를 찾을 때 까지 계속해서 상위 메서드로 거슬러 올라가면서 메모리의 호출 스택(call stack)을 탐색하게 된다.**


<br>

### Exception이 비용이 생기는 원인

위에서 언급한 것 처럼 호출 스택을 탐색하는 과정도 비용이지만, `fillInStackTrace()` 메서드가 호출 스택을 순회하며 클래스명, 메서드명, 코드 줄 번호 등의 정보를 모아 stacktrace로 만드는 과정 또한 비용 증가의 원인이라고 볼 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcvfD8q%2Fbtr3dbdvQIi%2FAdzJFLLWGbiCLyv2zpJ6qK%2Fimg.png)

`fillInStackTrace()`는 Throwable 클래스에 정의된 구현 메서드로 생성자에서 호출되도록 되어있다.

모든 Exception은 Throwable을 상속받도록 되어있기 때문에 해당 메서드를 가지고 있다.

일반적으로 stacktrace 생성하는 시간은 몇 밀리초에서 몇 초까지 다양하다. 하지만 이는 예외가 발생한 환경과 stacktrace의 depth, stackframe의 메서드 호출 수, JVM 버전 및 설정 등에 따라 다르기 때문에 시간을 특정하기는 어렵다.

그러나 확실한 것은 stacktrace가 깊을수록 오랜 시간이 걸린다는 것이다.

<br>

## 비용 절감 방법

두 가지 방법을 소개해보겠다.

### 1. fillInStackTrace() 재정의 하기

보통 NPE나 OOM와 같이 자바에서 기본적으로 제공하는 예외를 제외한, CustomException은 에러의 추족보다는 유효하지 않은 값일 때 하위 비즈니스 로직을 수행하지 못하도록 하기 위한 용도로 주로 사용된다. 따라서 보통 StackTrace가 필요가 없다.

```java
@Override 
public synchronized Throwable fillInStackTrace() { 
	return this; 
}
```

때문에 단순히 try-catch로 이후 flow를 제어하거나 Spring 환경에서 @ControllerAdvice로 예외 처리하는 경우에는 불필요한 성능 저하를 막기 위해 trace를 저장하지 않도록 오버라이딩 하여 처리할 수 있다.

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

### 예외 캐싱
static final로 선언하여 예외를 미리 캐싱해서 사용하는 것이다.

일종의 상수 값 형태로 예외를 캐싱해 두고 쓰는 것이 매번 같은 종류의 예외로 new로 생성하는 방법보다 효율적이다.

```java
public class CustomException extends RuntimeException {
	public static final CustomException INVALID_NICKNAME = new CustomException(ResponseType.INVALID_NICKNAME);     
	public static final CustomException INVALID_PARAMETER = new CustomException(ResponseType.INVALID_PARAMETER);     
	public static final CustomException INVALID_TOKEN = new CustomException(ResponseType.INVALID_TOKEN);     //생략 
}
```

위와 같이 Exception 클래스에 예외 상황에 대한 적당한 응답 메시지나 코드를 담도록 한 뒤, 아래처럼 예외 발생 상황에서 new 키워드 없이 throw를 수행하도록 한다.

```java
if (StringUtils.isBlank(parameter)) {
	throw WebtoonCoreException.INVALID_PARAMETER; 
}
```
