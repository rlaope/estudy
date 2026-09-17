# 벌크 수정, 삭제 연산과 영속성 컨텍스트

벌크 수정연산과 벌크 삭제연산을 각각 수행한 후에 별도로 영속성 컨텍스트 초기화 작업을 하지 않았다고 가정하자.
  
이후에 전체 조회 작업을 시행하면 Update는 연산이 적용되지 않은 영속성 컨텍스트 데이터, delete는 연산이 적용된 DB 데이터 겨로가가 조회되는 것을 확인할 수 있다. (Repeatable Read)

## Update 벌크 연산 상황

영속성 컨텍스트와 DB에 아래 데이터가 있다고 하자.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F0fNXl%2FbtrMeQwfudb%2FSrc85Tgg3MWHLzny58oyX0%2Fimg.png)

위 상황에서 아래와 같이 28살 미만은 이름을 전부 비회원으로 바꾸라는 bulk update 연산을 수행해보자.

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

벌크 연산은 영속성 컨텍스트가 아닌 실제 DB에 바로 쿼리를 날린다.
  
그렇기 때문에 작업을 수행한 후, em.clear()로 영속성 컨텍스트를 초기화해주어야 한다.  
  
그렇지 않으면 영속성 컨텍스트와 DB는 서로 다른 데이터가 저장돼있을 것이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FtJE4F%2FbtrMejSZ39d%2F0Lvlyd9coZjjRCvL7lbFuK%2Fimg.png)
  
위 테스트 코드에서는 update 벌크연산 이후에 별도의 영속성 컨텍스트 초기화 작업을 해주지 않았다. 
  
이 상태에서 QueryDSL selectFrom() 함수를 실행하면 영속성 컨텍스트의 1차 캐시에서 값을 가져오기 때문에 member1은 변경된 이름이 비회원이 아닌 member1이라는 값으로 조회된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNiDWh%2FbtrMdE4HILQ%2FIO0zVbHPKNRlAnhp1JW3n1%2Fimg.png)

이러한 문제를 해결하기 위해서 `em.clear()`로 영속성 컨텍스트를 초기화 해주도록 하자.

## Delete 벌크 연산 상황

아래와 같이 18살 초과의 데이터는 모두 지우라는 벌크 연산을 날린다고 해보자.

```java
queryFactory
        .delete(member)
        .where(member.age.gt(18)) // 18살 초과는 모두 지워라
        .execute();

```
여러 데이터를 한번의 쿼리만으로 삭제시키는 벌크 삭제 쿼리다.  
  
벌크 연산은 영속성 컨텍스트가 아닌 실제 DB에 바로 쿼리를 날린다.
  
따라서 해당 작업이 수행되면 아래와 같은 상태가 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F5T2ur%2FbtrMdMOZ9Yv%2F0qmv3kVs4ksUwEjIB5kOSk%2Fimg.png)

아까와 마찬가지로 QueryDSL의 selectFrom()을 수행하여 결과를 아래와 같이 테스트해보자.

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

영속성 컨텍스트의 1차 캐시에는 삭제 쿼리가 반영되지 않아 모든 데이터가 존재하므로 findAll한 반환값 List<\Member>의 사이즈는 4가 나올 것이라 예상된다.
  
그러나

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fk9YCv%2FbtrMcRpIT1O%2Fs1GAk4ZUHRoxgCLTOE1BA0%2Fimg.png)

우리는 4개의 데이터를 기대햇는데 실제로는 1건의 데이터가 나왔다.  
분명 영속성 컨텍스트에서는 4개의 데이터가 다 남아있음에도 불구하고 말이다.

<br>

## Delete 벌크 연산 상황 해답 및 영속성 컨텍스트 원리
이러한 가설을 세울 수 있겠다.

> 벌크 연산은 delete 일 때 영속성 컨텍스트에도 반영을 하는 말도 안되는 일이 일어난 것이다.

이 가설은 맞는 가설은 아니다.
  
영속성 컨텍스트는 그대로 4개의 데이터가 존재하는 것이 맞다.  
이는 아래 테스트로 확인이 가능하다.

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

EntityManager의 find함수를 수행하면 영속성 컨텍스트에서 member3를 찾는다.  
  
member3는 30살이고, 따라서 벌크 delete 연산으로 지워져야하는 대상이다.
  
해당 연산으로 영속성 컨텍스트에 member3이 없다면 해당 테스트는 실패해야 맞다. 그러나 성공했다..

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Frvo7e%2FbtrMeE3MAfX%2Fr265ryLF4Kw1IJtOcOwtgk%2Fimg.png)

위와 같이 영속성 컨텍스트에서 member3을 찾아오는데 성공했기 때문이다.  
따라서 영속성 컨텍스트엔 삭제 쿼리가 반영되지 않은 상태로 4개의 데이터가 존재하는 것이 맞다.
  
근데 왜 QueryDSL의 selectFrom() 작업 수행 결과로 1개의 데이터만 찾아오는 것일까?
  
일단 QueryDSL의 selectFrom() 작업은 스프링 데이터 JPA의 findAll()과 마찬가지로 JPQL을 날린다.
  
JPQL은 영속성 컨텍스트의 1차 캐시를 탐색하여 동일성을 보장하는 JPA의 findById 또는 EntityManager의 find() 함수와는 다르게 작동한다.
  
JPQL을 호출하면 아래와 같이 동작한다.

1. 실제 DB를 우선 조회한다.
2. DB에 조회한 값을 영속성 컨텍스트에 저장한다.
3. 저장할 때 고유식별자로 이미 영속성 컨텍스트에 해당 엔티티가 존재할 경우 JPA의 동일성 보장을 위해 DB의 데이터를 버리고 영속성 컨텍스트의 데이터를 반환해 사용한다.

따라서 QueryDSL의 selectFrom()을 수행하면 JPQL을 호출하여 실제 DB를 먼저 조회한다. 실제 DB에는 member1에 해당하는 엔티티가 존재하고, 영속성 컨텍스트에 해당 값을 저장한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbsyaXn%2FbtrMadUDhER%2F4IaGpUeKVCPPoSeCIXubw0%2Fimg.png)

이 때 member1에 해당하는 엔티티가 영속성 컨텍스트에 존재하므로 실제 DB의 데이터 대신 영속성 컨텍스트에 존재하는 데이터를 사용하게 된다. 
  
member2, member3, member4는 실제 DB에 존재하지 않기 때문에 영속성 컨텍스트를 확인할 때 조회 대상에 포함되지 않게 된다. 따라서 selectFrom을 한 최종 결과는 member1만 반환하게 된다.
  
따라서 해당 테스트의 결과는 4가 아닌 1이다.

<br>

## Update 벌크 연산 상황 해답

update는 어떻게 동작한 것일까?

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbSLD3Q%2FbtrMd51NFXN%2FtbZJbuHDdp0L4921jGKdrk%2Fimg.png)

update의 경우는 해당 엔티티가 삭제된 것이 아니기 때문에 실제 DB와 영속성 컨텍스트에 모두 데이터가 남아있는 상태다.
  
따라서 JPQL을 호출하면 아래와 같은 원리로 동작한다.

1. 실제 DB를 우선 조회한다.
2. 실제 DB에 존재하는 4개의 데이터를 영속성 컨텍스트에서 찾는다. bulk update 연산으로 고유식별자는 변하지 않으므로 영속성 컨텍스트에서 4개의 데이터를 모두 발견한다.
3. 따라서 실제 DB의 데이터 대신 update되지 않은 영속성 컨텍스트의 데이터 4개를 반환한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fcgsus1%2FbtrMaIUrHMl%2Fb6n9ecQ4LFdZFdikGTO4hk%2Fimg.png)

## 결론
벌크 연산은 영속성 컨텍스트가 아닌 실제 DB에 바로 쿼리를 날린다.  
영속성 컨텍스트의 원리를 제대로 이해하지 못한다면 결과에 대해 이해하지 못할 수 있다.  
영속성 컨텍스트의 원리를 확실하게 공부하자.
  
또한, 벌크 연산을 사용한다면 영속성 컨텍스트를 초기화해주는 em.clear(), 그리고 다른 추가적인 작업이 있다면 em.flush()를 해주는 것을 잊지 말자.