# Clean Architecture with Spring Boot

### Clean Architecture
The goal of many architectures is `separation of concerns`.
By separating concerns into layers, the role of each layer becomes clear.
Clean Architecture has the following objectives:

1.  Independent of Frameworks
2.  Easy to Test
3.  Independent of UI
4.  Independent of DB
5.  Independent of External Features

Simply put, it means that if the framework, DB, or UI changes, the business logic should not need to change.

<br>

### Dependency Rule

In Clean Architecture, dependencies must point inwards.
The inner circles should not know anything about the outer circles.
Similarly, the data format used by the outer circles should not be used by the inner circles.

In other words, the inner circles should perform their roles independently of the outer ones.

![](./image/cleanarchitecture.jpeg)

<br>

## Example Implementation

### Entity
Entities encapsulate the **most general and high-level rules**. Changes in the application's behavior (scenarios) should not affect this layer.

```java
@Getter
@AllArgsConstructor
public class Post {
    private final String title;
    private final String content;
    private final LocalDate createdAt;
    private int view;
    private boolean isDeleted;
    private final boolean isPublic;

    public boolean canShow() {
        return !isDeleted && isPublic;
    }

    public void delete() {
        this.isDeleted = true;
    }

    public void increaseView() {
        this.view += 1;
    }
}
```
As shown above, the Post class provides three methods: checking if the post can be shown, deleting a post, and increasing the view count.

Do these methods change based on scenario changes? No. For example, if the rule for increasing views changes from one view per hour for a duplicate member to one view every two hours, the `increaseView` method itself does not change.

<br>

### UseCase
This layer encapsulates the application's business rules. Changes in this layer should not affect the Entity layer, and changes in the UI, framework, etc., should not affect this layer.

However, if the application's behavior changes, this layer is affected. This layer executes the Entity's business logic according to the application's behavior.

The interface that abstracts the UseCase layer is called `inputBoundary`.
```java
public interface FindVisiblePostsInputBoundary {
    List<PostResponseModel> create();
}
```
The implementation of the UseCase layer's InputBoundary is called an Interactor.

```java
@Service
public class FindVisiblePostsInteractor implements FindVisiblePostsInputBoundary {
    private final PostGateway postGateway;

    public FindVisiblePostsInteractor(PostGateway postGateway) {
        this.postGateway = postGateway;
    }

    @Override
    public List<PostResponseModel> create() {
        List<PostGatewayResponseModel> postGatewayResponseModels = postGateway.findAll();

        List<Post> posts = postGatewayResponseModels
                .stream()
                .map(PostGatewayResponseModel::fromThis)
                .collect(Collectors.toList());

        List<PostResponseModel> postResponseModels = posts
                .stream()
                .filter(Post::canShow)
                .map(PostResponseModel::of)
                .collect(Collectors.toList());

        return postResponseModels;
    }
}
```

It filters and displays visible posts using the `canShow` method. If all posts are to be retrieved, the `canShow` method does not need to be executed.

<br>

### Interface Adapter
Interface Adapters, simply put, convert data formats that are easy for the Entity and Use Case layers to handle into data formats that are easy for the UI and DB to handle.

This is an interface for abstracting Interface Adapters.

```java
public interface PostGateway {
    void create(CreatePostGatewayRequestModel createPostGatewayRequestModel);
    List<PostGatewayResponseModel> findAll();
}
```
To use JPA, we define a table class. The Interface Adapter performs the role of converting to this format.

```java
@Entity
@Table(name = "post")
@Getter
@NoArgsConstructor
@AllArgsConstructor
public class PostTable {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(name = "title")
    private String title;

    @Column(name = "content")
    private String content;

    @Column(name = "created_at")
    private LocalDate createdAt;

    @Column(name = "view")
    private Integer view;

    @Column(name = "is_deleted", columnDefinition = "tinyint")
    private Boolean isDeleted;

    @Column(name = "is_public", columnDefinition = "tinyint")
    private Boolean isPublic;
}
```

The name of this implementation is `JPAPost`. Therefore, it converts the data format for use with JPA.

If another ORM is used instead of JPA, a class inheriting `PostGateway` can be implemented and used without changing the UseCase.

```java
@Service
public class JPAPost implements PostGateway {
    private final JPAPostRepository JPAPostRepository;

    public JPAPost(JPAPostRepository JPAPostRepository) {
        this.JPAPostRepository = JPAPostRepository;
    }

    @Override
    public void create(CreatePostGatewayRequestModel createPostGatewayRequestModel) {
        this.JPAPostRepository.save(new PostTable(
                null,
                createPostGatewayRequestModel.getTitle(),
                createPostGatewayRequestModel.getContent(),
                createPostGatewayRequestModel.getCreatedAt(),
                createPostGatewayRequestModel.getView(),
                createPostGatewayRequestModel.isDeleted(),
                createPostGatewayRequestModel.isPublic()
        ));
    }

    @Override
    public List<PostGatewayResponseModel> findAll() {
        return JPAPostRepository.findAll()
                .stream()
                .map(postTable -> new PostGatewayResponseModel(
                        postTable.getTitle(),
                        postTable.getContent(),
                        postTable.getCreatedAt(),
                        postTable.getView(),
                        postTable.getIsDeleted(),
                        postTable.getIsPublic()
                ))
                .collect(Collectors.toList());
    }
}

```

`JPARepository` is declared and used.

```java
@Repository
public interface JPAPostRepository extends JpaRepository<PostTable, Long> {
}
```

<br>

### Frameworks, Drivers
This refers to frameworks, databases, drivers, etc. They are located at the outermost layer and do not affect the inner concentric circles.

### 4 Circles?
This is a question asked and answered in every Clean Architecture article. It's because it's a question from the original Clean Architecture text. Of course, it doesn't have to be exactly four. However, the dependency rule must flow from the outside in.
