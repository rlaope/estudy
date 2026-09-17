# 일반 조인 Join은 언제 사용될까?

N+1 문제 해결 방법이나 패치 조인의 장점을 알아봤을 때 일반적으로 패치 조인이 일반 조인보다 더 좋아 보인다.  
  
그러나 일반 조인의 쓰임새도 분명 있는데, JPA는 기본적으로 DB <-> 객체의 일관성을 잘 고려해서 사용해야하기 때문에 로직에 꼭 필요한 엔티티만을 영속성 컨텍스트에 담아놓고 사용해야합니다.  
  
그러니 무작정 패치 조인을 사용해 전부 영속성 컨텍스트에 올려서 쓰기 보다는  
일반 조인을 적절히 이용해 필요한 엔티티만 영속성 컨텍스트에 올려서 사용하는 것이 괜한 오작동을 미리 방지할 수 있는 방법이기도 합니다.  
  
아래 예제와 같은 경우에는 Fetch Join보다는 일반 Join이 훨씬 더 효과적입니다.  

## 예제
team2member4라는 이름을 가지는 member가 속해있는 Team 조회 (멤버의 정보는 필요하지 않음)
  
연관 관계가 있는 엔티티가 **쿼리 검색 조건에는 필요하지만 실제 데이터는 필요하지 않은 상황**입니다.  
  
이전 설명에서 일반 조인은 조인 대상에 대해서는 영속성 컨텍스트에 담지 않는다고 했습니다.  
  
일반 조인의 이런 특성은 이번 예제에서 해결해야 할 상황에 적합해 보입니다.  
  
아래 코드를 실행해 보겠습니다.
> (아래 예제에서 사용된 JPQL은 예제를 위해 사용한 JPQL이며 아래처럼 컬렉션에 조건을 주는 형태의 JPQL은 지양되어야 합니다.)

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

사실 위 예제는 이전 일반 조인 예제와 동일한 이유로 `LazyInitializationException`이 발생합니다만, breakpoint를 걸어 결과를 확인해보면

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcylBwd%2Fbtq8ojy6Cw3%2FX0kFTGYOfjsdwK7qKrCMg1%2Fimg.png)

team2member4 라는 이름을 가진 member가 포함된 team2라는 Team이 조회가 되었습니다.  
  
역시나 일반 join에 사용된 Member는 초기화되지 않았다는 상태를 보여주고 있습니다.

이처럼 검색조건에만 연관 Entity가 사용될 경우에는 **일반 join을 적극적으로 사용하는 것이 효율적**임을 알 수 있습니다.

