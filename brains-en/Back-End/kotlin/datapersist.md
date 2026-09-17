# Implementing Persistence with Data Classes

Adding the `kotlin-jpa` plugin to your build file makes it easy to use JPA.

```kt
data class Person(
    val name: String,
    val dob: LocalDate
)
```

From a JPA perspective, data classes have two issues.
1. JPA requires a no-argument constructor unless default values are provided for all properties, but data classes do not have a no-argument constructor.
2. Creating a data class with `val` properties results in an immutable object, which is not designed to work well with JPA objects.

## No-Argument Constructor Issue
Kotlin provides two plugins to solve the no-argument constructor issue. The `no-arg` plugin allows you to select classes to which a no-argument constructor will be added, and you can define annotations that trigger the addition of a no-argument constructor. The `no-arg` plugin automatically configures no-argument constructors for Kotlin entities.

```kts
plugins {
	kotlin("plugin.jpa") version "1.3.72"
}

dependencies {
	implementation("org.springframework.boot:spring-boot-starter-data-jpa")
}
```

You can use the `no-arg` plugin by adding the necessary syntax to your build file, similar to the `kotlin-spring` plugin. The compiler plugin adds a synthetic no-argument constructor to Kotlin classes. This means that you cannot call the synthetic no-argument constructor directly from Java or Kotlin. However, Spring can call the synthetic no-argument constructor using reflection.

The `kotlin-jpa` plugin is easier to use than the `no-arg` plugin. The `kotlin-jpa` plugin is built upon the `no-arg` plugin. The `kotlin-jpa` plugin adds a no-argument constructor to classes automatically marked with the following annotations:

- @Entity
- @Embeddable
- @MappedSuperClass

## Difficulties Using Immutable Classes for JPA Entities
You typically don't want to use immutable classes for JPA entities. Therefore, the Spring development team recommends using simple Kotlin classes with `var` properties for fields that you want to be mutable when used as entities.
