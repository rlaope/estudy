# JPA Propagation Transaction Propagation Steps

JPA Propagation is an option that can be selected when calling another transaction during a transaction operation.

Through the propagation attribute of @Transactional, the called transaction can either reuse the caller's transaction or create a new one.

`REQUIRED`: This is the default value. It executes within a parent transaction, and if no parent transaction exists, it performs a new transaction.
`REQUIRED_NEW`: If a transaction already exists, it suspends it and creates a new one.

In addition to these, there are types such as REQUIRED_NEW, SUPPORTS, MANDATORY, NOT_SUPPORT, NEVER, and NESTED.

- MANDATORY: Requires a transaction. If no transaction is in progress, it throws an exception.
- SUPPORTS: Does not require a transaction. If a transaction is in progress, it uses it.
- NOT_SUPPORTS: Does not require a transaction. If a transaction is in progress, it suspends it and resumes the suspended transaction after the method completes.
- NEVER: Does not require a transaction. If a transaction is in progress, it throws an exception.
- NESTED: If a transaction is in progress, it executes the method within a nested transaction of the existing transaction.
