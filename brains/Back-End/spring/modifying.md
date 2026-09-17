# Spring Data JPA @Modifiying

`@Modifiying` 어노테이션은 `@Query` 어노테이션이라는 JPQL Query, Native Query를 통해 작성된 INSERT, UPDATE, DELETE 쿼리에 함께 사용되는 어노테이션이다.

기본적으로 JpaRepository에서 제공하는 메서드 혹은 메서드 네이밍으로 만들어진 쿼리에는 적용되지 않는다.

clearAutomatically, flushAutomatically 속성을 변경할 수 있으며 주로 벌크 연선과 같이 이용된다.
- clearAutomatically(default = false)
- flushAutomatically(default = false)

```java
// MemberInfoRepository extends JpaRepository<MemberInfo, Long>    
@Query("UPDATE MemberInfo m "
    + "SET m.name = :name "
    + "WHERE m.age >= :age")
void updateNameByAgeGreaterThan(@Param("name") String name, @Param("age")int age);
```

예를들어 위와 같은 조건에 맞는 모든 로우에 벌크 update를 하기 위해 `@Query` 어노테이션을 통해 쿼리를 짜고 실행해보자.

잘 실행될 거라고 기대하지만 에러가 뜨게 된다.

```
Queries that require a `@Modifying` annotation 
include INSERT, UPDATE, DELETE, and DDL statements.
```

그리고 그 쿼리에서는 `@Query` 어노테이션으로 작성한 insert, update, delete 문을 실행할 경우에는 반드시 `@Modifiying`을 붙이라고 강제하고 있다.

(붙이지 않을 경우 executeUpdate()가 아닌 getSingleResult()와 같은 select성 쿼리의 결과를 가져오는 em의 메서드가 호출되기 때문이다.)

그렇기에 `@Modifiying`을 붙이면 성공은 한다. 그렇지만 여기서 또 주의해야할 점이 존재한다.

바로 영속성 컨텍스트와 db 데이터 간의 동기화 문제인데. 

bulk성 쿼리는 바로 database에 변경을 반영하지만, 영속성컨텍스트위에 이전의 데이터가 존재한다면 그 다음 한 트랜잭션에서 해당 데이터를 조회할 때 영속성 컨텍스트에서 데이터를 가져와 일치하지 않는 문제가 발생한다.

**그렇기에 꼭 bulk 연산 이후에 그 데이터를 재사용하는 로직이 한 트랜잭션에 묶여있다면 꼭, 영속성 컨텍스트를 clear 해줘야한다.**

em.clear()를 직접 호출하는 방법도 있겠지만, `@Modifying` 어노테이션에서 clearAutomatically 옵션을 활성화 시켜주면 된다.

```java
    
// Repository    
@Query("UPDATE MemberInfo m "
    + "SET m.name = :name "
    + "WHERE m.age >= :age")
@Modifying(clearAutomatically = true)
void updateNameByAgeGreaterThan(@Param("name") String name, @Param("age") int age);​
```

