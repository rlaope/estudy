# JDK Proxy, CGLib Proxy

Among the most commonly used implementations of the Proxy pattern (Proxy/Agent) are JDK Proxy and CGLIB Proxy.

**The biggest difference between the two lies in which part of the Target they inherit to distinguish the proxy.**

### JDK Proxy

> JDK Proxy implements the proxy by inheriting the Target's superinterface.

Therefore, attempting to depend on a concrete class will result in a runtime error. It is thus necessary to depend on an interface and its implementation class.

Additionally, because it uses Java Reflection, `InvocationHandler` incurs extra overhead.

```java
public class TransactionHandler implements InvocationHandler {

    private Service target;

    public TransactionHandler(Service target) {
        this.target = target;
    }

    @Override
    public Object invoke(Object proxy, Method method, Object[] args) {
        // TODO - .. Transaction...
        // 메서드에 대한 명세와 파라미터등을 가져오는 과정에서 리플랙션을 사용한다.
        String ret = (Service) method.invoke(target, args); // type safety가 보장되지 않는다는 단점이 있다.
        return ret.execute();
    }
}
```

A typical use case is applying transactions to a user service. (While it's often better to implement this with other solutions like AOP or ProxyFactoryBean - MethodInterceptor(FactoryBean), it is also possible to implement it via InvocationHandler.)

<br>

### CGLib Proxy

CGLib creates a proxy by inheriting the target.

Unlike JDK Proxy, it does not use reflection; instead, it **manipulates bytecode to generate proxy objects.**

Furthermore, because it solves the problem by inheriting the implementation rather than implementing an interface, it offers performance advantages and a relatively lower rate of runtime errors.

CGLib generates proxies based on a class called Enhancer.

```java
Enhancer enhancer = new Enhancer();

enhancer.setSuperclass(ServiceImpl.class); 
enhancer.setSuperClass(NoOp.INSTANCE);

Object obj = enhancer.create();

ServiceImpl service = (ServiceImpl) obj;
service.execute(param);
```

We can see the proxy object being created through inheritance.

The `enhancer.setCallback(NoOp.INSTANCE)` code is an option for the Enhancer proxy object to directly access the original object.

Typically, proxy objects perform separate operations rather than directly calling the original object, and CGLib uses Callbacks for this purpose.

Among these, CGLib uses `net.sf.cglib.proxy.MethodInterceptor`, which allows placing an interceptor between the proxy and the original object to help manipulate method calls.

> MethodInterceptor, like InvocationHandler, defines a proxy, but the difference is that it takes `methodInvocation` as an argument and is aware of the target's state.

**ServiceProxy** -> **ServiceInterceptor** -> **ServiceImpl**

Because CGLib's MethodProxy approach is faster and causes fewer exceptions than Java Reflection, Spring Boot adopted CGLib as its default proxy object generation library.
