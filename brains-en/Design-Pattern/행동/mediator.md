# Mediator Pattern

The Mediator pattern is a design pattern that allows for reduced interdependencies between components by entrusting their interactions to a mediator.

## Example

For example, let's say we have a `Customer` class, `Restaurant`, and `TowelService`.

A `Customer` can have dinner at the `restaurant` and receive a towel through the `TowelService`.

In that case, `Customer` would implement methods like `getTowel` and `dinner` by depending on those two classes, which increases coupling due to mutual dependency.

By introducing a `FrontDesk` mediator that takes the `customer` as an argument and processes tasks based on `customer` information using the `restaurant` and `towelService`, we can eliminate the dependency between the `customer` and other services.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FrkCOt%2FbtqwA3d73qA%2FWzBqVmAnHxKxgR3W3T6Zc0%2Fimg.png)

### Advantages

An advantage is that it reduces mutual dependencies, allowing for the creation of loosely coupled code.

Even if many classes are introduced, the `Client` only needs to know about the mediator, making it easy to extend.

### Disadvantages

While it allows for easy extension and the creation of highly coupled class structures, one disadvantage is the **complexity of the mediator class**. This is because the mediator class holds all dependencies on related services.

However, when comparing this issue with the benefits gained from the Mediator pattern, I believe that using the Mediator pattern offers more advantages, making it a worthwhile pattern to apply.

<br>

### Example in Spring

In Spring, the `DispatcherServlet` (front controller) class is a prime example developed based on this pattern.

It acts as a mediator for requests and responses in Spring MVC, similar to `handlerMapping`.
