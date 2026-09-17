# JDK Proxy, CGLib Proxy

프록시 패턴, Proxy(대행자)를 구현한 것 중에서 가장 많이 쓰이는 종류로는 JDK Proxy와 CGLIB Proxy가 존재한다.

**둘의 가장 큰 차이점은 Target이 어떤 부분을 상속 받아서 프록시를 구분하는가에 따라 다르다.**

### JDK Proxy

> JDK Proxy는 Target의 상위 인터페이스를 상속받아서 프록시를 구현한다.

따라서 구체 클래스에 의존하려고하면 런타임 에러가 발생한다. 따라서 인터페이스를 의존하고 그 인터페이스의 구현 클래스가 꼭 필요하다.

또한 자바 Reflection을 사용하기때문에 `InvocationHandler` 추가적인 비용이 발생한다.

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

대표적으로 유저 서비스에 트랜잭션을 적용할 때 사용할 수 있다. (물론 AOP를 사용한다거나, ProxyFactoryBean - MethodInterceptor(FactoryBean)와 같은 다른 솔루션으로 구현하는 것이 더 좋지만 InvocationHandler를 통해서도 구현이 가능하다.)


<br>

### CGLib Proxy

CGLib는 target을 상속받아 프록시를 만든다.

JDK Proxy와 다르게 리플랙션을 사용하지 않고 **바이트 코드를 조작해 프록시 객체를 생성한다.**

게다가 인터페이스를 구현하지 않고도 해당 구현체를 상속받는 것으로 문제를 해결하기 때문에 성능상에 이점을 가지고 있고, 런타임 에러 발생률도 상대적으로 적다.

CGLib는 Enhancer라는 클래스를 바탕으로 프록시를 생성한다.

```java
Enhancer enhancer = new Enhancer();

enhancer.setSuperclass(ServiceImpl.class); 
enhancer.setSuperClass(NoOp.INSTANCE);

Object obj = enhancer.create();

ServiceImpl service = (ServiceImpl) obj;
service.execute(param);
```

상속을 통해서 프록시 객체가 생성되는 모습을 볼 수 있ㄷ.

`enhancer.setCallback(NoOp.INSTANCE)` 코드는 Enhancer 프록시 객체가 직접 원본 객체를 접근하기 위한 옵션이다.

기본적으로 프록시 객체들은 직접 원본 객체를 호출하기 보다는, 별도의 작업을 수행하는데 CGLib의 경우에는 Callback을 사용한다. 

그중 CGLib에서는 `net.sf.cglib.proxy.MethodInterceptor`인데, 프록시와 원본 객체 사이에 인터셉터를 두어 메서드 호출을 조작하는 것을 도와줄 수 있게 된다.

> MethodInterceptor는 InvocationHandler와 마찬가지로 proxy를 정의하는 것인데 다른점은, methodInvocation을 인자로 가지고 있으며 target의 상태를 알고있다.

**ServiceProxy** -> **ServiceInterceptor** -> **ServiceImpl**

자바 리플랙션 방식보다 CGLib의 MethodProxy 방식이 더 빠르고 예외를 덜 발생시키기 때문에 Spring Boot에서는 CGLib 방식을 기본 프록시 객체 생성 라이브러리로 채택했다.