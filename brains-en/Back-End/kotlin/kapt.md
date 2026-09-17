# [Kotlin] What is kapt?

![](image/kapt_0.jpg)

### kapt

kapt is a Gradle plugin for **running annotation processors using the Kotlin language.**

### annotation processor

An annotation processor reads annotations in source code at compile time and can generate or modify additional code based on them. This offers various benefits, such as automated code generation and improved runtime performance.

The kapt plugin can only be used in Kotlin projects that utilize the kotlin-gradle-plugin. To use kapt, you must first add the kotlin-kapt library to your project dependencies.

```
plugins {
    id("org.jetbrains.kotlin.jvm") version "1.5.31"
    id("org.jetbrains.kotlin.kapt") version "1.5.31"
}

dependencies {
    implementation("com.example:some-library:1.0")
    kapt("com.example:some-library-processor:1.0")
}
```

In the example above, the kotlin-kapt plugin is added, and the annotation processor for the com.example:some-library library is specified as the com.example:some-library-processor library.

### kapt tesk

The kapt plugin generates the following tesks to run annotation processors at compile time.

*   **kaptKotlin**: Generates additional code using Kotlin source code and annotation processors.
*   **compileKotlin**: Depends on the kaptKotlin tesk and compiles Kotlin source code, including the generated additional code.
*   **compileJava**: Depends on the kaptKotlin tesk and compiles Java source code generated using annotation processors.

### Differences and Advantages of kapt and annotation processors

kapt and annotation processors are closely related concepts.

*An **annotation processor** is a feature provided by Java that reads annotations in source code at compile time and can generate or modify additional code based on them. This offers various benefits, such as automated code generation and improved runtime performance.*

As mentioned, **kapt** is a Gradle plugin for running annotation processors in Kotlin. In other words, kapt is the plugin for executing annotation processors within Kotlin.

While *annotation processors can be used directly in Kotlin*, using kapt makes it more convenient to manage them within Gradle. **kapt automatically runs annotation processors and compiles the generated code.** Furthermore, kapt allows the use of Kotlin-specific annotation processors and simplifies **configuring code generation paths or classpath settings** for annotation processors.

Therefore, when using Kotlin, it is generally more convenient to use kapt. However, there are cases where annotation processors are needed even when using existing Java libraries, so you can use annotation processors directly if necessary.
