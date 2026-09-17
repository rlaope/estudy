# Managing Distributed Transactions in Microservices

There are broadly two ways to manage distributed transactions.

- Two Phase Commit Protocol
- Eventual Consistency and Compensation (SAGA Pattern)

### Two Phase Commit Protocol

It is an ACP (Atomic Commit Protocol) that confirms information has been successfully modified in transaction processing and database computer networking.

It provides a distributed algorithm that verifies transaction success and failure and coordinates these operations to occur atomically.

Looking at the operation process of 2PC (Two Phase Commit), a **Coordinator, which is the transaction manager**, is required, and the remaining nodes are called cohorts (or participants).

![](https://camo.githubusercontent.com/9cb280c73cd9b93c311f1f55c8a6cef42b4cfee391ce50d5f4cd50c60c730cee/68747470733a2f2f646f63732e676f6f676c652e636f6d2f64726177696e67732f642f31684c73353650356e77675a566e5266764845397747386e4e4479425f366b475f6443546b394d6f305a304d2f7075623f773d35343326683d353230)

2PC is divided into two phases: the work request phase and the commit phase.

#### Prepare Phase
1. The Coordinator sends a 'query to commit' message to the cohorts and waits for their responses.
2. Cohorts set transaction points, proceed, and then prepare to commit. If a cohort fails, it prepares redo logs and undo logs for rollback.
3. Each cohort sends an agreement message, indicating whether the operation succeeded or failed.

#### Commit Phase - Success

If success agreement messages are received from all cohorts, it's a success, and the commit is executed.
1. The Coordinator sends a commit message to all cohorts.
2. Each cohort releases resource locks after committing and then sends an acknowledgment to the coordinator.
3. The operation is complete when acknowledgments are received from all cohorts.

#### Commit Phase - Failed

If a failure agreement message is received from one or more cohorts, or if a timeout occurs, it's a failure, and a rollback is performed.

1. The coordinator sends a rollback message to all cohorts.
2. Each cohort rolls back using undo logs, releases resource locks, and then sends an acknowledgment.
3. The transaction is recovered when acknowledgments are received from all cohorts.


Since 2PC is a Blocking Protocol, if the Coordinator permanently fails, some cohorts may never resolve their transactions. Furthermore, 2PC can only be applied if distributed transactions between DBMSs are supported, which NoSQL databases do not support, and the DBMSs used together must be identical.

Also, 2PC is typically used when service requests come through a single endpoint and the database is internally distributed. In contrast, in an MSA environment, applications are distributed, and service requests are made through API communication between different applications, making 2PC difficult to implement.

<br>

### Saga Pattern

**Unlike 2PC, Saga's transaction management is handled by the application, not the DBMS.** In environments like MSA where applications are distributed, each application is only responsible for processing local DB transactions. Therefore, if a series of transaction requests fail for any application, rollback processing must be implemented at the application level.

**There are two types of Saga patterns: Choreography-based Saga and Orchestration-based Saga.**

#### Choreography-based Saga

It is implemented by **each service managing its local transaction**, changing its current state, and upon completion, emitting a completion event. This event is then passed to the next service managing the transaction to process it. If a transaction needs to be rolled back, it manages the transaction by emitting a **compensation event**, allowing a compensation transaction to be executed.

Product order example:
- commit
    - The order service creates an order, sets it to a pending state, and then publishes an `order created` event.
    - The customer service consumes the `order created` event, creates credit, and then generates a `credit reserved` event.
    - The order service receives the `credit reserved` event and changes the pending order to `approved`, allowing the transaction to be committed.
- rollback
    - The order service creates an order, sets it to a pending state, and then generates an `order created` event.
    - If the customer service receives the `order created` event but cannot create credit due to a credit limit, it generates a `credit limit exceeded` event.
    - The order service receives the `credit limit exceeded` event and changes the pending order to `reject`, rolling back the transaction.

The advantage of this implementation is a performance benefit due to the absence of separate orchestration, as it doesn't require creating instances or a dedicated orchestrator service. Consequently, it's easier to implement and understand the concept.

However, a disadvantage is that adding a transaction scenario increases the management overhead. It also becomes difficult to infer which service sends and receives which events, and all services must listen to events from every service they interact with.

#### Orchestration based Saga

There are multiple services, each with a single responsibility, and an orchestrator responsible for handling transactions between these services. Unlike choreography-based saga, where each service must listen to events from other services, the orchestrator is responsible for listening to all service events and triggering endpoints.

![](https://camo.githubusercontent.com/b199d7296ddb2085512dceae22fc5afa5d47406fa4e29726823f0f9aed94e707/68747470733a2f2f6d69726f2e6d656469756d2e636f6d2f6d61782f3638332f312a4f78666462667358324d377172763557735358414d672e706e67)

In the diagram above, the order orchestrator communicates with each service using a command/reply pattern.

In an orchestration-based saga, it's evident that the orchestrator knows the entire flow of a transaction. If a transaction error occurs, the orchestrator is also responsible for rolling back everything that happened previously due to that error.

Since the orchestrator can be viewed as a state machine where each transition corresponds to a command or message, one way to implement an orchestration-based saga is to apply the `State Machine Pattern`. The State Machine Pattern is easy to implement, making it a good pattern for structuring well-defined behaviors.

Orchestration-based saga is easy to maintain because only the orchestrator needs to be changed if the transaction scenario changes. Also, since it communicates with all services, circular dependencies between services can be avoided.

However, it has the disadvantage of being difficult to implement, and if a lot of transaction-related business logic accumulates in the orchestrator, maintenance will become very challenging. Therefore, when implementing an orchestration-based saga, it's best to manage it so that only logic related to transaction order is written (only command or reply).
