# Kotlin DSL

```gradle
plugins {
	kotlin("jvm") version "1.3.72"
}
```

Kotlin DSL can be used in Gradle 5.0 and later, and the `kotlin-gradle-plugin` registered in the Gradle plugin repository is available in Gradle 5.0 and later. Using the latest syntax, there's no need to specify where to find plugins, like in a `repositories` block. This applies to all Gradle plugins registered in the Gradle plugin repository. Furthermore, using the `plugins` block automatically applies the plugin, so there's no need to use an `apply` statement.

> The default build file names for using Kotlin DSL in Gradle are settings.gradle.kts and build.gradle.kts.

The biggest differences from Groovy DSL are as follows:
- All strings use double quotes.
- Parentheses are mandatory in Kotlin DSL.
- Kotlin uses the equals sign `=` instead of a colon `:` to assign values.

While using the `settings.gradle.kts` file is recommended, it's not mandatory. During the initialization phase of the Gradle build process, `settings.gradle.kts` is processed when Gradle determines which project build files to analyze. In a multi-project build, `settings.gradle.kts` contains information about whether a root's subdirectories are Gradle projects. Gradle configuration information and dependencies can be shared among subprojects. Subprojects can depend on other subprojects, and subprojects can even be built in parallel.

## Building a Kotlin Project with Gradle

```gradle
plugins {
    `java-libray` // (1)
	kotlin("jvm") version "1.3.72" // (2)
}

repositoies {
    jcentet()
}

dependencies {
    implementations(kotlin("stdlib")) // (3)
}
```

1. Add Gradle tasks from the Java library plugin
2. Add the Kotlin plugin to Gradle
3. Add the Kotlin standard library to the project at compile time
