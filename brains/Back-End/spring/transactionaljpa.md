# Spring Data JPA와 @Transactional

JPA는 트랜잭션 기반으로 사용된다.

그래서 JPA를 사용하면 우리는 자주 `@Transactional`어노테이션을 사용하는 것을 알 수 있다.

그리고 조회의 경우에는 `readOnly=true`옵션도 함께 주게된다.

오늘은 JPA와 @Transactional에 대해서 몇 가지 궁금한 점에 대해 기록해보겠다.

<br>

## readOnly = true를 설정하는 이유

우리는 조회용 서비스 메서드를 작성할 때 `@Transactional(readOnly = true)` 이렇게 서비스 위에 어노테이션을 달아주게 된다.

이 어노테이션을 달아주게 된다면 이점이 무엇이 있을까? 가독성?

물론 가독성에 대한 장점도 있겠지만 다른 이유도 존재한다.

### 변경 감지

JPA의 영속성 컨텍스트가 수행하는 변경 감지(Dirty Checking)와 관련이 있는데, 영속성 컨텍스트는 엔티티 조회시 초기 상태에 대한 **스냅샷**을 저장한다.

그리고 트랜잭션이 커밋될 때, 초기 상태 정보가 스냅샷, 엔티티의 상태를 비교하여 변경된 내용에 대해 업데이트 쿼리를 생성해 쓰기 지연 저장소에 저장한다.

그 후, 일괄적으로 쓰기 지연 저장소에 저장되어 있는 SQL Query를 flush하고 데이터베이스의 트랜잭션을 커밋함으로써 우리가 업데이트와 같은 메서드를 사용하지 않고도 엔티티의 수정이 이루어진다. 이를 더티 체킹이라고 한다.

다시 돌아가서, readOnly=true 옵션을 주게 되면 스프링은 JPA의 세션 플러시 모드를 MANUAL로 설정하게 된다.

> MANUAL 모드는 트랜잭션 내에서 사용자가 수동으로 플러시를 호출해야만 플러시가 수행되는 모드이다.


즉, 플러시를 수동으로 호출하지 않는 한, 수정 내역에 대해서 데이터베이스에 적용이 되지 않는다.

이는 두 가지 이점을 얻을 수 있다.
- 조회용으로 가져온 엔티티의 예상치 못한 수정 방지, 이는 트랜잭션 커밋시 영속성 컨텍스트가 자동으로 플러시 되지 않기 때문
- JPA는 readOnly=true일 때 트랜잭션 내에 조회하는 엔티티는 조회용임을 인식하고 변경 감지를 위한 스냅샷을 따로 보관하지 않기 때문에 메모리가 절약된다.

이는 DB Replication 부하 분산에도 이점을 둘 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbx66cl%2FbtsdZz2V4dX%2Fi9VUWmK5Hss2qUwI2vTd70%2Fimg.png)

레플리케이션 Master - Slave 구조로 복제본 데이터베이스를 운용할 때, 조회 작업은 Slave에서 수행하고 수정 작업은 Master에서 수행함으로써 트래픽을 분산시킬 수도 있다. CQRS

이러한 DB구조를 가져갈 때 readOnly=true인 메서드는 Slave에서 데이터를 가져오도록 동작할 수 있다.

그리고 OSIV가 false (`spring.jpa.open-in-view=false`)일때 지연 로딩을 사용하려고 하면 `@Transactional`어노테이션이 없으면 예외가 터진다. `LazyInitializationException`

> OSIV(Open Session In View)는 영속성 컨텍스트를 View Layer까지 유지하는 속성이며 클라이언트 요청 시점부터 영속성 컨텍스트를 생성한다. 그래서 필터 인터셉터 컨트롤러 부터 영속성 컨텍스트가 생성되어 유지됨으로써 View Layer에서도 지연 로딩을 사용할 수 있다.

OSIV가 false라면 영속성 컨텍스트는 트랜잭션 범위를 벗어나는 순간 엔티티는 영속성 컨텍스트에 관리를 받지 않는 준영속 상태가 된다. 즉 지연 로딩이 불가능하다.

```java
// application.properties (OSIV off) 
spring.jpa.open-in-view=false 

// @Transactional(readOnly = true) 
public Member getMember(Long userId) { 
  Member member = memberRepository.findByMemberId(userId).get();
  System.out.println(member.getTeam().getName()); // Lazy Loading return member; 
}
```

물론 커넥션이 다량의 트래픽으로 고갈되면 또 문제가 발생하기 때문에 @Transactional을 꼭 붙여주는 것이 좋다. 조회용에서도 readOnly=true를 적극 활용해보자

<br>

### em.persist(), em.find(), 트랜잭션

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

위의 테스트를 돌리면 실패가 뜬다.

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

하지만 위와 같이 작성하면 성공하게 된다.

이는 `em.find()`는 단순히 영속성 컨텍스트에 있는 값을 꺼내오는 메서드로 트랜잭션을 필요로 하지 않기 때문이다.(물론 락을 건다면 필요하다.)

그러나 여기서 조금 의아한점은 em.find()를 했을 때 트랜잭션을 안거는 것이 좋을까? 아니다. 트랜잭션을 아예 걸지 않으면 모든 select마다 커밋을 하기 때문에 성능이 떨어진다. 트랜잭션을 건다면 마지막 한 번만 커밋을 하기 때문에 좀 더 효율적이다.

어찌됐든 다시 돌아와서 이번엔 왜 `save`를 호출 했을때는 `find`가 잘 동작했을까? 이는 `SimpleJpaRepository`의 `save`를 보면 클래스에 `@Transactional(readOnly = true)`가 있고 `save` 메서드에 `@Transactional`이 있기 때문이다. 그렇기에 트랜잭셔널이 이미 있어서 사용이 가능했던 것이다.

<br>

### @DataJpaTest 사용시 주의할 점

마지막으로 @DataJpaTest 사용시 주의할 점을 언급하자면

일단 저 어노테이션에는 @Transactional이 디폴트로 들어가있다. 이게 왜 문제냐면

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

만약 지연로딩 기능을 사용하는데 서비스 코드에 트랜잭셔널이 없어 운영 환경에서는 예외가 터지는 서비스 코드라고 해보자. 그러나 DataJpaTest로 진행되면 트랜잭셔널이 있기 때문에 테스트가 통과되는 사태가 발생할 수 있다.

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

즉 프로덕션 코드는 동작하지 않는데, 테스트는 통과되는 아이러니한 상황이다.

물론 OSIV가 true라면 동작하지만 아까 위에서 언급한 커넥션 고갈로 인해 프로덕션에서 문제가 생길 수 있다.