# High Hibernate Performance, NaturalId

### NaturalId

If a natural key exists in your table design, you'll likely use it for record access as often as you use the primary key. In such cases, attaching Hibernate's `@NaturalId` annotation can improve performance with little effort.

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

Given an entity like the one above, how many queries would occur if we queried by the primary key and by the natural ID (email)?

A simple test of 100 repeated lookups shows that querying by ID results in 1 query, while querying by natural ID results in 101 queries.

Querying by ID results in only one query because the entity is retrieved from the persistence context. NaturalId, however, is fundamentally a Hibernate-specific feature, not part of the JPA standard. Therefore, JPA is unaware of this behavior and does not use the persistence context for natural ID lookups.

To make JPA aware, you can unwrap the `EntityManager` to a Hibernate `Session`.

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

By unwrapping in this way, you can enable the JPA persistence context to use the natural ID for object retrieval. This means that even in the 100-lookup test, only one query will be executed, thanks to the first-level cache.

**Using NaturalId offers the following advantages:**

For example, if you use auto-incrementing or random values for primary keys, you often encounter situations where test data in development/test environments differs from data in production. Consequently, most entities using auto-incrementing values as primary keys have different values in production and test environments. If the ID values differ and you need to quickly test and deploy, you might have to hardcode values for feature testing.

Imagine you've diligently developed and are about to deploy to a test environment, when an additional requirement arises: on the morning of the production deployment, new data is added, and you're asked to apply the same conditions (e.g., a discount) to this newly generated data that were already applied to existing data. In a beta environment, you might add new data and then update the data ID in a configuration file. However, in a production environment, this is impossible because you can only know the primary key once the data is actually created. Of course, you could check the generated data, add it to the code, and then deploy. But if a human error occurs during this process, it could lead to a critical issue where the wrong data gets the condition applied.

Therefore, to ensure data consistency and enable identification across different database environments, you can use a common, immutable unique key column. This is why natural keys are used.

### findByNaturalId()

Let's look at the internal workings in more detail. If we unwrap by natural ID, the first-level cache stores entity snapshots based on their keys. So, if it's cached by natural ID, would a simple ID query not be cached even if the ID exists?

That's not the case. Looking at the internal implementation, there's an operation called `persistenceContext.getNaturalIdResolutions().findCachedIdByNaturalId()`. This finds and returns the ID corresponding to the natural ID from a `naturalIdToPkMap`, effectively querying by the ID value.

![](https://techblog.woowahan.com/wp-content/uploads/2024/04/BaseNaturalIdLoadAccessImpl.doLoad.png)

Since the persistence context queries using the ID value returned above, `findById` also results in only a single query due to the first-level cache.

If you find it difficult to identify data across different environments due to auto-incrementing or random values, it's good to identify a unique and immutable column. And if you declare it with `@NaturalId`, it will behave similarly to `findById()` provided by `JpaRepository`, so if you've decided to use a natural key, be sure to add the annotation.

> However, it is said that an additional query occurs in Hibernate versions below 5.5. Avoid using it if your version is below 5.5.
