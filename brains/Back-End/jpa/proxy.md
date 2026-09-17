# JPA 프록시(Proxy)와 지연 로딩

## 프록시의 필요성
엔티티를 조회할 때 연관된 엔티티들이 항상 사용되는 것은 아니다.

```java
Member findMember = em.find(Member.class, member.getId());
System.out.println(findMember.getName());

// System.out.println(findMember.getTeam().getName());
```

만약 회원 엔티티만 출력해서 사용하는 경우 em.find()로 회원 엔티티를 조회할 때 회원과 연관된 팀 엔티티까지 데이터베이스에서 함께 조회해 두는 것은 효율적이지 않다.
  
JPA는 이런 문제를 해결하려고 엔티티가 실제 사용될 때까지 데이터베이스 조회를 지연하는 방법을 제공하는데 이것을 **지연 로딩**이라고 한다.
  
지연 로딩 기능을 사용하려면 실제 엔티티 객체 대신에 데이터베이스 조회를 지연할 수 있는 가짜 객체가 필요한데 이것을 **프록시 객체**라고 한다.

## 프록시

### 프록시 조회
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbOJyiT%2FbtrptBiQlYe%2FHgXKFATDiK83ANWaNZTMoK%2Fimg.png)

```java
Member findMember = em.getReference(Member.class, member.getId()); //프록시 객체 생성
```

`em.find()`: 데이터베이스를 통해서 실제 엔티티 조회  
`em.getReference()`: 데이터베이스 조회를 미루는 가짜(프록시) 엔티티 객체 조회

### 프록시의 구조

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbcxU4d%2FbtrpzLk0eJP%2F8oEjDMHdsQdinKGt28K1jk%2Fimg.png)

### 프록시 위임

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F7FqtT%2FbtrpIxMMZTD%2FxYRh8KwPoDg2EicKFlfBUK%2Fimg.png)

프록시 객체는 실제 객체의 찹조(target)을 보관한다.  
  
프록시 객체를 호출하면 프록시 객체는 실제 객체의 메서드를 호출한다.

## 프록시 객체의 초기화
프록시 객체 초기화란 프록시 객체가 실제 사용될 때 데이터베이스를 조회해서 **실제 엔티티 객체를 생성**하는 것이다.

### 초기화 과정

```java
Member findMember = em.getReference(Member.class, member.getId()); // 프록시 객체 생성
findMember.getName(); // 초기화를 통해 실제 엔티티가 생성된다.
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FclMuqx%2FbtrpqytaMa5%2FBCvgvyAPJO5g5i458KFQW0%2Fimg.png)

- 프록시 객체에 member.getName()을 호출해서 실제 데이터를 조회한다.
- 프록시 객체는 실제 엔티티가 생성되어 있지 않으면 영속성 컨텍스트 실제 엔티티 생성을 요청한다.
- 영속성 컨텍스트는 데이터베이스를 조회해서 실제 엔티티 객체를 생성한다.
- 프록시 객체는 생성된 실제 엔티티 객체의 참조를 Member target 멤버변수에 보관한다.
- 프록시 객체는 실제 엔티티 객체의 getName()을 호출해서 결과를 반환한다.

### 프록시 초기화 특징

- 프록시 객체는 처음 사용할 때 **한 번만 초기화된다.**
- 프록시 객체를 초기화한다고 프록시 객체가 실제 엔티티로 바뀌는 것은 아니다. 프록시 객체가 초기화되면 프록시 객체를 통해 실제 엔티티에 접근할 수 있다.
- 프록시 객체는 원본 엔티티를 상속받은 객체이므로 타입 체크 시에 주의해야한다.
  - == 비교 실패, 대신 instance of 사용


```java
Member member = new Member();
member.setName("member");
em.persist(member);

em.flush();
em.clear();

//영속성 컨텍스트에 찾는 엔티티가 있는 경우 엔티티를 반환한다.
Member findMember = em.find(Member.class, member.getId());
System.out.println(findMember.getClass()); //실제 엔티티 반환

Member reference = em.getReference(Member.class, member.getId());
System.out.println(reference.getClass()); //실제 엔티티 반환

em.clear();

//프록시가 이미 있는 경우 프록시를 반환한다.
Member reference2 = em.getReference(Member.class, member.getId());
System.out.println(reference2.getClass()); //프록시 반환

Member findMember2 = em.find(Member.class, member.getId());
System.out.println(findMember2.getClass()); //프록시 반환
```

영속성 컨텍스트의 도움을 받을 수 없는 준영속 상태일 때, 프록시를 초기화하면 문제가 발생한다.  
  
하이버네이트는 org.hibernate.LazyInitializationException 예외를 터트린다.  
no Session -> 영속성 컨텍스트에 없다는 말이다.

```java
//영속성 컨텍스트의 도움을 받을 수 없는 준영속 상태일 때, 프로시를 초기화하면 문제 발생
Member refMember = em.getReference(Member.class, member.getId());
System.out.println(refMember.getClass()); //Proxy

em.detach(refMember); //준영속 상태로 만들어준다.
//em.clear();
//em.close();

refMember.getName(); //프록시 초기화 불가능
```

## 프록시 확인

### 프록시 인스턴스의 초기화 여부
`PersistenceUnitUtil.isLoaded(Object entity)`

```java
//프록시 인스턴스의 초기화 여부 확인
Member refMember = em.getReference(Member.class, member.getId());
System.out.println(emf.getPersistenceUnitUtil().isLoaded(refMember)); //false
refMember.getName();
System.out.println(emf.getPersistenceUnitUtil().isLoaded(refMember)); //true
```

### 프록시 클래스 확인 방법
`entity.getClass().getName()` 출력

```java
Member refMember = em.getReference(Member.class, member.getId());
System.out.println(refMember.getClass().getName());
```

### 프록시 강제 초기화

`org.hibernate.Hibernate.initialize(entity)`

```java
// 강제초기화
Member refMember = em.getReference(Member.class, member.getId());
Hibernate.initialize(refMember);
```

JPA 표준에는 강제 초기화 메서드가 없다. (웨 메서드는 Hibernate 메서드)
  
결국 member.getName()을 이용해서 초기화 해주어야한다.