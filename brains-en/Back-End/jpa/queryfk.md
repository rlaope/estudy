# How to find an object by FK using the @Query annotation

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
The Repository interface above might look plausible at first glance, but if you actually run it, the application will crash with a 500 error. The reason is that `team` is mapped as an object, so to make it work, you need to:

```java
@Query(select u from User u where user.team.teamName = :team)
List<User> findByTeam(@Param('teamNmae')String teamName);
```

As shown above, you must map it to the `teamName` within the `team` object to find the FK object by its `teamName` value.
