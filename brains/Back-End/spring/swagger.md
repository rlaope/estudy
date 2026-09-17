# Swagger를 활용한 API 문서 자동화

### Swagger란?
- REST API를 설계, 빌드, 문서화 및 사용하는데 도움이 되는 OpenAPI 사양을 중심으로 구축된 오픈 소스 도구 세트
- 코드 몇 줄 추가를 통해 적용하기 쉬우며, 문서 화면에서 UI를 통해 바로 API 테스트가 가능한 장점

### Swagger 사용법
- build.gradle에 의존성 추가

```gradle
// build.gradle
dependencies {
    compile('io.springfox:springfox-swagger2:2.9.2')
    compile('io.springfox:springfox-swagger-ui:2.9.2')
}
```

- swagger 설정 추가

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
                .apis(RequestHandlerSelectors.basePackage("com.toyproject.book.springboot"))  // Swagger를 적용할 클래스의 package명
                .paths(PathSelectors.any())  // 해당 package 하위에 있는 모든 url에 적용
                .build()
                .apiInfo(apiInfo());
    }

    public ApiInfo apiInfo() {  // API의 이름, 현재 버전, API에 대한 정보
        return new ApiInfoBuilder()
                .title(API_NAME)
                .version(API_VERSION)
                .description(API_DESCRIPTION)
                .build();
    }
```

- controller에 swagger어노테이션 추가

```java
// PostsApiController.java
@RequiredArgsConstructor
@RestController
@RequestMapping("/api/v1")
@Api(tags = {"ToyProject API Test"})  // Swagger 최상단 Controller 명칭
public class PostsApiController {

    private final PostsService postsService;

    @GetMapping("/home")  // 스프링부트 리액트 연동 테스트
    @ApiOperation(value = "연동 테스트", notes = "스프링부트와 리액트 연동을 테스트한다.")  // Swagger에 사용하는 API에 대한 간단 설명
    public String getHome() {
        return "Hello World!";
    }

    @PostMapping("/posts")  // 등록 API
    @ApiOperation(value = "글 등록", notes = "글 등록 API")
    public Long save(@RequestBody PostsSaveRequestDto requestDto) {
        return postsService.save(requestDto);
    }

    @GetMapping("/posts/{id}")  // 조회 API
    @ApiOperation(value = "글 조회", notes = "글 조회 API")
    @ApiImplicitParam(name = "id", value = "글 아이디")  // Swagger에 사용하는 파라미터에 대해 설명
    public PostsResponseDto findById (@PathVariable Long id) {
        return postsService.findById(id);
    }

    @PutMapping("/posts/{id}")   // 수정 API
    @ApiOperation(value = "글 수정", notes = "글 수정 API")
    @ApiImplicitParam(name = "id", value = "글 아이디")
    public Long update(@PathVariable Long id, @RequestBody PostsUpdateRequestDto requestDto){
        return postsService.update(id, requestDto);
    }

    @DeleteMapping("/posts/{id}")   // 삭제 API
    @ApiOperation(value = "글 삭제", notes = "글 삭제 API")
    @ApiImplicitParam(name = "id", value = "글 아이디")
    public Long delete(@PathVariable Long id){
        postsService.delete(id);
        return id;
    }
}

```

- 데이터를 받은 RequestBody 형식에 파라미터 표현

```java
// PostsResponseDto
@Getter
public class PostsResponseDto {

    @ApiModelProperty(example = "글 아이디")  // Swagger에 해당 필드가 무엇인지 나타냄
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

[swagger 사이트](https://swagger.io/)