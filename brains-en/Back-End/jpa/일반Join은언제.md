# When to Use a Regular Join?

When looking at solutions for the N+1 problem or the advantages of fetch joins, fetch joins generally seem better than regular joins.

However, regular joins definitely have their uses. Since JPA fundamentally requires careful consideration of DB <-> object consistency, only entities strictly necessary for the logic should be kept in the persistence context.

Therefore, rather than indiscriminately using fetch joins to load everything into the persistence context,
it's a good way to prevent unnecessary malfunctions by appropriately using regular joins to load only the necessary entities into the persistence context.

In cases like the example below, a regular Join is much more effective than a Fetch Join.

## Example
Retrieve the Team to which a member named 'team2member4' belongs (member's information is not needed)

This is a situation where an associated entity is **needed for the query search condition but its actual data is not required**.

As explained previously, regular joins do not load the joined target into the persistence context.

This characteristic of regular joins seems suitable for the situation that needs to be resolved in this example.

Let's execute the code below.
> (The JPQL used in the example below is for illustrative purposes, and JPQL that applies conditions to collections like this should be avoided.)

```java
// TeamRespotory.java
@Query("SELECT distinct t FROM Team t join t.members m where m.name = :memberName")
public List<Team> findByMemberNameWithMemberUsingJoin(String memberName);

// TeamService.java
@Transactional
public List<Team> findByMemberNameWithMemberUsingJoin(String memberName){
  return teamRepository.findByMemberNameWithMemberUsingJoin(memberName);
}

// FetchJoinApplicationTests.java
@Test
public void joinConditionTest() {
  List<Team> memberUsingJoin = teamService.findByMemberNameWithMemberUsingJoin("team2member4");
  System.out.println(memberUsingJoin);
}
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlCj8n%2Fbtq8pifD5WD%2FFdNyoF5CwXkAoo3O4unzn0%2Fimg.png)

Actually, the above example throws a `LazyInitializationException` for the same reason as the previous regular join example, but if we set a breakpoint and check the result,

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcylBwd%2Fbtq8ojy6Cw3%2FX0kFTGYOfjsdwK7qKrCMg1%2Fimg.png)

A Team named 'team2' containing a member named 'team2member4' was retrieved.

As expected, the Member used in the regular join shows an uninitialized state.

Thus, it can be seen that **actively using regular joins is efficient** when an associated Entity is only used in search conditions.
