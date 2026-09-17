# 플러시 (Flush)

- 영속성 컨텍스트의 변경 내용을 DB에 반영하는 것을 말한다.
- Transaction commit이 일어날때 flush가 동작하는데, 이때 쓰기 지연 저장소에 쌓아 놨던 INSERT, UPDATE, DELETE SQL들이 DB에 날아간다
  - 주의 영속성 컨텍스트를 비우는것이 아님
- 쉽게 얘기해 영속성 컨텍스트의 변경 사항들이 DB의 상태를 맞추는 작업니다.
  - 플러시는 영속성 컨텍스트의 변경 내용을 DB에 동기화한다.

### 플러시 동작 과정
1. 변경을 감지
2. 수정된 엔티티를 쓰기 지연 sql 저장소에 등록
3. 쓰기 지연 sql저장소의 쿼리들을 DB에 전송

- 플러시가 발생한다고 해서 커밋이 이루어지는 것이 아니고 플러시 다음에 실제 커밋이 일어난다.
- 플러시가 동작할 수 잇는 이유는 데이터베이스 트랜잭션이라는 개념이 있기 때문이다.
  - 트랜잭션이 시작되고 해당 트랜잭션이 커밋되는 시점 직전에만 동기화해주면 되기 때문에 그 사이에 플러시 매커니즘의 동작이 가능하 것이다.
- JPA는 기본적으로 데이터를 맞추거나 동시성에 관련된 것들은 데이터베이스 트랜잭션에 위임한다.


### 영속성 컨텍스트를 플러시 하는 방법

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


- Q. 플러시가 일어나면 1차 캐시가 모두 지워질까?
  - NO! 그대로 남아있다.
  - 쓰기 지연 SQL 저장소에 있는 Query들만 DB에 전송까지만 되는 과정일뿐이다.

2. 트랜잭션 커밋 시 플러시 자동 호출


3. JPQL 쿼리 실행시 자동 플러시 호출

```java
em.persist(memberA);
em.persist(memberB);
em.persist(memberC);

// 중간에 JPQL 실행
query = entityManager.createQuery("select m from Member m", Member.class);
List<Member> members = query.getResultList();
```