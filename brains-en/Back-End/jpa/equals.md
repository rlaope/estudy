# JPA Entity equals(), hashcode() overriding

In Hibernate, it is recommended to override `equals` and `hashCode` when dealing with identifier classes or business keys. This is because when an ID is set with a generated value, it conflicts with the implementation specifications required by `Set` data structures. Data structures like `HashSet` and `HashMap` map data by creating hash buckets based on an object's hash value, and problems can arise in JPA because an object's `equals` and `hashCode` values can change before and after a transaction commit.

Therefore, [JPA Buddy](https://jpa-buddy.com/blog/hopefully-the-final-article-about-equals-and-hashcode-for-jpa-entities-with-db-generated-ids/) recommends writing it as follows.
It was previously recommended to use `Hibnernate.getClasS()`, but it is now said that bypassing it as shown below is necessary because it initializes proxies.

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

Consequently, since objects in an ORM are linked to database tables, if the database ID values are logically the same, then the objects should also be the same. Of course, it's true that if objects are different, their data is also different, but different data rows are not treated as different data. This is my personal opinion, but I think it comes down to whether you want to strictly enforce inconsistencies at the application code level or at the database level.

Personally, I find it more convenient to focus on the database.
