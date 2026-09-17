### Transaction and ACID

### Transaction
This is a unit that groups multiple operations together. All operations in a single block either execute completely or do not execute at all.

![](./Image/transaction.png)

#### Why Transactions Are Needed
For example, imagine transferring money from Bank A to Bank B, which involves withdrawing from A and depositing into B. What if the system suddenly crashes after the money is withdrawn from Bank A but before it's deposited into Bank B? A terrible situation would occur where the money is withdrawn but not deposited, effectively disappearing.

Transactions ensure that such situations do not happen.

Most databases offer various methods to prevent incomplete transfers, but the most fundamental method they commonly provide is ensuring data validity through transactions.

### ACID
What does ACID mean? Is it `acid`?

- ACID is an acronym for the characteristics of transactions that ensure data validity.

#### Atomicity
This characteristic means that all operations are either fully committed or fully rolled back.

#### Consistency
This characteristic means that data can only be modified according to predefined rules. It ensures that, for example, string values cannot be stored in a numeric column.

#### Isolation
This refers to the degree to which the operations of transaction A are visible to transaction B when both are executing concurrently.

#### Durability
This characteristic means that once a transaction is committed, its changes are permanently applied.
