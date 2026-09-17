# Saga Pattern

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FLiCWK%2Fbtrx07FIYlZ%2F3Jt15icBW8abKVlW40UKd1%2Fimg.png)

In the Saga pattern, the application, not the DBMS, is responsible for managing transactions. When an app is distributed, the DBs under each app are only responsible for handling local transactions.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlM0vB%2Fbtrx1ITg4Nj%2F4JO3jSBcbbsug1cc2n8hxk%2Fimg.png)

Therefore, in the event of consecutive transaction requests or failures for each app, the application must implement rollback processing (compensating transactions).

As shown in the figure above, the Saga pattern consists of a series of update operations, and the entire data is not persisted simultaneously but rather transactions occur in sequential steps.

Thus, when the final transaction required by the application's business logic is completed, it recognizes that the data has been fully persisted and terminates the process.

Unlike Two Phase Commit, transactions using Saga do not guarantee data isolation. However, by managing transactions at the application level, eventual consistency can be achieved, allowing for consistency among distributed databases.

Furthermore, since transaction management is handled solely by the application, there's an advantage in being able to configure DBMSs from different product families.

However, to achieve such consistency, it's crucial to meticulously check for any missing tasks during the process execution and design the system so that there are no omissions in compensating transaction processing for error recovery in case of failure.

<br>

## Types of Saga Patterns

### Choreography-Based Saga

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbJRsvG%2Fbtrx1HthITO%2F1RmKKioRKJYEILMpuhusO1%2Fimg.png)

Choreography-Based Saga manages local transactions within its own service and publishes an event when a transaction completes.

If there's a subsequent transaction to be performed, the app responsible for that transaction must receive a completion event and then process the next task.

At this time, events can be delivered asynchronously using a message queue service like Kafka.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbQWoIC%2Fbtrx1Jdy2pS%2FgmtWWcOH2AkuQw3Hl71k9k%2Fimg.png)

Each app has its own logic for managing transactions. Therefore, if a transaction fails midway, the failing app attempts a rollback by emitting a compensating event to cancel that transaction.

While such a configuration is easy to build, it's difficult for an operator to know the current state of a transaction.

### Orchestration-Based Saga

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdExzXU%2FbtrxXoWwu3A%2Fe2Yswn7fe6Kb4slODML6k1%2Fimg.pnghttps://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdExzXU%2FbtrxXoWwu3A%2Fe2Yswn7fe6Kb4slODML6k1%2Fimg.png)

In Orchestration-Based Saga, a separate Saga instance (Manager) exists for transaction processing.

All apps involved in a transaction progressively execute transactions under the Manager's direction and deliver the results to the Manager.

When the final transaction in the business logic completes, the Manager terminates, ending the entire transaction process.

If a failure occurs midway, the Manager triggers a compensating transaction to maintain consistency.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcebMKJ%2FbtrxYbWV7K6%2FtxEgJYtWdXRovRYiklw7xK%2Fimg.png)

Since the Manager orchestrates all management, distributed transactions are centralized.

This reduces complexity between services and makes implementation and testing relatively easier.

Another advantage is that rollbacks are easier because the Manager is aware of the current state of the transaction.

However, a disadvantage is the increased complexity of infrastructure implementation due to the need for an additional Orchestrator service to manage this.
