# JPA Proxy and Lazy Loading

## The Need for Proxies
When querying an entity, its associated entities are not always used.

```java
Member findMember = em.find(Member.class, member.getId());
System.out.println(findMember.getName());

// System.out.println(findMember.getTeam().getName());
```

If you only need to display and use the Member entity, it's inefficient to query the Member entity and its associated Team entity from the database together when using em.find().
  
To solve this problem, JPA provides a method to defer database queries until the entity is actually used, which is called **lazy loading**.
  
To use the lazy loading feature, a "fake" object that can defer database queries is needed instead of the actual entity object. This is called a **proxy object**.

## Proxy

### Querying a Proxy
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbOJyiT%2FbtrptBiQlYe%2FHgXKFATDiK83ANWaNZTMoK%2Fimg.png)

```java
Member findMember = em.getReference(Member.class, member.getId()); //프록시 객체 생성
```

`em.find()`: Queries the actual entity from the database  
`em.getReference()`: Queries a fake (proxy) entity object that defers database queries

### Proxy Structure

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbcxU4d%2FbtrpzLk0eJP%2F8oEjDMHdsQdinKGt28K1jk%2Fimg.png)

### Proxy Delegation

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F7FqtT%2FbtrpIxMMZTD%2FxYRh8KwPoDg2EicKFlfBUK%2Fimg.png)

A proxy object holds a reference (target) to the actual object.  
  
When a proxy object is called, it invokes the method of the actual object.

## Initializing a Proxy Object
Initializing a proxy object means querying the database to **create the actual entity object** when the proxy object is actually used.

### Initialization Process

```java
Member findMember = em.getReference(Member.class, member.getId()); // 프록시 객체 생성
findMember.getName(); // 초기화를 통해 실제 엔티티가 생성된다.
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FclMuqx%2FbtrpqytaMa5%2FBCvgvyAPJO5g5i458KFQW0%2Fimg.png)

- Call member.getName() on the proxy object to query the actual data.
- If the actual entity has not been created, the proxy object requests the persistence context to create the actual entity.
- The persistence context queries the database to create the actual entity object.
- The proxy object stores the reference to the created actual entity object in the `Member target` member variable.
- The proxy object calls getName() on the actual entity object and returns the result.

### Characteristics of Proxy Initialization

- A proxy object is initialized **only once** when first used.
- Initializing a proxy object does not mean the proxy object itself changes into the actual entity. Once initialized, the proxy object allows access to the actual entity.
- Since a proxy object inherits from the original entity, care must be taken during type checks.
  - `==` comparison fails; use `instance of` instead.

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

When in a detached state, unable to receive help from the persistence context, initializing a proxy causes issues.  
  
Hibernate throws a org.hibernate.LazyInitializationException.  
no Session -> This means it's not in the persistence context.

```java
//영속성 컨텍스트의 도움을 받을 수 없는 준영속 상태일 때, 프로시를 초기화하면 문제 발생
Member refMember = em.getReference(Member.class, member.getId());
System.out.println(refMember.getClass()); //Proxy

em.detach(refMember); //준영속 상태로 만들어준다.
//em.clear();
//em.close();

refMember.getName(); //프록시 초기화 불가능
```

## Checking Proxies

### Checking if a Proxy Instance is Initialized
`PersistenceUnitUtil.isLoaded(Object entity)`

```java
//프록시 인스턴스의 초기화 여부 확인
Member refMember = em.getReference(Member.class, member.getId());
System.out.println(emf.getPersistenceUnitUtil().isLoaded(refMember)); //false
refMember.getName();
System.out.println(emf.getPersistenceUnitUtil().isLoaded(refMember)); //true
```

### How to Check a Proxy Class
Print `entity.getClass().getName()`

```java
Member refMember = em.getReference(Member.class, member.getId());
System.out.println(refMember.getClass().getName());
```

### Forcing Proxy Initialization

`org.hibernate.Hibernate.initialize(entity)`

```java
// 강제초기화
Member refMember = em.getReference(Member.class, member.getId());
Hibernate.initialize(refMember);
```

The JPA standard does not have a method for forced initialization. (The above method is a Hibernate method)
  
Ultimately, initialization must be done using member.getName().
