# Writing Test Code with Kotest

## Kotest
Kotest is a testing framework that allows you to test Kotlin in a Kotlin-idiomatic way, specifically providing the following features:
1. It supports Kotlin-specific features provided by Kotlin. (Coroutine, Extension Function, Kotlin DSL, etc.)
2. It provides various Assertions in a Kotlin DSL style.
3. It offers various Test Layouts, including BDD.

> BDD : Behavior Driven Development
> A development methodology derived from TDD, it makes tests easy to understand because test cases are written based on scenarios.
> It primarily uses the Give-When-Then structure as its basic pattern.

### Advantages
- Can improve readability of nested test code.
- Clearer distinction with DSL-like constructs: increased readability.
- Kotlin is multi-platform, allowing for various platform styles.
  - Provides diverse test layouts.
  - Scala, Ruby... etc.

### Dependencies

```kts
dependencies {   
    testImplementation("io.kotest:kotest-runner-junit5-jvm:${KOTEST_VERSION}")
    testImplementation("io.kotest:kotest-assertions-core-jvm:${KOTEST_VERSION}")
    testImplementation("io.mockk:mockk:1.12.8") // use mocking in unit tests
}

tasks.test {
    useJUnitPlatform()
}
```

## Testing Styles
Main Test Layouts provided by Kotest

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

- You can add a description for the test code as a String after `test`.
- Since field variables cannot be used, it is mainly used for function testing.
- Similar to JUnit's `@Disabled`, you can exclude test code from execution using `xcontext` or `xtest`.

### Describe Spec
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
- The Spring framework uses BDD (given, when, then), and Ruby or JS also use similar `describe`, `it` keywords to write test code. Supports DCI (Describe, Context, It) layout.
- In the code above, `context` can be omitted.
- Similar to FunSpec, if you use `xdescribe` and `xit`, the corresponding case does not need to be executed.

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

- Provides BDD-style test code.
- It offers the familiar `given`, `when`, `then`.
- You can disable test code using `xgiven`, `xwhen`, `xthen`.

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

### Inspectors
If there is a collection in the test code, perform tests on its elements.
```kt
mylist.forExactly(3) {
    it.city shouldBe "Chicago"
} // Exactly 3 elements in mylist have city as Chicago

val xs = listOf("sam", "gareth", "timothy", "muhammad")

xs.forAtLeast(2) { // At least 2 elements must satisfy the lambda expression
    it.shouldHaveMinLength(7) // true if length is less than or equal to 7, false otherwise
}
```

### Exceptions
The test execution results in an Exception.
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

- You need to add the Spring extension by overriding `fun extensions() = listOf(SpringExtension)`.
- The `CalculatorService` above has a `MockComponent`.
- If you want to mock `MockComponent`, you can use `@MockkBean` or `@SpykBean` from `springmockk` to utilize Spring Boot's `@MockBean` and `@SpyBean` functionalities.
