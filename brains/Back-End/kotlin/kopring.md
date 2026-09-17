# Kotlin으로 Spring Framework 사용

- 스프링에서 작성한 클래스를 확장하는 프록시를 설정해야한다. 하지만 코틀린 클래스는 기본적으로 final이기 때문에 스프링에서 이를 자동으로 확장하기 위해서는 클래스를 open으로 변경해주는 **스프링 플러그인을 빌드 파일에 추가해야한다.**
- 프록시와 실체는 둘 다 같은 인터페이스를 구현하거나 같은 클래스를 확장한다. 들어 오는 요청을 프록시가 가로채고 이 프록시는 서비스가 요구하는 모든 것을 적용한 다음 실체로 요청을 전달한다. 프록시는 필요하다면 응답 또한 가로채서 더 많은 일을 한다. 
  - 예를 들어서 스프링 트랜잭션 프록시는 어떤 메서드 호출을 가로챈 다음 트랜잭션을 시작하고, 해당 메서드를 호출하고, 실체 메서드 안에 일어난 상황에 맞춰 트랜잭션을 커밋하거나 롤백한다.

스프링은 시동 과정에서 프록시를 생성한다. 실체 클래스라면 해당 클래스를 확장하는 데 이 부분이 코틀린에서 문제가 된다. 코틀린은 기본적으로 정적으로 결합한다. 즉 클래스가 open 키웓를 사용해 확장을 위한 open 으로 표시되지 않으면 메서드 재정이 또는 클래스 확장이 불가능하다. 코틀린에서 이런 문제를 `all-open` 플러그인으로 해결한다. **이 플러그인은 클래스와 클래스에 포함된 함수에 명시적으로 open 키워드를 추가하지 않고 명시적인 open 어노테이션으로 클래스를 설정한다.**
  
`all-open` 플러그인도 유용하지만 좀 더 뛰어난 스프링에 꼭 맞는 `kotlin-spring` 플러그인을 사용하는게 더 좋다.

```kts
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
	id("org.springframework.boot") version "2.3.4.RELEASE"
	id("io.spring.dependency-management") version "1.0.10.RELEASE"
	kotlin("jvm") version "1.3.72" // (1)
	kotlin("plugin.spring") version "1.3.72" // (2)
}

group = "com.kotlin"
version = "0.0.1-SNAPSHOT"
java.sourceCompatibility = JavaVersion.VERSION_1_8

repositories {
	mavenCentral()
}

dependencies {
	implementation("org.springframework.boot:spring-boot-starter")
	implementation("org.jetbrains.kotlin:kotlin-reflect") // (3)
	implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8") // (3)
}

tasks.withType<Test> {
	useJUnitPlatform()
}

tasks.withType<KotlinCompile> {
	kotlinOptions {
		freeCompilerArgs = listOf("-Xjsr305=strict") //(4)
		jvmTarget = "1.8"
	}
}
```

1. 코틀린 JVM 플러그인을 프로젝트에 추가
2. 코틀린 스프링 플러그인을 요구
3. 소스 코드가 코틀린으로 작성된 경우 필요
4. JSR-305와 연관된 널 허용 어노테이션 지원

`kotlin-spring` 플러그인은 다음 스프링 어노테이션으로 클래스를 열도록 설정되어 있다.

- @Component, @Configuration, @Controller, @RestController, @Service, @Repository
- @Async
- @Transactional
- @Cacheable
- @SpringBootTest

