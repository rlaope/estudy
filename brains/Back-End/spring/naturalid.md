# High Hiberante Performance, NaturalId

### NaturalId

테이블 디자인시에 자연키가 있다면, 레코드 접근 시 pk만큼 자연키를 사용할 것이다. 이떄 hibernate의 `@NaturalId` 어노테이션을 붙이면 큰 노력없이 성능 향상이 가능하다.

```java
@Table(name = "app_user")  
@Entity  
class User(  
	@NaturalId  
	@Column(name = "email", unique = true, nullable = false)  
	val email: String,  
  
	@Column(name = "name", nullable = false)  
	val name: String,  
  
	@Column(name = "intro")  
	val intro: String? = null,  
	  
	@Id  
	@Column(name = "user_key", unique = true, nullable = false)  
	@GeneratedValue(strategy = GenerationType.SEQUENCE)  
	@SequenceGenerator(name = "user_seq", sequenceName = "user_seq", allocationSize = 1)  
	val key: Long = 0L,  
)
```

위와 같은 엔티티에서 pk와 naturalId인 email로 조회를한다면 총 몇 번의 쿼리가 발생할까?

단순 100번 반복의 조회 테스트를 돌려보면 id로 조회는 1번, naturalid는 101번의 쿼리가 발생한다.

id로 조회하게 되면 영속성 컨텍스트에서 엔티티를 조회해오기 때문에 쿼리가 한 번 발생한 것이고 NaturalId는 기본적으로 JPA 표준이 아닌 Hibernate의 스펙이다. 따라서 jpa는 이 동작을 모르기에 영속성 컨텍스트를 사용하지 않는다.

그렇기에 jpa에서도 알 수 있도록 entity manager에 hibnernate session으로 unwrap한다.

```java
interface UserRepositoryCustom {  
	fun getReferenceByEmail(email: String): User  
}  
  
class UserRepositoryCustomImpl(  
@field:PersistenceContext  
private val entityManager: EntityManager,  
): UserRepositoryCustom {  
	override fun getReferenceByEmail(email: String): User {  
	return entityManager.unwrap(Session::class.java)  
	.bySimpleNaturalId(User::class.java)  
	.load(email)  
	?: throw EntityNotFoundException("User not found by email: $email")  
	}  
}
```

이렇게 unwrap하여 naturalid로도 해당 객체를 jpa 영속성 컨텍스트에서 사용할 수 있도록 변경이 가능하다. 이렇게 되면 저 100번 테스트에서도 한 번의 쿼리만 나갈 것이다. 1차캐시에 의해서


**NaturalId를 사용하면 다음과 같은 이점들이 있다.**

예를들어 pk를 ai값으로 사용하거나 Random 값으로 사용하게 되면 테스트, 운영 환경에서 테스트 용도 데이터와 운영환경과 다른 데이터가 종종 생겨날 때가 있다.

그래서 보통 자동 증가 값을 기본키로 사욯아는 엔티티 대부분이 운영환경과 테스트환경의 값이 다른데 id값이 다르다면 빠르게 테스트 후 배포해야하는 상황이라면 해당 기능 테스트를 위해 하드코딩해서 진행해야할 것이다.

열심히 개발해 테스트 환경에 배포하려는 순간 추가 요구사항이 발생해 운영배포일 오전에 새로운 데이터가 추가될 때 신규로 생성되는 데이터중에서 기존에 데이터에도 걸려있던 동일한 조건을 걸어달라는 요구사항이 추가된다면? 예를들면 할인

베타 환경의 경우 새로운 데이터를 추가후 데이터 아이디를 환경파일에 추가했겠지만 운영환경은 실질적으로 생겨야 pk를 알아낼 수 있기 때문에 불가능하다. 물론 생성된 데이터를 확인해 코드상에 추가한뒤 배포해도 되겠지만.. 이렇게 하다가 휴먼에러를 맞이하면 다른 데이터가 조건이 걸려버리는 치명적인 문제가 있을수도 있다.

그렇기에 이런 데이터 정합성, 다른 데이터베이스 환경에서도 식별할 수 있도록 공통된, 또 변경되지 않는 유니크 키를 통해 컬럼을 사용해볼 수 있다. 그렇기에 natural key를 사용하는 것이다.

### findByNaturalId()

여기서 내부 동작 과정을 좀 더 자세히 살펴보겠다 만약 그럼 natural id로 unwrap하게 된다면 1차캐시는 key를 토대로 엔티티 스냅숏을 캐싱해두는데 natural id로 캐싱해두었으니 그냥 id가 존재한다면 그냥 id query는 캐싱이 안될까?

그렇지 않다. 내부 구현을 보면 `persisteConext.getNaturalIdResolutions().findCachedIdByNatrualId()`라는 동작이 있는데 naturalIdToPkMap 이라는 곳에서 naturalId에 해당하는 id를 찾아 반환하기 때문에 결과적으로 id값으로 조회하는 것이다.

![](https://techblog.woowahan.com/wp-content/uploads/2024/04/BaseNaturalIdLoadAccessImpl.doLoad.png)

위에서 반환된 아이디값을 통해 영속성 컨텍스트에서 조회하고 있기 때문에 find by id도 1차캐시로 인해 쿼리가 단 한번만 나가는 것을 알 수 있다.

자동증가 값이나 랜덤값처럼 다른 환경에서 데이터를 식별하기가 어려움을 느끼게 된다면 유니크하고 불변의 컬럼을 식별하면 좋다. 그리고 @NaturalId로 선언하게 되면 JpaRepository에서 제공하는 findById()와도 같은 동작을 하기 때문에 만약 자연키로 사용하기로 했다면 꼭 어노테이션을 붙여주자.

> 그러나 hibnerate 5.5 미만일때는 추가 쿼리가 나온다고 한다. 5.5미만일때는 사용을 피하도록 하자