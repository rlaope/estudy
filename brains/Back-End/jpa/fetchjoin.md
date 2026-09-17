# 일반 조인(Join)과 패치 조인(Fetch Join)의 차이

## 차이점

### 일반 Join
- Fetch Join과는 달리 연관 엔티티에 Join을 걸어도 실제 쿼리에서 SELECT 하는 엔티티는 **오직 JPQL에서 조회하는 주체가 되는 엔티티만 조회하여 영속화**
- 조회의 주체가 되는 엔티티만 SELECT 해서 영속화하기 때문에 데이터는 필요하지 않지만 연관 Entity가 검색 조건에는 필요한 경우에 주로 사용됨

### Fetch Join
- 조회의 주체가 되는 Entity 이외에 Fetch Join이 걸린 연관 엔티티도 함께 SELECT 하여 **모두 영속화**
- Fetch Join이 걸린 Entity 모두 영속화하기 때문에 FetchType이 lazy인 엔티티를 참조하더라도 이미 영속성 컨텍스트에 들어있기 때문에 따로 쿼리가 실행되지 않은 채로 N+1 문제가 해결됨

## Join, Fetch Join 차이점 검증 Test

### 테스트 엔티티
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

### 일반 조인을 이용한 N+1 해결
Team과 Member를 조회할 때 일반 조인을 이용하는 경우와 이에 따른 실행 쿼리는

```java
// TeamRepository.java
@Query("SELECT distinct t FROM Team t join t.members")
public List<Team> findAllWithMemberUsingJoin();
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNLsjQ%2Fbtq8meq42aC%2FH0tuDcChKiJUkWYK6uXc00%2Fimg.png)

일반적으로 생각하는 Team과 Member가 join 된 형태의 쿼리가 실행되기는 합니다.  
  
특이한 점은 가져오는 컬럼들을 보면 **Team의 컬럼인 id, name만을** 가져오고 있습니다.  
  
이 상태에서 join을 이욯나 결과를 toString()으로 출력해보겠습니다.

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

쿼리는 Join이 된 형태로 실행되었지만 갑자기 `LazyInitializationException`이 발생합니다.  
  
> LazyInitializationException은 일반적으로 Session(Transaction) 없이 Lazy Entity를 사용하는 경우가 주 원인입니다.) 
  
breakpoint를 찍어보면 왜 `LazyInitalizationException`이 발생했는지 알 수 있습니다.

```java
@Test
public void joinTest(){
    List<Team> memberUsingJoin = teamService.findAllWithMemberUsingJoin();
    //break point
    System.out.println(memberUsingJoin)
}
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F84NBS%2Fbtq8reQQGn4%2FTgwYKnrXFv1TJJwDJQFKWk%2Fimg.png)

쿼리를 실제로 보면 join을 했는데 각 Team의 Lazy Entity인 members가 아직 초기화되지 않았다는 상태를 보여줍니다.  
  
**실제로 일반 join은 실제 쿼리에 join을 걸어주기는 하지만 join 대상에 대한 영속성까지는 관여하지 않습니다.**  
  
오직 join만 걸고 실제 영속성 컨텍스트에는 SELECT 대상만 담게 됩니다.  
위 내용을 기반으로 해서 다시 생각해보면 아래와 같은 정황으로 `LazyInitalizationExcepion`이 발생하는 것 입니다.

1. 일반 조인으로 Team 엔티티 초기화 완료
2. 하지만 일반 조인은 연관 엔티티 까지 초기화하지 않기 때문에 Member는 초기화 되지 않음
3. toString()으로 아직 초기화되지않은 members에 접근하면서 LazyInitalizationException이 발생

> 실제로 Team에 @ToString(exclude="memberS")를 설정하게 되면 members에 접근하지 않게되고 LazyInitalizationException 또한 발생하지 않습니다.

## Fetch Join을 이용한 N+1 해결
Fetch Join을 사용하는 코드와 이에 따라 실행되는 쿼리는 아래와 같습니다.

```java
// TeamRepository.java
@Query("SELECT distinct t FROM Team t join fetch t.members")
public List<Team> findAllWithMemberUsingFetchJoin();
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdvWdhc%2Fbtq8re4obM3%2FZo1R2qrCy3lnhYZKzKREc0%2Fimg.png)

일반 조인과 join의 형태는 똑같지만 SELECT하는 컬럼에서 부터 차이가 보인다.

- 일반 Join: join 조건을 제외하고 실제 질의하는 대상 엔티티에 대한 컬럼만 SELECT
- Fetch Join: 실제 질의하는 대상 엔티티와 Fetch Join이 걸려잇는 엔티티를 포함한 컬럼과 함께 SELECT

위처럼 쿼리에 사용되는 컬럼부터가 일반 join과 Fetch Join에 차이가 있습니다.  
  
Fetch Join의 실행결과를 toString()으로 출력해보면 아래와 같이 모든 Team과 Member가 담긴 것을 확인할 수 있습니다.

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

실행 결과
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