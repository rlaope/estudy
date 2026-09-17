# JPA Entity equals(), hashcode() overriding

Hibernate에서는 identifier class, business key를 갖는 경우에는 equals and hashcode를 오버라이딩 하기를 권장한다. generated value로 id가 설정된 경우 Set 자료 구조가 요구하는 구현 스펙과 충돌하기 때문이다. HashSet, HashMap과 같은 자료구조는 객체의 hash값을 토대로 해시 버킷을 생성해 데이터를 매핑하는데 jpa에서 트랜잭션 커밋 전/후로 객체의 equals, hashcode값이 바뀌기 때문에 문제가 발생할 수 있다.

그렇기에 [JPA Buddy](https://jpa-buddy.com/blog/hopefully-the-final-article-about-equals-and-hashcode-for-jpa-entities-with-db-generated-ids/)에서는 다음과 같이 작성하기를 권장한다.
Hibnernate.getClasS()를 사용하도록 권장했었는데, 프록시를 초기화 시키기에 아래와 같이 우회하는 것이 필요하다고 한다.

```java
@Entity
class MyEntity(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    val id: Long? = null,
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other == null) return false

        val otherEffectiveClass = if (other is HibernateProxy) {
            other.hibernateLazyInitializer.persistentClass
        } else {
            other::class.java
        }

        val thisEffectiveClass = if (this is HibernateProxy) {
            this.hibernateLazyInitializer.persistentClass
        } else {
            this::class.java
        }

        if (thisEffectiveClass != otherEffectiveClass) return false

        other as MyEntity
        return id != null && id == other.id
    }

    override fun hashCode(): Int {
        return if (this is HibernateProxy) {
            this.hibernateLazyInitializer.persistentClass.hashCode()
        } else {
            this::class.java.hashCode()
        }
    }
}
```

결과적으로 ORM에서 객체는 데이터베이스의 테이블과 연결되는 것이기 때문에 논리적으로 데이터베이스의 ID값이 같다면 객체도 같아야한다. 물론 객체가 다르면 데이터가 다른 것도 사실이지만, 데이터 로우들이 다르다고해서 다른 데이터로 취급하지 않는다. 이건 개인적인 생각으로 애플리케이션 코드 레벨에서의 불일치를 엄격하게 가져갈 것이냐 혹은 데이터베이스레벨에서 엄격하게 가져갈 것인가에 차이인 것 같다고도 생각된다.

갠적으로 디비에 초점을 맞추는게 더 편한듯 하다.