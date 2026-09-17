# Using Spring Framework with Kotlin

- Spring requires configuring proxies that extend classes written in Spring. However, Kotlin classes are `final` by default, so to allow Spring to automatically extend them, you **must add the Spring plugin to your build file** to make the classes `open`.
- Both proxies and concrete implementations either implement the same interface or extend the same class. The proxy intercepts incoming requests, applies everything the service requires, and then forwards the request to the concrete implementation. If necessary, the proxy also intercepts the response to perform additional tasks.
  - For example, a Spring transaction proxy intercepts a method call, starts a transaction, invokes the method, and then commits or rolls back the transaction based on what happened within the concrete method.

Spring creates proxies during startup. If it's a concrete class, extending that class becomes an issue in Kotlin. Kotlin is statically bound by default. This means that method overriding or class extension is not possible unless the class is marked as `open` for extension using the `open` keyword. Kotlin addresses this problem with the `all-open` plugin. **This plugin configures classes with explicit `open` annotations without explicitly adding the `open` keyword to the classes and functions contained within them.**

While the `all-open` plugin is useful, it's better to use the more advanced `kotlin-spring` plugin, which is specifically tailored for Spring.

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

1. Add the Kotlin JVM plugin to the project
2. Requires the Kotlin Spring plugin
3. Required when source code is written in Kotlin
4. Supports nullability annotations related to JSR-305

The `kotlin-spring` plugin is configured to open classes with the following Spring annotations:

- @Component, @Configuration, @Controller, @RestController, @Service, @Repository
- @Async
- @Transactional
- @Cacheable
- @SpringBootTest
