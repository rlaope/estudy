# Aggregate and Transaction Management

## Aggregate and Transaction

> When two users simultaneously modify an aggregate, consistency is broken.

![](https://camo.githubusercontent.com/1124c1bd5bbab6af788f0aa4c2830cc5a178271920961ad103b3fea7af7bbbb1/68747470733a2f2f333535333234383434362d66696c65732e676974626f6f6b2e696f2f7e2f66696c65732f76302f622f676974626f6f6b2d6c65676163792d66696c65732f6f2f6173736574732532462d4d35484f53747876782d4a723066715a6879572532462d4d43766b674932366a743949326d3679307933253246382e312e706e673f616c743d6d6564696126746f6b656e3d35643530633832322d383830632d343065322d386232652d613963336330356665613365)

A transaction is needed when two users simultaneously modify an aggregate.
Although the operator thread and customer thread conceptually refer to the same aggregate, they physically use different aggregate objects.
In this situation, each thread reflects its modifications to the DB when it commits its transaction.
At this point, the `consistency` of the aggregate is broken.

To prevent consistency from being broken, one of the following two approaches must be taken:
- Prevent the customer from modifying the aggregate while the operator is viewing the shipping information and changing its status.
- If the customer's information changes after the operator has viewed the shipping information, ensure the operator re-fetches the aggregate before making modifications.

In addition to DBMS-supported transactions, there are additional transaction techniques for aggregates: pessimistic locking and optimistic locking.

## Pessimistic Locking

> Pessimistic locking carries the risk of deadlocks.

Pessimistic locking is a method where **the thread that first acquires an aggregate prevents other threads from modifying that aggregate until it finishes using it**.

![](https://camo.githubusercontent.com/e2826dc4a36497e2545e5170e64871108ea2b23a0994516b61b920b68be8d83f/68747470733a2f2f333535333234383434362d66696c65732e676974626f6f6b2e696f2f7e2f66696c65732f76302f622f676974626f6f6b2d6c65676163792d66696c65732f6f2f6173736574732532462d4d35484f53747876782d4a723066715a6879572532462d4d43766b674932366a743949326d36793079332532462d4d43766c5f632d615534446767666e64693461253246382e322e706e673f616c743d6d6564696126746f6b656e3d38663735303930392d353362352d346638342d613134302d663439363064663163326135)

Thread 2 is blocked until Thread 1 releases the lock on the aggregate.
Since Thread 2 acquires the aggregate after Thread 1 commits its transaction, Thread 2 will see the changes made by Thread 1.
Pessimistic locking typically uses row-level locks provided by the DBMS.
It provides a locking mechanism, using queries like `for update`, that allows only one connection to access a specific record.
Spring Data JPA uses the `@Lock` annotation to specify the lock mode.
Hibernate uses a `for update` query when `PESSIMISTIC_WRITE` is used as the lock mode.

### Pessimistic Locking and Deadlocks

> Deadlocks must be carefully managed.

As the number of users increases, more threads fall into a deadlock state, rendering the system unresponsive.
To prevent this, a maximum wait time should be specified when acquiring a lock.
The `javax.persistence.lock.timeout` hint specifies the lock acquisition wait time in milliseconds, raising an exception if exceeded.
Since hints may not be applied depending on the DBMS, it's necessary to check if the relevant functionality is supported.
Spring Data JPA allows specifying query hints using the `@QueryHints` annotation.

## Optimistic Locking

> Optimistic locking verifies if changes are possible using versions.

Pessimistic locking does not solve all transaction conflict issues.

![](https://camo.githubusercontent.com/43afa4c1129bd7dfb31a2baf916e387e12177a46ecdfd76ad5f7cbd6caca3f6a/68747470733a2f2f333535333234383434362d66696c65732e676974626f6f6b2e696f2f7e2f66696c65732f76302f622f676974626f6f6b2d6c65676163792d66696c65732f6o2f6173736574732532462d4d35484f53747876782d4a723066715a6879572532462d4d43766b674932366a743949326d36793079332532462d4d43766c636937735a4c362d44487574677276253246382e342e706e673f616c743d6d6564696126746f6b656e3d30306331326434302d356533662d343039622d383663652d393861626564306136646132)

This describes a situation where a customer changes the shipping address while an operator is viewing the shipping information and changing the delivery status.
Instead of preventing simultaneous access, **optimistic locking** is needed to check if changes are possible at the point when modified data is actually reflected in the DBMS.
A numeric type property to be used as the aggregate version must be added.
> UPDATE aggtable SET version = version + 1, colx = ?, coly = ? WHERE aggid = ? and version = current_version

The query is executed only if the version of the aggregate to be modified matches the current aggregate's version.
If the modification succeeds, the version is incremented by 1; if the version values differ, the modification fails.

![](https://camo.githubusercontent.com/b0e9323dc86afca80507e1538b93878a97808e786cfd680d05fb922fc4d79d64/68747470733a2f2f333535333234383434362d66696c65732e676974626f6f6b2e696f2f7e2f66696c65732f76302f622f676974626f6f6b2d6c65676163792d66696c65732f6f2f6173736574732532462d4d35484f53747876782d4a723066715a6879572532462d4d43766b674932366a743949326d36793079332532462d4d43766c666f623638377973317a56386e7872253246382e352e706e673f616c743d6d6564696126746f6b656e3d61396532656361312d306666302d343636382d383535342d633431306230616463313365)

JPA can implement optimistic locking using the `@Version` annotation.
When using Spring's `@Transactional` annotation, an `OptimisticLockingFailureException` is thrown if a conflict occurs at the end of the transaction.
Applying optimistic locking to the initial conflict scenario results in the following flow:

![](https://camo.githubusercontent.com/583611f1eccee3f2de4c894a23a538b752a9159243ab17d8f52c884bd13e088f/68747470733a2f2f333535333234383434362d66696c65732e676974626f6f6b2e696f2f7e2f66696c65732f76302f622f676974626f6f6b2d6c65676163792d66696c65732f6f2f6173736574732532462d4d35484f53747876782d4a723066715a6879572532462d4d43766b674932366a743949326d36793079332532462d4d43766c6c6150573445304c6e416d69543573253246382e362e706e673f616c743d6d6564696126746f6b656e3d36643836323837662d386366632d343535642d383733632d353461373161323661313133)

```java
@Controller
public class OrderAdminController {
	private StartShippingService startShippingService;

	@RequestMapping(value = "/startShipping", method = RequestMethod.POST)
	public String startShipping(StartShippingRequest startReq) {
		try {
			startShippingService.startShipping(startReq);
			return "shippingStarted";
		} catch(OptimisticLockingFailureException | VersionConflicException ex) {
			// 트랜잭션 충돌
			return "startShippingTxConflict";
		}
	}
	... 
```

The following code handles `OptimisticLockingFailureException` thrown by the Spring framework and `VersionConflicException` thrown by the application service.
`VersionConflicException` indicates that someone has already modified the aggregate, while `OptimisticLockingFailureException` indicates that someone modified it almost simultaneously.

## Forced Version Increment

When an entity other than the aggregate root is changed, JPA does not update the version value because the aggregate root's own value remains unchanged.
However, from an aggregate perspective, if a component of the aggregate changes, the aggregate itself has logically changed.

To handle this issue, JPA uses `LockModeType.OPTIMISTIC_FORCE_INCREMENT` when executing a query to force a version increment at the end of the transaction.

## Offline Pessimistic Locking

> Offline pessimistic locking prevents the modification screen itself from being executed if someone is already viewing it.

To strictly prevent data conflicts, **offline pessimistic locking** prevents the modification screen itself from being executed if someone is already viewing it.
This cannot be implemented with pessimistic locking, which applies only within a single transaction, or with optimistic locking, which checks for version conflicts later.

![](https://camo.githubusercontent.com/533d638e8146b288ef7afc3df73bb7b37cf6b1a2abc38b30e7be679a5da0ae3c/68747470733a2f2f333535333234383434362d66696c65732e676974626f6f6b2e696f2f7e2f66696c65732f76302f622f676974626f6f6b2d6c65676163792d66696c65732f6f2f6173736574732532462d4d35484f53747876782d4a723066715a6879572532462d4d43766b674932366a743949326d36793079332532462d4d43766c694e37554e5031344f687530474a7a253246382e382e706e673f616c743d6d6564696126746f6b656e3d32303139323164382d646439302d343035342d383064632d663061346161303739656363)

In such a situation, if User A does not perform the modification request, the lock will not be released, so a lock expiration time must be set.

Once the lock's validity period expires, the lock should be released to allow other users to acquire it again.

However, if a modification request is performed shortly after the validity period has passed, it will fail.

To prevent this, a mechanism to periodically extend the validity period is required.

## LockManager Interface and Related Classes for Offline Pessimistic Locking

- Attempt to acquire lock
- Verify lock
- Release lock
- Extend lock expiration time

```java
public interface LockManager {
  LockId tryLock(String type, String id) throws LockException;  // Attempt to acquire lock
  void checkLock(LockId lockId) throws LockException;   // Verify lock
  void releaseLock(LockId lockId) throws LockException;   // Release lock
  void extendLockExpiration(LockId lockId, long inc) throws LockException;  // Extend lock expiration time
}
```

Functions executed after acquiring a lock must verify its validity, considering the following situations:
- If the lock's validity period has expired, another user might have acquired the lock.
- If a user who has not acquired the lock attempts to execute a function, the function execution must be prevented.

## Implementing LockManager using DB

Table creation query for storing lock information
```sql
create table locks (
  `type` varchar(255),
  id varchar(255),
  lockid varchar(255),
  expiration_time datetime,
  primary key (`type`, id)
) character set utf8;

create unique index locks_idx ON locks (lockid);
```
