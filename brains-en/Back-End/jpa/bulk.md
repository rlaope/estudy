# Bulk Update, Delete Operations and the Persistence Context

Let's assume that after performing bulk update and bulk delete operations, the persistence context was not explicitly initialized.
  
If a full retrieval operation is performed afterwards, you will observe that for updates, data from the persistence context (where the operation was not applied) is retrieved, and for deletes, data from the database (where the operation was applied) is retrieved. (Repeatable Read)

## Bulk Update Operation Scenario

Let's say the persistence context and the database contain the following data.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F0fNXl%2FbtrMeQwfudb%2FSrc85Tgg3MWHLzny58oyX0%2Fimg.png)

In the situation above, let's perform a bulk update operation to change the names of all members under 28 to 'non-member', as shown below.

```java
@Test
@Commit
public void bulkUpdate() {
    queryFactory
            .update(member)
            .set(member.username, "비회원")
            .where(member.age.lt(28)) // 28살 미만은 이름을 전부 비회원으로 바꿔라
            .execute();
 
    final List<Member> result = factory
            .selectFrom(member)
            .fetch();
 
    assertThat(result.get(0).getUsername()).isEqualTo("비회원");
}

```

Bulk operations send queries directly to the actual database, not to the persistence context.
  
Therefore, after performing the operation, you must initialize the persistence context with `em.clear()`.
  
Otherwise, the persistence context and the database will contain different data.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FtJE4F%2FbtrMejSZ39d%2F0Lvlyd9coZjjRCvL7lbFuK%2Fimg.png)
  
In the test code above, no separate persistence context initialization was performed after the bulk update operation.
  
If the QueryDSL `selectFrom()` function is executed in this state, `member1` will be retrieved with its original name 'member1' instead of 'non-member', because the value is fetched from the persistence context's first-level cache.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNiDWh%2FbtrMdE4HILQ%2FIO0zVbHPKNRlAnhp1JW3n1%2Fimg.png)

To solve this problem, let's initialize the persistence context with `em.clear()`.

## Bulk Delete Operation Scenario

Let's say we perform a bulk operation to delete all data for members over 18, as shown below.

```java
queryFactory
        .delete(member)
        .where(member.age.gt(18)) // 18살 초과는 모두 지워라
        .execute();

```
This is a bulk delete query that removes multiple data entries with a single query.
  
Bulk operations send queries directly to the actual database, not to the persistence context.
  
Therefore, after this operation is performed, the state will be as follows.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F5T2ur%2FbtrMdMOZ9Yv%2F0qmv3kVs4ksUwEjIB5kOSk%2Fimg.png)

Similar to before, let's test the result by executing QueryDSL's `selectFrom()` as follows.

```java
@Test
public void bulkDelete() {
    queryFactory
            .delete(member)
            .where(member.age.gt(18)) // 18살 초과는 모두 지워라
            .execute();
 
    final List<Member> result = queryFactory
            .selectFrom(member)
            .fetch();
 
    assertThat(result).hasSize(4);
}
```

Since the delete query is not reflected in the persistence context's first-level cache, all data still exists, so the size of the `List<Member>` returned by `findAll` is expected to be 4.
  
However,

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fk9YCv%2FbtrMcRpIT1O%2Fs1GAk4ZUHRoxgCLTOE1BA0%2Fimg.png)

We expected 4 data entries, but only 1 was actually returned.
Even though all 4 data entries clearly remained in the persistence context.

<br>

## Solution for Bulk Delete Operation Scenario and Persistence Context Principles
We could form the following hypothesis.

> It's an absurd situation where bulk operations, when deleting, are also reflected in the persistence context.

This hypothesis is not correct.
  
It is correct that the persistence context still contains all 4 data entries.
  
This can be confirmed with the following test.

```java
@Test
public voidbulkDelete() {
        factory
                    .delete(member)
                    .where(member.age.gt(18)) // 18살 초과는 모두 지워라
                    .execute();
        
        // em.find -> JPQL이 아니므로 영속성 컨텍스트에서 member3을 찾는다
        final Member findMember3 = em.find(Member.class, member3.getId());
 
    assertThat(findMember3.getUsername()).isEqualTo("member3");
}
```

Executing `EntityManager`'s `find` function searches for `member3` in the persistence context.
  
`member3` is 30 years old, and thus should be deleted by the bulk delete operation.
  
If `member3` were not in the persistence context after that operation, this test should fail. But it succeeded...

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Frvo7e%2FbtrMeE3MAfX%2Fr265ryLF4Kw1IJtOcOwtgk%2Fimg.png)

This is because `member3` was successfully retrieved from the persistence context as shown above.
Therefore, it is correct that the persistence context still contains 4 data entries, with the delete query not being reflected.
But why does QueryDSL's `selectFrom()` operation only retrieve 1 data entry?
  
First, QueryDSL's `selectFrom()` operation, like Spring Data JPA's `findAll()`, issues a JPQL query.
  
JPQL operates differently from JPA's `findById` or `EntityManager`'s `find()` function, which search the persistence context's first-level cache to ensure identity.
  
When JPQL is called, it operates as follows:

1.  It first queries the actual database.
2.  It stores the values retrieved from the DB into the persistence context.
3.  When storing, if an entity with the unique identifier already exists in the persistence context, to ensure JPA's identity, it discards the data from the DB and returns/uses the data from the persistence context.

Therefore, when `selectFrom()` in QueryDSL is executed, it calls JPQL to first query the actual database. An entity corresponding to `member1` exists in the actual database, and this value is stored in the persistence context.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbsyaXn%2FbtrMadUDhER%2F4IaGpUeKVCPPoSeCIXubw0%2Fimg.png)

At this point, since the entity corresponding to `member1` exists in the persistence context, the data from the persistence context is used instead of the data from the actual database.
  
`member2`, `member3`, and `member4` do not exist in the actual database, so they are not included in the query target when checking the persistence context. Therefore, the final result of `selectFrom` returns only `member1`.
  
Thus, the result of this test is 1, not 4.

<br>

## Solution for Bulk Update Operation Scenario

How did the update operation work?

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbSLD3Q%2FbtrMd51NFXN%2FtbZJbuHDdp0L4921jGKdrk%2Fimg.png)

In the case of an update, the entity was not deleted, so data remains in both the actual database and the persistence context.
  
Therefore, when JPQL is called, it operates on the following principle:

1.  It first queries the actual database.
2.  It finds the 4 data entries existing in the actual database within the persistence context. Since the unique identifiers do not change with a bulk update operation, all 4 data entries are found in the persistence context.
3.  Therefore, it returns the 4 data entries from the persistence context that were not updated, instead of the data from the actual database.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fcgsus1%2FbtrMaIUrHMl%2Fb6n9ecQ4LFdZFdikGTO4hk%2Fimg.png)

## Conclusion
Bulk operations send queries directly to the actual database, not to the persistence context.
If you don't properly understand the principles of the persistence context, you might not understand the results.
Make sure to thoroughly study the principles of the persistence context.
Also, if you use bulk operations, don't forget to initialize the persistence context with `em.clear()`, and if there are other additional operations, to call `em.flush()`.
