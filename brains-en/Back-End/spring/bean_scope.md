# Bean Scopes, Potential Issues, and Solutions


## Types of Bean Scopes

Bean scopes include the following types:
- Singleton bean
- Prototype bean
- Web scopes
    - request
    - session
    - application
    - websocket

In Spring, you can specify the scope using the `@Scope` annotation.

```java
@Scope("prototype")
public class MyPrototypeBean {
	...
}
```


### Singleton Bean
As the name suggests, this is a bean managed as a singleton.

It is the default value for Spring beans. The Spring container manages beans as singletons by default.

The Spring container always returns the same instance when returning an instance of a singleton bean.


### Prototype Bean
Unlike singleton beans, the Spring container always creates and returns a new instance each time a bean is retrieved.

The Spring container creates a prototype bean, performs DI and initialization, and then returns the bean to the client. After that, **it no longer manages the created prototype bean.**

This means that termination methods like `@PreDestroy` are not called.

<br>

## Issues when using Singleton and Prototype Scoped Beans together

If singleton beans and prototype beans are used together, they may not behave as expected.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbVMTvI%2FbtrosV10NmS%2F7Q5Mn8y6h1JiCZqpSvrrYk%2Fimg.png)

1. `clientBean` is used via DI. At the time of injection, it requests a prototype bean from the Spring container.
2. The Spring container creates a prototype bean and returns it to `clientBean`. The `count` field of the prototype is 0. Now the prototype bean is out of the Spring container's hands, and `clientBean` manages it (it holds the prototype bean in an internal field, by reference).

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FRCZpH%2FbtronJvaOpV%2FD6PCZ0yakVsx55ZzHSEBXK%2Fimg.png)

3. Client A requests `clientBean` from the Spring container and receives it. Since `clientBean` is a singleton, the same `clientBean` is always returned.
4. The client calls the `logic` of `clientBean`.
5. `clientBean` calls `addCount()` on `prototypeBean` to increment its count. The count becomes 1.
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fl1mTK%2FbtrokqXcP2O%2FOvhG1Yg6XFqXynZbRngcQk%2Fimg.png)
6. Client B requests `clientBean` from the Spring container and receives it. Since `clientBean` is a singleton, the same bean is returned.
7. The prototype bean held internally by `clientBean` was already injected in the past. It was newly created by requesting it from the Spring container at the time of injection, not newly created each time it's used.
8. Client B calls `clientBean.logic()`.
9. `clientBean` calls `addCount` on `prototypeBean` to increment the prototype bean's count. The count becomes 2.

The problem here is that client A and client B each called `logic` once. Therefore, the count of `PrototypeBean` should normally be 1 for each client, but client B's result is 2.

The prototype-scoped bean held by `clientBean` was already injected at the time of creation. It was newly created by requesting it from the Spring container at the time of injection, not **newly created each time it's used.**


### How can this be resolved?
This can be resolved by using the `ObjectFactory` and `ObjectProvider` classes.

This approach resolves the issue using DL (Dependency Lookup) rather than DI.

DL, unlike DI, means finding the necessary dependencies from the container at the required time, rather than having them injected at creation time.

```java
@RequiredArgsConstructor
static class SingletonBean {
	private final ObjectProvider<PrototypeBean> objectProvider;

	public int logic() {
		PrototypeBean prototypeBean = objectProvider.getObject();
		prototypeBean.addCount();
		return prototypeBean.getCount();
	}
}
```

> `ObjectFactory` requires the inconvenience of defining a separate `ObjectFactoryBeanCreator` in a configuration file or class, and it's a simple class with only `getObject`. `ObjectProvider` inherits from `ObjectFactory` and adds convenience features like options and stream processing. However, both depend on Spring API, as seen from their packages.

To solve the above problem, there is a method using JSR-330. The `Provider<T>` interface can solve this problem; it is an interface in the `javax.inject` package. Since Spring creates the relevant `FactoryBean` class for this method, it can be retrieved simply with a single `get()` method and is not dependent on the Spring framework.


## Injecting Request Scoped Beans into a Controller using DI/DL

This is similar to the problem that arises when using prototype beans and singleton beans together, so let's explore it.

First, let's look at DL. Suppose a controller uses a request-scoped bean (a web scope type).

Here, the controller is a singleton bean. Can a request-scoped bean be injected at the time of creation? No, because a request-scoped bean is only created when an HTTP request comes in, so it cannot be injected via DI.

And one more problem: a Controller is a singleton, so multiple clients use the same controller instance, but a request class is not.

### Solving with DL
Similar to the prototype bean problem, the `Provider<T>` interface can be used to ensure that the bean is called at the time of use.

```java
@RestController
public class MyController {
	Provider<MyRequest> myRequestProvider;

	@RequestMapping("/demo")
	public String demo(HttpServeltRequest request) {
		MyRequest provider = myRequestProvider.get();
		// logic to set new values for the provider request
	}
}

```

Thanks to `Provider`, bean creation can be delayed until the point of use. Since the logic to set new values is inserted at the time of use, a different request is created for each request.

### Solving with DI
How can this be solved with DI? Given that their creation times differ, a **proxy** can be introduced here to resolve it.

The `@Scope` annotation allows specifying the `ScopedProxyMode.TARGET_CLASS` option.

```java
@Scope(value = "request", proxyMode = ScopeProxyMode.TARGET_CLASS)
public class MyRequest {

}
```


This creates a fake proxy object for `MyRequest` and allows it to be pre-injected into other beans, regardless of the HTTP request. The Spring container uses the CGLIB method to create a fake proxy object that inherits `MyRequest` through bytecode manipulation. This fake proxy object is also created during dependency injection.

When a request comes in, this proxy object then delegates to the appropriate request, so the user cannot distinguish whether it is real or fake. It can be used identically, and since a different object is created for each request, a new request is generated for each incoming request (polymorphism).


### In Summary
Thanks to the proxy object, clients can conveniently use Request Scope as if they were using a singleton bean.

While there are various methods like `Provider` and `Proxy`, the core idea is **lazy processing** until the actual object lookup (until the required moment).

[[Spring Application Logic, Infrastructure Beans, Container Infrastructure Beans]]
