# Automating API Documentation with Swagger

### What is Swagger?
- An open-source toolset built around the OpenAPI Specification to help design, build, document, and consume REST APIs.
- Easy to apply by adding a few lines of code, with the advantage of being able to test APIs directly through the UI on the documentation screen.

### How to Use Swagger
- Add dependencies to build.gradle

```gradle
// build.gradle
dependencies {
    compile('io.springfox:springfox-swagger2:2.9.2')
    compile('io.springfox:springfox-swagger-ui:2.9.2')
}
```

- Add Swagger configuration

```java
// config/SwaggerConfig.java
@Configuration
@EnableSwagger2
public class SwaggerConfig {  // Swagger

    private static final String API_NAME = "ToyProject API";
    private static final String API_VERSION = "0.0.1";
    private static final String API_DESCRIPTION = "ToyProject API 명세서";

    @Bean
    public Docket api() {
        return new Docket(DocumentationType.SWAGGER_2)
                .select()
                .apis(RequestHandlerSelectors.basePackage("com.toyproject.book.springboot"))  // Package name of the class to which Swagger will be applied
                .paths(PathSelectors.any())  // Apply to all URLs under this package
                .build()
                .apiInfo(apiInfo());
    }

    public ApiInfo apiInfo() {  // API name, current version, and information about the API
        return new ApiInfoBuilder()
                .title(API_NAME)
                .version(API_VERSION)
                .description(API_DESCRIPTION)
                .build();
    }
```

- Add Swagger annotations to the controller

```java
// PostsApiController.java
@RequiredArgsConstructor
@RestController
@RequestMapping("/api/v1")
@Api(tags = {"ToyProject API Test"})  // Top-level Controller name in Swagger
public class PostsApiController {

    private final PostsService postsService;

    @GetMapping("/home")  // Spring Boot React integration test
    @ApiOperation(value = "연동 테스트", notes = "스프링부트와 리액트 연동을 테스트한다.")  // Brief description of the API used in Swagger
    public String getHome() {
        return "Hello World!";
    }

    @PostMapping("/posts")  // Registration API
    @ApiOperation(value = "글 등록", notes = "글 등록 API")
    public Long save(@RequestBody PostsSaveRequestDto requestDto) {
        return postsService.save(requestDto);
    }

    @GetMapping("/posts/{id}")  // Lookup API
    @ApiOperation(value = "글 조회", notes = "글 조회 API")
    @ApiImplicitParam(name = "id", value = "글 아이디")  // Description of parameters used in Swagger
    public PostsResponseDto findById (@PathVariable Long id) {
        return postsService.findById(id);
    }

    @PutMapping("/posts/{id}")   // Update API
    @ApiOperation(value = "글 수정", notes = "글 수정 API")
    @ApiImplicitParam(name = "id", value = "글 아이디")
    public Long update(@PathVariable Long id, @RequestBody PostsUpdateRequestDto requestDto){
        return postsService.update(id, requestDto);
    }

    @DeleteMapping("/posts/{id}")   // Delete API
    @ApiOperation(value = "글 삭제", notes = "글 삭제 API")
    @ApiImplicitParam(name = "id", value = "글 아이디")
    public Long delete(@PathVariable Long id){
        postsService.delete(id);
        return id;
    }
}

```

- Parameter representation in RequestBody format for received data

```java
// PostsResponseDto
@Getter
public class PostsResponseDto {

    @ApiModelProperty(example = "글 아이디")  // Indicates what this field is in Swagger
    private Long id;

    @ApiModelProperty(example = "글 제목")
    private String title;

    @ApiModelProperty(example = "글 설명")
    private String description;

    @ApiModelProperty(example = "공구 관련 링크")
    private String link;

    @ApiModelProperty(example = "공구 오픈채팅 링크")
    private String contact;

    @ApiModelProperty(example = "공구 가격")
    private String price;

    @ApiModelProperty(example = "공구 날짜")
    private String date;

    @ApiModelProperty(example = "작성자")
    private String author;

    public PostsResponseDto(Posts entity) {
        this.id = entity.getId();
        this.title = entity.getTitle();
        this.description = entity.getDescription();
        this.link = entity.getLink();
        this.contact = entity.getContact();
        this.date = entity.getDate();
        this.price = entity.getPrice();
        this.author = entity.getAuthor();
    }
}

```

[Swagger website](https://swagger.io/)
