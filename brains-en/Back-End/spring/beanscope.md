# Bean Scope, DL(Dependency Lookup), Provider

## Bean Scope
Literally, it refers to the scope within which a bean can exist.

Spring supports various scopes as follows:

- Singleton scope: The default scope, it is the broadest scope, maintained from the start to the end of the Spring container.
- Prototype scope: A very short-lived scope where the Spring container is only involved in the creation and dependency injection of prototype beans, and no longer manages them afterward.
- Web-related scopes
  - request: A scope maintained from when a web request enters until it exits.
  - session: A scope maintained from when a web session is created until it ends.
  - application: A scope maintained within the same range as the web's servlet context.

### Singleton Scope

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FVD74w%2FbtrxdPtqUWQ%2FIqzmgKBIg6VnaYA7LMdB8k%2Fimg.png)

When you look up a singleton-scoped bean, the Spring container always returns the same instance of the bean.

### Prototype Scope

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbWoFOS%2FbtrxgGvstHm%2FbbaQhbnvyn0lJOEL2mdom1%2Fimg.png)

When you look up a prototype-scoped bean, the Spring container always creates and returns a new instance.

When a prototype bean is requested from the Spring container, the Spring container creates the prototype bean and injects the necessary dependencies.

Singleton beans are created and initialized along with the container's creation, but prototype beans are created **when the bean is looked up** from the Spring container, and their initialization methods are also executed.

The Spring container only handles the creation, dependency injection, and initialization of prototype beans.

After returning the bean to the client, it no longer manages the created prototype bean.

The responsibility for managing prototype beans lies with the client. Therefore, termination callback methods like @PreDestroy are not called.

### Issues with Prototype Beans
What happens if singleton beans and prototype beans are used together?

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FSwAuD%2Fbtrxb2foASt%2FwydOfVZ9Z8rpV5YBBqZOr1%2Fimg.png)

Let's say clientBean contains a prototype bean, as shown in the figure above. Since clientBean is a singleton, it is created along with the Spring container, and dependency injection also occurs at that time.

1. clientBean uses automatic dependency injection. At the time of injection, it requests a prototype bean from the Spring container.
2. The Spring container receives the request for a prototype bean, creates it, and returns it to clientBean. The count field value of the prototype bean is 0.

clientBean stores the prototype bean in its internal field.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb0kVLM%2FbtrxhMWzuvp%2FFaJbGQnfaOKswJAD5AjHKK%2Fimg.png)

Client A requests and receives clientBean from the Spring container.

Since it's a singleton, the same clientBean is always returned.

3. Client A calls clientBean.logic().
4. clientBean calls prototypeBean's addCount() to increment the prototype bean's count. The count value becomes 1.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FXWo9K%2FbtrxfvaausD%2FvZCeZmjoF4wRBnpo1v5DC0%2Fimg.png)

Client B requests and receives clientBean from the Spring container.

Since it's a singleton, the same clientBean is always returned. The important point here is that the prototype bean held internally by clientBean is a bean that was already injected in the past. A new prototype bean was created by requesting it from the Spring container at the time of injection, but it is not created anew every time it is used.

5. Client B calls clientBean.logic().
6. clientBean calls prototypeBean's addCount() to increment the prototype bean's count. Since the original count value was 1, it becomes 2.

Although we expected to receive a count value of 1 for each request, we get 2. Ultimately, when singleton and prototype beans are used together, it does not behave as expected.

Spring generally uses singleton beans, so a singleton bean will end up using a prototype bean.

However, singleton beans only receive dependency injection at the time of creation.

Therefore, although a prototype bean is newly created and injected at the time a Spring singleton bean is created, the problem is that it persists along with the singleton bean.

Thus, prototype beans should not only be created anew at the time of injection but also created anew each time they are used.

<br>

## Dependency Lookup (DL), Provider

When using singleton and prototype beans together, how can we create a new prototype bean every time it's used?

The simplest way is for the singleton bean to request a new prototype from the Spring container every time it uses it.

```java
@Component
class ClientBean {

    //ClientBean injects the ApplicationContext itself and requests directly.
    @Autowired
    private ApplicationContext ac;

    public int logic() {
        PrototypeBean prototypeBean = ac.getBean(PrototypeBean.class);
        prototypeBean.addCount();
        return prototypeBean.getCount();
    }
}

@Component
@Scope("prototype")
class PrototypeBean {

    private int count = 0;

    public void addCount() {
        count++;
    }

    public int getCount() {
        return count;
    }

    @PostConstruct
    public void init() {
        System.out.println("PrototypeBean.init" + this);
    }

    @PreDestroy
    public void destroy() {
        System.out.println("PrototypeBean.destroy");
    }
}
```

The code above always creates a new prototype bean through ac.getBean() at runtime.

Finding the necessary dependencies directly, rather than receiving dependency injection (DI) from an external source, is called Dependency Lookup (DL).

However, if the entire Spring ApplicationContext is injected this way, the code becomes dependent on the Spring container, and unit testing becomes difficult.

Therefore, instead of injecting the entire Spring ApplicationContext, it is a better approach to inject and use ObjectProvider.

ObjectProvider provides a DL (Dependency Lookup) service that finds a specified bean in the container on your behalf.

(For reference, ObjectFactory existed in the past, and ObjectProvider was created by adding convenience features to it.)

```java
@Autowired
private ObjectProvider<PrototypeBean> prototypeBeanProvider;

public int logic() {
    PrototypeBean prototypeBean = prototypeBeanProvider.getObject();
    prototypeBean.addCount();
    int count = prototypeBean.getCount();
    return count;
}
```

When executed, you can see that a new prototype bean is always created through prototypeBeanProvider.getObject().

When ObjectProvider's getObject() is called, it internally finds and returns the corresponding bean through the Spring container.

Although it uses features provided by Spring, its simplicity makes it much easier to create unit tests or mock code.

It depends on Spring, but no separate libraries are needed.

ObjectProvider only provides DL-level functionality.

## Web Scope

Web scopes only operate in a web environment.

Unlike prototype scopes, web scopes are managed by Spring until the scope's termination.

Therefore, termination methods are called. Web scopes include the following types:

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F129Na%2FbtrxgwsVB9N%2FKUY0UipIpOKojT3JOl0VR1%2Fimg.png)

- request: A scope maintained from when an HTTP request enters until it exits. A separate bean instance is created and managed for each HTTP request.
- response: A scope that has the same lifecycle as an HTTP Session.
- application: A scope that has the same lifecycle as the servlet context.
- websocket: A scope that has the same lifecycle as a WebSocket.

An error occurs if a web scope is directly injected when registering a Spring bean.

Singleton beans are created along with the Spring container and share its lifecycle. However, web scopes (request scope in this case) are newly created when an HTTP request comes in and disappear when the response is sent. Therefore, they are not yet created at the time a singleton bean is created, making dependency injection impossible.

In this case, using a proxy can solve the problem.

```java
@Component
@Scope(value = "request", proxyMode = ScopedProxyMode.TARGET_ClASS)
public class MyLogger {
}
```

Add `proxyMode = ScopedProxyMode.TARGET_CLASS` to the @Scope attribute.
- If the target is a class, not an interface, choose TARGET_CLASS.
- If the target is an interface, choose INTERFACES.

This way, a fake proxy class for MyLogger can be created and pre-injected into other beans, regardless of the HTTP request.

When `proxyMode = ScopedProxyMode.TARGET_CLASS` is set for the Scope, the Spring container uses a bytecode manipulation library called CGLIB to create a fake proxy object that inherits from the web-scoped bean class.

Looking at the results above, you can see that a fake proxy object is registered, not the pure Java class bean I registered.

Then, the Spring container registers this fake proxy object instead of the real one, using the original class name (first letter lowercase, bean naming convention) (myLogger in this case).

Even if you look it up with `ag.getBean("myLogger", MyLogger.class)`, the proxy object is retrieved.

Therefore, this fake proxy object is also injected for dependency injection.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fsqofr%2FbtrxezqufXQ%2Fj7ZCbeg6dOh50PQsLbEYc1%2Fimg.png)

- When a request comes in, the fake proxy object contains delegation logic to request the real bean.
- The fake proxy object knows how to find the real myLogger internally.
- When a client calls myLogger.logic(), they are actually calling a method on the fake proxy object.
- The fake proxy object calls the real myLogger.logic() of the request scope.
- Because the fake proxy object is created by inheriting the original class, clients using this object can use it identically without knowing whether it's the original or not (polymorphism).
