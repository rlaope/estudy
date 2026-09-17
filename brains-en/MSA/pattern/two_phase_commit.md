# Two Phase Commit

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F8zMJw%2Fbtrx0uOU61k%2FAKjHsJeXfH1YAxzMdi6XS0%2Fimg.png)

This method is used in distributed database environments, and its functionality is primarily provided by RDBMSs. Two-Phase Commit, as the name suggests, is the process of persisting data through two phases.

As shown in the figure above, when multiple databases are distributed, there is a coordinator that orchestrates transactions.

The coordinator's role is to manage the transaction process through two phases when a transaction request is received.

The first phase is Prepare, which is essentially the process of asking the involved databases if they are ready to store the data.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FplqLW%2FbtrxXoWua3o%2FjWKzMmFsvJgXnlPvk2EPwK%2Fimg.png)

Upon receiving the message, the databases proceed with preparations for the Commit operation.

Once they are ready to persist the data, they inform the coordinator that they are ready; conversely, if it's not possible, they send a message indicating so.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcPqGWz%2Fbtrx0uBrKXJ%2FEx1ZHPJJqkaIK346CHkZ3k%2Fimg.png)

The coordinator waits for responses to the messages sent in the first phase. Once all messages are received, the second phase, commit, proceeds.

In the Commit phase, the coordinator sends a message to the involved databases to store the data, and the receiving databases persist the data in their respective databases.

Once transaction processing is complete in all databases, the entire transaction is terminated.

If, during the two phases, even one of the involved databases is unable to commit, a Rollback is requested from all databases.

Upon termination of the transaction, all database data is persisted. Therefore, **the scope of the transaction encompasses all databases processing the data**.

<br>

## Issues with Two-Phase Commit in an MSA Environment
It can only be applied if distributed transactions between DBMSs are supported. However, NoSQL product families do not support this, and the DBMSs used together must belong to the same product family.

Therefore, DBMS Polyglot configuration is difficult.

> Polyglot: Refers to supporting multiple languages.
> DBMS Polyglot: A management system that supports various types of databases (MySQL, Oracle, PostgreSQL...).

Furthermore, Two-Phase Commit is typically used when service requests come through a single API endpoint and databases are distributed internally.
However, in an MSA environment, service requests are made through inter-API communication from different applications, making implementation challenging.
