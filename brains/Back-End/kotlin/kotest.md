# kotest를 사용해 테스트 코드 작성하기

## kotest
코틀린을 코틀린스럽게 테스트할 수 있는 테스트 프레임워크로 구체적으로 다음과 같은 기능을 제공한다.
1. 코틀렌에서 제공하는 코틀린 특화 기능을 지원한다. (Coroutine, Extension Fuction, Kotlin DSL 등)
2. 다양한 Assertions를 Kotlin DSL 스타일로 제공한다.
3. BDD를 포함한 다양한 Test Layout을 제공한다.

> BDD : Behavior Driven Development  
> TDD에서 파생된 개발 방법으로 시나리오를 기반으로 테스트 케이스를 작성하기 때문에 테스트를 이해하기 쉽다.  
> Give-When-Then 구조를 기본 패턴으로 사용한다.

### 장점
- nested test code의 가독성을 가져올 수 있음
- DSL과 같은 구성으로 좀 더 명확하게 구분을 지을 수 있음: 가독성 증가
- Kotlin는 멀티 플랫폼이므로 다양한 플랫폼의 스타일이 가능
  - 다양한 테스트 레이아웃 제공
  - 스칼라 루비... etc

### Dependencies

```kts
dependencies {   
    testImplementation("io.kotest:kotest-runner-junit5-jvm:${KOTEST_VERSION}")
    testImplementation("io.kotest:kotest-assertions-core-jvm:${KOTEST_VERSION}")
    testImplementation("io.mockk:mockk:1.12.8") // unit test 에서 mockking 사용
}

tasks.test {
    useJUnitPlatform()
}
```

## Testing Styles
Kotest에서 제공하는 주요 Test Layout

## FunSpec

```kt
class CalFunSpec: FunSpec({
    test("1과 2를 더하면 3이 반환된다") {
        val stub = Calculator()
        val result = stub.calculate("1 + 2")
        result sholdBe 3
    }

    context("enabled test run"){
        test("test code run"){ // 실행
            val stub = Calculator()

            val result = sutb.calculate("1 + 2")

            result sholdBe 3
        }

        xtest("test code not run){ // 실행 안함
            val stub = Calculator()

            val result = sutb.calculate("1 + 2")

            result sholdBe 3
        }
    }

    xcontext("disabled test run"){ // 하위 모두 미 실행
        test("test code run but outer context is disabled){
            val stub = Calculator()

            val result = stub.caculate("1 + 2")
            
            result shouldBe 3
        }

        xtest("test code not run){
            val stub = Calculator()

            val result = stub.caculate("1 + 2")
            
            result shouldBe 3
        }
    }

    
})
```

- test 뒤에 String으로 테스트코드에 대한 설명을 추가할 수 있다.
- 필드 변수 사용이 불가능하므로 함수 테스트에 주로 사용 된다.
- junit에 @Disabled 와 같이 xcontext나 xtest를 통해서 test code를 실행에서 제외할 수 있다.


### Descirbe Spec
```kt
class CalDescribeSpec : DescribeSpec({
    val stub = Calculator()

    describe("calculate") {
        context("식이 주어지면") {
            it("해당 식에 대한 결과 값이 반환 된다") {
                calculations.forAll { (expression, data) ->
                    val result = stub.calculate(expression)

                    result shouldBe data
                }
            }
        }
    }
})
```
- spring 진영에서는 BDD(given, when, then) 쓰고 있고 Ruby나 js에서도 비슷하게 describe, it 키워드를 사용해 test code를 작성할 수 있습니다. DCI(Describe, Context, It) layout 지원
- 위 코드에서 context는 생략해도 됩니다.
- FunSpec과 동일하게 xdescribe와 xit을 사용하면 해당 case는 실행할 필요가 없습니다.

### Behavior Spec

```kt
class CalBehaviorSpec : BehaviorSepc({
    val stub = Calculator()

    Given("calculator){
        // before Each라고 생각하기
        val expression = "1 + 2"

        When("1과 2를 더하면"){
            val result = stub.calculate(expression)
            Then("3이 반환된다"){
                result shouldBe 3
            }
        }

        When("1 + 2 결과와 같은 String 입력시 동일한 결과가 나온다."){
            val result = stub.calculate(expression)
            Then("해당 하는 결과값이 반환된다."){
                result shouldBe stub.calculate("1 + 2")
            }
        }
    }
})
```

- BDD 스타일 테스트 코드를 제공합니다.
- 우리가 아는 given, when, then을 제공합니다.
- xgiven, xwhen, xthen을 통해서 테스트 코드를 disable 할 수 있다.

### Annotation Spec
```kt
class AnnotationSpecExample : AnnotationSpec() {

    @BeforeEach
    fun beforeTest() {
        println("Before each test")
    }

    @Test
    fun test1() {
        1 shouldBe 1
    }

    @Test
    fun test2() {
        3 shouldBe 3
    }
}
```

## Kotest Assertions

### Match

```kt
// 기본형
name shouldBe "sam" // assertThat(name).isEqualTo("sam")
name shouldNotBe null // assertThat(name).isNull()

// 체인형 -> 여러 조건을 chaining 할 수 있습니다
myImageFile.shouldHaveExtension(".jpg").shouldStartWith("https").shouldBeLowerCase()
```

### Inspectors 점검자
테스트 코드의 컬렉션이 있다면 요소에대한 테스트를 진행
```kt
mylist.forExactly(3) {
    it.city shouldBe "Chicago"
} // my list중 정확히 3개의 element의 city가 Chicargo 이다

val xs = listOf("sam", "gareth", "timothy", "muhammad")

xs.forAtLeast(2) { // 최소 2개의 요소가 lamdba 식이 true여야한다
    it.shouldHaveMinLength(7) // length가 7 보다 이하일 경우 true 초과 일 경우 false
}
```

### Exceptions
test의 실행 결과가 Exception이 발생
```kt
shouldThrow {
    assertThrows { }
    // code in here that you expect to throw an IllegalAccessException
}
```

## Spring Test
```kt
@SpringBootTest
internal class CalSpringBootBehavioWithMockSpec : BehaviorSpec() {
    override fun extensions() = listOf(SpringExtension)

    @Autowired
    private lateinit var calculatorService: CalculatorService

    @MockkBean
    private lateinit var mockComponent: MockComponent

    init {
        this.Given("calculate") {
            When("식이 주어지면") {
                Then("해당 식에 대한 결과값이 반환된다") {
                    calculations.forAll { (expression, data) ->
                        val result = calculatorService.calculate(expression)

                        result shouldBe data
                    }
                }
            }
        }

        this.Given("Mocking 한 값과 합을 구한다") {
            every { mockComponent.returnOne() } answers { 2 }

            When("덧셈 로직 실행") {
                val result = calculatorService.calPlus(2)
                Then("덧셈 결과") {
                    result shouldBe 4
                }
            }
        }
    }

    companion object {
        private val calculations = listOf(
            "1 + 3 * 5" to 20.0,
            "2 - 8 / 3 - 3" to -5.0,
            "1 + 2 + 3 + 4 + 5" to 15.0
        )
    }
}
```

- override fun extensions() = listOf(SpringExtension)를 통해서 spring extension을 추가해줘야 한다.
- 위에서 CalculatorService는 MockComponent를 가지고 있다.
- 그리고 MockComponent를 모킹하고 싶을 경우 springmockk의 @MockkBean이나 @SpykBean을 선언해 스프링 부트의 @MockBean, @SpyBean기능을 사용할 수 있다.