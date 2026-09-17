# Plugins and Dependencies for Running Spring in Kotlin

## Gradle Kotlin DSL Configuration
We need to configure a build tool to manage the project.

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

## Plugins
Let's look at the plugins required to run Spring in Kotlin.

> Plugin? A group of pre-configured tasks that include essential information needed during a specific build process, and can be customized as needed.

```kt
plugins {
    kotlin("jvm") version "1.4.32"
    kotlin("plugin.spring") version "1.4.32"
    id("org.springframework.boot") version "2.4.5"
    id("io.spring.dependency-management") version "1.0.11.RELEASE"
}
```

First, there are `kotlin(...)` and `id(...)`.

`kotlin(...)` is an abbreviation for Kotlin-specific IDs. Therefore, `kotlin(...)` can be expressed as `id(...)` and has the same meaning as `id('org.jetbrains.kotlin.<...>')`.

<br>

### kotlin("jvm")
Kotlin is a multi-platform language.

You can check which platforms are supported on the official website's [Supported platforms](https://kotlinlang.org/docs/multiplatform-dsl-reference.html) page.

In the project, we explicitly target the JVM.

### kotlin("plugin.spring")
This plugin is the allopen plugin.

By default, Kotlin classes are final, meaning inheritance is not possible unless the `open` keyword is explicitly used.

Spring AOP uses cglib, which employs the proxy pattern through inheritance.

Therefore, this plugin sets classes to be open by default.

### id("org.springframework.boot")
This is the plugin for using Spring Boot.

### id("io.spring.dependency-management")
This plugin is for managing versions of Spring-related dependencies uniformly.

## Dependencies
Let's take a look at the dependencies. I'll briefly explain the dependencies used when using Spring Boot in Java, and then discuss Kotlin-specific dependencies in more detail.

`implementatiion("org.springframeworkboot:spring-boot-starter-web")`: Dependency for using Spring Boot.

`testImplementation("org.springframework.boot:spring-boot-starter-test")`: Dependency for using Spring Boot tests.

`implementation("org.jetbrains.kotlin:kotlin-reflect")`: Kotlin does not provide reflection by default to reduce runtime library size, but adding this dependency allows you to use reflection.

`implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")`: This library provides essential functionalities in Kotlin.
- Functions like `let`, `apply`, `use`, `synchronized`, etc.
- Extension functions that aid in using collections.
- Various utilities for string manipulation.
- Functions related to IO and Threading.

## Main
```kt
@SpringBootApplication
class Application

fun main(args: Array<String>) {
    runApplication<Application>(*args)
}
```
After the build tool configuration is complete, create a Kotlin class in `src/main/kotlin/{package_name}` and enter the following code.
