# Spring Data JPA and @Transactional

JPA is used on a transaction basis.

Therefore, when using JPA, we often see the use of the `@Transactional` annotation.

And for queries, the `readOnly=true` option is often given along with it.

Today, I'll document a few questions I have about JPA and @Transactional.

<br>

## Reasons for setting readOnly = true

When writing service methods for queries, we typically add the `@Transactional(readOnly = true)` annotation above the service.

What are the benefits of adding this annotation? Readability?

While readability is certainly a benefit, there are other reasons as well.

### Dirty Checking

It's related to Dirty Checking performed by JPA's persistence context. When an entity is retrieved, the persistence context stores a **snapshot** of its initial state.

Then, when the transaction is committed, the initial state information (snapshot) is compared with the entity's current state. For any changes detected, an update query is generated and stored in the write-behind store.

After that, the SQL queries stored in the write-behind store are flushed in a batch, and the database transaction is committed. This allows entities to be modified without explicitly calling update methods. This process is called dirty checking.

Returning to the point, when the `readOnly=true` option is given, Spring sets JPA's session flush mode to MANUAL.

> MANUAL mode is a mode where flushing is performed only when the user explicitly calls flush within a transaction.

In other words, unless flush is manually called, modifications are not applied to the database.

This provides two benefits:
- Prevents unexpected modifications to entities fetched for queries, because the persistence context does not automatically flush when the transaction commits.
- When `readOnly=true`, JPA recognizes that entities queried within the transaction are for read-only purposes and does not store separate snapshots for dirty checking, thus saving memory.

This can also offer advantages in DB Replication load balancing.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbx66cl%2FbtsdZz2V4dX%2Fi9VUWmK5Hss2qUwI2vTd70%2Fimg.png)

When operating a replicated database with a Master-Slave structure, query operations can be performed on the Slave and modification operations on the Master, thereby distributing traffic. CQRS

With such a DB structure, methods marked with `readOnly=true` can be configured to fetch data from the Slave.

Furthermore, if OSIV is false (`spring.jpa.open-in-view=false`) and you try to use lazy loading without the `@Transactional` annotation, an exception (`LazyInitializationException`) will be thrown.

> OSIV (Open Session In View) is a property that maintains the persistence context up to the View Layer, creating it from the moment a client request is received. This means the persistence context is created and maintained from the filter, interceptor, and controller, allowing lazy loading to be used even in the View Layer.

If OSIV is false, the moment the persistence context goes out of the transaction scope, the entity becomes a detached state, no longer managed by the persistence context. This means lazy loading is not possible.

```java
// application.properties (OSIV off) 
spring.jpa.open-in-view=false 

// @Transactional(readOnly = true) 
public Member getMember(Long userId) { 
  Member member = memberRepository.findByMemberId(userId).get();
  System.out.println(member.getTeam().getName()); // Lazy Loading return member; 
}
```

Of course, if connections are exhausted due to high traffic, another problem arises, so it's good practice to always attach @Transactional. Let's actively utilize readOnly=true even for query operations.

<br>

### em.persist(), em.find(), Transactions

```java
@SpringBootTest
class PostServiceTest {

    @PersistenceContext
    private EntityManager em;

    @Test
    public void persistTest() {
        Post post = Post.builder()
            .content("글 입니다")
            .build();
        em.persist(post);
    }
    
	@Test
    void findTest() {
        em.find(Post.class, savedPost.getId());
    }
}
```

Running the above test will result in a failure.

```bash
No EntityManager with actual transaction available for current thread - cannot reliably process 'persist' call
javax.persistence.TransactionRequiredException: No EntityManager with actual transaction available for current thread - cannot reliably process 'persist' call
```

```java
@SpringBootTest
class PostServiceTest {

    @PersistenceContext
    private EntityManager em;

    @Autowired
    private PostRepository postRepository;

    private Post savedPost;

    @BeforeEach
    void setup() {
        Post post = Post.builder()
            .content("글 입니다")
            .build();
        savedPost = postRepository.save(post);
    }

    @Test
    void findTest() {
        em.find(Post.class, savedPost.getId());
    }
}
```

However, writing it as above will make it succeed.

This is because `em.find()` is a method that simply retrieves a value from the persistence context and does not require a transaction (though it would if a lock were applied).

However, a slightly puzzling point here is, is it better not to use a transaction when calling `em.find()`? No. If you don't use any transactions, a commit occurs for every select, leading to performance degradation. If you use a transaction, a commit happens only once at the end, which is more efficient.

Anyway, returning to the topic, why did `find` work correctly when `save` was called this time? This is because if you look at `SimpleJpaRepository`'s `save` method, the class has `@Transactional(readOnly = true)` and the `save` method itself has `@Transactional`. Therefore, a transactional context was already present, making it usable.

<br>

### Caveats when using @DataJpaTest

Finally, regarding caveats when using @DataJpaTest:

First, that annotation includes @Transactional by default. Why is this a problem?

``` java
@Target(ElementType.TYPE)
@Retention(RetentionPolicy.RUNTIME)
@Documented
@Inherited
@BootstrapWith(DataJpaTestContextBootstrapper.class)
@ExtendWith(SpringExtension.class)
@OverrideAutoConfiguration(enabled = false)
@TypeExcludeFilters(DataJpaTypeExcludeFilter.class)
@Transactional // ㅎㅇ
@AutoConfigureCache
@AutoConfigureDataJpa
@AutoConfigureTestDatabase
@AutoConfigureTestEntityManager
@ImportAutoConfiguration
public @interface DataJpaTest {
    //...
}
```

Let's say you're using lazy loading, and your service code lacks @Transactional, causing an exception in a production environment. However, if you proceed with DataJpaTest, the test might pass because @Transactional is present, leading to a problematic situation.

```java
@Service
@RequiredArgsConstructor
public class CustomPostService {

    private final PostRepository postRepository;

    // @Transactional
    public void accessPost(Long id) {
        Post post = postRepository.findById(id).orElseThrow(IllegalArgumentException::new);
        post.getComments().get(0).getContent(); // 지연로딩을 위한 코드!
    }
}
```

```java
@DataJpaTest
@Import(CustomPostService.class)
class PostServiceTest {

    @Autowired
    private CustomPostService postService;

    @Autowired
    private PostRepository postRepository;

    private Post savedPost;

    @BeforeEach
    void setup() {
        Post post = Post.builder()
            .content("글 입니다")
            .build();
        Comment comment = Comment.builder()
            .content("댓글입니다")
            .build();
        post.addComment(comment);

        savedPost = postRepository.save(post);
    }

    @DisplayName("프로덕션에 @Transactional을 붙이지 않았음에도 테스트가 통과된다!")
    @Test
    void accessCommentTest() {
        assertThatCode(() -> postService.accessComment(savedPost.getId()))
            .doesNotThrowAnyException();
    }
}
```

In other words, it's an ironic situation where the production code doesn't work, but the test passes.

Of course, it would work if OSIV were true, but as mentioned earlier, connection exhaustion could lead to problems in production.
