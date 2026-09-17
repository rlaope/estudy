# OSIV (Open Session In View)

## OSIV
It is a feature that keeps JPA's persistence context and Hibernate's session open until the view.

### Reason for Use
It extends the scope of the persistence context, which was previously maintained within services annotated with @Transactional, to include view rendering.

When a service annotated with @Transactional is called, Spring's transaction AOP operates.

Transaction AOP starts a transaction just before calling the method, and if the method completes successfully, it commits and ends the transaction. If an exception occurs, it performs a rollback.

## How it Works
![](https://github.com/leeseojune53/yatudy/raw/main/images/persistent_context.png?raw=true)

1. When a request comes in, the Servlet Filter/Interceptor creates a Persistence Context. However, it does not start a transaction.
2. When starting a transaction with @Transactional in the service layer, it finds the existing Persistence Context and starts the transaction.
3. If the method completes successfully, it commits and ends the transaction. If an exception occurs, it performs a rollback.
4. Since the Persistence Context is maintained up to the controller/view, the retrieved entities remain in a persistent state.
5. When a request comes through the Servlet Filter / Interceptor, the Persistence Context is closed. At this point, it closes immediately without calling flush.

### Caution
If you run the application with `spring.jpa.open-in-view` set to its default value of true, a WARN log will be left.

The reason for the WARN log is that OSIV maintains the persistence context and database connection from the initial database connection point until the API response. This allows for lazy loading in the Controller.

However, OSIV has both advantages and disadvantages. While the aforementioned feature is an advantage, it uses database connection resources for an extended period, which can lead to a shortage of DB connections later. This significantly increases the likelihood of failures and is a critical drawback.

To disable OSIV, you can set `spring.jpa.open-in-view` to false.

Another way to manage complexity, besides OSIV, is Command Query Responsibility Segregation (CQRS).

Simply put, it involves separating read-only transactions from core business logic.
