# @Query 어노테이션에서 FK로 객체를 찾는 방법

```java
class User {
    @Id
    private String username;
    
    @ManyToOne
    private School school;
}
```

```java
class Team {
    @Id
    private String teamName;

    @OneToMany
    private Set<User> users;

}
```

```java
@Query(select u from User u where user.teamName = :team)
List<User> findByTeam(@Param('teamName')String teamName);
```
위 Repository interface는 얼핏 보면 그럴싸 해보이지만 실제로 구동을 시키면 500과 함께 application이 죽는다. 그 이유는 team은 객체로 매핑되어있기 때문에 실제로 작동을 시키려면

```java
@Query(select u from User u where user.team.teamName = :team)
List<User> findByTeam(@Param('teamNmae')String teamName);
```

위와 같이 team객체 안에 있는 teamName으로 매핑을 해주어야 teamName으로 FK 객체의 값으로 찾을 수 있다.