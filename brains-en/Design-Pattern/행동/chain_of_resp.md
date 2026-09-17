# Chain of Responsibility Pattern

The Chain of Responsibility pattern is a **design pattern that allows requests to be passed along a chain of handlers**.

For example, let's say we have two handlers: a LoggingHandler (a handler that logs user information) and an AuthHandler (a handler that authenticates login status), both taking a Request (user information) object as an argument.

If a client needs to authenticate login status before logging information received via a Request through a LoggingHandler, it would have to perform validation using an `if` statement on the client side. (Alternatively, like the Decorator pattern, you could pass a RequestHandler (the parent class of handlers) as an argument during creation to handle authentication and authorization.) Therefore, if there are multiple such handlers, the logic for using handlers on the client side will inevitably grow.

A solution here is for each handler to have a `chain` field (injected via the constructor) representing the next handler to perform an action, and to call this `chain` after processing its own task.

This way, you can freely adjust and customize the desired operations within the chain.

### Example

```java

class Client {

    fun main() {
        val requestHandler = LoggingHandler(AuthHandler(null))
        requestHandler.doWork()
    }
}

```

As shown above, the last handler can pass `null` to avoid specifying the next chain.

### Advantages

An advantage is that various logics can be customized and configured freely, making it easy to add new features.

A disadvantage is that debugging is difficult (because multiple handlers are irregularly passed as arguments).

<br>

This Chain of Responsibility pattern can be found in Spring and Spring Boot, notably in `Filter` and `SecurityConfig`.

In the `Filter` class, `chain.doFilter()` means to pass control to the next operation.

In `SecurityConfig`, logic such as registering subsequent filters with `http.something.and().filterAfter()` can also be seen as an application of the Chain of Responsibility pattern.
