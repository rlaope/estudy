# Differences Between Regular Join and Fetch Join

## Differences

### Regular Join
- Unlike Fetch Join, even if you join an associated entity, the actual query only selects and persists **the entity that is the subject of the JPQL query**.
- Since only the entity that is the subject of the query is selected and persisted, it is mainly used when data is not needed but the associated entity is required for search conditions.

### Fetch Join
- In addition to the entity that is the subject of the query, associated entities involved in a Fetch Join are also selected and **all are persisted**.
- Since all entities involved in a Fetch Join are persisted, even if a `FetchType.LAZY` entity is referenced, it is already in the persistence context, thus resolving the N+1 problem without executing additional queries.

## Verifying Differences Between Join and Fetch Join

### Test Entities
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdL7LCv%2Fbtq8spj3TkP%2FKSpaDWDV8ry4k4SsSepMe1%2Fimg.png)

```java
@Builder
@Setter
@Getter
@NoArgsConstructor
@AllArgsConstructor
@ToString
// @ToString(exclude = "members") 
@Entity
public class Team {
  @Id
  @GeneratedValue(strategy = GenerationType.IDENTITY)
  private long id;
  
  private String name;
  
  @OneToMany(mappedBy = "team", fetch = FetchType.LAZY, cascade = CascadeType.PERSIST)
  @Builder.Default
  private List<Member> members = new ArrayList<>();

  public void addMember(Member member){
    member.setTeam(this);
    members.add(member);
  }
}
```

```java
@Builder
@Setter
@Getter
@NoArgsConstructor
@AllArgsConstructor
@ToString(exclude = "team")
@Entity
public class Member {
  @Id
  @GeneratedValue(strategy = GenerationType.IDENTITY)
  private long id;
  
  public String name;
  
  public int age;
  
  @ManyToOne(fetch = FetchType.LAZY)
  public Team team;
  
  public Member(String name, int age, Team team) {
    this.name = name;
    this.age = age;
    this.team = team;
  }
}
```

### Resolving N+1 with Regular Join
When querying Team and Member using a regular join, the corresponding executed query is as follows:

```java
// TeamRepository.java
@Query("SELECT distinct t FROM Team t join t.members")
public List<Team> findAllWithMemberUsingJoin();
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNLsjQ%2Fbtq8meq42aC%2FH0tuDcChKiJUkWYK6uXc00%2Fimg.png)

A query in the form of a join between Team and Member, as one would typically expect, is executed.

A peculiar point is that if you look at the columns being retrieved, **only the Team's columns, `id` and `name`**, are fetched.

Let's print the result of the join using `toString()` in this state.

```java
// TeamService.java
@Transactional
public List<Team> findAllWithMemberUsingJoin(){
  return teamRepository.findAllWithMemberUsingJoin();
}

// FetchJoinApplicationTests.java
@BeforeEach
public void init(){
  teamService.initialize();
}

@Test
public void joinTest() {
  List<Team> memberUsingJoin = teamService.findAllWithMemberUsingJoin();
  System.out.println(memberUsingJoin);
}
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb4H0A5%2Fbtq8mYBzggk%2Fn0ivKiQDgxTFiEmB38b1q0%2Fimg.png)

Although the query was executed as a join, a `LazyInitializationException` suddenly occurs.

> `LazyInitializationException` typically occurs when a lazy entity is used without a Session (Transaction).

If we set a breakpoint, we can understand why the `LazyInitializationException` occurred.

```java
@Test
public void joinTest(){
    List<Team> memberUsingJoin = teamService.findAllWithMemberUsingJoin();
    //break point
    System.out.println(memberUsingJoin)
}
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F84NBS%2Fbtq8reQQGn4%2FTgwYKnrXFv1TJJwDJQFKWk%2Fimg.png)

Looking at the actual query, even though a join was performed, it shows that the `members` (a lazy entity of each Team) have not yet been initialized.

**In reality, a regular join does apply a join to the actual query, but it does not concern itself with the persistence of the joined entities.**

It only performs the join, and only the selected entities are placed in the persistence context.
Based on the above, reconsidering the situation, the `LazyInitializationException` occurs under the following circumstances:

1. Team entities are initialized via a regular join.
2. However, since a regular join does not initialize associated entities, Member entities are not initialized.
3. Accessing the uninitialized `members` via `toString()` causes a `LazyInitializationException`.

> In fact, if `@ToString(exclude="members")` is set on Team, `members` will not be accessed, and `LazyInitializationException` will also not occur.

## Resolving N+1 with Fetch Join
The code using Fetch Join and the corresponding executed query are as follows:

```java
// TeamRepository.java
@Query("SELECT distinct t FROM Team t join fetch t.members")
public List<Team> findAllWithMemberUsingFetchJoin();
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdvWdhc%2Fbtq8re4obM3%2FZo1R2qrCy3lnhYZKzKREc0%2Fimg.png)

Although the join form is identical to a regular join, differences are visible starting from the SELECTed columns.

- Regular Join: SELECTs only the columns of the target entity being queried, excluding join conditions.
- Fetch Join: SELECTs columns of both the target entity being queried and the entities involved in the Fetch Join.

As shown above, there is a difference between regular join and Fetch Join even in the columns used in the query.

If we print the execution result of Fetch Join using `toString()`, we can see that all Teams and Members are included, as shown below.

```java
// TeamService.java
@Transactional
public List<Team> findAllWithMemberUsingFetchJoin(){
  return teamRepository.findAllWithMemberUsingFetchJoin();
}

//FetchJoinApplicationTests.java
@Test
public void fetchJoinTest() {
  List<Team> memberUsingFetchJoin = teamService.findAllWithMemberUsingFetchJoin();
  System.out.println(memberUsingFetchJoin);
}
```

Execution Result
```
[
    Team(
        id=1,
        name=team1,
        members=[
            Member(
                id=1,
                name=team1member1,
                age=1
            ),
            Member(
                id=2,
                name=team2member2,
                age=2
            ),
            Member(
                id=3,
                name=team3member3,
                age=3
            )
        ]
    ),
    Team(
        id=2,
        name=team2,
        members=[
            Member(
                id=4,
                name=team2member4,
                age=4
            ),
			Member(
                id=5,
                name=team2member5,
                age=5
            )
        ]
    )
]
```
