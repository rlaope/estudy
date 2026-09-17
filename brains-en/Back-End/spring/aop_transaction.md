# Separating Transactions Using Spring AOP and Dynamic Proxies

### **What is the Proxy Pattern?**

- It's a design pattern where a proxy receives a request, handles additional functionality, and then delegates the core functionality to the target (business logic).

<br>

### **What is the Decorator Pattern?**

- It's a design pattern that allows functionality to be added or modified dynamically. Clients can use related features as desired.

An example is InputStream. `BufferdInputStream(new FileInputStream(”a.txt”))`

<br>

### **Separating Transaction and Business Logic Responsibilities Using the Proxy Pattern + Problems**

Let's say there's a user service that contains both transaction management logic and business logic. When business logic and transactions are concentrated in one place, this can be resolved by creating a class to handle transactions, injecting the user business service, processing transaction operations before business operations are performed, and then delegating the business logic.

The problem here is that a proxy class must be created for each service class containing business logic.

<br>

### **How to Solve Duplicate Code? InvocationHandler + Problems**

> Above, there was the problem that a proxy class had to be created for each service class.
>

To solve this problem, we can use **dynamic proxies**, a feature of Java reflection. Among Java reflection features, there is `InvocationHandler`, which you implement. By overriding the `invoke()` method, you add additional functionality (which can be seen as transactions here) and apply it to the object that will have this functionality added.

You can create an object with added functionality using `Proxy.newInstance(ClassLoader, Class, Target);`, but the problem is that there's no other way to register the related object as a bean.

<br>

### What is FactoryBean, and Why is it Needed? + Problems

> Above, there was the problem that while you can create an object with added functionality using `Proxy.newInstance(ClassLoader, Class, Target);`, there's no other way to **register the related object as a bean**.
>

To solve this problem, we can try using something called `FactoryBean`. `FactoryBean` is an interface that requires the implementation of the following methods: `getObject()`, `getObjectType()`, `isSingleton()`.

A `FactoryBean` is a class that contains special logic for creating objects on behalf of Spring. If you implement `FactoryBean` and register it as a bean, when you retrieve the bean, it will return the class returned by `getObject()`. In simpler terms, it returns the class that the `FactoryBean` holds. So, if you implement `UserFactoryBean` and register it as a bean, `User` will be registered as a bean. **Therefore, you can put the logic for creating a proxy object based on the `InvocationHandler` implementation into the `FactoryBean`'s `getObject()` method and then register the `FactoryBean` as a bean.** + Additionally, if you want to retrieve the `FactoryBean` itself, you can use `getBean("`&factoryBean`")` by prefixing it with an ampersand.

However, there's still a problem here: a `FactoryBean` must be continuously created for each object (service). This leads to duplicate code for object creation.

<br>

### What is ProxyFactoryBean, and Why is it Needed?

> Above, there was the problem that a `FactoryBean` had to be continuously created for each object (service), leading to duplicate code for object creation.
>

To solve this problem, `ProxyFactoryBean` can be proposed as a solution. `ProxyFactoryBean` is a class that creates proxies and registers objects as beans. It uses the Decorator pattern to easily add additional functionality to any bean dynamically. `ProxyFactoryBean` implements something called `Advice MethodInterceptor`, and unlike `InvocationHandler`, it knows the state of the target object (it takes `MethodInvocation` as an argument) because it operates as a template as a callback object.

Therefore, `ProxyFactoryBean` can flexibly add multiple objects and advices as beans, solving the problem of duplicate code.

Finally, there's still one problem here. Although duplicate code has been reduced, the issue remains that `ProxyFactoryBean` must be continuously defined in XML or config files for each service that uses it.

<br>

### What is DefaultAdvisorAutoProxyCreator, and Why is it Needed?

> As mentioned above, although duplicate code was reduced, the problem remained that `ProxyFactoryBean` had to be continuously defined in XML or config files for each service that uses it.
>

We can solve this problem using a technique called bean post-processor (`BeanPostProcessor`). This is a technique for adding additional operations to a bean after it has been created, and `DefaultAdvisorAutoProxyCreator` is a representative class for this.

Thus, to apply multiple `Advisor`s to appropriate `Pointcut`s, `DefaultAdvisorAutoProxyCreator` helps to flexibly add desired additional functionalities by obtaining appropriate `Pointcut`s (classes to apply to) through pointcut expressions using the `AspectJExpressionPointcut` class.

You can apply it by registering the pointcuts of the relevant classes to be applied as beans along with `DefaultAdvisorAutoProxyCreator` and `Advisor` classes. For example:

- `execution(* minus(int, int))`
    - Allows all return types / Sets a pointcut for the `minus(int, int)` function.

Here, after defining an `Advice` related to `Transaction`, if you register it to apply these features to classes ending with `*ServiceImpl`, all services containing business logic will be able to use transaction-related features.

This is how to apply it using dynamic proxies.
