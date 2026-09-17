# Koltin에서 Spring을 실행하기 위한 plugin, dependencies

## gradle kotlin dsl 설정
프로젝트를 관리하기 위해 빌드 도구를 설정해 주어야 한다.
  
```kts
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.4.32"
    kotlin("plugin.spring") version "1.4.32"
    id("org.springframework.boot") version "2.4.5"
    id("io.spring.dependency-management") version "1.0.11.RELEASE"
}

group = "personal.project"
version = "0.0.1-SNAPSHOT"
java.sourceCompatibility = JavaVersion.VERSION_11

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.springframework.boot:spring-boot-starter-web")
    implementation("org.jetbrains.kotlin:kotlin-reflect")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
    testImplementation("org.springframework.boot:spring-boot-starter-test")
}

tasks.withType<KotlinCompile> {
    kotlinOptions {
        freeCompilerArgs = listOf("-Xjsr305=strict")
        jvmTarget = "1.8"
    }
}

tasks.withType<Test> {
    useJUnitPlatf
```

## plugin
kotlin에서 spring을 실행하기 위한 plugin들을 살펴보도록 하겠다.
  
> plugin? 미리 구성해둔 task들의 그룹으로 특정 빌드과정에서 필요한 기본 정보들을 포함하고 있으며 필요에따라 커스터마이징 할 수 있다.

```kt
plugins {
    kotlin("jvm") version "1.4.32"
    kotlin("plugin.spring") version "1.4.32"
    id("org.springframework.boot") version "2.4.5"
    id("io.spring.dependency-management") version "1.0.11.RELEASE"
}
```

먼저 kotlin(...)와 id(...)이 있다. 
  
kotlin(...)은 코틀린 전용 id를 축약해 놓은 것이다. 따라서 kotlin(...)은 id(...)로 표현할 수 있으며 `id('org.jetbrains.kotlin.<...>)`와 동일한 의미를 가진다.

<br>

### kotlin("jvm")
코틀린은 멀티 플랫폼을 지원하는 언어다.
  
어떤 플랫폼들을 지원하는지는 공식 홈페이지의 [Supported platforms](https://kotlinlang.org/docs/multiplatform-dsl-reference.html)에서 확인이 가능하다.
  
프로젝트에서는 jvm을 타겟으로 한다고 명시한다.

### kotlin("plugin.spring")
해당 플러그인은 allopen 플러그인이다.
  
코틀린은 기본적으로 클래스는 final로써 open 키워드를 명시적으로 사용하지 않으면 상속이 불가능하다.
  
spring AOP는 cglib를 사용하는데 이는 상속을 통해서 proxy 패턴을 사용합니다.
  
따라서 해당 플러그인은 클래스를 open이 기본으로 설정해줍니다.

### id("org.springframework.boot")
spring boot를 사용하기 위한 플러그인입니다.

### id("io.spring.dependency-management")
spring 관련 의존성의 버전관리를 일괄적으로 하기 위한 플러그인입니다.

## dependencies
의존성을 한번 보도록 하겠다. Java에서도 spring boot를 사용할 때 이용하는 의존성은 간단하게 설명하며 kotlin 전용 의존성은 좀 더 깊게 이야기하도록 하겠다.

`implementatiion("org.springframeworkboot:spring-boot-starter-web")`: spring boot를 이용하기 위한 의존성
  
`testImplementation("org.springframework.boot:spring-boot-starter-test")`: spring boot 테스트를 이용하기 위한 의존성
  
`implementation("org.jetbrains.kotlin:kotlin-reflect")`: kotlin은 런타임 라이브러리 용량을 줄이기 위해서 기본적으로 reflect를 제공해주지 않지만 이 의존성을 추가하면 reflect를 이용할 수 있다.
  
`implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")`: 이 라이브러리는 kotlin에서 필수적인 기능들을 제공한다.
- let, apply, use, synchronized 등의 함수
- collection의 이용에 도움이되는 확장 함수
- 문자열을 다루는 다양한 유틸들
- IO, Threading 관련 함수들

## Main
```kt
@SpringBootApplication
class Application

fun main(args: Array<String>) {
    runApplication<Application>(*args)
}
```
빌드 도구에 대한 설정이 마무리된 후 아래와 같은 코드를 src/main/kotlin/{package_name}에 코틀린 클래스를 만들어 입력한다.