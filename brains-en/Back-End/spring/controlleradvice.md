# Is ControllerAdvice implemented with AOP? Let's look at its operation.

## How ControllerAdvice Works
1. The DispatcherServlet catches the error.
2. A handler (HandlerExceptionResolver) capable of processing the error processes it.
3. Checks if the Controller's ExceptionHandler can handle it.
4. Checks if the ControllerAdvice's ExceptionHandler can handle it.
5. Invokes the ControllerAdvice's ExceptionHandler method to return the exception.

### DispatcherServlet's Error Catching
In Spring, the DispatcherServlet is the first to receive all requests.

Consequently, **error handling also begins at the DispatcherServlet**, and its core method, `doDispatch`, catches all Exceptions and Throwables as shown below.

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

Internally, it checks if `exception` is null, and if an exception exists, it handles the error.
Generally, the Exceptions we add will be processed in `processHandlerException`.

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

### A Handler (HandlerExceptionResolver) Capable of Processing the Error Processes It
We have confirmed that the DispatcherServlet has various exception handlers, HandlerExceptionResolver.
When handling exceptions, each implementation handles the exception, and if the **return result is not null, it means it was processed successfully**.
Among the implementations of HandlerExceptionResolver, ControllerAdvice is handled by ExceptionHandlerExceptionResolver.

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

### Checking if the Controller's ExceptionHandler Can Handle It

ExceptionHandler can be implemented in a Controller or in ControllerAdvice. Implementing it in ControllerAdvice is global, whereas implementing it in a Controller is local.

Therefore, the Controller's ExceptionHandler is checked first, **giving priority to the ExceptionHandler in the Controller**.

If the ExceptionHandler in the Controller can handle the exception, it creates and returns a `ServletInvocableHandlerMethod` containing the bean to handle the exception, the ExceptionHandler method to handle the exception, and the application context.

Here, the bean that handles the exception is the Controller.

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

### Checking if the ControllerAdvice's ExceptionHandler Can Handle It
If the ExceptionHandler in the Controller cannot handle it, **all registered ControllerAdvice beans are checked**.

If there is a ControllerAdvice's ExceptionHandler that can handle it, a `ServletInvocableHandlerMethod` is similarly created and returned.

Unlike before, the bean that handles the exception in `ServletInvocableHandlerMethod` is a ControllerAdvice bean, not a Controller.

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

### Invoking the ControllerAdvice's ExceptionHandler Method to Return the Exception
The returned `ServletInvocableHandlerMethod` contains the bean with the ExceptionHandler and the ExceptionHandler's implementation method.
Spring **uses the Reflection API to invoke the ExceptionHandler's implementation method and return the processed error.**

<br>

## Is ControllerAdvice Implemented with AOP?
To conclude, ControllerAdvice is not implemented with AOP.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FQJGre%2FbtrAroZya0Y%2FYN0JAc76kj1FszKXx3LeXk%2Fimg.png)

Looking at foreign posts, there are claims that ControllerAdvice's AOP is named after Advice. It is true that it takes its name from AOP's Advice.

This is because ControllerAdvice's operation feels like applying AOP to a Controller.

However, AOP is not actually applied. If AOP were applied, a proxy should have been used, for example, via JDK dynamic proxy or CGLib.

Furthermore, it should possess the concepts of AOP, but such aspects are absent.

ControllerAdvice is merely a Spring bean that assists with error handling at the DispatcherServlet level, which centrally processes requests, and it is not implemented with AOP.
