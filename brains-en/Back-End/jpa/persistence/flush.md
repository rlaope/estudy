# Flush

- Refers to reflecting changes in the persistence context to the DB.
- Flush operates when a transaction commit occurs, at which point the INSERT, UPDATE, and DELETE SQL statements accumulated in the write-behind store are sent to the DB.
  - Caution: It does not clear the persistence context.
- Simply put, it's the process of synchronizing the changes in the persistence context with the state of the DB.
  - Flush synchronizes the changes in the persistence context with the DB.

### Flush Operation Process
1. Detects changes
2. Registers modified entities in the write-behind SQL store
3. Sends queries from the write-behind SQL store to the DB

- Flush occurring does not mean a commit happens; the actual commit occurs after the flush.
- The reason flush can operate is due to the concept of database transactions.
  - Since synchronization only needs to happen just before the transaction commits, the flush mechanism can operate in between.
- JPA fundamentally delegates data consistency and concurrency concerns to database transactions.

### How to Flush the Persistence Context

1. em.flush

```java
// 영속 상태 (Persistence Context 에 의해 Entity 가 관리되는 상태)
Member member = new Member(200L, "A");
entityManager.persist(member);

entityManager.flush(); // 강제 호출 (쿼리가 DB 에 반영됨)

System.out.println("DB INSERT Query 가 즉시 나감. -- flush() 호출 후 --  Transaction commit 됨.");
tx.commit(); // DB에 insert query 가 날라가는 시점 (Transaction commit)
https://gmlwjd9405.github.io/2019/08/07/what-is-flush.html
```

- Q. Does flush clear the entire first-level cache?
  - NO! It remains as is.
  - It's merely the process of sending only the queries in the write-behind SQL store to the DB.

2. Automatic flush call on transaction commit

3. Automatic flush call when executing JPQL queries

```java
em.persist(memberA);
em.persist(memberB);
em.persist(memberC);

// 중간에 JPQL 실행
query = entityManager.createQuery("select m from Member m", Member.class);
List<Member> members = query.getResultList();
```
