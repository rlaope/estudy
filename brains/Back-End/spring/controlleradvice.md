# ControllerAdvice는 AOP로 구현되어 있을까? 동작 과정 살펴보기

## ControllerAdvice의 동작 과정
1. 디스패처 서블릿이 에러를 catch함
2. 해당 에러를 처리할 수 잇는 처리기(HandlerExceptionResolver)가 에러를 처리함
3. 컨트롤러의 ExceptionHandler로 처리가능한지 검사함
4. ControllerAdvice의 ExceptionHandler로 처리가능한지 검사함
5. ControllerAdvice의 ExceptionHandler 메서드를 invoke하여 예외를 반환.

### 디스패처 서블릿의 에러 catch
스프링에서 모든 요청을 가장 먼저 받는 곳은 디스패처 서블릿이다.  
  
그러다 보니 **에러 처리가 시작되는 곳 역시 디스패처 서블릿**인데, 디스패처 서블릿의 핵심 메서드인 doDispatch에는 다음과 같은 모든 Exception과 Throwable을 catch하고 있다.

```java
protected void doDispatch(HttpServletRequest request, HttpServletResponse response) throws Exception {
    try {
        // 요청을 컨트롤러로 위임하는 부분 생략
    }
    catch (Exception ex) {
        dispatchException = ex;
    }
    catch (Throwable err) {
        dispatchException = new NestedServletException("Handler dispatch failed", err);
    }
        
    processDispatchResult(processedRequest, response, mappedHandler, mv, dispatchException);
}
```

그리고 내부에서 exception이 null인지 아닌지 검사하여 exception이 존재하면 에러를 처리해주고 있다.  
일반적으로 우리가 추가한 Exception들은 processHandlerException에서 처리가 될 것이다.

```java
private void processDispatchResult(
    HttpServletRequest request,
     HttpServletResponse response,
    @Nullable HandlerExecutionChain mappedHandler,
    @Nullable ModelAndView mv,
    @Nullable Exception exception) throws Exception {

    boolean errorView = false;

    if (exception != null) {
        if (exception instanceof ModelAndViewDefiningException) {
            logger.debug("ModelAndViewDefiningException encountered", exception);
            mv = ((ModelAndViewDefiningException) exception).getModelAndView();
        }
        else {
            Object handler = (mappedHandler != null ? mappedHandler.getHandler() : null);
            mv = processHandlerException(request, response, handler, exception);
            errorView = (mv != null);
        }
    }

    // 생략
}
```

### 해당 에러를 처리할 수 있는 처리기 (HandlerExceptionResolver)가 에러를 처리함
디스패처 서블릿이 다양한 예외 처리기 HandlerExceptionResolver를 가지고 있음을 확인하였다.  
예외 처리 시에는 각각의 구현체들로부터 예외를 핸들링 시키는데, **반환 결과가 null이 아니라면 정상적으로 처리**된 것이다.   
HandlerExceptionResolver의 구현체 중에서 ControllerAdvice는 ExceptionHandlerExceptionResolver에 의해 처리된다.

```java
@Override
@Nullable
public ModelAndView resolveException(
    HttpServletRequest request, HttpServletResponse response, @Nullable Object handler, Exception ex) {

    if (this.resolvers != null) {
        for (HandlerExceptionResolver handlerExceptionResolver : this.resolvers) {
            ModelAndView mav = handlerExceptionResolver.resolveException(request, response, handler, ex);
            if (mav != null) {
                return mav;
            }
        }
    }
    return null;
}
```

### 컨트롤러의 ExceptionHandler로 처리 가능 여부 검사

ExceptionHandler는 Controller에 구현할 수도 있고, ControllerAdvice에도 구현할 수 있다. ControllerAdvice에 구현하는 것은 전역적인 반면에 Controller에 구현하는 것은 지역적이다.  
  
그러므로 **Controller에 있는 ExceptionHandler가 우선 순위를 갖도록** 먼저 컨트롤러의 ExceptionHandler 검사한다. 
  
그리고 컨트롤러에 있는 ExceptionHandler가 예외를 처리할 수 있다면 (예외를 처리할 빈, 예외를 처리할 ExceptionHandler 메서드, 애플리케이션 컨텍스트)를 담은 ServletInvocalbeHandlerMethod를 만들어 반환한다.  
  
여기서 예외를 처리할 빈에는 컨트롤러가 존재한다.

```java
@Nullable
protected ServletInvocableHandlerMethod getExceptionHandlerMethod(
        @Nullable HandlerMethod handlerMethod, Exception exception) {

    Class<?> handlerType = null;

    if (handlerMethod != null) {
        handlerType = handlerMethod.getBeanType();
        ExceptionHandlerMethodResolver resolver = this.exceptionHandlerCache.get(handlerType);
        if (resolver == null) {
            resolver = new ExceptionHandlerMethodResolver(handlerType);
            this.exceptionHandlerCache.put(handlerType, resolver);
        }
        Method method = resolver.resolveMethod(exception);
        if (method != null) {
            return new ServletInvocableHandlerMethod(handlerMethod.getBean(), method, this.applicationContext);
        }
        // For advice applicability check below (involving base packages, assignable types
        // and annotation presence), use target class instead of interface-based proxy.
        if (Proxy.isProxyClass(handlerType)) {
            handlerType = AopUtils.getTargetClass(handlerMethod.getBean());
        }
    }
    
    ...
}
```

### ControllerAdvice의 ExceptionHandler로 처리가능한지 검사함
컨트롤러에서 갖는 ExceptinoHandler로 처리가 불가능하면 **등록된 모든 ControllerAdvice 빈을 검사**한다.  
  
그리고 처리 가능한 ControllerAdvice의 ExceptionHandler가 있담녀 마찬가지로 ServletInvocableHandlerMethod를 만들어 반환한다.  
  
아까와 다르게 ServletInvocableHandlerMethod의 예외를 처리할 빈에는 컨틀로러가 아닌 ContorllerAdvice 빈이 존재한다.

```java
@Nullable
protected ServletInvocableHandlerMethod getExceptionHandlerMethod(
        @Nullable HandlerMethod handlerMethod, Exception exception) {

    ...

    for (Map.Entry<ControllerAdviceBean, ExceptionHandlerMethodResolver> entry : this.exceptionHandlerAdviceCache.entrySet()) {
        ControllerAdviceBean advice = entry.getKey();
        if (advice.isApplicableToBeanType(handlerType)) {
            ExceptionHandlerMethodResolver resolver = entry.getValue();
            Method method = resolver.resolveMethod(exception);
            if (method != null) {
                return new ServletInvocableHandlerMethod(advice.resolveBean(), method, this.applicationContext);
            }
        }
    }

    return null;
}
```

### ControllerAdvice의 ExceptionHandler 메서드를 Invoke하여 예외를 반환함
반환받은 ServletInvocableHandlerMethod에는 ExceptionHandler를 갖는 빈과 ExceptionHandler의 구현 메서드가 존재한다.  
스프링은 **리플렉션 api를 이용해 ExceptionHandler의 구현 메서드를 호출해 처리한 에러를 반환한다.**

<br>

## ControllerAdvice는 AOP로 구현되어 있나?
결론 부터 말하자면 ControllerAdvice는 AOP로 구현되어있지 않다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FQJGre%2FbtrAroZya0Y%2FYN0JAc76kj1FszKXx3LeXk%2Fimg.png)

해외 포스팅들을 보면 ControllerAdvice의 AOP는 Advice로 부터 이름을 따온 것이라는 내용들이 있다. 물론 AOP의 Advice에서 이름을 따온 것은 맞다.  
  
왜냐하면 ControllerAdvice의 동작 방식이 Controller에 AOP를 적용하는 느낌이기 때문이다.  
  
하지만 실제로 AOP가 적용된 것은 아니다. AOP를 적용하였으면 JDK 동적 프록시나 CGLib등을 이용해 프록시를 적용했어야 한다.  
  
또한 AOP가 갖는 개념들이 있어야 하지만 그러한 부분은 존재하지 않는다.  
  
ControllerAdvice는 단지 중앙 집중형으로 요청을 처리하는 디스패처 서블릿 단에서 에러 처리를 도와주는 스프링의 빈일 뿐이며 AOP로 구현된 것이 아니다.